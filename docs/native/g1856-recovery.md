# Game 1856 (f29 retaliator (1989)(ocean)[cr][b]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01856). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903408 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 40 of 40 named (0 author symbols), 40 of 40 described (1 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 157 of 157 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1536 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 788 of 880 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 172742 |
| code | 3674 |
| data | 726474 |
| padding | 6 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| F29 Retaliator (1989)(Ocean)[cr][b].adf:bootblock | 1 | 125 | 4 |
| logged disk load from F29 Retaliator (1989)(Ocean)[cr][b].adf boot-block load | 1 | 516 | 27 |
| logged disk load from F29 Retaliator (1989)(Ocean)[cr][b].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 216 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| F29 Retaliator (1989)(Ocean)[cr][b].adf bytes 899584-901120 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- f29 retaliator (1989)(ocean)[cr][b]/F29 Retaliator (1989)(Ocean)[cr][b].adf: custom-format (non-DOS) disk, 1661 non-empty blocks: matched against track-loaded segments
- reference run ref ends at a 68000 address error (odd PC an address after an address): the harness halted there, nothing after it ran (68000 HALTED: address error at odd PC an address after an address, vector 3 an address is not a handler in RAM (the 68000 double-faults and halts); run ended at frame 20)
- reference run ext ends at a 68000 address error (odd PC an address after an address): the harness halted there, nothing after it ran (68000 HALTED: address error at odd PC an address after an address, vector 3 an address is not a handler in RAM (the 68000 double-faults and halts); run ended at frame 20)
- f29 retaliator (1989)(ocean)[cr][b]/F29 Retaliator (1989)(Ocean)[cr][b].adf: the boot block reads 5632 bytes from disk offset 12288 to an address (found statically); recovered as a unit
- f29 retaliator (1989)(ocean)[cr][b]/F29 Retaliator (1989)(Ocean)[cr][b].adf: the boot block reads 204800 bytes from disk offset 18944 to an address (found statically); recovered as a unit
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 103 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- f29 retaliator (1989)(ocean)[cr][b]/F29 Retaliator (1989)(Ocean)[cr][b].adf: bytes 899584-901120 look like code no run loaded: recovered position-independently (load address unknown)
- f29 retaliator (1989)(ocean)[cr][b]/F29 Retaliator (1989)(Ocean)[cr][b].adf: 3 logged loads cover 211968 bytes of the image
- f29 retaliator (1989)(ocean)[cr][b]/F29 Retaliator (1989)(Ocean)[cr][b].adf: blocks 35-36 (bytes 17920-18944) outside the filesystem hold data, not code (opcode density 0.014, entropy 4.47)
- f29 retaliator (1989)(ocean)[cr][b]/F29 Retaliator (1989)(Ocean)[cr][b].adf: blocks 437-1756 (bytes 223744-899584) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.99)
- 1 never-loaded region(s), 1536 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 10752 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `clear_bit7_then_set_port` (F29 Retaliator (1989)(Ocean)[cr][b].adf boot-block load +an address to an address (5632 bytes)): Clears bit seven of a control byte, sets bits three through six, then clears one more bit chosen by the low bits of a data register plus three. Kind: bit-manipulation-based checksum or flag check. Pass path: the branch or state where DAT_00bfd100 retains expected bit patterns.
- `set_port_bits_clear_one` (F29 Retaliator (1989)(Ocean)[cr][b].adf boot-block load +an address to an address (5632 bytes)): Sets bits three through six of a control byte, then clears one bit chosen by the low bits of a data register plus three. Kind: bitmask-based check/trap. Pass path: The routine modifies a bit in a control/status word (an address) or a flag, likely used to validate a specific disk pattern or bit-sequence presence..
- `set_top_bit_then_call` (F29 Retaliator (1989)(Ocean)[cr][b].adf boot-block load +an address to an address (5632 bytes)): Sets bit seven of a control byte, calls a subroutine, then sets bits three through six in the same byte. Kind: sabotage path/loader protection. Pass path: sub_0596() executes with bit-set state.
- `step_head_to_target_track` (F29 Retaliator (1989)(Ocean)[cr][b].adf boot-block load +an address to an address (5632 bytes)): Compares a target value with a drive-control byte and steps the byte by two toward the target, calling a subroutine after each step, until the two are equal or a bit condition fails. Kind: track/head position check. Pass path: the loop terminates when the current head position (represented by _DAT_00001206) matches the target track/head position (in_D0w), effectively bypassing the need for head movement or extra steps..
- `spin_in_infinite_loop_3` (F29 Retaliator (1989)(Ocean)[cr][b].adf boot-block load +an address to an address (5632 bytes)): Spins forever in an empty loop with no other operations, so it never returns. Kind: disk hardware check (CIAB_PRB). Pass path: the loop terminates or exits when the condition or hardware-dependent counter (e.g., an address.l) matches d0, effectively bypassing the infinite hang/loop if the disk check or checksum validates or if the loop is a way to wait for a specific ha.

## Functions


### F29 Retaliator (1989)(Ocean)[cr][b].adf:bootblock

- `load_blocks_and_submit_io`: Copies several word blocks from a global area into memory, fills in two I/O request blocks and submits each through an exec library call, then writes a copper list of register and value pairs.
- `setup_copper_list_io`: Copies word blocks from a global area into memory, writes a copper list of register and value pairs with an end marker, and submits I/O requests through an exec library call.
- `empty_routine`: the routine contains no executable instructions.
- `subtract_byte_at_pointer`: Subtracts a byte in a data register from the byte at an address register.

### logged disk load from F29 Retaliator (1989)(Ocean)[cr][b].adf boot-block load

- `read_boot_block_chunks`: Repeatedly calls a routine with a count pair and advances an address while the low count is above 5632, then calls one more routine.
- `read_boot_block_chunks_2`: Repeatedly calls a routine with a count pair and advances an address while the low count is above 5632, then calls one more routine.
- `call_subroutine_then_stop`: Makes one subroutine call, then the decoded body ends in bad instruction data, so no further behaviour is recovered.
- `copy_run_then_check_limit`: Copies a run of bytes from a source pointer into a fixed memory block, calls a subroutine, then loops forever if a count plus one exceeds 159.
- `spin_in_infinite_loop`: Spins forever in an empty loop, so execution never leaves it.
- `stub_bad_instruction_data`: The decoded routine is reduced to bad instruction data, so its 87 instructions are not recovered and no behaviour can be named.
- `expand_bit_flags_then_fill`: Expands bits from a 32-bit register into a table of flag bytes, then writes output bytes from a source.
- `copy_memory_block`: This routine copies a fixed number of bytes from a source pointer to a destination pointer and increments a global variable.
- `copy_block_and_advance_counter`: Calls a subroutine, copies 128 longwords from a source pointer to a fixed buffer, and adds 512 to a global counter before returning.
- `spin_in_infinite_loop_2`: Spins forever in an empty loop, so execution never leaves it.
- `loop_indefinitely`: This routine performs an infinite loop and modifies a color register.
- `sum_longwords_into_checksum`: Sums 256 longwords starting from a fixed address and returns the total, as a simple checksum.
- `scan_bits_return_status`: Writes a word into a record through an address register, then scans bit positions downward in a control byte for a set bit, calling subroutines at each step.
- `poll_bits_return_flag`: Repeatedly sets a counter to five, scans its bits downward for a set bit, and calls subroutines around each attempt.
- `tiny_adda_clr_rts`: Small routine of 3 instructions that adds 4 to a7, then clears d0, then returns.
- `clear_bit7_then_set_port`: Clears bit seven of a control byte, sets bits three through six, then clears one more bit chosen by the low bits of a data register plus three.
- `set_port_bits_clear_one`: Sets bits three through six of a control byte, then clears one bit chosen by the low bits of a data register plus three.
- `set_top_bit_then_call`: Sets bit seven of a control byte, calls a subroutine, then sets bits three through six in the same byte.
- `write_control_values_then_call`: Writes two control values in sequence to a shared byte, calls a subroutine, and returns the data register unchanged.
- `step_head_to_target_track`: Compares a target value with a drive-control byte and steps the byte by two toward the target, calling a subroutine after each step, until the two are equal or a bit condition fails.
- `spin_in_infinite_loop_3`: Spins forever in an empty loop with no other operations, so it never returns.
- `run_control_sequence_until_clear`: Writes control values to a shared byte, waits until a status bit clears, then loops over a counter with subroutine calls.
- `start_disk_dma_transfer`: Waits for a status bit to clear, writes the disk sync word and disk control values, stores the sync word through an address register, then waits for the disk interrupt flag.
- `scan_sync_sector_headers`: Scans a track buffer for the disk sync word, skips runs of repeated sync words, and checks each header's two decoded values against stored values.
- `decode_mfm_long_pairs`: Decodes MFM data by combining pairs of longwords, masking alternate bits and shifting one half of each pair left by one, then writes the results to an output buffer.
- `start_blitter_transfer`: Waits for the blitter to be idle, sets its mask, modulo, pointer, and control registers, then starts a transfer sized from a width and a height.
- `xor_fold_longwords_masked`: XORs a run of longwords into one accumulator, then masks the high longword of the return value with alternate bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_check_counter`: Increments a 16-bit value stored at a memory location pointed to by a register and checks if the new value or its carry flag meets certain conditional criteria.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_interrupt_enable_sequence`: Writes interrupt-enable values to the interrupt register, one with the set-bit flag and several source bits, and calls a subroutine between two of the writes.
- `enable_saved_interrupt_mask`: Clears a stored counter, then writes a saved interrupt-enable value with the set-bit flag to the interrupt register and calls a subroutine.
- `update_audio_channel_counters`: Advances three counters, inverts one value and resets its period to ten when a countdown goes negative, and sets a flag when another counter reaches zero, then calls a subroutine.
- `wait_flag_clear_then_mask`: Waits until a shared flag becomes zero, then returns the input byte with a fixed bit pattern ORed into it.
- `write_saved_interrupt_values`: Writes two saved values to a shared register, then writes a third saved value with the set-bit flag, and returns.
- `read_joystick_position_clamped`: Reads joystick port movement, converts the low bits into X and Y deltas, and clamps the resulting position to a 320 by 200 area.
- `read_mouse_movement_counters`: Reads mouse port movement and button state, updates horizontal and vertical counters clamped to a 320 by 200 range, and writes a button flag to a shared byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `spin_in_infinite_loop_4`: Spins forever in an empty loop, so execution never returns from this routine.
