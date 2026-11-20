# Game 2804 (BallGame v1.0 2051): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02804). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (854654 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 4 of 4 named (0 author symbols), 4 of 4 described (3 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12 of 12 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 11097 |
| code | 36 |
| data | 842241 |
| unused | 1280 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 12 | 4 |

## Cases handled

- BallGame_v1.0_2051/BallGame_v1.0_2051_hdf/BallGame/Disk.1: raw disk image (360448 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- BallGame_v1.0_2051/BallGame_v1.0_2051.hdf: hard-disk image (AmigaDOS OFS, 928 blocks): its filesystem parses and all 7 files in it are byte-identical to the extracted tree BallGame_v1.0_2051_hdf, which is recovered file by file; no block outside the filesystem holds data
- BallGame_v1.0_2051/BallGame_v1.0_2051_hdf/BallGame/Disk.1: chunks 0-270 (bytes 0-69376) outside the filesystem hold data, not code (opcode density 0.017, entropy 4.20)
- BallGame_v1.0_2051/BallGame_v1.0_2051_hdf/BallGame/Disk.1: chunks 272-274 (bytes 69632-70400) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.49)
- BallGame_v1.0_2051/BallGame_v1.0_2051_hdf/BallGame/Disk.1: chunks 276-794 (bytes 70656-203520) outside the filesystem hold data, not code (opcode density 0.003, entropy 7.03)
- BallGame_v1.0_2051/BallGame_v1.0_2051_hdf/BallGame/Disk.1: chunks 798-1407 (bytes 204288-360448) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.07)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_movem_movea_jsr_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to a2, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_pea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_move_addq_rts`: Short code fragment that copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.
