# Game 1191 (gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)[t +1 pdx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01191). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (910192 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 27 of 27 named (0 author symbols), 27 of 27 described (0 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 333 of 333 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2048 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 748 of 748 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 25722 |
| code | 4644 |
| data | 863951 |
| padding | 3 |
| unused | 15872 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf:bootblock | 1 | 161 | 10 |
| logged disk load from Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf boot-block load | 1 | 159 | 3 |
| logged disk load from Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf boot-block load | 1 | 0 | 0 |
| logged disk load from Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2)[t +1 PDX].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 117 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 311 | 9 |
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
- `idle_in_endless_loop_stub`: Contains only an endless loop with no other work.
- `configure_blit_and_wait_idle`: Writes blitter masks, modulo values, and a size value, and loads the two address inputs into the blitter pointer registers.
- `decode_joystick_movement_index`: Sets a state byte to all ones or zero from its top bit, then folds two low bit pairs into an index.
- `advance_counter_and_call_handler`: Advances a counter by two and reads a value from a helper.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_returns_stack_word`: Returns a value read from the stack with no other work.
- `call_two_helpers_and_return`: Calls two helper routines and returns the two data register values it received.
- `run_sound_channel_frame_update`: A per-frame sound driver that updates four audio channels.
- `update_audio_channel_sequence_step`: Per-channel audio update that reads the next sequence byte and combines it with a running offset to look up a period value.
- `update_audio_sample_parameters`: Updates audio channel registers with new sample or frequency data, and checks a counter to trigger sample playback or stop audio channels.
- `update_audio_channel_sample`: Iterates through audio channel structures to update sample pointers, calculate next volume or pitch, and decrement remaining sample counts.
- `update_audio_sample_parameters_2`: Updates audio channel registers with new sample or frequency data, calculating volume or pitch-shifting offsets based on a table or pointer-driven or loop-driven logic, and triggers sample playback if certain conditions are met.
- `update_audio_sample_parameters_3`: Iterates through a list of audio channels to update sample pointers, volume levels, and frequency or period settings.
- `start_sample_on_audio_channel`: Starts a sample on one audio channel.
