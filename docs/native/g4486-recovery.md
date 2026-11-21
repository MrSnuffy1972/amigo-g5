# Game 4486 (mysterious worlds (1990)(playstone design)[cr fica][t +4 fica]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04486). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901232 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 7 of 7 named (0 author symbols), 7 of 7 described (0 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 93 of 93 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 295 of 295 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 232 |
| code | 510 |
| data | 5610 |
| padding | 928 |
| unused | 893952 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Mysterious Worlds (1990)(Playstone Design)[cr FICA][t +4 FICA].adf:BOOT | 2 | 87 | 3 |
| Mysterious Worlds (1990)(Playstone Design)[cr FICA][t +4 FICA].adf:bootblock | 1 | 29 | 1 |
| logged disk load from Mysterious Worlds (1990)(Playstone Design)[cr FICA][t +4 FICA].adf boot-block load | 1 | 0 | 0 |
| logged disk load from Mysterious Worlds (1990)(Playstone Design)[cr FICA][t +4 FICA].adf boot-block load | 1 | 0 | 0 |
| logged disk load from Mysterious Worlds (1990)(Playstone Design)[cr FICA][t +4 FICA].adf boot-block load | 1 | 155 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |

## Cases handled

- mysterious worlds (1990)(playstone design)[cr fica][t +4 fica]/Mysterious Worlds (1990)(Playstone Design)[cr FICA][t +4 FICA].adf: the boot block reads 33792 bytes from disk offset 867328 to an address (found statically); recovered as a unit
- mysterious worlds (1990)(playstone design)[cr fica][t +4 fica]/Mysterious Worlds (1990)(Playstone Design)[cr FICA][t +4 FICA].adf: the boot block reads 11776 bytes from disk offset 461824 to an address (found statically); recovered as a unit
- mysterious worlds (1990)(playstone design)[cr fica][t +4 fica]/Mysterious Worlds (1990)(Playstone Design)[cr FICA][t +4 FICA].adf: the boot block reads 36352 bytes from disk offset 484352 to an address (found statically); recovered as a unit
- runtime segment an address: 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- mysterious worlds (1990)(playstone design)[cr fica][t +4 fica]/Mysterious Worlds (1990)(Playstone Design)[cr FICA][t +4 FICA].adf:BOOT: ByteKiller 1.3 (xfd) stream found by ancient scan (1240 -> 1246 bytes) but only 1 of 1 sampled windows are in reference RAM: not used
- mysterious worlds (1990)(playstone design)[cr fica][t +4 fica]/Mysterious Worlds (1990)(Playstone Design)[cr FICA][t +4 FICA].adf: 3 logged loads cover 0 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Mysterious Worlds (1990)(Playstone Design)[cr FICA][t +4 FICA].adf:BOOT

- `unpack_bitstream_to_memory`: Reads a compressed bitstream using a bit reader to reconstruct data into a destination buffer starting at a specific memory address.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Mysterious Worlds (1990)(Playstone Design)[cr FICA][t +4 FICA].adf:bootblock

- `initialize_dma_channels`: Configures multiple DMA channels by setting their control registers and memory addresses through a series of moves and function calls, then performs a final jump to another routine.

### logged disk load from Mysterious Worlds (1990)(Playstone Design)[cr FICA][t +4 FICA].adf boot-block load

- `shift_and_store_byte`: Loads a long word from the stack, extracts its byte value, stores it as a word in a global variable, sets the condition code register to 16, and performs a right rotate operation on the original value before returning.
- `process_bit_data`: Processes bit-level data by shifting a value and using a subroutine to handle carry flags, then performs conditional operations based on the results before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_copper_list`: Sets up a copper list in memory with specific control words and initializes hardware registers, then waits for input before proceeding.
