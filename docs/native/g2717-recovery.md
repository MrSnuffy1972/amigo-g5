# Game 2717 (Aquaventura v1.2 0427): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02717). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4396993 top-level bytes in 13 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 1 of 1 named (0 author symbols), 1 of 1 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2 of 2 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 16509 |
| code | 4 |
| data | 4179776 |
| unused | 200704 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |

## Cases handled

- Aquaventura_v1.2_0427/Aquaventura_v1.2_0427_hdf/Aquaventura/Disk.1: raw disk image (976896 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Aquaventura_v1.2_0427/Aquaventura_v1.2_0427_hdf/Aquaventura/Disk.2: raw disk image (976896 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Aquaventura_v1.2_0427/Aquaventura_v1.2_0427.hdf: hard-disk image (AmigaDOS OFS, 4736 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree Aquaventura_v1.2_0427_hdf, which is recovered file by file; no block outside the filesystem holds data
- Aquaventura_v1.2_0427/Aquaventura_v1.2_0427_hdf/Aquaventura/Disk.1: chunks 0-21 (bytes 0-5632) outside the filesystem hold data, not code (opcode density 0.020, entropy 2.65)
- Aquaventura_v1.2_0427/Aquaventura_v1.2_0427_hdf/Aquaventura/Disk.1: chunks 48-1872 (bytes 12288-479488) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.44)
- Aquaventura_v1.2_0427/Aquaventura_v1.2_0427_hdf/Aquaventura/Disk.1: chunks 1896-3431 (bytes 485376-878592) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.38)
- Aquaventura_v1.2_0427/Aquaventura_v1.2_0427_hdf/Aquaventura/Disk.1: chunks 3456-3614 (bytes 884736-925440) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.48)
- Aquaventura_v1.2_0427/Aquaventura_v1.2_0427_hdf/Aquaventura/Disk.1: chunks 3639-3774 (bytes 931584-966400) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.69)
- Aquaventura_v1.2_0427/Aquaventura_v1.2_0427_hdf/Aquaventura/Disk.1: chunks 3814-3814 (bytes 976384-976640) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.12)
- Aquaventura_v1.2_0427/Aquaventura_v1.2_0427_hdf/Aquaventura/Disk.2: chunks 48-362 (bytes 12288-92928) hold 3 packed streams (BK: ByteKiller) found by scanning, 93% of the bytes
- Aquaventura_v1.2_0427/Aquaventura_v1.2_0427_hdf/Aquaventura/Disk.2: chunks 364-2444 (bytes 93184-625920) hold a BK: ByteKiller stream (532736 -> 61940 bytes)
- Aquaventura_v1.2_0427/Aquaventura_v1.2_0427_hdf/Aquaventura/Disk.2: chunks 2448-3220 (bytes 626688-824576) hold 1 packed streams (BK: ByteKiller) found by scanning, 97% of the bytes

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
