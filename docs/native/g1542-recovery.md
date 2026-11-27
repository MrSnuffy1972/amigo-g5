# Game 1542 (golden axe (1990)(virgin)[cr angels - genesis - defjam][t +2 humane - vanity]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01542). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (922572 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 178 of 178 named (0 author symbols), 178 of 178 described (56 by rule, 38 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1528 of 1528 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4716 of 4716 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10172 |
| code | 15726 |
| data | 704802 |
| padding | 2432 |
| unused | 189440 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| EndCLI | 2 | 117 | 2 |
| LoadWb | 11 | 261 | 15 |
| FakeMem | 1 | 136 | 6 |
| Golden Axe (1990)(Virgin)[cr Angels - Genesis - Defjam][t +2 Humane - Vanity].adf:GOLDEN_AXE | 1 | 392 | 18 |
| Golden Axe (1990)(Virgin)[cr Angels - Genesis - Defjam][t +2 Humane - Vanity].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1747 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 583 | 25 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| icon.library#unpacked | 15 | 1460 | 109 |

## Cases handled

- golden axe (1990)(virgin)[cr angels - genesis - defjam][t +2 humane - vanity]/Golden Axe (1990)(Virgin)[cr Angels - Genesis - Defjam][t +2 Humane - Vanity].adf:libs/icon.library: high-entropy data file (7.69 bits/byte), classified as packed data
- reference run failed (Karl 4 Nov 2026): run play-rs-x25k slid through memory (an address: 191 64 KB pages each with >= 29491 distinct PCs executed, 6082286 of its 6082573 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-x35k slid through memory (an address: 191 64 KB pages each with >= 29491 distinct PCs executed, 6082286 of its 6082573 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- 866 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 865 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 378 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- libs/icon.library: code lead from fmtread (5996 bytes, hunk exe), 0 of 3 windows confirmed in reference RAM (hunk exe, relocatable: no RAM confirmation needed): recovered
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
- `start_workbench_tool_with_error_report`: Saves d2-d5, checks the argument count, optionally compares a string, loads a resident tag, starts a tool, and on failure prints error messages with a delay before restoring registers and returning.
- `compare_null_terminated_strings`: Compares two null-terminated strings character by character and returns zero when they are equal, or a positive or negative value depending on which one is larger.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `pass_params_to_subroutine`: Calls another routine with a global value, the given parameter and the address of a stack slot.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
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
- `frag_bset_bra`: Short code fragment that sets bit d4 of d4, then jumps to another point in the code.
- `frag_move_lsr_btst_bne_rts`: Short code fragment that copies d0 to the memory at an offset from a5, then shifts d0 right by 8, then copies d0 to the memory at an offset from a5, then tests bit 0 of the memory at an offset from a5, then branches if the last result was not zero, then returns.
- `frag_bset_bra_2`: Short code fragment that sets bit d4 of d4, then jumps to another point in the code.
- `frag_move_lsr_btst_bne_rts_2`: Short code fragment that copies d0 to the memory at an offset from a5, then shifts d0 right by 8, then copies d0 to the memory at an offset from a5, then tests bit 0 of the memory at an offset from a5, then branches if the last result was not zero, then returns.
- `halt_baddata`: This routine contains no valid instructions, only data that Ghira misinterprets as code, so it cannot be described as performing any operation.

### Golden Axe (1990)(Virgin)[cr Angels - Genesis - Defjam][t +2 Humane - Vanity].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_audio_dma_state`: Opens a library through exec, clears a 17,920-byte region, saves and restores the DMA and interrupt registers around a wait loop, and calls routines that zero the audio channels.
- `clear_interrupt_request_bits`: Clears selected interrupt request flags by writing back a masked copy of the interrupt request register.
- `run_next_nonempty_table_handler`: Steps a stored index backwards through a table in eight-byte steps until it finds a non-empty entry, calls that entry as a routine, and advances the index by one entry when the result is non-zero.
- `adjust_word_nibbles_toward_target_2`: Walks two arrays of 16-bit words together and increments each word of the first array in steps of 1, 16 or 256 until each nibble matches the second array.
- `decrement_word_nibbles_toward_target`: Walks two arrays of 16-bit words together and decrements each word of the first array in steps of 1, 16 or 256 until each nibble matches the second array.
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

### icon.library#unpacked

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `initialize_icon_structure`: Checks a word value at offset 32 of the structure in a6 and branches to another location if it's non-zero.
- `frag_bset_moveq_rts`: Short code fragment that sets bit 3 of the memory at an offset from a6, then loads 0 into d0, then returns.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `setup_icon_data`: Copies two pointers to global variables after calling a library function with parameters derived from registers.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `frag_move_jsr_addq_rts`: Short code fragment that copies a0 to the stack, then copies a6 to the stack, then calls another routine, then adds 8 to a7, then returns.
- `frag_movem_move_jsr_lea_rts`: Short code fragment that saves registers on the stack, then copies a6 to the stack, then calls another routine, then loads the address of the memory at an offset from a7 into a7, then returns.
- `frag_movem_move_jsr_lea_rts_2`: Short code fragment that saves registers on the stack, then copies a6 to the stack, then calls another routine, then loads the address of the memory at an offset from a7 into a7, then returns.
- `icon_lib_save_a0_a1_call_helper`: Pushes registers a0, a1 and a6 onto the stack, calls a library helper, then removes the saved registers and returns.
- `frag_move_jsr_addq_rts_2`: Short code fragment that copies a0 to the stack, then copies a6 to the stack, then calls another routine, then adds 8 to a7, then returns.
- `frag_move_jsr_addq_rts_3`: Short code fragment that copies a0 to the stack, then copies a6 to the stack, then calls another routine, then adds 8 to a7, then returns.
- `frag_move_jsr_addq_rts_4`: Short code fragment that copies a6 to the stack, then calls another routine, then adds 4 to a7, then returns.
- `frag_movem_move_jsr_lea_rts_3`: Short code fragment that saves registers on the stack, then copies a6 to the stack, then calls another routine, then loads the address of the memory at an offset from a7 into a7, then returns.
- `icon_library_call_782_with_a0_a6`: Pushes a0 and a6 onto the stack, calls the library routine at offset 782, then discards both saved registers and returns.
- `icon_lib_push_a0_a6_call_second_helper`: Pushes a0 and a6 onto the stack, calls a different library helper, adds eight to the stack pointer to drop them and returns.
- `frag_movem_jsr_addq_rts`: Short code fragment that saves registers on the stack, then calls another routine, then adds 8 to a7, then returns.
- `frag_movem_jsr_addq_rts_2`: Short code fragment that saves registers on the stack, then calls another routine, then adds 8 to a7, then returns.
- `frag_movem_jsr_addq_rts_3`: Short code fragment that saves registers on the stack, then calls another routine, then adds 8 to a7, then returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `initialize_icon_structure_3`: Allocates memory for an icon structure and initializes its properties, then performs conditional operations based on the allocation result.
- `copy_icon_attributes`: Copies 44 bytes from one structure to another, then copies several long values from specific offsets of a source structure to corresponding offsets in a destination structure.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `copy_icon_header_and_dispatch`: Copies 44 bytes from offset 96 of the icon structure into a local buffer, fills header fields and copies several longwords, then calls the dispatch routine with the buffer and two arguments.
- `validate_icon_setup_2`: Validates icon parameters and initializes icon structure fields based on validation results, performing conditional operations and setting up offsets and flags.
- `validate_icon_parameters_2`: Validates icon setup parameters by checking structure offsets and bit flags, then performs conditional operations based on the validation results.
- `process_icon_structure`: Processes icon data by calling parameter setup routines for different structure fields, validates results, updates structure pointers, and handles nested list processing with recursive validation calls.
- `process_icon_list`: Iterates through a list of pointers, calling functions and updating a local stack variable.
- `frag_moveq_bra`: Short code fragment that loads 0 into d6, then jumps to another point in the code.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `validate_icon_data`: Validates icon dimensions and structure, updating data fields based on validation results and returning a status code.
- `validate_icon_structure_2`: Validates an icon structure by checking its flags and linked elements, performing recursive validation on nested components and updating a result status based on validation outcomes.
- `validate_icon_chain`: Validates icon structures recursively by processing nested elements, calling validation functions on different parts of the structure, and updating counters while iterating through lists.
- `iterate_icon_list`: Iterates through an icon list, incrementing counters and calling processing functions until end of list is reached, then returns the final processed value.
- `process_icon_list_recursively`: Iterates through a list of pointers, calling functions and updating a local stack variable.
- `process_icon_with_offset`: Calls library functions to process icon data with an offset, continuing the loop while the result of the first call is non-zero.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `call_icon_cleanup_with_offset`: Saves d2, loads the second stack argument into d2 and the third into a0, then calls the routine at offset 140 of that structure with d2 pushed, restores the stack and d2, and returns.
- `initialize_icon_structure_2`: Allocates memory for an icon structure and initializes its properties, then performs conditional operations based on the allocation result and returns a pointer to the structure or zero.
- `manage_icon_cleanup`: Handles icon reference counting by decrementing a counter and allocating memory when the counter reaches zero, then stores provided parameters in allocated structure fields before returning.
- `function_epilogue_4`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `manage_icon_reference`: Decrements a reference counter and allocates memory when counter reaches zero, storing provided parameters in allocated structure fields before returning success status.
- `decrement_counter_store_pair`: Decrements a 16-bit counter through the pointer in a2, then stores the 32-bit value in d2 and the 32-bit stack value into two consecutive 8-byte slots of a table whose base is read from offset 5 of the structure in a2.
- `process_icon_data_2`: Processes icon data by allocating memory, setting up icon properties, and calling multiple library functions to validate and configure icon parameters.
- `frag_move_bra`: Short code fragment that copies d6 to d0, then jumps to another point in the code.
- `call_hook_and_return_zero`: Pushes a small constant and d6 onto the stack, calls a library hook through a jump table, then returns zero in d0 after restoring d2-d6 and a2.
- `icon_call_with_offset_4e`: Saves d2-d4, loads two arguments from the stack, adds 78 to the second argument, pushes both and calls a library routine, then restores registers and returns.
- `parse_key_value_pairs`: Parses key-value pairs from a string list by computing checksums, comparing values, and returning pointer to value portion when matching key is found.
- `function_epilogue_5`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `compute_checksums_and_compare`: Computes checksums for two strings and compares the results, returning a boolean value based on whether the comparison succeeds.
- `validate_string_checksum`: Validates string checksums by computing them for two different strings and comparing results, returning one if they match or zero if they don't.
- `frag_tst_beq_move_addq_bra`: Short code fragment that tests d4, then branches if the last result was zero, then copies d4 to d0, then adds 1 to d0, then copies d0 to d2, then jumps to another point in the code.
- `frag_moveq_bra_2`: Short code fragment that loads 0 into d2, then jumps to another point in the code.
- `function_epilogue_6`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `parse_and_copy_string_data`: Copies string data from one location to another while processing numeric values, handles end-of-string conditions, and performs byte-by-byte comparisons during parsing.
- `parse_and_compare_strings`: Parses numeric strings by converting ASCII characters to digits, accumulates values into a register, handles end-of-string conditions, and compares bytes from memory locations until mismatch or end is found.
- `frag_moveq_bra_3`: Short code fragment that loads 0 into d5, then jumps to another point in the code.
- `parse_numeric_string`: Parses a numeric string by converting ASCII characters to digits, accumulating values into a register, and handling end-of-string conditions during conversion.
- `convert_ascii_to_digit`: Compares a value from the stack with 48 and branches if it's greater, compares it with 57 and branches if it's less, then subtracts 48 from the original value and returns the result.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `run_two_step_setup`: In the Icon.Library shared library, runs a first routine on the last two arguments and, when it returns a value, runs a second routine with that value.
- `allocate_and_configure_icon`: Allocates memory using a library function and sets up icon properties if allocation succeeds.
- `function_epilogue_7`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `validate_icon_parameters`: Calls a library function with three parameters, compares its result with a fourth parameter, and returns zero if they don't match or the result of the library call otherwise.
- `function_epilogue_8`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `validate_icon_dimensions`: Calls a library function with three parameters, compares its result with a fourth parameter, and returns zero if they don't match or the result of the library call otherwise.
- `function_epilogue_9`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `divide_with_remainder`: Performs unsigned division of a 32-bit value by a 16-bit divisor, storing both quotient and remainder in registers, with special handling for divisors larger than 65535.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `frag_move_bsr_rts`: Short code fragment that copies d2 to the stack, then copies d1 to d2, then copies d0 to d1, then calls another routine, then copies the top of the stack to d2, then returns.
- `frag_move_bsr_rts_2`: Short code fragment that copies d2 to the stack, then copies d1 to d2, then copies d0 to d1, then calls another routine, then copies d1 to d0, then copies the top of the stack to d2, then returns.
- `negate_result_on_flag`: Calls a helper for a value and negates that value when the flag the helper leaves in an address register is nonzero.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `frag_movea_move_beq_cmp_bne_subq_rts`: Short code fragment that copies the memory at an offset from a7 to a0, then copies the memory at an offset from a7 to d0, then copies the memory at a0 (advancing a0) to d1, then branches if the last result was zero, then compares d1 with d0, then branches if the last result was not zero, then subtracts 1 from a0, then copies a0 to d0, then returns.
- `frag_moveq_bra_4`: Short code fragment that loads 0 into d0, then jumps to another point in the code.
- `frag_movem_move_bne_rts`: Short code fragment that copies several registers to or from memory, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then branches if the last result was not zero, then returns.
- `frag_movem_tst_bne_subq_bra`: Short code fragment that copies several registers to or from memory, then tests the memory at a0 (advancing a0), then branches if the last result was not zero, then subtracts 1 from a0, then jumps to another point in the code.
- `compare_bytes_loop`: Compares bytes from two memory locations in a loop until a mismatch is found or end of string is reached, returning zero when strings match completely.
- `frag_moveq_bgt_rts`: Short code fragment that loads 1 into d0, then branches if the comparison was greater, then loads 255 into d0, then returns.
- `frag_movem_move_tst_beq_subq_bgt_bra`: Short code fragment that copies several registers to or from memory, then copies the memory at an offset from a7 to d0, then tests the memory at a0 (advancing a0), then branches if the last result was zero, then subtracts 1 from d0, then branches if the comparison was greater, then jumps to another point in the code.
- `frag_subq_bsr_rts`: Short code fragment that subtracts 1 from a0, then calls another routine, then returns.
- `frag_subq_blt_move_bne_ble_clr_bra`: Short code fragment that subtracts 1 from d0, then branches if the comparison was less, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then branches if the last result was not zero, then subtracts 1 from d0, then branches if the comparison was less or equal, then clears the memory at a0 (advancing a0), then jumps to another point in the code.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `frag_movea_move_addq_clr_rts`: Short code fragment that copies the memory at an offset from a7 to a0, then copies a0 to the memory at a0, then adds 4 to the memory at a0, then clears the memory at an offset from a0, then copies a0 to the memory at an offset from a0, then returns.
- `draw_icon_with_offsets`: Saves d2-d5/a2, checks the icon pointer, calls the library twice with the icon and computed x/y offsets, and returns one on success or zero on failure.
- `validate_icon_structure`: Validates icon parameters recursively by calling validation functions on nested structures, returning zero if any validation fails or one if all pass.
- `frag_moveq_bra_5`: Short code fragment that loads 1 into d0, then jumps to another point in the code.
- `function_epilogue_10`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `validate_icon_setup`: Validates icon parameters by performing a series of checks and calculations involving bit shifts and arithmetic operations, storing intermediate results in structure fields before returning a validated pointer or an error code.
- `function_epilogue_11`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `process_icon_data`: Processes icon data by iterating through parameters, calling routines to compute values, and storing results in structure fields at offsets 8 and 12.
- `process_icon_parameters`: Processes icon parameters by validating them through multiple function calls, updating structure fields with computed values, and returning error codes based on validation results.
- `frag_move_bra_2`: Short code fragment that copies a3 to d0, then jumps to another point in the code.
- `function_epilogue_12`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
