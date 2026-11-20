# Game 3100 (Blasteroids v1.2 0239): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03100). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2054106 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 3 of 3 named (0 author symbols), 3 of 3 described (3 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11 of 11 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 19838 |
| code | 42 |
| data | 1135153 |
| padding | 1 |
| unused | 899072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| Disk.1:bootblock | 1 | 9 | 2 |

## Cases handled

- Blasteroids_v1.2_0239/Blasteroids_v1.2_0239.hdf: hard-disk image (AmigaDOS OFS, 2208 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree Blasteroids_v1.2_0239_hdf, which is recovered file by file; no block outside the filesystem holds data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### Disk.1:bootblock

- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_bra_lea_move_dbra_jmp`: Short code fragment that jumps to another point in the code, then loads the address of a location in the code into a0, then copies 255 to d0, then loads the address of a fixed memory address into a1, then copies 0 to a fixed memory address, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then jumps to another point in the code.
