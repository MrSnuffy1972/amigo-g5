# Game 3602 (FantasyFlyer v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03602). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4851858 top-level bytes in 149 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 20 of 20 named (0 author symbols), 20 of 20 described (9 by rule, 10 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 423124 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 490 of 491 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 776478 |
| code | 1978 |
| data | 4073402 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 83 | 9 |
| LOADER | 1 | 401 | 10 |
| @PART-II_1200 (raw code file, load address unknown) | 1 | 0 | 0 |
| @PART-II_500 (raw code file, load address unknown) | 1 | 0 | 0 |
| @PART-I_1200 (raw code file, load address unknown) | 1 | 0 | 0 |
| @PART-I_500 (raw code file, load address unknown) | 1 | 0 | 0 |
| @BLUE3.1 (raw code file, load address unknown) | 1 | 0 | 0 |
| @BLUE3.2 (raw code file, load address unknown) | 1 | 0 | 0 |
| @BLUE3.4 (raw code file, load address unknown) | 1 | 0 | 0 |
| @BLUE3.5 (raw code file, load address unknown) | 1 | 0 | 0 |
| @M39-THR (raw code file, load address unknown) | 1 | 0 | 0 |
| @M4Z-EX4 (raw code file, load address unknown) | 1 | 0 | 0 |
| @Menu (raw code file, load address unknown) | 1 | 7 | 1 |

## Cases handled

- FantasyFlyer_v1.0/FantasyFlyer_v1.0.hdf: hard-disk image (AmigaDOS OFS, 5504 blocks): its filesystem parses and all 147 files in it are byte-identical to the extracted tree FantasyFlyer_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/General/@M04-kah: high-entropy data file (7.63 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL1/@M10-ENT: high-entropy data file (7.71 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL1/@M11-GRI: high-entropy data file (7.69 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL1/@M13-TRA: high-entropy data file (7.83 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL1/@M15-GUA: high-entropy data file (7.67 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL1/@M18-GHO: high-entropy data file (7.62 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL1/@M1A-ROU: high-entropy data file (7.70 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL1/@M1F-TH1: high-entropy data file (7.70 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL1/@M1W-SP1: high-entropy data file (7.72 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL1/@M1Y-FP1: high-entropy data file (7.66 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL1/@M1Z-EX1: high-entropy data file (7.74 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL2/@BLUE2.1: high-entropy data file (7.66 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL2/@BLUE2.5: high-entropy data file (7.61 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL2/@M20-UND: high-entropy data file (7.82 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL2/@M22-BEA: high-entropy data file (7.67 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL2/@M27-MED: high-entropy data file (7.64 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL2/@M2F-TH2: high-entropy data file (7.77 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL2/@M2W-SP2: high-entropy data file (7.84 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL2/@M2Y-FP2: high-entropy data file (7.65 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL2/@M2Z-EX2: high-entropy data file (7.80 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL3/@M30-PAL: high-entropy data file (7.62 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL3/@M32-TUR: high-entropy data file (7.67 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL3/@M37-WAT: high-entropy data file (7.62 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL3/@M39-THR: high-entropy data file (7.66 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL3/@M3A-BIG: high-entropy data file (7.61 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL3/@M3B-HBE: high-entropy data file (7.73 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL3/@M3F-TH3: high-entropy data file (7.70 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL4/@M42-CRU: high-entropy data file (7.61 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL4/@M47-SIL: high-entropy data file (7.62 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL4/@M4F-TH4: high-entropy data file (7.72 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL4/@M4W-SP4: high-entropy data file (7.75 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL4/@M4Z-EX4: high-entropy data file (7.60 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL5/@M5B-MUL: high-entropy data file (7.61 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL5/@M5F-TH5: high-entropy data file (7.65 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL5/@M5G-TH6: high-entropy data file (7.71 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL5/@M5Z-EX5: high-entropy data file (7.61 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/SEQS/@E2-TH: high-entropy data file (7.64 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/SEQS/@I1-FL: high-entropy data file (7.61 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/SEQS/@I3-sc: high-entropy data file (7.82 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/SEQS/@I5-TH: high-entropy data file (7.72 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/SEQS/@S2-TH: high-entropy data file (7.64 bits/byte), classified as packed data
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/Codes/@PART-II_1200: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/Codes/@PART-II_500: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/Codes/@PART-I_1200: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/Codes/@PART-I_500: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL3/@BLUE3.1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL3/@BLUE3.2: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL3/@BLUE3.4: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL3/@BLUE3.5: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL3/@M39-THR: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/LEVEL4/@M4Z-EX4: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- FantasyFlyer_v1.0/FantasyFlyer_v1.0_hdf/FantasyFlyer/data/Menu/@Menu: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 11 never-loaded region(s), 423124 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_cmpa_beq_jmp`: Short code fragment that compares a4 with a location in the code, then branches if the last result was zero, then jumps to another point in the code.
- `frag_lea_movem_move_tst_beq_bsr_jmp`: Short code fragment that loads the address of the memory at an offset from a5 into a6, then saves registers on the stack, then copies the memory at a1 to d0, then tests d1, then branches if the last result was zero, then calls another routine, then restores saved registers from the stack, then calls another routine, then jumps to the address in a6.
- `frag_lea_move_rts`: Short code fragment that loads the address of a location in the code into a3, then copies a3 to the memory at an offset from a3, then loads the address of a location in the code into a3, then copies a3 to d1, then returns.
- `frag_move_addi_rts`: Short code fragment that copies a location in the code to the stack, then adds 32 to the top of the stack, then returns.
- `string_parse_until_delimiter`: Shifts a value left by two bits, uses it as an address offset, reads characters from memory, and continues parsing until a delimiter or end condition is met.
- `frag_movea_move_bra`: Short code fragment that copies a1 to a0, then copies d2 to d0, then jumps to another point in the code.
- `parse_string_match`: Parses a string from memory, converts lowercase letters to uppercase, and compares each character with a reference string.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### LOADER

- `frag_jmp`: Short code fragment that jumps to another point in the code.
- `copy_and_process_data_2`: Copies data from a source buffer to a destination, processes bits based on a counter, and writes the results to another location.
- `copy_and_process_data`: Copies data from a source buffer to a destination, processes bits based on a counter, and writes the results to another location.
- `increment_and_check`: Increments a word value and performs a bitwise AND operation with 7, returning immediately if the result is not zero.
- `file_operation`: Opens a file for reading or writing based on a condition, reads or writes data to a buffer, and then closes the file handle.
- `initialize_game`: Initializes game resources by opening libraries, setting up hardware registers, and configuring audio and display parameters.
- `process_hunk_relocation`: Processes hunk relocation entries by calculating addresses, performing memory operations, and updating pointers within a data structure.
- `memory_availability_check`: Checks available memory and hardware capabilities, then sets flags in a global variable based on comparison results with predefined thresholds.
- `frag_clr_move_lsl_rts`: Short code fragment that clears d7, then copies a fixed memory address to d7, then shifts d7 left by 8, then copies a fixed memory address to d7, then shifts d7 left by 8, then copies a fixed memory address to d7, then returns.
- `memory_allocation`: Allocates memory blocks for a data structure, updates pointers, and handles cleanup by freeing previously allocated memory if allocation fails.

### @Menu (raw code file, load address unknown)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
