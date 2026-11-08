# Game 2447 (links - the challenge of golf v2.00 (1990)(access)(disk 1 of 3)[cr atx][a][hd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02447). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (908784 top-level bytes in 39 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 86 of 86 named (0 author symbols), 86 of 86 described (8 by rule, 74 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 805 of 805 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3349 of 3349 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 429449 |
| code | 10050 |
| data | 442614 |
| padding | 9775 |
| unused | 16896 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][a][HD].adf:GOLF | 2 | 267 | 8 |
| run | 2 | 117 | 2 |
| INTRO.EXE | 5 | 251 | 6 |
| INSERT | 2 | 267 | 8 |
| XLAN | 2 | 267 | 8 |
| Disk-Validator | 1 | 332 | 1 |
| Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][a][HD].adf:GOLF.H | 2 | 267 | 8 |
| Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][a][HD].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 809 | 24 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 754 | 20 |

## Cases handled

- links - the challenge of golf v2.00 (1990)(access)(disk 1 of 3)[cr atx][a][hd]/Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][a][HD].adf:1.LZ: high-entropy data file (7.72 bits/byte), classified as packed data
- links - the challenge of golf v2.00 (1990)(access)(disk 1 of 3)[cr atx][a][hd]/Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][a][HD].adf:DS.LZ: high-entropy data file (7.74 bits/byte), classified as packed data
- links - the challenge of golf v2.00 (1990)(access)(disk 1 of 3)[cr atx][a][hd]/Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][a][HD].adf:ECARD.LZ: high-entropy data file (7.90 bits/byte), classified as packed data
- links - the challenge of golf v2.00 (1990)(access)(disk 1 of 3)[cr atx][a][hd]/Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][a][HD].adf:IEW.LZ: high-entropy data file (7.76 bits/byte), classified as packed data
- runtime segment an address: 388 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 91 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- links - the challenge of golf v2.00 (1990)(access)(disk 1 of 3)[cr atx][a][hd]/Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][a][HD].adf:C/INTRO.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- INTRO.EXE

## Copy protection found

- none found

## Functions


### Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][a][HD].adf:GOLF

- `process_amiga_hunk_data`: Parses a data stream by identifying specific hunk type codes, allocating memory, and performing relocations or data copies.
- `decrement_and_call_library`: The routine decrements a value in a loop, calls a routine via a pointer, and uses the result to update a global variable before calling another routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_bitstream_data`: Loads a long value from a pointer in a0, performs several calls to other routines, and uses bitwise shifts and arithmetic to manipulate values and store a byte at a location pointed to by a3, then loops based on a comparison between a3 and a2.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bitstream_with_callbacks`: The routine performs bitwise shifts and rotations on a long value in a loop, calls several other routines, and uses the results to manipulate a stack or a buffer.
- `perform_trap_operation`: Moves a 16-bit word from an offset of 3085 of the structure in a2 to d4, performs a bitwise OR on d3 with 56, calls a trap, and performs a bitwise OR of d0 into d1.

### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_register_and_branch`: Moves the contents of d1 to d0 and then branches to another location.

### INTRO.EXE

- `init_and_check_pattern`: Invokes several routines and loops three times, then clears a memory location and, when a value matches a specific pattern, stores it and zeroes a global variable.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `expand_and_update_data`: The routine calls a routine multiple times to load constants and addresses, then enters a loop that calls another routine to expand data and adds the results to a destination buffer.
- `load_address_and_constant`: Loads the address of a constant into a1 and sets d0 to 2.
- `expand_bitstream_data`: Reads a compressed bitstream from a source buffer and expands it into a destination buffer using a variable-length decoding scheme.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### INSERT

- `process_amiga_hunk_relocations`: Parses a data stream containing AmigaDOS hunk types, performing memory allocation and copying data segments while applying relocations by calling another routine to resolve addresses.
- `process_hunk_data`: Decrements d1 in a loop, performs pointer arithmetic on a1, calls two library routines, and stores a calculated long value into a structure in a4.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `copy_bitstream_data`: Reads a length value from a pointer in a0, calculates a destination address based on bit shifts, and enters a loop to copy bytes from a source to a destination using a variable number of iterations determined by the bitstream content.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `subtract_word_values`: Subtracts a 16-bit value in d4 from a 16-bit value in d0.
- `extract_bitstream_bits`: Performs bitwise extraction from a bitstream by reading a long value from a pointer in a0 and using a bit count in d4 to shift and rotate bits into a result.
- `execute_trap_instruction`: Moves a 16-bit word from an offset of 3213 in a6 to d4, performs a bitwise OR on d1 with 56, calls a trap, and then performs a bitwise OR on d1 with the contents of d0.

### XLAN

- `process_amiga_hunk_relocations_2`: Parses a data stream containing AmigaDOS hunk types, performing memory allocation and data relocation for specific hunk codes.
- `allocate_and_relocate_hunk`: Calculates a size from a pointer and a value, calls a memory allocation routine, and then performs a relocation or data copy based on the hunk type.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_bitstream_data`: Reads a length value from a bitstream, calculates a destination pointer offset, and enters a loop to extract and store variable-length sequences of bytes into a buffer.
- `check_shifted_flag_3`: Tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `move_word_to_register`: Moves a 16-bit word from a memory location pointed to by a register into another register.
- `extract_variable_bitstream_bits`: Performs a bitwise extraction loop that reads a long value from a pointer in a0, uses a bit count in d4 to perform shifts and rotations, and updates a value in d5.
- `execute_trap_instruction_2`: Moves a 16-bit word from an offset of 3213 in a6 to d4, performs a bitwise OR on d2 with 88, calls a trap, and then performs a bitwise OR on d1 with the contents of d0.

### Disk-Validator

- `update_score_display_and_buffer`: Calculates a value from a global variable, converts it to an ASCII character, and stores it in a table.

### Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][a][HD].adf:GOLF.H

- `parse_amiga_hunk_data`: Parses a data stream by identifying specific hunk type codes and performing memory allocation and relocation.
- `allocate_and_relocate_hunk_data`: Calculates a size from a data stream, calls a memory allocation routine, and performs a relocation by subtracting a pointer value from a destination address.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_bitstream_data_2`: Reads a length value from a pointer in a0, then uses it to calculate an offset and a count for a loop.
- `test_shifted_flag`: Loads a longword from the address pointed to by a0 into d5, performs a logical right shift on d5 by the amount specified in d4, and returns early if the result is non-zero; otherwise it returns.
- `subtract_word`: Subtracts the value in register d4 from register d0.
- `process_bitstream_data_2`: Performs bitwise shifts and rotations on a long value in a loop, loading new values from a pointer in a0 when the current value becomes zero, and calls several other routines.
- `execute_trap_and_or`: Loads a word from an offset of a2 into d4, performs a bitwise OR on d3 with 56, calls a trap, and then performs a bitwise OR of d0 into d1.

### Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][a][HD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_state`: Obtains a base pointer from a global variable to initialize several global variables and a table of offsets, then configures hardware registers and enters a loop that polls a status flag while maintaining a counter and a small state machine.
- `fetch_supervisor_value`: The routine checks a flag in a global variable and returns a masked value from that variable when the flag is clear.
- `decode_and_fill_table`: The routine decodes a null-terminated string stored in a global variable by counting the distance from the start of the variable for each character and storing the distance doubled back into the variable.
- `decode_and_fill_table_2`: The routine decodes a global byte array by replacing each byte with twice the distance to the first matching byte in the same array, then overwrites the same storage with a 48-entry short table whose values are computed from an index using division and modulo arithmetic.
- `process_enemy_table`: Checks a 16-bit flag at offset 34 of a structure; if non-zero, it iterates through a nested table structure, using a global table to index data and calling another routine for each bit set in a byte, while updating two byte pointers with offsets derived from the structure.
- `set_bit_pair`: The routine sets a bit in two byte pointers and their offsets, using an input value to select the bit position, and repeats the operation for a second position derived from the input.
- `merge_enemy_bitmask_tables`: Checks a flag in a structure and returns if it is clear; otherwise, it iterates over a count of entries and an inner loop of twenty entries, reads an index from a byte table via a pointer, looks up a byte from a global variable table using that index, and ORs the byte into several fixed offsets in two parallel blocks pointed to by two base addresses.
- `toggle_and_update_pointers`: Toggles bit 0 of a 16-bit word at offset 4 of the structure in a5; if the result is zero, it updates several 16-bit values in a table using a base address from a global variable, otherwise it updates them using a different base address from another global variable.
- `execute_and_increment_list`: Checks if a 16-bit value at offset 54 of the structure in a5 is non-zero; if so, it calls a routine via a pointer found by adding offset 16 of a5 to a base address, then clears 32-bit values at offsets 18 and 32 and increments a 16-bit value at offset 16 if offset 18 was zero.
- `update_sprite_parameter_table`: Calculates offsets and dimensions based on values from a structure in a5 and populates a series of sprite parameter entries in a structure in a6 using values from two tables.
- `calculate_and_update_parameters`: Calculates a value using a table lookup and arithmetic, then populates a structure in a6 with various pointers and values based on a bit test, and finally calls another routine and updates a 16-bit field in a structure in a5.
- `update_position_and_mark_short`: The routine subtracts a 16-bit field from another 16-bit field twice, negates the subtracted field, and writes a constant byte to one byte field in the structure.
- `calculate_and_set_parameters`: Checks a 16-bit value at offset 24 of the structure in a5; if zero, it performs calculations involving a 16-bit value from offset 22 and populates a structure in a6 with various 16-bit and 32-bit values based on the result, then decrements offset 22 and potentially returns.
- `populate_parameter_structure`: Checks bit 6 of a 16-bit value at offset 2 of the structure in a6; if clear, it populates a structure in a6 with several 16-bit and 32-bit values derived from a global variable and a pointer, then stores a byte at offset 18 of the structure in a5.
- `load_address_and_branch`: Loads a constant into a0 and branches to another routine.
- `load_address_and_branch_2`: Loads a constant value into a6 and branches to another routine.
- `set_structure_fields`: Writes the constant value 65532 to offset 16 of the structure in a5 and writes the constant byte 255 to offset 18 of the structure in a5.
- `update_nibbles_and_state`: Checks offsets 52 and 54 of a structure in a5; if 54 is non-zero, it iterates through a table to increment nibbles in words; if 56 is non-zero, it writes several constants to a memory range and calls another routine twice to update global pointers.
- `process_nibbles_and_update_state`: Performs bitwise operations on d0 and d3 to decrement specific nibbles, and then either updates a global pointer and decrements a counter in a structure in a5, or executes a series of calls to other routines to update several global pointers based on values from a structure in a5.
- `adjust_high_nibble_of_word`: The routine takes a word from d0, extracts bits in the high byte range, subtracts a constant if non-zero, and returns the modified word with the extracted bits replaced.
- `adjust_middle_nibble_of_word`: The routine takes a word from d0, extracts a nibble in the middle of the word, subtracts a constant if non-zero, and returns the modified word with the extracted nibble replaced.
- `adjust_low_nibble_of_word`: The routine takes a word from d0, extracts the lowest 4 bits, subtracts 1 if they are non-zero, and returns the modified word with the extracted bits replaced.
- `increment_and_wrap_counter`: Masks a global variable, and if set performs several calls to other routines, increments a short field in a structure pointed to by a5, and wraps it to 200 when it underflows.
- `tiny_sub_2`: Small routine of 1 instruction that subtracts d0 from -(a2).

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_score_table`: Validates a header in a data block and, if present, copies fields from that block into a global table and builds derived offsets and sizes for entries.
- `reset_audio_channels`: The routine stores a pointer into a global variable and then clears four audio channel volume registers.
- `update_audio_and_process_table`: Updates a global word by adding a value to it and clamps the result between zero and sixty-four, then iterates through a table to call a routine four times with varying parameters, calculating and storing scaled values into specific global locations.
- `write_globals_and_table`: The routine writes constants and a value from a global variable into global variables, adds a constant to a word in a table pointed to by a pointer in memory, and touches hardware registers CIAB_CRA, CIAB_ICR, DMACON and INTREQ.
- `write_globals_and_table_2`: The routine writes values from several global variables into global variables, writes a constant into a global variable, stores a value from a global variable into a table entry, and touches hardware registers AUD0LCH, AUD0LEN, AUD1LCH, AUD1LEN, AUD2LCH, AUD2LEN, AUD3LCH, AUD3LEN, CIAB_ICR, INTREQ and custom+0.
- `push_data_and_load_block`: Pushes several bytes onto the stack, clears a word at offset 532 of a2, and if a value derived from a4 meets a specific threshold, it updates a counter at that offset and copies data from a calculated source into a4.
- `update_structure_data`: Ors a word from d7 into a value at offset 502 of a2, and if a word at offset 532 of a2 is non-zero, it copies several words and longs from a calculated source address into a structure in a4.
- `load_structured_data_2`: Calculates an offset from d7, loads a long value from a pointer into a3, and then copies several words and longs from the source to a structure in a4.
- `load_structured_data`: Clears a long value at offset 14 of a structure in a4, then if d1 is non-zero, it reads a word from a4, performs bitwise operations, and enters a loop that loads multiple words from a source into a destination structure and branches back to the start of the routine.
- `branch_data_loading`: Sets up a pointer in a0, and depending on the values of d0 and d1, either branches to another routine, copies a word from d0 into a4, or performs a data transfer from a calculated source into a4.
- `load_scaled_data_block`: Ors a word from d7 into offset 790 of a2, calculates a source address based on d0 and a0, and copies a long, a word, a long, and several other values from the source to a3 and a4 depending on a comparison between a word and a scaled byte.
- `load_scaled_word_variant`: Copies a word from a0 to a structure in a4, loads a long from a0 into a3, and then copies several words and longs from a0 to a3 and a4.
- `load_scaled_data_with_offset`: Clears a long at offset 14 of a4, and if d1 is non-zero, it performs a series of data transfers from a calculated source address to a3 and a4, including several longs and words.
- `load_scaled_word_offset`: Loads a long from a0 plus two bytes into a3, copies a word from a0 to a3, and then transfers several words and longs from a0 to a3 and a4.
- `load_scaled_word`: Decrements d1 and doubles it, loads a word from a4 into d0, calculates an offset by subtracting 1 from d0 and shifting left by 5, loads a word from a table at that offset into a4, and stores a word from a4 into offset 2 of the structure in a4 before branching to another routine.
- `dispatch_function_pointer`: Calls a routine, clears d0, loads a byte from offset 4 of the structure in a4 into d0, doubles d0, loads a word from a table at a pc-relative address into d0, and jumps to the address in d0.
- `write_globals_and_table_3`: Performs four iterations of calling a routine, loading a byte from offset 4 of a structure into d0, doubling d0, and jumping to a table entry, then calculates a scaled value using a word from a structure and a value from a table to store in a field of another structure, writes a routine pointer to a structure, and writes the constant 25 to a global variable.
- `update_timer_and_pointer`: The routine updates a byte field by adding an offset from a global table using an index stored in the structure, then checks the result for overflow.
- `update_character_ability_index`: The routine increments a short counter stored in a structure and, when the counter matches an input value, copies a word from a register into two locations derived from two pointers, updates a pointer to a table entry selected by the counter, and propagates fields from that entry into the structure.
- `process_structure_field`: The routine checks a byte at an offset in a structure, uses it as an index into a table to retrieve a value, and performs bitwise operations on that value before using it to calculate an offset for a pointer.
