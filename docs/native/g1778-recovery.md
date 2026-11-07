# Game 1778 (cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01778). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (908864 top-level bytes in 88 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 5522 raw-word lines |
| Functions named and described | pass, 67 of 67 named (0 author symbols), 67 of 67 described (4 by rule, 9 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 972 of 972 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 238642 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7589 of 7589 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 43952 |
| code | 6289 |
| data | 716208 |
| padding | 18511 |
| unused | 123904 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:john | 6 | 223 | 8 |
| Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:SSBOOT | 1 | 529 | 14 |
| Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:ATX-TRAINER.EXE | 2 | 235 | 8 |
| Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 872 | 34 |
| Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:cf2#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |
| Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:CF2.REL#unpacked (raw code file, load address unknown) | 1 | 5719 | 1 |
| Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:FONT.RAW#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:3.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (817 -> 41120 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:1.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2675 -> 41120 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:JON.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (94572 -> 107786 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:SENSPROD.PAL: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (93 -> 96 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:cf2: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (72124 -> 174774 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:KILLER.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (273625 -> 296072 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:VIRGPRES.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5374 -> 41120 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:9.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (942 -> 41120 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:DISK2.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11763 -> 51400 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:D.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6913 -> 41120 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:CFTITLE.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (10199 -> 51400 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:8.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9783 -> 41120 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:WON.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8328 -> 51400 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:B.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (14149 -> 41120 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:CF2.REL: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (14678 -> 22876 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:6.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11160 -> 41120 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:4.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11505 -> 41120 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:KILLER.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1227 -> 2989 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:2.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (20555 -> 41120 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:C.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4086 -> 41120 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:FONT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4371 -> 40992 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:7.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (13089 -> 41120 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:A.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11726 -> 41120 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:5.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (12228 -> 41120 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:JON.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2563 -> 7935 bytes)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:SENSPROD.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7533 -> 51400 bytes)
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 13 64 KB pages each with >= 29491 distinct PCs executed, 425984 of its 458048 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 13 64 KB pages each with >= 29491 distinct PCs executed, 425984 of its 458048 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 15 64 KB pages each with >= 29491 distinct PCs executed, 491520 of its 523457 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 15 64 KB pages each with >= 29491 distinct PCs executed, 491520 of its 523462 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-x25k slid through memory (an address: 15 64 KB pages each with >= 29491 distinct PCs executed, 491520 of its 523544 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- runtime segment an address: 609 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:cf2#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:CF2.REL#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:FONT.RAW#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:disk1: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf: blocks 11-21 (bytes 5632-11264) hold ASCII text
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:john: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- cannon fodder 2 (1994)(virgin)[cr pdx](disk 1 of 3)[t +11 atx]/Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:ATX-TRAINER.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 215766 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:ATX-TRAINER.EXE

## Copy protection found

- none found

## Functions


### Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:john

- `scan_unpack_tagged_blocks`: Walks a linked list of memory blocks, and for each block whose four-byte tag matches a packed-data signature it moves the data into place and unpacks it through a byte-copy loop driven by a bit reader.
- `find_table_entry_extract_bit`: Scans a table of word pairs until a masked word matches the second word, then reads a byte stored in that entry and compares it with a supplied byte.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: ANDs a word from one register with a word from another.
- `return_empty_routine`: The routine is empty; it contains no instructions that modify memory or registers and immediately returns.
- `build_bit_mask_table`: Reads a count and fills a stack buffer from a helper, then loops over a range of bit positions.
- `walk_marked_list_add_offsets`: While the list head carries a fixed marker value, it clears the entries and follows a chain of links a number of times given by a stored count.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:SSBOOT

- `boot_entry_load_and_relocate`: Program entry that copies the command-line argument into a buffer and appends a fixed extension, opens that file through the DOS library and reads it, then applies a relocation pass over bit masks before jumping to the loaded code.
- `apply_relocation_bits_and_jump`: Finds the first set bit of a mask in a register.
- `copy_block_jump_through_pointer`: Copies a block of bytes from a source to a destination and then jumps through a pointer that the decompiled body does not resolve.
- `copy_block_backwards_then_jump`: Copies a block of bytes backwards from a source to a destination, then jumps through a pointer that the decompiled body does not resolve.
- `print_text_via_dos_output`: Opens a library through the system library table, measures a text string up to its terminator, and writes it out through the DOS output and write routines.
- `scan_memory_region_types`: Loops over 32 memory regions, and for each tests up to 256 sample addresses through the exec memory-type call.
- `open_and_check_packed_file`: Opens a named file through the DOS library, seeks to and reads its header, and checks a four-byte tag that marks packed data.
- `decode_packed_chunk_stream`: Validates a four-byte packed-data tag, then decodes a packed chunk.
- `parse_bit_table_entry`: Scans a table of word pairs until a masked word matches the second word, then uses an entry byte as a shift count and a second entry byte to extract a bit from the masked word.
- `mask_and_conditional_call_2`: The routine masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `return_empty_routine_2`: The routine is empty and contains no instructions that modify memory or registers before returning.
- `read_big_endian_longword`: Reads four consecutive bytes from a pointer and concatenates them into a 32-bit value, with each byte shifted in behind the previous one, so the first byte becomes the most significant.
- `build_canonical_code_table`: Reads a count and a stack buffer of code lengths from a helper, then loops over bit lengths one to sixteen.
- `unpack_tagged_blocks_if_enabled`: Calls a helper to test a condition.

### Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:ATX-TRAINER.EXE

- `unpack_tagged_blocks_after_alloc`: Requests memory through the exec library, and if that succeeds walks the tagged block list, moves each packed block into place and unpacks it through the byte-copy loop.
- `extract_flag_bit_from_table`: Scans a table of word pairs until a condition on a global mask is met, then reads a byte from a table entry and uses it as an index into a global mask to extract a bit, calling another routine if a borrow condition is detected.
- `return_ones_or_helper_result`: Returns the all-ones word, unless a byte from a register is below sixteen; in that case it calls a helper and returns the helper's result.
- `mask_and_conditional_call_3`: Performs a bitwise AND between two words and, if one byte is less than another byte, calls another routine to retrieve a replacement word; otherwise, it returns the result of the bitwise AND.
- `return_immediately`: The routine is empty and performs no operations before returning.
- `fill_figure_table`: Reads a count, fills a table with values from a stack buffer, and then iterates through a loop to write bit masks and derived values into a table based on a key list.
- `walk_marked_list_add_offsets_2`: While the list head carries a fixed marker value, it clears the entries and follows a chain of links a number of times given by a stored count.
- `return_immediately_entry`: The body only returns, so it does nothing.

### Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_init_hardware`: Clears nine pairs of words and a large block of memory, stores joystick and status port values in shared globals, and writes constants to the interrupt, DMA, colour and copper control registers.
- `build_display_copper_table`: The listed facts show no hardware registers for this entry.
- `call_helper_twice_then_return`: Calls a helper routine twice in sequence and then returns.
- `combine_nibbles_into_20bit_value`: Combines the low nibble of the low word, the low word shifted right by four, and the high word shifted left by eight, then masks the result to twenty bits.
- `return_empty_entry_point`: The decompiled body is only a return, although the listed facts count 37 instructions.
- `render_text_glyphs_to_buffer`: Clears a 127-row buffer when its state is zero, then walks a text string, copying each character's eight-byte glyph into the buffer row by row with a fixed stride, and it handles a newline byte.
- `return_without_action_entry`: The decompiled body is only a return.
- `store_pointer_and_offset_pair`: Stores the input longword as two words in one pointer pair, and the same longword plus a fixed offset as two words in a second pair.
- `start_copper2_wait_raster`: When a flag bit is set, it writes the second copper list address and the jump register, sets a flag from comparing a byte with a fixed character, and waits until the raster position passes a fixed line.
- `spin_forever_when_flag_set`: When a shared flag bit is set, the routine enters an infinite loop, so the write of the flag value that follows is unreachable in the code shown.
- `store_table_sum_deltas`: Adds two table values selected by offsets held in the record and stores their sum, then fills 255 entries of a word table with the differences between successive sums minus a fixed offset.
- `xor_bit_pattern_into_bitplane`: XORs one bit per row across eight bit positions into two screen locations for each of 44 rows, using offsets taken from table values and the record.
- `swap_and_store_screen_pointers`: Swaps two longword pointers held in the record, computes a second pointer by adding a record offset, and writes the low and high words of these pointers into shared bitplane pointer globals, along with a pointer offset by a fixed amount.
- `configure_cia_timer_interrupt`: Stores a handler routine address in a shared word, programs the CIA-B timer and control registers with fixed values, and enables interrupt bits through the interrupt enable register.
- `clear_flag_set_dma_interrupts`: Clears one bit in a shared flag byte, then writes a fixed sequence of constants to the DMA control and interrupt enable registers.
- `scale_channel_levels_and_step`: Calls a setup helper, then multiplies four table bytes by a current level and shifts each product down into spaced words of the working record.
- `store_scaled_step_difference`: Multiplies a shared value by a constant and divides the product by the same value, then stores in the working record the byte difference between two successive scaled counter values.
- `run_countdown_sequencer_step`: Returns zero when a shared enable byte is clear.
- `write_timer_and_request_registers`: Writes a counter value, a derived value and a copied word to shared globals that the facts identify as the CIA-B interrupt and control registers, the DMA control register and the interrupt request register.
- `reset_audio_channel_registers`: Adjusts a shared value, then writes eight table entries into the pointer and length registers of all four audio channels, followed by a final constant write to the interrupt request register.
- `call_two_helper_routines`: The decompiler timed out, so no pseudo-code is available and the description rests only on the listed facts.
- `find_table_entry_under_limit`: Selects a nibble of the input byte according to a phase counter stored in the record, then scans a table row of up to 36 words for the first entry that does not exceed a stored limit.
- `decrease_position_with_floor`: Subtracts a delta from a position field in the record and raises the position to a fixed minimum when it falls below that floor.
- `step_value_toward_target`: Calls a helper, then moves a position field toward a target field by a step size taken from another field.
- `add_delta_reset_to_reference`: Adds a delta to a position field and, when the delta exceeds a second input value, replaces the position with a reference field from the record.
- `set_level_offset_when_flagged`: When the flag byte is nonzero, stores the input nibble byte in the record.
- `offset_position_by_table_value`: Looks up a table value by an index from the record byte, scales it by the low nibble of the stored byte, negates it when the sign bit is set, and writes the position plus this offset to the output.
- `relay_adjust_level_by_nibble`: Calls a helper, then adjusts a byte level in the record using the input byte.
- `adjust_level_by_nibble`: Adjusts a byte level in the record using the input byte.
- `store_level_byte`: Stores the input byte directly into a level field of the working record and returns.
- `set_index_scaled_clear_counter`: Stores the input word multiplied by four into an index field of the record and resets a counter field to zero.
- `store_byte_and_call_helper`: Stores its byte argument into a shared global and then calls a helper routine.
- `advance_position_by_stored_step`: Copies a nonzero input byte into a step field of the record when present.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.

### Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:cf2#unpacked (raw code file, load address unknown)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### Cannon Fodder 2 (1994)(Virgin)[cr PDX](Disk 1 of 3)[t +11 ATX].adf:CF2.REL#unpacked (raw code file, load address unknown)

- `execute_main_code`: This is a large code block consisting of over five thousand instructions.
