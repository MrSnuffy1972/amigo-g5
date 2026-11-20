# Game 3790 (gemstone legend (1990)(loriciel)[cr angels]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03790). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802240 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 26 of 26 named (0 author symbols), 26 of 26 described (7 by rule, 13 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 156884 bytes, 15 functions |
| Independent decoder cross-check (Ghidra against capstone) | 600 of 600 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 478 |
| data | 393250 |
| unused | 1408512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 1 of 2).adf:bootblock | 1 | 6 | 1 |
| Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 2 of 2).adf:bootblock | 1 | 123 | 8 |
| logged disk load from Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 1 of 2).adf boot-block load | 1 | 92 | 2 |
| Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 2 of 2).adf:blocks 656-881 (bytes 335872-451584)#unpacked (raw code file, load address unknown) | 1 | 379 | 15 |

## Cases handled

- gemstone legend (1990)(loriciel)[cr angels](disk 2 of 2)/Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 2 of 2).adf: custom-format (non-DOS) disk, 763 non-empty blocks: matched against track-loaded segments
- gemstone legend (1990)(loriciel)[cr angels](disk 1 of 2)/Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 1 of 2).adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- gemstone legend (1990)(loriciel)[cr angels](disk 1 of 2)/Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 1 of 2).adf: 1 logged loads cover 0 bytes of the image
- gemstone legend (1990)(loriciel)[cr angels](disk 2 of 2)/Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 2 of 2).adf: blocks 3-3 (bytes 1536-2048) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- gemstone legend (1990)(loriciel)[cr angels](disk 2 of 2)/Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 2 of 2).adf: blocks 16-54 (bytes 8192-28160) hold a BK: ByteKiller stream (19968 -> 26784 bytes)
- gemstone legend (1990)(loriciel)[cr angels](disk 2 of 2)/Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 2 of 2).adf: blocks 208-316 (bytes 106496-162304) hold a BK: ByteKiller stream (55808 -> 124652 bytes)
- gemstone legend (1990)(loriciel)[cr angels](disk 2 of 2)/Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 2 of 2).adf: blocks 320-360 (bytes 163840-184832) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.98)
- gemstone legend (1990)(loriciel)[cr angels](disk 2 of 2)/Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 2 of 2).adf: blocks 368-444 (bytes 188416-227840) hold a BK: ByteKiller stream (39424 -> 55502 bytes)
- gemstone legend (1990)(loriciel)[cr angels](disk 2 of 2)/Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 2 of 2).adf: blocks 464-487 (bytes 237568-249856) hold a BK: ByteKiller stream (12288 -> 40064 bytes)
- gemstone legend (1990)(loriciel)[cr angels](disk 2 of 2)/Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 2 of 2).adf: blocks 496-619 (bytes 253952-317440) hold a BK: ByteKiller stream (63488 -> 32032 bytes)
- gemstone legend (1990)(loriciel)[cr angels](disk 2 of 2)/Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 2 of 2).adf: blocks 624-644 (bytes 319488-330240) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.96)
- gemstone legend (1990)(loriciel)[cr angels](disk 2 of 2)/Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 2 of 2).adf: blocks 656-881 (bytes 335872-451584) hold a BK: ByteKiller stream (115712 -> 156884 bytes)
- gemstone legend (1990)(loriciel)[cr angels](disk 2 of 2)/Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 2 of 2).adf:blocks 656-881 (bytes 335872-451584)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- gemstone legend (1990)(loriciel)[cr angels](disk 2 of 2)/Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 2 of 2).adf: blocks 1536-1541 (bytes 786432-789504) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.77)
- gemstone legend (1990)(loriciel)[cr angels](disk 2 of 2)/Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 2 of 2).adf: blocks 1738-1759 (bytes 889856-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.76)
- 1 never-loaded region(s), 156884 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 2 packed track region(s), 37376 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 1 of 2).adf:bootblock

- `frag_move_jsr`: Short code fragment that copies 2 to the memory at an offset from a1, then copies an address to the memory at an offset from a1, then copies an address to the memory at an offset from a1, then copies 11264 to the memory at an offset from a1, then calls another routine (2 times in a row).

### Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 2 of 2).adf:bootblock

- `frag_bra_movem_lea_move_dbra_jmp`: Short code fragment that jumps to another point in the code, then saves registers on the stack, then loads the address of a location in the code into a0, then loads the address of a fixed memory address into a2, then copies 255 to d0, then copies the memory at a0 (advancing a0) to the memory at a2 (advancing a2), then loops back while counting down d0, then jumps to another point in the code.
- `setup_dma_and_decompress`: Configures DMA and custom hardware registers, performs data decompression using a bit-based algorithm, and initializes color palette entries to zero while waiting for a condition on a hardware port.
- `decompress_data`: Decompresses data using a bit-based algorithm that reads bits from input stream, handles different decompression modes based on bit values, copies data to output buffer, and manages bit buffer refill operations.
- `frag_moveq_bra`: Short code fragment that loads 8 into d1, then loads 8 into d4, then jumps to another point in the code.
- `read_compressed_data`: Reads compressed data using a bit reader routine with different bit counts based on comparison results, processes values into registers, and branches to different code paths depending on data conditions.
- `initialize_bit_reading`: Initializes a bit reading operation by setting up counters and calling a bit reader routine to process data from a buffer.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 1 of 2).adf boot-block load

- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Gemstone Legend (1990)(Loriciel)[cr Angels](Disk 2 of 2).adf:blocks 656-881 (bytes 335872-451584)#unpacked (raw code file, load address unknown)

- `frag_bsr_jmp`: Short code fragment that calls another routine (2 times in a row), then jumps to another point in the code.
- `initialize_game_state`: Sets up game data structures by loading addresses into pointers, calling processing routines for various components, clearing memory locations, and checking joystick status before proceeding with initialization.
- `process_block_loop`: Processes block data in two phases using a global table pointer and a counter, calling initialization and processing routines for each block entry.
- `initialize_block_data`: Reads block parameters from a pointer, initializes global variables with them, and performs a loop to set up multiple data entries based on a counter and offset values.
- `process_block_data`: Processes block coordinates and updates global variables with computed values, including handling conditional logic based on structure offsets and performing iterative operations with fixed increments.
- `initialize_graphics`: Configures graphics hardware registers including display window settings, bitplane configurations, color palette initialization, and coprocessor pointers while setting up DMA control for video output.
- `fill_color_data`: Writes pairs of 16-bit values to a memory area starting at address in a0, incrementing the pointer after each write.
- `frag_bsr_subq_bmi_btst_bne_rts`: Short code fragment that calls another routine (2 times in a row), then subtracts 1 from d7, then branches if the last result was negative, then tests bit 7 of a fixed memory address, then branches if the last result was not zero, then returns.
- `frag_clr_move_cmp_bne_rts`: Short code fragment that clears d0, then copies a fixed memory address to d0, then compares d0 with 248, then branches if the last result was not zero, then returns.
- `adjust_color_components`: Processes color data by comparing and adjusting component values in a loop that runs until no adjustments are made.
- `reduce_color_components`: Copies color data from one memory location to another, then processes the data by decrementing component values in a loop that continues until no more decrements are possible.
- `frag_move_bsr_dbra_rts`: Short code fragment that copies d7 to the stack, then copies 16 to d7, then calls another routine, then loops back while counting down d7, then copies the top of the stack to d7, then returns.
- `decode_pattern_data`: Decodes compressed pattern data using a bit-based decompression algorithm, writing decoded bytes to a destination buffer while managing bit reading and buffer refill operations.
- `bk_refill_bit_buffer_3`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_3`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
