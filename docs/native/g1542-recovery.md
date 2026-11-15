# Game 1542 (golden axe (1990)(virgin)[cr angels - genesis - defjam][t +2 humane - vanity]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01542). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916576 top-level bytes in 14 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 65 of 65 named (0 author symbols), 65 of 65 described (11 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1528 of 1528 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3240 of 3240 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10172 |
| code | 11134 |
| data | 703426 |
| padding | 2404 |
| unused | 189440 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| EndCLI | 2 | 117 | 2 |
| LoadWb | 11 | 261 | 15 |
| FakeMem | 1 | 136 | 6 |
| Golden Axe (1990)(Virgin)[cr Angels - Genesis - Defjam][t +2 Humane - Vanity].adf:GOLDEN_AXE | 1 | 376 | 14 |
| Golden Axe (1990)(Virgin)[cr Angels - Genesis - Defjam][t +2 Humane - Vanity].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1747 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 583 | 25 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- golden axe (1990)(virgin)[cr angels - genesis - defjam][t +2 humane - vanity]/Golden Axe (1990)(Virgin)[cr Angels - Genesis - Defjam][t +2 Humane - Vanity].adf:libs/icon.library: high-entropy data file (7.69 bits/byte), classified as packed data
- reference run failed (Karl 4 Nov 2026): run play-rs-x25k slid through memory (an address: 191 64 KB pages each with >= 29491 distinct PCs executed, 6082286 of its 6082573 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-x35k slid through memory (an address: 191 64 KB pages each with >= 29491 distinct PCs executed, 6082286 of its 6082573 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- 866 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 865 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 378 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (34 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: golden axe (1990)(virgin)[cr angels - genesis - defjam][t +2 humane - vanity]/Golden Axe (1990)(Virgin)[cr Angels - Genesis - Defjam][t +2 Humane - Vanity].adf:GOLDEN_AXE +12FE

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### EndCLI

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If the destination pointer on the stack is non-null, copies sixteen longwords from a fixed offset in a base register into it, then calls a routine through a function pointer.

### LoadWb

- `split_command_line_into_argv`: Finds the current task's command text, copies it, splits the arguments on spaces into a table of pointers and passes the count and table to a further routine.
- `pass_parameter_to_library_call`: Stores its argument in a stack slot and calls a library function through a pointer; when a flag is set it then forbids multitasking, calls two more library functions and returns the stored result.
- `invoke_two_function_pointers`: Calls two function pointers held at fixed negative offsets from a base register, one after the other, then returns.
- `return_library_result_or_fallback`: Returns the result of a library call when it is non-zero; otherwise it runs a fallback sequence through the library base using a fixed value and a stored result, then returns.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `start_resident_workbench_task`: Checks for a debug argument, looks up the workbench task as a resident tag and starts it as a new process, reporting 'cannot find' or 'cannot start' errors when those steps fail.
- `compare_null_terminated_strings`: Compares two null-terminated strings character by character and returns zero when they are equal, or a positive or negative value depending on which one is larger.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `pass_params_to_subroutine`: Calls another routine with a global value, the given parameter and the address of a stack slot.
- `measure_buffer_text_length`: Calls a library function through a global pointer, measures the text length of a 140-byte stack buffer up to its null terminator, and passes the buffer and length to another routine when the length is non-zero.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### FakeMem

- `patch_memory_list_and_copy`: Walks a linked list of memory entries and adds a size difference to each flagged pointer, then when a value is in range copies about 300 longwords to a fixed address and calls a library function.
- `reset_then_jump_through_pointer`: Selects a function pointer from a global, calls a reset subroutine, then calls the pointer through an unrecovered jump-table entry.
- `relink_memory_list_nodes`: Clears a field, then when the global flag is set writes a named node header and registers it, and finally detaches and re-attaches a node through library list calls.
- `walk_list_until_flagged_node`: Follows a chain of linked nodes from a fixed field, advancing until a node's flag byte has bit one set, then returns.
- `store_complement_checksum_and_jump`: Adds up 23 consecutive 16-bit words from a fixed offset, stores the one's complement of the total in a status field, then makes an indirect call whose target could not be recovered.
- `store_complemented_word_checksum`: Adds up 23 consecutive 16-bit words from a fixed offset in the working block and stores the one's complement of the sum at another fixed offset.

### Golden Axe (1990)(Virgin)[cr Angels - Genesis - Defjam][t +2 Humane - Vanity].adf:GOLDEN_AXE

- `initialize_system_and_display`: Program entry point: opens the graphics and intuition libraries through exec, takes the blitter, loads a display view, waits for the frame timing, then calls helper routines and ends with an indirect call through a stored pointer.
- `close_file_and_reply_message`: When a stored flag is non-zero, closes a file handle through the DOS library, disables multitasking and replies to a pending message.
- `invoke_graphics_library_function`: Makes a single indirect call into the graphics library through the stored library base and returns.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `write_null_terminated_string`: Returns at once when the output flag is zero; otherwise it measures a zero-terminated string and passes it to the DOS write call on the stored output handle.
- `flush_cache_and_enter_supervisor`: Calls the exec cache-control function when a stored word exceeds a threshold, then always calls the exec supervisor function through the library base.
- `copy_vectors_with_interrupts_disabled`: Turns off all interrupts, copies a 4 KB block to the low memory area holding the processor vectors, saves a register snapshot, runs a trap and helper routines, then restores the saved DMA and interrupt enable settings.
- `setup_library_and_wait`: Performs the same trap, copy, patch and library-open sequence as its neighbour, clears the region, then loops on a flag and calls helper routines until the loop condition clears.
- `setup_dma_and_init`: Disables interrupts through the custom interrupt-enable register, runs the shared setup sequence, switches DMA channels on and off through the custom DMA control register, and references the text IMP!.
- `patch_jump_and_open_library`: Writes either a short branch or a jump instruction into a code slot depending on two flag bits, opens and closes a library through exec, clears a region, then calls helper routines.
- `update_state_from_key_code`: Reads a byte at a fixed table offset and, depending on which of several code values it matches, updates a state byte and a jump target, then returns the bitwise complement of the code.
- `copy_indexed_table_block`: Looks up a start pointer in a table indexed by the low word of its argument, copies bytes from that source up to a table-defined end address into the destination buffer, then calls one more routine.
- `decrunch_imp_packed_data`: Unpacks a block that begins with the four-byte IMP! signature: it reads the packed and unpacked sizes from the header, then decodes bit-packed literals and back-references from the end of the data towards the start.
- `halt_baddata`: This routine contains no valid instructions, only data that Ghira misinterprets as code, so it cannot be described as performing any operation.

### Golden Axe (1990)(Virgin)[cr Angels - Genesis - Defjam][t +2 Humane - Vanity].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_audio_dma_state`: Opens a library through exec, clears a 17,920-byte region, saves and restores the DMA and interrupt registers around a wait loop, and calls routines that zero the audio channels.
- `clear_interrupt_request_bits`: Clears selected interrupt request flags by writing back a masked copy of the interrupt request register.
- `scan_handler_table_and_call`: Scans backwards through a table of routine pointers for a non-empty entry, calls it, and advances the table index when the call returns non-zero.
- `step_digit_fields_up`: For each word of a block, adds one unit to the units, tens or hundreds digit field of the first buffer wherever it differs from the matching field in the reference buffer, moving the value upward one step per call.
- `step_digit_fields_down`: For each word of a block, subtracts one unit from the units, tens or hundreds digit field of the first buffer wherever it differs from the matching field in the reference buffer, moving the value downward one step per call.
- `initialize_display_sequence`: Sets up display-related variables and hardware registers when the sequence state is not 2, then advances the frame counter and refreshes the display.
- `clamp_and_clear_joystick`: It reads the joystick data register, clamps a stored value between zero and a maximum, and then clears a block of memory before writing a single value at a position derived from that clamped value.
- `render_text_glyphs_to_bitmap_2`: Renders a zero-terminated string into the bitmap once per completion flag, copying eight glyph rows per character from the font table and wrapping to the next text row after eighty-one characters, then sets the completion flag.
- `render_text_glyphs_to_bitmap_3`: Loops over a zero-terminated string and copies eight glyph rows per character from the font table into the bitmap, wrapping to the next text row after eighty-one characters, and sets the completion flag at the terminator.
- `advance_frame_counter_and_refresh`: Increments a frame counter; every fourth call it copies a stored value into two slots of a short table, bumps a step counter and resets the frame counter.
- `refresh_two_table_slots`: Calls a helper, copies a stored value into two slots of a short table, and increments a step counter.
- `manage_sequence_state`: Manages the game sequence state by checking various conditions and updating counters, with different branches based on the current state value and input conditions.
- `advance_fire_button_state`: While the fire button is idle, advances a step counter; past one threshold it calls a handler, and past a second threshold it resets the state counters and restarts the sequence.
- `advance_step_counter_and_reset`: Advances a step counter; past one threshold it calls a handler, and past a second threshold it resets the state counters and restarts the sequence.
- `advance_sequence_until_reset`: Loops advancing a sequence counter, calling helper routines at thresholds and resetting several counters and the sequence state once the counter passes its limit.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `render_indexed_text_line`: When the display flag is clear, selects a string by stepping a source pointer a counter-determined number of times, renders its glyphs into the bitmap at a fixed destination and sets the rendered flag.
- `render_glyphs_until_terminator`: Copies glyph rows for each character of a zero-terminated string into the bitmap, wrapping to the next text row after eighty-one characters, and sets a completion flag when it reaches the terminator.
- `set_render_complete_flag`: Sets the text-render completion flag and returns.
- `advance_frame_counter_and_step`: Increments a frame counter; every fourth call it calls a helper, bumps a step counter and resets the frame counter.
- `call_routine_then_bump_counter`: Calls one other routine, then increments a counter stored in the program's data area.
- `open_cia_resources_and_graphics`: Opens the CIA chip resources and the graphics library through exec, reads the CIA-A and CIA-B port registers, and closes the graphics library again.
- `invoke_library_vector_twelve`: Makes a single call through a library base at a fixed negative offset and returns with no further recovered behaviour.
- `build_block_pointer_table`: Scans a byte table for a value that exceeds the previous entry, then fills a 30-entry pointer table with block addresses spaced by a computed stride, zeroing each block and setting the interrupt and DMA flags on completion.
- `clear_audio_dma_channels`: The routine writes zero to four audio volume registers and to a global variable, then writes a non-zero value to the same global variable and writes to DMACON and a custom hardware register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_and_jump_indirect`: Issues the processor reset instruction, then makes an indirect call whose target could not be recovered.
