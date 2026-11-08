# Game 2519 (escape from colditz (1990)(digital magic)(m4)[cr skr][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02519). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903152 top-level bytes in 45 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 47 of 47 named (0 author symbols), 47 of 47 described (10 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 269 of 269 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 647 of 647 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 711902 |
| code | 2246 |
| data | 167653 |
| padding | 11111 |
| unused | 10240 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| SKR | 2 | 24 | 4 |
| Escape from Colditz (1990)(Digital Magic)(M4)[cr SKR][a].adf:SKR_COLD | 2 | 85 | 4 |
| Escape from Colditz (1990)(Digital Magic)(M4)[cr SKR][a].adf:bootblock | 1 | 89 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 398 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- escape from colditz (1990)(digital magic)(m4)[cr skr][a]/Escape from Colditz (1990)(Digital Magic)(M4)[cr SKR][a].adf:LOADTUNE.MUS: high-entropy data file (7.82 bits/byte), classified as packed data
- runtime segment an address: 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 128 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 23 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `lookup_hashed_entry_and_copy` (runtime segment an address): Sums input bytes into a key reduced modulo seventy-two, follows a chain of linked entries for that key until one matches, then copies a block of longwords from the entry into a buffer with a stride. Kind: Manual/code-wheel lookup (hash of a typed code string). Pass path: Successful match: the inner loop hits the terminating 0 byte, stores the entry's data pointer (_u4__sub_0554) and its offset (_u4__sub_0550 = _u4__sub_0554 + _u4__sub_054C), then jumps to u4__l0_0116. There, if the entry's length dword (_u4.

## Functions


### SKR

- `call_library_entry`: Calls one routine, and when the zero flag is clear, calls a second routine and then a function pointer from a global table before calling a third routine and returning.
- `open_dos_library`: Calls the exec OldOpenLibrary function through the exec base with a library name taken from the routine's data and stores the returned library handle in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Escape from Colditz (1990)(Digital Magic)(M4)[cr SKR][a].adf:SKR_COLD

- `unpack_stream_then_relocate`: Decodes a packed stream backwards into a buffer, refilling its bit reader from the packed data.
- `cycle_color_register_values`: Writes a 16-bit counter that counts down through every possible value to the color register, one value per iteration.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Escape from Colditz (1990)(Digital Magic)(M4)[cr SKR][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `reset_system_then_jump`: Clears a global variable, issues the processor reset instruction, and then calls the code stored at a fixed low-memory address.
- `check_version_and_restore_blocks`: Checks a version byte in its base block and, when it does not match, issues trap 0 and resets.
- `clear_copy_and_start_copper`: Clears a block of longwords, copies a table of bytes into a second area in rows, copies a block of longwords into a fixed buffer, then waits for a status bit to clear.
- `copy_blocks_then_halt`: Copies a block of longwords into a fixed buffer, copies a table of words with a row stride into a second buffer, and then waits in an endless loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_loop_routine`: The routine contains no reachable instructions and consists of an infinite loop that never executes any operation.
- `check_interrupt_status_and_acknowledge`: Reads the interrupt enable and request registers and, when the master enable bit is set, masks the request bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `lookup_hashed_entry_and_copy`: Sums input bytes into a key reduced modulo seventy-two, follows a chain of linked entries for that key until one matches, then copies a block of longwords from the entry into a buffer with a stride.
- `store_offset_values_then_call`: Calls a subroutine, then stores three offset values derived from its result and two stored constants into globals.
- `copy_longwords_to_limit`: Copies longwords from a source pointer to a destination pointer until the source reaches a stored limit address.
- `copy_block_to_low_memory`: Copies a block of bytes from a source address into low memory starting at a fixed address, counting down until the count wraps around.
- `copy_low_memory_to_block`: Copies a block of bytes from low memory starting at a fixed address back to a source address, counting down until the count wraps around.
- `unpack_lz_codes_backwards`: Decodes a compressed stream into a buffer filled from its end toward its start.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_char_wrap_to_space`: Repeatedly decrements a character register for the count given in its input, wrapping to a space character whenever it reaches zero.
- `return_byte_with_bit_four`: Returns the low byte of its input with bit four set.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `return_byte_with_bit_four_2`: Returns its input byte with bit four set.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 16 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 208 into d2.
- `process_hashed_copy`: It loads a small table entry, computes its hash, and copies the referenced data into a destination buffer via a pointer.
- `call_two_then_clear_state`: Calls two subroutines, then clears two global words that hold state.
- `seek_track_for_offset`: Converts a byte offset into a track number and an offset within the track, using 5632 bytes per track.
- `set_control_bits_when_even`: When the input has its low bit clear, it clears bit one of a control byte and then sets bit zero, then calls a subroutine.
- `set_control_bits_when_odd`: When the input has its low bit set, it sets bit one of a control byte, clears bit zero and then sets bit zero again, then calls a subroutine.
- `pulse_control_while_status_set`: Sets bit one of a control byte, then while a status bit on a port is set, pulses bit zero of the control byte and calls a subroutine.
- `read_mfm_track_into_buffers`: Waits for the disk controller, starts a read with the MFM sync word and a DMA pointer, then searches the raw track for sync words and decodes each sector's data with the MFM mask while checking its checksum.
- `set_control_byte_defaults`: Writes a fixed control byte value and then clears bit three of it.
- `set_bit_two_by_parity`: Sets bit two of a control byte when the state counter is even and clears it when the counter is odd.
- `delay_countdown_loop`: Spins through a countdown loop of 3072 steps to wait briefly, then returns.
- `measure_string_length`: Counts the bytes of a zero-terminated string addressed by the input pointer and stores the count in a global variable.
- `unresolved_one_instruction_routine`: The decompiled code is unavailable.
- `tiny_ori_3`: Small routine of 1 instruction that merges 128 into d4.
- `unresolved_single_move_routine`: The decompiled code is unavailable.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 12 into d0, then merges 0 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `trap_copy_block_then_halt`: Issues trap 0, copies a block of its own code into the global area, and then waits in an endless loop.
- `copy_block_then_halt`: Copies a block of its own code into the global area and then waits in an endless loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_all_ones`: Returns the all-ones value and does nothing else.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
