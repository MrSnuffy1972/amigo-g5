# Game 848 (Rock Star Ate My Hamster): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00848). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 7 of 7 named (0 author symbols), 7 of 7 described (1 by rule, 6 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 144 of 144 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 196 of 196 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 546 |
| data | 254331 |
| padding | 611 |
| unused | 645632 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rock Star Ate My Hamster.adf:RockStarAteMyHamster | 1 | 178 | 6 |
| Rock Star Ate My Hamster.adf:bootblock | 1 | 18 | 1 |

## Cases handled

- Rock Star Ate My Hamster/Rock Star Ate My Hamster.adf:RockStarAteMyHamster: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rock Star Ate My Hamster.adf:RockStarAteMyHamster

- `init_custom_and_set_vector`: Writes the value 32767 to four consecutive 16-bit registers at offsets 150, 152, 154 and 156 of the custom chip base, disables audio mixing with a control register write, and stores the address of the following routine into a vector at offset 32.
- `copy_routine_to_zeropage`: Copies 645 bytes from the routine at 114 into the zero-page region at a fixed address, pushes the destination address and a value onto the stack, then jumps into a jumptable entry at offset 64 of the copied code.
- `copy_compressed_bytes`: Copies a run of bytes from the source pointed to by a1 into the buffer at address 33024, repeating the destination byte when a length byte follows a 109 marker, and stops when the source pointer reaches the table at a fixed address.
- `decompress_bitstream`: Decompresses a Huffman-style bitstream from the source buffer into the destination buffer at 33024, copying literal bytes and expanding run or back-reference codes, and returns when the destination is full.
- `pop_and_rotate_value`: Pops a word off the stack, saves its low byte and stores it into a register pair, rotates the value right through the carry flag, then returns; otherwise it sets up registers and calls a helper routine before jumping to a later routine.
- `count_trailing_zero_bits`: Repeatedly shifts a value in d0 right and calls a helper routine whenever a shifted bit becomes zero, accumulating a count in d2, then returns.

### Rock Star Ate My Hamster.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
