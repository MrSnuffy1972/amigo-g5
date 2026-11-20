# Game 3201 (ContinentalCircus v1.2 1720): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03201). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2027512 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 3 of 3 named (0 author symbols), 3 of 3 described (3 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10 of 10 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 9700 |
| code | 34 |
| data | 2004975 |
| padding | 3 |
| unused | 12800 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| Disk.1:bootblock | 1 | 8 | 2 |

## Cases handled

- ContinentalCircus_v1.2_1720/ContinentalCircus_v1.2_1720_hdf/ContinentalCircus/Disk.1: custom-format (non-DOS) disk, 1733 non-empty blocks: matched against track-loaded segments
- ContinentalCircus_v1.2_1720/ContinentalCircus_v1.2_1720.hdf: hard-disk image (AmigaDOS OFS, 2176 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree ContinentalCircus_v1.2_1720_hdf, which is recovered file by file; no block outside the filesystem holds data
- ContinentalCircus_v1.2_1720/ContinentalCircus_v1.2_1720_hdf/ContinentalCircus/Disk.1: blocks 2-109 (bytes 1024-56320) outside the filesystem hold data, not code (opcode density 0.045, entropy 6.25)
- ContinentalCircus_v1.2_1720/ContinentalCircus_v1.2_1720_hdf/ContinentalCircus/Disk.1: blocks 111-111 (bytes 56832-57344) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- ContinentalCircus_v1.2_1720/ContinentalCircus_v1.2_1720_hdf/ContinentalCircus/Disk.1: blocks 115-121 (bytes 58880-62464) outside the filesystem hold data, not code (opcode density 0.020, entropy 5.66)
- ContinentalCircus_v1.2_1720/ContinentalCircus_v1.2_1720_hdf/ContinentalCircus/Disk.1: blocks 125-126 (bytes 64000-65024) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.93)
- ContinentalCircus_v1.2_1720/ContinentalCircus_v1.2_1720_hdf/ContinentalCircus/Disk.1: blocks 134-359 (bytes 68608-184320) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.41)
- ContinentalCircus_v1.2_1720/ContinentalCircus_v1.2_1720_hdf/ContinentalCircus/Disk.1: blocks 361-368 (bytes 184832-188928) outside the filesystem hold data, not code (opcode density 0.005, entropy 3.21)
- ContinentalCircus_v1.2_1720/ContinentalCircus_v1.2_1720_hdf/ContinentalCircus/Disk.1: blocks 370-377 (bytes 189440-193536) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.47)
- ContinentalCircus_v1.2_1720/ContinentalCircus_v1.2_1720_hdf/ContinentalCircus/Disk.1: blocks 379-386 (bytes 194048-198144) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.81)
- ContinentalCircus_v1.2_1720/ContinentalCircus_v1.2_1720_hdf/ContinentalCircus/Disk.1: blocks 388-396 (bytes 198656-203264) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.13)
- ContinentalCircus_v1.2_1720/ContinentalCircus_v1.2_1720_hdf/ContinentalCircus/Disk.1: blocks 398-405 (bytes 203776-207872) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.75)
- ContinentalCircus_v1.2_1720/ContinentalCircus_v1.2_1720_hdf/ContinentalCircus/Disk.1: blocks 407-414 (bytes 208384-212480) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.95)
- ContinentalCircus_v1.2_1720/ContinentalCircus_v1.2_1720_hdf/ContinentalCircus/Disk.1: blocks 416-423 (bytes 212992-217088) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.73)
- ContinentalCircus_v1.2_1720/ContinentalCircus_v1.2_1720_hdf/ContinentalCircus/Disk.1: blocks 425-432 (bytes 217600-221696) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.91)
- ContinentalCircus_v1.2_1720/ContinentalCircus_v1.2_1720_hdf/ContinentalCircus/Disk.1: blocks 434-721 (bytes 222208-369664) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.76)
- ContinentalCircus_v1.2_1720/ContinentalCircus_v1.2_1720_hdf/ContinentalCircus/Disk.1: blocks 723-733 (bytes 370176-375808) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.88)
- ContinentalCircus_v1.2_1720/ContinentalCircus_v1.2_1720_hdf/ContinentalCircus/Disk.1: blocks 735-1759 (bytes 376320-901120) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.24)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### Disk.1:bootblock

- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_bra_lea_move_dbra_jmp`: Short code fragment that jumps to another point in the code, then loads the address of a location in the code into a0, then copies 255 to d0, then loads the address of a fixed memory address into a1, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then jumps to another point in the code.
