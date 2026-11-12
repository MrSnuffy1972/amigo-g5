# Game 1191 (gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)[t +1 pdx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01191). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (910192 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 55 of 55 named (0 author symbols), 55 of 55 described (1 by rule, 31 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 333 of 333 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2048 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2268 of 2268 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 22436 |
| code | 11772 |
| data | 860109 |
| padding | 3 |
| unused | 15872 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf:bootblock | 1 | 161 | 10 |
| logged disk load from Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf boot-block load | 1 | 159 | 3 |
| logged disk load from Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf boot-block load | 1 | 0 | 0 |
| logged disk load from Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1402 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 546 | 32 |
| Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf bytes 1024-3072 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)[t +1 pdx]/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf: custom-format (non-DOS) disk, 1727 non-empty blocks: matched against track-loaded segments
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)[t +1 pdx]/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf: the boot block reads 5632 bytes from disk offset 861696 to an address (found statically); recovered as a unit
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)[t +1 pdx]/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf: the boot block reads 5632 bytes from disk offset 872960 to an address (found statically); recovered as a unit
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)[t +1 pdx]/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf: the boot block reads 5632 bytes from disk offset 884224 to an address (found statically); recovered as a unit
- runtime segment an address: 100 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 233 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)[t +1 pdx]/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf: bytes 1024-3072 look like code no run loaded: recovered position-independently (load address unknown)
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)[t +1 pdx]/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf: 4 logged loads cover 18944 bytes of the image
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)[t +1 pdx]/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf: 16 track-loaded blocks found in recovered runtime segments and classified through them
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)[t +1 pdx]/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf: blocks 95-95 (bytes 48640-49152) outside the filesystem hold data, not code (opcode density 0.012, entropy 5.59)
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)[t +1 pdx]/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf: blocks 1749-1753 (bytes 895488-898048) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.47)
- 1 never-loaded region(s), 2048 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 11 packed track region(s), 854016 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `dispatch_via_nibble_table` (bootblock): Calls two helpers, sets a high bit on a status byte, then loops over a counter and dispatches through a jump table indexed by the low nibble of a computed offset. Kind: looping/decrementing counter-based loader/protection. Pass path: the branch where bVar2 (the counter) or the indirect jump/table lookup is executed.
- `drive_cia_bits_with_delay` (bootblock): If the input word is non-negative, derives a bit mask from its low bits and writes it to the port B control variable, using a second value when the flag argument is set. Kind: disk hardware/sync check. Pass path: the branch where in_D0 (the check result) >= 0, specifically resulting in the execution of u0__sub_01DE()} lack of a simple load/store or loop, but rather a conditional bit-masking and assignment to a control register (DAT_000004f1) followe.
- `wait_cia_port_bit_clear` (bootblock): Calls a helper routine, then polls the port B status byte until bit five is clear. Kind: disk/track check (sync/bit check). Pass path: the loop terminates and execution continues to the next instruction (after the bit clears).
- `set_flag_then_wait_clear` (bootblock): Sets bit two of a status byte, calls a helper routine, and waits until bit five of the status byte clears. Kind: disk hardware/sync check. Pass path: the loop terminates, allowing execution to continue to the next instruction (the return or any instruction following the call or the loop exit).
- `decode_raw_disk_sectors` (bootblock): Starts a raw disk read with the sync word, searches the buffer for a sector sync, and decodes ten sectors. Kind: sync/track-based protection/pattern detection. Pass path: bsr.w an address.
- `update_audio_channel_sequence_step` (runtime segment an address): Per-channel audio update that reads the next sequence byte and combines it with a running offset to look up a period value. Kind: track/sync check (looping or conditional branch based on hardware-specific lookup or track/sector validity). Pass path: The loop continues or returns normally if the check (e.g., checking for zero/validity or non-zero status in pointer/memory checks) does not trigger a jump or an alternative branch; specifically, the loop continues if `*pbVar9` (the byte che.
- `update_audio_sample_parameters_3` (runtime segment an address): Iterates through a list of audio channels to update sample pointers, volume levels, and frequency or period settings. Kind: checksum/data-integrity trap. Pass path: The loop continues or returns via the `uVar6 == 4` exit condition, bypassing the modification of `_DAT_00dff096` and the conditional logic that triggers `u5__start_sample_on_audio_channel` only if certain RAM flags are non-zero..
- `play_audio_samples` (runtime segment an address): Counts a loop variable down from 512 to zero and returns, otherwise it checks four different memory locations and calls another routine to start a sample on a specific audio channel if the byte at the location is non-zero. Kind: loader/sabotage path. Pass path: sub_0D40.

## Functions


### Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf:bootblock

- `load_disk_ranges_and_jump`: Sends three disk read requests through device I/O into fixed memory addresses, then sets interrupt and status words and calls a helper twice.
- `copy_and_call_setup_block`: Writes disable-all interrupt and DMA control words and a colour register value, then calls two helpers.
- `dispatch_via_nibble_table`: Calls two helpers, sets a high bit on a status byte, then loops over a counter and dispatches through a jump table indexed by the low nibble of a computed offset.
- `repeat_two_helpers_full_cycle`: Calls two helper routines once per pass over a sixteen-bit counter, looping until the counter wraps, then returns.
- `drive_cia_bits_with_delay`: If the input word is non-negative, derives a bit mask from its low bits and writes it to the port B control variable, using a second value when the flag argument is set.
- `wait_cia_port_bit_clear`: Calls a helper routine, then polls the port B status byte until bit five is clear.
- `wait_for_status_bits_clear`: While a status bit is set, calls two helper routines repeatedly.
- `set_flag_then_wait_clear`: Sets bit two of a status byte, calls a helper routine, and waits until bit five of the status byte clears.
- `countdown_delay_loop`: Counts a word down through about 4000 steps and returns.
- `decode_raw_disk_sectors`: Starts a raw disk read with the sync word, searches the buffer for a sector sync, and decodes ten sectors.

### logged disk load from Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf boot-block load

- `write_pattern_word_by_flag`: Writes one of two fixed alternating-bit pattern words to the location given by an address register, choosing the value by bit two of a register, and returns.
- `refill_bit_buffer_step`: Calls a helper to fetch a longword, then returns it shifted right by one with the top bit set.
- `shift_bit_buffer_count_times`: Shifts a bit buffer right once for each count, reloading it through a helper routine whenever it becomes zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_via_nibble_table_2`: Advances a pointer by sixteen and decrements a counter.
- `setup_bitplane_data`: Configures several hardware registers for display and DMA, then enters a loop that writes four 32-bit values to a table by splitting them into two 16-bit words and incrementing the source value by 8000 each iteration.
- `idle_in_endless_loop_stub`: Contains only an endless loop with no other work.
- `configure_blit_and_wait_idle`: Writes blitter masks, modulo values, and a size value, and loads the two address inputs into the blitter pointer registers.
- `update_color_register`: Shifts the value in a register left by one and uses it as an offset to write the contents of another register into a color register.
- `transform_data_blocks`: Uses nested loops to copy bytes from one memory location to another, where the destination pointer is periodically adjusted based on a counter and a value read from the source.
- `copy_and_transform_data`: Negates a byte in d3, and if it is not negative, it enters nested loops to move bytes from a0 to a1 and perform arithmetic on a1 and d0 before branching to another routine.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `decode_joystick_movement_index`: Sets a state byte to all ones or zero from its top bit, then folds two low bit pairs into an index.
- `advance_counter_and_call_handler`: Advances a counter by two and reads a value from a helper.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_returns_stack_word`: Returns a value read from the stack with no other work.
- `call_two_helpers_and_return`: Calls two helper routines and returns the two data register values it received.
- `call_silence_routine`: Saves a3 to the stack, calls another routine, and restores a3 from the stack before returning.
- `call_audio_config_routine`: Saves d1, d7, and a3 to the stack, calls another routine, and restores them from the stack before returning.
- `call_audio_reset_and_restore_registers`: Saves the current contents of registers d0 through d3, d7, and a0 through a6 to the stack, calls reset_audio_and_play_sample, restores the saved registers, and returns.
- `call_flag_set_routine`: Saves a3 to the stack, calls another routine, and restores a3 from the stack before returning.
- `call_subroutine_and_restore`: Calls a routine at a fixed address and returns.
- `silence_audio_channels_and_clear`: Clears the state of four audio channels and writes silent volume and control values to the sound hardware.
- `onc`: Sets four state flag bytes in globals to one.
- `configure_audio_hardware`: If a global byte is non-zero, it sets a hardware register to 255, performs a loop to set bits in a register, writes that register to another hardware register, and sets a global byte.
- `set_status_bytes`: Loads a pointer into a3, stores the value 3 into offset 113 and offset 114 of the structure pointed to by a3, and returns.
- `run_sound_channel_frame_update`: A per-frame sound driver that updates four audio channels.
- `update_audio_channel_sequence_step`: Per-channel audio update that reads the next sequence byte and combines it with a running offset to look up a period value.
- `update_audio_sample_parameters`: Updates audio channel registers with new sample or frequency data, and checks a counter to trigger sample playback or stop audio channels.
- `update_audio_channel_sample`: Iterates through audio channel structures to update sample pointers, calculate next volume or pitch, and decrement remaining sample counts.
- `update_audio_sample_parameters_2`: Updates audio channel registers with new sample or frequency data, calculating volume or pitch-shifting offsets based on a table or pointer-driven or loop-driven logic, and triggers sample playback if certain conditions are met.
- `update_audio_sample_parameters_3`: Iterates through a list of audio channels to update sample pointers, volume levels, and frequency or period settings.
- `modify_byte_and_branch`: Sets a byte at an offset from a pointer and then branches to another location.
- `move_byte_to_offset`: Moves a byte from a pointer to an offset of 3 from another pointer and then branches to another location.
- `move_two_bytes_to_offset`: Moves two consecutive bytes from a pointer to offsets 8 and 9 of another pointer, writes zero to an offset of a pointer, and then branches to another location.
- `extend_and_store_byte`: Moves a byte from a pointer to a register, extends the register to a word, stores that word in a global variable at a specific offset, and then branches to another location.
- `store_byte_from_pointer`: Moves a byte from a pointer to a global variable at offset 113, moves a constant byte to a global variable at offset 114, and then branches to another location.
- `update_sound_parameter`: Moves a byte from a pointer in a1 into d0, sign-extends it to a word, stores the word at offset 100 of the structure in a3, and branches to another routine.
- `process_data_table`: If a byte at offset 119 of a3 is zero, it loops 16 times to read a longword and a word from a table, calculates a value by dividing an address by the word, and stores the results in a global structure before updating two pointers in a3 and setting the byte at offset 119 to 255.
- `process_scaled_table`: If a byte at offset 118 of a3 is zero, it calculates a value by multiplying a word from a table by 50 and dividing by another word, then loops once to read a longword and a word from a table, calculates a division result using an address, and stores these values into a global structure and a byte at offset 14 of a5 before setting the byte at offset 118 to 255.
- `reset_audio_and_play_sample`: Resets the audio hardware, selects a specific audio channel based on a register value, and updates a structure with sample data including volume, period, and length from a table.
- `copy_and_play_audio`: Sets audio hardware registers, enables a DMA channel, copies 12 words from one memory location to another in a loop, and then updates a structure with values from a table and calls another routine.
- `calculate_table_offset`: Calls another routine, loads a byte from a constant offset, and then either branches to another routine or adds a constant to a pointer and branches to another routine.
- `configure_audio_and_play`: Enables a DMA channel, sets audio hardware registers, calculates an offset into a table using a byte from a structure, and then updates several hardware registers and structure members before calling another routine.
- `calculate_offset_values`: Adds a longword from a0 to d0, calculates a word by subtracting the half of d0 from a word at offset 8 of a0, stores the word at offset 164 of a6, and stores the new longword at offset 0 of a6.
- `play_audio_samples`: Counts a loop variable down from 512 to zero and returns, otherwise it checks four different memory locations and calls another routine to start a sample on a specific audio channel if the byte at the location is non-zero.
- `start_sample_on_audio_channel`: Starts a sample on one audio channel.
