# Game 929 (after the war (1989)(dinamic)[cr qtx][h acu]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00929). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903216 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 14 raw-word lines |
| Functions named and described | pass, 29 of 29 named (0 author symbols), 29 of 29 described (2 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 25 of 25 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 976 of 976 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3149 |
| code | 2360 |
| data | 890537 |
| padding | 2 |
| unused | 7168 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| After the War (1989)(Dinamic)[cr QTX][h ACU].adf:bootblock | 1 | 39 | 2 |
| logged disk load from After the War (1989)(Dinamic)[cr QTX][h ACU].adf boot-block load | 1 | 163 | 2 |
| logged disk load from After the War (1989)(Dinamic)[cr QTX][h ACU].adf boot-block load | 1 | 163 | 2 |
| logged disk load from After the War (1989)(Dinamic)[cr QTX][h ACU].adf boot-block load | 1 | 163 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 448 | 21 |

## Cases handled

- after the war (1989)(dinamic)[cr qtx][h acu]/After the War (1989)(Dinamic)[cr QTX][h ACU].adf: custom-format (non-DOS) disk, 1744 non-empty blocks: matched against track-loaded segments
- after the war (1989)(dinamic)[cr qtx][h acu]/After the War (1989)(Dinamic)[cr QTX][h ACU].adf: the boot block reads 3584 bytes from disk offset 896000 to an address (found statically); recovered as a unit
- after the war (1989)(dinamic)[cr qtx][h acu]/After the War (1989)(Dinamic)[cr QTX][h ACU].adf: the boot block reads 3584 bytes from disk offset 896000 to an address (found statically); recovered as a unit
- after the war (1989)(dinamic)[cr qtx][h acu]/After the War (1989)(Dinamic)[cr QTX][h ACU].adf: the boot block reads 3584 bytes from disk offset 896000 to an address (found statically); recovered as a unit
- runtime segment an address: 25 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- after the war (1989)(dinamic)[cr qtx][h acu]/After the War (1989)(Dinamic)[cr QTX][h ACU].adf: 3 logged loads cover 3584 bytes of the image
- after the war (1989)(dinamic)[cr qtx][h acu]/After the War (1989)(Dinamic)[cr QTX][h ACU].adf: blocks 1757-1757 (bytes 899584-900096) outside the filesystem hold data, not code (opcode density 0.027, entropy 5.60)
- 1 packed track region(s), 888832 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `stub_infinite_wait_loop` (runtime segment an address): Contains only an endless empty loop that never returns. Kind: sabotage path / infinite loop trap. Pass path: not determined.
- `select_drive_and_wait_ready` (runtime segment an address): Updates the low drive-control bits of the disk control register, waits a fixed count, then waits until a status bit clears. Kind: disk hardware/drive status check. Pass path: the instruction immediately following the call (e.g., bsr.w an address).
- `set_drive_bits_and_wait` (runtime segment an address): Sets the low two drive-control bits, waits a fixed count, then waits until a status bit clears. Kind: disk hardware/drive configuration check. Pass path: the loop completes and execution continues to the next instruction after the call.
- `set_control_byte_and_wait` (runtime segment an address): Stores 117 into a control global, waits a fixed count, then waits until a status bit clears. Kind: disk hardware status polling/sync check. Pass path: the instruction immediately following the call (bset.b #an address, an address.l).

## Functions


### After the War (1989)(Dinamic)[cr QTX][h ACU].adf:bootblock

- `bootblock_load_data_blocks`: Boot block entry that issues exec I/O requests: it reads 3584 bytes from a disk offset to a fixed address, then allocates a buffer and reads 1024 bytes into it.
- `three_instruction_undecoded_routine`: Three instructions at an address-taken entry with no decompiled body, so the behaviour is not recovered from the facts.

### logged disk load from After the War (1989)(Dinamic)[cr QTX][h ACU].adf boot-block load

- `refill_bit_buffer_word`: Reads the longword before the input pointer, joins its low byte with a register byte into a stored word, and returns the longword shifted right one bit with its top bit set.
- `discard_stream_bits`: Shifts the packed bit buffer once per requested bit, calling the refill routine whenever the buffer runs empty.

### logged disk load from After the War (1989)(Dinamic)[cr QTX][h ACU].adf boot-block load

- `refill_bit_buffer_word`: Reads the longword before the input pointer, joins its low byte with a register byte into a stored word, and returns the longword shifted right one bit with its top bit set.
- `discard_stream_bits`: Shifts the packed bit buffer once per requested bit, calling the refill routine whenever the buffer runs empty.

### logged disk load from After the War (1989)(Dinamic)[cr QTX][h ACU].adf boot-block load

- `refill_bit_buffer_word`: Reads the longword before the input pointer, joins its low byte with a register byte into a stored word, and returns the longword shifted right one bit with its top bit set.
- `discard_stream_bits`: Shifts the packed bit buffer once per requested bit, calling the refill routine whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_head_undecoded_routine`: Entered at a loop head and makes one call, with twelve instructions in total.
- `copy_interleaved_rows_to_planes`: Copies sixteen rows from a source table into four bitplanes located 8000 bytes apart, taking four bytes from each eight-byte source group and advancing 40 bytes per row.
- `write_intena_around_two_calls`: Writes an interrupt enable mask with the set bit, calls two routines, and then writes a second mask to the same register.
- `start_disk_track_read`: Converts a block number into track and sector values by dividing by 22 and then 11.
- `stub_infinite_wait_loop`: Contains only an endless empty loop that never returns.
- `select_drive_and_wait_ready`: Updates the low drive-control bits of the disk control register, waits a fixed count, then waits until a status bit clears.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `wait_disk_status_then_return`: Waits while one control bit is set, calling a routine on each pass, then waits until a second status bit clears.
- `set_drive_bits_and_wait`: Sets the low two drive-control bits, waits a fixed count, then waits until a status bit clears.
- `wait_count_then_status_clear`: Counts down a fixed count, then waits until a status bit clears.
- `wait_status_bit_clear`: Loops while a status bit is set and returns once it is clear.
- `set_control_byte_and_wait`: Stores 117 into a control global, waits a fixed count, then waits until a status bit clears.
- `decode_mfm_sector_after_sync`: Clears a 32-longword buffer, checks two words for the MFM sync pattern, and writes a padding pattern before decoding the sector data.
- `find_matching_table_entry_byte`: Scans 8-byte table entries from a pointer until the byte at offset 2 matches the input byte, leaving the pointer on the matched entry.
- `unpack_bitstream_block`: Unpacks a block using a 32-bit bit buffer read from the input, with carry bits selecting literal runs or back-references.
- `service_interrupt_request_counters`: Interrupt-style handler that checks request bits in the interrupt request and enable registers.
- `tiny_ori`: Small routine of 1 instruction that merges 250 into d0.
- `fill_plane_pointer_table`: Writes a series of 32-bit addresses into 8-byte table entries, starting from a base value and advancing by 8000 for each entry.
- `load_palette_from_table`: Copies 32 words from a source table into the colour registers, starting at the first colour register.
- `store_serial_bit_in_bitmap`: Handles one step of a serial input word: when its high bit is set it toggles a parity flag, and on one phase it writes a single bit into a 128-bit bitmap at an index taken from the input and a lookup table.
- `undecoded_bit_change_routine`: Twelve instructions including a bit-change operation at an address-taken entry.
