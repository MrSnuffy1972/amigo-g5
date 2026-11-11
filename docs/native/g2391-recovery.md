# Game 2391 (Mercenary v1.2 0955): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02391). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (833519 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 240 raw-word lines |
| Functions named and described | pass, 17 of 17 named (0 author symbols), 17 of 17 described (2 by rule, 15 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 95 of 95 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 62976 bytes, 4 functions |
| Independent decoder cross-check (Ghidra against capstone) | 302 of 610 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 15444 |
| code | 2408 |
| data | 745003 |
| padding | 8 |
| unused | 70656 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 209 | 12 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Disk.1 bytes 14848-46336 (code never loaded by a reference run) | 1 | 200 | 2 |
| Disk.1 bytes 193024-224512 (code never loaded by a reference run) | 1 | 200 | 2 |

## Cases handled

- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: raw disk image (362496 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Mercenary_v1.2_0955/Mercenary_v1.2_0955.hdf: hard-disk image (AmigaDOS OFS, 896 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree Mercenary_v1.2_0955_hdf, which is recovered file by file; no block outside the filesystem holds data
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 78 64 KB pages each with >= 29491 distinct PCs executed, 1277950 of its 1293645 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 78 64 KB pages each with >= 29491 distinct PCs executed, 1277950 of its 1293645 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 78 64 KB pages each with >= 29491 distinct PCs executed, 1277950 of its 1293645 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 78 64 KB pages each with >= 29491 distinct PCs executed, 1277950 of its 1293645 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- 1 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: bytes 14848-46336 look like code no run loaded: recovered position-independently (load address unknown)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: bytes 193024-224512 look like code no run loaded: recovered position-independently (load address unknown)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: 3 logged loads cover 69120 bytes of the image
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 24-44 (bytes 6144-11520) outside the filesystem hold data, not code (opcode density 0.006, entropy 4.70)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 48-50 (bytes 12288-13056) outside the filesystem hold data, not code (opcode density 0.029, entropy 3.94)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 198-335 (bytes 50688-86016) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.12)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 368-384 (bytes 94208-98560) outside the filesystem hold data, not code (opcode density 0.010, entropy 6.73)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 400-504 (bytes 102400-129280) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.36)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 528-542 (bytes 135168-139008) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.63)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 544-557 (bytes 139264-142848) outside the filesystem hold data, not code (opcode density 0.012, entropy 5.57)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 560-577 (bytes 143360-147968) outside the filesystem hold data, not code (opcode density 0.021, entropy 5.02)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 583-585 (bytes 149248-150016) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.65)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 588-588 (bytes 150528-150784) outside the filesystem hold data, not code (opcode density 0.008, entropy 0.22)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 592-598 (bytes 151552-153344) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 602-634 (bytes 154112-162560) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.19)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 640-667 (bytes 163840-171008) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.75)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 688-740 (bytes 176128-189696) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.76)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 744-746 (bytes 190464-191232) outside the filesystem hold data, not code (opcode density 0.029, entropy 3.95)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 894-1031 (bytes 228864-264192) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.12)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1064-1080 (bytes 272384-276736) outside the filesystem hold data, not code (opcode density 0.010, entropy 6.73)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1096-1200 (bytes 280576-307456) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.38)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1224-1238 (bytes 313344-317184) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.63)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1240-1253 (bytes 317440-321024) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.53)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1256-1273 (bytes 321536-326144) outside the filesystem hold data, not code (opcode density 0.021, entropy 5.02)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1279-1281 (bytes 327424-328192) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.65)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1284-1284 (bytes 328704-328960) outside the filesystem hold data, not code (opcode density 0.008, entropy 0.22)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1288-1294 (bytes 329728-331520) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1298-1330 (bytes 332288-340736) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.20)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1336-1363 (bytes 342016-349184) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.76)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1384-1415 (bytes 354304-362496) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.45)
- 2 never-loaded region(s), 62976 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `update_interrupt_mask_and_dma`: Disables interrupts via the certain hardware register and configures a DMA-based memory transfer.
- `execute_indirect_call`: Executes a function call using a pointer retrieved from a table.
- `modify_memory_via_offset`: writes a constant value to a memory location calculated by adding an offset to a register-based pointer.
- `set_global_value`: Sets a global variable to a constant value.
- `set_interrupt_register`: Sets a global variable to a fixed value and modifies the interrupt request register.
- `set_global_constant`: sets a global variable to a constant value.
- `check_status_and_call`: Checks a bit in a global variable and, if certain conditions involving a hardware register and a loop are met, calls a function via a pointer from a table.
- `toggle_bits_from_table`: Sets a global variable to zero, calls another routine, calls a function from a table, and then performs multiple bitwise XOR operations on a stack value and a pointer-based memory location.
- `toggle_bits_after_call`: Calls another routine, calls a function from a table, and then performs multiple bitwise XOR operations on a stack value and a pointer-based memory location.
- `update_ascii_char`: Increments a global variable, wraps it if it exceeds a certain value, and stores the result as an ASCII character in a global variable.
- `copy_and_configure_dma`: Copies data from a global source to a destination, performs a block-based memory copy, clears a hardware register, and configures a DMA-related register with a bit-shifted value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Disk.1 bytes 14848-46336 (code never loaded by a reference run)

- `store_address_to_ram`: stores a series of longwords or or even partial words from a source address into a range of memory located at a high address.
- `enter_infinite_loop`: executes an infinite loop that prevents further execution or traps the program flow.

### Disk.1 bytes 193024-224512 (code never loaded by a reference run)

- `populate_jump_table`: stores a sequence of instruction or jump target addresses into a table in high memory.
- `execute_infinite_loop`: The routine implements an infinite loop that repeatedly executes a no-operation or jump instruction without any side effects or state changes.
