# Game 4454 (MoonShineRacers v1.0 1540): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04454). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4084851 top-level bytes in 77 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 15 of 15 named (0 author symbols), 15 of 15 described (7 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 158 of 158 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 21176 |
| code | 596 |
| data | 4004711 |
| unused | 58368 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| Disk.1:bootblock | 1 | 156 | 14 |

## Cases handled

- MoonShineRacers_v1.0_1540/MoonShineRacers_v1.0_1540_hdf/MoonShineRacers/Disk.1: custom-format (non-DOS) disk, 1644 non-empty blocks: matched against track-loaded segments
- MoonShineRacers_v1.0_1540/MoonShineRacers_v1.0_1540_hdf/MoonShineRacers/Disk.2: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- MoonShineRacers_v1.0_1540/MoonShineRacers_v1.0_1540.hdf: hard-disk image (AmigaDOS OFS, 4384 blocks): its filesystem parses and all 9 files in it are byte-identical to the extracted tree MoonShineRacers_v1.0_1540_hdf, which is recovered file by file; no block outside the filesystem holds data
- MoonShineRacers_v1.0_1540/MoonShineRacers_v1.0_1540_hdf/MoonShineRacers/Disk.1: blocks 11-54 (bytes 5632-28160) outside the filesystem hold data, not code (opcode density 0.032, entropy 6.10)
- MoonShineRacers_v1.0_1540/MoonShineRacers_v1.0_1540_hdf/MoonShineRacers/Disk.1: blocks 57-674 (bytes 29184-345600) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.72)
- MoonShineRacers_v1.0_1540/MoonShineRacers_v1.0_1540_hdf/MoonShineRacers/Disk.1: blocks 684-685 (bytes 350208-351232) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.08)
- MoonShineRacers_v1.0_1540/MoonShineRacers_v1.0_1540_hdf/MoonShineRacers/Disk.1: blocks 692-695 (bytes 354304-356352) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.94)
- MoonShineRacers_v1.0_1540/MoonShineRacers_v1.0_1540_hdf/MoonShineRacers/Disk.1: blocks 699-699 (bytes 357888-358400) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- MoonShineRacers_v1.0_1540/MoonShineRacers_v1.0_1540_hdf/MoonShineRacers/Disk.1: blocks 701-701 (bytes 358912-359424) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.12)
- MoonShineRacers_v1.0_1540/MoonShineRacers_v1.0_1540_hdf/MoonShineRacers/Disk.1: blocks 704-706 (bytes 360448-361984) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.12)
- MoonShineRacers_v1.0_1540/MoonShineRacers_v1.0_1540_hdf/MoonShineRacers/Disk.1: blocks 709-1335 (bytes 363008-684032) outside the filesystem hold data, not code (opcode density 0.023, entropy 6.01)
- MoonShineRacers_v1.0_1540/MoonShineRacers_v1.0_1540_hdf/MoonShineRacers/Disk.1: blocks 1337-1680 (bytes 684544-860672) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.07)
- MoonShineRacers_v1.0_1540/MoonShineRacers_v1.0_1540_hdf/MoonShineRacers/Disk.2: blocks 2-1759 (bytes 1024-901120) hold 66 packed streams (C: Compact, F: Freeze/Melt 1.x, Ice: Pack-Ice v2.0 - v2.20) found by scanning, 88% of the bytes

## Cracktros (kept, not protection)

- none

## Copy protection found

- `setup_disk_dma_wait` (bootblock): Clears bit 2 of the global byte at a fixed address using the joystick fire state from offset 6 of the structure in a6, then configures the disk controller by writing the sync word, pointer and control registers and polls the interrupt request register with a countdown timeout before returning or writing the disk length register on failure. Kind: disk sync check. Pass path: The routine successfully waits for the disk interrupt (INT1) by polling CUSTOM_INTREQR and clearing it with CUSTOM_INTREQ = 2. If the check passes, the function returns normally after setting up DMA and waiting for the expected sync pattern.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### Disk.1:bootblock

- `frag_bra_lea_movea_move_dbra_jmp`: Short code fragment that jumps to another point in the code, then loads the address of a location in the code into a0, then copies an address to a1, then copies 255 to d7, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d7, then jumps to another point in the code.
- `initialize_hardware_and_loop`: Initializes several hardware registers and memory locations, performs a loop of memory clearing, updates joystick status bits, and manages a control loop with conditional calls to other routines.
- `frag_movea_move_cmpi_bne_jmp`: Short code fragment that copies an address to a5, then copies a5 to the memory at an offset from a6, then compares the memory at an offset from a6 with 5, then branches if the last result was not zero, then copies an address to a6, then jumps to the address in a6.
- `frag_bsr_clr_bra`: Short code fragment that calls another routine, then clears the memory at an offset from a6, then calls another routine, then jumps to another point in the code.
- `frag_bsr_addq_cmpi_bne_move_btst_bra`: Short code fragment that calls another routine, then adds 1 to the memory at an offset from a6, then compares the memory at an offset from a6 with 11, then branches if the last result was not zero, then adds 1 to the memory at an offset from a6, then copies the memory at an offset from a6 to d0, then tests bit 0 of d0, then branches if the last result was not zero, then jumps to another point in the code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `setup_disk_dma_wait`: Clears bit 2 of the global byte at a fixed address using the joystick fire state from offset 6 of the structure in a6, then configures the disk controller by writing the sync word, pointer and control registers and polls the interrupt request register with a countdown timeout before returning or writing the disk length register on failure.
- `check_signature`: Reads two 16-bit values from a memory location and compares them to a fixed value, branching to different locations based on whether the comparisons succeed.
- `frag_lea_bsr_ror_move_cmp_beq_bra`: Short code fragment that loads the address of the memory at an offset from a0 into a1, then calls another routine, then rotates d0 right by 8, then copies the memory at an offset from a6 to d1, then compares d0 with d1, then branches if the last result was zero, then loads the address of the memory at an offset from a0 into a0, then jumps to another point in the code.
- `frag_lea_movea_moveq_bsr_move_dbra_rts`: Short code fragment that loads the address of the memory at an offset from a0 into a0, then loads the address of the memory at an offset from a0 into a1, then copies the memory at an offset from a6 to a5, then loads 127 into d6, then calls another routine, then copies d0 to the memory at a5 (advancing a5), then loops back while counting down d6, then copies a5 to the memory at an offset from a6, then returns.
- `combine_masked_pointer_bits`: Returns the data register ANDed with the longword at the second pointer, ORed with the data register ANDed with the first pointer's longword shifted left by one bit.
- `update_joystick_status`: Modifies a byte in memory by incrementing it and writing it back, then calls another routine and writes a specific word value to a register.
- `write_word_bytes_then_poll`: Writes the low and high bytes of the data register word to two consecutive memory locations, then polls a status byte until its low bit is set.
- `mask_byte_and_store_value`: Keeps the top two bits of one memory byte, sets its bit 3, and writes the value 127 to a second memory byte.
