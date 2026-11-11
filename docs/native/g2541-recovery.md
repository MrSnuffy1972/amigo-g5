# Game 2541 (nitro (1990)(psygnosis)[cr hz][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02541). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

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
| Nitro (1990)(Psygnosis)[cr HZ][a].adf:bootblock | 1 | 7 | 1 |
| logged disk load from Nitro (1990)(Psygnosis)[cr HZ][a].adf boot-block load | 1 | 86 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 60 | 3 |

## Cases handled

- nitro (1990)(psygnosis)[cr hz][a]/Nitro (1990)(Psygnosis)[cr HZ][a].adf: custom-format (non-DOS) disk, 1213 non-empty blocks: matched against track-loaded segments
- nitro (1990)(psygnosis)[cr hz][a]/Nitro (1990)(Psygnosis)[cr HZ][a].adf: the boot block reads 20480 bytes from disk offset 606208 to an address (found statically); recovered as a unit
- runtime segment an address: 51 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- nitro (1990)(psygnosis)[cr hz][a]/Nitro (1990)(Psygnosis)[cr HZ][a].adf: 1 logged loads cover 20480 bytes of the image
- 1 packed track region(s), 600576 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Nitro (1990)(Psygnosis)[cr HZ][a].adf:bootblock

- `call_library_at_boot`: Called from the Kickstart strap as the boot block entry point, this seven-instruction routine invokes a single Amiga OS library routine (identified by the negative library offset) and then stops.

### logged disk load from Nitro (1990)(Psygnosis)[cr HZ][a].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_handle_interrupt`: It returns two registers unchanged, decrements a global variable, stores a constant into it, calls another routine, and clears an interrupt request flag before returning.
- `process_input_bits`: It resets a global variable, then repeatedly tests and sets specific bits in that variable while computing an intermediate value from a shifted copy of it.
- `configure_hardware_registers`: It writes fixed values into a couple of hardware control registers and assigns a sequence of constants to a global variable before returning, without reading any of the registers it configures.
