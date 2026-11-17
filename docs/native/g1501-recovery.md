# Game 1501 (dragon breed (1989)(activision)[cr skr][t skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01501). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903056 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 39 of 39 named (0 author symbols), 39 of 39 described (2 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 107 of 107 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1092 of 1093 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 15672 |
| code | 4702 |
| data | 662522 |
| unused | 220160 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf:bootblock | 1 | 9 | 1 |
| logged disk load from Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf boot-block load | 1 | 514 | 25 |
| logged disk load from Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf boot-block load | 1 | 74 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 97 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 383 | 3 |

## Cases handled

- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: custom-format (non-DOS) disk, 1328 non-empty blocks: matched against track-loaded segments
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: the boot block reads 16384 bytes from disk offset 720896 to an address (found statically); recovered as a unit
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: the boot block reads 1536 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 92 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 10 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: 2 logged loads cover 17920 bytes of the image
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: 2 track-loaded blocks found in recovered runtime segments and classified through them
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 22-80 (bytes 11264-41472) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.18)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 83-83 (bytes 42496-43008) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.47)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 85-88 (bytes 43520-45568) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.51)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 91-145 (bytes 46592-74752) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.98)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 150-154 (bytes 76800-79360) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.42)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 158-202 (bytes 80896-103936) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.54)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 207-211 (bytes 105984-108544) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.62)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 214-261 (bytes 109568-134144) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.63)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 263-307 (bytes 134656-157696) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.09)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 315-364 (bytes 161280-186880) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.84)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 367-370 (bytes 187904-189952) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.08)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 376-445 (bytes 192512-228352) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.69)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 451-496 (bytes 230912-254464) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.09)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 502-588 (bytes 257024-301568) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.05)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 594-696 (bytes 304128-356864) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.32)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 698-711 (bytes 357376-364544) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.75)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 713-727 (bytes 365056-372736) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.68)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 729-738 (bytes 373248-378368) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.51)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 740-743 (bytes 378880-380928) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.24)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 745-848 (bytes 381440-434688) outside the filesystem hold data, not code (opcode density 0.041, entropy 5.71)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 850-862 (bytes 435200-441856) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.22)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 876-876 (bytes 448512-449024) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.42)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 881-894 (bytes 451072-458240) outside the filesystem hold data, not code (opcode density 0.027, entropy 4.95)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 897-897 (bytes 459264-459776) outside the filesystem hold data, not code (opcode density 0.047, entropy 4.89)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 900-912 (bytes 460800-467456) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.01)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 1037-1395 (bytes 530944-714752) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.70)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 1452-1452 (bytes 743424-743936) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 1474-1474 (bytes 754688-755200) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 1496-1496 (bytes 765952-766464) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 1518-1518 (bytes 777216-777728) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 1540-1540 (bytes 788480-788992) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 1562-1562 (bytes 799744-800256) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 1584-1584 (bytes 811008-811520) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 1606-1606 (bytes 822272-822784) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 1628-1628 (bytes 833536-834048) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 1650-1650 (bytes 844800-845312) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 1672-1672 (bytes 856064-856576) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 1694-1694 (bytes 867328-867840) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 1716-1716 (bytes 878592-879104) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr skr][t skr]/Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf: blocks 1738-1738 (bytes 889856-890368) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- 1 executed PCs start inside another executed instruction (overlapping-instruction idiom): decoded and counted as instruction starts
- 1 packed track region(s), 52224 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `wait_for_disk_sync_word` (Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf boot-block load +an address to an address (16384 bytes)): Sets up the disk controller for reading with the standard MFM sync pattern, then waits for the sync interrupt flag with a timeout. Kind: sync check / hardware-dependent timing. Pass path: uVar1 = 0.

## Functions


### Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf:bootblock

- `decode_boot_block_stream`: Loads two boot segments through an Exec library call, then decodes a bit-driven stream of literal and back-reference codes into memory.

### logged disk load from Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf boot-block load

- `clear_four_longword_buffers`: Writes zero to four parallel buffers in one loop, advancing one longword in each buffer per step, for two thousand iterations.
- `copy_sixteen_words_strided`: Copies sixteen 16-bit words from a source pointer into a global buffer, writing them to every second word slot so the destination advances two words per step.
- `decrement_nibble_fields_in_table`: Rewrites each 16-bit word in a table by recombining its three nibble fields, subtracting one from every non-zero field.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `run_four_subroutine_steps`: Invokes the same subroutine four times in succession, then returns the two register values it was entered with.
- `stub_empty_return_routine`: The decompiled body is a bare return, and the listed CIA port and call data do not appear in the shown code.
- `write_control_byte_then_call`: Stores a fixed constant into a byte at the target pointer, clears bit three of that byte, calls a subroutine, and returns zero.
- `write_three_bytes_then_wait`: Writes three constants into a device register block addressed from the input pointer, then polls a status byte until its lowest bit is set.
- `write_three_bytes_then_wait_2`: Writes three constants, with a different middle value, into a device register block addressed from the input pointer, then polls a status byte until its lowest bit is set.
- `bit_bang_pulses_while_flagged`: While a flag bit in a global byte is set, it sets a bit in a control byte, calls a subroutine, and waits until a second flag bit clears.
- `alternate_control_bit_by_parity`: Increments a counter byte and branches on its parity.
- `pulse_control_bit_and_wait`: Clears and then sets the lowest bit of a control byte, calls a subroutine, and waits until a flag bit in a global byte is clear.
- `emit_signed_step_pulses`: Compares a stored byte with a new value and sets a direction bit on a control byte to match the sign of the difference.
- `pulse_control_bit_three`: Writes a fixed value to a control byte, clears bit three and then sets it again, and calls a subroutine to finish.
- `retry_bit_toggle_until_done`: Repeatedly checks two conditions by calling a subroutine, and while either returns nonzero it clears and sets a bit in a control byte through a series of further calls.
- `wait_for_disk_sync_word`: Sets up the disk controller for reading with the standard MFM sync pattern, then waits for the sync interrupt flag with a timeout.
- `decode_mfm_sector_with_checksum`: After confirming the sync word, it reads blocks of longwords through the sector-read routine, XOR-combines pairs into a checksum masked to every other bit, and compares it with an expected value.
- `scan_for_disk_sync_marker`: Reads 16-bit words from the input pointer until it finds the standard MFM sync word, then calls two subroutines and returns zero.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `merge_odd_even_bit_streams`: Masks every other bit of two longwords, shifts the first masked value left by one bit, and combines the two with a bitwise OR. The mask pattern is the usual split of MFM data bits into odd and even streams.
- `check_signature_then_copy_block`: If a register holds a fixed signature longword, it copies a block of about 120 bytes backwards into a global buffer, then copies a larger region in a counting loop and moves the buffer back.
- `decompress_back_reference_stream`: Reads a bit stream in a loop and either copies a literal byte or copies a run from earlier in the output, using length and distance lookups from tables.
- `delay_short_countdown`: Runs a four-iteration countdown loop with no other work, serving as a short fixed delay before it returns.
- `check_conditional_bits`: Returns immediately if the 31st bit of a 32-bit value is set or if the 0th bit of a 16-bit value is clear; otherwise it returns.
- `read_bits_backwards_into_register`: Shifts a bit buffer left one bit at a time, loading a new bit from a word at the input pointer whenever the buffer is empty and moving that pointer backwards.

### logged disk load from Dragon Breed (1989)(Activision)[cr SKR][t SKR].adf boot-block load

- `decompress_boot_segment_stream`: Clears the top bit of the interrupt enable register to disable interrupts, then decodes a bit-driven stream of literal and back-reference codes backwards into memory up to a fixed end address.
- `poll_until_register_zero`: Calls a subroutine repeatedly until the register it reads becomes zero, then returns.
- `shift_bit_out_with_refill`: Stores its input word into a global variable, then shifts one bit out of it and returns the rest of the value shifted right, with the removed bit placed in the top position.
- `read_previous_longword`: Returns the 32-bit value stored just before the input pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `start_copper_list_on_signal`: Waits on a byte at a fixed offset until it holds one of two text characters; the first character returns immediately, the second starts the copper.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_joystick_port_state`: Clears the vertical blank interrupt request, reads two port movement values into memory bytes, and sets a high bit on each when the matching fire-button flag is clear.
- `stub_self_comparing_wait_loop`: An unreached six-instruction gap that loops while a memory value equals itself, so the loop never exits, before returning an unchanged register.
- `advance_bcd_timer_every_fifty`: When a flag byte is clear, it increments a frame counter and every fifty steps advances a packed decimal value by one unit with carry, adjusting the digits and capping the tens digit.
- `perform_bit_manipulation`: Performs a series of bitwise operations and shifts on the contents of d0 and returns a 32-bit result.
- `process_cia_interrupt_input`: Triggers a custom chip interrupt and checks the CIAA interrupt status register for bit 3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_return_stack_value`: A three-instruction gap that decodes as code and only returns the value found at the top of the stack.
- `process_audio_channels`: Loops over four audio channels, counting down each channel's timer and reading a table to set its next step.
- `stub_decompiler_timed_out`: The decompiler timed out, so no pseudo-code exists for this 17-instruction routine.
