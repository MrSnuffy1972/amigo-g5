# Game 3812 (GlobalGladiators v1.1 0657): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03812). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4023652 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 3 of 3 named (0 author symbols), 3 of 3 described (2 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 75 of 75 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 8350 |
| code | 262 |
| data | 4014016 |
| unused | 1024 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| Disk.1:bootblock | 1 | 63 | 1 |
| Disk.2:bootblock | 1 | 10 | 1 |

## Cases handled

- GlobalGladiators_v1.1_0657/GlobalGladiators_v1.1_0657_hdf/GlobalGladiators/Disk.1: custom-format (non-DOS) disk, 1756 non-empty blocks: matched against track-loaded segments
- GlobalGladiators_v1.1_0657/GlobalGladiators_v1.1_0657_hdf/GlobalGladiators/Disk.2: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- GlobalGladiators_v1.1_0657/GlobalGladiators_v1.1_0657.hdf: hard-disk image (AmigaDOS OFS, 4320 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree GlobalGladiators_v1.1_0657_hdf, which is recovered file by file; no block outside the filesystem holds data
- GlobalGladiators_v1.1_0657/GlobalGladiators_v1.1_0657_hdf/GlobalGladiators/Disk.1: blocks 2-395 (bytes 1024-202752) outside the filesystem hold data, not code (opcode density 0.028, entropy 6.40)
- GlobalGladiators_v1.1_0657/GlobalGladiators_v1.1_0657_hdf/GlobalGladiators/Disk.1: blocks 397-398 (bytes 203264-204288) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- GlobalGladiators_v1.1_0657/GlobalGladiators_v1.1_0657_hdf/GlobalGladiators/Disk.1: blocks 400-1759 (bytes 204800-901120) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.52)
- GlobalGladiators_v1.1_0657/GlobalGladiators_v1.1_0657_hdf/GlobalGladiators/Disk.2: blocks 2-1759 (bytes 1024-901120) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.45)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### Disk.1:bootblock

- `initialize_boot_sequence`: Loops while writing a decreasing value to a color register, then allocates memory, checks the pointer, and if valid, proceeds to perform a sequence of memory allocations, pointer updates, and routine calls involving a structure, before finally clearing the color register and returning.

### Disk.2:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
