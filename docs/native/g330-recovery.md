# Game 330 (savage (1988)(firebird)[cr acc][t +2 acc]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00330). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916624 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 23 of 23 named (0 author symbols), 23 of 23 described (1 by rule, 20 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1010 of 1010 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1381 of 1381 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10385 |
| code | 5532 |
| data | 872487 |
| padding | 60 |
| unused | 28160 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Savage (1988)(Firebird)[cr ACC][t +2 ACC].adf:bootblock | 1 | 38 | 1 |
| logged disk load from Savage (1988)(Firebird)[cr ACC][t +2 ACC].adf boot-block load | 1 | 76 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 73 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 83 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 975 | 14 |
| Savage (1988)(Firebird)[cr ACC][t +2 ACC].adf:blocks 22-475 (bytes 11264-243712)+268#unpacked | 1 | 68 | 1 |
| Savage (1988)(Firebird)[cr ACC][t +2 ACC].adf:blocks 484-794 (bytes 247808-407040)+268#unpacked | 1 | 68 | 1 |

## Cases handled

- savage (1988)(firebird)[cr acc][t +2 acc]/Savage (1988)(Firebird)[cr ACC][t +2 ACC].adf: custom-format (non-DOS) disk, 1703 non-empty blocks: matched against track-loaded segments
- savage (1988)(firebird)[cr acc][t +2 acc]/Savage (1988)(Firebird)[cr ACC][t +2 ACC].adf: the boot block reads 2560 bytes from disk offset 512 to an address (found statically); recovered as a unit
- runtime segment an address: 68 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 80 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 862 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 5 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 call target, 2 immediate, 1 immediate+address
- savage (1988)(firebird)[cr acc][t +2 acc]/Savage (1988)(Firebird)[cr ACC][t +2 ACC].adf: 1 logged loads cover 2560 bytes of the image
- savage (1988)(firebird)[cr acc][t +2 acc]/Savage (1988)(Firebird)[cr ACC][t +2 ACC].adf:blocks 22-475 (bytes 11264-243712)+268#unpacked: hunk executable inside unpacked track data, recovered as unit u5 after the main pass
- savage (1988)(firebird)[cr acc][t +2 acc]/Savage (1988)(Firebird)[cr ACC][t +2 ACC].adf: blocks 22-475 (bytes 11264-243712) hold 1 packed streams (BK: ByteKiller) found by scanning, 99% of the bytes
- savage (1988)(firebird)[cr acc][t +2 acc]/Savage (1988)(Firebird)[cr ACC][t +2 ACC].adf:blocks 484-794 (bytes 247808-407040)+268#unpacked: hunk executable inside unpacked track data, recovered as unit u6 after the main pass
- savage (1988)(firebird)[cr acc][t +2 acc]/Savage (1988)(Firebird)[cr ACC][t +2 ACC].adf: blocks 484-794 (bytes 247808-407040) hold 1 packed streams (BK: ByteKiller) found by scanning, 99% of the bytes
- savage (1988)(firebird)[cr acc][t +2 acc]/Savage (1988)(Firebird)[cr ACC][t +2 ACC].adf: blocks 1705-1726 (bytes 872960-884224) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.11)
- savage (1988)(firebird)[cr acc][t +2 acc]/Savage (1988)(Firebird)[cr ACC][t +2 ACC].adf: blocks 1738-1748 (bytes 889856-895488) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.84)
- savage (1988)(firebird)[cr acc][t +2 acc]/Savage (1988)(Firebird)[cr ACC][t +2 ACC].adf: blocks 1752-1752 (bytes 897024-897536) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- 1 packed track region(s), 460800 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Savage (1988)(Firebird)[cr ACC][t +2 ACC].adf:bootblock

- `load_disk_data`: Performs a disk read operation via a system call, executes the loaded code, and then performs a memory copy loop to transfer a block of data from one location to another.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_color_palette`: Reads bitstream data to manipulate color values and updates the first hardware color register.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decompress_data_stream_3`: Performs a custom decompression algorithm on a data stream by scanning for specific control bytes to perform variable-length copies and multi-byte expansions, then calls a trap routine.
- `trigger_trap`: Loads a long value into a register and stores it at a specific memory location, then executes a trap.
- `disable_interrupts_and_jump`: Disables interrupts by setting the status register and jumps to a long address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_audio`: Sets interrupt enable registers, fills a memory buffer with a sequence of bytes in a loop, calls another routine, configures several hardware registers, and calls another routine before entering an infinite loop.
- `handle_interrupt_and_update_state`: Checks the custom interrupt request register and returns if a specific bit is set, otherwise it clears an interrupt bit and performs a series of conditional checks on global variables to update counters, modify a table of values through a loop, or trigger further routines and coprocessor configurations.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (bchg.b, ori.b) and then falls through into the code that follows, with no return of its own.
- `iterate_blitter_source_tables`: Increments a global counter and, if it reaches one, iterates through a table of pointers to select a source address and dimensions, then calls a routine to perform blitter operations on multiple blocks.
- `configure_blt`: Waits for a DMA controller bit to clear, then sets several Blitter registers including mode, area, and size using values from registers and a table.
- `copy_data_blocks`: Performs two nested loops to copy long words from two source tables into two destination tables using specific block sizes and offsets.
- `setup_coprocessor`: Writes a long value to a coprocessor register, reads a word from another register, and writes a value to the DMA controller.
- `update_audio_parameters`: Performs a loop to calculate and store values into a table based on a source pointer, clears several hardware registers and memory locations, and writes specific values to hardware registers and memory.
- `reset_audio_and_dma`: Sets a bit in the interrupt control register, clears the four audio volume registers, sets the audio register at offset 96 to 15, and enables four DMA channels.
- `set_global_constant`: Calls another routine, reads a byte from a hardware register, and writes a constant value into a hardware register.
- `update_audio_volumes`: Increments a global variable and, if it reaches 6, calls another routine four times with different pointers in a6 and corresponding hardware register addresses in a5 to update audio volumes.
- `manage_audio_channels_2`: Checks a byte at offset 2 of a structure in a6 to determine an action, which may involve updating a value in a4, writing a byte to offset 6 of a structure in a5, writing a byte to offset 8 of a structure in a5, updating a global variable, or modifying the CIAA_PRA register.
- `manage_audio_channels`: Checks if a pointer in a6 matches specific addresses to return predefined values, otherwise it calculates an audio volume by reading a byte from a table, multiplying it by 1024, adding an offset, and calling another routine four times to update the four hardware audio channel registers.
- `update_structure`: Copies a long value from a table into a structure, performs calculations to update several fields within that structure, and calls another routine.

### Savage (1988)(Firebird)[cr ACC][t +2 ACC].adf:blocks 22-475 (bytes 11264-243712)+268#unpacked

- `decompress_data_stream`: Reads a source buffer and decompresses it into a destination buffer using a custom scheme where specific byte markers trigger length-based block copies or multi-level nested expansions.

### Savage (1988)(Firebird)[cr ACC][t +2 ACC].adf:blocks 484-794 (bytes 247808-407040)+268#unpacked

- `decompress_data_stream_2`: Reads a compressed data stream from a source buffer and writes the decompressed output to a destination buffer using two specific control codes.
