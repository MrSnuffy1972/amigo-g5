# Game 4088 (Joe&MacCavemanNinja v1.0b 2179): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04088). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4373677 top-level bytes in 28 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 12 of 12 named (0 author symbols), 12 of 12 described (6 by rule, 6 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 768 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 150 of 150 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 7413 |
| code | 594 |
| data | 4364900 |
| padding | 2 |
| unused | 768 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 150 | 12 |
| Disk.1 bytes 0-768 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- Joe&MacCavemanNinja_v1.0b_2179/Joe&MacCavemanNinja_v1.0b_2179_hdf/Joe&MacCavemanNinja/Disk.1: raw disk image (983424 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Joe&MacCavemanNinja_v1.0b_2179/Joe&MacCavemanNinja_v1.0b_2179_hdf/Joe&MacCavemanNinja/Disk.2: raw disk image (970816 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Joe&MacCavemanNinja_v1.0b_2179/Joe&MacCavemanNinja_v1.0b_2179.hdf: hard-disk image (AmigaDOS OFS, 4704 blocks): its filesystem parses and all 7 files in it are byte-identical to the extracted tree Joe&MacCavemanNinja_v1.0b_2179_hdf, which is recovered file by file; no block outside the filesystem holds data
- Joe&MacCavemanNinja_v1.0b_2179/Joe&MacCavemanNinja_v1.0b_2179_hdf/Joe&MacCavemanNinja/Disk.1: bytes 0-768 look like code no run loaded: recovered position-independently (load address unknown)
- Joe&MacCavemanNinja_v1.0b_2179/Joe&MacCavemanNinja_v1.0b_2179_hdf/Joe&MacCavemanNinja/Disk.1: 1 logged loads cover 768 bytes of the image
- Joe&MacCavemanNinja_v1.0b_2179/Joe&MacCavemanNinja_v1.0b_2179_hdf/Joe&MacCavemanNinja/Disk.1: chunks 4-21 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.19)
- Joe&MacCavemanNinja_v1.0b_2179/Joe&MacCavemanNinja_v1.0b_2179_hdf/Joe&MacCavemanNinja/Disk.1: chunks 24-3841 (bytes 6144-983424) hold 9 packed streams (C: Compact, RNC1: Rob Northen RNC1 Compressor) found by scanning, 96% of the bytes
- Joe&MacCavemanNinja_v1.0b_2179/Joe&MacCavemanNinja_v1.0b_2179_hdf/Joe&MacCavemanNinja/Disk.2: chunks 0-3792 (bytes 0-970816) hold 10 packed streams (C: Compact, RNC1: Rob Northen RNC1 Compressor) found by scanning, 95% of the bytes
- 1 never-loaded region(s), 768 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_lea_move_bra`: Short code fragment that loads the address of a location in the code into a1, then copies a0 to the memory at a1, then jumps to another point in the code.
- `process_disk_request`: Loads several values into registers, performs conditional checks, and calls another routine with parameters after setting up memory addresses.
- `initialize_system`: Restores saved registers, negates a value, sets up interrupt vectors for two different routines, and then jumps to another address.
- `setup_interrupt_vectors`: Configures two interrupt vector entries by loading a routine address and storing it in a memory location, then jumps to another address.
- `check_disk_status`: Tests a global variable for zero value and branches if not zero, otherwise sets a register to one and performs multiplication operations before calling another routine.
- `frag_movem_not_bsr_clr_rts`: Short code fragment that restores saved registers from the stack, then inverts d0, then calls another routine, then restores saved registers from the stack, then clears d0, then returns.
- `frag_lea_cmpi_bne_move_rts`: Short code fragment that loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1, then compares the memory at a1 with 4153, then branches if the last result was not zero, then copies 20153 to the memory at a1 (advancing a1), then copies a0 to the memory at a1, then returns.
- `frag_move_movem_not_ror_cmp_bne_pea_add`: Short code fragment that copies a fixed memory address to d0, then saves registers on the stack, then inverts d0, then rotates d0 right by 1, then compares d0 with a location in the code, then branches if the last result was not zero, then pushes the address of a fixed memory address onto the stack, then copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.
- `frag_cmpi_bne_move_movem_rts`: Short code fragment that compares d0 with 88, then branches if the last result was not zero, then copies 240 to a fixed memory address, then compares d0 with 95, then branches if the last result was not zero, then copies 3840 to a fixed memory address, then copies 2319 to a fixed memory address, then restores saved registers from the stack, then returns.
- `swap_and_execute`: Exchanges two memory addresses, then invokes a function through a pointer in a global variable, followed by calling another routine with a fixed address.
- `save_game_data`: Calls a routine first, then performs address swapping and calls another routine with specific parameters.
