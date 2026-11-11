# Game 2304 (shinobi (1989)(sales curve)[cr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02304). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 27 of 27 named (0 author symbols), 27 of 27 described (1 by rule, 24 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 119 of 119 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 700 of 700 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 17952 |
| code | 440 |
| data | 863784 |
| unused | 18944 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Shinobi (1989)(Sales Curve)[cr].adf:bootblock | 1 | 31 | 3 |
| logged disk load from Shinobi (1989)(Sales Curve)[cr].adf boot-block load | 1 | 2 | 0 |
| logged disk load from Shinobi (1989)(Sales Curve)[cr].adf boot-block load | 1 | 0 | 0 |
| logged disk load from Shinobi (1989)(Sales Curve)[cr].adf boot-block load | 1 | 92 | 3 |
| Shinobi (1989)(Sales Curve)[cr].adf:blocks 33-282 (bytes 16896-144896)+22528#unpacked | 1 | 575 | 21 |

## Cases handled

- shinobi (1989)(sales curve)[cr]/Shinobi (1989)(Sales Curve)[cr].adf: custom-format (non-DOS) disk, 1720 non-empty blocks: matched against track-loaded segments
- 18 first-execution byte records contradict the logged opcode word (first an address): bad captures, the opcode word is used
- shinobi (1989)(sales curve)[cr]/Shinobi (1989)(Sales Curve)[cr].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- shinobi (1989)(sales curve)[cr]/Shinobi (1989)(Sales Curve)[cr].adf: the boot block reads 11776 bytes from disk offset 225280 to an address (found statically); recovered as a unit
- shinobi (1989)(sales curve)[cr]/Shinobi (1989)(Sales Curve)[cr].adf: the boot block reads 6656 bytes from disk offset 2048 to an address (found statically); recovered as a unit
- shinobi (1989)(sales curve)[cr]/Shinobi (1989)(Sales Curve)[cr].adf: 3 logged loads cover 19456 bytes of the image
- shinobi (1989)(sales curve)[cr]/Shinobi (1989)(Sales Curve)[cr].adf: blocks 18-18 (bytes 9216-9728) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.78)
- shinobi (1989)(sales curve)[cr]/Shinobi (1989)(Sales Curve)[cr].adf: blocks 20-23 (bytes 10240-12288) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.82)
- shinobi (1989)(sales curve)[cr]/Shinobi (1989)(Sales Curve)[cr].adf:blocks 33-282 (bytes 16896-144896)+22528#unpacked: hunk executable inside unpacked track data, recovered as unit u4 after the main pass
- shinobi (1989)(sales curve)[cr]/Shinobi (1989)(Sales Curve)[cr].adf: blocks 33-282 (bytes 16896-144896) hold 1 packed streams (Vice: Huffman compressor with RLE) found by scanning, 82% of the bytes
- shinobi (1989)(sales curve)[cr]/Shinobi (1989)(Sales Curve)[cr].adf: blocks 286-439 (bytes 146432-225280) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.70)
- shinobi (1989)(sales curve)[cr]/Shinobi (1989)(Sales Curve)[cr].adf: blocks 1314-1737 (bytes 672768-889856) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.86)
- shinobi (1989)(sales curve)[cr]/Shinobi (1989)(Sales Curve)[cr].adf: blocks 1746-1746 (bytes 893952-894464) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.74)
- shinobi (1989)(sales curve)[cr]/Shinobi (1989)(Sales Curve)[cr].adf: blocks 1749-1749 (bytes 895488-896000) outside the filesystem hold data, not code (opcode density 0.004, entropy 2.07)
- shinobi (1989)(sales curve)[cr]/Shinobi (1989)(Sales Curve)[cr].adf: blocks 1753-1753 (bytes 897536-898048) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- shinobi (1989)(sales curve)[cr]/Shinobi (1989)(Sales Curve)[cr].adf: blocks 1756-1759 (bytes 899072-901120) outside the filesystem hold data, not code (opcode density 0.024, entropy 3.43)
- 2 packed track region(s), 431616 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Shinobi (1989)(Sales Curve)[cr].adf:bootblock

- `initialize_boot_sequence`: Calls a routine to set the processor status, then performs two separate memory operations using a pointer in a1 to copy data blocks, and finally stores a pointer to another routine in a global variable.
- `copy_memory_block`: Sets the processor status register to a specific value, then copies a block of longwords from a source address to a destination address using a loop, and jumps to another routine.
- `configure_dma_and_audio`: Stores three values into a structure pointed to by a1, calls another routine using a pointer from a6, writes to the audio register, calls a routine at a fixed address, and writes to the audio register again before returning.

### logged disk load from Shinobi (1989)(Sales Curve)[cr].adf boot-block load

- `unpack_data_stream`: Initializes several values, then enters a complex loop that reads data from a source, performs bitwise operations and conditional branches, and writes results to a destination table before jumping to a location specified by a longword.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Shinobi (1989)(Sales Curve)[cr].adf:blocks 33-282 (bytes 16896-144896)+22528#unpacked

- `execute_boot_trap`: Sets several global variables, calls two routines, executes a trap instruction, and then enters an infinite loop after several calls to another routine.
- `copy_vblank_register`: Copies a longword from the video port register to a global variable and then performs an infinite loop.
- `setup_bonus_stage`: Writes values to a table, clears three longword blocks, sets several color registers to a constant value, and performs multiple loops involving bitwise inversion and calls to other routines.
- `adjust_color_palette`: Fills a table with a constant value, then enters a nested loop that compares two sets of words and adjusts the values in the first set based on bitwise differences before writing them to a destination table and clearing three other tables.
- `decode_and_write_values`: Uses a byte from a source to index into a table, performs a series of bitwise checks to modify a value, and writes the result to two different offsets in a destination structure within a loop.
- `populate_data_tables`: Stores several memory addresses into global variables, calls a routine to calculate a value, and if a comparison matches, fills two separate memory blocks with constant values before calling another routine.
- `clear_memory_block`: Clears a block of memory starting at the address in a0 one longword at a time, using a dbra loop counter, and returns.
- `setup_bitplane_parameters`: Writes several values to custom hardware registers, calls another routine, and then clears eight consecutive words starting at a memory location pointed to by a2.
- `copy_to_audio_registers`: Copies a block of 19 words from a source table to the audio hardware registers using a loop.
- `split_longwords_to_shorts`: Loops five times, reading a longword from a source and storing its high and low words into two separate 16-bit locations within a destination table.
- `split_longwords_to_shorts_short_block`: Loops twice, reading a longword from a source and storing its high and low words into two separate 16-bit locations within a destination table starting at a different offset.
- `set_audio_parameters`: Writes various longwords and words to the audio hardware registers, including a loop that writes a value to the DMA control register, and then sets a new set of parameters to the audio registers.
- `populate_data_table`: Performs a nested loop to calculate a value based on a table, stores it into a global structure, and then copies data from a source table into the destination structure using a loop.
- `execute_nested_loops`: Performs a nested loop that decrements two counters until they reach zero.
- `store_split_longwords`: Splits three longwords from a source table into high and low words and stores them into a destination table at offset six, then performs a loop that calls another routine fifteen times.
- `combine_table_bytes`: Calculates two indices from a value in d0, then runs two loops to combine bytes from a table into a destination buffer by performing a bitwise OR between one byte and a shifted version of another.
- `disable_interrupts_and_call`: Writes values to CIA interrupt registers and the interrupt enable register, modifies the status register, and calls another routine after storing a pointer in a global variable.
- `update_color_register`: Writes a 16-bit value from d0 to the color register at offset zero of the bitplane registers, increments d0, and then branches to another routine.
- `expand_compressed_tile_data`: Reads a byte from a source buffer to determine an index, then uses that index to fetch 8 bytes from a table and write them to three separate destination buffers at 40-byte intervals.
- `replace_chars_in_loop`: Checks if a byte at a specific memory location is the character thirty, and if so, replaces up to seven consecutive occurrences of that character with spaces.
- `configure_hardware_registers`: Writes several values to custom hardware registers and global variables, including setting specific bits in the interrupt enable register and clearing several bytes in memory.
