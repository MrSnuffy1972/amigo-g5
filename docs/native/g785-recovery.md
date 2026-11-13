# Game 785 (birds of prey (1991)(electronic arts)[cr razor](disk 1 of 2)(intro)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00785). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904608 top-level bytes in 30 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 18 of 18 units identical (bytes and relocations), 39 raw-word lines |
| Functions named and described | pass, 175 of 175 named (0 author symbols), 175 of 175 described (22 by rule, 49 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1652 of 1653 (99.94%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3492 of 3512 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 734823 |
| code | 12998 |
| data | 104287 |
| padding | 4884 |
| unused | 47616 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:intro | 1 | 188 | 36 |
| Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:ar | 2 | 1677 | 68 |
| Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:add44k | 3 | 475 | 32 |
| Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:execute | 2 | 117 | 2 |
| Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:cd | 2 | 282 | 5 |
| Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| logged disk load from Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 102 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 45 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 38 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 338 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 28 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 56 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 105 | 3 |

## Cases handled

- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (play-long-keys): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- birds of prey (1991)(electronic arts)[cr razor](disk 1 of 2)(intro)/Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf: 3 executed PCs whose RAM was overwritten before any snapshot match its bytes 358720-358784: recovered as a unit run at an address
- RAM snapshot an address: 88 executed PCs whose RAM was overwritten before any snapshot match its bytes 360768-361312: recovered as a unit run at an address
- RAM snapshot an address: 27 executed PCs whose RAM was overwritten before any snapshot match its bytes 360272-360480: recovered as a unit run at an address
- RAM snapshot an address: 13 executed PCs whose RAM was overwritten before any snapshot match its bytes 359920-360160: recovered as a unit run at an address
- RAM snapshot an address: 9 executed PCs whose RAM was overwritten before any snapshot match its bytes 360160-360288: recovered as a unit run at an address
- RAM snapshot an address: 6 executed PCs whose RAM was overwritten before any snapshot match its bytes 360528-360624: recovered as a unit run at an address
- 407 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 5 first-run image unit(s) in 1 version group(s); 636 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- birds of prey (1991)(electronic arts)[cr razor](disk 1 of 2)(intro)/Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:intro: Double Action 1.0 (xfd) stream found by ancient scan (35816 -> 183916 bytes) but only 5 of 26 sampled windows are in reference RAM: not used
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 immediate
- birds of prey (1991)(electronic arts)[cr razor](disk 1 of 2)(intro)/Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf: 1 logged loads cover 0 bytes of the image
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
- `return_negative_one`: Returns negative one in d0.
- `decode_rle_bitplane_rows`: Decodes a run-length coded bitmap into rows spaced forty bytes apart.
- `interpret_stream_timed_entry`: Enters a byte-coded command interpreter that first waits on a raster position and a CIA port status bit.
- `interpret_stream_copy_entry`: Starts a byte-coded command interpreter by copying thirty-two entries first, then dispatches the same opcodes for timing, counters and branches.
- `interpret_stream_direct_entry`: Loads a value from the stream and enters a byte-coded command interpreter that dispatches opcodes for timing, counters and branches.
- `interpret_stream_skip_entry`: Enters a byte-coded command interpreter at a point that reads a counter adjustment before dispatching opcodes.
- `process_command_byte`: Loads a byte from a table into d0, and if it is not zero, increments a value at an offset of a global variable before branching to another routine; otherwise, it sets d0 to zero and performs a bitwise change on a global variable.
- `execute_command_stream`: Interprets a custom bytecode stream to perform graphics operations, such as bitplane decoding, area clearing, and block copying, or to update a table of function pointers.
- `interpret_stream_plain_entry`: Enters the shared byte-coded command interpreter directly at the current stream pointer, dispatching opcodes for timing, counters and branches.
- `process_coded_command_stream`: Runs a byte-coded command stream, dispatching on each opcode to copy small tables, adjust counters, swap two global variables, and follow or set pointers.
- `interpret_stream_slot_init_entry`: Initializes one command slot from the stream value, clearing its counter and storing its pointer, then enters the shared command interpreter.
- `run_coded_command_stream_2`: Reads a byte-coded command stream and dispatches on each opcode to set timing, copy small tables, adjust counters or branch within the stream.
- `interpret_stream_clear_slot_entry`: Clears one command slot and enters the shared byte-coded command interpreter.
- `interpret_stream_fill_slot_entry`: Stores a value into a command slot and enters the shared byte-coded command interpreter.
- `interpret_stream_load_slot_entry`: Loads a saved slot pointer from the stream value and enters the shared byte-coded command interpreter.
- `interpret_stream_call_helper_entry`: Calls another routine and then enters the shared byte-coded command interpreter.
- `build_command_slot_table_entry`: Builds a table of slot addresses from a list of offsets stored in memory, then enters the shared byte-coded command interpreter.
- `interpret_stream_save_return_entry`: Saves the current stream position as a return point and enters the shared byte-coded command interpreter.
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
- `initialize_and_update_data_structures`: Calculates and stores several word-aligned offsets into a structure pointed to by a1, then performs a sequence of conditional calls to another routine using values from a structure in a2 and the newly calculated offsets.
- `execute_sequence_of_calls_2`: Performs a series of calls to a routine via a pointer in a5 using different arguments and addresses, performs conditional checks on the results, and eventually performs an indirect jump.
- `execute_sequence_of_calls`: Calls a routine via a pointer in a5 multiple times with varying arguments in d0 and a4, performs conditional logic based on the results, and finally jumps to a location in a6.

### Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_function`: executes a single library function call using a negative offset.

### logged disk load from Birds of Prey (1991)(Electronic Arts)[cr Razor](Disk 1 of 2)(Intro).adf

- `update_audio_channel_parameters`: Reads a structure via a pointer to update audio hardware registers including channel position, period, volume, and length, while also modifying DMA control and CIAA port A bits based on specific control flags.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_audio_and_dma`: Sets bit 1 of a hardware register, writes a byte to a global variable, clears several bytes in a global variable, clears four 16-bit hardware registers, sets a 16-bit hardware register to 15, and returns.
- `update_audio_and_dma_state`: Enables DMA by setting bit 15 of the DMA control register, updates the four audio channels with new location, length, and volume values from memory, and enters a loop that increments a global variable until it reaches 1024, at which point it performs a polling loop on a memory location before returning.
- `wait_for_vblank`: Polls a hardware register in a loop until its value changes, repeats this process five times, then executes a second loop that counts down from eight before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_scaling_factors`: Checks a global variable and, if it is not 0 or 2, sets two local variables to 0; then it reads a value from a structure using a pointer and an offset, calculates two scaled values based on that value, and stores them along with the original value into a destination structure.
- `clear_memory_block`: Clears a block of memory starting at a specific address for 4096 words using a loop, then calls another routine and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_values`: Writes two 16-bit values to offsets 64 and 88 of a structure in a6, increments a1 by 8, and enters a loop that decrements d0 until it reaches zero.
- `wait_for_bit_and_store`: Sets a global variable to 1 and enters a loop that checks if bit 6 of a byte at offset 2 of a structure in a6 is set; once the bit is clear, it stores a 32-bit constant at an offset of that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_indexed_values`: Checks if a value pointed to by a3 is zero; if so, it performs conditional checks on a structure in a0 and updates a value at a pointer in a2 with arithmetic that wraps around 2048; finally, it loads three 16-bit values from a table into a global variable using offsets derived from a2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_and_load_global`: Checks bit 1 of a global variable, toggles it, and if it was set, performs several arithmetic operations on a structure in a6 and loads a value from a pointer into the global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_bltcon1_and_call`: Clears the long word at the address of the BLTCON1 register, calls another routine, loads a value from the stack into a6, and calls a routine at an offset from a6.
- `copy_and_call_routine`: Copies a long word from an offset of a1 to a global variable, calls a routine at an offset from a6, and sets d0 to zero.
- `initialize_engine_state_and_tables`: Sets specific values in a structure pointed to by a6, clears a block of memory starting at a specific address, and performs a series of nested loops that manipulate data from a table using bitwise shifts and arithmetic to populate a memory region.
- `initialize_hardware_state`: Saves registers to the stack, writes 16384 to an offset of 156 in a structure pointed to by a6, calls three different routines, restores registers, and returns via an exception return.
- `decrement_table_elements`: Loads a count from a table into d7 and enters a loop that either decrements a value at a pointer in a1 or, if that value is zero, increments a value at an offset of a global variable if it does not equal 78; the loop repeats d7 times.
- `initialize_structure_from_globals`: Enters a loop that waits until a byte at offset 6 of a structure in a6 is negative one, then copies two long words from global variables to other global variables, calculates a value using a constant, stores that value and its high word into a structure in a0, and finally populates several fields of the structure in a6 with specific values.
- `update_structure_data`: Checks if a global variable is zero, then initializes a structure in a6 with several values including a pointer from a table, and enters a loop that processes a table of data to update various offsets in the structure in a6.
- `clear_d0_register`: Performs eight consecutive logical OR operations on register d0 with zero.
- `process_attribute_table`: Initializes several fields in a structure, then enters a loop that reads three words from a table, performs arithmetic and bitwise operations on the data, and stores the results into various offsets of a structure pointed to by a6.
- `initialize_table_pointers`: Loads several addresses into registers a0 through a5, moves a word from the table in a0 into d7, sets a byte in a global variable to zero, and then performs several logical OR operations on d0 with zero.
- `load_values_from_table`: Loads a word from a table in a4 using an index from a2, writes it to two global variables, calculates a second index, writes four more values from the table to global variables, and then performs several logical OR operations on d0 with zero before conditionally branching.
- `branch_if_not_equal`: Performs a conditional branch if the zero flag is not set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_global_word`: Loads an address from a global variable into a0, moves the word at that address into another global variable, and returns.
- `find_max_byte_and_offset_table`: Iterates through a sequence of bytes to find the maximum value, then calculates a pointer to a table entry based on that maximum value and an offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_and_branch`: Calls a subroutine and then performs an unconditional branch to another location.
- `update_dma_and_hardware_state`: Reads a control structure to update the DMA controller configuration and writes values to the CIAA port.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_word_to_structure`: Copies a 16-bit word from offset 16 of a structure in a4 to offset 6 of a structure in a5 and returns.
- `extract_bitfield_and_update_structure`: Reads a 16-bit value from a structure, extracts a 4-bit field to determine a branch path, and updates a 16-bit value at offset 8 of a second structure using a byte derived from a subtraction operation on a value at offset 19 of the first structure.
- `handle_input_commands`: Interprets a command byte from a4 to perform specific actions, such as modifying the CIAA_PRA register, updating a global variable, or writing a 16-bit value to offset 8 of a5.
