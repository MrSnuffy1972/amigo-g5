# Game 1454 (battle squadron - the destruction of the barrax empire! (1989)(innerprise software)[cr tic][t freestyle][a3]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01454). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903920 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 38 raw-word lines |
| Functions named and described | pass, 195 of 195 named (0 author symbols), 195 of 195 described (10 by rule, 152 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4612 of 4612 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9525 of 9525 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 36453 |
| code | 36416 |
| data | 830344 |
| padding | 579 |
| unused | 128 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf:bootblock | 1 | 50 | 5 |
| logged disk load from Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf boot-block load | 1 | 7988 | 126 |
| logged disk load from Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf boot-block load | 1 | 432 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 600 | 23 |
| logged disk load from Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf | 1 | 408 | 8 |
| logged disk load from Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf | 1 | 46 | 4 |

## Cases handled

- battle squadron - the destruction of the barrax empire! (1989)(innerprise software)[cr tic][t freestyle][a3]/Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf: custom-format (non-DOS) disk, 1750 non-empty blocks: matched against track-loaded segments
- battle squadron - the destruction of the barrax empire! (1989)(innerprise software)[cr tic][t freestyle][a3]/Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf: the boot block reads 67584 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- battle squadron - the destruction of the barrax empire! (1989)(innerprise software)[cr tic][t freestyle][a3]/Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf: the boot block reads 8192 bytes from disk offset 892928 to an address (found statically); recovered as a unit
- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 538 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- battle squadron - the destruction of the barrax empire! (1989)(innerprise software)[cr tic][t freestyle][a3]/Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf: 329 executed PCs whose RAM was overwritten before any snapshot match its bytes 641088-642832: recovered as a unit run at an address
- battle squadron - the destruction of the barrax empire! (1989)(innerprise software)[cr tic][t freestyle][a3]/Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf: 27 executed PCs whose RAM was overwritten before any snapshot match its bytes 0-448: recovered as a unit run at an address
- battle squadron - the destruction of the barrax empire! (1989)(innerprise software)[cr tic][t freestyle][a3]/Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf: 4 logged loads cover 77968 bytes of the image
- 2 packed track region(s), 822272 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `load_and_poll_structure` (Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf boot-block load +an address to an address (67584 bytes)): Clears bits 3 and 7 of a CIA register, calls another routine, and then loads a longword and two bytes from a structure into global variables. Kind: disk/track check loop. Pass path: the loop continues (u1__sub_EE3C is called) as long as _DAT_0000f108 is non-zero; the check fails/breaks when the value is 0.
- `clear_and_increment_flag` (Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf boot-block load +an address to an address (67584 bytes)): Clears bit 1 of a CIA register, increments a global word, calls another routine, and then branches based on the carry flag. Kind: checksum/integrity trap. Pass path: the branch where bVar3 is true (skipping the sub_EE70 call and the bit manipulation).
- `loop_cia_bit_clear` (Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf boot-block load +an address to an address (67584 bytes)): Calls another routine, clears and then sets bit 0 of a CIA register, and then enters a loop that repeats a number of times specified by a value in a global word. Kind: disk hardware/sync check. Pass path: the branch following the loop (the code continues execution normally if the bit condition is met).
- `handle_joystick_vector` (Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf boot-block load +an address to an address (67584 bytes)): Sets a custom chip register to a vector value, then loops while a joystick button bit is set, setting a port bit and calling another routine, and finally clears the register before returning. Kind: loader/boot-block protection. Pass path: the loop continues and calls u1__sub_EE50() if the bitmask check passes.
- `process_shifted_value` (Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf boot-block load +an address to an address (67584 bytes)): Compares a shifted version of a global variable with a shifted version of a value in d7, sets a bit in a CIA register, and calls another routine if the comparison fails. Kind: checksum/integrity trap. Pass path: the loop completes and returns the shifted value (the branch at an address is the failure/error path).
- `read_shifted_with_flag` (Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf boot-block load +an address to an address (67584 bytes)): Sets a flag in a custom chip register, reads a value from a global variable, clears the flag, and returns the value shifted right by one. Kind: disk hardware/sync check. Pass path: the branch at an address (the bcs.b instruction).
- `read_track_and_scan_sync` (Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf boot-block load +an address to an address (67584 bytes)): Waits for a status bit to clear, configures disk-related custom chip registers, then scans a buffer for sync words in a loop, deinterleaves each found sector, and copies the data into a destination buffer until the count reaches zero. Kind: disk track/sync check. Pass path: the loop continues and eventually reaches the data copy/load phase if the sync pattern and checksum match.

## Functions


### Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf:bootblock

- `issue_boot_io_request`: Entered from the boot strap; the decompiler timed out, so the only evidence is an exec library I/O call and two unnamed calls over thirteen instructions.
- `call_single_helper_stub`: A three-instruction routine reached by address and by execution that calls one unnamed routine; the decompiler timed out, so nothing more is recovered.
- `configure_copper_dma_interrupts`: A fourteen-instruction routine that the facts show touching the copper list pointer, DMA control, interrupt enable and interrupt request registers; the decompiler timed out, so its exact sequence is not recovered.
- `stub_bad_instruction_data`: Conditionally stores a fixed word into a global and then runs into undecodable bad instruction data, so the routine is only partly recovered.
- `read_block_via_library_call`: Fills an I/O request with a request type of two, an 8192-byte length, a destination and an offset, calls a library routine at offset minus 456 to run it, then calls one unnamed routine and clears a global four times.

### logged disk load from Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf boot-block load

- `replace_low_memory_vectors`: Saves the current values at several low-memory vector addresses, replaces them with new constants, sets a stack-based pointer and records the condition flags, after one unnamed call.
- `copy_global_to_low_memory`: Copies a global word into a fixed low-memory slot and returns.
- `initialize_loop_counter`: Loads the value 1599 into register d0 and decrements it until it reaches negative one, then returns.
- `write_looping_table_entries`: Selects a base value depending on a flag bit in a global variable, then loops three times writing a constant word and the evolving value into a table at offset 44148, advancing the pointer and incrementing the value each iteration.
- `transfer_memory_bytes`: Moves several bytes between various offsets of a structure pointed to by a6, including two byte-wise moves from positive offsets to negative offsets, one byte-wise move of the value 255 to a negative offset, one byte-wise move from a positive offset to a negative offset, one word-wise move from a positive offset to a negative offset, and clears two bytes at negative offsets.
- `initialize_global_state_data`: Initializes several global structures by writing various constants and clearing multiple memory locations, including a conditional block that sets a sequence of seven bytes based on the value of a short integer.
- `reset_game_state_and_clear_memory`: Clears various fields in a global structure, initializes a subset of parameters, and calls another routine four times to clear specific memory blocks of varying sizes.
- `clear_specific_offsets`: Clears a byte at offset 45 of a structure in a4, clears a word at offset 120 of a4, copies a word from offset 10628 of a5 to offset 58 of a4, and clears a word at offset 60 of a4.
- `initialize_structure_fields`: Sets various byte and word offsets within a structure pointed to by a6 to specific values, calls two routines to clear or copy data, and then uses a loop to clear a block of memory starting at an offset of 61 from a6.
- `clear_memory_block`: Clears a block of memory starting at the address in a0 one longword at a time, using a dbra loop counter, and returns.
- `copy_memory_block`: Copies a block of memory from the address in a1 to the address in a0 one longword at a time, using a dbra loop counter, and returns.
- `interleave_source_data`: Selects a source address from a table based on a flag bit, then checks two bytes at offsets 6c and 74 against several pairs before incrementing a counter at offset 38 and calling another routine.
- `copy_table_data`: Selects a source table pointer based on whether the value at offset 7068 equals 3 and the byte at offset -27710 is zero, then copies a run of bytes from that table into a contiguous block of global memory starting at offset 45640 and returns.
- `perform_memory_operations`: Copies a word-count of longwords from one memory location to another, or invokes a helper when the flag at offset 12 of the structure in a4 is 2 or negative 2, then performs a byte-by-byte reverse copy of a memory range when that flag is negative and the pointer equals an address, and returns.
- `write_data_to_table`: Waits until a byte at offset 6 in a6 reaches a threshold, then conditionally calls a routine when a flag byte in a5 is non-zero.
- `increment_bit_field`: Compares the byte in d1 with the value in d5, and when they match computes a mask from the low six bits of d6, then increments the word pointed to by a0 by one when the masked value falls within the range and differs from the value masked by d4.
- `clear_word_array`: Sets thirty-two consecutive words to zero using a loop.
- `tiny_moveq`: Small routine of 1 instruction that loads 47 into d3.
- `run_nested_wait_loop`: Waits for a byte at offset 6 of a structure to reach 240, then, if a byte at offset -25593 of another structure is nonzero, calls another routine.
- `decrement_bit_field`: Checks if a specific bit is set within a value and, if conditions are met, subtracts a bit value from a memory location.
- `decrement_and_update_table_entries`: Decrements two counters in global memory and, if a counter remains positive, iterates through a table to calculate and store new values into a destination structure.
- `load_address_constant`: Loads the constant 28892 into a0.
- `clear_memory_block_2`: Writes zeros to a block of memory using a loop counter of 15, each iteration clearing one longword at the pointer in a0, one byte thirteen bytes past it, and advancing the pointer by five, then returns.
- `branch_sprite_bitmaps`: Checks a word at an offset from a pointer in a5 and, if it is not zero, loads three values into registers a2, a4, and a3 before calling another routine; otherwise, it performs a different set of register loads and calls another routine.
- `update_sprite_bitmaps`: Decrements a 16-bit counter in a global structure and, while a flag at offset 72 is not 256, applies a bitmask to clear or set bits in 32-bit words of a bitmap buffer using a source table at offset 96.
- `copy_shared_state_parameters`: Reads a masked word and two flag bytes from a structure, then selects a source offset from a small set of constants to copy three consecutive word values into a fixed block of global memory.
- `process_keyboard_data`: Checks two values at offsets from a5 and, depending on their values, updates a pointer in a global variable, modifies a global variable, or writes a specific value to the keyboard serial register.
- `merge_and_encode_tables`: Loops over two tables of 16-bit values, advancing through them until both entries are zero; when both are zero, it writes a fixed byte sequence to a destination buffer and returns; otherwise, it stores a byte derived from the source values and copies four bytes from a table at offset 211a based on a shifted source value.
- `update_sprite_attribute_table`: Copies three 16-bit words from a source table into a destination buffer using an offset derived from a byte at offset 59 of the structure in a4, then conditionally fills an attribute list at offset 994 with coordinate and property values depending on sprite index ranges and status flags, otherwise it returns.
- `update_attribute_table`: Checks a 16-bit value at offset 52 of a structure in a4 and, if it is not 0 and not 60, loads two 16-bit values from a pointer-based source into d5 and d6, then writes those values into the structure in a3 at several specified offsets.
- `initialize_global_data`: Stores a longword into four consecutive global variables and then, based on a value from a global variable, loops to fill a range of memory with the word 512.
- `initialize_data_structures`: Stores the constant an address into four consecutive global variables, then decrements a pointer from offset 120 in a structure and fills it with the value 320 for a count of zero to seven.
- `increment_global_byte`: Increments a byte stored at a specific memory location by one.
- `copy_data_block`: Copies a series of words and longwords from a source buffer into a destination structure and updates a pointer within that structure.
- `identify_entity_type`: Compares a 16-bit value read from a table against a series of constants to determine an entity type, then sets a global variable to a specific value or jumps to the corresponding handler routine based on the match.
- `store_computed_offsets`: Stores a word from a source structure into a destination structure at offset 102, then copies several fields and computes a value combining two source words, storing the results into a destination structure at offsets 80, 84 and 88 before returning.
- `check_object_boundaries`: Loops over a list of objects, skipping those with a zero first field.
- `check_object_regions`: Loops over a list of objects, skipping those with a zero first field.
- `update_internal_state`: Compares a byte at offset 31 of a structure in a1 with 2, and if it matches, checks a global variable against 7 to determine a value for d0, then uses d0 to jump to a long address or performs memory writes based on a pointer in a global variable.
- `process_data_table`: Reads a base address from a global variable and iterates through a table of structures stepping by 40 bytes, checking byte values and conditionally clearing flags, updating counters, or calling another routine, decrementing a loop counter each pass.
- `process_global_updates`: Clears a word in a structure, calls a routine, and if a byte at offset 40 of that structure is 10, increments a 16-bit counter in a global variable and calls another routine.
- `copy_data_block_2`: Calculates an offset into a table using two values from a structure, retrieves a pointer from that location, and copies several words and longwords from the pointed-to source into the structure.
- `update_state_on_match`: When a global flag is clear and a structure field is zero, it reads two coordinates and searches a table of regions until a match is found.
- `setup_hunk_parsing`: Loads two different addresses into register a4 and calls another routine.
- `parse_amiga_hunk_data`: Parses an AmigaDOS hunk header by reading the type code and size, then copies the associated data blocks into a destination table using a loop, skipping entries whose type code does not match.
- `write_nibbles_to_memory`: Writes four bytes to a memory location starting at offset 30 of a4, where each byte is formed by taking a byte from d6, splitting it into two nibbles, and masking the second nibble.
- `add_digits_with_carry`: Initializes a counter to seven, then repeatedly decrements two source pointers, adds their bytes plus a carry into a destination byte, wraps the result by subtracting ten and setting a carry when it exceeds nine, and stores the value until the counter reaches negative one.
- `update_entity_state_and_reset_counters_2`: Checks specific byte and word values within a structure to conditionally toggle a bit in a global variable, decrement a counter, or initialize a series of values in a table.
- `update_entity_state_and_reset_counters`: Decrements a byte at offset 20 of a structure and resets it to 30 if it reaches zero, or decrements it and resets it to 4 if it reaches zero.
- `insert_into_sorted_table`: Reads a pair of values, scans a sorted table by five-word steps until the pair fits or the counter underflows, shifts later elements up by five words when a gap exists, then stores the new pair and a third value into the destination slot before returning.
- `load_glyph_generation_parameters`: Loads two memory addresses into registers a0 and a1 and branches into another routine.
- `setup_glyph_generation`: Loads two memory addresses into registers, calls another routine, and subtracts a constant from the first address.
- `generate_glyph_combinations`: Indexes a table using two source bytes, then loops eight times combining adjacent table bytes with a shift and inversion to produce two output bytes per iteration, advancing the destination and source pointers before returning.
- `copy_interleaved_sources_2`: Loads a memory address into a register, reads a byte from a global variable, calls another routine, subtracts a constant from the address, and then branches to call that same routine again.
- `setup_interleaved_copy`: Loads a memory address into a register, reads a byte from a global variable, calls another routine, and subtracts a constant from the address.
- `copy_interleaved_sources`: Chooses between two source pointers based on a global flag, then loops eight times copying a byte and a word from each source into a destination buffer at fixed offsets, advancing both sources and the destination before returning.
- `update_glyph_display_parameters`: Copies two longwords from a global structure to a local structure and calls a routine to load glyph data from three different source tables.
- `load_glyph_data`: Loops three times, using bytes from a source list to index into a table and copying multiple bytes from the table into a destination structure at specific offsets.
- `process_object_collision_state`: Checks a condition based on a byte and a 16-bit value in a structure, then iterates through two object tables.
- `update_object_position_scaling`: Searches a table for the first empty slot, shifts subsequent entries up to fill the gap, and then stores a new object's position, scaled size values, and status flags into that slot.
- `calculate_offset_sums`: Performs four separate calculations by adding a constant to a value from a structure, subtracting another value from a structure, and setting bits in a status register if the result is negative, finally returning the sum of the two resulting values.
- `sort_and_update_object_table`: Sorts a linked list of structures by shifting longwords within each element and updates a secondary table by calculating new memory addresses based on a global offset.
- `shift_memory_block_forward`: Loops copying a longword from five positions ahead into the destination, shifting four adjacent longwords forward each step, until the pointer reaches a fixed address, then restores the saved pointer and returns.
- `setup_data_pointers`: Loads two different addresses into a4 and d2, loads a byte from a structure in a5 into d1, calls another routine, then loads a different address into a4, sets d2 to 2, and loads another byte from a structure in a5 into d1.
- `update_interleaved_data_state`: Decrements a counter at offset 26 of the structure in a4 when it is nonzero, then decrements a byte at offset 24 and, when it reaches zero, clears a flag at offset 19 and either clears a byte at offset 41 or decrements a byte at offset 28, after which it calls another routine twice.
- `update_table_accumulators`: Calculates two new 16-bit values based on a word from a structure in a5, then calls two routines to add these values to specific global variables or table entries.
- `update_game_state_and_decrement_counter`: Decrements a byte at offset 49 of a structure, calculates new values for several offsets based on the result, and updates a table of values.
- `accumulate_table_values`: If the byte at offset 58 of the structure in a3 equals 3, it reads a 16-bit value from a table indexed by the low three bits of a global variable at offset -2440 in a5, then adds that value to the non-zero 16-bit fields at offsets 122 and 122 plus the offset-48 or offset-24 delta in a3, and finally adds a second table value to the non-zero fields at offsets 134 and 134 plus the same delta.
- `reset_state_variables`: It increments a 16-bit counter at offset -2440 in a5, then clears the 16-bit field at offset -12012 in a5 and sets bit 13 of that field if bit 0 of the byte at offset -2441 is set, and finally initializes the structure in a6 by storing a full word at offset 68, the value 2544 at offset 64, and clearing the words at offsets 66, 100 and 102.
- `pack_bitfield_data`: It clears the four bytes at the address in a1, then stores the low byte of the sum of d2 and 38 at offset 0, a shifted nibble of d1 plus 143 at offset 1, and the low byte of their sum at offset 2, setting bit 2 of the byte at offset 3 when that sum overflows, and stores a 32-bit value from a table indexed by d5 at offset 80, a pointer at offset 84, and a scaled value of d4 at offset 88.
- `setup_address_and_constant`: Loads a longword into d1 by adding a value from a global variable to a constant, and stores a word into d2.
- `write_incremental_table`: It stores the high 16 bits of d1 into the 16-bit word pointed to by a1, then stores the low 16 bits of d1 into the word four bytes later, adds d2 to d1, and repeats this pair of stores eight times while advancing the pointer by four bytes each iteration.
- `update_sprite_attribute_table_2`: Computes horizontal and vertical positions, color, and animation indices for a set of sprites based on input values, performs bounds checking against a limit, and writes the results into a sprite attribute structure; if values are out of bounds, it marks the sprite as hidden by modifying a status byte.
- `initialize_structure_table`: Stores the constant 42 and the value 2 into two word fields of a structure in a6, writes a longword of all ones and a shifted value derived from the incoming d1 word into other fields, then runs a four-iteration loop that stores a running longword, a second incrementing longword, and a word into consecutive fields before overwriting them with fixed constants and returning.
- `initialize_coordinate_table`: Initializes a structure with specific values and iterates through a table of pointers, calling another routine if a value exceeds a threshold.
- `initialize_coordinate_table_alt`: Initializes a structure with specific values and iterates through a table of pointers, calling another routine if a value is below a threshold.
- `clear_structure_and_loop_table`: Clears a word at offset 66, a word at offset 64, a word at offset 100 and a longword at offset 68 of the structure in a6, then iterates through a table starting at 23424 stepping by 8 and calls another routine for each entry whose second word is less than 288.
- `clear_structure_and_loop_table_high`: Clears a word at offset 66, a word at offset 64, a word at offset 100 and a longword at offset 68 of the structure in a6, then iterates through a table starting at 23424 stepping by 8 and calls another routine for each entry whose second word exceeds 383.
- `update_entity_state_and_dispatch_2`: Iterates through a structure pointed to by a4, modifying various bytes and words based on comparisons of status flags and counters.
- `modify_and_call_routine`: Subtracts 16 from two words loaded from a table, exchanges d4 and d3, calls another routine twice while masking the result with 31 and adding it to the original words, then exchanges d4 and d3 again.
- `update_entity_state_and_dispatch`: Updates a structure's properties by modifying bytes at various offsets based on comparisons and conditional logic, then calls another routine and potentially jumps to a function pointer.
- `increment_bcd_byte`: Checks if a byte at offset 65 of a structure in a6 is less than 75 and if two other word values in that structure fall within specific bounds, then increments a byte at offset 97 of the structure and adjusts it as BCD.
- `return_immediately`: This routine returns immediately without modifying any data or registers.
- `initialize_game_state_and_display`: Checks a flag in a structure to determine whether to return immediately or proceed with initializing various global parameters, including setting specific values in a table based on a bit-count loop.
- `decrement_and_update_status`: When two short fields of a5 are zero, selects a decrement value based on the short at offset 9898, subtracts it from five consecutive bytes and adjusts two more bytes, then counts the set bits of a byte with a three-iteration loop and stores a longword chosen by that bit count into a longword field of a5.
- `copy_bcd_data`: Checks a condition based on a value in a5, then either performs BCD adjustments and copies data using two routine calls or copies a block of nine words from one location to another.
- `adjust_bcd_and_copy_data`: Performs BCD adjustments on bytes at offsets 97 and 98 of a structure in a4, calls another routine twice, and then either copies a block of nine words from a source to a destination or copies a block of nine words from a source to a destination based on bits in a flag byte at offset -28583 of a5.
- `convert_to_ascii`: Converts a single byte into two ASCII characters representing its high and low nibbles and stores them in a buffer.
- `modify_structure_state`: Clears a byte at offset 45 of a3, calls another routine, then if a byte at offset 19 is below 175 and a byte at offset 49 is zero it sets several flag and word fields and copies a longword and a byte between fields, otherwise it stores 255 into the byte at offset 45 and returns.
- `process_sprite_list`: Walks a list of pointers advancing by twelve bytes each time, and for each entry clears a block of sixteen-byte slots when a marker word is the end value or invokes other routines based on a byte field, stopping when the pointer exceeds a limit or a counter is zero.
- `check_flag_and_loop_decrement`: Returns immediately if a byte at offset -24530 of a5 is nonzero, otherwise it checks a byte at offset -30148 of a5 and enters a loop that decrements a value at offset 27712 of a0 until it reaches -1.
- `copy_object_attributes`: Writes the incoming d3 word into a byte of a destination table, computes an offset from that word, selects a value based on whether the incoming d1 word is below 800, stores a series of words and longwords into the table, copies several values from a source table, conditionally adjusts a byte, and performs arithmetic on a source word to fill the remaining fields.
- `process_input_buffer_and_update_state`: Iterates through a structure at a fixed address to check for specific control characters, updates internal counters and memory pointers based on character matches, and calls another routine to handle specific state transitions.
- `modify_structure_fields`: Updates two word fields at offsets 29 and 30 of a structure in a4 by adding 4 and 8 to their existing values, and sets bit 5 of a byte at offset 15 of the same structure.
- `calculate_position_logic`: Performs complex calculations involving global variables and structure offsets to determine a value, potentially updating sprite attributes or initializing a structure table.
- `update_position_data`: Checks if bits 1 and 2 of a byte in a global variable are clear, then sets bit 5 of a byte at offset 15 of a structure in a4.
- `update_sprite_attributes`: Updates two different byte values at offsets -2463 and -2464 of a5 based on a value in a4 and a byte in a5, potentially calling another routine to adjust the values based on bounds checks against a4.
- `calculate_and_store_index`: Calculates a value by dividing a longword by a value derived from a4, then uses a series of comparisons to select an index from 1 to 8, stores this index in a byte at offset 63 of a4, and performs further logic involving a call to another routine and updating a word at offset 12 of a4.
- `fill_coordinate_table_2`: Computes two values based on offsets 6 and 2 of a4, stores them in a6 at offsets 100 and 102, calculates a memory address using a multiplication and a table base, and then enters a loop that stores a pointer and several values into a structure in a6.
- `fill_coordinate_table`: Loads a word from offset 6 of a4, sets d7 to 4, stores a2 and a1 into longword fields of a6, then runs a four-iteration loop that stores a2, a1, and a computed word into consecutive fields, adds 24576 to both pointers, and decrements the counter before returning.
- `copy_buffer_and_set_register_bit`: Returns immediately if a byte at offset -28548 of a5 is nonzero, otherwise it copies two bytes from a pointer stored at offset 6650 of a5 and advances that pointer, and sets bit 4 of a byte at offset 44 of a4 if specific bits in the CIAA_PRA hardware register are set.
- `process_joystick_input`: Reads the joystick and button registers to detect directional movement and button presses, updating a status byte with the results.
- `update_scrolling_parameters`: Reads configuration values from a global structure and computes a scrolling offset, adjusting a frame counter by one each call and wrapping it back to a starting value when it underflows.
- `update_sprite_attributes_and_bitplanes`: Resets four sprite pointers to a default value, then shifts sprite data through a loop and updates the sprite attribute bytes based on the processed pixel bits, returning when the work is complete.
- `process_bitplane_data`: Multiplies two bytes from a source pointer by ten and adds the results to a table at offset 66896, then loops nine times.
- `copy_scaled_data`: Copies data from a source table to a destination buffer by shifting a value from a table and adding a constant offset, repeating this process five times in a loop.
- `refresh_text_buffer`: Fills a memory block with space characters, then if bit 4 of either of two status bytes is set, advances a pointer and increments a counter, returning early if the counter reaches five.
- `load_buffer_pointers`: Loads a pointer into a1, loads a pointer into a3, calls another routine, then loads a different pointer into a1 and a different pointer into a3.
- `decode_buffer_and_poll`: Decodes a compressed buffer by expanding each digit into a run of bytes, repeating until a zero marker ends the data, then repeatedly polls a status byte and calls another routine until that byte clears.
- `copy_inverted_data_blocks`: Loops four times over a table of short values, inverting the value at offset 6 when it matches a running character, then jumps to the computed address and copies nested blocks of data using the offsets stored in each entry.
- `decompress_bit_stream_block`: Decompresses a bit-stream into a destination buffer using a variable-length coding scheme, where bits are read to determine either literal byte values or run-length repetitions.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `set_globals_and_custom`: Loads two addresses into two global variables, stores a value into a third global variable, writes a control word to a custom chip register, and returns.
- `mask_interrupt_request`: Saves registers, checks the interrupt status register, and if bit 6 is set, masks bit 6 of the interrupt mask register before restoring registers and returning from interrupt.
- `call_if_flag_set`: Reads a byte from a custom chip register into a register, stores it in a global variable, and calls a routine from a pointer in a global variable if bit 0 of that byte is set.
- `write_interrupt_register`: Writes a word to the interrupt mask register and returns.
- `load_and_poll_structure`: Clears bits 3 and 7 of a CIA register, calls another routine, and then loads a longword and two bytes from a structure into global variables.
- `clear_and_increment_flag`: Clears bit 1 of a CIA register, increments a global word, calls another routine, and then branches based on the carry flag.
- `loop_cia_bit_clear`: Calls another routine, clears and then sets bit 0 of a CIA register, and then enters a loop that repeats a number of times specified by a value in a global word.
- `wait_for_cia_bit`: Polls bit 5 of a CIA register in a loop until it is cleared, then returns.
- `handle_joystick_vector`: Sets a custom chip register to a vector value, then loops while a joystick button bit is set, setting a port bit and calling another routine, and finally clears the register before returning.
- `process_shifted_value`: Compares a shifted version of a global variable with a shifted version of a value in d7, sets a bit in a CIA register, and calls another routine if the comparison fails.
- `read_shifted_with_flag`: Sets a flag in a custom chip register, reads a value from a global variable, clears the flag, and returns the value shifted right by one.
- `read_track_and_scan_sync`: Waits for a status bit to clear, configures disk-related custom chip registers, then scans a buffer for sync words in a loop, deinterleaves each found sector, and copies the data into a destination buffer until the count reaches zero.

### logged disk load from Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf boot-block load

- `execute_graphics_pipeline`: Calls several subroutines to process graphics data, including clearing blocks, projecting points, and performing blits, then checks a bit in a CIA register to decide whether to store a value or return.
- `clear_block_and_init_sprite`: Writes a register group including the copper pointer, DMA control, interrupt enable and sprite control, zeroes a block of longwords between two addresses in a loop, and stores several table offsets and counts.
- `tiny_ori_ori_rts`: Small routine of 3 instructions that merges 208 into d3, then merges 108 into d0, then returns.
- `call_helper_then_bad_data`: Calls one unnamed routine and then runs into undecodable bad instruction data, so only that call is recovered.
- `fill_three_words_per_pass`: Repeatedly calls a helper three times per pass, writing the first two results with zeroed companions and storing the third masked to its low nine bits, until a count register reaches zero.
- `clear_bit_in_two_planes`: Computes a byte address from two inputs using a row stride of 48 bytes, then clears one selected bit in each of two bitplanes.
- `project_and_clip_points`: Walks 70 table entries, divides each stored position by a per-entry value to get projected coordinates, reseeds an entry from two helper values when it runs out or falls off-screen, and stores valid coordinates into an output list.
- `advance_linear_congruential_seed`: Multiplies a stored seed by a global value, adds a fixed constant and stores the result.
- `call_helper_per_string_byte`: Calls a helper for each byte of a zero-terminated byte string, stopping at the terminator.
- `tiny_ori`: Small routine of 1 instruction that merges 176 into d3.
- `call_helper_per_string_byte_2`: Calls a helper for each byte of a zero-terminated byte string, stopping at the terminator, with the same loop as its neighbour.
- `draw_eight_row_glyph`: Looks up eight bytes of character bitmap from a font table using the character code minus thirty-two, then writes each byte to a bitplane row with a stride of 96 bytes and a second copy 48 bytes further on.
- `cycle_four_step_counter`: Advances a four-step counter; on wrap it resets to zero and calls a helper, and it calls a second helper on every call.
- `start_fixed_blit_after_wait`: Waits until the blitter is idle, then writes its control, pointer, modulo and size registers for one fixed blit.
- `advance_wrapping_string_pointer`: Moves a pointer forward one byte through a zero-terminated table, wrapping to the base at the terminator, then calls a helper.
- `ramp_value_through_three_states`: Advances two wrapping counters, then runs a three-state controller that ramps a value up by one thousand until it passes a threshold, holds it for seventy steps, then ramps it down and resets at zero.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d2.
- `wait_raster_then_start_blit`: Waits for a particular raster position, then waits for the blitter to become idle and programs its control, destination pointer, modulo and size registers for one blit.
- `call_helper_when_pairs_differ`: Walks a list of index pairs up to the terminator value of 99, and for each pair compares two derived table values, calling a helper when they differ.
- `compact_pair_list_to_terminator`: Copies a list of three-word records into a packed list of six-byte entries, keeping two of the three words per record, then writes the terminator value of 99 after the last entry.
- `call_two_helpers_per_record`: For each three-word record up to the terminator value of 99, calls two helper routines.
- `call_helper_per_record`: For each three-word record up to the terminator value of 99, calls one helper routine.
- `scale_three_component_vector`: Scales three stored 16-bit components by a common fixed-point factor derived from a helper-supplied value and a multiplier, writing the results back in place.
- `call_three_helpers_in_order`: Calls three unnamed routines in sequence and returns; the listing shows no other work.
- `rotate_coordinate_pair`: Reads two coefficients from a table at an offset chosen by its input and applies a rotation-style update to two stored 16-bit values, doubling each product in fixed point.
- `set_pixel_and_line_deltas`: Sets one bit in a bitplane, computes a byte address from the position using a row stride of 48, works out the absolute distances along each axis to select a line octant, then writes the blitter pointer.
- `setup_line_blit_octant`: Computes a blitter pointer from a position with a row stride of 48, derives absolute axis distances and an octant code, then waits for the blitter and writes the line distance and octant.
- `draw_blitter_line_segment`: Computes a destination pointer, the absolute distances along each axis and an octant code, then waits until the blitter is idle and programs its data, masks, modulos, control and size registers to draw one line segment.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_to_command_dispatcher`: Performs a long jump to another routine.
- `jump_to_disable_dma`: Calls another routine via a long jump.
- `disable_dma_and_set_flag`: Disables DMA by writing to two custom chip registers, stores the value 1 into a global variable, and jumps to another routine.
- `toggle_global_bit_zero`: Toggles bit 0 of a byte in a global variable and returns via a jump.
- `toggle_and_update_buffer_2`: Checks a global flag, then uses the low two bits of a register to index into a table of pointers.
- `toggle_and_update_buffer_3`: Uses the low two bits of a register to index into a table, toggles a byte at offset 61 of the pointed-to structure, and if that byte is zero, copies data from a secondary structure into a target buffer and updates a global variable.
- `update_state_flags`: Clears a global word, conditionally copies a byte from one global variable to another and clears the source, and then sets two global bytes to one if certain conditions are met before calling another routine.
- `update_state_two`: Clears a 16-bit variable, copies a byte from one location to another if it is non-zero, and if two specific conditions are met, sets a 16-bit variable to 2, sets a byte to 1, and sets a flag to 1 before returning 2.
- `update_state_three`: Clears a 16-bit variable, copies a byte from one location to another if it is non-zero, and if two specific conditions are met, sets a 16-bit variable to 3, sets a byte to 1, and sets a flag to 1 before returning 3.
- `update_state_four`: Clears a 16-bit variable, copies a byte from one location to another if it is non-zero, and if two specific conditions are met, sets a 16-bit variable to 4, sets a byte to 1, and sets a flag to 1 before returning 4.
- `update_state_five`: Clears a 16-bit variable, copies a byte from one location to another if it is non-zero, and if two specific conditions are met, sets a 16-bit variable to 5, sets a byte to 1, and sets a flag to 1 before returning 5.
- `update_state_six`: Clears a 16-bit variable, copies a byte from one location to another if it is non-zero, and if two specific conditions are met, sets a 16-bit variable to 6, sets a byte to 1, and sets a flag to 1 before returning 6.
- `jump_and_set_d0`: Jumps to another routine, sets register d0 to 1, and then branches to another routine.
- `branch_with_d0_setup`: Sets d0 to either 2 or 3 and branches to another routine.
- `load_and_branch_d0_values`: Loads the value 4, 5, or 6 into register d0 and branches to another routine.
- `load_and_branch_d0_value`: Loads the value 7 into register d0 and branches to another routine.
- `initialize_four_data_structures`: Saves registers a0 through a3 on the stack, then makes four consecutive calls to another routine, each time loading the address of a different source table into register a0 before the call, and finally restores the saved registers and returns.
- `initialize_data_structure`: Clears numerous byte and word fields in a structure via a pointer, stores a constant into a field, then copies data from a source table into a destination buffer and returns.
- `parse_command_stream`: Parses a byte command stream, dispatching on opcodes in decimal 128 through 132 to update sprite attribute tables, reposition data-table pointers, and set control flags in the structure, otherwise it returns.
- `disable_dma_and_clear`: Writes control words to two custom chip registers, stores a value into a global variable, and returns.
- `toggle_flag_bit`: Toggles bit 0 of a byte in a global variable and returns.
- `toggle_and_update_buffer`: Uses the low two bits of a register to index into a table, toggles a byte in the pointed-to structure, and conditionally copies data from one location to another based on that byte before returning the original register value.
- `update_sprite_attribute_tables`: Toggles a flag through the DMA control register and clears four byte fields when a counter is nonzero.

### logged disk load from Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf

- `call_and_jump`: Calls a routine and then jumps to another routine.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `initialize_four_data_structures_2`: Saves registers a0 through a3 on the stack, then makes four consecutive calls to a routine that initializes a data structure, each time loading the address of a different source table into register a0 before the call, and finally restores the saved registers and returns.
- `clear_and_copy_structure`: Writes zeros to a series of byte and word offsets within the structure pointed to by a0, then copies a pointer value from that structure and uses it to fill several adjacent fields with data from the pointed-to location.
- `process_command_stream`: Initializes a command-processing structure by copying a pointer pair from a linked structure and setting a flag, then iterates through a byte stream, executing commands identified by markers 128 through 129, loading longwords from tables, updating pointers and counters, and returning when it encounters markers 131 or 133.
- `update_conditional_byte`: If a global byte is zero, it copies the value from another global byte into it, then sets a global word to one.
- `manage_dma_and_state`: Reads several global bytes and toggles a state flag, resetting a group of four longword globals to zero when a condition is met.

### logged disk load from Battle Squadron - The Destruction of the Barrax Empire! (1989)(Innerprise Software)[cr TIC][t Freestyle][a3].adf

- `perform_io_request`: Fills an I/O request structure with a request type of two, a length of 8192, a destination, and an offset, calls a library routine, calls another routine, and then copies 32 words from a table to a destination using a loop before jumping to an address.
- `trigger_trap_zero`: Loads the address of a constant into register a0, stores that address into a longword at an offset of 128 from the program counter, and then executes a trap instruction.
- `initialize_hardware_and_copy`: Disables interrupts, sets several DMA control registers to maximum values, updates a DMA address, and copies a longword from one memory location to another before jumping to a subroutine.
- `read_block_via_library_call_2`: Fills an I/O request with a request type of two, an 8192-byte length, a destination and an offset, calls a library routine at offset minus 456 to run it, then calls one unnamed routine and clears a global four times.
