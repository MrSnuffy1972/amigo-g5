# Game 18 (Baal): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00018). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

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
| Baal.adf:bootblock | 1 | 17 | 1 |
| logged disk load from Baal.adf boot-block load | 1 | 124 | 4 |
| logged disk load from Baal.adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 521 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |

## Cases handled

- Baal/Baal.adf: the boot block reads 1024 bytes from disk offset 2048 to an address (found statically); recovered as a unit
- Baal/Baal.adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 164 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 10 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Baal/Baal.adf: 2 logged loads cover 0 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Baal.adf:bootblock

- `init_structure_and_jump`: Initializes a structure pointed to by a1 with several fixed constants, calls two library functions, then reinitializes the same structure with different constants and jumps to a fixed address.

### logged disk load from Baal.adf boot-block load

- `tiny_sub`: Small routine of 1 instruction that subtracts d2 from d1.
- `store_four_word_entry`: Stores two input words from registers d0 and d1 into consecutive 16-bit slots of a buffer pointed to by a2, interleaving two fixed 16-bit constants, then returns the first input word increased by 256.
- `store_interleaved_words`: Stores two fixed words and two register values into consecutive 16-bit slots of a buffer pointed to by a2, then returns a register incremented by 512.
- `call_library_pair_twice`: Calls two library functions at fixed offsets from a base, alternating between them twice while passing the same register values to each call, then adds a constant to a register and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_and_combine_words`: Reads a 16-bit word from the address in a0, restores registers d1 and a0 from the stack, and returns the combined 32-bit value of that word and the leftover stack word.
- `decrement_timer_counters`: Checks a global flag and then, based on the value of a 16-bit counter at a global variable, decrements a byte at offset 5 of a structure and updates a set of ASCII digits at nearby offsets, rolling over each field when it reaches its limit.
- `cycle_audio_channels`: Increments a 16-bit counter at a global variable and resets it to 0 when it reaches 6.
- `process_cia_command`: Reads a 4-bit command from offset 2 of the structure in a6 and a nibble from offset 3, then adjusts a 16-bit value at offset 0 of a4 by adding or subtracting it with clamping, copies a byte to offset 8, or selects a table entry by a counter at offset 16 of a6.
- `toggle_cia_bit_and_store`: Toggles bit 1 of the CIA parallel register at address a fixed address, then reads a 4-bit value from a byte at offset 3 of the structure in a6, stores it as a 16-bit word at the address in a0, and returns.
- `setup_audio_and_dispatch`: Compares a register against several table addresses and returns early when they match, otherwise sets up audio registers, calls a handler four times, and manages counters before returning.
- `process_and_store_entry`: Copies a four-word entry from a table into a structure, updates several fields based on a selector, calls a routine, and stores results into another structure.
- `tiny_ori_andi`: Small routine of 2 instructions that merges 0 into d0, then masks (a0) with 30.
- `fill_address_table_loop`: Writes a base address and five derived addresses into a table, computes a scaled value, and enters an infinite loop when the running address exceeds 32768.
- `set_color_and_loop`: Writes a fixed 12-bit value to a color register, then enters an infinite loop that never returns.
- `fill_tables_and_init`: Fills two tables with four derived addresses using a decrementing loop, then writes a register and initializes a structure, falling through to another routine.
- `init_copper_structure`: Initializes a structure by clearing several fields, writing fixed words, pointing a copper list register at a table, and returning the copper read value.
- `update_copper_and_call`: Clears an interrupt mask register, updates a structure based on a flag, writes a copper list register, decrements a counter, and calls a stored routine before returning.
- `tiny_move_rte`: Small routine of 2 instructions that copies (a7)+ to d0, then returns from the exception.
- `write_masked_value_twice`: Masks a 12-bit value and writes it to two locations selected by a shifted index, then restores registers and returns.
- `apply_bitwise_ors`: Applies four bitwise OR operations to registers d1, d4, and d5 using fixed byte constants, then falls through to the next routine.
- `apply_bitwise_ors_pair`: Applies the same bitwise OR operation with a fixed byte constant to registers d6 and d7, then falls through to the next routine.
- `tiny_ori`: Small routine of 1 instruction that merges 1 into d0.
- `combine_input_bits`: Combines bits from a hardware register, a joystick movement register, and a parallel port register into a single value, stores it to a table, and returns.
- `read_cia_fire_bit`: Reads a byte from a global variable, masks bit 7 of it, and returns the masked value in d0.
- `swap_and_clear_flag`: Reads a 16-bit value from a global variable, clears that variable, and returns it with bit 7 cleared if it was set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_and_dispatch_loop`: Reads a byte from another routine, calls a handler while that byte equals the letter Y, and repeats until the byte equals the at sign.
