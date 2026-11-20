# Game 3445 (DragonScape v1.1): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03445). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1410224 top-level bytes in 56 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 15 of 15 named (0 author symbols), 15 of 15 described (2 by rule, 10 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 67338 bytes, 9 functions |
| Independent decoder cross-check (Ghidra against capstone) | 698 of 698 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 479955 |
| code | 2224 |
| data | 928040 |
| padding | 5 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| Dragonscape | 1 | 94 | 5 |
| Scape (raw code file, load address unknown) | 1 | 602 | 9 |
| tile3.dat (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- DragonScape_v1.1/DragonScape_v1.1.hdf: hard-disk image (AmigaDOS OFS, 1632 blocks): its filesystem parses and all 54 files in it are byte-identical to the extracted tree DragonScape_v1.1_hdf, which is recovered file by file; no block outside the filesystem holds data
- DragonScape_v1.1/DragonScape_v1.1_hdf/DragonScape/data/Scape: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- DragonScape_v1.1/DragonScape_v1.1_hdf/DragonScape/data/tile3.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 2 never-loaded region(s), 67338 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### Dragonscape

- `initialize_system`: Initializes system hardware by setting interrupt and DMA control registers to enable interrupts and DMA channels, then executes a trap instruction.
- `decode_bits`: Reads bits from a compressed data stream using a bit buffer and refills the buffer when needed, handling the packed data format for decompression.
- `write_decrementing_global_until_wrap`: Writes a 16-bit countdown into a global variable on each pass, looping until the counter wraps back to its starting value, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Scape (raw code file, load address unknown)

- `parse_form_header`: Parses a FORM header structure, reads data fields, calculates values, and processes chunks of data while setting up memory locations for further processing.
- `process_compressed_data`: Processes compressed data by reading values from a table, updating counters, and calling other routines to handle audio parameters and playback control.
- `clear_audio_registers`: Clears audio volume registers for all four audio channels and resets several global variables to zero.
- `frag_bsr_clr_rts`: Short code fragment that calls another routine, then clears the memory at an offset from a4 (2 times in a row), then returns.
- `copy_and_shift_data`: Copies data from one location to another while shifting bits, updates a counter, and sets a termination marker after processing eight bytes.
- `copy_byte_values`: Copies byte values from a table using an index offset to various memory locations, and calls another routine when a condition is met.
- `load_audio_parameters`: Loads audio parameters from a table based on an index, calculates a multiplier, and writes values to hardware registers for audio playback control.
- `copy_word_values`: Copies word-sized values from a table using an index offset and stores them at two different memory locations.
- `setup_audio_dma`: Configures audio period registers for all four channels, modifies DMA control settings, and sets volume registers for audio output.
