# Game 918 (turbo outrun (1989)(u. s. gold)(disk 1 of 2)[cr qtx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00918). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 14 of 14 named (0 author symbols), 14 of 14 described (1 by rule, 10 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 22 of 22 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 363 of 363 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 1062 |
| data | 155663 |
| padding | 1483 |
| unused | 742912 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| wait | 2 | 117 | 2 |
| Turbo Outrun (1989)(U. S. Gold)(Disk 1 of 2)[cr QTX].adf:turbo-run | 2 | 119 | 9 |
| Turbo Outrun (1989)(U. S. Gold)(Disk 1 of 2)[cr QTX].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- turbo outrun (1989)(u. s. gold)(disk 1 of 2)[cr qtx]/Turbo Outrun (1989)(U. S. Gold)(Disk 1 of 2)[cr QTX].adf:turbo-run: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_d1_to_d0_then_branch`: Copies the 16-bit value from register d1 into register d0, then branches to the start of another routine.

### wait

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value in a field, then writes a small header containing that value and a sequence of incrementing words into a buffer.
- `move_d1_to_d0`: Copies the 32-bit value from register d1 into register d0 and then branches to the start of another routine.

### Turbo Outrun (1989)(U. S. Gold)(Disk 1 of 2)[cr QTX].adf:turbo-run

- `init_and_jump`: Loads the global pointer into a6 and calls the exec library, then disables interrupts and clears DMA and interrupt-enable registers.
- `copy_words_loop_counter`: Copies a 32-bit word from the source pointer in a0 to the destination pointer in a1 on each pass, decrements the counter in d0, and loops while the counter is not zero, otherwise jumps to another routine.
- `process_and_store_bytes`: Builds a byte sequence in d0 through a series of oric instructions, loads a table address into a0 and a base address into a1, then reads longwords from the table, XORs them together and shifts the result right.
- `setup_and_branch`: Loads the value 8 into registers d1 and d4, then branches unconditionally to the middle of another routine.
- `decode_and_dispatch`: Loads the value 2 into d1 and calls the bit reader, then compares d2 with 2 and branches when it is smaller.
- `fill_and_write_color`: Computes a loop count from d2, calls the decrementing-global routine, then copies bytes from a source using d2 as an offset while dbra loops.
- `write_decrementing_global_until_wrap`: Stores a 16-bit countdown into a global variable on each pass, looping until the counter wraps back to its starting value, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Turbo Outrun (1989)(U. S. Gold)(Disk 1 of 2)[cr QTX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
