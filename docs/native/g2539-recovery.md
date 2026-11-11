# Game 2539 (nitro (1990)(psygnosis)[cr hz][a4]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02539). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901392 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 6 of 6 named (0 author symbols), 6 of 6 described (0 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 51 of 51 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 153 of 153 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 20155 |
| code | 536 |
| data | 603195 |
| padding | 2 |
| unused | 277504 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Nitro (1990)(Psygnosis)[cr HZ][a4].adf:bootblock | 1 | 7 | 1 |
| logged disk load from Nitro (1990)(Psygnosis)[cr HZ][a4].adf boot-block load | 1 | 86 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 60 | 3 |

## Cases handled

- nitro (1990)(psygnosis)[cr hz][a4]/Nitro (1990)(Psygnosis)[cr HZ][a4].adf: custom-format (non-DOS) disk, 1216 non-empty blocks: matched against track-loaded segments
- nitro (1990)(psygnosis)[cr hz][a4]/Nitro (1990)(Psygnosis)[cr HZ][a4].adf: the boot block reads 20480 bytes from disk offset 606208 to an address (found statically); recovered as a unit
- runtime segment an address: 51 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- nitro (1990)(psygnosis)[cr hz][a4]/Nitro (1990)(Psygnosis)[cr HZ][a4].adf: 1 logged loads cover 20480 bytes of the image
- nitro (1990)(psygnosis)[cr hz][a4]/Nitro (1990)(Psygnosis)[cr HZ][a4].adf: blocks 1621-1621 (bytes 829952-830464) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.73)
- nitro (1990)(psygnosis)[cr hz][a4]/Nitro (1990)(Psygnosis)[cr HZ][a4].adf: blocks 1642-1642 (bytes 840704-841216) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.01)
- nitro (1990)(psygnosis)[cr hz][a4]/Nitro (1990)(Psygnosis)[cr HZ][a4].adf: blocks 1662-1662 (bytes 850944-851456) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.22)
- 1 packed track region(s), 600576 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Nitro (1990)(Psygnosis)[cr HZ][a4].adf:bootblock

- `call_library_function`: Executes a library call using a negative or or offset-based identifier.

### logged disk load from Nitro (1990)(Psygnosis)[cr HZ][a4].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_call_routine`: Decrements a global variable and then calls another routine before returning a concatenated result.
- `modify_global_bitmask`: Performs a series of bitwise checks and updates a global variable based on certain bit patterns.
- `write_to_memory_and_registers`: Writes a constant value to a memory location via a pointer and modifies certain hardware registers.
