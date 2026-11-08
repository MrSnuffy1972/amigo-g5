# Game 842 (space hulk (1993)(electronic arts)(disk 1 of 3)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00842). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 7 of 7 named (0 author symbols), 7 of 7 described (2 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 206 of 206 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 261 of 261 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 620828 |
| code | 610 |
| data | 145231 |
| padding | 4915 |
| unused | 129536 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Space Hulk (1993)(Electronic Arts)(Disk 1 of 3).adf:Hulk | 6 | 251 | 6 |
| Space Hulk (1993)(Electronic Arts)(Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |

## Cases handled

- space hulk (1993)(electronic arts)(disk 1 of 3)/Space Hulk (1993)(Electronic Arts)(Disk 1 of 3).adf:Hulk: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Space Hulk (1993)(Electronic Arts)(Disk 1 of 3).adf:Hulk

- `swap_table_entry_value`: Saves the return address on the stack, runs several setup routines, then loops four times calling a routine and invoking a function pointer from a global structure.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `fill_buffer_from_table`: Fills a buffer with values from a table of byte counts, accumulating the copied byte counts and adding each source value into the destination, then returns.
- `load_table_and_constant`: Loads an address from a small table into a register and loads the constant two into another register, then returns.
- `copy_byte_block`: Copies a variable number of bytes from the source buffer in a0 to the destination buffer in a1, using a table of byte counts indexed by a value in d7, then restores the saved registers and returns.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Space Hulk (1993)(Electronic Arts)(Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
