# Game 1347 (treasure trap (1989)(electronic zoo)[cr flt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01347). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909504 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 60 of 60 named (0 author symbols), 60 of 60 described (3 by rule, 9 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 195 of 195 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2048 bytes, 23 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1212 of 1316 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 5967 |
| code | 4912 |
| data | 896460 |
| padding | 117 |
| unused | 2048 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Treasure Trap (1989)(Electronic Zoo)[cr FLT].adf:bootblock | 1 | 119 | 10 |
| logged disk load from Treasure Trap (1989)(Electronic Zoo)[cr FLT].adf boot-block load | 1 | 286 | 17 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 196 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 164 | 2 |
| Treasure Trap (1989)(Electronic Zoo)[cr FLT].adf bytes 1024-3072 (code never loaded by a reference run) | 1 | 551 | 23 |

## Cases handled

- treasure trap (1989)(electronic zoo)[cr flt]/Treasure Trap (1989)(Electronic Zoo)[cr FLT].adf: custom-format (non-DOS) disk, 1752 non-empty blocks: matched against track-loaded segments
- treasure trap (1989)(electronic zoo)[cr flt]/Treasure Trap (1989)(Electronic Zoo)[cr FLT].adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- runtime segment an address: 141 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 54 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- treasure trap (1989)(electronic zoo)[cr flt]/Treasure Trap (1989)(Electronic Zoo)[cr FLT].adf: bytes 1024-3072 look like code no run loaded: recovered position-independently (load address unknown)
- treasure trap (1989)(electronic zoo)[cr flt]/Treasure Trap (1989)(Electronic Zoo)[cr FLT].adf: 2 logged loads cover 7680 bytes of the image
- treasure trap (1989)(electronic zoo)[cr flt]/Treasure Trap (1989)(Electronic Zoo)[cr FLT].adf: blocks 10-10 (bytes 5120-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- 1 packed track region(s), 889856 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `set_cia_port_b_wait` (Treasure Trap (1989)(Electronic Zoo)[cr FLT].adf bytes 1024-3072 (code never loaded by a reference run)): Writes an inverted combination of a shared selector and a fixed mask to the CIA port B register. Kind: disk hardware check (CIAB_PRB). Pass path: the branch where the condition is met (non-zero/non-FFFF, or the loop continues/skips depending on the specific check logic, but specifically the bypass of the error/halt state or the successful validation of the register/port state or bit-.
- `step_cia_port_b_down` (Treasure Trap (1989)(Electronic Zoo)[cr FLT].adf bytes 1024-3072 (code never loaded by a reference run)): Writes an inverted value built from a shared selector and a different fixed mask to the CIA port B register, calls another routine, and decrements a shared counter. Kind: disk hardware/register check (CIAB_PRB). Pass path: the routine checks a specific bit/state in CIA Port B (CIAB_PRB) or a related hardware register, where a successful check (the presence of a specific disk/track or hardware signal) allows the program to proceed to the next instruction (e.g..
- `step_cia_port_b_up` (Treasure Trap (1989)(Electronic Zoo)[cr FLT].adf bytes 1024-3072 (code never loaded by a reference run)): Writes an inverted value built from a shared selector and another fixed mask to the CIA port B register, calls another routine, and increments a shared counter. Kind: disk hardware check (CIAB_PRB). Pass path: the code following the check (the logic that continues if the bitmask/condition or hardware state permits or if the bit is set, or specifically the branch not taken/taken depending on the hardware presence or lack thereof, though here it ac.
- `read_disk_track_via_dma` (Treasure Trap (1989)(Electronic Zoo)[cr FLT].adf bytes 1024-3072 (code never loaded by a reference run)): Configures the disk controller for a DMA transfer by setting the sync word, the disk pointer and length, the ADKCON control and the DMA control register. Kind: track/sync/checksum check. Pass path: the loop completes, checksums match, and the function returns the calculated sVar1 (total bytes read/processed).

## Functions


### Treasure Trap (1989)(Electronic Zoo)[cr FLT].adf:bootblock

- `boot_copy_code_low_memory`: Boot block entry: it sets fields of an I/O request, makes one library call, copies a block of code to low memory, calls two routines, and then continues into further setup.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `process_zero_terminated_string`: Walks a zero-terminated string, calling a helper for each character until the terminator, then returns zero.
- `process_string_until_terminator`: Walks a zero-terminated byte string through a pointer, calling a per-character routine for each byte until the terminating zero is reached.
- `return_registers`: returns the contents of two even-numbered or even-split registers as a single 32-bit or 64-bit value.
- `open_trackdisk_device_and_setup`: Allocates memory and a signal, locates the current task, and opens the trackdisk device through the exec library, then issues an I/O request on it.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `relay_two_routine_calls`: The decompiled body contains only a return, yet the listing records two outgoing calls and two callers.
- `update_dma_state`: Executes a routine that writes constant words to a shared area and clears two bytes in a shared state.
- `update_dma_channel_state`: Stores fixed constant words into a shared area and clears two shared state bytes.

### logged disk load from Treasure Trap (1989)(Electronic Zoo)[cr FLT].adf boot-block load

- `initialise_display_and_copper`: Clears the shared state area and a block of screen memory, then programs the bitplane modulo, the background and palette colour registers, the copper list pointer and the DMA control register.
- `step_state_and_fill_rows`: Steps a shared state value up or down, toggles two flag bits, and looks up a value in a character table.
- `fill_rows_from_next_entry`: Adjusts a shared state byte and indexes the character table one entry forward.
- `fill_rows_from_previous_entry`: Adjusts a shared state byte and indexes the character table one entry backward.
- `scroll_row_words_horizontally`: Shifts word-sized data within eight rows of forty words by two words each step.
- `decrement_counter_store_constants`: Decrements a frame counter kept in the shared state, and when it reaches zero resets it to eight and calls another routine.
- `poll_then_blit_glyph_rows`: Polls a source byte until it is nonzero, then copies the eight-byte bitmap for that character into screen memory.
- `poll_then_blit_glyph_cell`: Waits in a loop until the source byte is nonzero, advances the source pointer, and copies the eight-byte bitmap for that character to screen memory.
- `blit_glyph_with_stride`: Copies the eight-byte bitmap for a character code, found by subtracting the space code and scaling by eight, into a destination pointer.
- `tiny_moveq`: Small routine of 1 instruction that loads 120 into d1.
- `visit_each_text_character`: Scans forward through a zero-terminated text string and calls a per-character routine for every byte before the terminator.
- `do_nothing`: The routine is empty and performs no operations before returning.
- `decode_two_bit_tile_stream`: Reads packed input bytes two bits at a time.
- `resume_two_bit_stream_decode`: Continues the two-bit packed decode from saved input and output pointers, copying stored blocks for each nonzero symbol.
- `init_audio_channels_muted`: Points the first two audio channels at a shared sample area, sets each channel length to eight words, and sets both volumes to zero.
- `run_audio_period_setup_pair`: Calls two other routines in sequence and returns the data register unchanged.
- `advance_sound_sequence_step`: Decrements a countdown held in a state block and, when it expires, advances a sequence index, reads the next entry from a lookup table and stores the looked-up word through a caller-supplied pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_mouse_port_input`: Reads the mouse-port movement counter and the fire-button state, computes horizontal and vertical movement from earlier readings, and updates shared position and button flags.
- `push_table_words_until_zero`: Reads and clears a flag word, selects one of two fixed tables by the flag's low bit, then pushes words from that table onto the stack until a zero word has been copied.
- `increment_shared_counter`: Adds one to a shared counter variable and returns.
- `increment_counter_return_zero`: Adds one to the same shared counter variable and returns zero.
- `copy_mode_table_block`: Compares a shared mode value with six.
- `copy_fixed_table_six_pairs`: Copies six longword pairs from one of two fixed tables into a shared area, choosing the table from a low bit of the shared mode value.
- `return_void`: the routine is empty and performs no operations before returning.
- `decode_joystick_port_direction`: Reads the joystick port data word and decodes its direction bits through a lookup table into a shared state byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_save_disk_status`: Checks for a valid high score disk and updates a global buffer with string data if a specific signature is detected.
- `fetch_next_table_byte`: reads a byte from a table or an offset table and increments a global index.

### Treasure Trap (1989)(Electronic Zoo)[cr FLT].adf bytes 1024-3072 (code never loaded by a reference run)

- `bootstrap_forbid_and_copy`: Calls exec library functions to forbid task switching and enter supervisor state, then calls further routines and writes the DMA control register.
- `copy_block_to_low_memory`: Copies one thousand twenty-four longwords from a source pointer into the low memory vector area, then returns a success value.
- `read_bootcode_bytes_loop`: Initialises two shared words, then calls a routine in a loop and stores each returned byte in a buffer until a negative result is returned.
- `walk_string_until_terminator`: Walks a zero-terminated string through a pointer, calling a per-character routine for each byte before the terminating zero.
- `call_another_routine`: The routine calls another routine and returns the unmodified contents of two data registers.
- `chain_five_routine_calls`: The decompiled body shows one call with the data registers passed back, while the listing records five outgoing calls.
- `merge_alternating_bit_planes`: Walks a block of words, masking alternate bits from two source streams with an alternating-bit pattern and combining them, with one stream shifted left by one bit.
- `merge_alternating_bit_planes_alt`: Performs the same alternating-bit combine as the neighbouring entry, so it is a second entry point into the same operation.
- `merge_alternating_bit_words`: Performs the same alternating-bit combine of two word streams, shifting one left by one bit and writing the result to a destination block.
- `merge_alternating_bit_block`: Performs the same alternating-bit combine of two word streams into a destination block, with the same mask and shift as its neighbours.
- `test_and_reset_status_flag`: Checks bit one of a shared status word and, when it is set, rewrites the word keeping only that bit.
- `initialise_cia_timer_and_wait`: Writes two configuration values to CIA timer registers, waits in a loop until one status bit becomes set, then writes a final control value.
- `set_cia_port_b_wait`: Writes an inverted combination of a shared selector and a fixed mask to the CIA port B register.
- `step_cia_port_b_down`: Writes an inverted value built from a shared selector and a different fixed mask to the CIA port B register, calls another routine, and decrements a shared counter.
- `step_cia_port_b_up`: Writes an inverted value built from a shared selector and another fixed mask to the CIA port B register, calls another routine, and increments a shared counter.
- `wait_status_then_read_byte`: Calls a routine repeatedly while a shared status bit stays set, then calls another routine and returns a byte from it only if the bit is still set.
- `read_disk_track_via_dma`: Configures the disk controller for a DMA transfer by setting the sync word, the disk pointer and length, the ADKCON control and the DMA control register.
- `compare_strings_ignoring_case`: Compares two zero-terminated strings one byte at a time, converting lowercase letters to uppercase first.
- `cycle_background_colour_or_poll`: If a shared flag is set, it repeatedly writes the background colour register with alternating values for a fixed long count.
- `step_serial_table_byte`: Decrements a countdown and, while it is nonzero, reads the next byte from a table and advances the table index.
- `chain_two_routine_calls`: Calls two other routines in sequence and returns.
- `do_nothing_2`: This routine is empty and performs no operations before returning.
- `empty_routine`: the routine contains no executable instructions and performs no operations.
