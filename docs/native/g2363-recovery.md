# Game 2363 (jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02363). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906432 top-level bytes in 26 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 43 of 43 named (3 author symbols), 43 of 43 described (3 by rule, 15 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 928 of 928 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1464 of 1479 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 27970 |
| code | 5418 |
| data | 592789 |
| padding | 5823 |
| unused | 274432 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:JStrike | 1 | 222 | 20 |
| Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:TRDINTRO | 5 | 205 | 7 |
| Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1032 | 14 |

## Cases handled

- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:status: high-entropy data file (7.62 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:copilots: high-entropy data file (7.69 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:JSC: high-entropy data file (7.73 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:JSF: high-entropy data file (7.73 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:intro: high-entropy data file (7.68 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:menubgd: high-entropy data file (7.68 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:music: high-entropy data file (7.82 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:heli_sprite: high-entropy data file (7.66 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:sfx: high-entropy data file (7.81 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:menubgd2: high-entropy data file (7.68 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:brieffilm: high-entropy data file (7.71 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:slides: high-entropy data file (7.75 bits/byte), classified as packed data
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 712 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)[a]/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:TRDINTRO: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:TRDINTRO

## Copy protection found

- none found

## Functions


### Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:JStrike

- `BEGINING`: The decompiler timed out on this three-instruction program entry.
- `SUPER`: The decompiler timed out on this nine-instruction routine.
- `PROGGY`: Checks two memory-size values from the exec base and loops forever when the second is below a threshold.
- `setup_display_and_call_pointer`: Writes display setup values to the custom registers and a table, clears a block of longwords, calls other routines, then calls the routine whose address is held in a global cell.
- `configure_dma_and_interrupts`: Sets several values in a table, clears a block of memory, and then calls a routine via a pointer stored in a global variable.
- `finish_display_and_call_pointer`: Calls other routines, writes a colour and interrupt value, and calls the routine whose address is held in a global cell.
- `execute_infinite_loop`: Performs an infinite loop of no-operation instructions.
- `set_copper_and_call_pointer`: Calls other routines, points the copper at a fixed value, enables the interrupt flag, and calls the routine held in a global cell.
- `call_routine_then_pointer`: Calls a other routine, then calls the routine whose address is held in a global cell.
- `unpack_rle_until_terminator`: Unpacks a run-length stream from a source pointer into the destination.
- `copy_then_unpack_rle_stream`: Copies a counted block of bytes first, then decodes run-length records from the source.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `step_all_nibbles_toward_source`: For each non-negative word in a source list, it takes the next 32-bit table entry (skipping entries whose upper word is outside a fixed range) and moves its low, middle and high 4-bit fields one step toward the matching fields of the source word, storing the packed result back.
- `step_mid_field_pack_counter`: Walks a source word list and, for each entry, moves the middle 4-bit field of the stored value one step toward the source, combines it with a low-nibble counter and writes the packed word back.
- `step_high_field_pack_word`: For each non-negative source word, moves the high 4-bit field of the stored value one step toward the source, combines it with a middle-field counter and a low-field value, and writes the packed word into the table.
- `step_low_mid_pack_word`: For each non-negative source word, moves the low and middle 4-bit fields one step toward the source, combines them with a high-field counter, and writes the packed word back.
- `step_nibble_fields_list_pass`: Runs one pass over a source word list, moving all three 4-bit fields of each table value one step toward the source for every non-negative entry.
- `decrement_nibble_fields_sixteen_passes`: Sixteen times, walks a source word list and lowers each non-zero 4-bit field of the table value by one, writing back the packed word.
- `decrement_nibble_fields_in_list`: Walks a source word list once and reduces every non-zero 4-bit field of the next table value by one, storing the packed result.
- `wait_and_acknowledge_interrupt_bit4`: Polls the interrupt request register until its bit four is set, then writes that same bit to the interrupt clear register to acknowledge it, and returns the masked status value.

### Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:TRDINTRO

- `entry_free_memory_block`: Program entry stub that calls one routine, invokes a second routine through a stored pointer, calls the exec library FreeMem through a library offset, and clears a longword before returning.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `decode_compressed_stream_tables`: Clears a 16-entry table and reads a bit-stream through the shift and lookup routines to fill a code-length table.
- `shift_bit_stream`: Shifts bits from a register and updates a value using a bitmask or a value from a table.
- `lookup_table_masked_value`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `fill_two_tables`: The routine calls another routine to obtain a count, then fills a short table with that many values returned by the same routine, accumulating the values.
- `initialize_table_values`: Iteratively calculates and stores new values into a table using a pointer.

### Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_lookup_tables`: Calculates and populates a two-dimensional lookup table using a multiplication-based scaling algorithm.
- `split_long_to_word_pair`: splits a 32-bit register into two 16-bit halves and stores them into a destination array or structure at a fixed offset from a pointer.
- `update_blitter_transform_matrix`: Calculates a new transformation matrix or coordinate or rotation or scaling factor using a series of complex, bit-shifted multiplications and additions involving values from a table, then updates certain hardware registers and a global counter or flag.
- `parse_command_stream`: Iterates through a stream of command bytes, performing different actions such as clearing memory blocks, copying memory, or updating a value based on a command type.
- `initialize_audio_and_clear_tables`: Initializes a table of integers, finds the maximum value in a certain memory range, and clears a block of memory or a table or a structure.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `manage_recursive_counter_and_table_sync`: Increments a counter and recursively calls itself until a threshold is reached, at which point it resets the counter, clears a flag, and performs a series of writes to a table and hardware registers.
- `update_dma_config`: Writes specific values to a global variable and modifies DMA or interrupt control registers.
- `set_audio_registers`: Stores a series of specific values into several custom audio registers and sets a global variable.
- `update_entity_state_and_dma_2`: Updates an entity's properties or state flags by reading from a lookup table and applying conditional logic, including clamping certain byte values and updating a global variable or a remote memory location.
- `update_entity_state_and_dma`: Updates an entity's internal state or properties based on a type-specific flag, including conditional clamping of certain values and updating a global variable.
- `update_position_with_clamping`: Updates a memory location by adding or subtracting a value based on a flag, and clamps the result to a certain limit if a threshold is exceeded.
- `apply_scaled_offset`: Calculates a scaled value using a table lookup and a bitmask, then adds or subtracts it from a memory location while updating a counter.
- `update_entity_position_with_clamping`: Updates a memory location by adding or subtracting a value based on a flag and clamps the result to a certain limit if a threshold is exceeded.
