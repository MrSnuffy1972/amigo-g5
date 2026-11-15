# Game 743 (benefactor (1994)(psygnosis)[cr cls]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00743). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2703712 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 21 of 21 named (0 author symbols), 21 of 21 described (1 by rule, 8 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 115 of 115 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1536 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 777 of 777 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 5880 |
| code | 1610 |
| data | 2688027 |
| padding | 3 |
| unused | 8192 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3).adf:bootblock | 1 | 66 | 1 |
| logged disk load from Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3).adf boot-block load | 1 | 286 | 9 |
| logged disk load from Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3).adf boot-block load | 1 | 286 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 139 | 2 |
| Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3).adf bytes 1024-2560 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- benefactor (1994)(psygnosis)[cr cls](disk 1 of 3)/Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3).adf: custom-format (non-DOS) disk, 1747 non-empty blocks: matched against track-loaded segments
- benefactor (1994)(psygnosis)[cr cls](disk 2 of 3)/Benefactor (1994)(Psygnosis)[cr CLS](Disk 2 of 3).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- benefactor (1994)(psygnosis)[cr cls](disk 3 of 3)/Benefactor (1994)(Psygnosis)[cr CLS](Disk 3 of 3).adf: custom-format (non-DOS) disk, 1753 non-empty blocks: matched against track-loaded segments
- reference run failed (Karl 4 Nov 2026): run play-rs-x25k slid through memory (an address: 2 64 KB pages each with >= 29491 distinct PCs executed, 21755 of its 21988 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-x35k slid through memory (an address: 2 64 KB pages each with >= 29491 distinct PCs executed, 21755 of its 21988 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- benefactor (1994)(psygnosis)[cr cls](disk 1 of 3)/Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3).adf: the boot block reads 7680 bytes from disk offset 854528 to an address (found statically); recovered as a unit
- benefactor (1994)(psygnosis)[cr cls](disk 1 of 3)/Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3).adf: the boot block reads 5632 bytes from disk offset 854528 to an address (found statically); recovered as a unit
- runtime segment an address: 115 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- benefactor (1994)(psygnosis)[cr cls](disk 1 of 3)/Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3).adf: bytes 1024-2560 look like code no run loaded: recovered position-independently (load address unknown)
- benefactor (1994)(psygnosis)[cr cls](disk 1 of 3)/Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3).adf: 3 logged loads cover 9216 bytes of the image
- benefactor (1994)(psygnosis)[cr cls](disk 1 of 3)/Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3).adf: blocks 12-12 (bytes 6144-6656) outside the filesystem hold data, not code (opcode density 0.016, entropy 6.04)
- benefactor (1994)(psygnosis)[cr cls](disk 1 of 3)/Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3).adf: blocks 307-676 (bytes 157184-346624) hold 1 packed streams (IMP: File Imploder) found by scanning, 99% of the bytes
- benefactor (1994)(psygnosis)[cr cls](disk 1 of 3)/Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3).adf: blocks 1684-1759 (bytes 862208-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.07)
- benefactor (1994)(psygnosis)[cr cls](disk 2 of 3)/Benefactor (1994)(Psygnosis)[cr CLS](Disk 2 of 3).adf: blocks 2-1759 (bytes 1024-901120) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.05)
- benefactor (1994)(psygnosis)[cr cls](disk 3 of 3)/Benefactor (1994)(Psygnosis)[cr CLS](Disk 3 of 3).adf: blocks 1683-1759 (bytes 861696-901120) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.60)
- 1 never-loaded region(s), 1536 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 3 packed track region(s), 1514496 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3).adf:bootblock

- `setup_copper_and_display`: Its register accesses show it sets up the beam and bitplane control, the background colour, the copper list pointer and jump, and the DMA and interrupt enables.

### logged disk load from Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3).adf boot-block load

- `copy_word_to_color01_pair`: Obtains a word from a helper call and writes it both to a memory slot and to the COLOR01 palette register, twice in a row, advancing both pointers each time.
- `step_nibbles_toward_target`: Compares the three nibble fields of the input word with those of a target value and moves each nibble one step toward its target.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `clear_memory_array`: Writes zero into each of 8000 longword entries of a memory array in a loop.
- `open_graphics_and_intuition_libraries`: Opens the graphics and intuition libraries through the exec library routine, storing the two returned library base pointers in global data.
- `scan_text_lines_advancing_rows`: Walks a text buffer one newline-terminated line at a time, advancing a row value by sixteen after each line.
- `measure_text_and_call_library`: Counts the characters of a string up to a newline or terminator.
- `point_globals_and_load_font`: Points seven global pointers at one shared variable, then calls two library routines through the library base.
- `call_two_library_routines`: Calls two library routines in sequence through the library base pointer and does no other work.

### logged disk load from Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3).adf boot-block load

- `store_two_words_global`: Writes two consecutive words to a global variable starting at a fixed address.
- `adjust_nibbles_combine`: Compares nibbles of two values and adjusts each nibble toward the target by incrementing or decrementing, then combines the results.
- `wait_flag_then_idle_forever`: Spins until a global flag reads as minus one, then enters an endless loop with nothing else to do.
- `clear_memory_array_2`: Clears a fixed-size array in memory by writing zero to each of its entries in a loop.
- `call_open_libraries`: Reads a global pointer and calls the routine stored at two offsets from it, storing the returned values.
- `measure_text_and_call_library_2`: Counts the characters of a string up to a newline or terminator.
- `count_string_dispatch`: Counts characters in a string until a newline or terminator, then calls several routines when the count is non-zero.
- `point_globals_and_load_font_2`: Points global pointers at a shared variable and fixed addresses, then calls two library routines through the library base.
- `call_library_offsets`: Calls two routines located at specific offsets from a value stored in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `unpack_lz_backreference_stream`: Copies a block of words forward, then decodes a bit stream in which each step either copies a literal byte or repeats a run from earlier output.
- `decompress_lz_backreference_stream`: Reads a stream of bytes from a source pointer and performs a series of conditional expansions and memory copies to a destination buffer.
