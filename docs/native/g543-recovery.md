# Game 543 (Nova 9): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00543). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2703424 top-level bytes in 88 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 21 of 21 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 595 of 595 named (0 author symbols), 595 of 595 described (31 by rule, 562 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1040 of 1040 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 68064 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12169 of 12169 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1078843 |
| code | 39054 |
| data | 1534549 |
| padding | 21794 |
| unused | 29184 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:nx | 6 | 250 | 17 |
| Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:Install | 10 | 250 | 17 |
| Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:sp0 | 10 | 250 | 17 |
| Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:more | 6 | 250 | 17 |
| run | 2 | 458 | 12 |
| resident | 2 | 171 | 4 |
| sp | 35 | 960 | 52 |
| Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:sim | 12 | 250 | 17 |
| SKID_ROW | 2 | 267 | 8 |
| Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:title | 17 | 250 | 17 |
| Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:Nova | 3 | 317 | 13 |
| Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:gameover | 10 | 250 | 17 |
| Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:ttl | 12 | 250 | 17 |
| Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:intro | 21 | 3126 | 142 |
| Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:amigasnd.drv | 3 | 1820 | 112 |
| Nova 9 (1991)(Dynamix)(Disk 3 of 3).adf:story | 11 | 3039 | 114 |
| Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| ACTORS5.TBC (raw code file, load address unknown) | 1 | 0 | 0 |
| std.TBC (raw code file, load address unknown) | 1 | 0 | 0 |
| ACTORS6.TBC (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- nova 9 (1991)(dynamix)(disk 2 of 3)/Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:nova9.sng: high-entropy data file (7.74 bits/byte), classified as packed data
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:nx: Imploder (xfd) stream found by ancient scan (2876 -> 3436 bytes) but only 0 of 2 sampled windows are in reference RAM: not used
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:Install: Imploder (xfd) stream found by ancient scan (30232 -> 48476 bytes) but only 0 of 22 sampled windows are in reference RAM: not used
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:sp0: Imploder (xfd) stream found by ancient scan (117584 -> 145196 bytes) but only 3 of 68 sampled windows are in reference RAM: not used
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:more: Imploder (xfd) stream found by ancient scan (6876 -> 10548 bytes) but only 0 of 6 sampled windows are in reference RAM: not used
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:sim: Imploder (xfd) stream found by ancient scan (192780 -> 287724 bytes) but only 2 of 127 sampled windows are in reference RAM: not used
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:s/SKID_ROW: PowerPacker 4.0 (xfd) stream found by ancient scan (4736 -> 14188 bytes) but only 0 of 7 sampled windows are in reference RAM: not used
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:title: Imploder (xfd) stream found by ancient scan (123148 -> 169496 bytes) but only 1 of 77 sampled windows are in reference RAM: not used
- nova 9 (1991)(dynamix)(disk 2 of 3)/Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:gameover: Imploder (xfd) stream found by ancient scan (8752 -> 16156 bytes) but only 2 of 7 sampled windows are in reference RAM: not used
- nova 9 (1991)(dynamix)(disk 2 of 3)/Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:ttl: Imploder (xfd) stream found by ancient scan (40112 -> 81372 bytes) but only 0 of 37 sampled windows are in reference RAM: not used
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:tbc/ACTORS5.TBC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:tbc/std.TBC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:tbc/ACTORS6.TBC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- nova 9 (1991)(dynamix)(disk 2 of 3)/Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- nova 9 (1991)(dynamix)(disk 3 of 3)/Nova 9 (1991)(Dynamix)(Disk 3 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:Install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:sp0: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:more: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:sim: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:title: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- nova 9 (1991)(dynamix)(disk 2 of 3)/Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:gameover: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- nova 9 (1991)(dynamix)(disk 2 of 3)/Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:ttl: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- nova 9 (1991)(dynamix)(disk 2 of 3)/Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:intro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 3 never-loaded region(s), 68064 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:intro

## Copy protection found

- none found

## Functions


### Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:nx

- `initialize_aga_patch`: Executes a loop of function calls and checks a structure value against a specific bitmask to update a global variable.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `clear_cache_and_loop`: Calls a routine twice, then enters a loop that calls another routine and increments d5, repeating until d5 equals 2, before calling a routine via a pointer at a specific offset.
- `update_table_values`: Decrements a counter in d1 and loops to add the value from d3 to a table at a1 offset by d2, where d2 is calculated by adding d0 to zero.
- `branch_to_subroutine`: If a condition is met, it sets a1 to the value in a3, calls another routine, and then branches to a later instruction.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_in_loop`: Compares a6 with a2 and, if they are not equal, copies four bytes from the source pointed to by a2 to the destination pointed to by a1 in a loop that decrements d0 by 2 each iteration until d0 is zero, then loads a longword from the destination into d0 and returns.
- `decompress_bitstream_data_3`: Decodes a compressed bitstream by reading a control byte to determine repetition counts and bit lengths, then performs a series of bit-shifting and carry-propagating operations to expand the data into a destination buffer.
- `process_bitstream_loop`: Sets d7 to 2, then enters a loop that doubles d1, fetches a byte from the address in a0 into d1 if d1 is zero, adds d1 and d0 with carry, decrements d7, increments d0 by 6, and branches to another routine.
- `initialize_bitstream_parameters`: Sets d0 to 5, sets d7 to 3, and branches to another routine.
- `initialize_bitstream_parameters_2`: Sets d0 to 4, sets d7 to 2, and branches to another routine.
- `initialize_bitstream_parameters_3`: Sets d0 to 3, sets d7 to 1, and branches to another routine.
- `expand_bitstream_variable_length`: Reads a variable number of bits from a bitstream in a0 by shifting and fetching bytes, then uses a value from a table to determine how many additional bits to extract into a destination register.
- `shift_and_expand_bits`: Shifts the byte in d1 left by one bit and branches if the result is non-zero, otherwise it fetches a byte from the buffer pointed to by a0 into d1, adds it to d1 with carry, adds d2 to d2 with carry, and branches to another location.
- `decode_bitstream_expansion`: Reads a bit-length from a table and performs a bit-stream expansion by shifting bits from a source buffer into a destination register.
- `unpack_and_copy_compressed_data`: Reads a control byte from a source buffer to determine a bit-length and a repetition count, then performs a bit-stream expansion to decode values into a destination buffer.
- `push_registers_to_stack`: Pushes a byte from d0, a byte from d1, and a word from d2 onto the stack using a6 as a pointer, restores several registers, and performs a relative branch.

### Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:Install

- `initialize_game_state`: Executes a sequence of function calls and checks a global pointer to swap a memory value if a specific constant is found.
- `decrement_register_until_negative_one`: Decrements a 16-bit register value by one in a loop until the value reaches negative one.
- `clear_cache_and_loop_install`: Calls a routine twice, then enters a loop that calls another routine and increments d5, repeating until d5 equals 6, before calling a routine via a pointer at a specific offset.
- `accumulate_array_elements`: Calls a subroutine to determine a count, then enters a loop that calls another routine to fetch a value and adds that value to an array located at the address in a3.
- `branch_to_subroutine_alt`: Checks a condition, sets a1 to the value in a3, calls another routine, and then performs a branch.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `transfer_four_bytes_iteratively`: Checks if a6 equals a2; if not, it performs a loop that moves four bytes from the memory location at a2 to the memory location at a1, decrementing d0 by 2 each time, and finally moves a longword from the destination into d0 before returning.
- `expand_variable_length_bitstream_4`: Expands a compressed bitstream by reading control bytes to determine bit lengths and performing bitwise shifts and additions on a value in d1.
- `process_bitstream_loop_2`: Sets d7 to 2, then enters a loop that doubles d1, fetches a byte from the address in a0 into d1 if d1 is zero, adds d1 and d0 with carry, decrements d7, increments d0 by 6, and branches to another routine.
- `initialize_bitstream_parameters_4`: Sets d0 to 5, sets d7 to 3, and branches to another routine.
- `initialize_bitstream_parameters_5`: Sets d0 to 4, sets d7 to 2, and branches to another routine.
- `initialize_bitstream_parameters_6`: Sets d0 to 3, sets d7 to 1, and branches to another routine.
- `expand_variable_length_bitstream`: Performs bitwise expansion of a compressed stream by reading control bytes to determine bit lengths and doubling values in d1, potentially fetching additional bits from the source pointed to by a0.
- `process_bit_expansion`: Doubles the byte in d1 and branches if non-zero, otherwise it loads a byte from the source pointed to by a0 into d1, adds it to d1 with carry, adds d2 to d2 with carry, and branches to another location.
- `expand_bitstream_data`: Reads a control byte to determine bit lengths and performs bitwise expansion of a compressed stream into a destination buffer.
- `decompress_bitstream_data_2`: Reads a control byte to determine bit lengths and performs bitwise expansion of a compressed stream into a destination buffer.
- `push_registers_to_stack_2`: Pushes a byte from d0, a byte from d1, and a word from d2 onto the stack using a6 as a pointer, restores several registers, and performs a relative branch.

### Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:sp0

- `verify_and_swap_memory_constant`: Executes a loop of function calls and checks a value at a calculated offset from a global pointer.
- `decrement_d6_until_negative_one_2`: Decrements the value in register D6 until it reaches negative one and then returns.
- `accumulate_array_elements_loop`: Executes a loop six times, calling a subroutine to fetch values and adding them to an array located at a pointer in a1.
- `update_table_via_function_return`: Calls a subroutine and uses the returned value to calculate an offset, then adds the value from a register to a memory location at that offset within a table pointed to by a register.
- `conditionally_call_subroutine_with`: If a zero flag is set, it branches to another location; otherwise, it moves a pointer from a3 to a1, calls another routine, and branches to another location.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_from_source_2`: Compares two pointers and, if they are not equal, copies four bytes from a source pointed to by a register to a destination pointed to by another register using a loop that decrements a counter.
- `expand_variable_length_bitstream_6`: Reads a control word and two bytes from a table to determine a bit-length and initial values, then enters a loop that performs bitwise left-shifts on a value while fetching new bits from a source buffer to expand a compressed stream into a destination buffer.
- `expand_bitstream_loop`: Sets d7 to 2, doubles d1, and if d1 is not zero, enters a loop that fetches a byte from the address in a0 into d1, adds d1 and d0 with carry to themselves, decrements d7, and repeats; otherwise, it adds 6 to d0, sets d7 to 3, and branches to another location.
- `initialize_bitstream_parameters_7`: Sets d0 to 5, d7 to 3, and branches to another location.
- `initialize_bitstream_parameters_8`: Sets d0 to 4, d7 to 2, and branches to another location.
- `initialize_bitstream_parameters_9`: Sets d0 to 3, d7 to 1, and branches to another location.
- `expand_variable_length_bitstream_3`: Performs bitwise expansion of a compressed stream by reading bits from a source pointed to by a0 and using a lookup table to determine bit lengths and values.
- `update_bitstream_values`: Multiplies a byte in d1 by two and branches if the result is non-zero, otherwise it loads a byte from a buffer pointed to by a0 into d1, adds it to d1 with carry, adds d2 to d2 with carry, and branches to another location.
- `decompress_bitstream_data_2_3`: Reads a control byte from a table to determine bit lengths and performs bitwise expansion of a compressed stream into a destination buffer.
- `expand_bitstream_and_copy_data`: Reads a variable-length bitstream from a source buffer to calculate a value, then uses that value as an offset to copy a block of bytes from a source table to a destination buffer.
- `push_registers_and_branch_v3`: Writes d0, d1, and d2 to the stack at a4, restores multiple registers from the stack, and branches to a different location.

### Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:more

- `patch_aga_hardware`: Checks a specific bitmask against a value in a memory structure and, if it matches, updates a global variable and clears a memory location.
- `decrement_register_until_negative_one_3`: Decrements a 16-bit value stored in a register until it reaches negative one and then returns.
- `update_table_and_call_exec`: Calls a subroutine twice, then enters a loop that executes a subroutine and updates a table by adding a value to an entry at an offset determined by a counter.
- `add_values_to_table_more`: Doubles d0 and branches if positive; otherwise it calls another subroutine, then enters a loop that runs up to two times, calling another subroutine, calls a fourth subroutine, and adds the result to a table at offset d2 of a3.
- `branch_and_call_loop`: If a condition is met, it moves the value from a3 to a1, calls another routine, and then branches to a location after the initial conditional branch.
- `fall_through_entry_4`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_with_loop`: Compares a pointer in a2 with a pointer in a6 and, if they are not equal, copies four bytes from the source at a2 to the destination at a1 before decrementing a counter in d0 and looping if the counter is not zero; otherwise, it loads a longword from the destination into d0 and returns.
- `decompress_bitstream_data_5`: Decodes a compressed bitstream by reading control values from a table and performing bit-shifting and addition operations to expand data into a destination buffer.
- `expand_bitstream_loop_2`: Sets d7 to 2, doubles d1, and if d1 is not zero, enters a loop that fetches a byte from the address in a0 into d1, adds d1 and d0 with carry to themselves, decrements d7, and repeats; otherwise, it adds 6 to d0, sets d7 to 3, and branches to another location.
- `initialize_bitstream_parameters_10`: Sets d0 to 5, d7 to 3, and branches to another location.
- `initialize_bitstream_parameters_11`: Sets d0 to 4, d7 to 2, and branches to another location.
- `initialize_bitstream_parameters_12`: Sets d0 to 3, d7 to 1, and branches to another location.
- `expand_bitstream_variable_length_2`: Shifts bits from a source buffer into a register, fetching additional bytes from the buffer in a0 as needed, and uses a table to determine the number of bits to process or expand.
- `process_bitstream_byte`: Doubles the value in d1 and branches if non-zero, otherwise it decrements the pointer in a0 to fetch a byte into d1, performs two additions with carry on d1 and d2, and branches to another location.
- `decode_bitstream_compressed_data`: Reads a bitstream from a source buffer to expand data using a combination of bit-shifting, variable-length bit-length decoding, and a dictionary-based repetition mechanism.
- `expand_bitstream_data_2`: Reads a control byte from a source buffer to determine bit lengths and repetition counts, then performs bit-shifting and arithmetic to expand the bitstream into a destination buffer.
- `push_registers_and_branch_v4`: Stores d0, d1, and d2 onto the stack at a4, restores registers from the stack, and then performs a branch.

### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block_3`: Copies sixteen words from a source location to a destination location and then calls another routine.
- `initialize_entity_structure_and_copy_data`: Initializes a structure by clearing various offsets, updates specific attributes using a table lookup, and performs a loop to copy a sequence of bytes from one memory location to another based on a calculated length.
- `update_entity_attribute_and_copy_byte_2`: Fetches a byte from a table using an index stored at offset 0 of a structure, stores it at offset 20, increments a value at offset 24, and calls another routine.
- `update_entity_attribute_and_copy_byte`: Calculates a new value for a field at offset 22 of a structure in a1 by adding 11 to a value at offset 18, then stores this in offset 22.
- `update_entity_state_and_call_handler`: Updates several fields in an entity structure within a table, including setting a byte to 10 and copying longwords between offsets, then calls another routine.
- `branch_with_offset`: Adds the value at offset 84 of the structure in a1 to one and then branches to another routine.
- `test_status_longword`: Tests a longword at offset 48 of a structure in a1 and branches to another routine if the value is non-zero, otherwise it branches to another location.
- `mask_and_branch_on_status_bits`: Checks if a longword at offset 48 of a structure has specific bits set; if the value is zero, it branches to another routine, otherwise it performs a bitwise OR to set the upper eight bits of that longword and branches to another routine based on a comparison with a constant.
- `update_entity_state_and_table_2`: Copies a longword from offset 64 of a structure to offset 108 of the same structure, then calls another routine with parameters loaded from registers and a pointer derived from a table.
- `update_entity_state_and_table`: Calculates an index into a table using values from a structure, updates a longword at that index, and clears several fields in a secondary table.
- `process_data_structures`: Performs several calls to a function using values from a structure, modifies a longword in a table based on a calculation, and performs an indirect jump.

### resident

- `initialize_resident_memory_structure`: Traverses a linked list to find the maximum value in a sequence and then populates a memory structure with a pattern of incrementing values.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location to a destination pointed to by the stack and then executes a function via a pointer.
- `update_structure_and_call_handlers`: Calculates a value by shifting a register, stores it at offset 4 of a structure in a1, and performs several calls to a routine in a5 while updating various offsets in the structure in a1.
- `jump_to_address`: Clears register d1 and performs an indirect jump using the address in a6.

### sp

- `parse_and_store_command_tokens`: Parses a string into tokens by iterating through characters and handling specific control characters and whitespace.
- `invoke_indirect_pointers`: Invokes two function pointers stored at specific offsets within a structure pointed to by a register.
- `call_indirect_pointers`: Invokes two function pointers stored at offsets within a structure pointed to by a register.
- `update_task_status`: Retrieves a task pointer, modifies a value at an offset within that task's structure using a specific constant, and calls an indirect routine.
- `self_calling_guard`: Calls another routine unconditionally, then checks a global variable and, if non-zero, calls another routine with the input parameter, passes the result to another routine, and finally calls another routine unconditionally.
- `manage_resource_nodes`: Iterates through a linked list of nodes to find specific entries based on string matching and bitwise conditions.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `allocate_and_patch_memory`: Checks fields of a global structure and, if conditions are met, obtains a memory block and a code pointer via library calls, copies a small code stub from another routine into the target, updates a pointer in the global structure, computes a checksum over a short array in the structure and stores its complement, then returns.
- `sum_and_store_inverted`: Iterates through a table of words to calculate a sum, stores the bitwise NOT of that sum in a global variable, and then performs an indirect call.
- `patch_resource_function`: Checks a global variable and calls a routine through a pointer.
- `check_flag_and_copy_word`: The routine calls another routine and tests its return value.
- `dispatch_conditional_pointer_call`: Compares a value against a global variable and a constant, then executes another routine via a pointer if the conditions are met.
- `dispatch_conditional_call`: Checks if a register contains a specific value and, if not, calls another routine via a pointer stored in a global variable.
- `update_interrupt_handler`: Updates a global variable and writes a value to the INTENA register with the interrupt enable bit set, then performs an indirect call to a routine retrieved from a table via an offset.
- `call_indirect_via_offset`: Calls a routine using a pointer retrieved from a global variable with a specific offset.
- `initialize_patch_function_table`: Writes a computed offset to a global variable and fills a 16-entry table with successive addresses of a routine, incrementing the pointer each iteration.
- `return_input_value_2`: An empty routine that simply returns the value provided in a register.
- `execute_recursive_calls`: Performs a sequence of repeated calls to another routine before returning a value.
- `return_input_value`: The routine receives a value in D0 and returns it unchanged.
- `dispatch_conditional_call_2`: Checks a value in a global variable against a threshold and calls a function through a pointer if the condition is met.
- `copy_data_via_loop`: Copies a sequence of words from one memory location to another using a loop.
- `execute_loop_and_mask_bit`: Executes a routine multiple times through a loop and returns a value from a pointer with the highest bit set.
- `get_masked_value`: Calls an indirect routine and returns a value from a pointer with the highest bit set.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `empty_stub_routine`: The routine contains four instructions and performs no observable work; it does not access memory, registers, or call other code, and immediately returns to its caller.
- `find_node_by_string`: Iterates through a linked list and returns a pointer to a node if a string stored within the node matches a provided input string.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `reset_global_variable`: Sets a global variable to zero and returns its previous value.
- `call_indirect_via_table`: The routine calls another routine through a pointer stored in a global variable.
- `execute_conditional_logic`: Calls a routine with a value from a pointer, and if the result is non-zero, performs two subsequent routine calls using that same value.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `scan_for_marker`: Iterates through an array of words starting from an input pointer.
- `calculate_offset_pointer`: Calculates a new pointer by reading a 16-bit value from an input pointer and applying a bit-shifted offset.
- `compute_word_offset`: Reads a 16-bit value from two bytes after an input pointer.
- `patch_global_pointers`: The routine sequentially calls a helper routine to compute offsets and stores the returned pointers into a series of global variables.
- `traverse_linked_list`: Traverses a linked list and executes a function pointer stored in a global variable if certain bitwise conditions are met by the list elements.
- `nop_routine`: The routine is empty and performs no operations.
- `return_from_subroutine_2`: The routine performs no operations and immediately returns to its caller.
- `compute_and_store_byte_logic`: Calls a function pointer from a global structure and combines its result with register values using bitwise logic.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_other_routine_with_stack`: Calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:sim

- `execute_initialization_sequence`: Performs a series of function calls and checks a value in memory against a constant to update a global variable.
- `decrement_d6_to_minus_one`: Decrements the value in a 16-bit register until it reaches negative one.
- `accumulate_array_elements_4`: Calls a subroutine three times, then enters a loop that runs up to eight times, calling another subroutine and adding a value to an array at offset an address of a pointer in a1.
- `accumulate_array_elements_2`: Calls a subroutine to determine a count, then executes a loop that calls another routine to fetch a value and adds it to an array located at the address in a3, using an index derived from the count.
- `branch_to_subroutine_v2`: If a condition is met, it sets a1 to a3, calls another routine, and then branches.
- `fall_through_entry_5`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_loop`: Compares a6 and a2, and if they are not equal, copies four bytes from the source at a2 to the destination at a1 in a loop that decrements d0, then loads a long value from a1 into d0 before returning.
- `decompress_bitstream_data_6`: Reads a control word and a byte from a table to determine a bitstream length and initial value, then enters a loop that shifts the value left and fetches additional bytes from a source buffer in a0 to fill the bitstream.
- `expand_bitstream_loop_3`: Sets a loop counter in d7 to 2, shifts d1 left by one bit, and if the result is non-zero, branches to a later instruction; otherwise, it fetches a byte from the buffer in a0 into d1, adds it to d1 and d0 with carry, decrements d7 in a loop, increments d0 by 6, sets d7 to 3, and branches to another routine.
- `set_bitstream_parameters_5_3`: Sets d0 to 5, d7 to 3, and branches to another routine.
- `set_bitstream_parameters_4_2`: Sets d0 to 4, d7 to 2, and branches to another routine.
- `set_bitstream_parameters_3_1`: Sets d0 to 3, d7 to 1, and branches to another routine.
- `expand_bitstream_data_element`: Performs a series of bit-shifting operations on a value in d1, fetching additional bytes from the buffer in a0 when a shift results in zero, and uses the resulting value to index into a table to load a byte into d6.
- `expand_bit_data_2`: Performs a conditional branch based on doubling d1, then if zero, fetches a byte from the buffer in a0 into d1, adds it to d1 with carry, adds d2 to d2 with carry, and branches to another location.
- `decode_bitstream_and_copy_data_2`: Reads a bitstream from a source buffer to determine a sequence of values, which are used as indices into a table to select data blocks that are then copied to a destination buffer.
- `decompress_and_copy_data`: Reads a control byte to determine a bit-stream length and uses it to decode values from a source buffer into a destination buffer.
- `push_registers_and_branch`: Pushes bytes from d0 and d1, a word from d2, and several registers and data registers onto the stack before performing a relative branch.

### SKID_ROW

- `parse_amiga_hunk_data`: Iterates through a data stream to identify and process specific hunk type codes.
- `decrement_and_loop`: Decrements a value in a loop until it reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_bit_stream_2`: Performs complex memory manipulation including conditional loops, byte copying, and pointer arithmetic based on bitwise checks.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_stream`: Processes a stream of bits by performing bitwise shifts and rotations based on a counter and a data source.
- `modify_memory_via_pointer`: Increments a byte at a location pointed to by a register and negates a short integer at an offset from another register.

### Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:title

- `execute_title_initialization`: Executes a loop of function calls and performs a memory check against a constant to update a global variable.
- `decrement_d6_to_negative_one`: Decrements the value in a 16-bit data register until it reaches negative one.
- `accumulate_array_elements_variant`: Calls a subroutine three times, then enters a loop that runs up to thirteen times, calling another subroutine and adding a value to an array at offset an address of a pointer in a1.
- `accumulate_array_elements_3`: Calculates a value using a function call, then enters a loop that adds the result of that function to elements in an array pointed to by a third register, decrementing a counter until it reaches zero.
- `branch_to_subroutine_v3`: Checks a condition, sets a1 to a3, calls another routine, and then branches to a different instruction.
- `fall_through_entry_6`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_loop_alt`: Checks if a6 equals a2, otherwise it copies four bytes from the source at a2 to the destination at a1 using a loop controlled by d0, and finally loads a long value from a1 into d0.
- `decompress_bitstream_data_block_2`: Reads a control word and two bytes from a table to determine a bit count and loop count, then performs bit-stream decompression by shifting a value in a register and fetching bytes from a source pointer to reconstruct the data into a destination buffer.
- `extract_bitstream_loop`: Sets a loop counter in d7 to 2, shifts d1 left by one bit, and if the result is non-zero, branches to a later instruction; otherwise, it fetches a byte from the buffer in a0 into d1, adds it to d1 and d0 with carry, decrements d7 in a loop, increments d0 by 6, sets d7 to 3, and branches to another routine.
- `set_extraction_parameters_5_3`: Sets d0 to 5, d7 to 3, and branches to another routine.
- `set_extraction_parameters_4_2`: Sets d0 to 4, d7 to 2, and branches to another routine.
- `set_extraction_parameters_3_1`: Sets d0 to 3, d7 to 1, and branches to another routine.
- `extract_variable_length_bitstream_value`: Reads a variable number of bits from a bitstream in a0 to reconstruct a value in d2, using a control byte to determine the bit count and loop iterations.
- `modify_bitstream_registers`: Checks if d1 doubled is non-zero to branch, otherwise it moves a byte from the buffer at a0 into d1, adds it to d1 with carry, adds d2 to d2 with carry, and branches to another location.
- `decompress_bitstream_data_block`: Reads a bitstream from a source pointer to reconstruct values, using a control byte to determine bit counts and loop iterations.
- `unpack_compressed_data_block`: Reads a control byte from a source pointer to determine a bit-count and a loop count, then performs a bit-stream decompression by shifting and adding bytes from a source buffer into a destination buffer.
- `push_registers_and_branch_2`: Pushes bytes from d0 and d1, a word from d2, and several registers and data registers onto the stack before performing a relative branch.

### Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:Nova

- `initialize_game_environment`: Opens required system libraries and initializes several data structures and global pointers.
- `decompress_bitstream_data`: Processes a nested data stream using bit-masking and control codes to expand compressed bytes into a destination buffer.
- `unpack_compressed_data_stream`: Iterates through a nested data structure to decompress and expand a byte stream into a destination buffer.
- `decrement_and_process_table`: Decrements a value at a pointer and, if it reaches zero, performs bitwise transformations on a table before calling a function pointer.
- `process_data_table_2`: Decrements a value at a pointer and, if zero, iterates through a table to perform bitwise comparisons and updates values before calling a function pointer.
- `search_and_call_function_2`: Searches through a byte table for a matching value and then calls a function pointer stored at an offset from a pointer.
- `search_and_call_function`: Performs a bitwise search through a table to find a matching value and then calls a function pointer stored at a specific offset.
- `check_available_memory`: Calls a library function to check available memory and executes two other routines if the returned value is below a threshold.
- `trigger_memory_error_dialog`: Sets a value in a global variable and executes two functions via pointers retrieved from a global variable.
- `execute_library_calls`: Calls two functions using pointers retrieved from a global variable.
- `copy_memory_block`: Copies ten words of data from a source memory location to a destination memory location.
- `calculate_offset_address`: Calculates a value using multiplication and addition involving three input parameters and a global constant.
- `copy_ten_words`: Copies ten words of data from a source memory location to a destination memory location using a loop.

### Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:gameover

- `execute_sequence_and_swap_constant`: Executes a series of function calls and a loop containing function calls, then checks a value at an offset from a pointer derived from a global variable; if that value matches a specific constant, it swaps the value with a stored constant and clears the original source.
- `decrement_d6_until_negative_one`: Decrements the value in a data register until it reaches negative one and then returns.
- `iterate_and_accumulate_array`: Executes a subroutine six times within a loop, using the results to fetch values and add them to an array located at a specific offset from a pointer.
- `add_values_to_table`: Doubles d0 and branches if positive; otherwise it calls another subroutine, then enters a loop that calls another subroutine, calls a fourth subroutine, and adds the result to a table at offset d2 of a3.
- `branch_if_equal`: Branches if a byte is zero; otherwise it sets a1 to a3, calls another subroutine, and enters a loop that branches back to the start.
- `fall_through_entry_7`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `move_bytes_with_counter`: If a6 is not equal to a2, it enters a loop to copy four bytes from the address in a2 to the address in a1, reducing d0 by 2 per iteration, and concludes by moving a longword from the destination into d0 and returning.
- `expand_variable_length_bitstream_5`: Reads a control byte from a table to determine bit lengths and then enters a loop that performs bitwise expansion by shifting and adding bits from a source buffer into a destination buffer.
- `accumulate_bitstream_values`: Iterates through a bitstream by reading bytes from a pointer in a0, adding the byte and a value from d0 to d1 using carry-aware addition.
- `load_constants_and_branch`: Loads the values 5 and 3 into registers d0 and d7 respectively, then performs a short branch to another routine.
- `load_initial_values`: Loads the values 4 and 2 into registers d0 and d7, then branches to another routine.
- `load_initial_values_alt`: Loads the values 3 and 1 into registers d0 and d7, then branches to another routine.
- `expand_variable_length_bitstream_2`: Performs bitwise expansion of a compressed stream by doubling the value in d1 and conditionally loading new bits from the source pointed to by a0 into d1 using the carry flag.
- `update_bit_stream`: Performs a left shift on d1 and branches if non-zero, otherwise it moves a byte from the buffer at a0 into d1, adds it to d1 with carry, adds d2 to d2 with carry, and branches to another location.
- `decompress_bitstream_data_2_2`: Reads a control byte to determine bit lengths and performs bitwise expansion of a compressed stream into a destination buffer.
- `decompress_bitstream_to_buffer`: Reads a variable-length bitstream from a source buffer to extract bit values, then uses those values as an index to copy a block of bytes from a table into a destination buffer.
- `push_registers_and_branch_3`: Stores bytes from d0 and d1 and a word from d2 onto the stack pointed to by a4, restores several registers from the stack, and branches to another location.

### Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:ttl

- `check_memory_constant`: Performs a series of function calls and checks a memory location for a specific constant to swap a value.
- `decrement_register_until_negative_one_2`: Decrements a 16-bit register value by one repeatedly until the value reaches negative one.
- `clear_cache_and_loop_ttl`: Calls a routine twice, then enters a loop that calls another routine and increments d5, repeating until d5 equals 8, before calling a routine via a pointer at a specific offset.
- `accumulate_array_elements_via_function`: Calls a subroutine to obtain a value, then enters a loop that executes a number of times determined by a register, calling another subroutine and adding the resulting value to an array at an offset calculated from a register.
- `branch_if_equal_variant`: Branches if a byte is zero; otherwise it sets a1 to a3, calls another subroutine, and enters a loop that branches back to the start.
- `fall_through_entry_8`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_from_source`: Compares a pointer in a6 with a pointer in a2; if they are equal, it returns, otherwise it copies four bytes from the source pointed to by a2 to the destination pointed to by a1, decrements a counter in d0, and loops if the counter is not zero; if the counter reaches zero, it loads a new value into d0 from the destination and returns.
- `decompress_bitstream_data_4`: Reads a control byte and a length from a table to determine how many bytes to copy from a source buffer to a destination buffer.
- `process_bitstream_loop_3`: Sets d7 to 2, performs a signed addition on d1, and if the result is non-zero, branches to another location; otherwise, it fetches a byte from the buffer in a0 into d1, adds it to d1 and d0 with carry, decrements d7 in a loop, adds 6 to d0, sets d7 to 3, and branches to another routine.
- `load_initial_values_v2`: Loads the values 5 and 3 into registers d0 and d7, then branches to another routine.
- `load_initial_values_v3`: Loads the values 4 and 2 into registers d0 and d7, then branches to another routine.
- `load_initial_values_v4`: Loads the values 3 and 1 into registers d0 and d7, then branches to another routine.
- `expand_bitstream_data_sequence`: Performs a series of bit-shifting and conditional byte fetches from a buffer in a0 to expand a bitstream into a sequence of values.
- `expand_bit_data`: Multiplies d1 by two and branches if non-zero, otherwise it pulls a byte from the pointer in a0 into d1, adds it to d1 with carry, adds d2 to d2 with carry, and branches to another location.
- `decode_bitstream_and_copy_data`: Reads a bit-stream from a source buffer to decode a sequence of values, using a control byte to determine the number of bits to process.
- `decode_bitstream_and_copy_data_3`: Reads a control byte from a table to determine a bitstream length and performs bit-shifting operations to reconstruct a value from a source buffer.
- `push_registers_and_branch_alt`: Pushes d0, d1, and d2 onto the stack at a4, restores a set of registers from the stack, and then branches.

### Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:intro

- `initialize_system_hardware_and_resources`: Configures the Amiga bitplane, audio, and DMA hardware while initializing resource interfaces and interrupt vectors.
- `call_and_increment_result`: Calls a function via a pointer from a provided base address and adds eight to the returned value if it is non-zero.
- `conditionally_call_function`: Calls a function via a pointer from a provided base address if a specific register is non-zero.
- `call_recursive_routines`: The routine calls another routine three times in succession.
- `configure_blt_hardware`: Sets various hardware registers for bitplane transfers and iterates through a loop to populate a table with memory addresses.
- `execute_nested_calls`: Performs a series of conditional calls to another routine based on its return value.
- `loop_and_call_routine`: Executes a loop that calls another routine multiple times, with an additional call performed after the loop completes.
- `execute_dispatch_table_callbacks`: Iterates through a table of data, performing various conditional checks and calling multiple subroutines based on the values found.
- `wait_for_hardware_status_2`: Repeatedly calls another routine and polls a global variable until specific bits match a constant value.
- `process_input_queue`: Repeatedly removes a word from a table and executes a function pointer if the value matches a specific address.
- `call_audio_and_accumulation_routines`: Calls a routine to update hardware audio registers and then calls a routine to accumulate table values into a destination buffer.
- `call_audio_and_accumulation_routines_2`: Calls a routine to update hardware audio registers and then calls a routine to accumulate values from a table into a destination buffer.
- `call_subroutines_recursively`: The routine calls itself twice before returning.
- `update_audio_registers`: Iterates through a table to find a zero byte, increments it, and writes several values to hardware audio registers and DMA/interrupt control registers.
- `wait_for_zero_byte`: Enters a loop that continues as long as the byte at the location specified by a register is non-zero.
- `set_interrupt_and_call_functions`: Updates several global variables and calls a function via a pointer three times in a loop.
- `configure_interrupt_and_dma`: Sets an interrupt vector and modifies the interrupt enable and DMA control registers.
- `decrement_and_halt_80`: Decrements a byte through a pointer and enters an infinite loop if the value reaches zero, otherwise sets a global variable to a specific value.
- `decrement_byte_and_halt_on_zero`: Decrements a byte through a pointer and checks if the result is zero.
- `decrement_and_check_zero`: Decrements a byte through a pointer and checks if the result is zero.
- `decrement_and_halt`: Decrements a byte through a pointer and enters an infinite loop if the resulting value is zero, while writing to a hardware register.
- `accumulate_table_values`: Reads values from a table using offsets derived from a source buffer and stores the accumulated sums into a destination buffer.
- `parse_data_header_and_recurse`: Validates a memory block for a specific magic signature and non-zero length before storing header fields into global variables.
- `write_words_to_table`: Waits for a specific bitmask in a global variable and then writes two words to a table using an index derived from a register.
- `initialize_resource_interface`: Checks if a global variable is zero, then retrieves a library interface from a pointer in a global variable to execute a sequence of functions.
- `validate_data_structure`: Calls another routine and checks a byte from an offset in the returned pointer.
- `execute_callback_chain_2`: Executes a function via a pointer from a global variable and performs conditional function calls based on the result.
- `copy_data_block_2`: Checks if a pointer is non-zero and then copies a sequence of longwords from that pointer into a global variable.
- `store_longword_to_table`: Waits for a condition in a global variable and then stores a 32-bit value into a table as two separate words.
- `release_resource_interface`: Checks if a global variable is non-zero and, if so, executes two functions through offsets of that variable before clearing the global variable.
- `copy_string_and_search_pattern`: Copies a null-terminated string into a global buffer and then scans a memory region for a specific sequence of two words.
- `pack_bitfield_into_word`: Constructs a 32-bit value by performing bitwise shifts and logical operations on three input registers and stores the result at a pointer address.
- `initialize_system_resources`: Performs a series of complex library calls and hardware register manipulations involving stack operations and conditional branching.
- `find_linked_list_element`: Traverses a linked list of longwords by following offsets until a specific constant is matched.
- `pop_word_from_queue`: Removes a word from a table and shifts the remaining elements forward if the table is not empty.
- `rotate_color_components`: Iterates through a source color table and applies a modular arithmetic rotation to each individual color component.
- `disable_interrupt_handler`: Checks if a global flag is set, then clears that flag, sets a second global variable to a maximum value, and executes a routine via a function pointer.
- `wait_for_hardware_status`: Sets a global variable to a specific value based on a register, loops until a bitmask condition is met, and then clears three memory locations and the global variable.
- `wait_and_set_blitter_config`: Polls a bit in a global variable before writing several configuration values to memory locations used for hardware setup.
- `update_dma_pointers`: Calculates a series of memory addresses based on an offset and a loop counter, then stores these resulting values into a sequence of global variables.
- `copy_and_obfuscate_data`: Copies a block of data into global variables and performs a bitwise transformation on a sequence of values.
- `populate_graphics_buffer`: Allocates a destination buffer and performs a nested loop to copy large blocks of data from source tables.
- `copy_masked_data`: Iterates through a source buffer and copies bytes to a destination only if a bitwise comparison with a mask table succeeds.
- `write_table_values`: Waits for a condition in a global variable to be met, then writes two specific words into a table.
- `write_long_to_table`: Waits for a condition in a global variable to be met, then stores a 32-bit value into a table as two words.
- `modify_memory_word`: Performs bitwise manipulations on input values and stores the resulting 32-bit value at a pointer address.
- `reset_hardware_registers`: Sets a global variable based on a register value, waits for a condition, and then clears several memory locations.
- `copy_string_and_search`: Copies a null-terminated string to a global variable and then searches a memory area for a specific pattern of words.
- `sync_object_data_table`: Checks a condition against a global variable before copying a sequence of longwords from a memory location into a global table and updating a global pointer.
- `rotate_color_palette_data`: Copies a source table of color data to a destination buffer while applying a modular arithmetic rotation to each color component.
- `apply_color_palette_rotation`: Copies a source table of color data to a destination buffer while applying a modular arithmetic rotation to each color component.
- `update_mouse_cursor_position`: Calculates new cursor coordinates by computing the difference between current and previous input values, applies boundary clamping, and updates a data structure.
- `update_object_state`: Updates a structure of values using arithmetic operations and conditional bounds checking, modifies several global variables, and executes a callback.
- `update_object_state_and_execute_callback`: Updates a structure of values using arithmetic operations and conditional bounds checking, modifies several global variables, and executes a function pointer stored in a table.
- `update_state_and_callback`: Updates a table with calculated values, modifies several global variables, and executes a function pointer from a table.
- `update_state_and_call_callback`: Calculates values based on a table, updates several global variables, and executes a function pointer from a table.
- `copy_data_with_mask_check`: Iterates through a source buffer and copies bytes to a destination only if a bitwise comparison with a mask table succeeds, potentially calling another routine during the process.
- `copy_and_search_data`: Copies a block of seventeen bytes from one location to another and then performs a conditional search through a table to find a matching byte.
- `copy_data_with_mask`: Copies a sequence of bytes from a source buffer to a destination buffer, conditionally performing the copy based on a bitwise comparison between a mask table and a control byte array.
- `dispatch_nested_data_block_type`: Reads a header containing two 16-bit values and two 16-bit pointers from a source, then reads a control byte to determine the next operation.
- `unpack_nested_data_blocks`: Copies a block of 40 bytes from a source pointer to a destination pointer, then enters a nested loop structure that iterates through a 2D array of data defined by two counters.
- `process_indexed_byte`: Calls another routine, decrements the pointer in a3 by 40, and then either moves a byte from a0 into d5 and branches or moves a byte from a0 into d3 and stores it at an offset in a3 determined by d5.
- `mask_and_store_byte`: Masks the high bit of a byte in register d5, reads a byte from the address in a0, and stores that byte at an offset in a3 determined by the masked value.
- `unpack_nested_data_blocks_2`: Performs a bit-rotation on a byte read from a source pointer and uses the result to control a nested loop that copies bytes to a destination pointer.
- `branch_on_masked_nibble`: Masks the upper two bits of a byte from a register and branches to different locations based on whether the resulting value is four, eight, or any other value, otherwise it calls another routine and then branches to another location.
- `call_and_branch`: Calls another routine and then branches to another location.
- `branch_via_subroutine`: Calls another routine and then branches to another routine.
- `copy_memory_block_4`: Calculates two source and destination addresses using another routine, then copies ten words of data from the source to the destination using a loop.
- `calculate_scaled_offset_address`: Multiplies a 16-bit value from a register by a 16-bit value from another register, adds it to a 32-bit value, shifts the result left by three bits, performs several additions to scale the value, and adds it to a pointer in a register before returning.
- `extract_nibble_and_branch`: Extracts a byte from a register, shifts it right by four bits, masks the lower three bits of the result, and then loads a byte from the address in a0 into d0 before branching to another routine.
- `manage_library_and_interrupts`: Interacts with system libraries to manage resources and manipulates hardware interrupt registers.
- `configure_bitplane_and_copper_data`: Calculates bitplane dimensions and color palette parameters based on a bitmask, then populates a table with interleaved copper instruction data.
- `execute_callback_chain`: Executes a function pointer from a global structure, potentially calling another function if the result is non-zero, and updates a global pointer from a specific offset.
- `execute_callback_pointers_and_reset`: Checks several global variables for non-zero values, executes functions located at an offset from a global pointer, and resets both the global variables and several other global variables to zero.
- `initialize_display_parameters`: Calculates display dimensions, bitplane counts, and color palette settings based on a bitmask retrieved from a function call.
- `swap_and_copy_pointers`: Swaps two global variables and copies several consecutive longwords from a memory location to a destination buffer.
- `update_dma_buffer_pointers`: Calculates a series of memory addresses based on an offset and a loop counter, then stores these resulting values into a sequence of global memory locations.
- `update_vertical_position`: Performs a loop to calculate a series of words based on an input value and stores them in a table, while interacting with a hardware register.
- `update_bit_patterns`: Stores a bitwise mask in a global variable, calculates a value based on the sign of d0, and writes a modified version of d0 to another global variable.
- `free_memory_via_callback`: Checks a global variable, and if non-zero, calls a function using a size from another global variable and then clears the global variable and another global variable.
- `allocate_memory_via_callback`: If a global variable is zero, it calls a function to allocate memory and stores the resulting pointer in two global variables; otherwise it returns -1.
- `verify_and_load_data_block`: Reads a data block from a file, allocates memory, and verifies that the loaded content begins with a specific four-byte signature before calling another routine and freeing the allocated memory.
- `call_exec_functions`: Calls two different library functions using addresses from a table and then restores registers before returning.
- `return_zero`: Returns a constant value of zero.
- `validate_and_process_data_header`: Checks a memory block for a specific magic signature and non-zero length before performing a series of recursive calls and data assignments.
- `unpack_compressed_data_stream_2`: Iterates through a table of offsets to locate data blocks and performs a custom decompression routine.
- `traverse_and_decrement_linked_list`: Iterates through a linked list by decrementing values at pointer locations and following next-node offsets.
- `initialize_interrupt_server`: Sets a global variable to a specific value and calls a function using a pointer from a global variable.
- `remove_interrupt_server`: Clears a global variable and sets another to a specific value before calling a function via a global pointer.
- `perform_blitter_transfer`: Calculates bitwise values and memory addresses to perform a series of hardware blitter operations using a loop.
- `setup_blitter_registers`: Writes several values to hardware registers to prepare for a blitter operation.
- `validate_structure_element`: Checks a byte within a data structure retrieved from another routine.
- `traverse_offset_list_2`: Iterates through a list of memory locations by adding an offset found within each element until a match is found.
- `find_linked_list_element_2`: Traverses a linked list of longwords by following pointers stored within the nodes.
- `update_dma_buffers`: Calculates a sequence of memory addresses and stores them into a series of global variables.
- `obfuscate_data_block`: Copies a sequence of longwords into global variables and applies a bitwise transformation to a data block.
- `search_linked_list`: Traverses a linked list of longwords, checking for a specific constant value, and returns a pointer to the element following the match.
- `traverse_offset_list`: Iterates through a list of structures using an offset stored within each element to find a matching longword.
- `copy_and_obfuscate_data_block`: Copies a sequence of longwords from a pointer into a series of global variables, performs a bitwise transformation on a data block using a loop, and then overwrites several global variables with the previously stored values.
- `process_data_structure`: Traverses a data structure and performs conditional operations based on values found within the elements.
- `expand_data_blocks`: Processes a multi-dimensional data structure by copying and expanding bytes based on sign-extended values and length counters.
- `copy_data_blocks`: Performs a nested loop to copy blocks of data from a source to a destination based on predefined dimensions.
- `copy_data_sequence`: Checks a condition against a global variable before copying a sequence of longwords from a memory location into a global table.
- `calculate_divisor_limit`: Performs arithmetic comparisons on input values and executes a loop to find a specific divisor based on global constants.
- `assign_global_pointers`: Copies values from two global variables into two other global variables.
- `copy_graphics_buffer_blocks`: Allocates memory and performs a nested loop to copy large blocks of data from source tables to a destination buffer.
- `calculate_sequence_values`: Computes a sequence of values through iterative addition and stores the results into a series of global memory locations.
- `clip_and_update_blitter_parameters`: Clips a coordinate pair against a bounding box and updates Blitter control registers and memory pointers based on the resulting offsets.
- `configure_blt_line_drawing`: Calculates and writes parameters to the Blitter registers to perform line drawing or shape filling.
- `configure_blitter_sequence_2`: Compares several values from global variables and registers, and if certain conditions are met, performs calculations to set up blitter parameters.
- `configure_blitter_sequence`: Calculates blitter parameters and source/destination addresses based on several global variables, then enters a loop that checks a bitmask in a global variable to conditionally update the blitter control, path, and size registers for a series of operations.
- `configure_blitter_parameters`: Waits for a bit in a global variable to clear before writing several values to memory locations used for hardware configuration.
- `apply_bitmask_to_memory_2`: Performs bitwise AND and OR operations on a memory buffer using a bitmask.
- `apply_bitmask_to_memory`: Performs bitwise AND and OR operations on a memory buffer using a bitmask.
- `extract_bitfield_value`: Checks if a set of coordinates is within a specific range before calculating a memory offset.
- `return_constant_value`: Returns a constant value of negative one.
- `calculate_line_raster_parameters`: Calculates slope, step increments, and coordinate offsets for line drawing.
- `generate_line_raster_parameters`: Calculates and stores incremental step values, slope constants, and coordinate offsets for drawing a line segment.
- `calculate_line_segment_parameters`: Calculates step increments, slope-related constants, and coordinate offsets for a line segment.
- `generate_line_segment_buffer`: Calculates the step increments and pixel data for a line segment based on start and end coordinates.
- `rasterize_line_segment_buffer`: Calculates the length and step increments for a line segment and populates a memory buffer with interpolated pixel data and pattern information.
- `render_3d_line_segment`: Calculates pixel coordinates for a line segment using linear interpolation and writes color and pattern data into a memory buffer.
- `calculate_and_render_line`: Performs linear interpolation to determine pixel coordinates and then writes color and pattern data into a memory buffer.
- `rasterize_transformed_polygon`: Calculates vertex positions via linear interpolation and populates a buffer with scanline data.
- `calculate_polygon_vertices`: Computes the intersection points of line segments using linear interpolation to determine the vertices of a transformed shape.
- `render_scaled_bitplane_graphics`: Calculates scaling parameters and performs bitplane-aware blitter transfers to render transformed graphics.
- `apply_bitmask`: Calls another routine to perform bitwise operations on memory.
- `setup_blt_parameters`: Calculates transfer parameters and configures bitplane transfer hardware registers within a loop.
- `configure_blt_transfer`: Calculates parameters and updates several hardware registers to perform a Blitter transfer.
- `process_keyboard_input`: Reads from a serial register and stores processed bit-manipulated data into a table.
- `open_resource_and_initialize`: Opens a resource and uses a pointer from a global variable to call several functions through a retrieved library interface.
- `close_resource_and_clear`: Calls two functions via a pointer retrieved from a global variable and then clears that global variable.
- `fetch_byte_from_table`: Returns a word composed of a high byte and a byte fetched from a table using an index.
- `pop_queue_element`: Removes a word from a table and shifts the remaining elements forward.
- `get_recursive_bit_shifted_value`: If a global variable is non-zero, it calls another routine and constructs a 16-bit value using a byte from a table lookup at an index derived from the returned value and a bit-shifted version of that value.
- `get_recursive_concatenated_value`: If a global variable is non-zero, it calls another routine and constructs a 16-bit value by concatenating the returned byte with a byte from a table lookup at an index derived from the returned value and a bit-shifted version of that value.
- `fetch_joystick_table_value`: Processes bits from the joystick data register to calculate an index and returns a byte from a table at a calculated offset.
- `calculate_joystick_mask`: Reads the joystick status register, performs bitwise operations to derive an index, and constructs a bitmask by ORing several bytes from a table into a single byte.
- `modify_d0_register`: Tests bit 0 of d0 and branches; otherwise it performs bitwise operations on d0 using a value from a memory location and returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_blitter_operation`: Sets multiple Blitter mode and control registers to perform a hardware graphics transfer.
- `initialize_blitter_registers`: Writes specific values to Blitter registers to prepare for a transfer.

### Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:amigasnd.drv

- `execute_indirect_call`: Performs an indirect function call using a pointer derived from a global variable and a register value.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `check_status_match`: Searches through a table for a matching pair of bytes and sets or clears bytes in a structure based on the result.
- `set_status_bytes`: Sets one byte to one and another byte to zero at specific offsets within a structure.
- `update_status_flags`: Compares two bytes against values in two different tables and sets or clears bytes in a global structure based on the result.
- `clear_matching_entries`: Compares two byte values against a sequence of four entries in a table and stores a zero at a specific offset if both bytes match.
- `search_and_clear_byte_2`: Iterates through a sequence of four entries to find a match for two byte values and stores a zero at a specific offset if a match is found.
- `search_and_clear_byte`: Compares two byte values against a sequence in two separate memory locations.
- `update_audio_command_state`: Processes a command byte to either update a byte in a table, store a value at a calculated offset, or iterate through a four-entry sequence to find a matching pair.
- `process_audio_command`: Evaluates a command byte to either store a byte in a table at a calculated offset or search a sequence of four entries for a matching pair.
- `scan_audio_table`: Updates a byte in a table based on a specific value or scans a sequence of four entries to find a match where a secondary byte is not negative.
- `conditionally_scan_table`: Checks if a byte matches a value in a table while ensuring a secondary byte at a specific offset is not negative.
- `search_audio_data`: Compares a character against a sequence of bytes in a global structure and updates a local variable if a match is found.
- `store_byte_at_offset`: Stores a byte into a memory location calculated from a base address, an offset, and a word value.
- `update_audio_register`: Calculates a value based on input registers and stores it in a memory location, then iterates through a four-byte sequence to check for matching values.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `get_and_set_byte`: Returns a byte from a memory location and, if the input byte is not negative one, updates that location with the input byte.
- `get_and_set_word`: Returns a word from a memory location and, if the input word is not negative thirty-two thousand seven hundred sixty-eight, updates that location with the input word.
- `update_byte_via_pointer`: Reads a byte from a memory location and conditionally overwrites it with a value from a register if that register does not contain a specific negative value.
- `update_index_table_entry`: Iterates through a data table to find a matching byte and updates a value at a calculated offset within a structure.
- `update_index_table`: Iterates through a table to find a matching byte and updates a value at a calculated offset within a structure.
- `compare_and_recursively_update_audio`: Compares a byte value against a four-byte sequence in a table and calculates a new value based on the number of matches.
- `call_routine_twice`: The routine calls another routine twice in succession and then returns.
- `process_byte_stream_2`: Counts occurrences of a specific byte in a buffer and subtracts that count from values in another memory area.
- `subtract_and_store_byte`: Subtracts two values and stores the result in a memory location pointed to by a register.
- `process_byte_stream`: Iterates through a memory buffer, skipping zero bytes and subtracting values from a register until a condition is met, then writes the remainder back to the buffer.
- `modify_audio_channel`: Searches for a specific byte value in a structure, updates it, and increments a value at a calculated memory offset.
- `update_sample_envelope_values`: Compares a byte value against a table and either decrements the stored value or resets it to zero.
- `update_audio_sample_indices_2`: Iterates through a small table to match specific byte values and updates corresponding entries in a data structure.
- `update_audio_sample_indices`: Iterates through a small table to match specific byte values and updates corresponding entries in a data structure.
- `process_audio_indices`: Uses a loop to traverse a table and update bytes in a structure based on matching values.
- `execute_empty_routine`: This routine consists of ten instructions but performs no observable operation, as it does not modify any registers, memory, or hardware state before returning.
- `update_status_table`: Stores a byte at a calculated offset and, if that byte is zero, iterates through a table to update specific flags based on a matching index.
- `initialize_audio_sample_event`: Clears several fields in a global structure associated with a specific index and searches a sample table for a matching time range.
- `register_audio_event`: Searches a table for a specific value, then populates a global structure with various data points including pointers and calculated offsets.
- `configure_audio_parameter`: Writes multiple values to a structure based on an index and updates a specific byte in a table after calling another routine.
- `call_audio_reset`: Calls a routine that clears specific memory locations and resets the audio hardware registers.
- `clear_audio_channel_data`: Resets several memory locations and offsets within a global structure to zero.
- `select_audio_parameter_and_write`: Determines a 16-bit value based on a stack-based condition and combines it with a 24-bit value to be returned.
- `return_stack_value`: Returns a 24-bit value formed by combining a constant with a 4-byte field taken from a 32-bit value located on the stack at a fixed offset.
- `update_audio_voice_counters`: Iterates through a sequence of four entries and increments specific values in a table based on the contents of a structure.
- `process_audio_envelope_transitions`: Manages the state machine for an audio channel's volume envelope by incrementing or decrementing counters and updating volume levels using lookup tables.
- `update_audio_envelope_state`: Updates an audio channel's volume and envelope state by incrementing or decrementing a counter and applying values from lookup tables.
- `add_audio_data_value`: Adds a value from a table to a byte in a structure and clamps the result to a maximum value.
- `process_audio_envelope`: Modifies a byte in a structure using values from lookup tables and updates a status byte based on conditional checks.
- `subtract_audio_data_value`: Subtracts a value from a table from a byte in a structure and clamps the result to a minimum value.
- `transform_audio_sample_stream`: Iterates through a data stream to perform byte-wise subtraction and replacement using values retrieved from lookup tables.
- `decrement_audio_envelope`: Decrements a byte in a structure and performs a table lookup to update another value if a threshold is reached.
- `modify_audio_data`: Subtracts a value from a byte in a structure using an offset from another table and calls itself recursively if a condition is met.
- `compute_volume_presence_flag`: Calculates a product of three bytes retrieved from tables and a scaled short value, then stores a boolean flag in a global variable based on whether the result is non-zero and a specific condition is met.
- `calculate_volume_scaling_3`: Calculates a value based on the product of multiple inputs and stores the result in a global variable.
- `calculate_volume_scaling_2`: Calculates a scaled volume value by multiplying multiple inputs and dividing by a constant, then stores the result in a memory location.
- `update_volume_scaling_byte`: Calculates a scaled volume value by multiplying two inputs and dividing by a constant, then stores the result in a memory location.
- `calculate_volume_scaling`: Calculates a value based on the product of two inputs and stores the result in a global variable.
- `populate_audio_sequence_2`: Populates a structure and a secondary table with data extracted from a pointer-based sequence.
- `populate_audio_sequence`: Populates a structure and a secondary table with data extracted from a pointer-based sequence.
- `initialize_audio_sequence`: Sets a flag and populates a structure with data from a pointer-based sequence before calling another routine.
- `reset_audio_hardware_state`: Calls a routine to reset the audio hardware and returns a value from a memory location via a pointer.
- `check_handler_table`: Compares a value against a table of four integers and calls another routine if a match is found.
- `recursive_call`: The routine contains no instructions and immediately returns.
- `update_channel_state`: Searches a table for a matching integer, then writes a byte and a word to specific offsets in a structure.
- `extract_and_clear_byte`: Writes a byte to a global structure, updates a word at a calculated offset, and retrieves a byte from a table before clearing it.
- `clear_and_return_byte`: Sets a word at a fixed offset to one, retrieves a byte from a table using an index, clears that byte in the table, and returns the byte shifted left by sixteen bits.
- `fetch_audio_parameter`: Retrieves a value from one of several tables based on a mode and index provided in a register.
- `fetch_audio_table_value`: Returns a byte from one of several tables based on a mode value and an index.
- `select_sequence_data`: Selects a byte from one of several tables based on a mode value and an index.
- `select_sequence_data_offset`: Selects a byte from one of several tables based on a mode value and an index.
- `fetch_sequence_index`: Retrieves a value from a table based on a byte index or searches for a specific sequence in a table.
- `search_sequence_index`: Searches a table for a specific four-byte sequence or retrieves a value from a table using a byte as an index.
- `find_sequence_index`: Searches a table for a sequence of four matching bytes and returns a value derived from the index found.
- `return_constant_value_2`: Returns a constant 16-bit value.
- `update_buffer_pointers`: Stores a value in one memory location and adds a base address to four other memory locations.
- `get_concatenated_data`: Concatenates two 16-bit values from a table into a single 32-bit result.
- `initialize_audio_structures`: Allocates memory and populates a series of offsets with a base address, then initializes several interrupt handler structures with specific values and function pointers.
- `initialize_audio_device`: Opens the audio device and initializes a series of interrupt handler structures with specific parameters and function pointers.
- `initialize_audio_interrupt_handlers`: Initializes a table of structures with specific values and function pointers, then writes to a hardware register to enable interrupts.
- `release_audio_resources`: Performs cleanup operations including calling functions via pointers and interacting with system-level resources.
- `reset_audio_hardware`: Clears specific memory locations and resets the audio hardware register.
- `update_audio_registers_2`: Loads a byte and a word from a table into specific hardware registers and clears a third register.
- `execute_conditional_delay_loop`: Checks a value in a table and, if zero, executes a loop that decrements a counter derived from multiple memory lookups and arithmetic offsets.
- `delay_loop_execution`: Executes a loop that decrements a counter based on values retrieved from multiple tables.
- `return_immediately_3`: This routine performs no operations and returns control to the caller.
- `perform_no_operation`: This routine executes a series of instructions that result in no functional change to the processor state before returning to the caller.
- `return_immediately_2`: This routine is empty and performs no operations before returning to the caller.
- `write_audio_register`: Writes a 16-bit value to a hardware register.
- `write_audio_register_alt`: Writes a 16-bit value to the audio hardware register.
- `decrement_and_update_audio`: Decrements a byte in a table and, if it reaches a specific value, updates hardware registers with values from a table.
- `update_sample_buffer`: Copies a large block of sample data from a source buffer to a destination buffer based on an index and offset.
- `update_audio_hardware`: updates a hardware register with a value from a table and modifies a memory location based on a calculated index.
- `store_table_word`: Stores a 16-bit word from a table into a memory location.
- `update_voice_sample_buffer`: Copies a large block of sample data from a source buffer to a destination buffer based on an index.
- `update_audio_hardware_alt`: updates a hardware register with a value from a table and clears a flag in a memory location.
- `store_table_word_2`: Stores a 16-bit word from a table into a memory location via a pointer.
- `validate_and_set_global_byte`: Checks the values of two consecutive global variables and recursively calls itself if specific conditions are met, otherwise sets a global variable to a constant value.
- `set_sample_flag`: Checks a byte via a pointer and then stores a constant value into a global variable.
- `set_sample_playback_flag`: Checks a byte via a pointer and then stores a constant value into a global variable.
- `set_interrupt_request`: Calls another routine and then sets a bit in a hardware register.
- `reset_audio_state_flag`: Evaluates a series of global variables through conditional checks and either recurses or sets a global variable to a constant value.
- `update_audio_state_flag`: Sets a global variable to a constant value and conditionally writes a byte to a memory location pointed to by a register.
- `update_audio_sample_buffer`: Copies a block of sample data to a destination buffer and sets a flag in a global variable to signal an audio interrupt request.
- `set_audio_interrupt_flag_high`: The routine calls a helper to copy sound samples and then sets a different bit in a global audio state variable to signal an interrupt request.
- `set_audio_flag_low`: Checks a series of global variables and writes a constant value to a global variable.
- `update_audio_sample_status`: Checks two memory locations for null values and writes a byte to one of them if it is non-zero.
- `update_audio_sample_buffer_2`: Updates a sample buffer by copying data and then triggers an audio interrupt request.
- `set_audio_interrupt_request`: The routine writes a constant to the INTREQ register and then calls another routine.
- `update_audio_state_flag_2`: Checks a global variable and performs recursive calls based on its value before setting a global variable to a constant value.
- `reset_interrupt_flag`: Checks a byte via a pointer and then writes a constant value to a global variable.
- `update_audio_sample_state`: Checks a byte via a pointer and then writes a constant value to a global variable.
- `reset_audio_interrupt_flag`: The routine clears a hardware interrupt request flag by writing a constant to a global variable that maps to the INTREQ register after calling another routine.
- `write_hardware_constants`: Writes specific constant values to two hardware registers.
- `modify_data_table`: Iterates through a table of pointers to find specific entries and performs arithmetic on a range of bytes within a memory block.
- `process_data_table`: iterates through a table to find specific entries and performs arithmetic on bytes within a memory range.

### Nova 9 (1991)(Dynamix)(Disk 3 of 3).adf:story

- `initialize_resource_loading_sequence`: Checks a magic constant in a data stream and initiates a deeply nested series of conditional calls to load, validate, and decompress animation and graphics resources.
- `execute_conditional_routine_chain`: Calls another routine and sets a global variable to one if the return value is negative, then calls that same routine a second time.
- `poll_hardware_input`: Repeatedly calls another routine to check values from a hardware register and performs table operations based on the results.
- `process_input_loop`: Iterates through a loop checking a hardware register and compares a value against a global variable until a specific condition is met.
- `load_address_from_table`: calculates an offset from a local variable to index into a table and stores the resulting value into a global variable.
- `modify_register_values`: Writes the byte in d0 to the stack twice and then performs several bitwise OR operations on d0.
- `copy_table_data_2`: Copies a block of data from a source table to two different destination memory locations.
- `copy_table_data`: Copies a block of data from a source table to two different destination memory locations.
- `increment_global_word`: Increments a 16-bit global variable by one.
- `loop_until_null`: Repeatedly calls another routine until it returns a zero value.
- `swap_boundary_values`: Assigns the value of one global variable to another, calls a routine, and then repeats the process using a different global variable.
- `process_magic_header`: Checks a memory pointer for a specific magic constant and updates several global variables and status flags through a conditional chain.
- `configure_video_display_parameters_2`: Calculates screen dimensions, bitplane counts, and memory requirements based on a bitmask.
- `initialize_resource_pointer`: Sets a global variable to zero and attempts to populate it by calling a function through a pointer stored in a global variable.
- `check_magic_constant`: compares a value at a pointer to a specific constant and calls a function pointer if they match.
- `decompress_structured_data_stream`: Parses a control-coded data stream to perform variable-length decoding.
- `validate_and_reset_resource_pointers`: Checks a global pointer through an indirect call and conditionally executes further indirect calls based on the result.
- `clear_and_call_handler`: checks if a global variable is non-zero, sets it to zero, and executes a function pointer from a global structure.
- `copy_data_block_3`: Copies a sequence of longwords from a source pointer into a global variable while waiting for a specific bitmask condition.
- `initialize_global_state`: sets a global variable to negative one, decrements other global variables, and executes a function pointer from a global structure.
- `release_resource_handle`: Checks if a global pointer is non-zero and, if so, executes two functions via that pointer before clearing the pointer.
- `cleanup_graphics_resources`: Releases allocated memory and system resources by invoking function pointers from a global structure.
- `update_clipping_rectangle`: Adjusts a rectangular boundary by comparing input coordinates against predefined limits and recalculating dimensions.
- `pop_table_element_2`: decrements a global counter and shifts elements in a table to remove the first entry.
- `reset_interrupt_state`: Clears a global variable and sets another to a specific value before calling a routine via a pointer in a global variable.
- `update_dma_parameters`: swaps two global variables and copies a series of calculated longwords into a sequence of global variables.
- `validate_resource_type`: Checks various global values and function return codes to determine if a resource matches specific signature constants.
- `perform_conditional_copy`: Copies bytes from a source to a destination while checking against a global table and potentially calling another routine if a bitwise match is found.
- `fall_through_entry_9`: Entry point part-way through straight-line code: runs one instruction (lea.l) and then falls through into the code that follows, with no return of its own.
- `write_long_to_table_2`: Waits for bit 8 of the VPOSR register to be set, then stores a 32-bit value into a table as two words at an offset calculated from a value in d0.
- `update_pointer_value_via_bit_shifts`: Checks if a pointer is null, then performs a series of bitwise rotations and shifts on values in d0, d1, and d2 to construct a new 32-bit word, which is then stored at the address pointed to by a0.
- `reset_hardware_registers_2`: Checks if a global variable is zero and if a value in a register is within a specific range, then clears four longwords at specific offsets in a structure and sets a value in a custom register.
- `sync_display_parameters`: Synchronizes a series of longwords from a source pointer into a global variable while waiting for a specific bitmask condition.
- `rotate_palette_colors`: Calculates a bitwise rotation offset for color components and uses it to transform a sixteen-word color table into a destination buffer via a loop.
- `transform_palette_colors_2`: Copies a sixteen-word table to a destination buffer, then iterates through a specified range of color entries to apply a signed offset to each color component while clamping the results within a four-bit range.
- `transform_palette_colors`: Iterates through a source table to extract color components, applies a signed offset and clamps the results between two bounds, and writes the transformed 16-bit color values to a destination table.
- `apply_clamped_palette_transform`: Iterates through a source table, extracts bytes, applies a signed offset and clamps the result between two bounds, performs a bitwise rotation, and writes the transformed 16-bit value to a destination pointed to by a1.
- `write_clamped_palette_data`: Calculates a clamped value between two bounds, performs a bitwise rotation, and writes the result to a destination pointed to by a1 in a loop.
- `process_conditional_copy`: Copies bytes from a source to a destination while performing bitwise comparisons against a global table and potentially calling another routine via a jump table.
- `copy_and_process_data`: Copies three words from one memory location to another and then enters a loop that performs bitwise comparisons and conditional byte transfers.
- `decompress_custom_data_stream`: Parses a structured data stream containing control codes to perform variable-length decoding.
- `copy_memory_block_2`: Calls another routine twice to obtain two memory addresses and then copies ten words from the first address to the second address.
- `calculate_offset_address_2`: Performs arithmetic on input registers to calculate and return a specific memory address.
- `copy_ten_words_2`: Obtains two memory addresses from another routine and copies ten words from the first address to the second.
- `manage_library_resources`: Interacts with graphics and intuition libraries to configure display parameters and clean up allocated resources while manipulating interrupt enable registers.
- `configure_video_display_parameters`: Calculates display dimensions, bitplane counts, and color palette values based on a bitmask.
- `cleanup_library_resources`: Executes a sequence of function calls to check global pointers, frees associated memory, and clears those pointers while interacting with intuition.library and hardware registers.
- `clear_memory_pointers`: Checks several global pointers for non-zero values, calls a function to free the associated memory, and then clears those pointers and several other global variables.
- `initialize_video_buffer_structures`: Calculates display dimensions and bitplane counts based on a bitmask, then populates a table with memory addresses for video buffers.
- `swap_and_copy_pointers_2`: Swaps two values in a table and then copies a series of longwords from a code location into several global variables.
- `modify_dma_control`: Swaps two values in a table and then copies a series of calculated longwords into a set of global variables while touching the DMA control register.
- `copy_calculated_longwords`: Calculates a series of longwords based on an input and stores them into a set of global variables while updating the video port register.
- `modify_word_and_store`: Stores a bit-manipulated version of a word into a global variable and updates two other global variables based on the sign bit and a masked value.
- `free_memory_via_callback_2`: Checks a global variable, and if non-zero, calls a function using a size from another global variable and then clears the global variable and another global variable.
- `allocate_memory_via_callback_2`: Checks if a global variable is zero, and if so, calls a function to allocate memory and stores the resulting pointer in two global variables.
- `load_and_decompress_file`: Opens a file, reads its contents, and performs several library calls to manage memory and file handles; if a specific four-byte signature is found in the data, it calls decompress_custom_data_stream and returns negative one, otherwise it returns zero.
- `close_and_free_resources`: Calls two different functions using values from registers and then returns zero.
- `update_mouse_input_state`: Reads mouse movement from a hardware port and updates clamped coordinate values within a data structure.
- `update_clamped_parameters`: Clamps values against a table, performs arithmetic to derive new values, and stores the results into a series of memory locations.
- `process_clamped_values`: Clamps input values against a table, performs arithmetic to derive new values, and executes a callback function via a pointer in a table.
- `clamp_value_and_execute_callback`: Clamps a value between two bounds stored in a table and performs arithmetic to store the result and a derived byte into a memory location.
- `update_table_and_execute_callback`: Updates a table entry with a clamped value and performs arithmetic to store derived bytes into a memory location.
- `initialize_interrupt_handler`: Sets several global variables to zero or decremented values and executes a function pointer from a global structure.
- `remove_interrupt_handler`: Resets a global variable to zero and executes a function pointer from a global structure.
- `store_sequential_pointers`: Stores a word from d0 into a global variable, then sequentially calls another routine four times to store the values from a0, the return value of the first call, a2, and a3 into a table at offsets 0, 4, 8, and 12.
- `execute_sequential_calls`: If d0 is not zero and a global variable is zero, it stores d0 into that global variable, calls another routine, and then performs four sequential calls to another routine using values 0, 1, 2, and 3 in d0 with addresses loaded from a table.
- `store_and_wait_vpos`: Stores d0 into a global variable, calls another subroutine three times with d0 set to 1, 2, and 3, and then enters a loop that waits for a specific bit pattern in the custom video port register.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `clip_and_fill_rectangle`: Clips a rectangular area against defined boundaries and uses the blitter to perform a patterned area fill.
- `update_blitter_parameters_2`: Calculates blitter and DMA control values based on global variables and updates hardware registers.
- `update_blitter_parameters`: Calculates values based on several global variables and updates multiple hardware registers including the blitter and DMA control.
- `configure_blitter_parameters_2`: Waits for bit 6 of a custom register to clear before writing several values to memory locations used for hardware configuration.
- `apply_bitmask_to_buffer`: Performs bitwise operations on a memory buffer using a bitmask derived from a global variable.
- `test_bit_pattern`: Checks a bitmask within a table at a calculated offset.
- `return_all_ones`: The routine returns a value consisting of all bits set to one.
- `execute_blitter_area_fills`: Configures and triggers the Amiga Blitter to perform area fills or block transfers using calculated dimensions and bitmask patterns.
- `process_keyboard_input_2`: Reads a byte from the keyboard serial register, updates a global variable, and stores a bit-manipulated value into a table if a condition is met.
- `open_resource_handler`: Attempts to open a resource and, if successful, performs several operations using the returned pointer before returning a failure code.
- `close_resource_handler`: Checks if a resource pointer is valid and, if so, performs two operations using that pointer before clearing it.
- `get_concatenated_byte`: Returns a 16-bit value formed by concatenating a bit-shifted version of a register with a byte from a table.
- `pop_table_element`: Decrements a global counter and shifts elements within a table to remove the first entry, returning the removed value.
- `get_recursive_bit_shifted_value_2`: If a global variable is non-zero, it calls another routine and constructs a 16-bit value using a byte from a table lookup at an index derived from the returned value and a bit-shifted version of that value.
- `get_concatenated_table_value`: If a global variable is non-zero, it calls another routine to retrieve a byte, then constructs a 16-bit value by concatenating that byte with a byte retrieved from a table using an index calculated from the retrieved byte and its own bit-shifted value; otherwise it returns zero.
- `fetch_joystick_table_value_2`: Calculates an index by processing bits from the joystick 1 data register and returns a byte from a table at that calculated offset.
- `calculate_joystick_mask_2`: Reads the joystick 1 data register, calculates an index via bitwise manipulation, and returns a byte formed by ORing a value from a table at that index with a bitmask constructed from multiple bytes in another table.
- `process_joystick_bits`: Checks bit 0 of d0, and if not set, performs arithmetic and bitwise operations on d0 before returning.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `configure_blitter_transfer`: Sets up Blitter hardware registers and memory locations using values derived from input registers and a loop.
- `setup_blitter_state`: Writes specific values to memory locations and Blitter registers to prepare for a hardware transfer.
- `validate_resource_header`: Checks a memory block for specific magic signatures and validates its structure using library function calls.
- `execute_conditional_callback`: Checks if a value at a pointer matches a specific four-byte constant and, if so, calls a function pointer stored at a fixed offset from a global variable.
- `copy_with_conditional_branch`: Copies a block of bytes from a source to a destination while checking each byte against a global table.
- `process_complex_conditional_chain`: Checks a magic constant, updates several global variables, and executes a sequence of conditional routines based on various status checks.
- `expand_nested_data_streams`: Iterates through a structured data format to expand and copy byte sequences into a destination buffer.
- `expand_nested_data_stream`: Iterates through a structured data format to copy byte sequences into a destination buffer.
- `decompress_nested_data_streams`: Traverses a structured data format to expand and copy byte sequences into a destination buffer using nested loops and length descriptors.
- `decrement_and_check_counter_2`: Decrements a value at a pointer offset and traverses a linked list if the value reaches zero.
- `decrement_and_check_counter`: Decrements a value at an offset from a pointer and calls another routine if the value reaches zero.
- `initialize_interrupt_server_2`: Sets a global variable to a specific value and calls another routine if a certain condition is met.
- `remove_interrupt_server_2`: Resets a global variable and calls another routine if a certain condition is met.
- `configure_blitter_transfer_2`: Calculates several values and uses them to populate Blitter registers and memory locations through a loop.
- `setup_blitter_state_2`: Writes specific values to Blitter registers and memory locations based on global variables.
- `evaluate_conditional_chain`: Recursively calls another routine and checks a byte at a specific offset from the returned pointer to determine control flow.
- `traverse_data_chain_2`: Walks through a linked list of data elements by following offsets until a target value is found or the end is reached.
- `search_linked_list_2`: Traverses a linked list of structures identified by a specific four-byte constant to find an entry matching a given value.
- `update_dma_parameters_2`: Swaps two global values and populates a series of memory locations with calculated longwords based on a loop.
- `sync_global_state_and_transform_table`: Copies a sequence of longwords from a pointer into a series of global variables, performs a bitwise nibble-swapping transformation on a data block, and waits for a specific bitmask condition in a global variable before updating that variable with the previously stored values.
- `search_linked_list_3`: Iterates through a list of data blocks marked by a constant signature to locate a specific entry based on a provided value.
- `traverse_data_chain`: Walks through a chain of data elements by using an offset stored within each element to find a matching value.
- `update_data_structures`: Copies a series of longwords from a pointer to several global variables, modifies a sequence of values using bitwise operations, and waits for a bitmask condition in a global variable before updating a global pointer.
- `evaluate_conditional_data_chain`: Recursively calls another routine and checks a byte at a specific offset from the returned pointer.
- `process_nested_buffers`: Performs complex data manipulation on nested buffers, involving sign-based branching and byte copying across multiple loops.
- `copy_multidimensional_data`: Copies blocks of data using nested loops to move bytes from a source to a destination based on calculated dimensions.
- `synchronize_data_block`: Waits for a specific bitmask condition in a global variable before copying a series of longwords from a source pointer to that global variable.

### Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine`: This routine is empty and performs no operations before returning.
