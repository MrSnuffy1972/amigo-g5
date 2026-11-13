# Game 420 (Sim City Cd32): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00420). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802240 top-level bytes in 14 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 10 of 10 named (0 author symbols), 10 of 10 described (2 by rule, 7 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 303 of 303 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3026 |
| code | 746 |
| data | 1368845 |
| padding | 5175 |
| unused | 424448 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| killaga | 1 | 16 | 1 |
| Sim City CD32 (1991)(Infogrames)(Disk 1 of 2)[HD].adf:Cls | 2 | 20 | 1 |
| Sim City CD32 (1991)(Infogrames)(Disk 1 of 2)[HD].adf:LhA | 2 | 267 | 8 |

## Cases handled

- sim city cd32 (1991)(infogrames)(disk 1 of 2)[hd]/Sim City CD32 (1991)(Infogrames)(Disk 1 of 2)[HD].adf:1.lha: high-entropy data file (8.00 bits/byte), classified as packed data
- sim city cd32 (1991)(infogrames)(disk 1 of 2)[hd]/Sim City CD32 (1991)(Infogrames)(Disk 1 of 2)[HD].adf:2.lha: high-entropy data file (8.00 bits/byte), classified as packed data
- sim city cd32 (1991)(infogrames)(disk 2 of 2)[hd]/Sim City CD32 (1991)(Infogrames)(Disk 2 of 2)[HD].adf:3.lha: high-entropy data file (7.95 bits/byte), classified as packed data
- sim city cd32 (1991)(infogrames)(disk 2 of 2)[hd]/Sim City CD32 (1991)(Infogrames)(Disk 2 of 2)[HD].adf:4.lha: high-entropy data file (7.99 bits/byte), classified as packed data
- sim city cd32 (1991)(infogrames)(disk 1 of 2)[hd]/Sim City CD32 (1991)(Infogrames)(Disk 1 of 2)[HD].adf:LhA: PowerPacker 4.0 (xfd) stream found by ancient scan (36576 -> 53592 bytes) but only 0 of 26 sampled windows are in reference RAM: not used
- sim city cd32 (1991)(infogrames)(disk 1 of 2)[hd]/Sim City CD32 (1991)(Infogrames)(Disk 1 of 2)[HD].adf:LhA: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### killaga

- `init_graphics_library`: Opens the graphics library via the library vector table, stores the returned base pointer in a global variable, and calls two further library functions through it.

### Sim City CD32 (1991)(Infogrames)(Disk 1 of 2)[HD].adf:Cls

- `open_dos_library`: Opens the dos library via the library vector table and stores the returned base pointer in a local variable.

### Sim City CD32 (1991)(Infogrames)(Disk 1 of 2)[HD].adf:LhA

- `relocate_hunk_data`: Walks through a buffer of AmigaDOS hunk type codes, copying the data of each executable or relocation hunk into a new block and rewriting the hunk's pointers, then returns.
- `dispatch_table_index`: Decrements a 16-bit counter in a register and branches away if it is negative, otherwise indexing into a table of pointers by doubling the offset and jumping to the selected entry.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_shifted_bytes`: Initializes several registers with constants and a stack pointer, then reads a byte from a buffer and returns early if it is zero.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_64bit_right`: Shifts a 64-bit value right by a variable amount using a loop that reads bytes from a source buffer and accumulates the result across registers, returning early when the shifted value becomes zero.
- `trap_with_shifted_byte`: Loads a word from a structure through a base pointer into a data register, sets bit 8 of another data register, invokes a system trap, and merges that register into a third data register.
