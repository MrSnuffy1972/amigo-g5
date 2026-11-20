# Game 3696 (FootballerOfTheYear2 v1.0 0654): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03696). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1095531 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 2 of 2 named (0 author symbols), 2 of 2 described (2 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 480000 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3 of 3 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 8371 |
| code | 10 |
| data | 1087137 |
| padding | 13 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| FOTY2 (raw code file, load address unknown) | 1 | 1 | 1 |

## Cases handled

- FootballerOfTheYear2_v1.0_0654/FootballerOfTheYear2_v1.0_0654.hdf: hard-disk image (AmigaDOS OFS, 1184 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree FootballerOfTheYear2_v1.0_0654_hdf, which is recovered file by file; no block outside the filesystem holds data
- FootballerOfTheYear2_v1.0_0654/FootballerOfTheYear2_v1.0_0654_hdf/FootballerOfTheYear2/FOTY2: high-entropy data file (7.65 bits/byte), classified as packed data
- FootballerOfTheYear2_v1.0_0654/FootballerOfTheYear2_v1.0_0654_hdf/FootballerOfTheYear2/FOTY2: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 480000 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### FOTY2 (raw code file, load address unknown)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
