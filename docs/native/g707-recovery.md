# Game 707 (MortalKombat v0.9 0288): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00707). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4398511 top-level bytes in 122 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 1 of 1 named (0 author symbols), 1 of 1 described (0 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Independent decoder cross-check (Ghidra against capstone) | 2 of 2 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 21514 |
| code | 4 |
| data | 4333979 |
| padding | 6 |
| unused | 43008 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| Disk.1 bytes 43776-163584 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- MortalKombat_v0.9_0288/MortalKombat_v0.9_0288_hdf/MortalKombat/Disk.1: raw disk image (970752 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- MortalKombat_v0.9_0288/MortalKombat_v0.9_0288_hdf/MortalKombat/Disk.2: raw disk image (970752 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- MortalKombat_v0.9_0288/MortalKombat_v0.9_0288.hdf: high-entropy data file (7.69 bits/byte), classified as packed data
- MortalKombat_v0.9_0288/MortalKombat_v0.9_0288_hdf/MortalKombat/Disk.1: bytes 43776-163584 look like code no run loaded: recovered position-independently (load address unknown)
- MortalKombat_v0.9_0288/MortalKombat_v0.9_0288_hdf/MortalKombat/Disk.1: 1 logged loads cover 119808 bytes of the image
- MortalKombat_v0.9_0288/MortalKombat_v0.9_0288_hdf/MortalKombat/Disk.1: chunks 0-128 (bytes 0-33024) outside the filesystem hold data, not code (opcode density 0.057, entropy 6.11)
- MortalKombat_v0.9_0288/MortalKombat_v0.9_0288_hdf/MortalKombat/Disk.1: chunks 671-3791 (bytes 171776-970752) hold 75 packed streams (RNC1: Rob Northen RNC1 Compressor) found by scanning, 83% of the bytes
- MortalKombat_v0.9_0288/MortalKombat_v0.9_0288_hdf/MortalKombat/Disk.2: chunks 0-3689 (bytes 0-944640) hold 36 packed streams (C: Compact, RNC1: Rob Northen RNC1 Compressor) found by scanning, 95% of the bytes
- MortalKombat_v0.9_0288/MortalKombat_v0.9_0288_hdf/MortalKombat/Disk.2: chunks 3696-3697 (bytes 946176-946688) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- MortalKombat_v0.9_0288/MortalKombat_v0.9_0288_hdf/MortalKombat/Disk.2: chunks 3720-3721 (bytes 952320-952832) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- MortalKombat_v0.9_0288/MortalKombat_v0.9_0288_hdf/MortalKombat/Disk.2: chunks 3744-3745 (bytes 958464-958976) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- MortalKombat_v0.9_0288/MortalKombat_v0.9_0288_hdf/MortalKombat/Disk.2: chunks 3768-3769 (bytes 964608-965120) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `return_constant_all_ones`: The routine returns the constant value all bits set.
