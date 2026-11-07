# Game 1213 (last ninja 3 (1991)(system 3)(disk 1 of 2)[cr company][t +6 dc]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01213). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905200 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 53 of 53 named (0 author symbols), 53 of 53 described (2 by rule, 51 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 961 of 961 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1271 of 1271 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 5024 |
| code | 4158 |
| data | 893458 |
| unused | 2560 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Last Ninja 3 (1991)(System 3)(Disk 1 of 2)[cr Company][t +6 DC].adf:bootblock | 1 | 16 | 1 |
| logged disk load from Last Ninja 3 (1991)(System 3)(Disk 1 of 2)[cr Company][t +6 DC].adf boot-block load | 1 | 155 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1100 | 49 |

## Cases handled

- last ninja 3 (1991)(system 3)(disk 1 of 2)[cr company][t +6 dc]/Last Ninja 3 (1991)(System 3)(Disk 1 of 2)[cr Company][t +6 DC].adf: custom-format (non-DOS) disk, 1753 non-empty blocks: matched against track-loaded segments
- last ninja 3 (1991)(system 3)(disk 1 of 2)[cr company][t +6 dc]/Last Ninja 3 (1991)(System 3)(Disk 1 of 2)[cr Company][t +6 DC].adf: the boot block reads 5120 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 804 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- last ninja 3 (1991)(system 3)(disk 1 of 2)[cr company][t +6 dc]/Last Ninja 3 (1991)(System 3)(Disk 1 of 2)[cr Company][t +6 DC].adf: 1 logged loads cover 5120 bytes of the image
- last ninja 3 (1991)(system 3)(disk 1 of 2)[cr company][t +6 dc]/Last Ninja 3 (1991)(System 3)(Disk 1 of 2)[cr Company][t +6 DC].adf: blocks 12-118 (bytes 6144-60928) outside the filesystem hold data, not code (opcode density 0.057, entropy 5.52)
- last ninja 3 (1991)(system 3)(disk 1 of 2)[cr company][t +6 dc]/Last Ninja 3 (1991)(System 3)(Disk 1 of 2)[cr Company][t +6 DC].adf: blocks 123-125 (bytes 62976-64512) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.47)
- 1 packed track region(s), 836096 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Last Ninja 3 (1991)(System 3)(Disk 1 of 2)[cr Company][t +6 DC].adf:bootblock

- `initialize_and_process_table`: Initializes a data structure with fixed values, clears a region of memory by copying from a source, and then processes a table of bytes through a series of bit tests and shifts, storing results back into the table.

### logged disk load from Last Ninja 3 (1991)(System 3)(Disk 1 of 2)[cr Company][t +6 DC].adf boot-block load

- `decode_bitpacked_table`: Fills a block of memory by copying bytes from a global variable, then decodes a bit-packed stream and scatters the resulting values into a lookup table, branching on special marker bytes.
- `shift_and_store_nibble`: Reads a value from a pointer, stores a combined byte through another pointer, and returns the value shifted right with its carry bit set.
- `shift_and_reload_nibbles`: Loops over a count of nibbles, shifting a value right and reloading it from another routine whenever it becomes zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_display_and_interrupts`: Initializes the display and interrupt hardware by writing to the custom registers, sets up the display copper, and enables interrupts.
- `wait_for_flag_bit_set`: It writes the value 32 to a global variable and then spins in a loop until that variable's bit 5 becomes set, effectively waiting for an external agent to modify it.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `handle_blitter_status_check`: If the value in register D3 equals negative three, it writes all-ones into two consecutive 32-bit global variables; otherwise it calls another routine twice in a row.
- `decode_mfm_sector_loop_2`: Splits a byte into sector and offset fields, then loops over the sector count, filling gaps with an alternating-bit pattern and decoding sectors while advancing the sector position.
- `fill_buffer_with_gap_pattern`: It stores a fixed alternating-bit gap pattern into a buffer of 416 longwords, unless the first longword already holds that pattern, in which case it skips the work.
- `validate_sector_number_retry`: Reads a byte and returns when it is below eleven, which is the valid sector range.
- `decode_mfm_sector_loop`: Loops over a count of sectors, computing a destination offset from a passed value and a running counter.
- `blit_mfm_decode_pass`: Programs the blitter with source, mask and destination pointers and a size computed from a sector offset, then runs one bit-mask pass over a track buffer.
- `advance_sector_position_by_count`: Steps a sector index forward by one for each count in a register, wrapping back to zero after the eleventh sector, and calls a routine on each step.
- `start_blitter_copy_pass`: Computes three blitter pointer values from its input and an offset, programs the blitter control and size registers for a two-row copy, then starts the blitter.
- `validate_bitplane_interleave`: Verifies a bitplane memory layout by checking that two interleaved planes hold their expected alternating bit patterns, then repeatedly compares corresponding longwords across both planes and rejects mismatches, finally advancing through the buffer in fixed strides until the whole region is validated.
- `modify_byte_by_flag`: It reads a byte through a pointer and inspects bit 0 of the preceding byte to choose a branch, then either sets bit 7 or clears bit 7 of the target byte before writing it back.
- `check_bitplane_parity`: XORs the even-positioned bits of a run of longwords, then compares the result against a value derived from the same data and returns a flag byte depending on whether they match.
- `interleave_two_bitplane_words`: It takes the even-positioned bits from the first longword, shifted left one bit, and the odd-positioned bits from the second longword, then ORs them into a single combined longword.
- `calculate_xor_checksum_masked`: It XOR-accumulates a run of longwords whose count comes from a passed value, advancing a pointer each iteration, then returns the running value masked with the alternating-bit pattern.
- `shift_bits_across_long_pair`: It shifts the first longword left and the following word right by the same count, then merges the two shifted halves into a single 32-bit result.
- `call_self_three_times`: Performs three sequential calls to the same routine and returns without altering any data itself.
- `store_masked_bitplane_word`: It builds a value from the complemented alternating-bit pattern of the input combined with a shifted neighbour, clears the top bit when bit 0 of the preceding byte is set, and stores the result through the pointer.
- `search_patterned_table`: It scans a table of paired short values, matching a run of -23462 and then comparing successive longwords against a table until a match is found, returning the associated index or -1 on exhaustion.
- `conditional_blitter_setup`: Branches on a register value, loading and storing a word through a pointer on one path, and on the other path programs the blitter control, size and pointer registers for a decode pass before calling a routine.
- `start_blitter_decode_pass`: Programs the blitter control, size and pointer registers for a decode pass, using a shifted parameter and a computed size, then calls a routine.
- `initialise_disk_controller_fields`: It writes a DMA control word and an alternating fill pattern into fixed offsets of a control block, then clears three state fields within that block.
- `start_disk_dma_read`: Programs the disk controller for an MFM read, stores the buffer pointer and transfer length, enables disk DMA, and starts a read of a fixed length.
- `start_disk_dma_write`: Polls a disk status bit and, when the transfer-complete flag is set, programs the disk control register for write precompensation, enables disk DMA channels, and starts a write of a fixed length.
- `wait_disk_transfer_done`: It polls a status word for a fixed number of passes, and when the transfer-complete bit is set it adjusts two fields and returns zero, otherwise it clears a field and returns -1 after the timeout.
- `tiny_moveq`: Small routine of 1 instruction that loads 255 into d1.
- `deselect_drive_bit`: Reads a byte from register D1 and clears a single bit in the CIAB port B register, with the bit position selected by a stored drive index.
- `wait_drive_ready_status`: Sets up a status register and waits for a disk status bit to clear within a fixed count, returning zero on success and an error code on timeout.
- `seek_drive_to_track`: Reads the stored track position from a global variable and compares it with a target track passed in a register.
- `wait_disk_status_clear`: Waits for a status bit in a global variable to clear, polling a CIAB port and decrementing a counter each iteration, and returns a fixed value if the counter runs out.
- `read_ciab_port_set_bit`: Reads a byte from a CIAB port and, when a flag indicates a positive value, clears one of its bits.
- `read_ciab_port_delay`: Stores a value read from a CIAB port into a global variable, then spins in a loop that counts down from fifteen thousand before returning.
- `decrement_to_zero`: Decrements a counter in a register until it reaches zero, acting as a short busy-wait delay loop.
- `read_ciab_port_mask`: Reads a byte from a CIAB port and combines it with a bit mask derived from a shift amount.
- `setup_blitter_and_decrement_2`: Waits for a flag bit to clear, then programs custom graphics and blitter registers, zeroes a run of longwords in a global array, and sets up disk and interrupt control registers.
- `handle_input_button_decrement`: Reads a joystick or mouse button state and, when a button is pressed, decrements a per-button counter and adjusts a numeric value stored in a table.
- `setup_blitter_and_decrement`: Stores a fixed value into a global variable, sets a blitter pointer register, calls a routine, then decrements a counter byte and returns two input values.
- `copy_strided_byte_pairs`: Copies seven source bytes into a destination, writing each byte at a stride of 60 bytes and also at an offset 46 bytes further on within a destination buffer.
- `call_routine_sixteen_times`: Loops calling another routine sixteen times, then returns that routine's last result incremented by one.
- `init_and_wait_flag_bit_2`: Initialises a structure with a set of fixed control and pointer values, then polls a byte in that structure and loops until bit 6 of that byte is set.
- `init_and_wait_flag_bit`: Initialises a structure with a set of fixed control and pointer values, then polls a byte in that structure and loops until bit 6 of that byte becomes clear.
- `wait_while_flag_bit_set`: Polls a byte in a structure and loops until bit 6 of that byte becomes clear.
- `match_byte_counter_adjust`: Reads a byte from a global variable and from a location offset from a pointer, then stores the pointer's byte elsewhere.
- `decode_top_bit_byte_stream_2`: Looks up a type byte in a table indexed by a value, and for type one converts a number into its decimal digit characters stored into a global variable, with other types handled separately.
- `decode_top_bit_byte_stream`: Reads a byte stream from a pointer, branching on whether each byte has its top bit set, and writes the decoded bytes to a destination.
- `copy_bytes_with_strided_pairs`: Copies seven source bytes into a destination, writing each byte at a stride of 60 bytes and also at an offset 46 bytes further on within a destination stride of 92 bytes.
- `copy_bytes_at_strided_offsets`: Copies seven source bytes into a destination, advancing the source by 60 bytes and the destination by 46 bytes on each iteration.
- `build_pointer_table_from_strings`: Walks a run of zero-terminated strings starting at a fixed address and stores a pointer to each string into a table at a fixed address.
