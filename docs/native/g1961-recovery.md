# Game 1961 (leander (1991)(psygnosis)(disk 1 of 3)[cr skr][t +8 legend]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01961). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906368 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 49 of 49 named (0 author symbols), 49 of 49 described (3 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 127 of 127 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1183 of 1183 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 7075 |
| code | 4210 |
| data | 893534 |
| padding | 13 |
| unused | 1536 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Leander (1991)(Psygnosis)(Disk 1 of 3)[cr SKR][t +8 Legend].adf:bootblock | 1 | 106 | 4 |
| logged disk load from Leander (1991)(Psygnosis)(Disk 1 of 3)[cr SKR][t +8 Legend].adf boot-block load | 1 | 93 | 3 |
| logged disk load from Leander (1991)(Psygnosis)(Disk 1 of 3)[cr SKR][t +8 Legend].adf boot-block load | 1 | 92 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 892 | 40 |

## Cases handled

- leander (1991)(psygnosis)(disk 1 of 3)[cr skr][t +8 legend]/Leander (1991)(Psygnosis)(Disk 1 of 3)[cr SKR][t +8 Legend].adf: custom-format (non-DOS) disk, 1755 non-empty blocks: matched against track-loaded segments
- leander (1991)(psygnosis)(disk 1 of 3)[cr skr][t +8 legend]/Leander (1991)(Psygnosis)(Disk 1 of 3)[cr SKR][t +8 Legend].adf: the boot block reads 2048 bytes from disk offset 899072 to an address (found statically); recovered as a unit
- leander (1991)(psygnosis)(disk 1 of 3)[cr skr][t +8 legend]/Leander (1991)(Psygnosis)(Disk 1 of 3)[cr SKR][t +8 Legend].adf: the boot block reads 3584 bytes from disk offset 2048 to an address (found statically); recovered as a unit
- runtime segment an address: 127 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- leander (1991)(psygnosis)(disk 1 of 3)[cr skr][t +8 legend]/Leander (1991)(Psygnosis)(Disk 1 of 3)[cr SKR][t +8 Legend].adf: 2 logged loads cover 5632 bytes of the image
- leander (1991)(psygnosis)(disk 1 of 3)[cr skr][t +8 legend]/Leander (1991)(Psygnosis)(Disk 1 of 3)[cr SKR][t +8 Legend].adf: 2 track-loaded blocks found in recovered runtime segments and classified through them
- leander (1991)(psygnosis)(disk 1 of 3)[cr skr][t +8 legend]/Leander (1991)(Psygnosis)(Disk 1 of 3)[cr SKR][t +8 Legend].adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.95)
- 2 packed track region(s), 891392 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `set_port_and_check_tag` (runtime segment an address): Calls several routines, saves an input pointer, and sets the second CIA port register to all ones. Kind: Disk hardware check (CIAB_PRB register read). Pass path: Reads CIAB_PRB to verify the attached disk controller hardware; the check passes when the hardware matches the expected value, allowing execution to continue past the call (the routine returns normally and u3__sub_0230 proceeds)..

## Functions


### Leander (1991)(Psygnosis)(Disk 1 of 3)[cr SKR][t +8 Legend].adf:bootblock

- `allocate_and_configure_boot`: Boot-block entry point.
- `send_io_request_wait_status`: Sends a request through the exec device I/O call and then waits in an empty loop until a status bit clears, after which it runs a closing routine.
- `open_graphics_and_init_copper`: Opens the graphics library by name and calls two library routines.
- `copy_long_clear_word`: Copies a longword from one structure into a field 128 bytes into another structure, then clears a word field 136 bytes further along.

### logged disk load from Leander (1991)(Psygnosis)(Disk 1 of 3)[cr SKR][t +8 Legend].adf boot-block load

- `count_global_down_full_cycle`: Writes successive words to a global variable, counting down from minus one through every 16-bit value until it wraps back to minus one, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Leander (1991)(Psygnosis)(Disk 1 of 3)[cr SKR][t +8 Legend].adf boot-block load

- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_nine_step_frame_loop`: Runs an endless loop that calls nine routines in order on each pass.
- `select_adkcon_mode_and_write`: Clears several state words, copies a hardware register value into one of them, and chooses one of two routine calls by testing a passed-in value against 1 and 6.
- `set_port_and_check_tag`: Calls several routines, saves an input pointer, and sets the second CIA port register to all ones.
- `search_record_table_for_key`: Stores two pointers in a structure and calls one routine, then scans up to 128 records spaced 32 bytes apart for a matching first longword.
- `find_name_in_table`: Compares a name against consecutive 32-byte table entries, ignoring letter case, and moves on to the next entry at each mismatch.
- `save_copy_restore_scratch_block`: Saves a 60-byte block of code or data into a buffer, copies a region whose length is held in a global variable, calls routines, and then writes the saved 60 bytes back.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `decode_rle_block`: Copies a byte sequence when a marker is absent, otherwise decodes a run-length block by repeating a source byte and copying variable-length chunks into a destination buffer.
- `unpack_packed_block_pcrel_entry`: Shares the same unpacking code as its neighbours, reached here through a program-counter-relative address.
- `unpack_packed_block_secondary`: Shares the same PP20 unpacking code as its neighbours and is called from two places.
- `test_upper_bits_then_return`: Shifts a register right by one and tests whether the result is nonzero, then returns in either case.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `shift_out_bits_from_stream`: Shifts single bits out of a bit register one at a time, loading the next 32-bit word from memory whenever the register runs empty.
- `split_size_and_copy_loop`: Stores a pointer and a size, splits the size into a quotient and remainder by 5632, and then calls routines in a loop until a stored pointer is reached.
- `set_control_bit_when_clear`: When bit zero of a word at offset 12 in the supplied structure is clear, it clears bit one and sets bit zero of a control byte, then calls a routine.
- `pulse_bit_while_status_set`: Writes a control value to a byte at the supplied pointer, clearing bit three and setting bit one.
- `pulse_bit_count_halved_times`: Clears bit one of a control byte, then halves a word read from the structure and toggles bit zero once per step, calling a routine on each step, until the count reaches zero.
- `configure_then_wait_for_ready`: Writes fixed control bytes to four device registers and sets one bit in another, then busy-waits until bit zero of the first register reads as set.
- `copy_sector_bytes_to_buffer`: Sets or clears bit two of a control byte according to bit zero of a structure word, then calls a routine.
- `decode_mfm_disk_sector`: Starts a floppy disk read by setting the disk sync word and disk pointer registers and enabling DMA. It then scans for sync words and decodes the interleaved odd and even bit fields of each block into a buffer while accumulating an XOR checksum.
- `unpack_packed_stream_runs`: Decodes a bit-packed stream read from the end of a buffer.
- `decode_bit_counts_copy_runs`: Reads bit-packed counts from the end of the input, then in a loop copies runs of bytes backwards within the output buffer, using the decoded count as each run length, until the stream ends.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `forward_single_routine_call`: Calls one other routine and returns straight away, adding nothing of its own.
- `chain_six_routine_calls`: Calls another routine six times in sequence and returns, with no other work.
- `poll_until_zero_then_call`: Calls a routine repeatedly until it returns zero, then calls a second routine and returns.
- `poll_then_call_four_routines`: Calls a routine repeatedly until it returns zero, makes one fixed call, then calls four further routines in turn and returns.
- `poll_routine_then_fixed_call`: Calls a routine once and, if it returns zero, makes a fixed call and returns.
- `poll_routine_fixed_call_alt`: Has the same code as its neighbouring routine: it polls a routine until it returns zero, then makes a fixed call.
- `poll_routine_unreached_copy`: Has the same code as the routines around it, and is reached only by decoding a gap in the file as code.
- `poll_routine_then_two_calls`: Calls a routine repeatedly until it returns zero, then makes two fixed calls and returns.
- `repeat_routine_in_timed_rounds`: Sets a counter to 63 and, for each count, waits until a global value reads minus one, then calls a routine 32 times.
- `count_up_routine_rounds`: Counts a global value up from zero to 64.
- `wait_for_flag_then_halt`: Waits until a global value reads minus two, then enters an endless loop that never returns.
- `lookup_nibbles_into_word`: Takes a 16-bit value, splits three nibbles out of it, looks each up in a byte table at a computed base, combines the results into one word and stores that word through a pointer.
- `setup_blitter_and_clear_buffer`: Copies eight longwords from a fixed source to a destination, clears 10000 longwords at a fixed address, advances a wrapping counter, and writes constants to global variables that the facts map to blitter registers.
- `wait_flag_set_values`: Waits until a global value equals minus 112, then writes three fixed values to global variables.
- `install_vector_and_enable_irqs`: Stores the address of a routine into a global vector slot, then writes a constant with bit 15 set to the interrupt-enable register, which enables the interrupt sources it selects.
- `handle_interrupt_flag_bits`: Checks bits of a global state word.
- `clear_and_copy_work_buffers`: Clears ten thousand longwords at one fixed address, then copies seventy longwords from one buffer into another.
