# Game 1781 (cannon fodder 2 - new levels (19xx)(-)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01781). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 64 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 4997 raw-word lines |
| Functions named and described | pass, 17 of 17 named (0 author symbols), 17 of 17 described (2 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 130 of 130 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 174110 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5774 of 5774 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 392 |
| code | 1918 |
| data | 277493 |
| padding | 13061 |
| unused | 608256 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Cannon Fodder 2 - New Levels (19xx)(-).adf:SSBOOT | 1 | 529 | 14 |
| Cannon Fodder 2 - New Levels (19xx)(-).adf:bootblock | 1 | 18 | 1 |
| Cannon Fodder 2 - New Levels (19xx)(-).adf:cf2#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |
| Cannon Fodder 2 - New Levels (19xx)(-).adf:cf2.rel#unpacked (raw code file, load address unknown) | 1 | 5226 | 1 |

## Cases handled

- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:MAPM4.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (187 -> 300 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:MAPM2.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (191 -> 300 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:cf2: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (62453 -> 153206 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:MAPM3.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1130 -> 3516 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:PSTUFF.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11229 -> 40960 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:VMENU.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (16741 -> 51400 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:MAPM1.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (992 -> 3060 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:INTBASE.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (15248 -> 30720 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:INTCOPT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (25703 -> 53920 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:cf2.rel: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (13193 -> 20904 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:INTSUB0.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (584 -> 1280 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:INTBASE.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (114 -> 480 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:INTPLAY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (20493 -> 36000 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:INTSUB0.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (60 -> 320 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:INTARMY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22781 -> 53760 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:INTBASE.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (203 -> 480 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:MAPM3.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (192 -> 220 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:MAPM4.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (855 -> 3516 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:MAPM1.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (230 -> 300 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:INTBASE.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1114 -> 3095 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:INTSUB0.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (154 -> 320 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:MAPM2.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1079 -> 3060 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:INTBASE.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (877 -> 1920 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:INTBASE.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (34296 -> 41938 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:INTSUB0.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11497 -> 20352 bytes)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:cf2#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- cannon fodder 2 - new levels (19xx)(-)/Cannon Fodder 2 - New Levels (19xx)(-).adf:cf2.rel#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 153206 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Cannon Fodder 2 - New Levels (19xx)(-).adf:SSBOOT

- `load_relocation_file_and_run`: Program entry that copies a name from its input, appends a relocation-file extension, and calls library routines to load the named file.
- `scan_mask_patch_and_jump`: Scans a mask from its highest bit downward until it finds a set bit, adds a value derived from that bit to a list of words, copies a short routine to a fixed low address, and jumps to it.
- `copy_byte_block_then_dispatch`: Copies a counted block of bytes forward from a source to a destination, then makes an indirect call through a pointer the decompiler could not resolve.
- `copy_bytes_backward_then_dispatch`: Copies a counted block of bytes backward, from the end toward the start, then makes an indirect call through a pointer the decompiler could not resolve.
- `print_text_via_dos_output`: Opens a library through the system library table, measures a text string up to its terminator, and writes it out through the DOS output and write routines.
- `scan_memory_region_types`: Loops over 32 memory regions, probing up to 256 sample addresses in each with the operating system memory-type call.
- `open_and_validate_packed_file`: Opens a file through the DOS library, reads and seeks through it, allocates memory, and checks a header signature.
- `decode_compressed_chunk`: Checks a header signature, then decodes a compressed data stream by copying a block of bytes into a stack buffer and expanding run and copy sequences into the destination.
- `scan_table_for_bit_mask`: Scans a table of word pairs until a masked register value matches the second word of a pair, then builds a bit mask from a byte in that entry and a register value.
- `mask_and_conditional_call`: Masks one word with another and, if one byte register is less than a second byte register, calls another routine and returns its result instead of the masked value.
- `return_empty_routine`: The routine is empty; it contains no instructions that modify memory or registers and immediately returns.
- `accumulate_32bit_from_bytes`: Reads four consecutive bytes from a pointer and shifts each into a 32-bit result, which it returns.
- `build_huffman_code_table`: Reads a count of symbols and their code lengths into a stack buffer, then assigns codes for each length from one to sixteen.
- `empty_routine`: The routine contains no executable instructions.

### Cannon Fodder 2 - New Levels (19xx)(-).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Cannon Fodder 2 - New Levels (19xx)(-).adf:cf2#unpacked (raw code file, load address unknown)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### Cannon Fodder 2 - New Levels (19xx)(-).adf:cf2.rel#unpacked (raw code file, load address unknown)

- `stub_raw_code_bad_data`: The start of a raw code file of over five thousand instructions, but the decompiler produced only bad instruction data at this entry.
