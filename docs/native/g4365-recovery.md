# Game 4365 (MentalImage1 v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04365). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2050936 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 4 of 4 named (0 author symbols), 4 of 4 described (4 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16 of 16 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 15885 |
| code | 64 |
| data | 2034987 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 7 | 3 |
| Disk.1:bootblock | 1 | 9 | 1 |

## Cases handled

- MentalImage1_v1.0/MentalImage1_v1.0_hdf/MentalImage1/Disk.1: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- MentalImage1_v1.0/MentalImage1_v1.0.hdf: hard-disk image (AmigaDOS OFS, 2208 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree MentalImage1_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- MentalImage1_v1.0/MentalImage1_v1.0_hdf/MentalImage1/Disk.1: blocks 2-1759 (bytes 1024-901120) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.02)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_pea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_move_addq_rts`: Short code fragment that copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.

### Disk.1:bootblock

- `frag_move_movea_jsr_jmp`: Short code fragment that copies 0 to a fixed memory address, then copies 2 to the memory at an offset from a1, then copies a location in the code to the memory at an offset from a1 (3 times in a row), then copies a fixed memory address to a6, then calls another routine, then copies a location in the code to a0, then jumps to the address in a0.
