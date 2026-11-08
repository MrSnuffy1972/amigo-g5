# Game 1310 (interphase (1990)(imageworks)[cr defjam - ccs]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01310). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 7 of 7 named (0 author symbols), 7 of 7 described (1 by rule, 6 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 144 of 144 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 188 of 188 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 628 |
| code | 520 |
| data | 239987 |
| padding | 1041 |
| unused | 658944 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Interphase (1990)(ImageWorks)[cr Defjam - CCS].adf:inter | 1 | 178 | 6 |
| Interphase (1990)(ImageWorks)[cr Defjam - CCS].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- interphase (1990)(imageworks)[cr defjam - ccs]/Interphase (1990)(ImageWorks)[cr Defjam - CCS].adf:inter: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Interphase (1990)(ImageWorks)[cr Defjam - CCS].adf:inter

- `init_custom_registers`: Loads the custom register base into a6, clears the interrupt request and enable registers along with four palette registers at offsets 150, 154, 156 and 158, then stores the address of the following code into the vector at offset 32.
- `copy_rle_and_jump`: Copies 645 bytes from a source table into the screen buffer at address 1024, then sets the stack pointer and jumps to a routine at offset 64 of the copied data.
- `expand_rle_run`: Reads bytes from the source pointer in a1 and copies them to the screen buffer at address 1024, expanding a repeat marker that stores a byte value followed by a count into that many copies.
- `expand_rle_run_2`: Copies a run of bytes from the source pointer in a1 to the screen buffer at address 1024 until a repeat marker is reached, then decodes a Huffman-coded stream by calling a decoder routine and expanding each run into the screen buffer using a lookup table.
- `pop_and_rotate`: Pops a word off the stack into d0, saves its low byte into d7 and stores d7 into the register pointed to by a5, then rotates d0 right and returns.
- `shift_and_call_loop`: Loops a number of times given by a1 decremented by one, shifting a0 right until it becomes zero and calling a helper routine whenever a zero bit is found.

### Interphase (1990)(ImageWorks)[cr Defjam - CCS].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
