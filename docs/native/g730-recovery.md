# Game 730 (FlyHarder v1.1): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00730). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3103766 top-level bytes in 31 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 53 raw-word lines |
| Functions named and described | pass, 468 of 468 named (0 author symbols), 468 of 468 described (2 by rule, 466 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1204 of 1204 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8958 of 8958 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 469308 |
| code | 29824 |
| data | 2604542 |
| padding | 92 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 193 | 17 |
| APlayHigh | 1 | 805 | 12 |
| APlayTitle | 1 | 805 | 65 |
| Logo | 1 | 425 | 39 |
| fly | 1 | 6584 | 326 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 140 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 4 |

## Cases handled

- FlyHarder_v1.1/FlyHarder_v1.1.hdf: hard-disk image (AmigaDOS OFS, 3392 blocks): its filesystem parses and all 26 files in it are byte-identical to the extracted tree FlyHarder_v1.1_hdf, which is recovered file by file; no block outside the filesystem holds data
- FlyHarder_v1.1/FlyHarder_v1.1_hdf/FlyHarder/data/BFlyHarderLevels: high-entropy data file (7.66 bits/byte), classified as packed data
- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 139 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f150): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `wait_for_keyboard_input` (fly): The routine builds a bitmask from a small range of bytes and then repeatedly waits for a hardware event, polling a custom chip register. Kind: track/manual check. Pass path: the loop continues or breaks to allow the game to proceed (the `break` or bypass of the `u4__l0_2A74` retry or the `if (10 < (byte)(u2 >> 8))` check).
- `poll_joystick_and_wait` (fly): The routine polls the joystick/mouse fire buttons via CIAA_PRA and a global variable, waits for a change, then sets a global flag and busy-waits on CIAA_TAHI until a condition on CIAA_TALO is met. Kind: manual/code-wheel/colour-card lookup. Pass path: bVar2 == *pbVar4, skipping the while loop and proceeding to the bitmasking or exit.
- `configure_input_mask_table` (fly): Iteratively modifies a global variable and populates a table with values based on bitwise checks and a loop counter. Kind: manual/code-wheel/colour-card lookup. Pass path: bsr.w an address.
- `process_joystick_input` (fly): The routine reads joystick/mouse fire button states from a hardware register and updates a global byte that tracks pressed buttons. Kind: track/sync check (manual/code-wheel lookup). Pass path: the branch leading to u4__l0_2EA6 (returning the input/valid pattern or successful match otherwise).
- `reset_game_state` (fly): The routine clears and initializes a block of state pointed to by A6, copies values from a table indexed by a field in that block, updates several global variables and a small table, and then zeroes a set of globals. Kind: track/data-dependent state reset (sabotage/reset path). Pass path: bra.b an address.
- `empty_loop_wait` (runtime segment an address): The routine contains no executable instructions; it is an empty loop that waits for a global variable to equal a specific character and then loops forever. Kind: sync/track check. Pass path: bra.w an address.

## Functions


### game.slave

- `return_negative_one`: The routine returns a constant value with all bits set, which is the signed integer negative one.
- `initialize_display_and_dma`: The routine clears a small block of a global variable, then calls a table of functions via an indirect pointer, writes values to a global variable and to hardware registers COLOR00, COP1LCH and DMACON to enable DMA, and branches based on the return value of another table entry, invoking further table functions and another routine before returning.
- `return_table_entry_value`: The routine calls another routine obtained from a global table and returns the result concatenated with the incoming D0 value.
- `call_indirect_table_entry`: The routine invokes a function pointer taken from a table and returns the combined result of two input registers.
- `call_through_table_entry`: The routine calls another routine via an indirect jump through a table entry.
- `zero_memory_table`: The routine clears a block of memory by writing zeros to consecutive long words.
- `clear_zero_table`: The routine clears a fixed-size table of 32-bit entries in chip memory by writing zeros to successive words, using the incoming value in D0 to compute the number of entries to clear.
- `return_byte_from_table`: The routine reads a byte from a table pointed to by a register, inverts it, shifts it right by one, and compares the result with a global variable and a constant.
- `check_highscore_table`: The routine loads a global variable and a table pointer, calls a function through the table to obtain a status, and if the status is non-zero it calls another function through the table passing the global variable and then calls another routine.
- `decode_high_score_entry`: The routine reads a global variable and uses it as a base address to call a table entry, then calls another routine twice and returns a 64-bit value formed from two input registers.
- `xor_decrement_buffer`: The routine XORs each byte of a buffer pointed to by a register with a decreasing byte value starting at a constant, advancing the pointer each iteration for a fixed count, and returns a constant.
- `invoke_callback_and_return_concat`: The routine calls another routine via a pointer stored in a table and then returns the concatenation of two input values.
- `check_and_call_other_routine`: The routine invokes a function pointer stored in a global table to perform an operation, then conditionally invokes the same table entry again and calls another routine twice based on a flag variable that remains unchanged.
- `copy_string_with_lookup`: The routine copies a fixed number of bytes from a source pointer to a destination pointer, decrementing a counter.
- `mask_byte_with_constant`: The routine returns the low byte of the value it receives in D0 combined with a constant using a bitwise OR. It does not perform any memory accesses or library calls.
- `tiny_ori`: Small routine of 1 instruction that merges 103 into d0.
- `or_byte_with_constant`: The routine returns the low byte of the value it receives in D0 ORed with a constant.

### APlayHigh

- `initialize_memory_table`: The routine clears a memory table and initializes a global variable structure.
- `disable_audio_dma_and_poll_input`: The routine disables audio DMA and the audio channels, then copies a small set of words from a table into hardware audio registers and waits for a short delay.
- `reset_audio_channel_pointers`: The routine resets the four Amiga audio channel pointers AUD0LCH to AUD3LCH and then updates a global audio state word by combining parts of a control word with a period value.
- `decode_sound_effect_header`: The routine reads a pointer from a table and decodes a sound effect header stored in a structure pointed to by a register.
- `decode_note_timing`: The routine decodes a note timing value from a packed structure pointed to by a register, using a global variable and a table indexed by a byte in the structure.
- `copy_word_from_table`: The routine copies a 16-bit word from an offset in a table pointed to by a register to an offset in a structure pointed to by another register.
- `find_next_table_entry`: The routine walks a 72-byte table indexed by a byte from a control block, finds the first entry whose value is greater than a masked field of the control block, then selects the preceding entry (or the last if the flag is clear) and writes that entry into the control block, clearing a flag byte and setting a direction byte depending on the comparison with another field.
- `update_timer_and_lookup_value`: The routine updates a countdown timer stored in a structure pointed to by a register, clamping it against a limit and clearing the limit when reached.
- `apply_pitch_offset_to_sample`: The routine computes a pitch offset from a byte in a structure, optionally indexes a global table or applies a signed scaling, multiplies by a nibble from the same structure, shifts the result, adds a base value from the structure to a destination word, and updates a signed accumulator byte in the structure.
- `decode_note_byte`: The routine reads a byte from a table entry and dispatches on its low nibble, updating fields in the entry and in a secondary structure, and returns a value derived from the byte.
- `mask_byte_from_pointer`: The routine stores a byte from a pointer into another byte of the same structure.
- `handle_high_score_entry`: The routine reads a nibble from a byte in the caller’s structure, uses it as an index into a global table, adds the table entry to another byte in the structure, and if the result has its high bit set it clears that byte, updates a pointer in the structure to the next entry in a list or wraps to the start, and toggles the sign of the byte pointed to.

### APlayTitle

- `initialize_title_screen_data`: The routine clears a global flag and a set of RAM locations, then builds a zero-initialized table of pointers by iterating over a byte table and writing entries into a global structure.
- `reset_audio_channels_3`: The routine resets the four audio channel volume registers to zero and disables audio DMA by writing to the DMA control register.
- `reset_title_screen_state`: The routine increments a global counter and, when the counter wraps, it clears a flag, calls another routine four times, burns cycles with two delay loops, copies eight words from one memory area to another, and then manipulates a pair of global variables that control a small state machine and a timer value.
- `clear_title_screen_buffer`: The routine calls another routine and then clears a global variable buffer by writing zero bytes into it until a terminator is reached, advancing an index and wrapping it within a byte range.
- `set_audio_channel_length`: The routine updates the Amiga audio channel length registers AUD0LCH through AUD3LCH and manipulates a set of global variables that appear to hold a playback position and control flags.
- `reset_audio_channels_4`: The routine clears a global flag, calls another routine four times, then performs two delay loops and copies four pairs of values from a table in memory to the four Amiga audio channel length registers.
- `decode_note_header`: The routine decodes a packed note header stored through a pointer, using a table to look up timing and pitch values and to update fields in a structure pointed to by another pointer.
- `parse_audio_command_byte`: Parses a command or note header byte to update playback timing, note or instrument, and duration.
- `decode_input_byte`: The routine decodes a byte from a table using an index derived from a register value and updates several fields in a structure pointed to by a register, then returns a value derived from the decoded byte.
- `decode_input_byte_2`: The routine reads a byte from a table via an index register, masks the low nibble to select a case and then performs a small set of per-case transformations on bytes and words in the same table and in another table pointed to by a register.
- `handle_play_title_state`: The routine processes a state byte from a table entry, updates a global variable and several fields in the table, copies values to another structure, and then dispatches behavior based on the low nibble of the state byte.
- `reset_audio_channels_5`: The routine clears the four audio channel length and channel registers, writes a value to DMACON to enable audio DMA, and then manipulates a global variable and a small table of channel parameters.
- `update_animation_frame`: The routine updates a 16-bit animation field in a structure pointed to by a5 using values from a structure pointed to by a6 and a global variable.
- `copy_word_from_table_to_table`: The routine copies a 16-bit word from an offset in a table pointed to by a register to an offset in another table pointed to by a register.
- `select_palette_entry`: The routine selects a 16-bit value from a table indexed by a nibble extracted from a byte and a global variable, with a fallback to a fixed location when the global variable modulo three is zero, and stores the result into a word at an offset from a pointer.
- `find_matching_entry`: The routine walks a short table derived from a global variable and a byte from a register, comparing each entry to a short from a register.
- `copy_word_to_a5_offset`: The routine copies a 16-bit value from an address derived from a register through an offset to an address derived from another register through an offset.
- `find_table_entry`: The routine looks up an index in a table derived from a global variable and a byte from a pointer.
- `store_word_to_offset`: Stores a word from a register into a memory location at a given offset.
- `update_title_timer`: The routine updates a 16-bit timer value stored at an offset from a pointer.
- `set_title_timer_and_offset`: The routine updates a 16-bit timer field in a structure pointed to by an unmodified register, adding a nibble from another byte in the same structure, clamping the low 12 bits to a minimum value, and copies the low 12 bits to a 16-bit field in a second structure pointed to by another unmodified register.
- `select_next_animation_frame`: The routine selects a frame index from a table indexed by a byte in a structure and stores it into a field of that structure, with a flag byte updated to indicate direction.
- `clear_two_byte_field`: The routine stores a zero word into a field at a fixed offset from an address in A6.
- `process_animation_frame`: The routine updates a byte stored at offset three of a structure pointed to by a register, clears it, and then adjusts a short counter at offset sixteen using the byte at offset twenty-three.
- `update_progress_and_lookup_table`: The routine updates a progress counter in a structure using a byte step value and a direction flag, clamps it to a limit stored in the structure and clears the limit when reached.
- `retrieve_font_glyph_width`: It updates a pair of timer fields in a structure, clamps the value at a lower bound and derives an index from D0 and the structure.
- `other_routine`: This routine appears to modify a byte in a structure using nibble masking from another byte, then computes a scaled signed offset from a table or arithmetic and writes the result to a word in another structure, and updates a signed byte accumulator with a nibble-derived increment.
- `update_note_pitch`: The routine reads a byte from a note structure, extracts a note index and a type flag, selects a base pitch from a global table or computes it from the index, applies a sign and a velocity scaling factor, adds an offset, stores the resulting pitch value back into a structure, and updates a phase accumulator byte in the note structure.
- `apply_signed_delta_to_word_2`: The routine reads a signed byte from a register, optionally negates it based on a sign flag in a structure, scales it with a nibble from the same structure, adds the result to a base word from the structure and stores the sum into a word pointed to by another pointer.
- `apply_signed_delta_to_word_3`: The routine reads a byte from a table entry, combines it with a byte from a register to form a signed value, scales it with a nibble from the same entry, optionally negates it based on a sign flag byte, adds the result to a 16-bit base value from the entry and stores the sum into a 16-bit field pointed to by another pointer.
- `apply_signed_delta_to_short`: The routine reads a byte from a table using an index in D0 and a base pointer in A4, combines it with a nibble from a structure in A6, shifts the result to form a signed delta.
- `update_score_and_flags`: The routine writes a short value derived from two inputs to a global variable and updates a byte in another global structure by combining a nibble from a neighboring byte with the existing byte.
- `adjust_byte_from_nibble`: The routine reads a nibble from a byte in memory, adds or subtracts it from a signed byte stored at a fixed offset in the same structure, clamps the result to a 0 to 64 range, and writes the resulting byte to a word field in another structure.
- `update_nibble_counter_2`: The routine reads a nibble from a byte in a structure via an unreferenced pointer, adds or subtracts it from a signed counter byte in the same structure, clamps the counter to a range, and writes the counter value to a word in another structure via an unreferenced pointer.
- `decode_and_scale_value`: The routine reads bytes from a structure pointed to by an argument, merges nibbles from one byte into another, selects a base value from a global table or computes it arithmetically, scales it by a nibble from the structure, applies a sign and an offset, clamps the result to a range, stores the result to a location pointed to by another argument, and updates a byte in the structure with an accumulated value.
- `apply_signed_8bit_offset`: The routine reads a signed byte from a structure, optionally negates it based on a flag, combines it with a high byte from a register, scales it with a nibble from the structure, applies a sign flag, adds a base offset, clamps the result to 0-64, stores the result to a table entry, and updates a flag byte in the structure.
- `compute_sprite_offset`: The routine computes a signed 16-bit value from a byte in a structure and a byte from a register, scales it, adds a bias byte from the same structure, clamps the result to a zero-to-sixty-four range, and stores it as a 16-bit value through a pointer.
- `update_clamped_value`: Calculates a scaled value, applies a sign, adds an offset, and clamps the result between zero and a certain limit before storing it in a table.
- `clamp_and_store_value`: The routine clamps a signed difference between two short inputs to the range zero to sixty-four and stores the result as a word at an offset in a structure pointed to by a pointer.
- `copy_byte_and_accumulate_2`: The routine copies a byte from an offset in a structure to another offset in the same structure if the source byte is non-zero.
- `set_flag_in_table`: The routine stores the value 1 into a 16-bit field located eight bytes after the address held in a register.
- `adjust_nibble_counter_2`: The routine reads a nibble from a byte at an offset in a structure pointed to by a register, adds it to or subtracts it from a signed counter byte at another offset in the same structure, clamps the counter to a range between zero and a maximum value, and writes the counter value to a word at an offset in another structure pointed to by a second register.
- `decrement_and_clamp_byte_2`: The routine reads a byte from an offset in a structure pointed to by a register, subtracts the low nibble of another byte from the same structure, writes the result back, and clamps it to zero if it became negative.
- `clear_global_byte_flag`: The routine writes to a global variable.
- `clamp_byte_to_word_and_store`: The routine reads a byte from an offset in a structure pointed to by a register, clamps the value to a maximum of sixty-four, stores the clamped value as a byte at another offset in the same structure, and writes the same clamped value as a word to an offset in a structure pointed to by another register.
- `validate_nibble_sum`: The routine reads a byte from an address derived from an uninitialized register, adds its low and high nibbles, and stores the sum in a global variable.
- `check_string_terminator`: The routine checks whether the byte at an offset from an address is non-zero.
- `decode_input_byte_3`: The routine reads a byte from a table pointed to by a register, masks the low nibble and dispatches on its value.
- `decode_input_byte_nibble`: The routine decodes a nibble taken from a byte in a table pointed to by a register and uses it to select one of several small code paths that read or modify bytes and words in the same table and in a second table pointed to by another register.
- `update_global_from_ciaa_pra_2`: The routine reads a byte from an address derived from an uninitialized pointer, masks bit 0, shifts it left by one, and ORs the result with a global variable after clearing bit 1 of that variable.
- `mask_and_merge_byte_4`: The routine clears the low nibble of a byte at an offset from an address in A6 and then merges the low nibble of another byte at an offset from the same address into it.
- `clear_low_nibbles_of_byte`: The routine clears the low nibble of a byte at an offset from an address and then ORs in the low nibble of another byte from the same base, effectively copying the low nibble from one byte to another while preserving the high nibble of the destination.
- `a6_offset_mask_store`: The routine stores a masked byte from an A6-relative offset into another A6-relative location.
- `decode_title_byte`: The routine reads a nibble from a byte pointed to by a register, updates a byte at an offset in the same structure, and writes a byte from another offset to a global variable, then sets the global variable to a constant.
- `store_byte_from_pointer_to_global`: The routine writes a byte from a register into a byte at an offset from a pointer, then copies a byte from another offset of the same pointer into a global variable and finally writes a constant byte value into that global variable.
- `store_nibble_via_pointer`: The routine writes a byte derived from a register value shifted right by four positions into a location computed from an uninitialized pointer plus a constant offset.
- `mask_and_merge_nibble`: The routine masks the low nibble of a byte at an offset from a pointer in A6 and then merges the high nibble from another byte at an offset from the same pointer, storing the result back to the first byte.
- `decode_and_copy_title_data`: The routine reads a nibble from a table entry and, if a global variable and the entry satisfy a modulo condition, copies two 32-bit words and one 16-bit word from the table through a pointer, performs two 300-iteration delay loops, writes a modified value to a global variable, and then copies two more 32-bit words from the table through the same pointer.
- `wrap_nibble_and_set_table`: The routine wraps a nibble from a byte in a structure and stores the result into a table entry.
- `decrement_and_clamp_byte_3`: When a global flag is set, the routine decrements a byte in a table by the low nibble of another byte from the same table, clamps it to zero if it underflows, and writes the result to a word field in a different structure.
- `clear_flags_on_condition`: The routine checks a nibble from a byte pointed to by an argument register against a global variable.
- `check_and_update_table_entry`: The routine reads a nibble from a table, compares it with a global variable and, if equal and a flag is non-zero, copies two words from the table into a destination structure, inserts a delay loop, updates the global variable with a table field and another delay, then copies two more words from the table into the destination.
- `set_title_byte_from_register`: The routine checks a global variable for a zero value and, if the condition holds, reads a nibble from a register, adds one, and stores the result into a global variable.
- `set_nibble_and_toggle_byte`: The routine checks a global variable and, if zero, combines a nibble from a structure field into another byte field.
- `reset_title_music_pointer`: The routine reads a nibble from a structure field and uses it as an index into a global table, adds the table entry to another structure byte and checks the sign bit.

### Logo

- `copy_self_to_ram_and_execute`: The routine copies a block of bytes from its own code into a fixed RAM area and then jumps to the copied code.
- `init_logo_buffers`: The routine conditionally copies a block of bytes from a source buffer to a destination buffer using a length derived from the first word of the input, then calls another routine several times with flags and pointers derived from that copy.
- `trigger_debug_trap`: The routine immediately triggers a debug trap and returns the value it received in register D0 unchanged.
- `empty_routine`: The routine contains no instructions and performs no operations.
- `initialize_jump_table_2`: Fills a memory area with a sequence of addresses and then populates a table with certain function pointers or offsets.
- `store_byte_and_dispatch`: The routine stores a byte from a register through a pointer and then, if a register value is less than another, it performs an indirect jump through a table to call another routine.
- `copy_logo_bytes`: The routine copies seven bytes from a source location determined by a byte offset in memory to a destination area, or stores a single byte from a register when the offset is zero.
- `copy_three_bytes`: The routine copies three bytes from a source location to a destination location using a pointer offset.
- `check_byte_and_dispatch`: The routine examines a byte located before a pointer and, if the high bits are set, it calls a handler selected from a table using the low two bits.
- `copy_bytes_from_table`: The routine copies bytes from a table pointed to by a register into a buffer pointed to by another register.
- `decode_and_copy_tile`: The routine decodes a tile descriptor from a byte stream pointed to by a register and copies a fixed-size tile block from a table into a destination buffer pointed to by another register.
- `copy_bytes_from_table_2`: The routine copies a small block of bytes from a table into a destination buffer pointed to by a register.
- `copy_run_length_block`: The routine copies a run-length encoded block from a source buffer into a destination buffer.
- `copy_logo_tile`: The routine copies a block of bytes from a table pointed to by a global variable into a buffer pointed to by a global variable.
- `copy_tile_bytes`: The routine copies a variable-length block of bytes from a source area into a destination area.
- `copy_pattern_from_table`: The routine copies a byte pattern from a table into memory.
- `decode_rle_block`: The routine decodes a run-length encoded block pointed to by a pointer, using a table indexed by the first byte, and writes the decoded bytes to a destination buffer.
- `decode_logo_tile`: The routine decodes a byte from a source pointer and uses it to copy a small block of bytes from a table into a destination buffer, handling a simple run-length or offset encoding.
- `copy_byte_backwards`: The routine copies bytes backwards from a source area defined by a byte in A0 to a destination area referenced by A2.
- `copy_bytes_with_offset`: The routine copies a small number of bytes backwards from a source area to a destination area using an offset derived from a byte at the source pointer minus one.
- `decode_and_shift_bytes`: The routine reads a control byte from the byte before a pointer and, if the control byte is non-zero, uses its low bits to shift a small block of bytes backwards in memory.
- `move_bytes_from_table`: It copies a small block of bytes from a global table into a buffer using a byte taken from the input and then calls a routine selected by a byte from the input.
- `shift_bytes_from_table`: The routine copies a byte from a register to a location pointed to by a pointer, or if a flag is set it reads a control byte, selects a source offset, copies a byte from a table and shifts a few bytes from that table into a destination buffer.
- `decode_logo_tile_2`: The routine decodes a byte from a source pointer and copies a small tile pattern into a destination buffer, using a table indexed by the source byte.
- `decode_and_indirect_call`: The routine decodes a byte from an input buffer to produce an output byte and copies a small variable-length field backwards, then performs an indirect call through a table using a byte from the input buffer.
- `copy_bytes_with_offset_2`: The routine copies a small number of bytes backwards from a source area determined by a byte at an offset from a pointer.
- `copy_logo_tile_2`: The routine copies a small tile from a source area into a destination buffer using a byte from the source header to decide the copy length and offset, then it calls another routine via a jump table indexed by a byte from the source header.
- `decode_logo_tile_3`: The routine reads a byte from memory just before a pointer and uses its bits to decide whether to copy a single byte from a register into a buffer or to copy a small block of bytes from a table indexed by the high bits of that byte.
- `decode_logo_tile_row`: The routine decodes a byte from a source buffer using a table lookup and copies a small number of bytes backwards into a destination buffer.
- `copy_bytes_with_offset_3`: The routine copies bytes between two memory areas using an offset derived from a byte at the source address minus one, writing to a destination area addressed by an argument.
- `copy_logo_tile_3`: The routine copies a small tile from a source area into a destination buffer using a byte from the source as a control value.
- `copy_byte_backward`: The routine copies a byte from a register into a buffer and optionally copies a short backwards copy of bytes from a table indexed by a byte from memory.
- `copy_plane_bytes`: The routine copies a small number of bytes from a source plane into a destination plane, using a byte from the source plane to decide the copy length and offset.
- `decode_rle_block_2`: The routine decodes a run-length encoded block pointed to by a0 into a buffer referenced by a2, using a length byte and a table of values from a global variable, and then dispatches through a jump table using a5 and a byte from a0 when the output pointer has passed the input pointer.
- `decode_logo_data`: The routine decodes a packed byte stream pointed to by a0 into an output buffer pointed to by a1, using a control byte per run.
- `call_table_dispatch`: The routine checks a flag in a register and a range condition on two registers, then indexes a table of routine pointers using a masked value from the flag register and calls the selected entry.
- `copy_planes_block`: The routine copies 8-byte blocks from a source pointer in A3 to a destination pointer in A4, repeating the copy for a count held in D3 and an outer count held in D1.
- `copy_struct_array`: The routine copies a sequence of 8-byte blocks from one memory area to another, using two counters that are decremented each iteration.
- `copy_8_byte_blocks`: The routine copies blocks of eight bytes from one memory area to another, moving backwards through the source and destination pointers until a loop counter expires.

### fly

- `initialize_game_state`: The routine clears a block of RAM, then calls another routine to set up a jump table and performs a series of indirect calls through a table.
- `copy_memory_block`: The routine copies a fixed-size block of bytes from a source pointer to a destination pointer, then writes a constant value to a location offset from the destination and calls another routine.
- `initialize_memory_buffers`: The routine writes a constant to a global variable, calls another routine, then clears a large block of memory to zero and stores pointers to that block and to several other memory areas in global variables.
- `empty_routine_2`: The routine contains no executable instructions and performs no operations.
- `cleanup_library_handles`: The routine calls another routine and then checks a flag in a structure returned by that call.
- `clear_and_find_linked_entry`: The routine clears a global variable and copies a pointer from a table into another global variable.
- `clear_linked_list_flags`: The routine walks a singly linked list whose head pointer is taken from an offset in a structure pointed to by a0.
- `copy_joypad_state_block`: The routine copies six words from a fixed table to a block in the structure pointed to by a0, then writes a byte from a global variable and another byte from a RAM location into the structure, and finally writes two words formed by combining a global variable with a constant bit into the structure.
- `check_and_disable_interrupts_dma`: The routine first checks a byte at an offset from a pointer and returns early if it is zero.
- `clear_and_set_custom_interrupts`: The routine returns early if a byte at an offset from an input pointer is zero; otherwise it writes a mask to a global variable, calls another routine, then overwrites the global variable with masked values read from two words at offsets from another pointer.
- `set_game_state_and_load_word`: The routine exits immediately when a flag byte is clear.
- `copy_input_to_table`: The routine copies six 32-bit values from an input structure into a fixed table and then writes two bytes derived from the input structure into a global variable and a RAM location, OR-ing each with a constant bit.
- `decode_5_bytes_with_xor_shift`: The routine reads five bytes from a pointer passed in a1 and builds a 32-bit value in d1 by combining a running accumulator with each byte shifted and masked.
- `convert_register_to_ascii_digits`: The routine extracts the low nibble of a value in a register, converts it to an ASCII digit and stores it through a pointer that is decremented each iteration.
- `initialize_copper_list`: The routine writes a set of values into a block of memory that is used as a copper list, then loads a pointer to a table, writes four words into that table and calls another routine.
- `initialize_game_state_2`: The routine writes constants to fields of a structure pointed to by a0, stores the result of a recursive call into a field of that structure, clears and sets bytes in several structures, and then calls a number of other routines.
- `wait_for_flag_and_mask`: The routine calls another routine three times, then clears a byte at an offset from a pointer passed in a register, spins until that byte becomes non-zero, and returns the low three bits of that byte.
- `wait_for_flag_and_mask_low_bits`: The routine clears a byte at an offset from a pointer in A0, then spins until that byte becomes non-zero, and finally returns the low three bits of that byte.
- `copy_39_bytes_from_a1`: The routine copies 39 bytes from the address in a1 to a global variable using a loop.
- `empty_routine_3`: The routine contains three 68000 instructions and performs no observable work; it ends immediately without accessing memory, registers, or calling other code.
- `store_word_at_pointer_and_shift_byte`: The routine selects a 16-bit value based on the low bit of the input byte, writes the low and high bytes of that value to the two bytes preceding the address in A1, and returns the input byte shifted right by one.
- `check_input_state_and_branch`: The routine calls another routine twice and then tests two bytes in a structure pointed to by a0, together with bits in a global variable and the CIAA_PRA register.
- `set_video_mode_and_load_end_text`: The routine clears bytes in a structure, modifies bytes pointed to by pointers, writes several fields in another structure, repeatedly calls other routines until a return value is zero, and then sets a global variable and clears bytes in another structure.
- `set_display_window_and_intro_text`: The routine configures the display window register DIWSTRT and manipulates bytes in structures referenced via pointers, using the intro text strings BIntro1.con through BIntro7.con.
- `toggle_byte_with_loops`: The routine calls another routine several times and toggles a byte at a fixed offset from a pointer returned by a caller.
- `self_recursive_loop_wait`: The routine calls itself twice and then spins until the high byte of a global variable equals a constant value.
- `call_other_routine_five_times`: The routine sets a short counter to 4 and then repeatedly calls another routine, decrementing the counter each iteration, until the counter reaches -1.
- `set_global_parameters`: Executes a routine and then overwrites a global variable with several different constant values and function pointers.
- `disable_interrupt_and_clear_flag`: The routine checks a global variable for a specific bit.
- `return_stack_value`: The routine returns the 32-bit value that was passed in a register or on the stack without modification.
- `initialize_system_state`: Executes an indirect function call and then calls another routine and sets a global variable to a constant value.
- `reset_interrupt_request`: The routine clears an interrupt request bit in the INTREQ register by writing a mask to it.
- `wait_for_pointer_condition`: The routine receives a pointer in a register and a value in a register.
- `decode_and_clear_table`: The routine decodes 32 16-bit values from a source table into a destination table using a nibble permutation and then clears the next 32 32-bit words.
- `swap_and_toggle_loop`: The routine swaps two 32-bit fields in a structure pointed to by a pointer, toggles a byte flag in the structure and sets a short offset based on the flag.
- `loop_three_calls_until_flag`: The routine repeatedly calls another routine three times in sequence.
- `repeated_calls_loop`: The routine repeatedly calls another routine three times per iteration until a short value returned in D1 is zero.
- `accumulate_array_elements`: The routine adds the contents of a global variable table to a second table in place, updating each element with the sum of the two corresponding entries.
- `subtractive_prefix_accumulator`: The routine subtracts each element of a 32-bit integer array from the element 32 bytes ahead in the same array, updating the later element in place.
- `set_palette_entries`: The routine calls another routine three times and stores each returned word into three different locations pointed to by the first argument.
- `decode_nibble_pair`: The routine takes a 32-bit value in D1 and extracts two nibbles from its high bytes, adds eight to each, masks them, and combines them with the low byte of the input to form a 16-bit result.
- `reset_hardware_state`: The routine clears a block of custom hardware registers, clears a block of global memory and a block of memory pointed to by a pointer, then writes a constant to a global variable and stores the address of another routine into a global variable.
- `copy_palette_and_timing_from_table`: The routine copies several words and longwords from a table indexed by the low byte of D0 scaled by 26, using A0 as a base pointer, into fixed offsets of the structure pointed to by A0.
- `patch_jump_table`: The routine builds a jump table from a table of offsets in memory and patches the table entries with relocated addresses.
- `fill_plane_with_or_of_rows`: The routine takes a pointer to a header in a1 and processes a 2-dimensional block of 16-bit words.
- `copy_data_and_set_pointer`: Copies a fixed number of words from a global table to a destination buffer and updates a global pointer with the new buffer address.
- `clear_zero_table_3`: Clears a specific table in memory.
- `return_input_value`: The routine calls another routine and then returns the value that was in register D0 on entry, leaving the call result unused.
- `clear_zero_table_2`: The routine clears a table of 32-bit entries by writing zero to each element.
- `skip_newline_loop`: The routine calls another routine, then repeatedly calls it again while the byte immediately before a pointer held in a register equals newline.
- `decode_font_indices`: The routine reads bytes from a pointer until a zero byte is found.
- `traverse_linked_list`: Traverses a structure in memory, updating pointers to point to a new location based on a count or offset found in the data.
- `handle_decompression_state`: The routine checks a flag in a structure and either branches to a decompression loop or calls callbacks stored in the structure, then copies bytes from a source buffer pointed to by the structure into a destination buffer and invokes another routine with the buffers and several parameters.
- `copy_plane_data_with_offsets`: The routine conditionally selects a source block and a destination buffer based on a flag in a structure, then copies a variable-length plane of bytes from the source into the destination and afterwards copies the bytes back in reverse order using a calculated offset, adjusting pointers with a base address and a length derived from the source header.
- `process_level_entry`: The routine reads an integer from an offset in a structure, calls another routine via a pointer derived from that integer, stores the result into another offset in the same structure, and if the result is non-zero it calls a second routine via a pointer derived from the same integer and returns the values in D0 and D1.
- `invoke_two_library_functions`: The routine reads an integer from an offset in a structure pointed to by a register, uses it as a base to call two library functions via negative offsets, and returns a 64-bit value formed from two input registers.
- `search_and_copy_string`: The routine walks a linked list of structures and, for each entry whose flag is set, it iterates a second linked list.
- `adjust_linked_list_node`: The routine walks a linked list via a pointer stored in a structure to find a node whose flag is clear or whose size field is small enough, then updates the node's linked-list pointers and size fields, clears a flag in the node, and writes a small header into the node using values from registers and a source string, followed by copying a byte string from a pointer into the node.
- `copy_string_from_linked_list`: The routine walks a linked list of structures via a global variable, checks a flag in each entry and then scans a linked list of sub-entries for a matching type and identifier.
- `insert_payload_into_chain`: The routine walks a linked list of structures via a pointer stored at an offset in a register, skipping entries until a condition on a flag and a size field is met, then finds the last element of a sub-list and links a new node into it.
- `rotate_table_bytes`: The routine rotates a block of bytes in a table by copying a slice from a source offset to a destination pointer, wrapping around a fixed-size buffer, and repeats until the whole block has been moved.
- `copy_list_elements`: Iterates through a list of pointers and copies short words from a source structure into a destination buffer until a certain condition or null pointer is met.
- `search_table_for_zero`: Iterates through a table, checking if certain memory values or flags are zero or if a string-based lookup returns a certain value.
- `compare_case_insensitive_string`: The routine compares a zero-terminated string pointed to by a register with a fixed-length string stored in memory at an offset from a pointer, performing case-insensitive matching for ASCII letters.
- `hash_string_to_index`: The routine measures the length of a zero-terminated byte string in A2, then iterates over its characters applying a shift-mix with a constant multiplier and a case-fold for ASCII letters, masking the accumulator, and finally returns a 32-bit value formed from the low 16 bits of the accumulator modulo a constant scaled and offset, combined with the incoming D1 register.
- `compute_tile_offset`: The routine divides a 16-bit input value by eleven, compares the resulting byte to a global variable and, if different, updates the global variable and calls another routine.
- `wait_for_keyboard_input`: The routine builds a bitmask from a small range of bytes and then repeatedly waits for a hardware event, polling a custom chip register.
- `apply_bit_rotation_to_table`: Performs a bitwise left-shift and masking operation on every element in a table, storing the result back to that table.
- `combine_two_words`: The routine returns a value computed from a register and two words from a pointer.
- `pack_angle_to_words`: The routine takes a 16-bit value from a register, divides it by a constant and stores the quotient as a 16-bit word through a pointer, then stores the remainder split into two 16-bit words through the same pointer.
- `poll_joystick_and_wait`: The routine polls the joystick/mouse fire buttons via CIAA_PRA and a global variable, waits for a change, then sets a global flag and busy-waits on CIAA_TAHI until a condition on CIAA_TALO is met.
- `set_flag_bits_and_wait`: The routine sets a byte pointed to by a pointer by ORing in a bit, then clearing a bit, then ORing in another bit, and then writes two values to a global variable, looping until the variable is odd.
- `clear_input_port_bits`: The routine clears two bits in a byte pointed to by a register, sets the least significant bit, writes a constant to a global variable, then writes another constant to the same global variable and spins until the least significant bit of that variable is set.
- `configure_input_mask_table`: Iteratively modifies a global variable and populates a table with values based on bitwise checks and a loop counter.
- `trap_onz_handler`: The routine contains a single trap instruction and returns the value it receives in D0.
- `empty_routine_4`: The routine contains no instructions and performs no operations.
- `trap_stub`: The routine is a 12-instruction stub that traps to the OS and returns a 64-bit value formed by concatenating the two input registers.
- `store_register_to_global`: Stores the contents of a register into a specific global variable.
- `trap_stub_2`: The routine contains a single trap instruction and returns the value it receives in D0 without modification.
- `do_nothing_routine`: The routine is empty and performs no operations.
- `clear_name_table_entries`: The routine clears name table entries for a set of items and then scans the table for a matching first character.
- `call_other_routine_return_input`: The routine calls another routine and then returns the value it received in D0.
- `process_joystick_input`: The routine reads joystick/mouse fire button states from a hardware register and updates a global byte that tracks pressed buttons.
- `store_register_to_global_2`: Stores a byte from a register into a global variable.
- `decode_font_indices_wrapper`: This routine calls a routine that decodes font indices.
- `check_input_device_state`: The routine reads a global variable and a byte from a table indexed by a structure field, testing bits that include the joystick/mouse fire buttons and disk status via CIAA_PRA, and returns early if any test fails.
- `self_decrement_loop`: The routine repeatedly calls another routine twice per iteration while decrementing an uninitialized short counter until it reaches -1.
- `wait_for_vposr_low_word`: The routine loads a 16-bit value from a register and repeatedly reads the low 9 bits of a global variable until they match the loaded value.
- `wait_for_vertical_position`: The routine reads a 16-bit value from a global variable, shifts it right by eight bits and masks it with a 9-bit mask, then loops until the resulting value is not less than the input word in D1.
- `set_global_flag`: The routine writes a constant byte value to a global variable and then returns.
- `initialize_jump_table`: The routine fills a table of function pointers with the address of another routine and then populates the same table using values from a global variable and from a byte buffer, effectively building a jump table.
- `store_byte_and_dispatch_2`: The routine stores a byte from a register into memory via a pointer and, if a condition on two pointers holds, it performs an indirect call through a table indexed by a byte from memory.
- `copy_block_fixed_size`: Copies a fixed 7-word block from a source address to a destination address based on a byte value.
- `copy_three_bytes_and_dispatch`: The routine copies three bytes from a source pointer to a destination pointer and then, if a condition on two input values holds, it dispatches through a table indexed by a byte from memory.
- `dispatch_conditional_copy`: Performs a single byte store or branches to a routine based on the lower bits of a byte value.
- `copy_block_small_variable`: Copies a variable number of words from a source address to a destination address using a bit-shifted length and a masked offset.
- `copy_block_dual_mode`: Copies either a fixed 4-word block or a variable number of words depending on a threshold check of a combined 16-bit value.
- `process_block_and_dispatch`: Checks a 16-bit value to determine whether to copy a block of words using a specific offset or a different calculation method before executing an indirect call.
- `copy_block_high_offset`: Copies a variable number of words from a source address to a destination address using a high-range offset calculation.
- `copy_block_mid_high_offset`: Copies a variable number of words from a source address to a destination address using a mid-high range offset calculation.
- `copy_block_mid_offset`: Copies a variable number of words from a source address to a destination address using a mid-range offset calculation.
- `copy_data_with_offset`: Copies a block of data from a source address calculated via an offset to a destination address, then performs an indirect call.
- `copy_block_variable_offset`: Copies a variable number of words from a source address to a destination address using a calculated offset.
- `copy_bytes_with_offset_4`: The routine copies a small number of bytes backwards from a source area determined by a byte at an offset from a pointer, using a length and offset encoded in that byte, and stores the result into a destination area pointed to by a register.
- `shift_bytes_by_offset`: The routine reads a control byte from one byte before a pointer and, if the control byte is non-zero, uses its low two bits as a repeat count and its high bits as an offset to copy a small block of bytes backwards from a source area into a destination area.
- `copy_data_with_offset_2`: Copies a variable number of words from a calculated source address to a destination and executes a subroutine via a table.
- `copy_data_with_offset_3`: Copies data from a source address determined by an offset to a destination and branches to a function pointer.
- `copy_data_with_offset_4`: Copies a block of words from a source address using a specific offset and then calls another routine.
- `copy_data_with_offset_5`: Copies data from a calculated source to a destination and performs an indirect jump to a subroutine.
- `copy_data_with_offset_6`: Copies a block of data from a source address with an offset to a destination and executes a function.
- `copy_data_with_offset_7`: Copies words from a source address determined by an offset to a destination and calls a routine from a table.
- `copy_data_with_offset_8`: Copies a block of data from a calculated source to a destination and performs an indirect call.
- `copy_data_with_offset_9`: Copies data from a source address using an offset to a destination and executes a subroutine.
- `copy_data_with_offset_10`: Copies a variable number of words from a source address to a destination and branches to a function pointer.
- `copy_data_and_dispatch`: Copies a variable number of words from a source address to a destination address based on a byte value and then performs an indirect call.
- `copy_byte_with_offset_logic`: Copies a single byte or a sequence of bytes from a source location to a destination based on a bitmask check.
- `copy_variable_length_block`: Copies a block of data from a source to a destination based on a length and offset encoded within a control byte.
- `copy_memory_with_offset`: Copies a block of data from a source address to a destination address using a value derived from a byte offset.
- `copy_memory_with_offset_2`: Copies a block of data from a source address to a destination address using a calculated offset.
- `copy_memory_block_with_offset`: Copies a block of data from a source address to a destination address using a calculated offset.
- `write_repeated_values`: The routine reads a count and a step from a byte and a global table, then writes a decreasing value repeatedly into a destination buffer, using byte or word stores depending on alignment.
- `decode_rle_compressed_block`: The routine decodes a run-length encoded block pointed to by a0 into a destination buffer pointed to by a1, using a secondary pointer in a2 as an upper bound.
- `dispatch_table_call`: The routine checks a condition on a register value and, if true, selects an entry from a table of routine pointers using a masked index derived from another register and then calls the selected routine.
- `copy_struct_array_2`: The routine copies a sequence of 8-byte structures from a source pointer to a destination pointer, using two counters in D1 and D3 to control the number of iterations.
- `copy_backward_8byte_blocks`: The routine copies a series of 8-byte blocks from one memory area to another, moving the source and destination pointers backwards.
- `initialize_object_pointers`: The routine clears two fields in a structure pointed to by a0, stores a pointer to an internal field inside the structure, writes a constant to a short field and to another short field in the structure, then fills a table of sixty pointers inside the structure with addresses spaced by a fixed stride starting from an internal field.
- `toggle_and_shift_data`: Toggles a bit in a local variable and, if a certain condition is met, shifts a series of bytes backward in memory while inserting a new byte.
- `update_table_and_shift_data`: Modifies a global variable, updates a pointer-based table, and shifts a series of bytes backward in memory.
- `clear_bit_in_table`: The routine clears a single bit in a byte table.
- `fetch_next_byte_from_circular_buffer`: The routine waits until a flag byte in the object structure is not a space character, then reads a byte from a small circular buffer pointed to by the object, advances the buffer index modulo 32, increments the flag, and returns the byte in the low part of the result.
- `decode_tile_color_effect`: The routine walks a decode table stored in a global variable and compares each entry against bytes in a structure pointed to by a0.
- `update_display_bounds`: Toggles a bit, increments a counter, and conditionally increments a value in a structure, while also performing a series of function calls and updating a global variable.
- `update_display_palette`: The routine reads a 16-bit value from the object structure, derives an index, and if it changed, writes a repeated byte pattern into several byte offsets of a block of memory starting from a base pointer plus a constant.
- `update_display_palette_2`: The routine updates a cached palette index derived from a stored value in the object structure and, when it changes, writes bytes to a set of offsets in a per-frame buffer using a small lookup table, then clears trailing bytes in the same buffer.
- `fill_tile_palette_data`: The routine updates a range of bytes in a table pointed to by a field in the object, using a 32-bit value from two fields in the object.
- `populate_palette_data`: Iterates through a bit-manipulated value to copy a series of bytes from a source table into a destination buffer.
- `copy_sprite_pattern`: The routine copies a series of bytes from a table addressed by a global variable and an index derived from a register value into a destination buffer pointed to by a pointer argument.
- `copy_sprite_table`: The routine copies a fixed set of seven bytes from a table indexed by the low bits of a rotating value in D0, using A2 as the table base, into a destination buffer pointed to by A1.
- `copy_palette_and_tiles`: The routine copies a block of bytes from a global variable into a buffer pointed to by a register, stepping through the buffer with a fixed stride, then fills a second buffer with bytes taken from the global variable using an index derived from a byte in another structure, writing several bytes per iteration at fixed offsets within each entry.
- `copy_palette_entries`: The routine copies a series of bytes from a source table pointed to by a global variable, using an index table, into a destination buffer passed in a register, writing seven bytes at fixed offsets within each destination entry.
- `copy_sprite_frame_bytes`: The routine copies a sequence of bytes from a source table into a destination buffer, writing one byte at a time to several fixed offsets within each destination block.
- `clear_palette_entries`: The routine clears a block of memory pointed to by a pointer by writing a byte value to each byte and to an offset within each entry, then writes a different byte value to a set of spaced locations.
- `check_and_branch_on_timer`: The routine checks a flag stored in an object structure; if set, it reads a timer value, calls another routine, and conditionally calls it again when the timer modulo a constant is non-zero.
- `check_and_maybe_call_twice`: The routine checks a flag stored in an object structure; if the flag is set it reads a timer value, calls another routine, and if the timer value modulo a constant is non-zero it calls the same routine a second time.
- `set_high_bit_in_buffer`: The routine processes a buffer pointed to by a pointer in memory, setting the high bit of each byte in the buffer.
- `set_flag_bits_in_buffer`: The routine walks a table of offsets and pointers using a loop counter derived from a word value.
- `execute_callback_chain`: The routine reads a global variable and if it is non-zero it repeatedly calls a function pointer stored in that variable, stopping when the call returns zero.
- `execute_callback_chain_2`: The routine reads an integer from a global variable and, if it is non-zero, treats it as a pointer to a structure.
- `execute_function_table_loop`: The routine repeatedly calls a function pointer taken from a table, advancing the pointer each iteration, until the called function returns zero.
- `process_entity_list`: Iterates a linked list of entities, skipping those with an active timer and advancing a per-entity animation counter.
- `update_active_entities`: The routine walks a linked list of structures, decrements a counter field and updates a timer byte.
- `decrement_linked_list_timers`: The routine updates a linked list of objects, advancing position fields from a source pointer and clearing a timer field, then walks the list decrementing a per-object countdown and invoking another routine when coordinates match a range.
- `update_active_objects`: The routine updates a linked list of objects by decrementing a counter, moving objects through a state table and calling another routine when the object overlaps a region.
- `store_joystick_flag`: The routine stores a value derived from a global variable into a two-byte field at an offset from a pointer passed in a0.
- `update_sprite_position_and_visibility`: The routine updates a per-object animation frame index and a visibility flag, then computes a screen position from a table entry and the object’s base coordinates, clamps the result to the object’s bounding box, and writes the final coordinates back to the object structure.
- `update_sprite_position`: The routine updates a sprite’s horizontal and vertical position by selecting an entry from a table using an index stored in a global variable, applying offsets and clamping the result to bounds stored in the sprite structure, and writes the clamped values back.
- `update_sprite_position_and_bounds`: The routine updates a sprite's position from a table entry, applies a scaling step for a specific mode, clamps the coordinates within the sprite's bounds, and updates the sprite's bounding box and internal offsets.
- `clamp_and_update_viewport`: The routine updates a pair of short offsets and a pair of word offsets in a structure pointed to by a register, clamping them to limits stored in the structure and to a global variable.
- `clamp_and_update_scroll_offsets`: The routine clamps two signed short deltas against limits stored in the structure, updates position accumulators with saturation against limits, refreshes derived high bytes, calls another routine twice, then writes back adjusted values to fields in the structure and updates screen-relative offsets.
- `clamp_and_update_scroll`: The routine clamps two short values read from the input structure to limits stored in the structure, writes the clamped values back, and updates two 32-bit position fields with overflow protection.
- `update_scroll_position`: The routine updates a pair of scroll offsets stored in a structure, clamps them to limits, derives 8-bit page values, calls other routines, writes back several fields including position deltas and a rectangle, and conditionally increments a counter based on a flag, a global variable, a table lookup and the CIAA_PRA register, toggling a byte in memory.
- `increment_and_toggle_byte`: Increments a global variable and toggles the value of a byte in a table based on a bit check in a hardware register.
- `empty_stub_routine`: The routine contains no instructions and performs no operations.
- `reset_collision_state`: The routine clears a flag byte and a short timer in a structure, then calls several other routines passing derived offsets from two short fields.
- `test_collision_and_handle_event`: The routine checks overlap of a mask with three words from a table entry and, if set, marks a global flag and calls another routine.
- `set_display_list_and_wait_vblank`: The routine updates several hardware registers to set up a display list and then waits for a vertical blank condition.
- `set_copperlist_pointer`: The routine builds a copper list entry from input values and writes it to copper registers, then updates a pointer table with the new list address.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `set_memory_flags_and_table`: The routine sets a flag byte in memory and a flag byte at an address one byte before a pointer, then writes a small table of four short values derived from a counter into memory starting at an offset from the pointer.
- `set_flag_and_fill_table`: The routine sets a flag byte at an offset before a pointer and fills a short table at a fixed offset from that pointer with decreasing values derived from a counter, writing four entries.
- `fill_short_table_descending`: The routine writes a descending sequence of three 16-bit values into a table pointed to by an address in a register plus a fixed offset.
- `shift_tile_row_data`: The routine moves a block of 16-bit tile entries between three tables in memory using offsets derived from two input values and a base pointer.
- `fill_palette_entries`: The routine fills a series of word entries in a table using values derived from a global variable and a table, writing a 16-bit value from a pointer and a 16-bit zero into two separate tables while iterating a counter.
- `return_stack_slot`: The routine returns the value of a stack slot passed in by its caller.
- `fill_and_clear_table_entries`: The routine writes a sentinel value to a series of table entries by repeatedly calling another routine and advancing a pointer returned by that call, then writes a computed value or zero to another series of entries using the same call-and-advance pattern, and finally writes the sentinel again to a further series of entries using the same pattern.
- `store_word_and_long_to_pointer`: The routine stores a 16-bit value from a register into the location pointed to by a pointer, then stores a 32-bit value from an unused register into the next four bytes after that pointer.
- `set_object_pointers`: The routine checks a flag byte in the object structure and, depending on its value, writes two pointer fields and a type identifier to a fixed offset in the object, then copies a value from another offset and stores it twice with a constant added to one copy.
- `process_sprite_dma_queue`: The routine walks a linked list of entries, updates their counters and, when a counter expires, sets a bit in a byte-mapped buffer.
- `set_collision_bit_and_queue`: The routine sets a bit in a byte table using a value derived from two input registers, then loops over a range of a register value, calling another routine to queue a linked structure when the value is out of range, otherwise it reads from a table to compute a position and writes several fields in a hardware control block, including a command word and timing values, and updates a small stack of pointers.
- `initiate_player_state`: The routine initializes a player state structure and a linked list node using input registers and a flag, then enters a loop that updates counters in a global linked list, manipulates a bitmap byte, and checks a table entry before breaking.
- `process_event_queue`: The routine walks a linked list of event structures, updates their counters, and for each entry whose timer has expired it sets a bit in a global bit table and decrements a counter field.
- `build_linked_list_node`: The routine computes a ratio from two signed 16-bit inputs and, if non-zero, calls another routine and uses its 64-bit result together with values read from a global variable to fill a small structure in memory.
- `link_object_list_entry`: The routine builds a linked list node in a caller-provided structure using values read from a global variable and from another routine, and links the node into a list whose head is stored in the global variable.
- `link_object_list`: The routine builds a small linked list node in memory using a global variable and a table entry, then fills five fields of the node with values derived from registers and the result of another routine.
- `build_linked_list_node_2`: The routine conditionally builds a linked list node using a global variable and a table.
- `dispatch_indirect_call`: Checks if two registers are zero before performing an indirect call to a routine using an offset from a global variable.
- `check_and_dispatch`: The routine checks two short values and returns if both are zero.
- `indirect_table_call`: The routine loads a short value from an address offset from an uninitialized register and uses it as an index into a global variable to obtain a function pointer, then calls that function indirectly.
- `call_indirect_from_table`: The routine reads a short value from an address derived from an uninitialized register, adds it to the address of a global variable to form a table index, and calls the routine found there via an indirect jump.
- `call_indirect_from_table_2`: The routine reads a short value from an address held in a register, adds it to the address of a global variable, and calls the routine found at the resulting address.
- `empty_routine_5`: The routine contains no instructions and returns immediately.
- `poll_bitplane_dma_and_copy`: The routine waits for a bitplane DMA sync flag in a global variable, then programs two custom chip registers for bitplane DMA, and if a table pointer is non-zero it clears the pointer and iterates over entries, writing a length and an offset to custom chip registers and waiting for the sync flag each iteration, updating a position register and a modulo register derived from a short value from the table.
- `decrement_counter_with_wrap`: The routine receives a pointer in a0 and accesses a short field at a fixed offset from it.
- `increment_short_at_pointer`: The routine increments a 16-bit value located two bytes before the address held in an uninitialized pointer register.
- `wait_for_dma_and_reset_blitter`: The routine spins until a global variable’s bit is clear, then writes to that global variable and to custom chip registers.
- `decrement_counter_and_schedule_event`: The routine checks a flag byte and a time comparison in a structure.
- `decrement_counter_and_schedule_event_2`: The routine checks a flag byte in the object and a time comparison, then calls another routine and updates a timer field by adding an offset to a limit field.
- `decrement_flag_and_schedule_event`: The routine checks a flag byte and a timer value in a structure, calls another routine, updates a timer field, then builds a message block in an output area with several fields and a command word, and finally decrements the flag byte.
- `check_and_schedule_event`: The routine checks a flag byte and a timer value against a limit stored in a structure.
- `swap_linked_list_entry`: The routine checks a vertical position from VPOSR against a value in the object and a global variable.
- `empty_routine_6`: The routine contains no instructions and returns immediately.
- `process_event_with_bounds_check`: Checks if an event's active area or or certain bounds-related values overlap with a specified or calculated range.
- `check_collision_bounds`: The routine checks that two short values lie within ranges defined by other registers and by two short values read from a structure pointed to by a pointer.
- `increment_counter_and_link_node`: The routine increments a byte at a fixed offset from a pointer in A0 and then calls another routine.
- `increment_bcd_counter`: The routine increments a byte at a fixed offset from a pointer in a0 and adds BCD-adjusted digits from a1 to a BCD counter stored at a fixed offset from a0.
- `increment_counter_and_link_list`: The routine increments a byte field in a structure pointed to by a0 and a short field in the same structure.
- `insert_node_into_list`: Updates a pointer in a linked list, increments a global counter, and appends a new node to a list managed by a pointer in a global variable.
- `manage_object_timer`: The routine updates a per-object countdown and bounds checks two 16-bit positions against a stored rectangle.
- `validate_and_dispatch_collision`: The routine checks a set of range conditions on several input values and two fields from a structure pointed to by a pointer.
- `process_bcd_score_time`: The routine adds four BCD values from RAM to four bytes in a score buffer and performs BCD adjustment on each.
- `update_display_position`: The routine updates a set of hardware registers with values derived from input parameters and a table pointed to by a pointer in memory.
- `decrement_counter_and_dispatch`: The routine reads a counter from a structure, decrements it and, if it reaches zero, increments a byte in a global variable, updates a linked list of pointers and increments a global counter.
- `process_collision_response`: The routine updates state in a table pointed to by A0 and A3 and branches through a function table.
- `spawn_effect_and_update_list`: The routine tests a collision mask derived from two coordinates and a table, then it either spawns a short-lived effect object by linking it into a global list and initializing its fields from the coordinates and registers, or it processes a timed effect by decrementing a counter and removing the effect from a linked list when the counter expires or the effect leaves a bounding box.
- `check_collision_bounds_2`: The routine checks whether two ranges defined by registers and a global variable overlap within limits stored in a structure pointed to by a pointer.
- `increment_counter_and_link_node_2`: The routine increments a byte at a fixed offset from a pointer in A0 and then calls another routine.
- `increment_bcd_counter_2`: The routine increments a byte in a global variable and adds BCD-adjusted values from a structure to four consecutive BCD bytes in another global variable.
- `increment_and_link_list`: The routine increments a byte field in the structure pointed to by a0 and a short counter in the same structure.
- `insert_node_into_list_2`: Performs the same logic as the previous routine, updating a linked list structure and incrementing a global counter or list pointer.
- `check_flag_bytes_and_recurse`: The routine checks four consecutive flag bytes in a structure and, if any flag is set, calls another routine and updates a pointer from its result.
- `calculate_buffer_offset`: Calculates a memory offset based on the product of three values and stores a pointer to a new memory region into a global variable.
- `link_collision_entry`: The routine checks a bounding box against two 16-bit values and, on hit, writes two integers and a routine pointer into a structure, links a global list node into a global variable, and returns a flag byte.
- `insert_node_to_list`: Checks if a value is within a range and either inserts a node into a linked list or clears the high bit of a byte in a structure.
- `dispatch_linked_node`: Checks if a value is within a range and either calls an indirect function from a table or updates a linked list and clears a bit in a byte.
- `update_timer_and_bounds`: Decrements a value in a structure, checks certain bounds or flags, and potentially decrements a global pointer or resets it to zero.
- `decrement_timer_counter`: The routine decrements a counter in memory and resets it when it reaches zero.
- `set_up_linked_list_node`: The routine calls another routine and, if a condition is false, writes a pointer to a global variable into an output structure, links the pointed-to node back to the structure, stores the node’s value, and fills several short fields in the structure with sums of input registers and constant values.
- `link_list_node_init`: The routine calls another routine and, if a zero flag is set, it builds a small linked-list node in memory pointed to by a register.
- `call_other_routine`: The routine performs a single call to another routine and then returns.
- `update_table_entry`: The routine adds a value derived from a global variable and an input word to a short field in a structure pointed to by a pointer, then XORs the result with a constant.
- `insert_linked_node`: The routine checks a bounding box against two 16-bit values and, if the test succeeds, writes three values into a structure, stores a pointer to a global variable in the structure, and links the structure into a singly linked list kept in that global variable by swapping pointers.
- `insert_linked_node_2`: The routine checks a bounding box against two 16-bit values and, if inside, writes three fields from registers into a structure, links the structure into a singly linked list whose head is stored in a global variable, and updates the predecessor link.
- `check_and_dispatch_or_clear_flag`: The routine checks a bounding box against two 16-bit values using a short table.
- `clear_flag_in_memory`: The routine clears a flag bit in a byte located at an offset from a base pointer.
- `double_table_entry`: The routine reads a short value from an object structure, uses it as an index into a global table and another global table, then doubles the indexed entry and adds the carry from doubling a 32-bit value, storing the result back into both tables.
- `call_other_routine_twice`: The routine calls another routine twice in succession and then returns.
- `shift_fill_table`: The routine writes 16 successive words from a register to a memory area pointed to by a pointer, shifting the register right by one bit each iteration.
- `decrement_counter_and_update_state`: The routine decrements a counter in a table and, when the counter reaches zero, updates a global variable with a sum, clears a flag, writes BCD-adjusted byte increments to four bytes in a structure, and calls another routine twice.
- `add_bcd_time_offset`: The routine adds a BCD-encoded time offset from a structure pointed to by a1 to four BCD bytes at fixed offsets from a pointer in a0, adjusting each byte after addition.
- `update_display_position_2`: The routine updates a global variable and two RAM locations with a byte derived from a table entry, then computes a screen position from registers and a table pointer and writes the result to several RAM locations.
- `store_value_and_set_backlink`: The routine reads a pointer and an integer from a structure pointed to by A3, writes the integer through the pointer, and if the integer is non-zero writes the pointer back into the word preceding the integer.
- `decrement_counter_with_clamp`: The routine clears a byte in a structure pointed to by a0, calls another routine, then reads a 16-bit value from an offset in a structure pointed to by a0, subtracts a constant from it, and writes the result back, clamping the value to zero if it would become negative.
- `check_bounds_and_dispatch`: The routine checks a set of range conditions on several registers and two fields from a structure pointed to by a pointer, then uses values from a table pointed to by another pointer to index into a jump table and call another routine.
- `validate_and_dispatch`: The routine checks four short values from a structure against two offsets from a short pointer, and if the comparisons succeed it calls through an indirect jump table; otherwise it calls another routine via an indirect address derived from the pointer.
- `call_indirect_via_table`: The routine reads a short value from an offset in a pointer passed in a1 and adds a base address to form an indirect address.
- `update_palette_entry`: The routine updates a palette entry in a table pointed to by a pointer in memory.
- `update_display_list_and_wait`: The routine updates a small control block pointed to by a register, advances a byte counter and a word index in a table referenced through a pointer, and writes several computed values to hardware registers.
- `set_blitter_copy_parameters`: The routine computes a blitter modulo and a vertical start position from its inputs and a table base, writes those values together with source pointers to the blitter control registers, waits for the blitter to become idle, and then updates a linked list of blitter control words by adjusting a word in the list and the associated destination offset, storing the updated list head back to the table.
- `fill_palette_table`: The routine fills a table pointed to by an argument with a repeating pattern derived from a global variable.
- `wait_for_flag_and_init`: The routine spins until a bit in a global variable is cleared, writes several values to custom chip registers, sets the same global variable, and then calls another routine, conditionally calls it again with a result, and returns a combined value from two further calls.
- `adjust_two_word_values`: The routine takes a signed 16-bit value from D0 and a pointer in A4 to two consecutive words.
- `clamp_and_accumulate`: The routine reads a signed value from D0 and a value from a global variable pointed to by A4.
- `set_palette_entry_and_schedule_update`: The routine writes a nibble derived from a global variable into a table entry and then combines it with the existing value.
- `store_word_pair_to_table`: The routine writes four 16-bit values through a pointer in A1.
- `process_animation_frames`: The routine iterates over a byte stream of frame counts and a parallel counter, updating a short counter, handling wrap-around, and repeatedly writing a computed address and a set of values to a shared structure while polling a busy flag.
- `update_animation_frames`: The routine walks a byte mask and a 16-bit index table, repeatedly writes a pointer, a base value and a control word to a hardware register block, waiting for a busy flag to clear between writes, and advances frame counters and offsets for two animation structures.
- `initialize_flyer_physics_state`: The routine writes a large set of constant and computed values into a structure pointed to by a0, initializing fields that appear to hold positions, velocities, flags and a small 16-byte pattern, and it copies a byte from a global variable into several fields.
- `setup_blitter_copy_parameters`: The routine computes source and destination positions from its inputs, writes a set of blitter control words and source/destination pointers into a structure pointed to by a0, programs the blitter registers BLTAPTH and BLTCON0 and then fills several blitter move descriptors with calculated offsets and lengths.
- `empty_routine_7`: The routine contains no instructions and returns immediately.
- `empty_routine_8`: The routine contains no instructions and returns immediately.
- `clear_flag_and_call`: The routine masks a global variable to clear a bit and set another bit, then conditionally calls another routine if the cleared bit was previously clear.
- `do_nothing`: The routine contains no instructions and returns immediately.
- `set_flag_and_call`: The routine checks a flag byte in a structure and, if the flag is clear and a global variable is below a threshold, it writes a constant to a global variable, updates the global variable to a fixed value, and calls another routine.
- `set_global_and_call`: The routine checks a flag byte in a structure and a value in a register.
- `set_timer_and_call`: The routine checks a flag byte in a structure and a value in a register.
- `set_timer_and_call_2`: The routine checks a flag byte in the structure pointed to by a0 and a value in d0.
- `set_timer_and_call_3`: The routine checks a flag byte in a structure and a global variable, then writes a constant to a global variable, updates the global variable to a fixed value, and calls another routine.
- `set_global_and_call_2`: The routine checks a flag in a structure pointed to by a pointer and a value in a register.
- `check_flag_and_trigger_callback`: The routine checks a flag byte in a structure and, if the flag is clear and a counter is below a threshold, it writes a constant to a global variable, updates the counter, and calls another routine.
- `check_and_trigger_flag`: The routine checks a flag byte in the structure pointed to by a0 and a value in d0.
- `check_and_set_flag`: The routine checks a flag byte in the structure pointed to by a0 and a value in d0.
- `decrement_global_counter`: The routine checks a global variable and, if it is non-zero, decrements it and tests whether the result is zero.
- `set_global_flag_2`: The routine sets a global variable to one and returns.
- `set_global_flag_from_object_byte`: The routine writes a constant byte value to a global variable and then conditionally clears it to zero based on the least significant bit of a byte located at an offset from a pointer argument.
- `copy_short_table_with_early_exit`: The routine copies a short table from a source structure into a global table, writing a second short field from the same source for each entry.
- `copy_short_table`: The routine copies a short table from a source structure into a global table, storing each source value and a companion value from a second source offset.
- `check_and_call_other_routine_2`: The routine checks a signed global variable and two other global variables.
- `reset_state_flags`: The routine clears several global variables to zero, sets one global variable to a constant, stores a value from a global variable into an entry of a table indexed by a masked global variable, and clears three additional global variables to zero.
- `fill_global_with_decrementing_short`: The routine repeatedly writes a signed 16-bit value to a global variable, decrementing it each iteration from -1 up to -4095, with the loop terminating when the value reaches -1.
- `check_and_trigger_updates`: The routine checks a signed global variable and, depending on its sign and the values of two other global variables, either clears a global flag or calls another routine one or two times.
- `process_message_word`: The routine saves a global word, extracts bytes from an input word and uses a nibble from the input to index a per-channel structure in RAM. If the channel is enabled it stores parts of the input word into the channel’s fields, updates a few flags and copies a value from a table indexed by the low bits of the input.
- `self_referential_init`: The routine writes a global variable to a RAM location, makes a global variable point to itself, and stores the address of another routine in a RAM location.
- `check_and_reset_flag`: The routine tests a signed global variable and, if negative, checks two other global variables for zero.
- `check_two_flags_and_branch`: The routine checks two short values referenced through an address register.
- `call_two_routines`: The routine calls another routine twice and returns the value it received in a stack argument.
- `empty_stub_routine_2`: The routine contains no instructions and returns immediately.
- `decrement_and_wait_for_flag`: The routine checks a counter in the structure pointed to by a6.
- `copy_word_and_wait_flags`: The routine copies a word from a source structure to a destination structure and then waits until a flag word and a byte flag are both non-zero, repeatedly clearing the byte flag and calling another routine.
- `clear_state_and_retry`: The routine repeatedly clears a byte at an offset in a structure pointed to by A6, calls another routine, and checks the same byte after each call, returning early if the byte stays zero.
- `process_script_state`: The routine updates a byte field in a structure using a value from a table indexed by a short field, then loops while the byte is below a threshold, calling another routine when a flag is clear.
- `execute_script_opcode`: Executes a conditional or or logic-based command by interpreting an opcode and updating object or pointer-based state.
- `update_object_state`: The routine updates fields of a structure pointed to by a0 using values from a structure pointed to by a6 and a table pointed to by a4.
- `process_message_state`: The routine reads a message state byte from a structure and dispatches on its low nibble, updating fields in the message object and a related control block, handling timers, counters and a small state machine.
- `process_script_command`: The routine reads a byte from a table pointed to by a structure and dispatches on its low nibble, updating fields in the structure and in a second structure accessed via a pointer.
- `increment_short_field`: The routine increments a 16-bit value located at an offset from a pointer passed in a0.
- `populate_object_attribute_table`: Iterates through a table of attribute or pointer values to update a list of object properties, including calculating or clearing certain target coordinates and range-limiters.
- `load_object_pointers`: The routine reads a set of word entries from a table pointed to by a structure and, for each valid entry, stores the entry and computes a pointer by adding a base address to an offset taken from a second table, then clears associated fields.
- `initialize_entity_offsets`: The routine copies six word values from a source pointer and an optional seventh into a structure in A5, and for each valid entry it writes a base-plus-table offset into a long field, clears a long field and sets a word field to all ones.
- `reset_state_flag`: The routine calls another routine and then clears a word at an offset from a register, sets a global variable to a constant, and returns the value originally on the stack.
- `process_script_opcodes`: The routine processes a script-like opcode stream stored in a global table.
- `process_sound_script`: The routine processes a byte-code script stored in RAM and drives the Amiga audio hardware via AUD0LCH. It decodes opcodes from a table, updates several global variables and counters, and handles script flow control such as jumps, delays and termination.
- `execute_sound_script_opcode`: Parses and executes a command or value from a script buffer, modifying sound-related state or pointer offsets.
- `handle_script`: A routine that updates fields of a structure pointed to by a pointer and another structure, applying a counter and a scaling calculation from bytes in the structures, then dispatches work based on a script opcode stored in the structure.
- `process_object_state`: The routine indexes a table via a pointer stored in a structure and dispatches on a byte field from the fetched entry.
- `increment_short_at_offset`: The routine increments a 16-bit value located at a fixed offset from a base pointer.
- `decode_script_command`: The routine reads a command byte from a table entry and performs a switch on its value, updating fields in a structure pointed to by an argument and a second pointer.
- `decrement_repeat_counter`: A routine that reads a counter from a structure and returns a value derived from it, updating an accumulator and a secondary field when a flag is set.
- `decrement_timer_and_accumulate`: The routine checks a flag byte in a structure and, if set, adds a byte value to a short accumulator and optionally mirrors the sum to another structure.
- `update_wrap_around_counter`: The routine updates a 16-bit counter stored in a structure pointed to by a register, handling wrap-around using a signed offset field and a byte flag.
- `update_timer_word`: The routine reads a word from an offset in a structure pointed to by a register, adds an input word, and compares it with another input word.
- `process_delayed_byte_value`: The routine works on a byte structure pointed to by a register.
- `wrap_bytes_in_byte_buffer`: The routine uses A5 as a pointer to a small byte buffer.
- `copy_byte_and_clear`: The routine copies a byte from an offset within a structure pointed to by A5 to another offset in the same structure, then clears the source byte to zero.
- `accumulate_and_reset_counter`: The routine adds a byte from an object structure to another byte in the same structure and compares the result with a byte in D0.
- `decode_packet_header`: The routine decodes a 32-bit value passed in D0 and uses a byte from a global variable to select one of four sub-structures in a table.
- `set_entity_state_from_table`: The routine copies bytes from a source structure to a destination structure and, for certain source type values, indexes a global table with a value derived from an input register and writes the result into the destination.
- `update_entity_state`: Checks a byte in a structure and either clears a byte in another structure or copies several fields, including a calculated pointer, from one structure to another.
- `copy_object_fields_conditional`: The routine copies several bytes from a source structure to a destination structure, using a value derived from a register to index into a table in the source structure for a pointer assignment.
- `copy_actor_state_fields`: Copies a few bytes from one structure pointed to by a register to another structure pointed to by a register, sets a flag byte to one, clears a byte, copies a byte and a word, selects a word from a global table using a masked byte from the source structure as index and stores it, and stores a word from the stack into the source structure before returning the concatenation of an argument and a register.
- `clear_dma_channel`: The routine selects an entry from a table using the low two bits of the input word and copies the corresponding value into a global variable.
- `reset_game_state`: The routine clears and initializes a block of state pointed to by A6, copies values from a table indexed by a field in that block, updates several global variables and a small table, and then zeroes a set of globals.
- `reset_game_state_flags`: The routine sets a global variable to one, clears another global variable, and clears a two-byte field via an uninitialized pointer.
- `set_flagged_global_from_register`: The routine reads a register value and stores it ORed with a constant to a global variable.
- `reset_audio_channels_6`: The routine clears several global variables, sets a global variable to a constant, stores a value from a global variable into an entry of a table indexed by a masked global variable, and clears four additional global variables.
- `store_self_reference_in_ram`: The routine writes a global variable into a RAM location, then stores the address of that global variable back into the global variable itself, and finally stores the address of another routine into a RAM location.
- `clear_color00_register`: The routine clears the color register by writing a sequence of values to a global variable, effectively resetting COLOR00.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_copper_and_wait`: The routine clears a global variable, fills a short table with a sequence of values and copies from another table, then builds a series of copper list entries with fixed words and a pointer, clears a block of memory, and enters a loop that waits for a global variable to change and checks a memory location before calling another routine repeatedly, returning a value from D1 when a flag in the global variable is clear.
- `empty_loop_wait`: The routine contains no executable instructions; it is an empty loop that waits for a global variable to equal a specific character and then loops forever.
- `fill_score_table`: The routine reads a block of bytes from a table, adds a running offset to each byte and stores the results as words into another table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `perform_empty_operation`: The routine performs no operations and returns immediately to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_9`: The routine contains no instructions and returns immediately.
- `empty_routine_10`: The routine contains no instructions and performs no operations.
- `empty_routine_11`: The routine contains no instructions and returns immediately.
- `empty_routine_12`: The routine contains no instructions and performs no operations.
