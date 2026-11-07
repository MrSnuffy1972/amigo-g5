# Game 514 (Jungle Strike): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00514). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2703360 top-level bytes in 103 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 28 of 28 named (3 author symbols), 28 of 28 described (2 by rule, 21 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 177 of 177 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 445 of 445 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 389848 |
| code | 1434 |
| data | 1932431 |
| padding | 26367 |
| unused | 353280 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:JStrike | 1 | 222 | 20 |
| Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:TRDINTRO | 5 | 205 | 7 |
| Jungle Strike (1994)(Ocean)[cr Trader's Dream](Disk 1 of 3).adf:bootblock | 1 | 18 | 1 |

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
