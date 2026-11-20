# Game 3023 (CaptainDynamo v1.2 0995): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03023). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (709750 top-level bytes in 65 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 4 of 4 named (0 author symbols), 4 of 4 described (1 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 187 of 187 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 13715 |
| code | 444 |
| data | 695591 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| VC | 1 | 185 | 3 |

## Cases handled

- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995.hdf: hard-disk image (AmigaDOS OFS, 832 blocks): its filesystem parses and all 34 files in it are byte-identical to the extracted tree CaptainDynamo_v1.2_0995_hdf, which is recovered file by file; no block outside the filesystem holds data
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/BLOCKS1.RAW: Imploder Data (xfd)-packed, unpacked with ancient (33128 -> 64032 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/BLOCKS2.RAW: Imploder Data (xfd)-packed, unpacked with ancient (39154 -> 64032 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/BLOCKS3.RAW: Imploder Data (xfd)-packed, unpacked with ancient (29510 -> 64032 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/DMUS1: Imploder Data (xfd)-packed, unpacked with ancient (10968 -> 37488 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/DMUS2: Imploder Data (xfd)-packed, unpacked with ancient (5536 -> 14110 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/DMUS3: Imploder Data (xfd)-packed, unpacked with ancient (6832 -> 31362 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/FIN.PIC: Imploder Data (xfd)-packed, unpacked with ancient (3744 -> 30752 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/HEIGHTMAP1: Imploder Data (xfd)-packed, unpacked with ancient (3264 -> 81920 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/HEIGHTMAP2: Imploder Data (xfd)-packed, unpacked with ancient (2816 -> 81920 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/HEIGHTMAP3: Imploder Data (xfd)-packed, unpacked with ancient (3986 -> 81920 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/HEIGHTMAP4: Imploder Data (xfd)-packed, unpacked with ancient (3146 -> 81920 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/HEIGHTMAP5: Imploder Data (xfd)-packed, unpacked with ancient (1856 -> 81920 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/HEIGHTMAP6: Imploder Data (xfd)-packed, unpacked with ancient (1760 -> 81920 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/HEIGHTMAP7: Imploder Data (xfd)-packed, unpacked with ancient (1146 -> 81920 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/HEIGHTMAP8: Imploder Data (xfd)-packed, unpacked with ancient (1564 -> 81920 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/LOGO.RAW: Imploder Data (xfd)-packed, unpacked with ancient (1216 -> 2208 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/MAP1: Imploder Data (xfd)-packed, unpacked with ancient (2044 -> 5120 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/MAP2: Imploder Data (xfd)-packed, unpacked with ancient (2202 -> 5120 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/MAP3: Imploder Data (xfd)-packed, unpacked with ancient (2036 -> 5120 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/MAP4: Imploder Data (xfd)-packed, unpacked with ancient (1882 -> 5120 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/MAP5: Imploder Data (xfd)-packed, unpacked with ancient (1990 -> 5120 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/MAP6: Imploder Data (xfd)-packed, unpacked with ancient (2086 -> 5120 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/MAP7: Imploder Data (xfd)-packed, unpacked with ancient (774 -> 5120 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/MAP8: Imploder Data (xfd)-packed, unpacked with ancient (728 -> 5120 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/PANEL.RAW: Imploder Data (xfd)-packed, unpacked with ancient (3376 -> 7712 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/PRESENT.RAW: Imploder Data (xfd)-packed, unpacked with ancient (5484 -> 40992 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/SPRITES.RAW: Imploder Data (xfd)-packed, unpacked with ancient (64030 -> 166464 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/STARS: Imploder Data (xfd)-packed, unpacked with ancient (306 -> 1288 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/TIFONT.RAW: Imploder Data (xfd)-packed, unpacked with ancient (1026 -> 1924 bytes)
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/VC: IMP: File Imploder stream found by ancient scan (30014 -> 45692 bytes) but only 0 of 16 sampled windows are in reference RAM: not used
- CaptainDynamo_v1.2_0995/CaptainDynamo_v1.2_0995_hdf/CaptainDynamo/data/VC: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### VC

- `setup_stack_and_jump`: Adjusts the stack pointer to a fixed location, loads an address from a specific offset into a register, stores that address at another fixed location in memory, and then executes a trap instruction.
- `initialize_hardware`: Sets up custom hardware registers including DMA control, interrupt enable, and interrupt request with specific values, clears a color register, and configures a global variable with a predefined constant before calling another routine.
- `decode_compressed_data`: Decompresses data using a custom bit-based algorithm, reading from a source buffer and writing to a destination buffer while managing bit positions and decoding operations.
