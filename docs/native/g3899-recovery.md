# Game 3899 (HardNHeavy v1.0 1631): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03899). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1571362 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 5 of 5 named (0 author symbols), 5 of 5 described (2 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 26624 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 55 of 55 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10068 |
| code | 246 |
| data | 1453776 |
| padding | 8 |
| unused | 107264 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 55 | 5 |
| Disk.1 bytes 640000-666624 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: raw disk image (691200 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631.hdf: hard-disk image (AmigaDOS OFS, 1696 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree HardNHeavy_v1.0_1631_hdf, which is recovered file by file; no block outside the filesystem holds data
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: bytes 640000-666624 look like code no run loaded: recovered position-independently (load address unknown)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: 1 logged loads cover 26624 bytes of the image
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 0-694 (bytes 0-177920) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.90)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 699-784 (bytes 178944-200960) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.87)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 786-1131 (bytes 201216-289792) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.59)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 1134-2039 (bytes 290304-522240) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.07)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 2069-2082 (bytes 529664-533248) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.28)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 2100-2113 (bytes 537600-541184) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.44)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 2131-2144 (bytes 545536-549120) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.41)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 2162-2175 (bytes 553472-557056) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.35)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 2185-2185 (bytes 559360-559616) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 2187-2187 (bytes 559872-560128) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.07)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 2189-2189 (bytes 560384-560640) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.07)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 2191-2191 (bytes 560896-561152) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.07)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 2193-2193 (bytes 561408-561664) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.07)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 2195-2195 (bytes 561920-562176) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.07)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 2197-2197 (bytes 562432-562688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.07)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 2199-2199 (bytes 562944-563200) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.07)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 2605-2622 (bytes 666880-671488) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.85)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 2624-2634 (bytes 671744-674560) outside the filesystem hold data, not code (opcode density 0.048, entropy 5.43)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 2641-2680 (bytes 676096-686336) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.64)
- HardNHeavy_v1.0_1631/HardNHeavy_v1.0_1631_hdf/HardNHeavy/Disk.1: chunks 2689-2699 (bytes 688384-691200) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.32)
- 1 never-loaded region(s), 26624 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `setup_dma_and_jump`: Configures DMA settings by writing to a hardware control register, sets up multiple jump table entries with various routine addresses, and then jumps to a memory location pointed to by address 400.
- `setup_memory_map`: Writes specific 16-bit and 32-bit values to multiple memory locations, including a jump table entry at offset 5000, and returns immediately without modifying any registers.
- `frag_move_cmp_beq_dbra_rts`: Short code fragment that copies 12500 to d0, then copies d0 to the stack, then copies a fixed memory address to d0, then compares d0 with a fixed memory address, then branches if the last result was zero, then copies the top of the stack to d0, then loops back while counting down d0, then returns.
- `wait_for_input`: Reads from hardware input ports to check for joystick or mouse button presses, compares vertical position register values in a loop, and decrements a counter until it reaches zero or a button is pressed.
