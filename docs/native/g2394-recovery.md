# Game 2394 (super obliteration (1993)(d. papworth)[a3]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02394). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 9 of 9 named (0 author symbols), 9 of 9 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 10 of 10 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 319 of 319 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3842 |
| code | 848 |
| data | 210968 |
| padding | 1430 |
| unused | 684032 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Super Obliteration (1993)(D. Papworth)[a3].adf:SuperObliteration | 1 | 309 | 8 |
| Super Obliteration (1993)(D. Papworth)[a3].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- super obliteration (1993)(d. papworth)[a3]/Super Obliteration (1993)(D. Papworth)[a3].adf:SuperObliteration: Grants Computing (GC!3) (xfd) stream found by ancient scan (191332 -> 452576 bytes) but only 0 of 177 sampled windows are in reference RAM: not used

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Super Obliteration (1993)(D. Papworth)[a3].adf:SuperObliteration

- `initialize_memory_pool`: Allocates a memory pool of fixed size, initializes pointers for data structures, and sets up a table of allocated blocks.
- `call_if_equal`: Checks if a value in register a0 matches a constant from a table, and if so, calls a routine using a pointer from a5.
- `lookup_table_value`: Loops through a table using d7 as a counter, performs bitwise shifts on d6, calls another routine, and returns a word from a table in a2 using an index calculated from d1 and a value from a4.
- `calculate_and_store_value`: Clears a 16-bit counter in a global variable and sets bit 3 of the status byte at offset 34 of the structure in a6, then returns.
- `process_task_data`: Copies data from a source pointed to by a0 into a destination pointed to by a1, performs arithmetic to calculate a value stored at a7 offset 1600, and uses a loop to copy bytes from a source to a destination while performing bitwise operations and calls to other routines.
- `populate_buffer_with_data`: Iterates through a source array to calculate a sum and copy values, clears a subsequent memory range, and then populates a destination buffer.
- `apply_bitmask_to_rotated_value`: Calculates a value by performing arithmetic on d7 and d1, rotates d6, calls another routine using a pointer from a5, and returns a bitwise AND of the result with a mask derived from d1 and d0.
- `process_array_and_populate_buffer`: Iterates through a source array to calculate a sum and copy values, clears a subsequent memory range, and then populates a destination buffer with values derived from the source array and a counter.

### Super Obliteration (1993)(D. Papworth)[a3].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
