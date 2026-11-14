# Game 2499 (statix (playable preview) (1995)(black legend)[m interactive]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02499). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (878592 top-level bytes in 19 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 20 of 20 named (0 author symbols), 20 of 20 described (2 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 6 of 6 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1547 of 1547 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 13796 |
| code | 4546 |
| data | 391677 |
| padding | 5213 |
| unused | 463360 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Type | 1 | 426 | 2 |
| Assign | 2 | 117 | 2 |
| button | 1 | 5 | 1 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| disk-validator | 1 | 386 | 1 |
| Statix (playable preview) (1995)(Black Legend)[m Interactive].adf:Statix_Intro | 1 | 242 | 5 |
| Statix (playable preview) (1995)(Black Legend)[m Interactive].adf:Statix_Main | 1 | 242 | 5 |
| Statix (playable preview) (1995)(Black Legend)[m Interactive].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- statix (playable preview) (1995)(black legend)[m interactive]/Statix (playable preview) (1995)(Black Legend)[m Interactive].adf:Pics/Pic1: PowerPacker Data (xfd)-packed, unpacked with ancient (28556 -> 32032 bytes)
- statix (playable preview) (1995)(black legend)[m interactive]/Statix (playable preview) (1995)(Black Legend)[m Interactive].adf:Mods/Mod1: PowerPacker Data (xfd)-packed, unpacked with ancient (55692 -> 79898 bytes)
- statix (playable preview) (1995)(black legend)[m interactive]/Statix (playable preview) (1995)(Black Legend)[m Interactive].adf:Statix_Intro: Titanics Cruncher 1.2 (xfd) stream found by ancient scan (179748 -> 272600 bytes) but only 0 of 117 sampled windows are in reference RAM: not used
- statix (playable preview) (1995)(black legend)[m interactive]/Statix (playable preview) (1995)(Black Legend)[m Interactive].adf:Statix_Main: Titanics Cruncher 1.2 (xfd) stream found by ancient scan (85612 -> 384936 bytes) but only 0 of 52 sampled windows are in reference RAM: not used

## Cracktros (kept, not protection)

- Statix (playable preview) (1995)(Black Legend)[m Interactive].adf:Statix_Intro

## Copy protection found

- none found

## Functions


### Type

- `parse_type_options_and_dump`: Opens dos.library, walks the argument string checking for the single-letter flags H and N, and rejects the combination of both with an error message.
- `read_and_format_hex_line`: Reads bytes through a library call and builds a line of two hex digits per byte from a 16-character digit table, padding with spaces and a period for non-printable values, with a 60-space initial fill.

### Assign

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_d1_to_d0_branch`: Copies the 32-bit value in register d1 into register d0, then branches to a point inside another routine.

### button

- `read_fire_button_status`: Clears register d0, then reads the CIA-A port A register at the fire-button bit and returns 5 in d0 when that bit is low, otherwise it returns 0.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `scan_list_and_build_frame`: Walks a linked list of entries to find the largest value in a field, then, if the setup call succeeds, writes that value and a run of sequential words into a buffer, sets up a stack frame, copies sixteen words between a temporary area and a global buffer, and installs a function pointer, returning 0 on success or -1 otherwise.
- `copy_d1_to_d0_branch_alt`: Copies the 32-bit value in register d1 into register d0, then branches to a point inside another routine.

### disk-validator

- `updates_score_display_and_resets_state`: Writes two digit characters derived from a counter at offset 744 of a game structure into a display buffer, sets several state fields in that structure to values derived from other fields, then calls a routine through a function pointer and loops over a table of 32-bit entries, filling each with a fixed address up to an index held in the structure.

### Statix (playable preview) (1995)(Black Legend)[m Interactive].adf:Statix_Intro

- `decompress_bitstream_into_buffer`: Opens dos.library and, after rejecting the combined H and N option flags, sets up a bit-driven decoder that reads a compressed stream from a source pointer and writes expanded bytes through the destination pointer in a1 using back-references and inverted literal copies.
- `saves_calls_offset_function`: Saves registers d0-d7 and a1-a6 on the stack, loads two longwords from memory into d1 and d2 and sets d3 to 5120, then calls a function at a negative offset from a5.
- `expand_compressed_bytes`: Decodes a bit-driven stream from d0 and d2 into bytes written through a1, using repeated back-references and inverted literal copies.
- `shift_bits_into_word`: Clears d2 and shifts d0 left one bit per pass, repeating as the count in d1 decrements.
- `fetch_longword_and_rotate`: Loads the longword at the pointer in a0 into d0 and advances a0 by four bytes, then calls another routine when a0 equals a constant address held in memory.

### Statix (playable preview) (1995)(Black Legend)[m Interactive].adf:Statix_Main

- `decompress_stream_into_allocated_blocks`: Opens dos.library, reports an error and exits if the argument string contains a conflicting flag, then repeatedly reads a bit-packed compressed stream through a helper, allocating memory blocks and expanding back-referenced and inverted byte runs into them.
- `saves_calls_offset_function_2`: Saves registers d0-d7 and a1-a6 on the stack, loads two longwords from memory into d1 and d2 and sets d3 to 5120, then calls a function at a negative offset from a5.
- `decode_lz_bitstream_to_buffer`: Decodes a bit-packed compressed stream: reads flag bits from a shared bit-buffer word, copies literal bytes with inverted values into the output at a1 and, for back-references, copies earlier output bytes forward, looping until the output pointer reaches the end address computed from a1 plus d0.
- `shift_bits_into_word_2`: Clears d2 and shifts d0 left one bit per pass, repeating as the count in d1 decrements.
- `fetch_longword_and_shift`: Reads the longword at a0 into d0 and advances a0 by four, calling another routine only when the new a0 matches a stored constant address.

### Statix (playable preview) (1995)(Black Legend)[m Interactive].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
