# Game 984 (buggy boy (1988)(elite)[cr bs1 - tkt - ina][f defjam]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00984). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 11 of 11 named (0 author symbols), 11 of 11 described (1 by rule, 8 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 42 of 42 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 216 of 216 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 232 |
| code | 720 |
| data | 185355 |
| padding | 1085 |
| unused | 713728 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Buggy Boy (1988)(Elite)[cr BS1 - TKT - INA][f Defjam].adf:load | 1 | 206 | 10 |
| Buggy Boy (1988)(Elite)[cr BS1 - TKT - INA][f Defjam].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- buggy boy (1988)(elite)[cr bs1 - tkt - ina][f defjam]/Buggy Boy (1988)(Elite)[cr BS1 - TKT - INA][f Defjam].adf:load: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Buggy Boy (1988)(Elite)[cr BS1 - TKT - INA][f Defjam].adf:load

- `run_program_entry`: Calls the system setup routine, disables interrupts and DMA channels, calls the graphics library, copies a word to a palette entry, checks for a 20562 marker and branches to a decompression routine before jumping to the entry point.
- `reconstruct_bitstream_from_packed_data`: Writes the current value of the pointer register to the color palette register at offset 608, then repeatedly reads a packed 32-bit value, extracts its bits through repeated right-shifting and bitwise operations, sets the most significant bit of each new byte, and writes the reconstructed bytes back to memory.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_right_bits_with_early_exit`: The routine shifts a 32-bit value right by one bit repeatedly, loading a new 32-bit word from a table pointed to by a pointer each time the shifted value becomes zero, and stops early if a counter reaches -1.
- `init_system_memory`: Opens several libraries, copies a block of words from a table into memory at a fixed address until a -2 marker is reached, then zeroes a region from a fixed address to a fixed address and calls a series of setup routines.
- `disable_interrupts_dma`: Calls a setup routine, disables interrupts via the INTENA register, disables DMA channels via the DMACON register and jumps to the code at a fixed address.
- `copy_source_byte`: Loads two addresses into pointers, compares a byte at the first pointer with 178 and branches to the shared routine if they match, otherwise copies a byte from the first pointer to the second and jumps to the shared routine.
- `advance_source_pointer`: Advances the source pointer by one, compares the byte it now references with one and branches away if they differ, otherwise returns.
- `store_marker_byte`: Compares a byte at the source pointer with zero and branches away if it is not zero, otherwise stores the value 178 at the destination pointer, advances the source pointer and jumps to the shared routine.
- `write_palette_byte`: Writes a byte value into the COLOR00 palette register, then copies a byte from the source pointer to the destination pointer, decrements a counter and loops while it is nonzero before advancing the source pointer.

### Buggy Boy (1988)(Elite)[cr BS1 - TKT - INA][f Defjam].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
