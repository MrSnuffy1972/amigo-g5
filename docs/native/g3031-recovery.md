# Game 3031 (CarlLewisChallenge v1.1 0639): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03031). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4021536 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 6 of 6 named (0 author symbols), 6 of 6 described (5 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 188928 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 208 of 208 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 5648 |
| code | 1060 |
| data | 3940549 |
| padding | 39 |
| unused | 74240 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| Disk.1:bootblock | 1 | 8 | 2 |
| Disk.2:bootblock | 1 | 8 | 2 |
| Disk.1 bytes 386048-419328 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 582656-605696 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.2 bytes 1024-54784 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.2 bytes 64000-137216 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.2 bytes 137728-143360 (code never loaded by a reference run) | 1 | 190 | 1 |

## Cases handled

- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.1: custom-format (non-DOS) disk, 1656 non-empty blocks: matched against track-loaded segments
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.2: custom-format (non-DOS) disk, 1715 non-empty blocks: matched against track-loaded segments
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639.hdf: hard-disk image (AmigaDOS OFS, 4320 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree CarlLewisChallenge_v1.1_0639_hdf, which is recovered file by file; no block outside the filesystem holds data
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.1: bytes 386048-419328 look like code no run loaded: recovered position-independently (load address unknown)
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.1: bytes 582656-605696 look like code no run loaded: recovered position-independently (load address unknown)
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.2: bytes 1024-54784 look like code no run loaded: recovered position-independently (load address unknown)
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.2: bytes 64000-137216 look like code no run loaded: recovered position-independently (load address unknown)
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.2: bytes 137728-143360 look like code no run loaded: recovered position-independently (load address unknown)
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.1: 2 logged loads cover 56320 bytes of the image
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.1: blocks 2-304 (bytes 1024-156160) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.20)
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.1: blocks 306-526 (bytes 156672-269824) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.96)
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.1: blocks 528-528 (bytes 270336-270848) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.07)
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.1: blocks 576-584 (bytes 294912-299520) outside the filesystem hold data, not code (opcode density 0.020, entropy 3.55)
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.1: blocks 588-593 (bytes 301056-304128) outside the filesystem hold data, not code (opcode density 0.005, entropy 3.97)
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.1: blocks 609-752 (bytes 311808-385536) outside the filesystem hold data, not code (opcode density 0.026, entropy 4.38)
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.1: blocks 826-1135 (bytes 422912-581632) outside the filesystem hold data, not code (opcode density 0.014, entropy 5.15)
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.1: blocks 1208-1759 (bytes 618496-901120) hold ASCII text
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.2: 3 logged loads cover 132608 bytes of the image
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.2: blocks 303-319 (bytes 155136-163840) outside the filesystem hold data, not code (opcode density 0.015, entropy 2.43)
- CarlLewisChallenge_v1.1_0639/CarlLewisChallenge_v1.1_0639_hdf/CarlLewisChallenge/Disk.2: blocks 321-1759 (bytes 164352-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.45)
- 4 never-loaded region(s), 183296 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

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

### Disk.2:bootblock

- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_bra_lea_move_dbra_jmp_2`: Short code fragment that jumps to another point in the code, then loads the address of a location in the code into a0, then copies 255 to d0, then loads the address of a fixed memory address into a1, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then jumps to another point in the code.

### Disk.2 bytes 137728-143360 (code never loaded by a reference run)

- `initialize_blitter`: Sets up blitter registers with values from a table based on a global variable, waits for blitter completion, and updates multiple structure fields with blitter parameters.
