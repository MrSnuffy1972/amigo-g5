# Game 4269 (LittlePuffInDragonland v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04269). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1305834 top-level bytes in 34 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 5 of 5 named (0 author symbols), 5 of 5 described (2 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 103 of 103 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 567078 |
| code | 284 |
| data | 738468 |
| padding | 4 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| PUFF | 1 | 101 | 4 |

## Cases handled

- LittlePuffInDragonland_v1.0/LittlePuffInDragonland_v1.0.hdf: hard-disk image (AmigaDOS OFS, 1440 blocks): its filesystem parses and all 32 files in it are byte-identical to the extracted tree LittlePuffInDragonland_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- LittlePuffInDragonland_v1.0/LittlePuffInDragonland_v1.0_hdf/LittlePuffInDragonland/data/PUFF: Vice: Huffman compressor with RLE stream found by ancient scan (28672 -> 47580 bytes) but only 0 of 17 sampled windows are in reference RAM: not used
- LittlePuffInDragonland_v1.0/LittlePuffInDragonland_v1.0_hdf/LittlePuffInDragonland/data/PUFF: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_hardware` (PUFF): Sets up hardware registers including CIAB_PRB, COLOR00, DMACON, and INTENA with specific bit patterns, configures stack pointer, and initializes memory locations at fixed addresses. Kind: disk hardware check. Pass path: CIAB_PRB read value matches expected hardware signature.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### PUFF

- `initialize_hardware`: Sets up hardware registers including CIAB_PRB, COLOR00, DMACON, and INTENA with specific bit patterns, configures stack pointer, and initializes memory locations at fixed addresses.
- `frag_lea_bsr_jmp`: Short code fragment that loads the address of a location in the code into a6, then loads the address of a fixed memory address into a3, then calls another routine, then jumps to another point in the code.
- `initialize_data_table`: Initializes a data table by copying 256 words from one memory location to another.
- `decode_table_entry`: Processes a table entry by shifting bits from a data pointer, accessing tables based on shifted values, and handling conditional branches based on sign flags of retrieved words before returning.
