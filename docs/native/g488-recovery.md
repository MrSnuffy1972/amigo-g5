# Game 488 (F29 Retaliator): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00488). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1804528 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 71 of 71 named (0 author symbols), 71 of 71 described (1 by rule, 35 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 157 of 157 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1340 of 1521 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 335553 |
| code | 6282 |
| data | 1462164 |
| padding | 17 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| F29 Retaliator (1989)(Ocean)(Disk 1 of 2).adf:bootblock | 1 | 125 | 4 |
| F29 Retaliator (1989)(Ocean)(Disk 2 of 2).adf:bootblock | 1 | 125 | 4 |
| logged disk load from F29 Retaliator (1989)(Ocean)(Disk 1 of 2).adf boot-block load | 1 | 516 | 27 |
| logged disk load from F29 Retaliator (1989)(Ocean)(Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| logged disk load from F29 Retaliator (1989)(Ocean)(Disk 2 of 2).adf boot-block load | 1 | 516 | 27 |
| logged disk load from F29 Retaliator (1989)(Ocean)(Disk 2 of 2).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 216 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |

## Cases handled

- f29 retaliator (1989)(ocean)(disk 1 of 2)/F29 Retaliator (1989)(Ocean)(Disk 1 of 2).adf: custom-format (non-DOS) disk, 1661 non-empty blocks: matched against track-loaded segments
- f29 retaliator (1989)(ocean)(disk 2 of 2)/F29 Retaliator (1989)(Ocean)(Disk 2 of 2).adf: custom-format (non-DOS) disk, 1745 non-empty blocks: matched against track-loaded segments
- reference run ref ends at a 68000 address error (odd PC an address after an address): the harness halted there, nothing after it ran (68000 HALTED: address error at odd PC an address after an address, vector 3 an address is not a handler in RAM (the 68000 double-faults and halts); run ended at frame 20)
- reference run ext ends at a 68000 address error (odd PC an address after an address): the harness halted there, nothing after it ran (68000 HALTED: address error at odd PC an address after an address, vector 3 an address is not a handler in RAM (the 68000 double-faults and halts); run ended at frame 20)
- f29 retaliator (1989)(ocean)(disk 1 of 2)/F29 Retaliator (1989)(Ocean)(Disk 1 of 2).adf: the boot block reads 5632 bytes from disk offset 12288 to an address (found statically); recovered as a unit
- f29 retaliator (1989)(ocean)(disk 1 of 2)/F29 Retaliator (1989)(Ocean)(Disk 1 of 2).adf: the boot block reads 204800 bytes from disk offset 18944 to an address (found statically); recovered as a unit
- f29 retaliator (1989)(ocean)(disk 2 of 2)/F29 Retaliator (1989)(Ocean)(Disk 2 of 2).adf: the boot block reads 5632 bytes from disk offset 12288 to an address (found statically); recovered as a unit
- f29 retaliator (1989)(ocean)(disk 2 of 2)/F29 Retaliator (1989)(Ocean)(Disk 2 of 2).adf: the boot block reads 204800 bytes from disk offset 18944 to an address (found statically); recovered as a unit
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 103 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- f29 retaliator (1989)(ocean)(disk 1 of 2)/F29 Retaliator (1989)(Ocean)(Disk 1 of 2).adf: 2 logged loads cover 210432 bytes of the image
- f29 retaliator (1989)(ocean)(disk 1 of 2)/F29 Retaliator (1989)(Ocean)(Disk 1 of 2).adf: blocks 35-36 (bytes 17920-18944) outside the filesystem hold data, not code (opcode density 0.014, entropy 4.47)
- f29 retaliator (1989)(ocean)(disk 1 of 2)/F29 Retaliator (1989)(Ocean)(Disk 1 of 2).adf: blocks 437-1759 (bytes 223744-901120) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.99)
- f29 retaliator (1989)(ocean)(disk 2 of 2)/F29 Retaliator (1989)(Ocean)(Disk 2 of 2).adf: 2 logged loads cover 210432 bytes of the image
- f29 retaliator (1989)(ocean)(disk 2 of 2)/F29 Retaliator (1989)(Ocean)(Disk 2 of 2).adf: blocks 2-23 (bytes 1024-12288) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.69)
- f29 retaliator (1989)(ocean)(disk 2 of 2)/F29 Retaliator (1989)(Ocean)(Disk 2 of 2).adf: blocks 35-36 (bytes 17920-18944) outside the filesystem hold data, not code (opcode density 0.014, entropy 4.47)
- f29 retaliator (1989)(ocean)(disk 2 of 2)/F29 Retaliator (1989)(Ocean)(Disk 2 of 2).adf: blocks 437-1759 (bytes 223744-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.73)
- 1 packed track region(s), 10752 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `set_ciab_bits_clear_one` (F29 Retaliator (1989)(Ocean)(Disk 1 of 2).adf boot-block load +an address to an address (5632 bytes)): Sets bits 3 to 6 of a global value and clears one bit chosen from the input register plus three. Kind: CIAB bit manipulation routine (part of a disk/hardware check). Pass path: Routine manipulates CIAB port bits via CIAB_PRB; the check's pass state is determined by the branch taken after the call in the caller (sub_05AE), which then executes ori.b #an address, an address.l.
- `setup_disk_sync_and_dma` (F29 Retaliator (1989)(Ocean)(Disk 1 of 2).adf boot-block load +an address to an address (5632 bytes)): Waits for the disk to be idle, writes the disk sync word into the given buffer and sets the disk sync register, then programs the disk DMA pointer and length and enables DMA. The listing also shows the DMA control and interrupt request registers touched. Kind: Boot-block sync/track check (DSKSYNC). Pass path: beq.b an address (taken when DSKSYNC matches the expected sync byte).
- `clear_bit_and_set_port_bits` (F29 Retaliator (1989)(Ocean)(Disk 2 of 2).adf boot-block load +an address to an address (5632 bytes)): Clears bit seven of a global value, then sets bits three through six while clearing one bit chosen from the low bits of the input register plus three. Kind: CIAB timer/counter-based anti-piracy check (disk/track timing check). Pass path: The routine clears a CIAB bit (CIAB_PRB), then sets specific CIA/CIAB port bits based on the value in d0. This is a classic Amiga copy-protection pattern: the loader writes to CIAB registers to start/configure a timer or counter, then the c.

## Functions


### F29 Retaliator (1989)(Ocean)(Disk 1 of 2).adf:bootblock

- `copy_boot_blocks_run_io`: Copies a 20-word block and then eight blocks of 180 words into a fixed memory area, writes four copper-style list words for colour registers, calls the same library routine twice, and copies a 144-longword table; the decompiled body then ends in undecodable bytes.
- `copy_blocks_and_set_copper`: Copies a 20-word block and eight blocks of 180 words into memory, sets the first copper list pointer, calls the exec DoIO routine twice, copies a 144-longword table, and ends in undecodable bytes that the decompiler could not translate.
- `describe_empty_data`: This routine contains no executable instructions, only data that was misidentified as code by the disassembler.
- `copy_blocks_set_copper`: A short routine of 44 instructions that copies data blocks and then sets up the copper list, which initializes the Amiga display.

### F29 Retaliator (1989)(Ocean)(Disk 2 of 2).adf:bootblock

- `copy_boot_blocks_and_init`: Issues a trap to obtain an address, copies a small block and eight larger blocks of words into memory, then writes two sets of fields into a structure.
- `copy_boot_blocks_run_io_2`: Copies a block of boot data into a fixed memory area, then fills in a structure with several pointers and sizes.
- `stub_bad_instruction_data_2`: The decoded routine is reduced to bad instruction data, so its behaviour is not recovered and no operation can be named.
- `match_checksum_in_table`: Computes a length-weighted sum and rotate-and-xor checksums over a byte string, then searches a 128-entry table of signature and offset pairs.

### logged disk load from F29 Retaliator (1989)(Ocean)(Disk 1 of 2).adf boot-block load

- `loop_calls_until_limit`: Repeatedly calls an unnamed routine with a 64-bit value and the address register, taking the next value from another routine while its low word is at least 5633, then makes one more call and ends in undecodable bytes.
- `loop_calls_until_limit_2`: Repeatedly calls an unnamed routine with a running value, then breaks when that value drops below a limit before calling one more routine and reaching undecodable bytes.
- `call_routine_then_stop`: Calls one unnamed routine and then reaches undecodable bytes, so the decompiler gives no further behaviour to recover.
- `copy_bytes_and_check_limit`: Copies a block of bytes into a fixed buffer, calls an unnamed routine, then loops forever if a register value plus one exceeds 159; otherwise it returns.
- `stub_infinite_loop`: Contains only an infinite loop with no visible writes.
- `stub_bad_instruction_data`: The decompiled body is only undecodable bytes after 87 instructions and six calls, so no behaviour can be recovered from it.
- `unpack_flags_and_runs`: Fills a 256-entry flag table with bits taken from a bit stream, then writes output: a zero flag writes a literal byte and a one flag writes a run of repeated bytes, stopping when the output count reaches zero.
- `copy_block_and_advance_counter`: Copies a block of bytes from a source pointer into a fixed destination buffer, then adds 512 to a global counter before returning.
- `copy_block_and_advance_global`: Calls an unnamed routine, copies 128 longwords from the address register into a fixed destination, and adds 512 to a global counter.
- `stub_infinite_loop_2`: Contains only an infinite loop, so it never returns.
- `stub_infinite_loop_3`: Contains only an infinite loop.
- `sum_table_entry_pairs`: Adds together both longwords of each of 128 pairs in a fixed table and returns the total.
- `scan_bit_mask_for_match`: Stores a word into a fixed record slot, then scans a global bit mask downward for set bits, calling unnamed routines for each one.
- `scan_bit_mask_for_match_2`: Scans a global bit mask downward for set bits, calling unnamed routines for each one and adding a large constant to a global when a set bit is found.
- `tiny_adda_clr_rts`: Small routine of 3 instructions that adds 4 to a7, then clears d0, then returns.
- `update_ciab_port_bits`: Clears bit 7 of a global value, sets bits 3 to 6, then clears one bit chosen from the input register plus three.
- `set_ciab_bits_clear_one`: Sets bits 3 to 6 of a global value and clears one bit chosen from the input register plus three.
- `set_port_bits_around_call`: Sets the top bit of a global value, calls one unnamed routine, then sets bits 3 to 6.
- `prepare_ciaa_state_and_call`: Sets a global value to 4 and then to 166, calls one unnamed routine and returns the input register value.
- `step_control_value_toward_target`: Adjusts a global control value from its low bits and steps it by two toward a target value in the input register, calling an unnamed routine on each step.
- `stub_infinite_loop_4`: Contains only an infinite loop with no visible writes.
- `poll_status_with_retry_count`: Sets a global counter sequence, waits while a status bit is set, then makes up to three attempts through unnamed routines, counting a global down and returning when it reaches zero or when a routine reports success.
- `setup_disk_sync_and_dma`: Waits for the disk to be idle, writes the disk sync word into the given buffer and sets the disk sync register, then programs the disk DMA pointer and length and enables DMA. The listing also shows the DMA control and interrupt request registers touched.
- `scan_disk_sectors_for_sync`: Scans a track buffer for disk sync words and, for each sector header whose check values match, sets its bit in an eleven-bit sector mask.
- `decode_interleaved_longword_pairs`: For each pair of longwords, combines the odd-bit part of the first with the even-bit part of the second, doubled, into one output longword.
- `run_blitter_with_masks`: Waits for the blitter to be free, programs its first-word masks, data word, modulo, pointer and control registers, starts a blit whose size comes from the input value, and waits until it finishes.
- `xor_checksum_longwords`: XORs together a run of longwords whose count is the input value divided by four, and returns the result.

### logged disk load from F29 Retaliator (1989)(Ocean)(Disk 2 of 2).adf boot-block load

- `call_until_limit_break`: Repeatedly calls an unnamed routine with a running value, then breaks when that value drops below a limit before calling one more routine an.
- `process_data_block`: Iterates over a block of data using a counter, copying each entry into a fixed buffer while invoking a routine for every item, and stops once the counter reaches its limit.
- `empty_routine`: This routine is empty, containing no recoverable instructions.
- `copy_bytes_then_infinite_loop`: Copies a block of bytes from a source pointer into a fixed destination buffer, then calls an unnamed routine and spins forever if a register value plus one exceeds a limit.
- `stub_infinite_loop_6`: The decoded routine is reduced to an infinite loop with no visible writes, so no operation can be named.
- `stub_bad_instruction_data_3`: The decoded routine is reduced to bad instruction data, so its 87 instructions are not recovered and no behaviour can be named.
- `expand_bit_flags_then_fill`: Expands bits from a 32-bit register into a table of flag bytes, then writes output bytes from a source.
- `maybe_copy_word_block`: Copies a block of words from one table to another when a pointer reaches a certain address, then advances a global variable by a fixed amount.
- `copy_block_and_advance_counter_2`: Calls a subroutine, copies 128 longwords from a source pointer to a fixed buffer, and adds 512 to a global counter before returning.
- `loop_indefinitely`: Loops forever without returning, executing an unconditional infinite loop.
- `loop_writing_color_register`: Loops forever while repeatedly writing to the COLOR00 hardware register.
- `sum_longwords_into_checksum`: Iterates through 255 longwords starting at a fixed address, adding each pair into a running total.
- `store_word_and_scan_bit_mask`: Stores an input word into a fixed memory location, then repeatedly scans a global bit mask downward for set bits.
- `scan_bit_mask_and_store`: Repeatedly scans a global bit mask downward for set bits, calling unnamed routines for each one and adding a large constant to a global when a set bit is found.
- `return_zero`: Returns zero without performing any other operation.
- `clear_bit_and_set_port_bits`: Clears bit seven of a global value, then sets bits three through six while clearing one bit chosen from the low bits of the input register plus three.
- `set_port_bits_clear_one`: Sets bits three through six of a global value, then clears one bit chosen from the low bits of the input register plus three.
- `set_top_bit_then_call`: Sets bit seven of a global value, calls an unnamed routine, then sets bits three through six in the same byte.
- `write_control_values_then_call`: Sets a global value to four and then to 166, calls an unnamed routine, and returns the input register unchanged.
- `step_control_value_toward_target_2`: Sets and clears bits in a global control byte, then steps that byte by two toward a target value in a register, calling an unnamed routine after each step until the values match or a bit condition fails.
- `spin_in_infinite_loop`: Spins forever in an empty loop with no other operations, so it never returns.
- `wait_and_call_with_retry_count`: Waits for a status bit to clear, then repeatedly calls several unnamed routines, decrementing a counter and checking its low bits, returning when a condition is met or the counter runs out.
- `start_disk_dma_transfer`: Waits for a status bit to clear, writes disk sync and control values into a global variable and a buffer, then sets the pointer and length registers to start a DMA transfer.
- `verify_sector_and_set_mask`: Walks a buffer of shortwords looking for the sync marker 17545, then verifies decoded data against expected values and sets a bit in an eleven-bit sector mask when they match.
- `decode_mfm_long_pairs`: Decodes MFM data by combining pairs of longwords, masking alternate bits and shifting one half of each pair left by one, then writing the results to an output buffer.
- `start_blitter_transfer`: Waits for the blitter to be idle, sets its mask, modulo, pointer, and control registers, then starts a transfer sized from a width and a height, and waits again until the blitter is idle.
- `xor_fold_longwords_masked`: XORs a run of longwords into one accumulator, then masks the high longword of the return value with alternate bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `trigger_trap_and_update_stack`: Triggers a software interrupt, increments a 16-bit value on the stack pointed to by a5, and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_interrupt_enable_mask`: Writes an interrupt enable mask, calls one unnamed routine, then writes a second mask.
- `set_interrupt_enable_and_call`: Clears a stored value, then writes a saved value with bit 15 set into the interrupt enable register and calls one unnamed routine.
- `advance_audio_frame_counters`: Increments two counters, clears a flag, and counts a third down from ten, inverting a fourth on each wrap.
- `wait_flag_then_mask_byte`: Loops while a shared flag is non-zero, then returns the input byte with bit 0 and bits 3 to 7 forced on.
- `restore_interrupt_enable_state`: Writes two saved values to a global register, then writes a third saved value with bit 15 set, which enables the interrupts it selects.
- `update_joystick_position`: Reads the joystick port movement and updates horizontal and vertical positions clamped to the screen range, then sets direction bits.
- `read_mouse_movement_and_buttons`: Decodes the mouse port movement and button bits into per-axis deltas, clamps the horizontal and vertical positions to 0 to 319 and 0 to 199, and writes the press state into the output structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_infinite_loop_5`: Contains only an infinite loop with no visible writes, and the decompiler removed the unreachable blocks around it; no further behaviour is recovered.
