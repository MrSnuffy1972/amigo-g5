# Game 3750 (FutureTank v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03750). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2028853 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 9 of 9 named (0 author symbols), 9 of 9 described (6 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 41472 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 79 of 79 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 22295 |
| code | 272 |
| data | 1970956 |
| padding | 2 |
| unused | 35328 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| Disk.1:bootblock | 1 | 14 | 1 |
| logged disk load from Disk.1 boot-block load | 1 | 63 | 7 |
| Disk.1 bytes 11264-52736 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- FutureTank_v1.0/FutureTank_v1.0_hdf/FutureTank/Disk.1: custom-format (non-DOS) disk, 1689 non-empty blocks: matched against track-loaded segments
- FutureTank_v1.0/FutureTank_v1.0.hdf: hard-disk image (AmigaDOS OFS, 2176 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree FutureTank_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- FutureTank_v1.0/FutureTank_v1.0_hdf/FutureTank/Disk.1: the boot block reads 11264 bytes from disk offset 0 to an address (found statically); recovered as a unit
- FutureTank_v1.0/FutureTank_v1.0_hdf/FutureTank/Disk.1: bytes 11264-52736 look like code no run loaded: recovered position-independently (load address unknown)
- FutureTank_v1.0/FutureTank_v1.0_hdf/FutureTank/Disk.1: 2 logged loads cover 52736 bytes of the image
- FutureTank_v1.0/FutureTank_v1.0_hdf/FutureTank/Disk.1: blocks 104-104 (bytes 53248-53760) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.35)
- FutureTank_v1.0/FutureTank_v1.0_hdf/FutureTank/Disk.1: blocks 106-106 (bytes 54272-54784) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.35)
- FutureTank_v1.0/FutureTank_v1.0_hdf/FutureTank/Disk.1: blocks 108-189 (bytes 55296-97280) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.30)
- FutureTank_v1.0/FutureTank_v1.0_hdf/FutureTank/Disk.1: blocks 220-377 (bytes 112640-193536) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.47)
- FutureTank_v1.0/FutureTank_v1.0_hdf/FutureTank/Disk.1: blocks 380-389 (bytes 194560-199680) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.08)
- FutureTank_v1.0/FutureTank_v1.0_hdf/FutureTank/Disk.1: blocks 395-456 (bytes 202240-233984) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.10)
- FutureTank_v1.0/FutureTank_v1.0_hdf/FutureTank/Disk.1: blocks 462-472 (bytes 236544-242176) outside the filesystem hold data, not code (opcode density 0.048, entropy 5.57)
- FutureTank_v1.0/FutureTank_v1.0_hdf/FutureTank/Disk.1: blocks 478-707 (bytes 244736-362496) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.95)
- FutureTank_v1.0/FutureTank_v1.0_hdf/FutureTank/Disk.1: blocks 710-898 (bytes 363520-460288) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.54)
- FutureTank_v1.0/FutureTank_v1.0_hdf/FutureTank/Disk.1: blocks 902-1023 (bytes 461824-524288) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.71)
- FutureTank_v1.0/FutureTank_v1.0_hdf/FutureTank/Disk.1: blocks 1034-1169 (bytes 529408-599040) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.73)
- FutureTank_v1.0/FutureTank_v1.0_hdf/FutureTank/Disk.1: blocks 1174-1759 (bytes 601088-901120) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.83)
- 1 never-loaded region(s), 41472 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### Disk.1:bootblock

- `initialize_dma_and_colors`: Sets up DMA control and clears color registers, then configures a structure with specific values before calling a library routine and jumping to another location.

### logged disk load from Disk.1 boot-block load

- `frag_ori_jmp`: Short code fragment that merges 174 into the memory at an offset from a4, then jumps to another point in the code.
- `setup_interrupts_and_dma`: Copies data from a source table to a destination, configures DMA and interrupt settings, updates global variables, and performs several memory operations before jumping to a final address.
- `copy_and_compare_bytes`: Processes byte sequences by comparing them against stored values and copying bytes to an output buffer based on comparison results, with loop control and conditional branching.
- `frag_move_tst_bne_bra`: Short code fragment that copies the memory at a0 (advancing a0) to d1, then copies the memory at a0 (advancing a0) to d0, then tests d1, then branches if the last result was not zero, then copies d0 to the memory at a1 (advancing a1), then jumps to another point in the code.
- `frag_andi_move_dbra_bra`: Short code fragment that masks d1 with 255, then copies d0 to the memory at a1 (advancing a1), then loops back while counting down d1, then jumps to another point in the code.
- `frag_move_bra`: Short code fragment that copies the memory at a0 (advancing a0) to d1, then copies d6 to d0, then jumps to another point in the code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
