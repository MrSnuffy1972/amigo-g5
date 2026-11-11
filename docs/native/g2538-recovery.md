# Game 2538 (nitro (1990)(psygnosis)[cr hz][a3]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02538). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

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
| data | 601659 |
| padding | 2 |
| unused | 279040 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Nitro (1990)(Psygnosis)[cr HZ][a3].adf:bootblock | 1 | 7 | 1 |
| logged disk load from Nitro (1990)(Psygnosis)[cr HZ][a3].adf boot-block load | 1 | 86 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 60 | 3 |

## Cases handled

- nitro (1990)(psygnosis)[cr hz][a3]/Nitro (1990)(Psygnosis)[cr HZ][a3].adf: custom-format (non-DOS) disk, 1213 non-empty blocks: matched against track-loaded segments
- nitro (1990)(psygnosis)[cr hz][a3]/Nitro (1990)(Psygnosis)[cr HZ][a3].adf: the boot block reads 20480 bytes from disk offset 606208 to an address (found statically); recovered as a unit
- runtime segment an address: 51 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- nitro (1990)(psygnosis)[cr hz][a3]/Nitro (1990)(Psygnosis)[cr HZ][a3].adf: 1 logged loads cover 20480 bytes of the image
- 1 packed track region(s), 600576 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Nitro (1990)(Psygnosis)[cr HZ][a3].adf:bootblock

- `process_input_state`: Performs a small sequence of bitwise operations on a global variable and stores the result back, with the whole block repeated twice.

### logged disk load from Nitro (1990)(Psygnosis)[cr HZ][a3].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_call`: Invokes another routine, decrements a global variable, stores a constant into it, calls a further routine, and returns the passed input registers.
- `set_timer_registers`: Writes a fixed value into a register slot, sets a global variable to a constant, then stores a value through a pointer.
- `transform_button_state`: Clears a global variable, applies a series of conditional bit manipulations, and returns the original input registers unchanged.
