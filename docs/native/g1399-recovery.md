# Game 1399 (b.c. kid (1992)(ubi soft)[cr dc]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01399). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1803056 top-level bytes in 99 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 25 of 25 named (0 author symbols), 25 of 25 described (2 by rule, 23 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 371 of 371 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 231228 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 470 of 470 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 119700 |
| code | 1510 |
| data | 1552579 |
| padding | 12019 |
| unused | 117248 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:bckid | 1 | 178 | 6 |
| B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:Dual_Crew | 1 | 99 | 3 |
| B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 183 | 15 |
| B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:SND0#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:SND3#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:L__X#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:L__0: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (10012 -> 16252 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:L__1: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (37684 -> 62320 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:L__2: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (15466 -> 31312 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:L__3: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (29624 -> 51172 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:L__4: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (33040 -> 58020 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:L__5: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (16120 -> 34196 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:SND0: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (79490 -> 114370 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:SND1: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (37664 -> 44494 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:SND2: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (36432 -> 45696 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:SND3: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (38822 -> 45528 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:SND6: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (31896 -> 38112 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:L__B: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (32174 -> 81176 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:E__1: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (47894 -> 101800 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:E__2: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (32046 -> 73256 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:E__3: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (53356 -> 116592 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:L__X: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (42996 -> 71330 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:E__4: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (48852 -> 111368 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:L__6: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (25788 -> 45820 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:L__7: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (17448 -> 30664 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:L__8: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (4348 -> 14948 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:L__9: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (12192 -> 21912 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:SND5: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (37940 -> 43940 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:L__B: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (32174 -> 81176 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:E__5: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (30304 -> 73040 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:E__6: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (49648 -> 123440 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:E__7: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (48820 -> 121680 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:E__8: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (27588 -> 75864 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:E__9: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (48580 -> 115696 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:END2: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (21606 -> 53288 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:E_10: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (19310 -> 55544 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:E_11: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (45660 -> 111504 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:E_12: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (43670 -> 103120 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:E_13: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (27394 -> 79104 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:E_14: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (32790 -> 88712 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:L_11: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (27098 -> 49700 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:E_15: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (46834 -> 127040 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:L_12: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (12682 -> 31848 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:E_16: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (39874 -> 117408 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:L_13: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (28244 -> 58324 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:E_17: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (8096 -> 20270 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:L_14: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (6688 -> 13628 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:L_15: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (6628 -> 19352 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:L_16: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (15764 -> 30360 bytes)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf:L_17: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (5088 -> 16580 bytes)
- runtime segment an address: 133 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:SND0#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:SND3#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:L__X#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- b.c. kid (1992)(ubi soft)[cr dc](disk 2 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- b.c. kid (1992)(ubi soft)[cr dc](disk 1 of 2)/B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:bckid: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 3 never-loaded region(s), 231228 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:bckid

- `init_interrupts`: Points a register at the custom chip registers, clears a register, and writes a mask value to four interrupt-related registers to disable interrupts and DMA. It then sets the interrupt enable register to enable interrupts and loads a routine address into a global vector table.
- `copy_buffer_and_jump`: Enables coprocessor and interrupt priority, sets up the stack pointer, then copies 645 bytes from a source table into a buffer at address an address using a loop, and finally jumps indirectly through a global variable.
- `unpack_escape_run_stream`: Copies bytes from the source stream into a buffer until the source pointer reaches a fixed end address.
- `copy_and_unpack_stream`: Copies a run of bytes from a source buffer to a destination buffer, then decodes a compressed stream by reading bit flags and using them to index a lookup table, copying the resulting bytes back into the destination until the source is exhausted.
- `store_byte_and_return`: Pops a long value from the stack into a register, saves its low byte to a memory location, rotates the value right by one bit, and returns.
- `copy_bits_and_bytes`: Divides a loop count by two and, when the value is odd, calls a helper routine to compute a bit count, repeating this for each bit.

### B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:Dual_Crew

- `copy_block_and_process_bits`: Copies a block of bytes from a source area to a destination area in memory, then reads a 32-bit value and, for each set bit, rotates it into a running value and stores the resulting bytes into a buffer.
- `restore_stack_and_disable_interrupts`: Pops a 32-bit word from the stack, saves its low byte into a word at offset 5 of the structure in a5, disables interrupts via the custom chip, and returns the rotated value in d0.
- `count_set_bits`: Counts the number of set bits in the 16-bit value in d1 and stores the result in a byte at offset 2 of the structure in a5, then returns.

### B.C. Kid (1992)(UBI Soft)[cr DC](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_display_and_wait_vblank`: Initializes the custom chip registers for a display mode, writing bitmap modulation, copper enable, and display enable values, then clears a 32-byte block in a structure and sets a pointer in it.
- `copy_two_words_to_block`: Copies two words from a global cell into a block addressed through the base register at a fixed offset.
- `store_four_fixed_values`: Writes four fixed values, including one negative value, into consecutive global cells.
- `set_status_cell_ones`: Writes a fixed all-ones value to a status cell and returns.
- `save_state_and_disable_chips`: Saves three words from a structure into global cells, then writes an all-disabling value to three chip control registers, and later calls the graphics library to read a screen pointer into a structure cell.
- `call_routine_per_table_byte`: Clears a routine pointer at a fixed address, then repeatedly reads a byte from a table at a fixed address, calling another routine for each byte until it encounters a zero.
- `dispatch_handler_command`: Dispatches on the byte in d0: value 1 copies a byte from the pointer in a0 into a byte at offset 4 of a global variable, value 2 stores 255 at offset 0, value 3 clears that same byte, and any other value calls another routine.
- `advance_counter_and_pointer`: Saves registers on the stack, loads a table address, and calls a routine to map a character to a glyph row.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `process_byte_in_loop`: Pushes a byte onto the stack, loads a table address, converts another byte into a signed value by subtracting 32, and branches when it is below 40.
- `copy_eight_strided_bytes`: Copies eight bytes from a source offset to a destination at a fixed stride, advancing the source by a fixed row step each time.
- `copy_glyph_with_optional_mirror`: Maps a character to a glyph row like the other glyph routines and copies eight bytes into the destination.
- `busy_wait_fixed_count`: Runs a full 16-bit countdown loop doing no other work, then returns.
- `clear_sixteen_words`: Clears sixteen words starting at a block addressed through the base register at a fixed offset.
- `clear_memory_block_bytes`: Writes zero to a fixed-length run of bytes starting at a fixed address.
