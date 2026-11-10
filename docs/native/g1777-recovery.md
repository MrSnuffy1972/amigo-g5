# Game 1777 (cannon fodder (1993)(virgin)[cr flt](disk 1 of 3)[t +4 pdx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01777). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904992 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 42 of 42 named (0 author symbols), 42 of 42 described (1 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 815 of 815 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1168 of 1168 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 16027 |
| code | 4086 |
| data | 873089 |
| padding | 14 |
| unused | 11776 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3)[t +4 PDX].adf:bootblock | 1 | 29 | 1 |
| logged disk load from Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3)[t +4 PDX].adf boot-block load | 1 | 155 | 3 |
| logged disk load from Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3)[t +4 PDX].adf boot-block load | 1 | 116 | 2 |
| logged disk load from Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3)[t +4 PDX].adf boot-block load | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 844 | 35 |

## Cases handled

- cannon fodder (1993)(virgin)[cr flt](disk 1 of 3)[t +4 pdx]/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3)[t +4 PDX].adf: custom-format (non-DOS) disk, 1734 non-empty blocks: matched against track-loaded segments
- extended reference run ignored: it does not re-execute 400 of the 788 PCs the canonical run logged (121382 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 6 64 KB pages each with >= 29491 distinct PCs executed, 98304 of its 121382 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- cannon fodder (1993)(virgin)[cr flt](disk 1 of 3)[t +4 pdx]/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3)[t +4 PDX].adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- cannon fodder (1993)(virgin)[cr flt](disk 1 of 3)[t +4 pdx]/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3)[t +4 PDX].adf: the boot block reads 5632 bytes from disk offset 2048 to an address (found statically); recovered as a unit
- cannon fodder (1993)(virgin)[cr flt](disk 1 of 3)[t +4 pdx]/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3)[t +4 PDX].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 666 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- cannon fodder (1993)(virgin)[cr flt](disk 1 of 3)[t +4 pdx]/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3)[t +4 PDX].adf: 3 logged loads cover 17920 bytes of the image
- cannon fodder (1993)(virgin)[cr flt](disk 1 of 3)[t +4 pdx]/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3)[t +4 PDX].adf: blocks 22-23 (bytes 11264-12288) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.55)
- cannon fodder (1993)(virgin)[cr flt](disk 1 of 3)[t +4 pdx]/Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3)[t +4 PDX].adf: blocks 1474-1737 (bytes 754688-889856) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.70)
- 1 packed track region(s), 734208 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3)[t +4 PDX].adf:bootblock

- `load_and_unpack_boot_payload`: Boot-block entry that loads a block from disk with an operating system I/O request, then unpacks it backward into memory with a bit-stream decoder until a fixed boundary.

### logged disk load from Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3)[t +4 PDX].adf boot-block load

- `decode_bitpacked_block`: Copies a block of bytes from a source to a destination, then reads a bit-packed stream and decodes it into a destination buffer, storing one byte per decoded value until the source is exhausted.
- `read_bit_word_store_byte`: Reads the longword just before the address pointer, stores a byte built from a register and the low byte of that longword through a second pointer, and returns the longword shifted right one bit with its top bit set.
- `shift_bits_refilling_from_word`: Shifts a bit buffer right one bit per step for a count held in a register, and calls a routine to refill the buffer whenever it becomes zero.

### logged disk load from Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3)[t +4 PDX].adf boot-block load

- `return_half_or_input_word`: Returns one word shifted right by one bit when a second register holds the value one, and otherwise returns the input word unchanged.
- `lookup_table_word_masked`: Reads a word from a fixed table in memory, indexed by a register value doubled, and ANDs it with a second register before returning it.

### logged disk load from Cannon Fodder (1993)(Virgin)[cr FLT](Disk 1 of 3)[t +4 PDX].adf boot-block load

- `copy_stack_then_unpack_payload`: Copies a block of longwords from a fixed address into a buffer, then unpacks a compressed block backward into memory with a bit-stream decoder until a fixed boundary, and zero-fills a region afterward.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_input_loop_with_copper`: Sets a copper list address and clears a fixed memory region, then loops until a control flag is set, reading the mouse port movement register and the vertical beam position on each pass.
- `assemble_handler_from_flagged_parts`: Copies a fixed leading fragment of code into a buffer, appends three optional fragments chosen by flag bytes, and then copies a fixed trailing fragment.
- `assemble_handler_from_flagged_parts_2`: Copies a fixed leading fragment into a buffer, appends optional code fragments chosen by flag bytes, and then copies a fixed trailing fragment.
- `assemble_handler_with_marker_byte`: Stores a fixed marker byte into the buffer, then copies optional code fragments chosen by flag bytes and a fixed trailing fragment.
- `assemble_handler_with_fixed_header`: Stores a fixed header longword and a zero word into the buffer, then copies optional code fragments chosen by two flag bytes and a fixed trailing fragment.
- `assemble_handler_with_fixed_header_2`: Stores a fixed header longword and a zero word into the buffer, then copies an optional code fragment chosen by a flag byte and a fixed trailing fragment.
- `write_marker_and_copy_fragment`: Stores a fixed marker byte at an offset in the buffer, then copies a fixed-length code fragment into it.
- `write_jump_stub_and_call`: Writes a jump-to-subroutine instruction with a fixed target into a buffer, sets a global value, and then calls through a pointer the decompiler could not resolve.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `poll_status_then_clear_word`: Polls a status byte until it reads as set, calls one unnamed routine, and repeats while a counter is non-negative.
- `execute_timed_byte_stream`: Initializes a data structure and a loop to process a timed byte stream.
- `run_two_polled_transfers`: Initializes a transfer counter and a pointer, then runs two polling loops that each wait for a status byte before making one call.
- `initialize_record_bounds_and_flag`: Stores the constants zero, zero, 319 and 143 into four words of a record, copies two more words into it from a second record, and sets a flag byte to negative one.
- `adjust_bounds_and_emit_records`: Moves two pairs of bounds toward their targets by fixed step sizes and clamps them to limits.
- `configure_bounds_and_emit`: Sets a flag bit in a global variable and writes a fixed pattern to a field pointed to by an incoming pointer.
- `build_clamped_rectangle_records`: Computes the same clamped rectangle records around a stored point as the routine before it, without setting the leading control fields.
- `run_polled_send_loops`: Runs two loops that each wait for a status byte to read as set before making one call, repeating while a control byte is nonzero.
- `run_flagged_polled_segments`: Runs four polling loops in sequence, each waiting for a status byte before making one call.
- `run_timed_polled_loop`: Writes a fixed set of control values, runs one polled loop, then makes 200 polled calls that each wait for two status values in turn.
- `poll_until_global_bit_clears`: Writes a fixed control block, runs one polled loop, then repeats a double status wait and call while a bit in a global byte stays set.
- `poll_until_global_bit_clears_2`: Writes the same fixed control block and polled loop as the routine before it, with a different pointer offset, then repeats a double status wait and call while a bit in a global byte stays set.
- `configure_block_parameters_and_call`: Sets a pointer and four small parameters, makes several calls with differing pointers, and stores a fixed value in a control word.
- `poll_call_until_flag_clears`: Sets a flag byte to negative one, then repeatedly waits for a status byte and makes one call while the flag stays nonzero.
- `repeat_double_poll_call`: Repeats a count held in a register, waiting for two status values in turn and making one call on each pass.
- `set_pointer_and_flag_state`: Stores a pointer and two zero words into a control block, then sets a flag byte to negative one.
- `set_pointer_counts_and_flag`: Stores a pointer and two word values, 280 and 12, into a control block, then sets a flag byte to negative one.
- `tick_two_counters_and_toggle`: Decrements a counter, and while it stays non-negative also decrements a second counter.
- `decrement_counter_swap_pairs`: Decrements a counter stored in a global variable and, once it reaches zero, toggles a flag byte and swaps two pairs of values in a table.
- `reassign_words_then_call`: Reassigns two control words to themselves and then makes one call.
- `interpret_timed_byte_stream`: On each timer expiry, reads a zero-terminated byte stream in which a newline byte advances a row counter and any other byte makes one call and advances a column position.
- `write_block_registers_and_wait`: Writes four control values and a size word into a block of hardware registers, then waits while a busy bit is set.
- `draw_glyph_for_character`: Maps a character to a glyph index, with space, plus and underscore having fixed indices and digits and capital letters offset from their codes.
- `dispatch_rectangle_handler_for_point`: Walks a table of six-word records until it finds one whose rectangle contains a stored point, then calls the handler whose pointer is stored in that record.
- `ease_position_toward_targets`: When a flag byte is clear, takes new target values from a source record and moves two stored positions toward them by a step derived from the difference.
- `setup_blitter_line_draw`: Orders two endpoints, computes their horizontal and vertical distances and an octant code, then waits for the blitter to be free and writes line-drawing control values and a size word to its registers.
