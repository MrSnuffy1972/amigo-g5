# Game 4273 (Livingstone2 v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04273). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1757183 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 3 of 3 named (0 author symbols), 3 of 3 described (1 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 46144 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 412 of 412 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12365 |
| code | 1608 |
| data | 1739109 |
| padding | 5 |
| unused | 4096 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| Disk.1 bytes 768-2560 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 3328-6912 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 85504-87552 (code never loaded by a reference run) | 1 | 410 | 2 |
| osemu.400 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- Livingstone2_v1.0/Livingstone2_v1.0_hdf/Livingstone2/Disk.1: raw disk image (738304 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Livingstone2_v1.0/Livingstone2_v1.0.hdf: hard-disk image (AmigaDOS OFS, 1888 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree Livingstone2_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- Livingstone2_v1.0/Livingstone2_v1.0_hdf/Livingstone2/Disk.1: bytes 768-2560 look like code no run loaded: recovered position-independently (load address unknown)
- Livingstone2_v1.0/Livingstone2_v1.0_hdf/Livingstone2/Disk.1: bytes 3328-6912 look like code no run loaded: recovered position-independently (load address unknown)
- Livingstone2_v1.0/Livingstone2_v1.0_hdf/Livingstone2/Disk.1: bytes 85504-87552 look like code no run loaded: recovered position-independently (load address unknown)
- Livingstone2_v1.0/Livingstone2_v1.0_hdf/Livingstone2/osemu.400: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Livingstone2_v1.0/Livingstone2_v1.0_hdf/Livingstone2/Disk.1: 3 logged loads cover 7424 bytes of the image
- Livingstone2_v1.0/Livingstone2_v1.0_hdf/Livingstone2/Disk.1: chunks 0-1 (bytes 0-512) outside the filesystem hold data, not code (opcode density 0.039, entropy 1.73)
- Livingstone2_v1.0/Livingstone2_v1.0_hdf/Livingstone2/Disk.1: chunks 31-156 (bytes 7936-40192) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.61)
- Livingstone2_v1.0/Livingstone2_v1.0_hdf/Livingstone2/Disk.1: chunks 158-332 (bytes 40448-85248) outside the filesystem hold data, not code (opcode density 0.054, entropy 6.27)
- Livingstone2_v1.0/Livingstone2_v1.0_hdf/Livingstone2/Disk.1: chunks 345-1680 (bytes 88320-430336) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.34)
- Livingstone2_v1.0/Livingstone2_v1.0_hdf/Livingstone2/Disk.1: chunks 1684-2883 (bytes 431104-738304) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.42)
- 3 never-loaded region(s), 44096 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### Disk.1 bytes 85504-87552 (code never loaded by a reference run)

- `initialize_dma_channels`: Clears a series of words in memory starting at a specific address, then sets bits in a hardware register to enable DMA channels.
- `update_audio_parameters`: Updates audio channel parameters and handles audio buffer management by copying data from a source to a hardware register and setting up audio playback.
