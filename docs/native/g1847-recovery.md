# Game 1847 (f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01847). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 24 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 17 of 17 units identical (bytes and relocations), 28 raw-word lines |
| Functions named and described | pass, 49 of 49 named (0 author symbols), 49 of 49 described (4 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 188 of 188 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 146678 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1066 of 1076 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 163848 |
| code | 3186 |
| data | 683795 |
| padding | 4787 |
| unused | 45568 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:library | 3 | 17 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:frontend | 3 | 11 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:type | 2 | 117 | 2 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:GOLLUM! | 3 | 26 | 2 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:assign | 4 | 18 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:f15 | 3 | 16 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:end | 3 | 9 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:select | 3 | 18 | 2 |
| explode.library | 1 | 402 | 25 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:game | 3 | 19 | 2 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:Crystal | 1 | 166 | 6 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:wait | 2 | 117 | 2 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:bootblock | 1 | 138 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| LB.WLD (raw code file, load address unknown) | 1 | 0 | 0 |
| PG.WLD (raw code file, load address unknown) | 1 | 0 | 0 |
| CE.WLD (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:F15II(scenario)/LB.WLD: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:F15II(scenario)/PG.WLD: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:F15II(scenario)/CE.WLD: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:frontend: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:f15: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:select: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 3 never-loaded region(s), 146678 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `clear_memory_then_setup_display` (Crystal): Program entry that sets several global values, clears a block of longwords from a fixed address up to a fixed end address, and makes six routine calls. Kind: loader/protection loop. Pass path: the loop breaks when u10__d0_0570 == '@', proceeding to display setup.

## Functions


### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:library

- `open_explode_library_loop`: Program entry that repeatedly calls a library-open routine until it returns a non-zero library pointer, then makes one more library call.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:frontend

- `open_library_loop_then_halt`: Repeatedly calls a library-open routine until it returns a non-zero library pointer, then makes one library call.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If a pointer argument is non-null, copies sixteen longwords from a table at a fixed offset of a base register into it.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:GOLLUM!

- `open_dos_library_then_run`: Program entry that opens the dos library by name, makes three library calls, then one further call, and returns zero.
- `wait_while_flag_bit_set`: Busy-waits while bit six of a global byte stays set, then returns.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:assign

- `open_library_then_bad_tail`: Program entry that calls a library-open routine until it returns a non-zero library pointer, makes one library call, and then performs a no-op byte copy.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:f15

- `open_library_then_store_flags`: Program entry that opens a library by calling a library-open routine until it returns a non-zero pointer, then makes one library call and stores a value into a table.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:end

- `open_library_bad_tail`: Calls a library-open routine until it returns a non-zero library pointer, makes one library call, and then reaches bad instruction data.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:select

- `open_library_call_bad_tail`: Calls a library-open routine until it returns a non-zero library pointer, makes one library call and one further routine call, and then reaches bad instruction data.
- `return_from_subroutine`: The routine is empty and performs no operations.

### explode.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `init_explode_library_base`: Library initialisation routine in the explode library.
- `increment_library_open_count`: Adds one to a counter field inside the explode library base structure.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `scan_for_explode_signature`: Looks up an index through a library function and checks the table entry at that index for a fixed instruction pattern followed by the name explode.library.
- `increment_counter_and_call`: Increments a global counter in the explode library data, calls one routine, and returns the saved first argument.
- `find_task_set_priority_around`: Finds the current task, calls the priority-setting exec routine on it, makes routine calls, sets the priority again, and returns its first argument unchanged.
- `decode_packed_record_fields`: Fills a structure from a packed input record, copying nibble-coded fields, two longwords from a table, and words from the input.
- `decode_explode_command_loop`: Decompression loop in the explode library.
- `dispatch_explode_block_loop`: Outer loop of the explode library decompressor.
- `copy_explode_window_bits`: Bit-level decoder in the explode library.
- `shift_explode_data_blocks`: Performs a bitwise left shift on a series of data blocks by processing bytes through a series of carry-propagating operations.
- `copy_explode_runs_backward`: Explode library copy routine.
- `read_explode_bits_accumulator`: Explode library bit reader.
- `enter_explode_bit_decode_five`: Entry point into the explode library bit decoder that starts with a count of five and a field width of three.
- `enter_explode_bit_decode_four`: Entry point into the explode library bit decoder that starts with a count of four and a field width of two.
- `enter_explode_bit_decode_three`: Entry point into the explode library bit decoder that starts with a count of three and a field width of one.
- `enter_explode_bit_decode_two`: Entry point into the explode library bit decoder that starts with a count of two and no extra field width.
- `expand_explode_bit_fields`: Explode library routine that expands bit fields from the input into the output.
- `copy_explode_bit_field_chunks`: Explode library routine that copies bit-field chunks from the input window into the output, shifting control bits as it goes and calling one routine for each chunk.
- `fill_explode_output_span`: Explode library routine that loops over a count field and fills output bytes through two routines, shifting control bits and adding copied values into the output.
- `store_explode_state_record`: Explode library routine that stores the current control and bit values into a state record, then loops over the input.
- `release_explode_buffer_memory`: Loops over a count field, calling a routine and a library function on each pass.
- `add_into_global_total`: Adds a register value into a global running total and returns.
- `delay_countdown_loop`: Counts a register down from its starting value to minus one as a delay, then returns.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:game

- `open_library_bad_data_end`: Program entry that calls a library-open routine until it gets a non-zero result and makes one more call.
- `modify_explosion_parameters`: Updates a memory location via a pointer using a negation and addition, decrements a second value at an offset, and performs a bitwise left shift on a value at a third offset.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:Crystal

- `clear_memory_then_setup_display`: Program entry that sets several global values, clears a block of longwords from a fixed address up to a fixed end address, and makes six routine calls.
- `copy_74_bytes_low_memory`: Copies 74 bytes from a block of code into a fixed low-memory area and returns.
- `patch_branch_return_when_flagged`: When a global flag is set, overwrites several instruction words with a short branch and a return instruction, then clears a byte at another offset.
- `update_keyboard_state_byte`: Reads a global keyboard-state byte, clears related bits, then computes an inverted, shifted value and stores it back when the shifted value is zero.
- `fill_word_pairs_from_source`: Writes 48 pairs of 16-bit words into a buffer at a fixed address.
- `draw_string_glyphs_into_plane`: Draws each character of a zero-terminated string by copying eight glyph bytes from a font table into a buffer, advancing 40 bytes per row across a 320-pixel-wide plane.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:wait

- `scan_linked_table_for_max`: Program entry that sets up a 140-byte stack buffer and calls one routine through a register.
- `copy_sixteen_longs_from_table`: If a pointer argument is non-null, copies sixteen longwords from a table at a fixed offset of a base register into it.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `set_boot_library_request`: Boot-block routine called from the boot entry.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.
