# Game 1386 (no second prize (1990)(thalion)[cr ministry]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01386). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 25 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 10 of 10 named (0 author symbols), 10 of 10 described (0 by rule, 6 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1 of 1 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 257 of 257 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 23254 |
| code | 824 |
| data | 859634 |
| unused | 17408 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| No Second Prize (1990)(Thalion)[cr Ministry].adf:bootblock | 1 | 17 | 1 |
| logged disk load from No Second Prize (1990)(Thalion)[cr Ministry].adf boot-block load | 1 | 94 | 3 |
| logged disk load from No Second Prize (1990)(Thalion)[cr Ministry].adf boot-block load | 1 | 56 | 2 |
| logged disk load from No Second Prize (1990)(Thalion)[cr Ministry].adf boot-block load | 1 | 90 | 4 |

## Cases handled

- no second prize (1990)(thalion)[cr ministry]/No Second Prize (1990)(Thalion)[cr Ministry].adf: custom-format (non-DOS) disk, 1721 non-empty blocks: matched against track-loaded segments
- no second prize (1990)(thalion)[cr ministry]/No Second Prize (1990)(Thalion)[cr Ministry].adf: the boot block reads 16896 bytes from disk offset 805376 to an address (found statically); recovered as a unit
- no second prize (1990)(thalion)[cr ministry]/No Second Prize (1990)(Thalion)[cr Ministry].adf: the boot block reads 2560 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- no second prize (1990)(thalion)[cr ministry]/No Second Prize (1990)(Thalion)[cr Ministry].adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- no second prize (1990)(thalion)[cr ministry]/No Second Prize (1990)(Thalion)[cr Ministry].adf: 3 logged loads cover 25088 bytes of the image
- no second prize (1990)(thalion)[cr ministry]/No Second Prize (1990)(Thalion)[cr Ministry].adf: blocks 22-1564 (bytes 11264-801280) hold 24 packed streams (IMP: File Imploder, TPWM: Turbo Packer) found by scanning, 98% of the bytes
- no second prize (1990)(thalion)[cr ministry]/No Second Prize (1990)(Thalion)[cr Ministry].adf: blocks 1606-1693 (bytes 822272-867328) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.70)
- no second prize (1990)(thalion)[cr ministry]/No Second Prize (1990)(Thalion)[cr Ministry].adf: blocks 1705-1737 (bytes 872960-889856) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.70)
- no second prize (1990)(thalion)[cr ministry]/No Second Prize (1990)(Thalion)[cr Ministry].adf: blocks 1749-1759 (bytes 895488-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.70)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### No Second Prize (1990)(Thalion)[cr Ministry].adf:bootblock

- `initialize_structure_and_call_library`: Initializes a 48-byte structure in a1 with a 16-bit value and three 32-bit values, then calls the library routine at a6.

### logged disk load from No Second Prize (1990)(Thalion)[cr Ministry].adf boot-block load

- `write_decrementing_global_until_wrap`: Writes a 16-bit countdown into a global variable on each pass, looping until the counter wraps back to its starting value, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from No Second Prize (1990)(Thalion)[cr Ministry].adf boot-block load

- `initialize_system_and_jump`: Disables interrupts and clears the interrupt and DMA masks, then stores a set of initial values into a table of global variables and custom registers.
- `read_capped_entry_field`: Reads a flag bit from a field of a structure and returns the high word of another field when the bit is clear.

### logged disk load from No Second Prize (1990)(Thalion)[cr Ministry].adf boot-block load

- `decompress_huffman_block`: Decompresses a Huffman-coded block from the packed stream into a destination buffer: it reads bits from the depacker, walks the code tree to recover each symbol, then copies literal bytes or back-references (with lengths from a size table) into the output.
- `return_without_action`: Shown body is only a return, so it does no work in the pseudo-code.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
