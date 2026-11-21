# Game 4370 (MervTheMerciless v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04370). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (704229 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 1 of 1 named (0 author symbols), 1 of 1 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 430080 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2 of 2 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 9681 |
| code | 4 |
| data | 694544 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| MERV.BIN#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- MervTheMerciless_v1.0/MervTheMerciless_v1.0.hdf: hard-disk image (AmigaDOS OFS, 768 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree MervTheMerciless_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- MervTheMerciless_v1.0/MervTheMerciless_v1.0_hdf/MervTheMerciless/MERV.BIN: Imploder Data (xfd)-packed, unpacked with ancient (300026 -> 430080 bytes)
- MervTheMerciless_v1.0/MervTheMerciless_v1.0_hdf/MervTheMerciless/MERV.BIN#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 430080 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
