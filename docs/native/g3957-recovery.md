# Game 3957 (Hotshot v1.0 2083): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03957). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1296344 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 2 of 2 named (0 author symbols), 2 of 2 described (1 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 429824 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 15 of 15 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 138567 |
| code | 48 |
| data | 1157729 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| hots | 2 | 13 | 1 |
| main (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- Hotshot_v1.0_2083/Hotshot_v1.0_2083.hdf: hard-disk image (AmigaDOS OFS, 1408 blocks): its filesystem parses and all 13 files in it are byte-identical to the extracted tree Hotshot_v1.0_2083_hdf, which is recovered file by file; no block outside the filesystem holds data
- Hotshot_v1.0_2083/Hotshot_v1.0_2083_hdf/Hotshot/data/main: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 429824 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### hots

- `enter_superstate`: Enters supervisor mode using a system call, then performs a bitwise XOR operation on a sequence of words in memory.
