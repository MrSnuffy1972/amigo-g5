# Game 1658 (soccer kid (1994)(krisalis software)(disk 1 of 4)[cr][t +9 atx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01658). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904592 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 15 of 15 named (0 author symbols), 15 of 15 described (0 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 22 of 22 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 246 of 246 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1296 |
| code | 986 |
| data | 901798 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Soccer Kid (1994)(Krisalis Software)(Disk 1 of 4)[cr][t +9 ATX].adf:bootblock | 1 | 215 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 31 | 2 |

## Cases handled

- soccer kid (1994)(krisalis software)(disk 1 of 4)[cr][t +9 atx]/Soccer Kid (1994)(Krisalis Software)(Disk 1 of 4)[cr][t +9 ATX].adf: custom-format (non-DOS) disk, 1757 non-empty blocks: matched against track-loaded segments
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 135 64 KB pages each with >= 29491 distinct PCs executed, 2211840 of its 2211898 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-long-joy slid through memory (an address: 135 64 KB pages each with >= 29491 distinct PCs executed, 2211840 of its 2211898 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-long-keys slid through memory (an address: 135 64 KB pages each with >= 29491 distinct PCs executed, 2211840 of its 2211898 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-long-mixed slid through memory (an address: 135 64 KB pages each with >= 29491 distinct PCs executed, 2211840 of its 2211898 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 135 64 KB pages each with >= 29491 distinct PCs executed, 2211840 of its 2211898 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 135 64 KB pages each with >= 29491 distinct PCs executed, 2211840 of its 2211898 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 135 64 KB pages each with >= 29491 distinct PCs executed, 2211840 of its 2211898 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 135 64 KB pages each with >= 29491 distinct PCs executed, 2211840 of its 2211898 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- runtime segment an address (play-long-joy): 22 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- soccer kid (1994)(krisalis software)(disk 1 of 4)[cr][t +9 atx]/Soccer Kid (1994)(Krisalis Software)(Disk 1 of 4)[cr][t +9 ATX].adf: 5 track-loaded blocks found in recovered runtime segments and classified through them
- soccer kid (1994)(krisalis software)(disk 1 of 4)[cr][t +9 atx]/Soccer Kid (1994)(Krisalis Software)(Disk 1 of 4)[cr][t +9 ATX].adf: blocks 1017-1215 (bytes 520704-622592) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.10)
- 2 packed track region(s), 795136 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Soccer Kid (1994)(Krisalis Software)(Disk 1 of 4)[cr][t +9 ATX].adf:bootblock

- `load_blocks_then_copy_code`: Issues exec DoIO requests to load blocks, disables interrupts and DMA channels, and copies routine code into two fixed buffers, 26 longwords and then 28 words.
- `install_vectors_and_copy_code`: Stores three addresses into low-memory cells, calls three unnamed routines, then clears a global and copies 28 words of routine code into a fixed buffer.
- `call_then_spin_forever`: Calls one unnamed routine and takes a byte result from it, then loops forever, because setting the top bit of a byte never produces zero.
- `copy_word_block`: Copies consecutive 16-bit words from a source pointer to a destination pointer for a count held in a data register, moving one more word than the count.
- `write_text_marker_by_flag`: Writes the four-character text FAT0 or FAT1 into a buffer, choosing FAT1 when bit 13 of a global value is set.
- `detect_refresh_rate_hz`: Resets CIA port and timer registers, reads a 16-bit count from two CIA bytes after a timed wait, and returns 50 when the count is below a threshold, otherwise 60.
- `wait_until_status_byte_clear`: Repeatedly checks a status byte at a fixed offset from a base register and returns minus one once its top bit is clear.
- `build_ram_entry_table`: Clears 188 bytes of a structure and writes the text RAMC and RAMF into it, then loops a fixed number of times storing each value from an unnamed routine as a RAMP entry when it is below a threshold, and otherwise into a fixed field.
- `find_list_node_with_bit`: Walks a linked list starting from a pointer in a structure, moving to each node's next pointer while the bit selected by a data register is not set in the node's 16-bit field, and stops at the end of the list.
- `advance_list_until_bit_set`: Walks a linked list from a pointer held in an address register, moving to the next node while the bit selected by a data register is not set in the node's 16-bit field.
- `init_field_if_flag_clear`: Stores a computed 32-bit value, built from a 16-bit argument and a constant, into a structure field when a flag byte in memory is zero.
- `copy_longword_and_store_field`: Copies one 32-bit value from a source pointer to a destination pointer, then stores a packed 32-bit value built from a 16-bit argument and a constant into a structure field.
- `patch_code_words_then_call`: Overwrites several words and longwords of a code region with fixed instruction patterns, such as no-operation and branch words, then calls a routine through the start of that region.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_hardware_and_jump`: Compares a 16-bit value in a global variable to 368, and if it is less than 368, writes a value to a hardware register, clears the global variable, and jumps to a fixed address; otherwise, it disables interrupts, sets three hardware registers to 32767, and jumps to a fixed address.
- `populate_data_table`: Reads a bit from a hardware register and writes it to a hardware register, then selects a starting value based on a global variable and writes a sequence of incrementing values into a table using a loop, finally increments a global variable and returns from an interrupt.
