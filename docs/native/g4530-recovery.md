# Game 4530 (NinjaSpirit v1.2 1669): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04530). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2024260 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 2 of 2 named (0 author symbols), 2 of 2 described (1 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 24 of 24 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 7095 |
| code | 84 |
| data | 1123641 |
| unused | 893440 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| Disk.1:bootblock | 1 | 22 | 1 |
| logged disk load from Disk.1 boot-block load | 1 | 0 | 0 |

## Cases handled

- NinjaSpirit_v1.2_1669/NinjaSpirit_v1.2_1669.hdf: hard-disk image (AmigaDOS OFS, 2176 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree NinjaSpirit_v1.2_1669_hdf, which is recovered file by file; no block outside the filesystem holds data
- NinjaSpirit_v1.2_1669/NinjaSpirit_v1.2_1669_hdf/NinjaSpirit/Disk.1: the boot block reads 5120 bytes from disk offset 418816 to an address (found statically); recovered as a unit
- NinjaSpirit_v1.2_1669/NinjaSpirit_v1.2_1669_hdf/NinjaSpirit/Disk.1: 1 logged loads cover 0 bytes of the image
- NinjaSpirit_v1.2_1669/NinjaSpirit_v1.2_1669_hdf/NinjaSpirit/Disk.1: blocks 11-21 (bytes 5632-11264) hold ASCII text

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### Disk.1:bootblock

- `allocate_and_execute`: Allocates memory and executes a routine through a function pointer, returning the result of that execution.
