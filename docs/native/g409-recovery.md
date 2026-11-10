# Game 409 (Ruff 'n' Tumble): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00409). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802320 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 13 of 13 named (0 author symbols), 13 of 13 described (1 by rule, 12 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2 of 2 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 238 of 238 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4871 |
| code | 662 |
| data | 1796061 |
| padding | 214 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2).adf:bootblock | 1 | 31 | 2 |
| logged disk load from Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2).adf boot-block load | 1 | 205 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- ruff 'n' tumble (1994)(renegade)[cr pdx](disk 1 of 2)/Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1754 non-empty blocks: matched against track-loaded segments
- ruff 'n' tumble (1994)(renegade)[cr pdx](disk 2 of 2)/Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- ruff 'n' tumble (1994)(renegade)[cr pdx](disk 1 of 2)/Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2).adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ruff 'n' tumble (1994)(renegade)[cr pdx](disk 1 of 2)/Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2).adf: 1 logged loads cover 5632 bytes of the image
- ruff 'n' tumble (1994)(renegade)[cr pdx](disk 1 of 2)/Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2).adf:blocks 72-1759 (bytes 36864-901120)+144384#unpacked: high-entropy data file (7.83 bits/byte), classified as packed data
- ruff 'n' tumble (1994)(renegade)[cr pdx](disk 1 of 2)/Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2).adf:blocks 72-1759 (bytes 36864-901120)+465920#unpacked: high-entropy data file (7.82 bits/byte), classified as packed data
- ruff 'n' tumble (1994)(renegade)[cr pdx](disk 1 of 2)/Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2).adf: blocks 72-1759 (bytes 36864-901120) hold 5 packed streams (C: Compact, RNC1: Rob Northen RNC1 Compressor) found by scanning, 90% of the bytes
- ruff 'n' tumble (1994)(renegade)[cr pdx](disk 2 of 2)/Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 2 of 2).adf: blocks 2-1759 (bytes 1024-901120) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.19)
- 2 packed track region(s), 29696 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2).adf:bootblock

- `initialize_boot_sequence`: Saves a1, calls a routine, sets d1, calls a routine via a pointer in a6, and if the return value d0 is not zero, updates a structure in a1 with several values and calls another routine via a pointer in a6.
- `set_boot_parameters`: Writes certain values to offsets 36, 40, and 44 of a structure pointed to by a1, calls a routine via a pointer in a6, calls another routine, and returns a constant.

### logged disk load from Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2).adf boot-block load

- `copy_block_with_alignment_fixups`: Adjusts source and destination pointers to even addresses and then copies a block of longwords using a loop that calls another routine.
- `initialize_huffman_tables`: Loads a byte from a pointer, rotates it, and calls several routines to initialize a table at a certain offset, then decrements a counter and loops.
- `copy_and_process_data_blocks`: Uses a loop to copy bytes between two memory locations, potentially calling several routines, and updates d6 with a bit-manipulated value before a conditional jump.
- `match_bitmasked_pair_and_extract_bits`: Scans word pairs at a0 until the masked first word equals the second, then reads two byte fields and returns a bit value built from d6 shifted by the field counts, calling the refill routine when d7 runs low.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_d0_with_d6_and_refill_bits`: Masks d0 with d6, subtracts d1 from d7 and, if the bit budget went negative, calls the refill routine before shifting d6 right by d1 and returning the masked value.
- `refill_bit_buffer_from_a3_stream`: Adds d1 to d7, shifts d6 right by d7, swaps d6, advances a3 by four, and loads two bytes backwards into d6 with a byte swap, then resets d7 to sixteen minus d1.
- `read_four_bytes_into_d0`: Loads four bytes from the address in a0 into the low bytes of d0, advancing a0 by four, then returns.
- `build_huffman_decode_tables`: Reads a code-length table via a helper, then fills a0 with per-symbol masks, bit patterns, lengths and offsets for decoding, returning after all lengths are processed.
- `build_jump_table_on_stack`: Calculates a difference between a pointer and a global variable, then loops to copy values from a source pointer to a stack-based destination before performing an indirect jump.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_color_register`: Writes a value to the certain hardware register and then enters an infinite loop.
