# Game 864 (alien bash (1993)(s. law - g. cumming)(pd)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00864). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 9 of 9 named (0 author symbols), 9 of 9 described (3 by rule, 6 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 216 of 216 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 245 of 245 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 95492 |
| code | 636 |
| data | 32352 |
| padding | 1056 |
| unused | 771584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Alien Bash (1993)(S. Law - G. Cumming)(PD).adf:AlienBash | 3 | 235 | 8 |
| Alien Bash (1993)(S. Law - G. Cumming)(PD).adf:bootblock | 1 | 10 | 1 |

## Cases handled

- alien bash (1993)(s. law - g. cumming)(pd)/Alien Bash (1993)(S. Law - G. Cumming)(PD).adf:AlienBash: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Alien Bash (1993)(S. Law - G. Cumming)(PD).adf:AlienBash

- `process_node_data`: Allocates memory via a system call and, if the pointer is valid, walks a linked list of node markers looking for one whose header contains the magic value a fixed address, then copies and rearranges the node's data blocks into a destination buffer.
- `mask_and_conditional_call`: Masks a word register with another register and subtracts a byte register; if the subtraction is not negative it calls another routine and uses its returned word as a shift amount, otherwise it returns the masked word.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `fill_figure_table`: Iterates over a stack buffer comparing values, and when a match occurs writes bit masks and derived byte values into a table at fixed offsets, looping until a counter reaches a threshold before restoring registers and returning.
- `scan_table_extract_bit`: Reads word pairs from a table, advancing two words each time, until a masked word equals the following word, then reads a byte at offset 60 and subtracts it from a register; if the result is not negative it calls another routine and uses its returned word as a shift amount, otherwise it continues.
- `fill_figure_table_2`: Reads a count from a stack buffer and stores that many bytes into a table, then iterates over a key list comparing each value and, on a match, writes a bit mask and derived byte values into the table at offsets 0, 32, and 33, before returning.
- `free_node_list`: Checks whether a pointer holds a specific marker and, if so, clears a linked list of nodes by walking through them, then adds a computed value to a structure field and frees memory through the operating system library.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Alien Bash (1993)(S. Law - G. Cumming)(PD).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
