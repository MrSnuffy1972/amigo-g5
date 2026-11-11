# Game 2536 (nitro (1990)(psygnosis)[cr hz]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02536). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

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
| data | 876091 |
| padding | 2 |
| unused | 4608 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Nitro (1990)(Psygnosis)[cr HZ].adf:bootblock | 1 | 7 | 1 |
| logged disk load from Nitro (1990)(Psygnosis)[cr HZ].adf boot-block load | 1 | 86 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 60 | 3 |

## Cases handled

- nitro (1990)(psygnosis)[cr hz]/Nitro (1990)(Psygnosis)[cr HZ].adf: custom-format (non-DOS) disk, 1749 non-empty blocks: matched against track-loaded segments
- nitro (1990)(psygnosis)[cr hz]/Nitro (1990)(Psygnosis)[cr HZ].adf: the boot block reads 20480 bytes from disk offset 606208 to an address (found statically); recovered as a unit
- runtime segment an address: 51 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- nitro (1990)(psygnosis)[cr hz]/Nitro (1990)(Psygnosis)[cr HZ].adf: 1 logged loads cover 20480 bytes of the image
- nitro (1990)(psygnosis)[cr hz]/Nitro (1990)(Psygnosis)[cr HZ].adf: blocks 1224-1759 (bytes 626688-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.00)
- 1 packed track region(s), 600576 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Nitro (1990)(Psygnosis)[cr HZ].adf:bootblock

- `invoke_library_function`: Calls a library function using a pointer or index passed in a register.

### logged disk load from Nitro (1990)(Psygnosis)[cr HZ].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_and_call`: Decrements a global variable, assigns it a new constant value, and calls another routine.
- `modify_global_bits`: Performs a series of bitwise checks and conditional bitwise or operations on a global variable.
- `write_to_memory_and_registers`: Writes a constant value to a memory location and updates certain hardware or interrupt registers.
