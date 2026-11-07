# Game 1782 (cannon fodder 2 - not very festive (1995)(sensible software)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01782). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 83 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 5012 raw-word lines |
| Functions named and described | pass, 17 of 17 named (0 author symbols), 17 of 17 described (2 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 130 of 130 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 174158 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5786 of 5786 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 701 |
| code | 1918 |
| data | 388859 |
| padding | 15050 |
| unused | 494592 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:SSBOOT | 1 | 529 | 14 |
| Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:bootblock | 1 | 18 | 1 |
| Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:cf2#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |
| Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:cf2.rel#unpacked (raw code file, load address unknown) | 1 | 5238 | 1 |

## Cases handled

- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:MAPM4.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (190 -> 300 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:DESBASE.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (576 -> 1920 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:MAPM2.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (195 -> 300 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:DESSUB0.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (386 -> 1280 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:ICESUB0.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (89 -> 320 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:DESBASE.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (111 -> 480 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:cf2: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (62447 -> 153206 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:MAPM3.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1127 -> 4542 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:PSTUFF.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11229 -> 40960 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:ICEBASE.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (542 -> 1920 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:VMENU.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (17937 -> 51400 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:MAPM1.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (514 -> 1236 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:DESARMY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22133 -> 53760 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:DESBASE.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (153 -> 480 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:ICESUB0.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (278 -> 1280 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:ICEBASE.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (79 -> 480 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:cf2.rel: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (13227 -> 20952 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:DESBASE.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1145 -> 3145 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:ICEARMY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22872 -> 53760 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:ICEBASE.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (118 -> 480 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:DESBASE.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (30737 -> 36694 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:ICEBASE.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1203 -> 3207 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:MAPM4.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (931 -> 3060 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:DESBASE.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (21226 -> 30720 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:DESCOPT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (31882 -> 53920 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:MAPM1.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (177 -> 280 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:MAPM2.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (868 -> 3060 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:ICEBASE.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (38322 -> 44704 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:DESSUB0.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (13121 -> 20352 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:DESSUB0.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (40 -> 320 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:ICEBASE.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (13511 -> 30720 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:ICECOPT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (25932 -> 53920 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:ICESUB0.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9291 -> 20352 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:DESSUB0.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (114 -> 320 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:ICESUB0.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (51 -> 320 bytes)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:cf2#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- cannon fodder 2 - not very festive (1995)(sensible software)/Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:cf2.rel#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 153206 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:SSBOOT

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
- `empty_routine`: This routine contains no executable instructions.

### Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:cf2#unpacked (raw code file, load address unknown)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### Cannon Fodder 2 - Not Very Festive (1995)(Sensible Software).adf:cf2.rel#unpacked (raw code file, load address unknown)

- `stub_raw_code_bad_data`: The start of a raw code file of over five thousand instructions, but the decompiler produced only bad instruction data at this entry.
