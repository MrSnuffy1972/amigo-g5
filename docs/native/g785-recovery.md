# Game 785 (birds of prey (1991)(electronic arts)[cr razor](disk 1 of 2)(intro)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00785). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901200 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 147 of 147 named (0 author symbols), 147 of 147 described (23 by rule, 18 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1100 of 1100 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2608 of 2611 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 734823 |
| code | 9616 |
| data | 104311 |
| padding | 4834 |
| unused | 47616 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:intro | 1 | 188 | 36 |
| Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:ar | 2 | 1699 | 73 |
| Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:add44k | 3 | 475 | 32 |
| Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:execute | 2 | 117 | 2 |
| Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:cd | 2 | 117 | 2 |
| Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |

## Cases handled

- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (play-long-keys): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- birds of prey (1991)(electronic arts)[cr razor](disk 1 of 2)(intro)/Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:intro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:intro

## Copy protection found

- none found

## Functions


### Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:intro

- `relocate_code_and_jump`: Disables interrupts, enables a DMA channel, copies a block of code into a RAM buffer, and then calls the routine at the start of that buffer.
- `unpack_backward_packed_stream`: Copies a block of packed data to the end of its buffer, then decodes it from end to start.
- `unpack_backward_packed_literals`: Decodes bits from a longword stream read backward, chooses a copy length from the bit pattern, and writes literal bytes through a jump table before the stream is exhausted.
- `dispatch_unpack_handler`: Calls a handler through an unrecovered jump table pointer and returns.
- `store_byte_and_dispatch`: Writes a byte to the address before a pointer, then calls one of two handlers from a table depending on whether a count is zero.
- `dispatch_by_byte_flag`: Calls a handler from a table, choosing between two entries by whether a byte value is zero.
- `forward_control_unrecovered_table`: Makes one indirect call through a jump table that the decompiler could not reconstruct, then returns.
- `forward_control_unrecovered_table_2`: Makes one indirect call through a jump table that could not be recovered, then returns.
- `forward_control_unrecovered_table_3`: Makes one indirect call through a jump table that the decompiler could not reconstruct, then returns.
- `call_jumptable_routine`: Performs an indirect call through an unrecovered jump table, then returns to the caller.
- `decode_bitstream_literals_and_runs`: Reads bits from a packed longword one at a time and selects a length class from them.
- `route_by_selector_below_six`: Compares a byte selector with six and jumps through one of two unrecovered pointers, one for values below six and one for all others.
- `forward_control_unrecovered_table_4`: Makes one indirect call through a jump table that could not be recovered, then returns.
- `dispatch_selector_in_range`: Checks whether a byte selector lies inside a small range and jumps through one of two unrecovered pointers accordingly.
- `forward_control_unrecovered_table_5`: Makes one indirect call through a jump table that the decompiler could not reconstruct, then returns.
- `dispatch_via_jumptable_2`: Looks up a value in a jump table and jumps to the selected routine.
- `forward_control_unrecovered_table_6`: Makes one indirect call through a jump table that the decompiler could not reconstruct, then returns.
- `dispatch_nonzero_selector_flag`: Tests whether a selector byte is nonzero and jumps through one of two unrecovered pointers depending on the result.
- `call_unrecovered_jumptable`: Performs a single indirect call through an unrecovered jump table, then returns to the caller.
- `dispatch_selector_below_three`: Compares a selector byte with three and jumps through one of two unrecovered pointers, the first for values below three and the second otherwise.
- `dispatch_selector_above_two`: Compares a selector byte with two and jumps through one of two unrecovered pointers, the second for values above two and the first otherwise.
- `choose_handler_above_two_pair`: Compares a selector byte with two and jumps through one of two unrecovered pointers, one for values above two and one for the rest.
- `route_via_jumptable`: Reads a byte selector from a global variable and uses it to index into a jump table, then transfers control to the routine selected by that entry.
- `dispatch_via_jumptable`: Looks up an entry in an unrecovered jump table using a value and then jumps to the selected routine.
- `dispatch_jumptable_routine`: Dispatches to one entry selected by an unrecovered jump table, then returns to the caller.
- `copy_bytes_backwards_counted`: Copies a counted block of bytes from the end of a source region toward its start, decrementing the source and destination pointers each step.
- `route_via_selector`: It reads a byte selector from a global variable via a jump table and transfers control to the routine that selector selects.
- `route_via_jumptable_3`: It performs an indirect jump through a jump table indexed by a byte selector read from a global variable, transferring control to the routine selected by that value.
- `route_via_jumptable_2`: Loads the address of a jump table, passes a value to dispatch_selector_below_three, and then jumps through the selected table entry to run another routine.
- `decode_bitstream_copy_stage`: Walks a bit-packed input word by word, copying literal bytes forward into the output and then copying a run of earlier bytes.
- `decode_bitstream_bit_refill_loop`: Shifts bits out of a packed longword and reloads the next word from the input when the register runs empty.
- `decode_bitstream_refill_then_copy`: Refills the bit register from the input when it is exhausted, then copies literal bytes forward and a run of earlier bytes into the output.
- `unpack_backward_literal_runs`: Copies an initial region forward, then walks an encoded byte stream and writes literal or repeated bytes into the destination backwards.
- `write_byte_then_enable_dma`: Stores one byte at the current output position and writes two 16-bit values into a custom chip register block, setting the master DMA enable and interrupt enable bits.
- `copy_escape_coded_bytes_backwards`: Copies bytes backwards from a source buffer and treats two marker byte values specially: one marker starts a repeat run and the other ends the copy.
- `copy_escape_coded_zero_runs`: Copies bytes backwards from a source buffer, inserting a run of zero bytes whose length is taken from the byte before it when a marker value appears.

### Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:ar

- `run_animation_sound_sequence`: Program entry point that loads and plays a sequence of animation and sound data files, touching the DMA control and the joystick and disk status port.
- `open_dos_and_graphics_libraries`: Opens two libraries through the exec open-library call, then requests the DOS output handle, storing each result.
- `close_two_library_bases`: Calls the exec close-library routine twice, releasing the two library bases that were opened earlier.
- `wait_disk_port_or_halt`: Tests one bit of the CIA port register and, when that bit is set, busy-waits until a register reads a fixed value and then loops forever.
- `free_nonzero_memory_blocks`: Walks eleven saved address and size pairs and frees each one whose two stored words are both nonzero, using the exec free-memory call.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `write_error_message_to_output`: Writes a fixed file access and memory error message to the stored DOS handle using the DOS write call.
- `check_file_steps_with_cleanup`: Clears two file-state fields, stores the given argument, and runs three checks in order, stopping at the first failure.
- `lock_and_examine_file`: Locks a file through the DOS lock call and then examines the lock with the DOS examine call.
- `unlock_stored_file_lock`: Calls the DOS unlock routine on the stored lock handle, but only when that handle is nonzero.
- `open_read_close_file`: Opens a file through the DOS open call, reads from it, then closes it.
- `allocate_memory_block`: Allocates a memory block through the exec allocation call, using one of two equivalent request forms, and stores the returned address.
- `free_block_if_allocated`: Frees the stored memory block through the exec free-memory call, but only when both of its stored words are nonzero.
- `run_coded_command_stream`: Reads a byte-coded command stream and dispatches on each opcode to set timing, copy small tables, adjust counters or branch within the stream.
- `wait_button_edge_with_flash`: Waits for a button bit in the pot port register to go low and then high, setting the background color to a fixed red value while it waits.
- `blit_copy_fixed_region`: Waits for the blitter to be idle, then programs a straight copy of a 40-word wide, 125-row region from source to destination.
- `blit_copy_fixed_region_reversed`: Performs the same fixed 40-word by 125-row blitter copy as its neighbour, but with the two pointer values swapped.
- `blit_clear_fixed_region`: Waits for the blitter to be idle, then starts a blit that writes zeros to a fixed 40-word by 125-row destination region.
- `fade_palette_colors_down`: Repeats fifteen steps in which it walks 32 colour words and decreases each 4-bit channel by one, stopping at zero, and waits on a flag between passes.
- `allocate_split_buffer_pointers`: Requests an 80000-byte memory block through the exec allocation call and stores its start as two buffer pointers, the second 40000 bytes further on.
- `free_split_buffer_pointers`: Frees the 80000-byte buffer through the exec free-memory call, but only when both stored buffer words are nonzero.
- `decode_rle_bitplane_rows`: Decodes a run-length coded bitmap into rows spaced forty bytes apart.
- `interpret_stream_timed_entry`: Enters a byte-coded command interpreter that first waits on a raster position and a CIA port status bit.
- `interpret_stream_copy_entry`: Starts a byte-coded command interpreter by copying thirty-two entries first, then dispatches the same opcodes for timing, counters and branches.
- `interpret_stream_direct_entry`: Loads a value from the stream and enters a byte-coded command interpreter that dispatches opcodes for timing, counters and branches.
- `interpret_stream_skip_entry`: Enters a byte-coded command interpreter at a point that reads a counter adjustment before dispatching opcodes.
- `interpret_stream_counter_entry`: Enters a byte-coded command interpreter that adds a parity-toggled or literal counter value before dispatching opcodes.
- `execute_command_stream`: Interprets a custom bytecode stream to perform graphics operations, such as bitplane decoding, area clearing, and block copying, or to update a table of function pointers.
- `interpret_stream_plain_entry`: Enters the shared byte-coded command interpreter directly at the current stream pointer, dispatching opcodes for timing, counters and branches.
- `process_coded_command_stream`: Runs a byte-coded command stream, dispatching on each opcode to copy small tables, adjust counters, swap two global variables, and follow or set pointers.
- `interpret_stream_slot_init_entry`: Initializes one command slot from the stream value, clearing its counter and storing its pointer, then enters the shared command interpreter.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `interpret_stream_entry_b`: Enters the shared byte-coded command interpreter after a slot lookup from the stream value, dispatching opcodes for timing, counters and branches.
- `run_coded_command_stream_2`: Reads a byte-coded command stream and dispatches on each opcode to set timing, copy small tables, adjust counters or branch within the stream.
- `interpret_stream_clear_slot_entry`: Clears one command slot and enters the shared byte-coded command interpreter.
- `interpret_stream_fill_slot_entry`: Stores a value into a command slot and enters the shared byte-coded command interpreter.
- `interpret_stream_load_slot_entry`: Loads a saved slot pointer from the stream value and enters the shared byte-coded command interpreter.
- `interpret_stream_call_helper_entry`: Calls another routine and then enters the shared byte-coded command interpreter.
- `build_command_slot_table_entry`: Builds a table of slot addresses from a list of offsets stored in memory, then enters the shared byte-coded command interpreter.
- `interpret_stream_save_return_entry`: Saves the current stream position as a return point and enters the shared byte-coded command interpreter.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `interpret_stream_restore_return_entry`: Restores the saved return position and enters the shared byte-coded command interpreter.
- `interpret_stream_raster_sync_entry`: Swaps two saved counter words, waits on a raster position and then enters the shared byte-coded command interpreter.
- `process_coded_command_stream_2`: Runs a byte-coded command stream, dispatching on each opcode to set timing, copy small tables, adjust counters, or branch within the stream.
- `interpret_stream_set_flag_entry`: Sets a control flag to minus one and enters the shared byte-coded command interpreter at the current stream pointer.
- `interpret_stream_clear_flag_entry`: Clears a control flag to zero and enters the shared byte-coded command interpreter at the current stream pointer.
- `interpret_stream_conditional_entry`: Branches on a control flag and, when it is clear, skips forward in the stream before entering the shared byte-coded command interpreter.
- `interpret_stream_store_table_entry`: Stores two stream bytes into a table entry selected by a third byte and enters the shared byte-coded command interpreter.
- `disable_interrupts_and_dma`: Saves the current interrupt-enable and DMA-control register values in memory, then writes masks that disable all interrupts and all DMA channels.
- `restore_interrupts_and_dma`: Writes the saved interrupt-enable and DMA-control values back to the chip registers with their set bit forced on.
- `start_copper_list_dma`: Points the copper at a list in memory, clears the second copper pointer and enables the copper, bitplane and master DMA bits.
- `set_copper_list_pointer`: Loads the copper list pointer from a stored structure, clears the second pointer and writes a DMA enable mask for copper, sprite and bitplane channels.
- `set_five_bitplane_pointers`: Writes five bitplane addresses into the display structure, each forty thousand bytes divided into eight-thousand-byte steps from the given base.
- `install_vblank_interrupt_handler`: Saves the current interrupt vector value, installs a new handler address in its place and writes an interrupt enable mask for the vertical-blank and master interrupt bits.
- `restore_saved_interrupt_vector`: Puts back the interrupt vector value saved earlier, reversing the handler installation.
- `vertical_blank_interrupt_handler`: Checks the vertical-blank request bit; when it is clear it resets a counter to a fixed value.
- `store_reg_into_global`: Stores a 32-bit value from a register into a fixed global location, then sets specific bits in two hardware control registers before executing a bad instruction.
- `restore_interrupts_and_dma_2`: Stores the value in register D1 into a longword at the address in A6, then sets specific bits in a hardware register and in a byte at a fixed address.
- `save_register_to_global`: Stores the value in a register into a global variable via a pointer, then sets bits in a hardware register and a global variable.
- `suppress_requesters_and_retry_lock`: Finds the current process, saves its window-pointer field and sets that field to minus one to suppress DOS requesters.
- `initialize_audio_channel_tables`: Fills thirty-two instrument records with addresses thirty bytes apart, scans another table for its largest entry, and clears the audio volume and output state.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `start_audio_channels_from_table`: Advances a frame counter and, when it restarts, waits briefly, loads the audio channel pointers from a table and enables audio DMA.
- `update_audio_channel_row`: Decodes the next packed step for one audio channel from a row table, choosing a value with a modulo-three counter, and points the channel at its sample start.
- `play_tracker_pattern_row`: Decodes one pattern row for a channel: it reads a sample number and period from packed bytes, looks up thirty-byte instrument records, and programs the channel's pointer, length, period and volume registers with DMA enabled.
- `apply_tracker_effect_command`: Runs the effect command for one audio channel on each tick, changing volume or pitch from a lookup table selected by a modulo-three counter, and then writes the resulting period register.
- `find_nearest_period_index`: Searches a per-instrument period table for the first entry not greater than the current pitch, clamps to the last entry and sets a direction flag.
- `slide_pitch_toward_target`: Moves a 16-bit pitch value up or down by a rate byte, clamps it at a target, and then looks up the nearest period table index.
- `compute_vibrato_offset`: Computes a pitch offset from a waveform table or a square or ramp shape, scaled by a depth nibble, adds it to the base period and advances the phase.
- `process_tracker_row_commands`: Handles the row command for one audio channel, covering effects such as sample offset and pattern position jump.
- `store_channel_nibble_field`: Copies the low four bits of a parameter byte into a per-channel field and returns.
- `advance_sample_pointer_and_negate`: Adds a table value to a channel counter and, when its sign bit is set, resets it and moves the sample pointer, then negates the byte at the new pointer.

### Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:add44k

- `run_startup_then_open_libraries`: Program entry that performs one setup call and then runs the library-opening and cleanup sequence, keeping the bases it obtains.
- `open_graphics_and_intuition_libraries`: Opens the graphics and intuition libraries by name through the library-open routine, keeping both bases.
- `clear_slot_record_store_stack`: Clears forty-two longwords of a per-slot record, stores the stack position and a global value into it, then calls another routine.
- `open_dos_and_mathffp_libraries`: Opens the dos and mathffp libraries by name through the library-open routine and keeps their bases.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `close_stored_libraries_and_resources`: Runs a ten-step wait, closes each stored library base that is nonzero and releases a stored resource through its routine.
- `append_string_bounded`: Appends a zero-terminated source string to the end of a destination string, copying at most about thirty-two thousand bytes and always terminating the result.
- `append_string_with_limit`: Appends bytes from a source string to the end of a destination string, stopping at the source terminator or when the count passed in runs out, and then terminates the result.
- `copy_string_to_buffer`: Copies a zero-terminated string, including its terminator, into a destination buffer and returns the destination pointer.
- `measure_string_length`: Walks a zero-terminated string to its terminator and returns the number of bytes before it.
- `copy_string_padded_to_count`: Copies bytes from a source string up to a counted limit, stopping at its terminator, then pads the rest of the destination with zero bytes.
- `release_table_slot_entry`: Checks a slot index against the table size and whether the entry is in use.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `close_stored_library_base`: Calls the library close routine on a library base kept in a global slot.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `open_library_via_stored_base`: Calls the library open routine through a library base kept in a global slot, passing its arguments on unchanged.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `forward_library_call_vector`: Calls a library function at a fixed vector offset through a stored library base and returns.

### Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:execute

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: Copies sixteen longwords from a fixed table into a caller buffer when that buffer pointer is nonzero, then calls a routine through a stored pointer.

### Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:cd

- `build_context_block_with_handler`: Allocates a block sized by the largest value found in a list and fills it with a sequence of tagged values.
- `copy_words_then_call`: Copies a fixed number of 32-bit words from a source table into a destination buffer when that buffer is non-null, then invokes a routine through a pointer.

### Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_function`: executes a single library function call using a negative offset.
