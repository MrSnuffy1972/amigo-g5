# Game 3476 (DungeonsOfAvalon v1.0 1956): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03476). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1683804 top-level bytes in 55 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 23 of 23 named (0 author symbols), 23 of 23 described (8 by rule, 11 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 515 of 515 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 243544 |
| code | 1240 |
| data | 1439020 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 49 | 7 |
| AVALON_E | 2 | 233 | 8 |
| ZERET_INTRO | 2 | 233 | 8 |

## Cases handled

- DungeonsOfAvalon_v1.0_1956/DungeonsOfAvalon_v1.0_1956.hdf: hard-disk image (AmigaDOS OFS, 1856 blocks): its filesystem parses and all 43 files in it are byte-identical to the extracted tree DungeonsOfAvalon_v1.0_1956_hdf, which is recovered file by file; no block outside the filesystem holds data
- DungeonsOfAvalon_v1.0_1956/DungeonsOfAvalon_v1.0_1956_hdf/DungeonsOfAvalon/data/DAT/M1: PowerPacker Data (xfd)-packed, unpacked with ancient (3612 -> 8960 bytes)
- DungeonsOfAvalon_v1.0_1956/DungeonsOfAvalon_v1.0_1956_hdf/DungeonsOfAvalon/data/DAT/M2: PowerPacker Data (xfd)-packed, unpacked with ancient (1988 -> 4972 bytes)
- DungeonsOfAvalon_v1.0_1956/DungeonsOfAvalon_v1.0_1956_hdf/DungeonsOfAvalon/data/DAT/PICS: high-entropy data file (7.68 bits/byte), classified as packed data
- DungeonsOfAvalon_v1.0_1956/DungeonsOfAvalon_v1.0_1956_hdf/DungeonsOfAvalon/data/DAT/S1: PowerPacker Data (xfd)-packed, unpacked with ancient (50068 -> 55982 bytes)
- DungeonsOfAvalon_v1.0_1956/DungeonsOfAvalon_v1.0_1956_hdf/DungeonsOfAvalon/data/DAT/S2: PowerPacker Data (xfd)-packed, unpacked with ancient (63204 -> 67942 bytes)
- DungeonsOfAvalon_v1.0_1956/DungeonsOfAvalon_v1.0_1956_hdf/DungeonsOfAvalon/data/S/DGFX: PowerPacker Data (xfd)-packed, unpacked with ancient (32856 -> 81984 bytes)
- DungeonsOfAvalon_v1.0_1956/DungeonsOfAvalon_v1.0_1956_hdf/DungeonsOfAvalon/data/S/ENDGFX: PowerPacker Data (xfd)-packed, unpacked with ancient (10072 -> 40064 bytes)
- DungeonsOfAvalon_v1.0_1956/DungeonsOfAvalon_v1.0_1956_hdf/DungeonsOfAvalon/data/S/MENU: PowerPacker Data (xfd)-packed, unpacked with ancient (7480 -> 40064 bytes)
- DungeonsOfAvalon_v1.0_1956/DungeonsOfAvalon_v1.0_1956_hdf/DungeonsOfAvalon/data/S/PART_D: PowerPacker Data (xfd)-packed, unpacked with ancient (5956 -> 20410 bytes)
- DungeonsOfAvalon_v1.0_1956/DungeonsOfAvalon_v1.0_1956_hdf/DungeonsOfAvalon/data/S/PART_E: PowerPacker Data (xfd)-packed, unpacked with ancient (5964 -> 20410 bytes)
- DungeonsOfAvalon_v1.0_1956/DungeonsOfAvalon_v1.0_1956_hdf/DungeonsOfAvalon/data/S/TITEL: PowerPacker Data (xfd)-packed, unpacked with ancient (30736 -> 80064 bytes)
- DungeonsOfAvalon_v1.0_1956/DungeonsOfAvalon_v1.0_1956_hdf/DungeonsOfAvalon/data/AVALON_E: PowerPacker 3.0 (xfd) stream found by ancient scan (62352 -> 121148 bytes) but only 0 of 46 sampled windows are in reference RAM: not used
- DungeonsOfAvalon_v1.0_1956/DungeonsOfAvalon_v1.0_1956_hdf/DungeonsOfAvalon/data/ZERET_INTRO: PowerPacker 3.0 (xfd) stream found by ancient scan (4392 -> 8216 bytes) but only 0 of 4 sampled windows are in reference RAM: not used
- DungeonsOfAvalon_v1.0_1956/DungeonsOfAvalon_v1.0_1956_hdf/DungeonsOfAvalon/data/AVALON_E: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- ZERET_INTRO

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_cmpa_beq_jmp`: Short code fragment that compares a4 with a location in the code, then branches if the last result was zero, then jumps to another point in the code.
- `frag_lea_bsr_jmp`: Short code fragment that loads the address of the memory at an offset from a5 into a6, then calls another routine, then jumps to the address in a6.
- `install_pc_relative_pointer_into_global`: Loads a pc-relative address into a3, stores that address into memory at a3 minus offset, then loads another pc-relative address into a3 and moves it into d1 before returning.
- `frag_move_addi_rts`: Short code fragment that copies a location in the code to the stack, then adds 32 to the top of the stack, then returns.
- `process_game_logic`: Executes a game logic routine that checks for a zero condition, calls another routine if not zero, and performs multiple memory operations including shifts and jumps based on calculated values.
- `frag_move_pea_addq_rts`: Short code fragment that copies a3 to the stack, then pushes the address of a fixed memory address onto the stack (2 times in a row), then copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.

### AVALON_E

- `load_and_decode_data`: Processes and loads decoded data blocks using memory allocation and copying operations, handling different block types through conditional logic and loop structures.
- `decrement_and_loop`: Decrements a word register in a loop until it reaches zero, then returns immediately without performing any other operations.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decode_and_copy_data`: Decodes data by performing shifts and arithmetic operations, then copies bytes between memory locations using a loop with decrementing counters.
- `frag_lsr_beq_rts_move_roxr`: Short code fragment that shifts d5 right by d4, then branches if the last result was zero, then returns, then copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by d4, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `shift_and_process_bits`: Processes bits through multiple shift operations and conditional branches, updating registers and memory locations based on the results of these operations.
- `read_and_trap`: Reads a 16-bit value from a global variable and executes a trap instruction, followed by a bitwise OR operation between two data registers.

### ZERET_INTRO

- `load_and_relocate_segments`: Processes executable segments by allocating memory for each, copying data into the allocated blocks, and updating relocation information.
- `decrement_and_loop_2`: Decrements a word register in a loop until it reaches zero, performing no other operations during the loop iterations.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `decode_and_copy_data_2`: Decodes data by performing shifts and arithmetic operations, then copies bytes between memory locations using a loop with decrementing counters.
- `frag_lsr_beq_rts_move_roxr_2`: Short code fragment that shifts d5 right by d4, then branches if the last result was zero, then returns, then copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by d4, then returns.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `shift_and_process_bits_2`: Processes bits through multiple shift operations and conditional branches, updating registers and memory locations based on the results of the shifts.
- `read_and_trap_2`: Reads a word from a global variable at offset 2313 in the structure pointed to by a1, executes a trap instruction, and performs a bitwise OR operation between d2 and d0.
