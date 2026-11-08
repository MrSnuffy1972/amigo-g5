# Game 1371 (super twintris (1994)(-)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01371). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 4 of 4 named (0 author symbols), 4 of 4 described (1 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 77 of 77 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 109 of 109 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 272 |
| data | 113213 |
| padding | 691 |
| unused | 786944 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Super Twintris (1994)(-)[a].adf:twin.exe | 1 | 99 | 3 |
| Super Twintris (1994)(-)[a].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- super twintris (1994)(-)[a]/Super Twintris (1994)(-)[a].adf:twin.exe: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Super Twintris (1994)(-)[a].adf:twin.exe

- `init_and_copy_table`: Initializes d7 to zero, loads a table address into a1, points a5 at the color register at a fixed address, and copies a block of bytes from that table into the memory pointed to by a4 until the destination reaches the end of the source.
- `pop_and_rotate_return`: Pops a long from the stack into d0, stores its low byte at offset 5 of the structure in a5, sets the carry flag via the condition-code register, then returns d0 rotated right through the carry flag with the high bit set.
- `shift_and_rotate_loop`: Repeatedly shifts a 32-bit value in d0 right by one bit and, while it is nonzero, decrements a counter in d1 and calls a routine that rotates a value in d2 left through the carry flag, decrementing the same counter until it reaches zero, then returns.

### Super Twintris (1994)(-)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
