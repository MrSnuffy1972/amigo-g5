# Game 514 (Jungle Strike): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00514). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2708672 top-level bytes in 105 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 43 of 43 named (3 author symbols), 43 of 43 described (3 by rule, 34 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 928 of 928 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1464 of 1479 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 389848 |
| code | 5418 |
| data | 1933759 |
| padding | 26367 |
| unused | 353280 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:JStrike | 1 | 222 | 20 |
| Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:TRDINTRO | 5 | 205 | 7 |
| Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1032 | 14 |

## Cases handled

- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:status: high-entropy data file (7.62 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:copilots: high-entropy data file (7.69 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:JSC: high-entropy data file (7.73 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:JSF: high-entropy data file (7.73 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:intro: high-entropy data file (7.68 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:menubgd: high-entropy data file (7.68 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:music: high-entropy data file (7.82 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:heli_sprite: high-entropy data file (7.66 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:sfx: high-entropy data file (7.81 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:menubgd2: high-entropy data file (7.68 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:brieffilm: high-entropy data file (7.71 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:slides: high-entropy data file (7.75 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:world4blks: high-entropy data file (7.72 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:scientist: high-entropy data file (7.66 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:world2blks: high-entropy data file (7.64 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:mission4stat: high-entropy data file (7.73 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:mission2stat: high-entropy data file (7.73 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:brief: high-entropy data file (7.69 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:newsfilm: high-entropy data file (7.71 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:deathfilm: high-entropy data file (7.70 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:madtalk: high-entropy data file (7.60 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:world3blks: high-entropy data file (7.80 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:objects3: high-entropy data file (7.60 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:objects4: high-entropy data file (7.61 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:mission3stat: high-entropy data file (7.72 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:music: high-entropy data file (7.82 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:winfilm: high-entropy data file (7.68 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:scorefilm: high-entropy data file (7.63 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:mission1stat: high-entropy data file (7.70 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:sprites1: high-entropy data file (7.68 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:sprites2: high-entropy data file (7.61 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:sprites3: high-entropy data file (7.61 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:sprites4: high-entropy data file (7.63 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:awac: high-entropy data file (7.73 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:brieffilm: high-entropy data file (7.71 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 2 of 3).adf:slides: high-entropy data file (7.75 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:mission6stat: high-entropy data file (7.75 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:russian: high-entropy data file (7.66 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:brief: high-entropy data file (7.69 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:world7blks: high-entropy data file (7.76 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:deathfilm: high-entropy data file (7.70 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:mission9stat: high-entropy data file (7.65 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:world5blks: high-entropy data file (7.62 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:mission7stat: high-entropy data file (7.64 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:mission5stat: high-entropy data file (7.64 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:objects5: high-entropy data file (7.63 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:music: high-entropy data file (7.82 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:world8blks: high-entropy data file (7.70 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:sprites5: high-entropy data file (7.62 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:awac: high-entropy data file (7.73 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:sprites6: high-entropy data file (7.63 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:sprites8: high-entropy data file (7.63 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:sprites9: high-entropy data file (7.66 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:world6blks: high-entropy data file (7.77 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 3 of 3).adf:mission8stat: high-entropy data file (7.71 bits/byte), classified as packed data
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 712 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- jungle strike (1994)(ocean)[cr trader's dream](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:TRDINTRO: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:TRDINTRO

## Copy protection found

- none found

## Functions


### Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:JStrike

- `BEGINING`: A three-instruction entry point routine that calls another routine.
- `SUPER`: A nine-instruction routine that modifies DMA and interrupt enable registers before calling another routine.
- `PROGGY`: Checks memory boundaries, configures DMA and hardware registers, and executes a sequence of routines with varying parameters.
- `clear_memory_block`: Fills a specific range of memory starting at a high address with zeros.
- `initialize_dma_and_memory`: Configures hardware registers for DMA and video synchronization, then clears a large block of memory.
- `execute_complex_operations`: Performs indirect function calls, expands memory data, and adjusts a color table.
- `enter_infinite_loop`: Performs an infinite loop that prevents further execution.
- `execute_indirect_call_2`: Performs a series of function calls, modifies a global variable, and then executes a routine via a pointer stored in that global variable.
- `execute_indirect_call`: Calls another routine using a pointer stored in a global variable.
- `expand_memory_data`: Copies data from a source to a destination while expanding the data based on values found within the source stream.
- `copy_memory_block`: Copies a block of memory from a source to a destination, using a length byte to determine the number of bytes to transfer.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_color_table`: Iterates through a table and adjusts the lower three bitfields of each entry to match a target value over several passes.
- `adjust_color_nibbles`: Modifies the middle and high nibbles of values in a table to match a target pattern through iterative bit manipulation.
- `modify_color_bits`: Adjusts the high-order bits of values in a table to align with a target bitmask.
- `update_color_table`: Iterates through a table of values and adjusts specific bitfields within each entry to match a target color pattern.
- `decrement_color_values`: Decrements specific bitfield components of values within a table based on a comparison with another set of data.
- `decrement_color_table`: Iterates through a nested loop to subtract bitfield values from a table of entries over multiple passes.
- `subtract_color_components`: Subtracts specific bitfield values from a list of entries in a table until a condition is met.
- `wait_for_interrupt`: Polls a bit in a hardware register and writes a value to a memory location once the bit is set.

### Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:TRDINTRO

- `initialize_program_environment`: Performs a sequence of subroutine calls and frees memory using a system library.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `initialize_decompression_tables`: Initializes and populates several data tables by iteratively calculating values and performing bitwise operations.
- `shift_bit_stream`: Shifts bits from a register and updates a value using a bitmask or a value from a table.
- `lookup_table_masked_value`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `fill_two_tables`: The routine calls another routine to obtain a count, then fills a short table with that many values returned by the same routine, accumulating the values.
- `initialize_table_values`: Iteratively calculates and stores new values into a table using a pointer.

### Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_lookup_tables`: Calculates and populates a two-dimensional lookup table using a multiplication-based scaling algorithm, clears a range of memory, and initializes several global or hardware-related flags and pointers.
- `split_long_to_word_offsets`: Swaps the high and low words of a 32-bit value in a register and stores the new high word at offset 2 and the new low word at offset 6 from a pointer in a0, then increments the pointer by 8 and returns.
- `update_blitter_transform_matrix`: Calculates a new transformation matrix using a series of complex, bit-shifted multiplications and additions involving values from a table, then updates certain hardware registers and a global counter or flag.
- `parse_command_stream`: Iterates through a stream of command bytes in a buffer, performing actions such as clearing memory blocks, copying memory between tables, or updating values based on specific command opcodes.
- `initialize_audio_and_clear_tables`: Initializes a table of integers with incrementing values, finds the maximum byte value in a memory range, and clears a series of structures in a table based on that maximum value.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `manage_recursive_counter_and_table_sync`: Increments a local counter and recursively calls itself until a threshold is reached, at which point it resets several global variables and hardware registers.
- `update_dma_config`: Writes specific values to a global variable and modifies DMA or interrupt control registers.
- `set_audio_registers`: Stores a series of specific values into several custom audio registers and sets a global variable.
- `update_entity_state_from_table`: Loads an entity structure from a pointer, extracts an index from its bytes, and fetches data from a table to update the structure's fields.
- `update_parameterized_accumulator_and`: Updates a 16-bit field in a structure by adding a value and clamps it to a limit, then modifies various fields in a structure and potentially updates a DMA controller register or a global variable based on a command byte.
- `update_accumulator_value`: Adds or subtracts a value from a 16-bit field in a structure, clamps it to a limit, and calls another routine if a specific byte is non-zero.
- `apply_scaled_offset`: Calculates a scaled value using a table lookup and a bitmask, then adds or subtracts it from a memory location while updating a counter.
- `process_parameterized_accumulator_updates`: Reads a command byte from a structure to determine how to modify various 16-bit and 8-bit fields, including clamping values to specific limits, performing table-based scaling, and updating a 16-bit field at offset 6 of a second structure.
