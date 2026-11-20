# Game 3802 (GhostBattle v1.2): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03802). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2258760 top-level bytes in 27 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 13 of 13 named (0 author symbols), 13 of 13 described (13 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 34 of 34 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12676 |
| code | 134 |
| data | 2242974 |
| unused | 2976 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 34 | 13 |

## Cases handled

- GhostBattle_v1.2/GhostBattle_v1.2_hdf/GhostBattle/Disk.1: raw disk image (997792 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- GhostBattle_v1.2/GhostBattle_v1.2.hdf: hard-disk image (AmigaDOS OFS, 2432 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree GhostBattle_v1.2_hdf, which is recovered file by file; no block outside the filesystem holds data
- GhostBattle_v1.2/GhostBattle_v1.2_hdf/GhostBattle/Disk.1: chunks 0-3885 (bytes 0-994816) hold 20 packed streams (IMP: File Imploder) found by scanning, 99% of the bytes

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_adda_tst_btst_bne_rts`: Short code fragment that adds a location in the code to a2, then tests a fixed memory address (3 times in a row), then tests bit 6 of a fixed memory address, then branches if the last result was not zero, then tests a fixed memory address, then returns.
- `frag_lea_bra`: Short code fragment that loads the address of the memory at an offset from a2 into a2, then jumps to another point in the code.
- `frag_lea_addq_bra`: Short code fragment that loads the address of an indexed entry from a1 into a1, then loads the address of the memory at an offset from a2 into a2, then adds 2 to the top of the stack, then jumps to another point in the code.
- `frag_move_bra`: Short code fragment that copies 641 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies 643 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_bra_3`: Short code fragment that copies 64014 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_bra_4`: Short code fragment that copies 3220 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_bra_5`: Short code fragment that copies 1025 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_bra_6`: Short code fragment that copies 3212 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_bra_7`: Short code fragment that copies 64040 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_bra_8`: Short code fragment that copies 660 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_bra_9`: Short code fragment that copies 449 to the memory at an offset from a6, then jumps to another point in the code.
