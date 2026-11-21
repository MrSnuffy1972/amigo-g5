# Game 4512 (NeverMind v1.0 1507): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04512). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2192938 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 3 of 3 named (0 author symbols), 3 of 3 described (1 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 43264 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 117 of 122 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 20929 |
| code | 478 |
| data | 2106251 |
| unused | 65280 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| Disk.1 bytes 0-1024 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 798720-799232 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 800768-806144 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 807168-811520 (code never loaded by a reference run) | 1 | 68 | 1 |
| Disk.1 bytes 812032-826368 (code never loaded by a reference run) | 1 | 52 | 1 |
| Disk.1 bytes 827904-845568 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- NeverMind_v1.0_1507/NeverMind_v1.0_1507_hdf/NeverMind/Disk.1: raw disk image (952320 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- NeverMind_v1.0_1507/NeverMind_v1.0_1507.hdf: hard-disk image (AmigaDOS OFS, 2368 blocks): its filesystem parses and all 7 files in it are byte-identical to the extracted tree NeverMind_v1.0_1507_hdf, which is recovered file by file; no block outside the filesystem holds data
- NeverMind_v1.0_1507/NeverMind_v1.0_1507_hdf/NeverMind/Disk.1: bytes 0-1024 look like code no run loaded: recovered position-independently (load address unknown)
- NeverMind_v1.0_1507/NeverMind_v1.0_1507_hdf/NeverMind/Disk.1: bytes 798720-799232 look like code no run loaded: recovered position-independently (load address unknown)
- NeverMind_v1.0_1507/NeverMind_v1.0_1507_hdf/NeverMind/Disk.1: bytes 800768-806144 look like code no run loaded: recovered position-independently (load address unknown)
- NeverMind_v1.0_1507/NeverMind_v1.0_1507_hdf/NeverMind/Disk.1: bytes 807168-811520 look like code no run loaded: recovered position-independently (load address unknown)
- NeverMind_v1.0_1507/NeverMind_v1.0_1507_hdf/NeverMind/Disk.1: bytes 812032-826368 look like code no run loaded: recovered position-independently (load address unknown)
- NeverMind_v1.0_1507/NeverMind_v1.0_1507_hdf/NeverMind/Disk.1: bytes 827904-845568 look like code no run loaded: recovered position-independently (load address unknown)
- NeverMind_v1.0_1507/NeverMind_v1.0_1507_hdf/NeverMind/Disk.1: 6 logged loads cover 43264 bytes of the image
- NeverMind_v1.0_1507/NeverMind_v1.0_1507_hdf/NeverMind/Disk.1: chunks 48-204 (bytes 12288-52480) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.15)
- NeverMind_v1.0_1507/NeverMind_v1.0_1507_hdf/NeverMind/Disk.1: chunks 216-2136 (bytes 55296-547072) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.61)
- NeverMind_v1.0_1507/NeverMind_v1.0_1507_hdf/NeverMind/Disk.1: chunks 2146-2159 (bytes 549376-552960) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.48)
- NeverMind_v1.0_1507/NeverMind_v1.0_1507_hdf/NeverMind/Disk.1: chunks 2208-3095 (bytes 565248-792576) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.13)
- NeverMind_v1.0_1507/NeverMind_v1.0_1507_hdf/NeverMind/Disk.1: chunks 3374-3415 (bytes 863744-874496) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.05)
- NeverMind_v1.0_1507/NeverMind_v1.0_1507_hdf/NeverMind/Disk.1: chunks 3417-3426 (bytes 874752-877312) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.89)
- NeverMind_v1.0_1507/NeverMind_v1.0_1507_hdf/NeverMind/Disk.1: chunks 3456-3719 (bytes 884736-952320) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.98)
- 6 never-loaded region(s), 43264 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### Disk.1 bytes 807168-811520 (code never loaded by a reference run)

- `copy_memory_block`: Copies 179 words from one memory location to another using a loop that decrements a counter in register d0.

### Disk.1 bytes 812032-826368 (code never loaded by a reference run)

- `compare_and_copy`: Loads a value from memory, compares it with a constant, and branches based on the result.
