# Game 2880 (BigNoseTheCaveman v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02880). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (930331 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 4 of 4 named (0 author symbols), 4 of 4 described (3 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 75406 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 24 of 24 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 249105 |
| code | 90 |
| data | 681136 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 7 | 3 |
| VC | 1 | 17 | 1 |
| EOLG2.CMP (raw code file, load address unknown) | 1 | 0 | 0 |
| EOLG3.CMP (raw code file, load address unknown) | 1 | 0 | 0 |
| EOLG4.CMP (raw code file, load address unknown) | 1 | 0 | 0 |
| EOLG5.CMP (raw code file, load address unknown) | 1 | 0 | 0 |
| ISL1.BIN (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- BigNoseTheCaveman_v1.0/BigNoseTheCaveman_v1.0.hdf: hard-disk image (AmigaDOS OFS, 1024 blocks): its filesystem parses and all 18 files in it are byte-identical to the extracted tree BigNoseTheCaveman_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- BigNoseTheCaveman_v1.0/BigNoseTheCaveman_v1.0_hdf/BigNoseTheCaveman/data/VC: IMP: File Imploder stream found by ancient scan (74992 -> 180000 bytes) but only 0 of 67 sampled windows are in reference RAM: not used
- BigNoseTheCaveman_v1.0/BigNoseTheCaveman_v1.0_hdf/BigNoseTheCaveman/data/EOLG2.CMP: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BigNoseTheCaveman_v1.0/BigNoseTheCaveman_v1.0_hdf/BigNoseTheCaveman/data/EOLG3.CMP: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BigNoseTheCaveman_v1.0/BigNoseTheCaveman_v1.0_hdf/BigNoseTheCaveman/data/EOLG4.CMP: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BigNoseTheCaveman_v1.0/BigNoseTheCaveman_v1.0_hdf/BigNoseTheCaveman/data/EOLG5.CMP: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BigNoseTheCaveman_v1.0/BigNoseTheCaveman_v1.0_hdf/BigNoseTheCaveman/data/ISL1.BIN: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BigNoseTheCaveman_v1.0/BigNoseTheCaveman_v1.0_hdf/BigNoseTheCaveman/data/VC: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 5 never-loaded region(s), 75406 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_pea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_move_addq_rts`: Short code fragment that copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.

### VC

- `initialize_custom_registers`: Sets up custom hardware registers by enabling DMA and interrupts, clearing color registers, and preparing stack and global data structures.
