# Game 2448 (links - the challenge of golf v2.00 (1990)(access)(disk 1 of 3)[cr atx][hd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02448). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

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
| Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][HD].adf:GOLF | 2 | 267 | 8 |
| run | 2 | 117 | 2 |
| INTRO.EXE | 5 | 251 | 6 |
| INSERT | 2 | 267 | 8 |
| XLAN | 2 | 267 | 8 |
| Disk-Validator | 1 | 332 | 1 |
| Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][HD].adf:GOLF.H | 2 | 267 | 8 |
| Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][HD].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 809 | 24 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 754 | 20 |

## Cases handled

- links - the challenge of golf v2.00 (1990)(access)(disk 1 of 3)[cr atx][hd]/Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][HD].adf:1.LZ: high-entropy data file (7.72 bits/byte), classified as packed data
- links - the challenge of golf v2.00 (1990)(access)(disk 1 of 3)[cr atx][hd]/Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][HD].adf:DS.LZ: high-entropy data file (7.74 bits/byte), classified as packed data
- links - the challenge of golf v2.00 (1990)(access)(disk 1 of 3)[cr atx][hd]/Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][HD].adf:ECARD.LZ: high-entropy data file (7.90 bits/byte), classified as packed data
- links - the challenge of golf v2.00 (1990)(access)(disk 1 of 3)[cr atx][hd]/Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][HD].adf:IEW.LZ: high-entropy data file (7.76 bits/byte), classified as packed data
- runtime segment an address: 388 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 91 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- links - the challenge of golf v2.00 (1990)(access)(disk 1 of 3)[cr atx][hd]/Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][HD].adf:C/INTRO.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- INTRO.EXE

## Copy protection found

- none found

## Functions


### Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][HD].adf:GOLF

- `process_amiga_hunk_relocations`: Iterates through a data stream to identify and process AmigaDOS hunk types, performing memory allocations, copying data segments, and applying relocations by calling another routine to resolve addresses.
- `decrement_and_call_library`: Decrements a value in a loop, calls a routine via a pointer, and if a condition is met, calls another routine and stores a result in a structure.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_bitstream_data`: Loads a long value from a pointer in a0, then enters a loop that calls another routine and performs bitwise operations and memory moves based on the loaded value.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bitstream_and_stack`: Performs bitwise shifts and rotations on a long value in a loop, calls several routines including check_shifted_flag and tiny_sub, and pushes bytes onto a stack based on a counter.
- `perform_trap_operation`: Moves a 16-bit word from an offset of 1549 in a2 to d4, performs a bitwise OR on d3 with 56, calls a trap, and performs a bitwise OR of d0 into d1.

### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_and_branch`: Moves the value from d1 to d0 and then branches.

### INTRO.EXE

- `init_and_check_pattern`: Invokes several routines and loops three times, then clears a memory location and, when a value matches a specific pattern, stores it and zeroes a global variable.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `expand_and_store_bitstream`: Calls load_address_and_constant and decode_and_expand_bitstream multiple times, uses a loop to add values to a buffer, and calls a routine via a pointer if a condition is met.
- `load_address_and_constant`: Loads an address from an offset of 508 in the program counter into a1 and sets d0 to 2.
- `decode_and_expand_bitstream`: Reads a bitstream from a source buffer, performing bitwise shifts and additions to expand data based on a variable length and bit-depth specified in a table.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### INSERT

- `parse_amiga_hunk_data`: Parses a data stream containing AmigaDOS hunk types to perform memory allocation and relocation.
- `process_hunk_relocations`: Decrements d1 in a loop, and if not zero, performs pointer arithmetic, calls two library routines, and stores a value in a structure in a4.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `expand_compressed_bitstream`: Reads a length value from a bitstream in a0, calculates a target memory address in a3 based on bit shifts, and performs a loop to write bytes into memory.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `subtract_word_values`: Subtracts the word in register d4 from the word in register d0.
- `extract_bitstream_bits`: Extracts a specified number of bits from a long value pointed to by a0, performing bitwise shifts and rotations to process the data, and calls check_shifted_flag and tiny_sub within a loop.
- `execute_trap_with_data`: Moves a word from an offset of 3085 of the structure in a2 to d4, performs a bitwise OR on d1 with 56, executes a trap, and then performs a bitwise OR on d1 with d0.

### XLAN

- `process_amiga_hunk_relocations_2`: Parses a data stream containing AmigaDOS hunk types, performing memory allocations and copying data segments while applying relocations.
- `process_hunk_relocation_data`: Iterates through a data stream to identify and process AmigaDOS hunk types, performing memory allocations, copying data segments, and applying relocations.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `expand_bitstream_data`: Reads a length value from a stack-based pointer, calculates a destination address by shifting the value, and enters a loop that writes bytes to memory using a count derived from the bitstream.
- `check_shifted_flag_3`: Tests whether a logical right shift of d4 by d5 bits results in a non-zero value and returns early if it is; otherwise it returns.
- `move_word_to_d0`: Moves a 16-bit word from register d4 into register d0.
- `extract_bitstream_bits_2`: Extracts a specified number of bits from a long value pointed to by a0 and performs bitwise rotations and shifts to populate a bit buffer.
- `execute_trap_with_different_data`: Moves a word from an offset of 3085 of the structure in a2 to d4, performs a bitwise OR on d2 with 88, executes a trap, and then performs a bitwise OR on d1 with d0.

### Disk-Validator

- `update_score_and_display_data`: Calculates a value based on a global variable and stores it in a structure, then enters a loop to clear a range of memory in a table.

### Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][HD].adf:GOLF.H

- `parse_amiga_hunk_data_2`: Parses a bitstream of data by iterating through memory to identify specific hunk type codes, performing memory allocations, and applying relocations or data moves based on the identified types.
- `process_hunk_relocation_segment`: Iterates through a data stream to process AmigaDOS hunk types, performing memory allocations, copying data segments, and applying relocations.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_bitstream_data`: Reads a byte from a source pointer in a0, determines a count and an offset via bitwise shifts, and then enters a loop to copy a sequence of bytes from a source address to a destination address in a3.
- `check_shifted_flag_4`: Loads a longword from the address pointed to by a0 into d5, then performs a logical right rotation of d5 by the value in d4; if the resulting value is zero, it returns, otherwise it returns early.
- `subtract_word`: Subtracts the value in register d4 from register d0.
- `process_bitstream_data_2`: Loads a long value from a pointer in a0, then enters a loop that calls another routine and performs bitwise operations and memory moves.
- `execute_trap_and_or`: Moves a word from a table at an offset of a2 into d4, performs a bitwise OR on d3 with 56, calls a trap, and then performs a bitwise OR of d0 into d1.

### Links - The Challenge of Golf v2.00 (1990)(Access)(Disk 1 of 3)[cr ATX][HD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_state`: Obtains a base pointer from a global variable to initialize several global variables and a table of offsets, then configures hardware registers and enters a loop that polls a status flag while maintaining a counter and a small state machine.
- `fetch_supervisor_value`: The routine checks a flag in a global variable and returns a masked value from that variable when the flag is clear.
- `decode_and_fill_table`: The routine decodes a null-terminated string stored in a global variable by counting the distance from the start of the variable for each character and storing the distance doubled back into the variable.
- `decode_and_fill_table_alt`: The routine decodes a global byte array by replacing each byte with twice the distance to the first matching byte in the same array, then overwrites the same storage with a 48-entry short table whose values are computed from an index using division and modulo arithmetic.
- `process_enemy_table`: Checks a 16-bit flag at offset 34 of a structure; if non-zero, it iterates through a multi-level table structure, using values from a global table to index data and calling another routine for each bit set in a byte, while updating two byte pointers with offsets derived from the structure.
- `set_bit_pair`: The routine sets a bit in two byte pointers and their offsets, using an input value to select the bit position, and repeats the operation for a second position derived from the input.
- `merge_enemy_bitmask_tables`: Checks a flag at offset 32 of a structure; if set, it iterates through two nested loops to read indices from a table, look up bytes from a global table, and OR those bytes into several fixed offsets within two parallel memory blocks.
- `toggle_and_update_pointers`: Toggles bit 0 of a 16-bit word at offset 4 of the structure in a5; if the result is zero, it updates several 16-bit values in a table using a base address from a global variable, otherwise it updates them using a different base address from another global variable.
- `execute_and_increment_list`: Checks if a 16-bit value at offset 54 of the structure in a5 is non-zero; if so, it calls a routine via a pointer found by adding offset 16 of a5 to a base address, then clears 32-bit values at offsets 18 and 32 and increments a 16-bit value at offset 16 if offset 18 was zero.
- `update_sprite_parameter_table`: Calculates offsets and dimensions based on values from a structure in a5 and populates a series of sprite parameter entries in a structure in a6 using values from two tables.
- `calculate_and_update_structure`: Calculates a value based on two 16-bit fields from a table and a global variable, then populates a structure in a6 with various 16-bit and 32-bit values before calling another routine and updating a 16-bit field in a5.
- `update_position_and_mark_short`: The routine subtracts a 16-bit field from another 16-bit field twice, negates the subtracted field, and writes a constant byte to one byte field in the structure.
- `calculate_and_set_parameters`: Checks a 16-bit value at offset 24 of the structure in a5; if zero, it performs calculations involving a 16-bit value from offset 22 and populates a structure in a6 with various 16-bit and 32-bit values based on the result, then decrements offset 22 and potentially returns.
- `populate_parameter_structure`: Checks bit 6 of a 16-bit value at offset 2 of the structure in a6; if clear, it populates a structure in a6 with several 16-bit and 32-bit values derived from a global variable and a pointer, then stores a byte at offset 18 of the structure in a5.
- `load_address_and_branch`: Loads a constant into a0 and branches to another routine.
- `load_address_and_branch_2`: Loads a constant value into a6 and performs a short branch to another routine.
- `set_structure_values`: Writes a 16-bit constant to offset 16 of the structure in a5 and a byte to offset 18 of the structure in a5, then returns.
- `update_nibbles_and_state`: Checks offsets 52 and 54 of a structure in a5; if they are zero, it either increments nibbles in a table via a loop or writes a series of constants to memory and calls another routine twice.
- `process_nibbles_and_update_state`: Performs bitwise operations on d0 and d3 to adjust nibbles, and contains logic that decrements a 16-bit value at offset 64 of a structure in a5, calls several other routines, and writes to various memory locations.
- `adjust_high_nibble_of_word`: The routine takes a word from d0, extracts bits in the high byte range, subtracts a constant if non-zero, and returns the modified word with the extracted bits replaced.
- `adjust_middle_nibble_of_word`: The routine takes a word from d0, extracts a nibble in the middle of the word, subtracts a constant if non-zero, and returns the modified word with the extracted nibble replaced.
- `adjust_low_nibble_of_word`: The routine takes a word from d0, extracts the lowest 4 bits, subtracts 1 if they are non-zero, and returns the modified word with the extracted bits replaced.
- `increment_and_wrap_counter`: Masks a global variable, and if set performs several calls to other routines, increments a short field in a structure pointed to by a register and wraps it when it underflows.
- `tiny_sub_2`: Small routine of 1 instruction that subtracts d0 from -(a2).

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_score_table`: Validates a header in a data block, then copies fields from that block into a global table, builds derived offsets and sizes for entries, and normalizes a table of values by computing cumulative sums.
- `reset_audio_channels`: The routine stores a pointer into a global variable and then clears four audio channel volume registers.
- `update_audio_and_data_structures`: Increments a global word and clamps it to a maximum value of 64, then iterates through a table to call several function pointers, updating various global values and scaling them by the clamped value.
- `write_globals_and_table`: Writes constants and a value from a global variable into global variables, adds a constant to a word in a table pointed to by a pointer in memory, and touches hardware registers CIAB_CRA, CIAB_ICR, DMACON and INTREQ.
- `write_globals_and_table_2`: Writes values from several global variables into global variables, writes a constant into a global variable, stores a value from a global variable into a table entry, and touches hardware registers AUD0LCH, AUD0LEN, AUD1LCH, AUD1LEN, AUD2LCH, AUD2LEN, AUD3LCH, AUD3LEN, CIAB_ICR, INTREQ and custom+0.
- `initialize_and_load_data`: Pushes several bytes onto the stack, clears a word at offset 532 of a structure in a2, and if a value derived from a byte at offset 4 of a structure in a4 matches a constant, increments the word at offset 532 of a2 and checks d0 and d1; if d1 is non-zero, it calculates an offset for a0 and copies a word, a long, and a word from a0 into a structure in a4.
- `update_data_structure`: Ors a word from d7 into a location at offset 786 of a structure in a2, checks a byte at offset 532 of a structure in a2, and if non-zero performs arithmetic on d0 to calculate an offset for a0, then copies a word, a long, and a word from a0 into a structure in a4 before branching.
- `load_data_structure_2`: Copies several words and long values from a source pointed to by a0 into a destination structure in a3 and a4.
- `load_data_structure`: Clears a 32-bit field at offset 14 of a structure in a4, then enters a loop that reads data from a pointer in a0 and stores it into a structure in a3 and a4, finally calling another routine.
- `branch_data_loading`: Sets a0 to a calculated address, and if d0 is non-zero, stores it in a structure in a4; if d1 is non-zero, it calculates an offset for a0 and copies a word, a long, and a word from a0 into a structure in a4 before branching.
- `process_table_entry`: Ors d7 into offset 790 of a structure in a2, calculates an offset for a0 based on d0, and then reads a long and a word from a0 to compare the word against a value derived from a byte at offset 5 of a structure in a4; if the comparison fails, it performs arithmetic to store a long and two words into a structure in a3 and a4, and continues to move several more values from a0 into a4.
- `fetch_table_value_2`: Moves a word from a source pointed to by a0 into a structure in a4, then copies a long and a word from a0 into a3 and a4.
- `load_modified_table_entry`: Clears a 32-bit field at offset 14 of a structure in a4, and if d1 is non-zero, performs arithmetic on d0 to calculate an offset for a0, then reads a long and a word from a0 to compare the word against a value derived from a byte at offset 5 of a structure in a4; if the comparison fails, it stores a long and two words into a structure in a3 and a4, and continues to move several more values from a0 into a4.
- `fetch_table_value_offset`: Copies a long and a word from a source pointed to by a0 into a3 and a4, then moves a word from a0 into a structure in a4.
- `fetch_table_value`: Calculates an index using a value from a structure in a4, uses it to select a pointer from a table in a1, and stores a word from that pointer into offset 2 of the structure in a4 before branching to another routine.
- `execute_jump_table_entry`: Calls another routine, then uses a byte from a structure in a4 to index into a table and jumps to the resulting address.
- `write_globals_and_table_3`: Repeatedly calls another routine and invokes function pointers from a table, writes scaled values into fields of a structure, touches the CIAB_CRA hardware register, and writes a constant to a global variable.
- `update_timer_and_pointer`: Updates a byte field by adding an offset from a global table using an index stored in the structure, then checks the result for overflow.
- `update_character_ability_index`: Increments a short counter stored in a structure and, when the counter matches an input value, copies a word from a register into two locations derived from two pointers, updates a pointer to a table entry selected by the counter, and propagates fields from that entry into the structure.
- `process_structure_field`: Checks a byte at offset 5 of a structure; if non-zero, it uses a value from a table to determine a branch, performs bitwise operations on a byte from the structure, and calculates an offset using a value from the structure.
