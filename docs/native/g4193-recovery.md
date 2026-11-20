# Game 4193 (KwikSnax v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04193). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (733728 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 8 of 8 named (0 author symbols), 8 of 8 described (3 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 616992 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 121 of 121 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 9905 |
| code | 300 |
| data | 723523 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| VC | 1 | 117 | 6 |
| AMIGA1.DAT#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| AMIGA2.DAT#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| MUSIC.DAT#unpacked (raw code file, load address unknown) | 1 | 2 | 1 |

## Cases handled

- KwikSnax_v1.0/KwikSnax_v1.0.hdf: hard-disk image (AmigaDOS OFS, 800 blocks): its filesystem parses and all 9 files in it are byte-identical to the extracted tree KwikSnax_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- KwikSnax_v1.0/KwikSnax_v1.0_hdf/KwikSnax/data/AMIGA1.DAT: PowerPacker Data (xfd)-packed, unpacked with ancient (86592 -> 200000 bytes)
- KwikSnax_v1.0/KwikSnax_v1.0_hdf/KwikSnax/data/AMIGA2.DAT: PowerPacker Data (xfd)-packed, unpacked with ancient (99856 -> 253152 bytes)
- KwikSnax_v1.0/KwikSnax_v1.0_hdf/KwikSnax/data/MUSIC.DAT: PowerPacker Data (xfd)-packed, unpacked with ancient (100668 -> 163840 bytes)
- KwikSnax_v1.0/KwikSnax_v1.0_hdf/KwikSnax/data/PIC.DAT: PowerPacker Data (xfd)-packed, unpacked with ancient (14552 -> 32066 bytes)
- KwikSnax_v1.0/KwikSnax_v1.0_hdf/KwikSnax/data/AMIGA1.DAT#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- KwikSnax_v1.0/KwikSnax_v1.0_hdf/KwikSnax/data/AMIGA2.DAT#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- KwikSnax_v1.0/KwikSnax_v1.0_hdf/KwikSnax/data/MUSIC.DAT#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 3 never-loaded region(s), 616992 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### VC

- `setup_stack_and_jump`: Adjusts the stack pointer to a fixed location, loads a program counter address into a register, stores that address at a specific memory location, and then executes a trap instruction.
- `initialize_hardware`: Sets up multiple hardware registers including DMA control, interrupt enable, and color settings.
- `decode_data_block`: Processes a data block by copying longwords and bytes according to bit patterns, then performs bit manipulation and memory operations including shifts, accumulations, and conditional writes based on computed values.
- `frag_lsr_beq_rts_move_roxr`: Short code fragment that shifts d5 right by d4, then branches if the last result was zero, then returns, then copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by d4, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `shift_and_accumulate`: Shifts data right while accumulating bits from the least significant position, using a loop counter and multiple data registers.

### MUSIC.DAT#unpacked (raw code file, load address unknown)

- `frag_move_jmp`: Short code fragment that copies the memory at a1 to the memory just below a0 (moving a0 down), then jumps to another point in the code.
