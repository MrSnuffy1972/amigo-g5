# Game 1533 (gem'x (1991)(demonware)[cr][t +3 gin]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01533). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911216 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 41 of 41 named (0 author symbols), 41 of 41 described (5 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 514 of 514 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 961 of 961 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4614 |
| code | 3822 |
| data | 558724 |
| padding | 1016 |
| unused | 343040 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Gem'X (1991)(Demonware)[cr][t +3 GIN].adf:Game | 1 | 481 | 20 |
| Gem'X (1991)(Demonware)[cr][t +3 GIN].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 462 | 20 |

## Cases handled

- reference run failed (Karl 4 Nov 2026): run play-long-joy slid through memory (an address: 2 64 KB pages each with >= 29491 distinct PCs executed, 31745 of its 32343 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-long-keys slid through memory (an address: 2 64 KB pages each with >= 29491 distinct PCs executed, 31745 of its 32343 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-long-mixed slid through memory (an address: 2 64 KB pages each with >= 29491 distinct PCs executed, 31745 of its 32343 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 177 64 KB pages each with >= 29491 distinct PCs executed, 5766138 of its 6114259 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 177 64 KB pages each with >= 29491 distinct PCs executed, 5766138 of its 6114259 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 177 64 KB pages each with >= 29491 distinct PCs executed, 5766138 of its 6114259 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 177 64 KB pages each with >= 29491 distinct PCs executed, 5766138 of its 6114259 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- runtime segment an address: 290 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- gem'x (1991)(demonware)[cr][t +3 gin]/Gem'X (1991)(Demonware)[cr][t +3 GIN].adf:Game: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Gem'X (1991)(Demonware)[cr][t +3 GIN].adf:Game

- `program_entry_setup_display`: Serves as the program entry.
- `close_handle_and_reply`: When a stored handle is nonzero, closes a DOS handle, forbids task switching and replies to a message.
- `call_graphics_library_function`: Calls a graphics library function at a fixed offset from a stored library base and returns.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `write_text_via_dos`: Returns at once when a control word is zero.
- `conditionally_flush_cpu_caches`: Calls a library routine through a pointer read at a fixed offset when a word in a structure exceeds a threshold, then calls a second library routine.
- `write_dma_and_interrupt_control`: Writes to the DMA control and interrupt enable registers, where bit 15 of the written value selects enable or disable.
- `pass_control_to_routine`: A two-instruction routine that only passes control on to one other routine.
- `set_background_color_and_dma`: Writes the background color register and the DMA and interrupt enable registers, and calls three other routines.
- `setup_imp_marker_and_hardware`: Refers to the text IMP! and touches two custom hardware registers, and calls three other routines.
- `select_and_copy_loader_image`: Checks a byte in a header and branches to one of two paths: one copies a block of longwords to a fixed destination, the other matches a name against a table, uppercases a string, copies a larger block and patches words according to a flag.
- `copy_block_and_patch_jump`: Copies a block of longwords to a fixed destination, calls one routine, then writes words chosen by the bits of a flag, followed by a jump-style word and a routine address.
- `reset_cia_serial_interrupt_state`: If a flag bit is set, it sets another bit, calls one routine and clears that bit again.
- `send_keyboard_serial_sequence`: Calls one routine, then repeatedly writes two fixed byte values to the keyboard serial register, waiting on a status bit after each write.
- `update_key_state_from_byte`: Updates a 16-bit state word from one input byte: certain specific byte values set the state or XOR it with fixed constants, and the state is then replaced by a bit-rotated form of the inverted byte.
- `install_header_routine_and_copy`: Stores a routine address and a byte from a header structure, uppercases a string, searches a table, copies a block of longwords and then calls one routine.
- `copy_matched_table_block`: Searches a table of entries by comparing bytes of a name, copies a block of longwords for the matched entry, then calls one routine.
- `copy_longword_block_to_destination`: Looks up a source offset from a table entry, copies longwords to a destination until the source end is reached, then calls one routine.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `unpack_imp_packed_data`: Checks for the IMP! magic word and, when it matches, unpacks the packed data backwards by reading a bit stream and copying runs of bytes.

### Gem'X (1991)(Demonware)[cr][t +3 GIN].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `install_temporary_interrupt_handler`: Saves a few system values, clears a block of longwords, installs an interrupt handler routine, and waits in a loop on a flag before restoring the saved values.
- `open_graphics_and_reset_hardware`: Opens the graphics library by name and closes it again, and writes to the audio, DMA and interrupt control registers, including the copper location registers.
- `read_masked_interrupt_request_bits`: Reads the interrupt request register, masks the result with a fixed bit pattern and stores it in a global variable.
- `run_next_nonempty_table_handler`: Steps a stored index backwards through a table in eight-byte steps until it finds a non-empty entry, calls that entry as a routine, and advances the index by one entry when the result is non-zero.
- `adjust_word_nibbles_toward_target`: Walks two arrays of 16-bit words together and increments each word of the first array in steps of 1, 16 or 256 until each nibble matches the second array.
- `decrement_word_nibbles_toward_target`: Walks two arrays of 16-bit words together and decrements each word of the first array in steps of 1, 16 or 256 until each nibble matches the second array.
- `run_input_state_machine`: Runs a small input state machine that uses counters, bit toggles and comparisons, and reads the fire button, mouse movement and right and middle button registers.
- `stub_single_instruction_entry`: A one-instruction entry whose decompiled body is identical to the routine that follows it, so the decompiler output is unreliable at this address.
- `update_mouse_position_counter`: Clamps a counter between zero and a fixed maximum, stores a shifted copy of it in a byte, and writes a fixed word pattern to a small block with one indexed word replaced.
- `copy_text_glyphs_if_pending`: Converts each character of a text string into eight-row glyph columns copied from a font table into a bitplane buffer, moving to a new line after a fixed number of characters.
- `tiny_ori_2`: Small routine of 1 instruction that merges 1 into d1.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into (a0).
- `copy_text_glyphs_when_idle`: If a state byte is not already one, it sets it and, unless a completion flag is set, copies the glyph columns for each character of the text string into a buffer.
- `copy_text_glyphs_to_buffer`: Copies the glyph columns for each character of a null-terminated text string into a buffer, moving to a new line after a fixed number of characters.
- `advance_frame_counter_and_fill`: Increments a counter and, once it passes three, calls another routine, copies a few global words, writes a block of words from a global value and increments a byte counter.
- `fill_block_and_increment_counter`: Calls one routine, copies a few global words, writes four words from a global value into a block, and increments a global counter.
- `open_cia_resources_and_libraries`: Opens the named resources b.resource and ciab.resource and the graphics library, and closes the library again, touching the port registers of both CIA chips along the way.
- `call_library_offset_entry`: Calls one library function through a fixed jump-table offset and is called by two routines.
- `build_linked_slot_table`: Scans a byte table until a terminating value, then builds a set of linked slots by zeroing words and storing pointers to them, and finally sets flag bits and clears several words.
- `clear_audio_dma_channels`: The routine writes zero to four audio volume registers and to a global variable, then writes a non-zero value to the same global variable and writes to DMACON and a custom hardware register.
