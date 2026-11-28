# Game 2542 (nitro (1990)(psygnosis)[cr hz][t +5 full - evil]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02542). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906480 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 39 of 39 named (0 author symbols), 39 of 39 described (2 by rule, 15 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 429 of 429 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 975 of 975 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 21445 |
| code | 3110 |
| data | 597765 |
| unused | 284160 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil].adf:bootblock | 1 | 7 | 1 |
| logged disk load from Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil].adf boot-block load | 1 | 127 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 841 | 36 |

## Cases handled

- nitro (1990)(psygnosis)[cr hz][t +5 full - evil]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil].adf: custom-format (non-DOS) disk, 1180 non-empty blocks: matched against track-loaded segments
- nitro (1990)(psygnosis)[cr hz][t +5 full - evil]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil].adf: the boot block reads 20480 bytes from disk offset 606208 to an address (found statically); recovered as a unit
- runtime segment an address: 429 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- nitro (1990)(psygnosis)[cr hz][t +5 full - evil]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil].adf: 1 logged loads cover 20480 bytes of the image
- nitro (1990)(psygnosis)[cr hz][t +5 full - evil]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil].adf: blocks 11-11 (bytes 5632-6144) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.02)
- 1 packed track region(s), 594944 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `copy_disk_blocks_2542f7` (runtime segment an address): Saves registers, disables disk controller settings, obtains transfer values, copies partial and full 512-byte blocks through a2 and a1, restores control, and returns zero. Kind: disk hardware check. Pass path: The routine reads from disk using CIAB_PRB and DSKLEN, and performs a copy of data with checks that suggest it's validating disk integrity or detecting copied tracks. The use of ADKCONR and ADKCON suggests this is interacting with the Amiga.

## Functions


### Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil].adf:bootblock

- `init_and_copy_strings`: Initializes a set of fixed values in a memory structure, then calls a startup routine.

### logged disk load from Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil].adf boot-block load

- `tiny_clr`: Small routine of 1 instruction that clears d0.
- `xor_byte_pattern`: Walks through a 32-bit value one byte at a time, toggling a global variable with the XOR of the current byte whenever that byte is zero, then decrements the value until it wraps past zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_coprocessor_dispatch`: Waits for a copper-list interrupt, then calls a series of library functions through pointers.
- `copy_memory_and_dispatch`: Copies a fixed block of memory into the system's global variable area, then reads a byte from a known location and repeatedly dispatches to another routine based on that byte's value while decrementing a counter and copying data through pointers.
- `clear_memory_block`: It clears a block of memory by writing zero bytes to consecutive words until a pointer reaches a limit, then returns.
- `load_text_table_row_2542f1`: Computes a table position from two a5 words, reads bytes through a0 until a negative marker, subtracts 32, calls a loader, then increments a row word.
- `load_table_values`: Casts an index value to a signed byte and stores a byte from a global table into the first byte of a destination buffer.
- `count_null_bytes`: Walks through a string until it reaches a null byte, counting the characters along the way.
- `update_string_index`: Examines a control byte and then scans a string to locate a non-zero byte, moving forward or backward depending on its value.
- `decrement_and_wrap_counter`: Decrements a byte in a structure and, if it wraps to zero, resets it and adjusts a counter up or down based on another byte.
- `parse_digit_fields_2542f2`: Initializes pointers and two a5 words, converts source character digits into bytes at a3 until zero, calls a counter, then branches onward.
- `finalize_timer_fields_2542f3`: Conditionally copies a byte and calls the text loader repeatedly, writes four derived bytes and two state words in a5, then loads another text sequence.
- `process_timer_command`: Decrements a timer byte and, when it reaches zero, resets it to four and prepares a flag.
- `scan_command_slots_2542f4`: Scans twelve bytes from a5, classifies values one and two by branching, advances d3 by nine for other values, and returns after d2 expires.
- `load_type_one_command_2542f5`: Points a0 at an a5-relative text area, writes d4 and d3 into two a5 state words, calls the text loader, then branches back.
- `select_type_two_text_2542f6`: Loads a0 with another a5-relative text address and branches directly into the shared command-loading path without calling or returning here.
- `copy_disk_blocks_2542f7`: Saves registers, disables disk controller settings, obtains transfer values, copies partial and full 512-byte blocks through a2 and a1, restores control, and returns zero.
- `read_disk_sector_2542f8`: Saves registers, computes track and side values, programs disk hardware through a4 and CIA bits through a5, retries transfers, updates a counter, and branches onward.
- `clear_drive_side_bit_2542f9`: Clears bit 2 in the byte addressed by a5 and branches directly back into the disk retry sequence.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `decode_track_sectors_2542f10`: Scans synchronization marks, merges alternating MFM bits, checks decoded data, copies 128 decoded longs to a3, adjusts a0 and d1, restores registers, and returns zero.
- `abort_decode_toggle_motor_2542f11`: Restores d2 through d7 and a1 through a3, discards one stack long, toggles bit 1 through a6, then branches to the disk sequence.
- `verify_sector_checksum_2542f12`: Saves d0, d1 and a1, combines two stored checksum longs, XORs 256 source longs, masks and compares the result, restores registers, and returns.
- `call_drive_pulse_fallthrough_2542f13`: Calls the nearby drive pulse target and then falls through into the following instruction stream without an explicit return.
- `set_direction_wait_ready_2542f14`: Sets bit 1 through a5, tests bit 4 through a6, branches to request another pulse while set, and otherwise returns.
- `pulse_drive_step_bit_2542f15`: Clears and sets bit 0 through a5, then branches to the shared delay and readiness path.
- `move_head_steps_2542f16`: Clears direction bit 1, adds d3 to a position word, reverses negative d3 and sets direction, then calls one pulse per remaining step.
- `prepare_long_delay_2542f17`: Pushes d0, loads its low word with 45056, then branches to the shared delay code without returning in this fragment.
- `delay_until_cia_ready_2542f18`: Pushes d0, loads 6000, calls the countdown, repeatedly tests bit 5 through a6 until clear, restores d0, and returns.
- `write_cia_timer_wait_2542f19`: Writes the low and high bytes of d0 to two a6 offsets, writes 25 to a control byte, waits for bit 0 to clear, and returns.
- `configure_drive_lines_2542f20`: Applies three masks to the byte through a5, clears bits 7 and 3, calls the long delay path, and returns.
- `reset_drive_lines_timer_2542f21`: Applies three masks to the byte through a5, loads d0 with 45056, then branches to the shared CIA timer writer.
- `decode_encoded_stream`: Decrements a counter in a data structure and, when it reaches zero, reloads it and invokes four helper routines to decode a length-prefixed string and interpret a stream of encoded bytes while updating channel and instrument state.
- `decode_length_prefixed_string`: Decrements a counter stored in a data structure and then decodes a length-prefixed string from a table, advancing a pointer by the decoded length and updating several associated fields.
- `process_encoded_stream`: Decrements a counter and interprets a stream of encoded bytes, updating channel and instrument state through helper routines.
- `update_table_coordinate_from_entry`: Reads a table entry selected by an offset and updates a coordinate value plus a size value stored through a pointer, then adjusts a direction index from a nibble of the entry.
- `update_channel_period_and_vibrato`: Updates a channel's period and vibrato state from its instrument entry, applying a counter-driven sweep and a sign flip, then stores the new period and volume.
- `store_capped_display_values_2542f22`: Preserves d0, d1 and a0, masks d1 to four bits, caps d0 at 64, stores both words through a0, restores registers, and returns.
