# Game 4402 (MikeReadsComputerPopQuiz v1.0 0867): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04402). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (824972 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 3 of 3 named (0 author symbols), 3 of 3 described (3 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7 of 7 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 19513 |
| code | 18 |
| data | 789567 |
| padding | 2 |
| unused | 15872 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 7 | 3 |

## Cases handled

- MikeReadsComputerPopQuiz_v1.0_0867/MikeReadsComputerPopQuiz_v1.0_0867_hdf/MikeReadsComputerPopQuiz/Disk.1: raw disk image (344064 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- MikeReadsComputerPopQuiz_v1.0_0867/MikeReadsComputerPopQuiz_v1.0_0867.hdf: hard-disk image (AmigaDOS OFS, 896 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree MikeReadsComputerPopQuiz_v1.0_0867_hdf, which is recovered file by file; no block outside the filesystem holds data
- MikeReadsComputerPopQuiz_v1.0_0867/MikeReadsComputerPopQuiz_v1.0_0867_hdf/MikeReadsComputerPopQuiz/Disk.1: chunks 0-3 (bytes 0-1024) outside the filesystem hold data, not code (opcode density 0.049, entropy 4.40)
- MikeReadsComputerPopQuiz_v1.0_0867/MikeReadsComputerPopQuiz_v1.0_0867_hdf/MikeReadsComputerPopQuiz/Disk.1: chunks 24-80 (bytes 6144-20736) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.22)
- MikeReadsComputerPopQuiz_v1.0_0867/MikeReadsComputerPopQuiz_v1.0_0867_hdf/MikeReadsComputerPopQuiz/Disk.1: chunks 96-520 (bytes 24576-133376) outside the filesystem hold data, not code (opcode density 0.023, entropy 4.94)
- MikeReadsComputerPopQuiz_v1.0_0867/MikeReadsComputerPopQuiz_v1.0_0867_hdf/MikeReadsComputerPopQuiz/Disk.1: chunks 523-892 (bytes 133888-228608) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.12)
- MikeReadsComputerPopQuiz_v1.0_0867/MikeReadsComputerPopQuiz_v1.0_0867_hdf/MikeReadsComputerPopQuiz/Disk.1: chunks 896-1321 (bytes 229376-338432) outside the filesystem hold data, not code (opcode density 0.039, entropy 6.37)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_pea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_move_addq_rts`: Short code fragment that copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.
