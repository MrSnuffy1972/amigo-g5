# Game 2537 (nitro (1990)(psygnosis)[cr hz][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02537). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903760 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 43 of 43 named (0 author symbols), 43 of 43 described (0 by rule, 35 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 456 of 456 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 512 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 785 of 785 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 18628 |
| code | 4368 |
| data | 429692 |
| unused | 451072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Nitro (1990)(Psygnosis)[cr HZ][a2].adf:bootblock | 1 | 7 | 1 |
| logged disk load from Nitro (1990)(Psygnosis)[cr HZ][a2].adf boot-block load | 1 | 86 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 692 | 40 |
| Nitro (1990)(Psygnosis)[cr HZ][a2].adf bytes 1024-1536 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- nitro (1990)(psygnosis)[cr hz][a2]/Nitro (1990)(Psygnosis)[cr HZ][a2].adf: custom-format (non-DOS) disk, 877 non-empty blocks: matched against track-loaded segments
- nitro (1990)(psygnosis)[cr hz][a2]/Nitro (1990)(Psygnosis)[cr HZ][a2].adf: the boot block reads 18944 bytes from disk offset 425984 to an address (found statically); recovered as a unit
- runtime segment an address: 456 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- nitro (1990)(psygnosis)[cr hz][a2]/Nitro (1990)(Psygnosis)[cr HZ][a2].adf: bytes 1024-1536 look like code no run loaded: recovered position-independently (load address unknown)
- nitro (1990)(psygnosis)[cr hz][a2]/Nitro (1990)(Psygnosis)[cr HZ][a2].adf: 2 logged loads cover 19456 bytes of the image
- nitro (1990)(psygnosis)[cr hz][a2]/Nitro (1990)(Psygnosis)[cr HZ][a2].adf: 4 track-loaded blocks found in recovered runtime segments and classified through them
- nitro (1990)(psygnosis)[cr hz][a2]/Nitro (1990)(Psygnosis)[cr HZ][a2].adf: blocks 7-830 (bytes 3584-425472) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.61)
- nitro (1990)(psygnosis)[cr hz][a2]/Nitro (1990)(Psygnosis)[cr HZ][a2].adf: blocks 869-879 (bytes 444928-450560) outside the filesystem hold data, not code (opcode density 0.032, entropy 5.47)
- 1 never-loaded region(s), 512 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Nitro (1990)(Psygnosis)[cr HZ][a2].adf:bootblock

- `call_library_function`: Calls a library function via a system call.

### logged disk load from Nitro (1990)(Psygnosis)[cr HZ][a2].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_control_flags`: Clears several bytes or words in a structure to a default or reset value and executes a conditional branch or call.
- `copy_memory_block`: Copies a block of memory from a source pointer to a destination pointer until a specific terminator value is encountered.
- `decrement_and_branch_table`: Decrements a counter, checks a condition, and either executes a branch or resets several control flags and updates a pointer-based jump target.
- `swap_struct_word_pairs`: Swaps two 32-bit values stored at adjacent offsets within a structure.
- `set_and_wait_for_zero`: Sets a value at an offset in a structure to zero and then loops until that same memory location contains zero.
- `initialize_table_data`: Initializes a table or buffer with a sequence of values, including a loop that increments a counter and writes a pattern of even numbers and associated data or pointers.
- `decrement_loop_counter`: Calculates a loop limit based on a masked value from a structure and executes a loop that decrements a counter.
- `load_and_store_indexed_values`: Loads a pointer and a word from the structure in a4, indexes into a table using the pointer, stores a modified word at offset 62e and two more words at offsets 654 and 656, then branches onward.
- `store_struct_fields`: Copies certain short or word values from a source structure into a destination structure and executes a sequence of other routines.
- `set_copy_size`: Writes a constant value to a global variable and calls a routine that copies a block of words.
- `update_status_flag`: Checks if a value at a pointer is negative, then writes either a constant or the value itself to a global variable and sets a status flag.
- `store_value_to_global`: Stores a word from a pointer into a global variable.
- `update_structure_values`: Copies a word from a pointer in a4 to offset 1616 of a structure in a5, writes 6 to offset 1612 of that structure, and returns.
- `copy_triple_word`: Copies three consecutive words from a pointer into a global structure and sets a trailing word to zero.
- `execute_jump_table`: Loads a table address into a2, pops several values from a4 into registers, and performs an indirect jump using a value from a table indexed by a register.
- `store_word_at_indexed_offset`: Looks up an offset from a table indexed by the first word addressed by the pointer, then stores the second word at that offset in the slot record.
- `load_pair_from_table`: Reads a value from a table and loads two adjacent words from a fixed offset in that table into a global structure.
- `transform_nibble_data`: Calculates a new nibble-based value by multiplying parts of a source word by a provided value and storing the result in a destination pointer.
- `copy_block_of_words`: Calculates a block size based on a global variable and a table value, then copies a block of words from one pointer to another.
- `increment_stream_pointer`: Increments a pointer or counter within a structure and conditionally updates a secondary counter or a value in a table.
- `decode_variable_length_stream`: Parses a stream of bytes that includes instructions for copying or skipping data, using a lookup table to determine jump offsets or copy lengths.
- `initialize_complex_buffer`: Initializes a buffer or structure with a mix of constant values, copied data from a source, and a loop that clears a memory range to zero.
- `update_struct_members_variant`: Performs a similar sequence to routine R1, writing a pointer-derived address and bit-shifted values to a different set of offsets in a structure.
- `decode_direction_bits_to_steps`: Decodes direction bits from a register value into two signed step values, one per axis, using exclusive-or of paired bits.
- `update_clamped_values`: Calculates the difference between a source and a target byte and updates a signed or unsigned counter, clamping the result within a specific range.
- `update_state_flags`: Checks bit flags in a structure and, depending on the result, updates counters, modifies values in a table, or executes other routines.
- `clear_bit_write_port_pair`: Clears bit 1 of a control byte, writes two values to port-like offsets and clears a third byte.
- `handle_flag_invert_and_wait`: Checks two condition bits in a structure in a6 and, when they are set, inverts a byte at offset 3072, writes 81 to offset 3584, and waits for a flag at offset -2049 to change before clearing it.
- `loop_routine_by_table_count`: Reads a byte from a global table and uses it as a loop counter, calling another routine each iteration until the counter reaches zero, then returns.
- `conditional_update_global`: Loads a 32-bit value from offset 1576 of a structure in a5, performs bit tests on a doubled version of that value, and either branches to another routine or stores the value back to offset 1576 of the structure before returning.
- `increment_and_store_value`: Tests bit 1 of d0; if it is not set, increments d0, then stores the resulting 32-bit value into offset 1576 of a structure in a5 and returns.
- `check_flags_and_decrement`: Reads two adjacent words of a structure in a6, masks off several bits, and branches to a label when none of the tested bits are set.
- `set_flag_and_branch`: Writes 128 to offset 156 of a structure in a6, loads the address at offset 130 of a structure in a5 into a0, and branches to another routine.
- `set_flag_and_branch_alt`: Writes 256 to offset 156 of a structure in a6, loads the address at offset 150 of a structure in a5 into a0, and branches to another routine.
- `set_flag_and_branch_alt2`: Writes 512 to offset 156 of a structure in a6, loads the address at offset 170 of a structure in a5 into a0, and branches to another routine.
- `set_flag_and_clear_bit`: Writes 1024 to the word at offset 92 of the structure in a6, then checks a byte at offset 18 of a structure in a5 and clears it when it is set before returning.
- `decrement_and_branch`: Tests the word at offset 8 of a structure in a0; if it is not zero, decrements it and branches to another routine.
- `clear_fields_then_branch`: Copies a pointer from a structure in a0 to another register, clears several fields at offsets 8 and 10 of that structure and a byte at offset 19, copies a value from offset 6 to offset 154 of the structure in a6, then branches into another routine and returns.
- `write_pending_word_to_chips`: When a pending word is nonzero, writes it with bit 15 set to one chip register and writes it shifted left by seven to another, then clears the pending word.
- `update_clamped_accumulator`: Adds a value to a counter and updates a secondary or pointer-based value, clamping the result or resetting it if it falls below zero or exceeds a limit.
