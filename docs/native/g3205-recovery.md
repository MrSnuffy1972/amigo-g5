# Game 3205 (CoolSpot v1.0 1345): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03205). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (6058734 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 3 of 3 named (0 author symbols), 3 of 3 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 70 of 70 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 28627 |
| code | 254 |
| data | 5905435 |
| padding | 2 |
| unused | 124416 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 7 | 2 |
| Disk.1:bootblock | 1 | 63 | 1 |

## Cases handled

- CoolSpot_v1.0_1345/CoolSpot_v1.0_1345_hdf/CoolSpot/Disk.1: custom-format (non-DOS) disk, 1756 non-empty blocks: matched against track-loaded segments
- CoolSpot_v1.0_1345/CoolSpot_v1.0_1345_hdf/CoolSpot/Disk.2: custom-format (non-DOS) disk, 1742 non-empty blocks: matched against track-loaded segments
- CoolSpot_v1.0_1345/CoolSpot_v1.0_1345_hdf/CoolSpot/Disk.3: custom-format (non-DOS) disk, 1533 non-empty blocks: matched against track-loaded segments
- CoolSpot_v1.0_1345/CoolSpot_v1.0_1345.hdf: hard-disk image (AmigaDOS OFS, 6496 blocks): its filesystem parses and all 7 files in it are byte-identical to the extracted tree CoolSpot_v1.0_1345_hdf, which is recovered file by file; no block outside the filesystem holds data
- CoolSpot_v1.0_1345/CoolSpot_v1.0_1345_hdf/CoolSpot/Disk.1: blocks 2-315 (bytes 1024-161792) outside the filesystem hold data, not code (opcode density 0.041, entropy 6.53)
- CoolSpot_v1.0_1345/CoolSpot_v1.0_1345_hdf/CoolSpot/Disk.1: blocks 317-318 (bytes 162304-163328) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- CoolSpot_v1.0_1345/CoolSpot_v1.0_1345_hdf/CoolSpot/Disk.1: blocks 320-1759 (bytes 163840-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.25)
- CoolSpot_v1.0_1345/CoolSpot_v1.0_1345_hdf/CoolSpot/Disk.2: blocks 2-1743 (bytes 1024-892928) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.86)
- CoolSpot_v1.0_1345/CoolSpot_v1.0_1345_hdf/CoolSpot/Disk.3: blocks 2-1306 (bytes 1024-669184) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.82)
- CoolSpot_v1.0_1345/CoolSpot_v1.0_1345_hdf/CoolSpot/Disk.3: blocks 1308-1535 (bytes 669696-786432) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.47)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_movem_movea_jsr_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to a2, then calls another routine, then restores saved registers from the stack, then returns.

### Disk.1:bootblock

- `initialize_boot_sequence`: Loops while writing a decreasing value to a register, then calls a routine to allocate memory, checks the pointer, and if valid, proceeds to perform a sequence of memory allocations, pointer updates, and routine calls involving a structure, before finally clearing a register and returning.
