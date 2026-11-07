# Game 1952 (Kick Off 2 - Return to Europe (Europe) (Addon)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01952). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4862648 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 12 of 12 named (0 author symbols), 12 of 12 described (0 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 30 of 30 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 153 of 153 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2250570 |
| code | 714 |
| data | 2609780 |
| padding | 1584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Kick Off 2 - Return to Europe (Europe) (Addon).adf:bootblock | 1 | 46 | 3 |
| Kick Off 2 - Return to Europe (Europe) (Addon).ipf#adf:bootblock | 1 | 46 | 3 |
| logged disk load from Kick Off 2 - Return to Europe (Europe) (Addon).adf boot-block load | 1 | 0 | 0 |
| logged disk load from Kick Off 2 - Return to Europe (Europe) (Addon).ipf#adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 107 | 6 |

## Cases handled

- Kick Off 2 - Return to Europe (Europe) (Addon)/Kick Off 2 - Return to Europe (Europe) (Addon).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- Kick Off 2 - Return to Europe (Europe) (Addon)/Kick Off 2 - Return to Europe (Europe) (Addon).ipf decoded by disk-analyse to a 901120-byte disk image (tracks: T0.0-79.1: AmigaDOS)
- Kick Off 2 - Return to Europe (Europe) (Addon)/Kick Off 2 - Return to Europe (Europe) (Addon).ipf#adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- Kick Off 2 - Return to Europe (Europe) (Addon)/Kick Off 2 - Return to Europe (Europe) (Addon).adf: the boot block reads 120320 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- Kick Off 2 - Return to Europe (Europe) (Addon)/Kick Off 2 - Return to Europe (Europe) (Addon).ipf#adf: the boot block reads 120320 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- runtime segment an address: 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Kick Off 2 - Return to Europe (Europe) (Addon)/Kick Off 2 - Return to Europe (Europe) (Addon).adf: 1 logged loads cover 120320 bytes of the image
- Kick Off 2 - Return to Europe (Europe) (Addon)/Kick Off 2 - Return to Europe (Europe) (Addon).adf: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.45)
- Kick Off 2 - Return to Europe (Europe) (Addon)/Kick Off 2 - Return to Europe (Europe) (Addon).adf: blocks 246-1759 (bytes 125952-901120) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.98)
- Kick Off 2 - Return to Europe (Europe) (Addon)/Kick Off 2 - Return to Europe (Europe) (Addon).ipf#adf: 1 logged loads cover 120320 bytes of the image
- Kick Off 2 - Return to Europe (Europe) (Addon)/Kick Off 2 - Return to Europe (Europe) (Addon).ipf#adf: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.45)
- Kick Off 2 - Return to Europe (Europe) (Addon)/Kick Off 2 - Return to Europe (Europe) (Addon).ipf#adf: blocks 246-1759 (bytes 125952-901120) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.98)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Kick Off 2 - Return to Europe (Europe) (Addon).adf:bootblock

- `boot_alloc_absolute_and_io`: Boot block that calls the exec library to allocate memory at a fixed address and to run an I/O request through DoIO, and writes the colour, DMA and interrupt control registers.
- `copy_block_to_low_memory`: Copies longwords in a counted loop from a start label in the program to a fixed low memory address, then calls one other routine.
- `store_value_and_copy_longword`: Writes the value 431 to a global variable, decrements a RAM location by a value held in an unaffected register, copies a longword from an address passed in a register into a fixed memory location, and then stops execution.

### Kick Off 2 - Return to Europe (Europe) (Addon).ipf#adf:bootblock

- `allocate_memory_and_copy_block`: Reserves a block of memory at a fixed address through an executive allocation call, performs an executive input/output request, enables interrupts and DMA channels via their control registers, and then copies a run of longwords from a label in the program to low memory before calling another routine.
- `invoke_unnamed_routine`: An eleven-instruction routine reached by a taken address that invokes one unnamed routine and is called by the memory-allocation-and-input/output routine; its exact actions cannot be described because the decompiler failed.
- `enable_dma_channels`: A five-instruction routine that enables DMA channels through its control register and is reached only by a taken address; its full behaviour cannot be described because the decompiler failed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `pulse_bit_around_calls`: Loops indefinitely, setting bit 2 of a byte at one pointer, calling one routine, clearing the same bit at a second pointer, and calling that routine again.
- `poll_bit_with_timeout`: Clears bit 4 of a byte at a pointer and polls that byte until bit 4 is set, bounded by a counter, then waits for a word's top bit.
- `search_byte_pattern_in_stream`: Waits for the top bit of a word at a pointer to be set, then compares the following bytes against the character D and a second fixed byte value, with a counter that restarts the search.
- `wait_for_word_top_bit`: Two-instruction routine that polls a word through a pointer until its top bit is set, repeating the poll several times in a row.
- `compare_polled_byte_against_input`: Polls a word at a pointer and compares its low byte with an input byte register, returning on the first mismatch.
- `repeat_control_write_and_poll`: Repeats 120 times: writes three fixed byte values to three offsets from a base pointer, then polls the first offset until its lowest bit is clear.
