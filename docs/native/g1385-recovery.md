# Game 1385 (no second prize (1990)(thalion)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01385). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 25 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 9 of 9 named (0 author symbols), 9 of 9 described (0 by rule, 7 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1 of 1 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 194 of 194 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 6814 |
| code | 680 |
| data | 796346 |
| unused | 97280 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| No Second Prize (1990)(Thalion).adf:bootblock | 1 | 48 | 3 |
| logged disk load from No Second Prize (1990)(Thalion).adf boot-block load | 1 | 56 | 2 |
| logged disk load from No Second Prize (1990)(Thalion).adf boot-block load | 1 | 90 | 4 |

## Cases handled

- no second prize (1990)(thalion)/No Second Prize (1990)(Thalion).adf: custom-format (non-DOS) disk, 1566 non-empty blocks: matched against track-loaded segments
- no second prize (1990)(thalion)/No Second Prize (1990)(Thalion).adf: the boot block reads 2560 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- no second prize (1990)(thalion)/No Second Prize (1990)(Thalion).adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- no second prize (1990)(thalion)/No Second Prize (1990)(Thalion).adf: 2 logged loads cover 8192 bytes of the image
- no second prize (1990)(thalion)/No Second Prize (1990)(Thalion).adf: blocks 22-1564 (bytes 11264-801280) hold 24 packed streams (IMP: File Imploder, TPWM: Turbo Packer) found by scanning, 98% of the bytes
- no second prize (1990)(thalion)/No Second Prize (1990)(Thalion).adf: blocks 1749-1756 (bytes 895488-899584) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.51)
- no second prize (1990)(thalion)/No Second Prize (1990)(Thalion).adf: blocks 1758-1758 (bytes 900096-900608) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### No Second Prize (1990)(Thalion).adf:bootblock

- `init_ciab_and_start_io`: Initializes the CIAB parallel interface by loading control bytes and clearing the VBLANK enable bit, then checks the VBLANK flag and returns if it is not yet set.
- `check_supervisor_magic`: Compares the longword at a global variable against the fixed value an address and, when they do not match, returns; otherwise it stores the supervisor-mode magic number an address into that variable, checks the value in register d0, and returns.
- `install_supervisor_vector`: Compares a longword global variable against a fixed magic value and, when they match, overwrites that variable with the 68000 supervisor-mode vector a fixed address, then returns.

### logged disk load from No Second Prize (1990)(Thalion).adf boot-block load

- `setup_dma_and_jump`: Disables interrupts, then clears the DMA and interrupt-mask custom registers at offsets 384, 390 and 392 of the custom chip base.
- `read_capped_entry_field`: Returns the high word of one field when a flag bit is clear; when the flag bit is set it returns the field capped at a fixed limit, and returns zero when neither applies.

### logged disk load from No Second Prize (1990)(Thalion).adf boot-block load

- `decompress_packed_stream`: Reads a longword from the packed stream table and a longword from a fixed location at offset 1024, XORs them and folds the result into a checksum stored in a global variable, then shifts the value right by one bit.
- `return_without_action`: Shown body is only a return, so it does no work in the pseudo-code.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
