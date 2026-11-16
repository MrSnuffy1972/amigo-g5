# Game 693 (lure of the temptress (1992)(virgin)[cr csl]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00693). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3604480 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 30 of 30 named (0 author symbols), 30 of 30 described (17 by rule, 11 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3 of 3 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 471 of 471 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 22903 |
| code | 1582 |
| data | 3550811 |
| unused | 29184 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf:bootblock | 1 | 27 | 2 |
| logged disk load from Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf boot-block load | 1 | 350 | 26 |
| logged disk load from Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf boot-block load | 1 | 94 | 2 |
| logged disk load from Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf boot-block load | 1 | 0 | 0 |

## Cases handled

- lure of the temptress (1992)(virgin)[cr csl](disk 1 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf: custom-format (non-DOS) disk, 1743 non-empty blocks: matched against track-loaded segments
- lure of the temptress (1992)(virgin)[cr csl](disk 2 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 2 of 4).adf: custom-format (non-DOS) disk, 1735 non-empty blocks: matched against track-loaded segments
- lure of the temptress (1992)(virgin)[cr csl](disk 3 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 3 of 4).adf: custom-format (non-DOS) disk, 1749 non-empty blocks: matched against track-loaded segments
- lure of the temptress (1992)(virgin)[cr csl](disk 4 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 4 of 4).adf: custom-format (non-DOS) disk, 1748 non-empty blocks: matched against track-loaded segments
- lure of the temptress (1992)(virgin)[cr csl](disk 1 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf: the boot block reads 5632 bytes from disk offset 512 to an address (found statically); recovered as a unit
- lure of the temptress (1992)(virgin)[cr csl](disk 1 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf: the boot block reads 4096 bytes from disk offset 870912 to an address (found statically); recovered as a unit
- lure of the temptress (1992)(virgin)[cr csl](disk 1 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf: the boot block reads 16896 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- lure of the temptress (1992)(virgin)[cr csl](disk 1 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf: 3 logged loads cover 26624 bytes of the image
- lure of the temptress (1992)(virgin)[cr csl](disk 1 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf: blocks 12-14 (bytes 6144-7680) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.66)
- lure of the temptress (1992)(virgin)[cr csl](disk 1 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf: blocks 43-51 (bytes 22016-26624) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.25)
- lure of the temptress (1992)(virgin)[cr csl](disk 1 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf: blocks 57-57 (bytes 29184-29696) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.40)
- lure of the temptress (1992)(virgin)[cr csl](disk 1 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf: blocks 59-60 (bytes 30208-31232) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.82)
- lure of the temptress (1992)(virgin)[cr csl](disk 1 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf: blocks 63-64 (bytes 32256-33280) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.88)
- lure of the temptress (1992)(virgin)[cr csl](disk 1 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf: blocks 69-1700 (bytes 35328-870912) outside the filesystem hold data, not code (opcode density 0.013, entropy 7.01)
- lure of the temptress (1992)(virgin)[cr csl](disk 1 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf: blocks 1709-1715 (bytes 875008-878592) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.12)
- lure of the temptress (1992)(virgin)[cr csl](disk 1 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf: blocks 1749-1759 (bytes 895488-901120) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.11)
- lure of the temptress (1992)(virgin)[cr csl](disk 2 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 2 of 4).adf: blocks 10-10 (bytes 5120-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- lure of the temptress (1992)(virgin)[cr csl](disk 2 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 2 of 4).adf: blocks 12-28 (bytes 6144-14848) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.14)
- lure of the temptress (1992)(virgin)[cr csl](disk 2 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 2 of 4).adf: blocks 34-34 (bytes 17408-17920) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.40)
- lure of the temptress (1992)(virgin)[cr csl](disk 2 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 2 of 4).adf: blocks 36-37 (bytes 18432-19456) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.84)
- lure of the temptress (1992)(virgin)[cr csl](disk 2 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 2 of 4).adf: blocks 40-41 (bytes 20480-21504) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.88)
- lure of the temptress (1992)(virgin)[cr csl](disk 2 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 2 of 4).adf: blocks 1327-1523 (bytes 679424-780288) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.27)
- lure of the temptress (1992)(virgin)[cr csl](disk 2 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 2 of 4).adf: blocks 1525-1759 (bytes 780800-901120) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.19)
- lure of the temptress (1992)(virgin)[cr csl](disk 3 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 3 of 4).adf: blocks 8-8 (bytes 4096-4608) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.16)
- lure of the temptress (1992)(virgin)[cr csl](disk 3 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 3 of 4).adf: blocks 10-10 (bytes 5120-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- lure of the temptress (1992)(virgin)[cr csl](disk 3 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 3 of 4).adf: blocks 12-14 (bytes 6144-7680) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.29)
- lure of the temptress (1992)(virgin)[cr csl](disk 4 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 4 of 4).adf: blocks 10-10 (bytes 5120-5632) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.87)
- lure of the temptress (1992)(virgin)[cr csl](disk 4 of 4)/Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 4 of 4).adf: blocks 12-14 (bytes 6144-7680) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.44)
- 4 packed track region(s), 2454016 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf:bootblock

- `frag_bsr_move_jsr_lea_adda_jmp`: Short code fragment that calls another routine, then copies an address to the memory at an offset from a1, then copies 5632 to the memory at an offset from a1, then copies 512 to the memory at an offset from a1, then copies 2 to the memory at an offset from a1, then calls another routine, then loads the address of a fixed memory address into a0, then copies 66 to d0, then adds d0 to a0, then jumps to the address in a0.
- `initialize_display`: Sets up display registers with specific values, calls a library routine twice, clears a memory location, sets a word value at an offset, and calls the library routine again before returning.

### logged disk load from Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf boot-block load

- `frag_move_bra`: Short code fragment that copies 3840 to a fixed memory address, then copies 2048 to a fixed memory address, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies 0 to a fixed memory address, then copies 3840 to a fixed memory address, then jumps to another point in the code.
- `setup_graphics`: Configures multiple graphics hardware registers including DMA control, bitplane control, and color settings, initializes a table of values, sets up a coprocessor list address, and configures display timing registers.
- `frag_lea_moveq_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a1, then loads 15 into d1, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d1, then returns.
- `frag_movea_lea_jsr_tst_beq_move_rts`: Short code fragment that copies a fixed memory address to a6, then loads the address of a fixed memory address into a1, then calls another routine, then tests d0, then branches if the last result was zero, then copies an address to d0, then returns.
- `frag_lea_jsr_tst_beq_move_rts`: Short code fragment that loads the address of a fixed memory address into a1, then calls another routine, then tests d0, then branches if the last result was zero, then copies an address to d0, then returns.
- `frag_lea_jsr_tst_beq_move_rts_2`: Short code fragment that loads the address of a fixed memory address into a1, then calls another routine, then tests d0, then branches if the last result was zero, then copies an address to d0, then returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `rotate_data_blocks_2`: Processes data blocks by rotating bits in pairs of long and word values, repeating the operation 32 times for each block before moving to the next block.
- `frag_dbra_move_roxl_bcs_bra`: Short code fragment that loops back while counting down d5, then copies the memory at a1 (advancing a1) to d6, then copies 31 to d5, then rotates d6 left through the extend flag by 1, then branches if the carry flag is set, then copies an indexed entry from a2 to d4, then jumps to another point in the code.
- `rotate_data_blocks_4`: Processes data blocks by rotating bits in pairs of long and word values, repeating the operation 32 times for each block before moving to the next one.
- `frag_move_bra_3`: Short code fragment that copies an indexed entry from a2 to d4, then jumps to another point in the code.
- `frag_dbra_move_roxl_bcs_bra_2`: Short code fragment that loops back while counting down d5, then copies the memory at a1 (advancing a1) to d6, then copies 31 to d5, then rotates d6 left through the extend flag by 1, then branches if the carry flag is set, then copies an indexed entry from a2 to d4, then jumps to another point in the code.
- `compare_data_bytes`: Compares a byte from memory with a register value, and branches conditionally based on equality results at multiple offsets from a base address.
- `frag_clr_bsr_bra`: Short code fragment that clears d4, then calls another routine, then jumps to another point in the code.
- `frag_clr_bsr_bra_2`: Short code fragment that clears d4, then calls another routine, then jumps to another point in the code.
- `frag_clr_bsr_bra_3`: Short code fragment that clears d4, then calls another routine, then jumps to another point in the code.
- `rotate_data_blocks_3`: Processes data blocks by rotating bits in pairs of long and word values, repeating the operation 32 times for each block before moving to the next block.
- `frag_dbra_move_bra`: Short code fragment that loops back while counting down d3, then copies d0 to d3, then jumps to another point in the code.
- `rotate_data_loop`: Processes data through multiple iterations of loading values from memory, performing bit rotation operations on them, and repeating this sequence for a fixed number of times.
- `rotate_data_step`: Processes data by loading values from memory, performing bit rotation operations, and repeating this sequence for a fixed number of times before falling through to another routine.
- `frag_dbra_move_roxl`: Short code fragment that loops back while counting down d5, then copies the memory at a1 (advancing a1) to d6, then copies 31 to d5, then rotates d6 left through the extend flag by 1, then rotates d4 left through the extend flag by 1.
- `rotate_data_blocks`: Processes data blocks by rotating bits in pairs of long and word values, repeating the operation 32 times for each block before moving to the next.
- `process_color_data`: Copies data between memory locations in a loop, processes color values by masking and decrementing bits, and calls other routines multiple times while iterating through the data.
- `fade_colors`: Processes color data by loading values from memory, performing bit operations on them, and writing results to graphics registers in a loop that decrements a counter until it reaches zero.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.

### logged disk load from Lure of the Temptress (1992)(Virgin)[cr CSL](Disk 1 of 4).adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
