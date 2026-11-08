# Game 768 (baal (1988)(psyclapse)[cr defjam - ccs][t defjam]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00768). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905840 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 14 raw-word lines |
| Functions named and described | pass, 27 of 27 named (0 author symbols), 27 of 27 described (4 by rule, 23 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 174 of 174 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 675 of 675 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1823 |
| code | 1938 |
| data | 3007 |
| unused | 899072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Baal (1988)(Psyclapse)[cr Defjam - CCS][t Defjam].adf:bootblock | 1 | 17 | 1 |
| logged disk load from Baal (1988)(Psyclapse)[cr Defjam - CCS][t Defjam].adf boot-block load | 1 | 124 | 4 |
| logged disk load from Baal (1988)(Psyclapse)[cr Defjam - CCS][t Defjam].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 521 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |

## Cases handled

- baal (1988)(psyclapse)[cr defjam - ccs][t defjam]/Baal (1988)(Psyclapse)[cr Defjam - CCS][t Defjam].adf: the boot block reads 1024 bytes from disk offset 2048 to an address (found statically); recovered as a unit
- baal (1988)(psyclapse)[cr defjam - ccs][t defjam]/Baal (1988)(Psyclapse)[cr Defjam - CCS][t Defjam].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 164 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 10 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- baal (1988)(psyclapse)[cr defjam - ccs][t defjam]/Baal (1988)(Psyclapse)[cr Defjam - CCS][t Defjam].adf: 2 logged loads cover 0 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Baal (1988)(Psyclapse)[cr Defjam - CCS][t Defjam].adf:bootblock

- `init_and_jump_bootblock`: Initializes a structure pointed to by a1 with four fixed constants, calls a Kickstart routine and then a routine at the address stored at offset 60 of that structure, then repeats the initialization with different constants and jumps to the address stored at offset 60 of a different structure.

### logged disk load from Baal (1988)(Psyclapse)[cr Defjam - CCS][t Defjam].adf boot-block load

- `tiny_sub`: Small routine of 1 instruction that subtracts d2 from d1.
- `store_four_word_entry`: Stores the word in d0 and the word in d1 into consecutive 16-bit slots of a buffer pointed to by a2, interleaving the fixed constants -2 and 384, then returns the first input word increased by 256.
- `store_interleaved_words`: Stores two fixed words and two register values into consecutive 16-bit slots of a buffer pointed to by a2, then returns a register incremented by 512.
- `call_library_pair_twice`: Calls a library function at offset -240 from the base in a6, then a library function at offset -60, then repeats both calls after adding 384 to the first input word, and finally advances a pointer by the value of the fourth input register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_from_stack`: Reads a word from the address in a0, restores registers d1 and a0 from the stack, and returns the combined 32-bit value formed by that word and the leftover stack word.
- `decrement_timer_counters`: Checks a global flag and calls a routine when it is set, then, based on the value of a 16-bit counter at a global variable, decrements a byte at offset 5 of a structure and updates a set of ASCII digits at nearby offsets, rolling over each field when it reaches its limit.
- `advance_audio_frame_counter`: Increments a 16-bit counter at a global variable and resets it to zero along with a second global when it reaches 6, otherwise it checks four byte flags and calls a routine for each one that is nonzero.
- `dispatch_volume_command`: Dispatches a command byte at offset 2 of the structure in a6: increments or decrements a 16-bit counter at offset 0 of the structure in a4 by the low nibble of the byte at offset 3, clamping it between 113 and 856, or stores the byte at offset 8 of the structure in a5, otherwise it returns.
- `toggle_cia_bit_and_store`: Toggles bit 1 of the parallel register at the fixed CIA address, reads the low four bits of the byte at offset 3 of the structure in a6, stores that value as a 16-bit word at the address in a0, and returns.
- `setup_audio_and_dispatch`: Compares a register against several table addresses and returns early when they match, otherwise sets up audio registers, calls a handler four times, and manages counters before returning.
- `process_and_store_entry`: Copies a four-word entry from a table into a structure, updates several fields based on a selector, calls a routine, and stores results into another structure.
- `tiny_ori_andi`: Small routine of 2 instructions that merges 0 into d0, then masks (a0) with 30.
- `fill_table_with_offsets`: Writes a base address and five derived addresses into consecutive slots of a table, then returns immediately when a computed value exceeds a threshold.
- `set_color_infinite_loop`: Writes the fixed constant 61699 to the color register at the fixed address and then branches back to itself, forming an infinite loop.
- `fill_copper_tables_init`: Fills two tables with four derived addresses each, then initializes a structure by clearing fields and writing fixed words based on a register value.
- `init_copper_structure`: Initializes a structure by clearing several fields, writing fixed words, pointing a copper list register at a table, and returning the copper read value.
- `update_copper_and_call`: Clears an interrupt mask register, updates a structure based on a flag, writes a copper list register, decrements a counter, and calls a stored routine before returning.
- `tiny_move_rte`: Small routine of 2 instructions that copies (a7)+ to d0, then returns from the exception.
- `write_masked_value_twice`: Masks the word in d0 to 12 bits, shifts the word in d1 left by one, and writes the masked value to both the color register at the fixed address and a global variable selected by the shifted index, then restores registers and returns.
- `set_bits_or_operands`: Performs four bitwise OR operations, setting fixed bit patterns into the words d1, d4, and d5 in sequence, and falls through to the next routine.
- `set_bits_or_operands_2`: Performs four bitwise OR operations, setting the fixed bit pattern 154 into the words d6 and d7 in sequence, and falls through to the next routine.
- `tiny_ori`: Small routine of 1 instruction that merges 1 into d0.
- `combine_input_bits`: Reads the blitter register and the joystick movement register, combines selected bits from them along with a bit from the parallel register into a single value, stores that value as a 16-bit word at the address in a0, and returns.
- `read_cia_parallel_mask`: Reads a byte from the parallel register at the fixed CIA address and returns it with bit 7 masked off.
- `swap_and_clear_flag`: Reads a 16-bit value from a global variable, clears that variable, and returns it with bit 7 cleared if it was set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_until_at_character`: Reads a byte from a global variable and calls a routine while that byte equals the character Y, then returns when it equals the character @.
