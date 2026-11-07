# Game 1168 (fate - gates of dawn (1991)(reline)(disk 1 of 3)[cr fsn]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01168). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907856 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 64 of 64 named (0 author symbols), 64 of 64 described (5 by rule, 10 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1020 of 1020 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1664 of 1664 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3168 |
| code | 5198 |
| data | 787435 |
| padding | 2999 |
| unused | 109056 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| TYPE | 2 | 117 | 2 |
| DIR | 9 | 29 | 2 |
| Fate - Gates of Dawn (1991)(reLINE)(Disk 1 of 3)[cr FSN].adf:FUSION | 6 | 233 | 7 |
| Fate - Gates of Dawn (1991)(reLINE)(Disk 1 of 3)[cr FSN].adf:FATE | 2 | 85 | 4 |
| Fate - Gates of Dawn (1991)(reLINE)(Disk 1 of 3)[cr FSN].adf:bootblock | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 56 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 385 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 748 | 35 |

## Cases handled

- runtime segment an address: 54 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 166 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 490 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- fate - gates of dawn (1991)(reline)(disk 1 of 3)[cr fsn]/Fate - Gates of Dawn (1991)(reLINE)(Disk 1 of 3)[cr FSN].adf: blocks 11-14 (bytes 5632-7680) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.37)
- fate - gates of dawn (1991)(reline)(disk 1 of 3)[cr fsn]/Fate - Gates of Dawn (1991)(reLINE)(Disk 1 of 3)[cr FSN].adf:C/DIR: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- fate - gates of dawn (1991)(reline)(disk 1 of 3)[cr fsn]/Fate - Gates of Dawn (1991)(reLINE)(Disk 1 of 3)[cr FSN].adf:FUSION: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 packed track region(s), 736768 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `setup_copper_and_enable_dma` (runtime segment an address): Disables all interrupts, enables a set of DMA channels and points the copper at its list before starting it. Kind: disk hardware/DMA setup for custom track reading. Pass path: execution continues to the next instruction after the routine completes (u6__sub_04E8).
- `acknowledge_interrupt_and_reset_control` (runtime segment an address): Writes a single interrupt-acknowledge bit, runs an unnamed routine, then writes a full byte mask to a control register. Kind: disk hardware/interrupt control (likely a timing or sync check). Pass path: the execution flow following the return of u6__sub_014C (the instructions immediately following the call site).

## Functions


### TYPE

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: Copies sixteen longwords from a structure field at offset 536 into a buffer whose pointer is passed on the stack, then calls a routine through a stored pointer.

### DIR

- `free_memory_blocks_loop`: Calls one helper, then frees memory blocks through the exec FreeMem call in a counted loop, clears a longword in a structure and calls another helper.
- `decrement_until_minus_one`: Decrements a 16-bit counter until it reaches minus one, a pure busy-wait loop with no calls or library use.

### Fate - Gates of Dawn (1991)(reLINE)(Disk 1 of 3)[cr FSN].adf:FUSION

- `free_memory_blocks_loop_2`: Calls one helper, then frees memory blocks through the exec FreeMem call in a counted loop of three passes, clears a longword, and calls a helper.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.
- `unpack_bit_coded_data`: Decodes bit-coded packed data by repeatedly reading control bits through a function pointer and copying literal or repeated runs into memory.
- `unpack_packed_chunk`: Decodes one chunk of the packed stream by copying literal bytes backwards from the source and reading bit flags, taking a second path when the bit buffer ends.
- `unpack_backward_stream_loop`: Main loop of the backward decompressor: it copies runs of bytes from the source into the destination, reads bit flags from a shifted bit buffer, and stops when the run counter reaches zero.
- `decode_backward_lz_copies`: Expands a bit-packed compressed stream by reading variable-length length and distance codes from a bit buffer and copying previously written bytes backwards into the output.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Fate - Gates of Dawn (1991)(reLINE)(Disk 1 of 3)[cr FSN].adf:FATE

- `start_decompression_entry`: Entry point that disables interrupts through the interrupt enable register and then runs the bitstream unpacker over the packed data.
- `return_global_value`: Returns a value stored in a global variable.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Fate - Gates of Dawn (1991)(reLINE)(Disk 1 of 3)[cr FSN].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_bad_instruction_data`: The code is only a stub: it writes a value to the colour 0 register and then reaches bad instruction data.
- `install_vectors_and_copy_block`: Writes the same fallback handler address into several low-memory interrupt vector slots and waits on the video beam position register.
- `clear_interrupt_request_bit`: Writes one bit to the interrupt request register to acknowledge a pending interrupt, then returns.
- `store_three_values_to_ram`: Stores two register parameters and one stacked value into three fixed RAM cells and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_copper_and_enable_dma`: Disables all interrupts, enables a set of DMA channels and points the copper at its list before starting it.
- `acknowledge_interrupt_and_reset_control`: Writes a single interrupt-acknowledge bit, runs an unnamed routine, then writes a full byte mask to a control register.
- `copy_table_with_stride`: Writes a short list of fixed value pairs into memory, then copies a 44-entry word table into a destination array spaced 16 bytes apart.
- `execute_unreachable_code`: An unreachable routine that contains an infinite loop.
- `execute_unreachable_code_2`: An unreachable routine that contains an infinite loop and calls another routine.
- `null_routine`: The routine is empty and consists of an infinite loop.
- `update_color_interpolation`: Invokes a routine to blend two packed 12-bit color values using a signed fractional multiplier.
- `interpolate_rgb_colour`: Blends two packed 12-bit colour values channel by channel using a signed fraction held in a register.
- `initialise_record_and_pointer_table`: Calls four unnamed routines, then fills a record with fixed starting values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `build_pointer_table_from_offsets`: Computes a set of absolute pointer slots by adding a base value to fixed table entries.
- `lookup_then_invoke_callback`: Looks up an entry through an indirect call and, if one is found, fills a small record and calls a second indirect routine on it.
- `wrap_call_with_flag_toggle`: Clears a flag word to zero, makes one unnamed call, then sets the flag to all ones before returning.
- `zero_global_block_and_call`: Clears a flag, zeroes a block of 776 bytes of memory starting at a fixed global, then makes one unnamed call.
- `service_audio_tick_counter`: Advances a tick counter and, when it reaches its limit, resets it and runs a further step.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `silence_audio_and_reset_state`: Sets the volumes of the four audio channels to zero and disables their DMA and interrupts.
- `open_ciab_resource_and_call`: Opens the ciab.resource through an exec library call and, if a handle comes back, fills a small structure and calls a second library entry on it.
- `install_musicirq_interrupt_handler`: Fills a small interrupt structure that references the musicirq name and passes it to a library call at a fixed offset.
- `return_constant_one`: Returns the constant value one.
- `return_doubled_global_byte`: Returns a 16-bit value made by repeating one global byte in both halves.
- `compute_and_write_timer_count`: Computes a timer count by dividing a fixed constant by a global divisor, falling back to a fixed default when the divisor is zero.
- `silence_audio_and_clear_block`: Clears 776 bytes of global memory, sets a global control byte to fifteen and zeroes the volume of audio channel zero.
- `store_pair_and_set_flag`: Copies one stored word into another global slot, stores the incoming value into a third slot and sets a global flag word to one.
- `reset_flag_and_clear_block`: Clears a global flag word, zeroes 776 bytes of global memory, then calls one unnamed routine and returns.
- `service_tick_slots_with_dma`: Advances a tick counter and, when it reaches its limit, resets it and runs a further step.
- `select_channel_data_pointer`: Reads the parameter record for the current channel.
- `service_four_audio_channels`: Calls two unnamed routines, then calls another routine for each of four audio channel pointers that is non-zero.
- `advance_sequence_step`: Advances a step counter and, when it reaches its limit, resets the counter, moves the sequence index forward with a loop point and loads four longword values for the next step from a table.
- `load_channel_values_from_table`: For each of four enabled channels, loads a longword from a table at the channel's stored index plus the current step and stores it in that channel's output slot.
- `perform_indirect_jump`: A twelve instruction routine that performs an indirect jump to an address held in a register.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a0, so the target is only known at run time.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `store_word_into_record_field`: Stores its incoming 16-bit value into a fixed offset within a record addressed by a register and returns nothing else.
- `decode_channel_command_byte`: Decodes a command byte for one audio channel.
- `bind_channel_table_entry`: Looks up a table entry for the channel's index.
- `copy_channel_block_into_buffer`: Copies a block of table data for the channel's index into a destination buffer in 16-byte groups.
- `load_channel_record_fields`: Stores the channel's index, locates the matching fixed-size record in the table and copies its timing, scaling and volume fields into the channel state.
- `copy_channel_timing_fields`: Copies timing, scaling and volume fields from a table record into the channel state and clears the related counters.
- `clear_channel_playback_state`: Zeroes several position, length and index fields of a channel state block and clears a status byte.
- `call_four_channel_routines`: Calls one unnamed routine four times in a row and returns.
- `update_channel_tick_state`: Advances one audio channel by a tick: it applies table-driven offsets, interpolates the level toward a target, updates the playback counters and clamps the stored volume between zero and 64.
- `run_channel_effect_step`: Counts down a per-channel timer and, when it expires, reloads it and applies one of several sample effects selected by an effect code.
- `execute_indirect_jump`: A single instruction routine that performs an indirect jump via a jump table.
- `negate_sample_byte_and_advance`: Negates the sample byte at the current position, advances that position by one and wraps it back to the start of the block when it passes the block end.
