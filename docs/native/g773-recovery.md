# Game 773 (worms (1995)(ocean)(m3)(disk 1 of 3)[cr ott]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00773). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (946360 top-level bytes in 67 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 21 of 21 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 319 of 319 named (0 author symbols), 319 of 319 described (44 by rule, 89 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 12124 of 12126 (99.98%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 15749 of 15749 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 503259 |
| code | 58088 |
| data | 293915 |
| padding | 12250 |
| unused | 78848 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:Run | 2 | 458 | 12 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:INSTALL | 3 | 205 | 10 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:OCEAN | 4 | 205 | 7 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:RunBack | 3 | 163 | 2 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:TEAM17 | 4 | 205 | 7 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:FLOPPYWORM | 4 | 424 | 30 |
| mathtrans.library | 18 | 848 | 106 |
| diskfont.library | 15 | 1044 | 80 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:HOOKTRO | 2 | 235 | 8 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:WORMS | 3 | 160 | 15 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:HDWORM | 4 | 407 | 30 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 71 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 67 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 72 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11019 | 1 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:OCEAN#unpacked | 2 | 67 | 1 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:TEAM17#unpacked | 2 | 72 | 2 |

## Cases handled

- worms (1995)(ocean)(m3)(disk 1 of 3)[cr ott]/Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:TWSTUFF/TW6: CrunchMania Data (xfd)-packed, unpacked with ancient (41486 -> 61440 bytes)
- reference run failed (Karl 4 Nov 2026): run play-rs-x25k slid through memory (an address: 103 64 KB pages each with >= 29491 distinct PCs executed, 1687552 of its 1700034 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-x35k slid through memory (an address: 103 64 KB pages each with >= 29491 distinct PCs executed, 1687552 of its 1700034 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EAE) A7=an address (A7)=an address, slid to exec.library -an address): 586 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 71 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 54 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 60 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 11022 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 2 first-run image unit(s) in 1 version group(s); 0 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- worms (1995)(ocean)(m3)(disk 1 of 3)[cr ott]/Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:INSTALL: CrunchMania Normal (xfd) stream found by ancient scan (15240 -> 28456 bytes) but only 0 of 12 sampled windows are in reference RAM: not used
- worms (1995)(ocean)(m3)(disk 1 of 3)[cr ott]/Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:OCEAN: crunched executable decrunched by XFDMaster (CrunchMania Normal (xfd), 8992 -> 26392 bytes), 10 of 11 windows confirmed in reference RAM: recovered as a hunk executable
- worms (1995)(ocean)(m3)(disk 1 of 3)[cr ott]/Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:TEAM17: crunched executable decrunched by XFDMaster (CrunchMania Normal (xfd), 8180 -> 48836 bytes), 13 of 14 windows confirmed in reference RAM: recovered as a hunk executable
- worms (1995)(ocean)(m3)(disk 1 of 3)[cr ott]/Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:HOOKTRO: ProPack (RNC) (xfd) stream found by ancient scan (2240 -> 25112 bytes) but only 1 of 2 sampled windows are in reference RAM: not used
- worms (1995)(ocean)(m3)(disk 1 of 3)[cr ott]/Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:WORMS: CrunchMania Normal (xfd) stream found by ancient scan (137088 -> 353940 bytes) but only 0 of 171 sampled windows are in reference RAM: not used
- worms (1995)(ocean)(m3)(disk 1 of 3)[cr ott]/Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:INSTALL: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- worms (1995)(ocean)(m3)(disk 1 of 3)[cr ott]/Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:OCEAN: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- worms (1995)(ocean)(m3)(disk 1 of 3)[cr ott]/Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:TEAM17: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- worms (1995)(ocean)(m3)(disk 1 of 3)[cr ott]/Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:WORMS: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longwords_then_call`: When its destination pointer is set, copies sixteen longwords from a source block into it, then calls a routine through a stored pointer.
- `initialize_entity_state_and_animation`: Updates an entity structure by setting various offsets, including an animation frame fetched from a table, and performs a loop to copy a sequence of bytes from one memory location to another based on a calculated length.
- `update_entity_animation_frame`: Fetches a byte from a table using an index stored at offset 0 of a structure, stores it at offset 20 of the structure, and increments the frame index at offset 24.
- `update_entity_sprite_data`: Calculates a new sprite index and position for an entity by reading values from a structure, then copies a byte from a table in a0 to the entity's sprite data at a calculated offset.
- `update_entity_state_and_call_handler`: Updates several fields in a structure pointed to by a1, including setting a byte to 10 and copying long values between offsets, then calls another routine.
- `add_offset_and_branch`: It sets d1 to 1, adds a long value from an offset of a1 to d1, and then branches to another location.
- `test_long_at_offset_40`: Tests a long value at offset 40 of a structure in a1 and branches if it is non-zero, otherwise it branches to another routine.
- `modify_bitmask_at_offset_48`: Loads a long value into register d1, performs a bitwise AND with a long value at offset 48 of a structure in a1, and branches if the result is zero; otherwise it performs a bitwise OR on the value at offset 48 of the structure in a1 with a constant, then compares this new value to 133 in register d1 and branches based on the comparison result.
- `update_entity_and_sync_target`: Copies a long value from offset 64 to offset 108 of a structure, then calls another routine with parameters loaded from offset 52, offset 48, and offset 132 of a second structure.
- `reset_entity_state_and_update_pointers`: Clears several fields in a structure pointed to by a1, updates a target array at a0 using an index calculated from a1, and performs a series of data transfers between structures before calling another routine.
- `update_structure_and_decrement_counters`: Calls several routines using parameters from a structure in a2, updates a long value at offset 128 of a1, and conditionally decrements a long value at an offset in a0 based on values in a1.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:INSTALL

- `run_entry_with_exec_calls`: Program entry for the INSTALL file.
- `decrement_to_minus_one_2`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `decompress_and_expand_data_tables`: This routine clears a memory block and then performs a decompression process involving bit-shifting and table lookups to reconstruct data.
- `shift_bit_stream_2`: Shifts bits from a register and updates a value using a bitmask or a value from a table.
- `extract_bit_stream_value`: Performs a loop up to 16 times to extract a value from a bit stream using a pointer in a2 and compares it against a value from a4, returning a word from a table in a0.
- `lookup_table_masked_value_2`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `fill_two_tables_2`: The routine calls another routine to obtain a count, then fills a short table with that many values returned by the same routine, accumulating the values.
- `initialize_table_values_2`: Iteratively calculates and stores new values into a table using a pointer.
- `decompress_data_blocks`: Uses a routine in a4 to copy data from a source to a destination in a1, and subsequently processes a series of blocks by calling the same routine and adding values to a table.
- `tiny_move_rts`: Small routine of 2 instructions that copies d3 to d6, then returns.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:OCEAN

- `run_entry_with_exec_calls_2`: Program entry for the OCEAN file, with the same body as the INSTALL entry: several routine calls, a call through a stored function pointer, a cleared memory slot, an exec FreeMem call and a return of two register values.
- `decrement_to_minus_one_3`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `decode_compressed_block_stream`: Decodes a compressed block by reading codes from a bit stream, copying runs of earlier output or writing literal bytes, and building lookup tables.
- `shift_bit_stream`: Shifts bits from a register and updates a value using a bitmask or a value from a table.
- `lookup_table_masked_value`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `fill_two_tables`: The routine calls another routine to obtain a count, then fills a short table with that many values returned by the same routine, accumulating the values.
- `initialize_table_values`: Iteratively calculates and stores new values into a table using a pointer.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:RunBack

- `parse_and_run_background_command`: Parses a command line and runs it as a background task through the Amiga DOS library, redirecting its input and output to NIL. It returns an error text for an invalid parameter and reports a started background task on success.
- `measure_string_then_call_library`: Walks a null-terminated string to its end, then makes two calls through library function pointers.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:TEAM17

- `run_entry_with_exec_calls_3`: Program entry for the TEAM17 file, with the same body as the INSTALL entry: several routine calls, a call through a stored function pointer, a cleared memory slot, an exec FreeMem call and a return of two register values.
- `decrement_to_minus_one`: Decrements a 16-bit register value in a loop until it reaches minus one, then returns.
- `initialize_data_tables_and_process_buffer`: Initializes several tables by clearing memory and calling other routines to fill them with specific values.
- `shift_bit_stream_2`: Shifts bits from a register and updates a value using a bitmask or a value from a table.
- `lookup_table_masked_value_2`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `fill_two_tables_2`: The routine calls another routine to obtain a count, then fills a short table with that many values returned by the same routine, accumulating the values.
- `initialize_table_values_2`: Iteratively calculates and stores new values into a table using a pointer.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:FLOPPYWORM

- `scan_list_and_allocate_handler`: Scans a linked list for its largest value, then allocates a block sized from that maximum, fills it with a pattern of longwords, copies a sixteen-longword block into it and returns zero.
- `call_library_function_three_times`: Writes zero through a pointer and calls a library function at a fixed offset, stores the returned value, and repeats this for two more table entries before returning zero.
- `call_three_routines_return_zero`: Calls three routines in turn and returns zero.
- `store_four_results_to_globals`: Calls one routine four times and stores each returned value in its own global slot, at four fixed offsets from a base register.
- `allocate_memory_with_offset`: Calls the Amiga exec AllocMem function through a pointer.
- `call_free_mem`: Wrapper that calls the exec library function FreeMem and returns its result to the caller.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `poll_open_library_until_success`: Repeatedly calls the Amiga exec OpenLibrary function through a pointer until it returns a nonzero result, then returns.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `forward_single_call`: A wrapper that calls one routine and returns.
- `copy_byte_buffer`: Copies a byte sequence from a source to a destination, moving eight bytes at a time for large counts and the remaining bytes singly, then writes a zero terminator after the copied bytes.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `clear_globals_and_call`: Clears two global variables to zero with a call in between and returns the address of its own routine.
- `repeat_call_until_flag_set`: Calls one routine repeatedly until its result sets the zero flag, then returns.
- `allocate_linked_block_or_trap`: Calls a function through a pointer to obtain a memory block, links the block into a list and returns its data address when the block is present.
- `unlink_or_allocate_list_entry`: Walks a linked list for the entry it was given.
- `jump_to_save_restore_global_value`: Jump stub: passes control straight on to save_restore_global_value.
- `jump_to_save_restore_global_d0`: Jump stub: passes control straight on to save_restore_global_d0.
- `find_list_tail`: Follows a linked list through its pointers until it reaches a null entry, then stores the address of that final pointer in a global variable.
- `save_restore_global_value`: Saves the value of a global variable, calls two routines, then restores the saved value.
- `save_restore_global_d0`: The routine saves the value of a global variable to a local register, calls two other routines, and then restores the global variable to its original value.
- `allocate_block_and_link`: Allocates a block through the Amiga exec AllocMem function and, when the allocation succeeds, links the block into a list and returns its data address.
- `free_list_entry_via_exec`: Searches a linked list for the given entry.
- `unlink_entry_from_list`: Walks a linked list to find the entry whose successor is the one being removed, then unlinks it and calls a library function.
- `allocate_entry_and_push_list`: Allocates a block through the Amiga exec AllocMem function, pushes it onto a list by storing the previous head and clearing its second word, and returns the block.
- `free_all_list_entries`: Frees the head entry of a linked list through the Amiga exec FreeMem function, then frees each following entry in turn.
- `tiny_ori`: Small routine of 1 instruction that merges 5 into d0.
- `scan_list_and_install_handler`: Scans a linked list for its largest value, allocates a block sized from that maximum, fills it with a longword pattern, copies a sixteen-longword block to and from a buffer, and stores the address of a handler routine in the block before returning zero.
- `copy_stack_data`: Copies sixteen longwords from a source address to a destination address pointed to by a stack pointer, then calls another routine.
- `update_entity_list_and_decrement_counters`: Updates a pointer in a structure by shifting a value right by two bits, calls several subroutines to manage data, clears an entry in a table based on an index, and decrements a value in a list if it is greater than zero before performing an indirect jump.

### mathtrans.library

- `stub_unimplemented_instruction`: A single instruction that the decompiler reports as unimplemented, so the routine stops at once and no behaviour is recovered.
- `store_address_in_global`: Stores its address-register input in a library global and returns.
- `release_library_node_if_idle`: If a counter field in a library node is zero, it unlinks the node from its list, frees it through the Amiga exec FreeMem function and returns a library global value.
- `set_node_flag_bit_three`: Sets bit three of a flag byte in a library node and returns zero.
- `acquire_library_node_use`: If a flag bit in a library node is clear, it increments the node's use counter and, on the first use, makes a library vector call.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `release_library_use_counter`: Decrements a library node's use counter.
- `forward_single_call_2`: A wrapper in the mathtrans library that calls one routine and returns.
- `compute_bitwise_square_root`: Computes a bit-by-bit square root of an input value over twenty-three steps.
- `refine_bit_quotient_value`: Sets one bit in the first input, chosen by the low bits of the second, then runs a doubling and subtracting loop on the running difference while a counter counts down.
- `mask_library_result_bits`: Calls a library function through the library base and returns its result with one bit cleared and another set.
- `call_jump_table_sequence`: Makes a fixed sequence of indirect calls through entries at set offsets from a base register, alternating between two entries and making one extra call to a third entry.
- `dispatch_flagged_library_calls`: Calls a library entry, tests two flags it sets, then runs one of two short sequences of library calls.
- `select_library_call_path`: Picks among several library call sequences using whether one value is negative and whether another is zero, then returns a further call's result with some bits cleared.
- `conditionally_call_library_entries`: Calls library entries depending on whether one input is zero and whether another is negative, then returns a further call's result with some bits cleared.
- `call_library_by_sign_flags`: Chooses library calls from zero and sign tests on three inputs, then returns a further call's result with some bits cleared.
- `dispatch_library_calls_by_sign`: Chooses between library calls according to the sign of one input and whether another input is zero, then returns a masked result.
- `call_library_then_mask`: Makes one library call through the library base, then returns another call's result with some bits cleared.
- `call_library_entry_twice`: Calls the same library entry twice in a row and then returns the result of a further call with some bits cleared.
- `mask_result_after_double_call`: Calls a library entry twice, then returns the result of one more call with some bits cleared.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `compute_bit_serial_quotient`: Runs a shift-and-subtract loop that builds a result one bit at a time, with a fixed iteration limit.
- `divide_by_bit_shifts`: Repeatedly shifts a divisor-like value and subtracts it from an accumulator to build a quotient, with a fixed step limit.
- `accumulate_shifted_until_counter_expires`: Repeatedly adds a shifted value to an accumulator and subtracts another value until a counter runs out.
- `compute_trig_by_iterative_rotation`: Reduces an angle into range using fixed-point multiples of a constant, then runs a shift-and-add rotation loop over a table of constants.
- `rotate_angle_and_flag_zero`: Runs the same fixed-point angle rotation loop over a constant table, then calls a routine and calls another one only when the result is nonzero.
- `subtract_table_steps_from_angle`: Reduces a fixed-point angle by subtracting successive constants from a table, then calls a routine with the result.
- `rotate_angle_by_stack_flag`: Reduces a fixed-point angle and runs a table-driven rotation loop, then uses a stacked signed value to choose whether to call one routine or a second one before returning.
- `add_table_steps_to_angle`: Adds successive table constants to a fixed-point angle while a sign test holds, then calls a routine with the result.
- `rotate_angle_branch_on_stack`: Runs the table-driven angle rotation, then uses a stacked value to decide whether to call one routine or another before returning.
- `reduce_angle_and_rotate_table`: Reduces an angle by repeatedly adding a multiple of a constant, then runs a table-driven rotation loop.
- `rotate_table_with_stack_branch`: Runs the table-driven rotation loop on its input and then uses a stacked signed value to choose between two routine calls before returning.
- `walk_table_with_shifted_steps`: Walks a table, adding or subtracting shifted copies of each entry to an accumulator according to sign tests.
- `stub_returns_immediately`: Stub that does nothing and returns straight to its caller.
- `stub_forward_call_return`: Makes a single call to another routine and returns, with no other work.
- `call_two_routines_in_order`: Calls two routines one after the other and returns, with no other work.
- `normalize_float_mantissa`: Doubles a value until its sign bit is set, adjusting an exponent-like count as it goes.
- `normalize_mantissa_if_large`: Returns at once when a high-part input is already at its limit.
- `call_three_routines_in_order`: Calls one routine, then calls two more in sequence.
- `call_four_routines_in_order`: Calls one routine, then three more in sequence.
- `call_routines_for_nonzero_byte`: If a byte input is nonzero and incrementing it does not overflow, calls one routine, then two routines with the incremented value, then one more routine.
- `empty_library_routine`: This library routine is empty.
- `stub_no_operation`: Stub that performs no work and returns immediately.
- `return_immediately_2`: This library routine performs no operations and returns to the caller immediately.
- `stub_empty_entry`: Stub with five instructions that performs no work and ends immediately.
- `stub_empty_routine`: Stub with four instructions that performs no work and returns to its caller.
- `normalize_byte_mantissa_stack`: Returns at once for a zero input.
- `call_four_routines_sequence`: Makes four calls to routines in sequence and returns.
- `accumulate_table_by_sign`: Walks a table for a fixed count, adding each entry to an accumulator while it is negative and subtracting it otherwise.
- `subtract_table_from_accumulator`: Adds the first table entry to an accumulator, then subtracts later entries while the accumulator stays nonnegative.
- `normalize_and_scale_value`: Returns early for a zero input.
- `return_immediately_3`: This library routine performs no operations and returns immediately to the caller.
- `scale_value_with_shift_loop`: Makes several calls, then runs a shift-and-subtract loop that builds a quotient one bit at a time.
- `run_shift_loop_then_normalize`: Runs a shift-and-subtract loop over a counter and calls a routine when the counter expires.
- `call_routine_three_times`: Calls one routine three times in succession and returns.
- `normalize_signed_value_branch`: If the input's sign bit is set, makes one call and returns.
- `normalize_nonzero_value_calls`: Makes two calls, then for a nonzero input doubles a scaled copy until its sign bit is set and makes several calls on the result.
- `stub_empty_library_entry`: Stub with no body that performs no operation.
- `stub_returns_immediately_2`: The recovered body is only a return, so it does no visible work.
- `return_immediately_6`: This library routine performs an immediate return.
- `stub_returns_immediately_3`: The recovered body is only a return; the calls listed for it do not appear in the pseudo-code, so it does no visible work.
- `return_immediately_5`: This library routine is empty and performs no operations before returning to its caller.
- `return_from_library_call`: This routine is an empty function that performs no operations and immediately returns to its caller.
- `return_immediately_4`: This library routine is empty and performs no operations before returning.
- `invoke_callback_and_return`: Calls the routine whose address is held in an address register, then returns the data register's original value unchanged.
- `call_indirect_return_d0`: Calls the routine at the address passed in an address register, then returns the data register's value from entry unchanged.
- `call_routine_preserve_d0`: Makes a call to a routine and returns the data register's original value unchanged.
- `library_branch_to_offset`: As library code, it loads an address into a0 and branches to a relative offset.
- `library_branch_to_offset_indirect`: As library code, it loads an address into a0 and branches to a relative offset.
- `load_pointer_and_branch`: Loads a longword pointer from a memory location into a register and then performs a short branch.
- `load_library_offset_pointer`: As library code, it loads a pointer from a specific offset into a0 and then branches to the next instruction.
- `load_library_offset_pointer_2`: As library code, it loads a pointer from a specific offset into a0 and then branches to the next instruction.
- `load_pointer_and_branch_2`: Loads a pointer from a memory location into a register and performs a short branch to another routine.
- `load_address_and_branch`: Loads a longword from a memory location into a register and performs a short branch to another routine.
- `load_pointer_and_branch_3`: Loads a pointer from a global variable into a0 and performs a short branch to another routine.
- `load_address_and_branch_2`: Loads the address of a table into a register and performs a short branch to another routine.
- `library_dispatch_handler`: This library code returns the value contained in a register.
- `return_register_value`: This routine returns the current value held in a data register to the caller.
- `clear_memory_block`: Fills a memory area pointed to by a register with zero.
- `clear_table_entries`: Writes zero to three consecutive memory locations pointed to by a table and calls a library function for each entry.
- `return_register_value_2`: The routine returns the current value held in a data register to the caller.
- `clear_and_call_table`: Writes zero to a memory location pointed to by a register and calls a library function, repeating this process for three consecutive entries in a table.
- `store_register_via_pointer`: Calls a routine, then stores the 32-bit data register value through the address held in a second data register before returning.
- `execute_indirect_call`: An empty routine that performs a single indirect call to another routine and returns the resulting value in a register.
- `call_sequential_routines`: This library routine calls three different routines in a specific sequence.
- `call_routines_with_increment`: This library routine calls a sequence of routines based on whether a byte input is non-zero and can be incremented without overflowing.
- `return_on_zero_input`: This library routine returns immediately if the input value is zero.
- `return_on_zero_input_early`: This library routine performs an early return when provided with a zero input.
- `call_routine_thrice`: This library routine calls another routine three times in succession.
- `call_indirect_signed`: This library routine calls a routine at an address held in an address register, with conditional logic based on a sign bit.
- `execute_empty_stub`: This library routine is an empty stub that performs no operations.
- `return_immediately`: This library routine performs no work and returns immediately.
- `call_library_entry_keep_d0`: Calls a library entry through the library base and returns the data register value from entry unchanged.
- `call_library_offset_a`: This library routine calls a routine at an address calculated by adding an offset to a library base value.
- `call_library_entry_keep_pair`: Calls a library entry through the library base and returns the two data register values from entry unchanged, as one 64-bit result.
- `invoke_callback_and_return_concat_2`: This library code invokes another routine via a pointer stored in a global variable and returns the concatenation of two input values.
- `call_library_offset_b`: This library routine calls a routine at a different offset relative to a library base value.
- `invoke_library_offset_routine`: This library code calls a routine at an offset from a library base and returns the value from a data register.
- `invoke_indirect_callback`: Fetches a function pointer from a global variable with a specific offset and executes it.
- `execute_indirect_callback`: Fetches a function pointer from a global variable with a specific offset and executes it.
- `invoke_indirect_callback_2`: Fetches a function pointer from a global variable with a fixed offset and executes it.
- `invoke_callback_and_return_concat`: This library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_offset_keep_d0`: Calls a routine at an address formed by adding an offset register to a library base value, then returns the data register value from entry unchanged.
- `normalize_fixed_point_value`: Normalizes a 32-bit fixed-point value: it swaps the two halves when the value is small, then doubles it while counting down a shift count until the sign bit is set or the count reaches its limit.

### diskfont.library

- `set_field_if_handle_found`: Calls a library function through the library base.
- `check_and_set_font_handle`: This library code calls a routine via a table entry using the library base.
- `open_libraries_and_init_list`: Stores its arguments, opens two system libraries, and when both succeed initialises a list header inside a new structure and returns it.
- `close_library_and_free_memory`: Calls two system cleanup functions, one closing a library and the other freeing a memory block, then returns zero.
- `remove_unused_list_entries`: Walks a linked list and removes entries whose use count is not positive, calling a cleanup routine for each one.
- `release_or_defer_resource`: If a use count is zero and the list is empty, closes the libraries, frees the memory, unlinks the node and returns its stored value.
- `set_flag_bit_return_zero`: Sets the top bit of a flag byte in the structure and returns zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Clears the top bit of a flag byte and increments a 16-bit counter field in the same structure.
- `decrement_counter_maybe_release`: Decrements a 16-bit counter field.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d0.
- `populate_font_metadata_table`: Populates a metadata table by traversing a linked list of font definitions and reading font data from files on disk.
- `null_operation`: Performs a logical OR operation on a data register with zero.
- `enumerate_font_entries_into_buffer`: Fills a caller-supplied buffer with descriptors for font entries: when flag bit 0 is set it walks the in-memory font list, copying each name and its attributes into the buffer, and when flag bit 2 is set it scans the current directory for font files, reads their headers and appends matching entries; it returns the count of entries found and the unused buffer space.
- `collect_font_entries_into_buffer`: Walks a linked list of named font entries, copying each name into an output buffer and recording its position and attributes.
- `collect_list_entries_into_buffer`: Copies each entry of a linked list into an output buffer, recording its position and attributes.
- `scan_names_into_buffer`: Scans a text buffer for terminated names, copying each one into a list entry's output slot and recording its position.
- `copy_text_into_entry_names`: Copies characters from a source text into a name buffer, emitting an entry each time the buffer fills.
- `collect_entries_with_font_suffix`: Walks a linked list looking for names whose extension after a dot matches the font extension.
- `collect_entries_until_list_end`: Walks a text buffer, copying each name into a list entry and reading font file records when a flag is set.
- `read_entries_from_text_stream`: Copies characters from a text buffer into a name, reading more data from an open file when the buffer runs out, and emits records as entries fill.
- `read_font_records_and_close`: Reads fixed-size records from an open file into entries, collects names from the directory list, and when finished closes the file and frees its buffers.
- `stub_return_zero`: Returns zero and performs no other work.
- `compute_weighted_flag_distance`: Computes a score for two records.
- `sum_weighted_flag_differences`: Starts from a stored value in the second record and subtracts a table entry for each of seven bit positions where the two flag bytes differ.
- `subtract_weighted_flag_bits`: Walks seven bit positions, subtracting a table entry for each position where a mask bit is set in an input flag byte, and returns the running total.
- `subtract_weighted_second_flags`: Walks seven bit positions, subtracting table entries for set bits in two flag bytes in turn, and returns the running total.
- `return_constant_sentinel`: Returns a fixed large negative sentinel value without using its arguments.
- `find_best_font_entry_match`: Looks up a font by name, first trying an already loaded font entry, then scanning font file records for the highest-scoring match and linking the chosen one into the font list.
- `link_fonts_directory_best_matches`: Looks up the FONTS directory, reads its font records, and repeatedly links the highest-scoring record into the font list until none remain.
- `read_font_file_best_matches`: Reads font records from a file, repeatedly linking the highest-scoring record into the font list until none remain.
- `link_highest_scoring_font_entries`: Repeatedly finds the record with the highest score in a block, links it into the font list and clears its slot, until no positive score remains.
- `register_fonts_from_records`: Scans a block of font records for the highest-scoring entry, links it into the font list while registering it with the graphics system, and returns the last linked entry.
- `select_and_link_font_entries`: Repeatedly selects the highest-scoring record from a block and links it into the font list, releasing a loaded segment between selections.
- `close_and_release_font_selection`: Selects highest-scoring font records and links them into the font list until none remain, then closes fonts and releases locks and memory.
- `load_font_file_records`: Opens a font file named by its argument, truncates its name at the extension dot, counts the font records, allocates a block for them and copies the matching records in.
- `load_disk_font_table`: Scans a directory with examine and next-entry calls to count font entries, allocates a table for them, and copies each font's name and attribute bytes into it.
- `free_stored_font_block`: Passes a stored block pointer and its size to the memory release routine.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem_2`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `copy_null_terminated_string`: Copies bytes from a source string into a destination buffer up to and including the terminating zero byte.
- `append_at_string_end`: Moves to the terminating zero byte of a string and passes that position and a second argument to a further routine.
- `advance_and_call_string_end`: Advances a pointer to the terminating zero byte of a string and passes that pointer to another routine.
- `measure_string_length`: Counts the bytes of a zero-terminated string, excluding the terminator, and returns the count.
- `count_null_terminated_bytes`: Adds one for each byte read from a string, including the terminating zero, to an incoming count and returns the total.
- `compare_null_terminated_strings`: Compares two zero-terminated strings byte by byte, returning zero when they are equal, a negative value when the first sorts lower, and one otherwise.
- `compare_skipped_string_prefix`: Compares a string, starting one byte in, against a second string byte by byte.
- `compare_null_terminated_strings_2`: This library routine compares two null-terminated strings byte by byte.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_character_in_string`: Scans a zero-terminated string from its start for a given byte and returns a pointer to its first occurrence, or zero if the terminator comes first.
- `find_character_from_current`: Checks the current byte of a string against a given byte, advancing through the string until a match is found.
- `find_character_after_position`: Advances past the current position in a string and returns a pointer to the next occurrence of a given byte, or zero if the terminator is reached first.
- `find_last_character_in_string`: Measures a zero-terminated string and scans backward from its end for a given byte.
- `scan_backward_for_character`: Steps backward from a position toward a lower bound, returning a pointer to the first byte matching a given value, or zero if the bound is reached first.
- `scan_backward_bounded_character`: Checks the byte before a position against a lower bound, stepping backward until a byte matches a given value.
- `skip_path_directory_prefix`: Returns the position just past the first slash in a path, or past the first colon when there is no slash.
- `increment_returned_value`: Returns its incoming value increased by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_handler_and_return`: Calls another routine and returns, with no other work.
- `call_external_handler`: This library code calls another unnamed routine and returns, acting as a dispatcher for an external function pointer.
- `call_other_routine`: This library routine calls another routine and then returns.
- `call_another_routine`: This library code acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:HOOKTRO

- `unpack_compressed_hunk_data`: Finds a packed data section with a fixed four-byte signature in a linked list, copies its compressed bytes into a temporary area, and expands them by copying literal runs and back-references into the target.
- `decode_table_bit_symbol`: Scans word-pair entries until one matches the current input bits, then uses the entry's stored bit count to extract a value and returns it with one further bit set.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: Ands two 16-bit values together.
- `return_empty_routine`: The routine is empty; it contains no instructions that modify memory or registers and immediately returns.
- `build_prefix_code_tables`: Reads a count and a list of code lengths from a buffer, then assigns codes by length.
- `process_nested_data_structures`: This routine traverses a series of nested pointers and updates values in a buffer by adding calculated offsets.
- `stub_empty_routine_2`: Empty routine of one instruction that returns immediately without doing anything.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:WORMS

- `run_program_entry_sequence`: Program entry that runs startup calls, invokes a stored function pointer, releases a memory block through a library call, clears a longword before a pointer, and returns.
- `spin_countdown_delay`: Spins a 16-bit counter down from its starting value until it reaches minus one, doing nothing else.
- `unpack_and_copy_bitstream`: Reads a length and a bit-shift value from a table, then enters a loop that calls another routine to fetch bits and stores the resulting bytes into a destination buffer.
- `execute_conditional_branches`: Loads 14 into d4 and 5 into d1, calls another routine, and if the carry flag is set, it loads 14 into d1, calls another routine, adds d0 to d4, and branches to another location; otherwise it branches to another location.
- `call_routine_and_load_registers`: Calls another routine and, if the carry flag is not set, loads the value one into registers d1 and d4 before branching to another routine.
- `branch_and_load_constants`: Calls another routine and, if the carry flag is not set, loads the values 2 and 3 into registers d1 and d4 before branching to another routine.
- `call_subroutine_and_load_constants`: Calls a subroutine, branches if the carry flag is set, otherwise it loads the values 4 and 7 into registers d1 and d4 and branches to another routine.
- `validate_and_adjust_index`: Calls a subroutine to retrieve a value, adds it to twenty-three, and compares the result to twenty-two.
- `call_subroutine_and_branch`: Calls a subroutine, then branches to a different location if the carry flag is set, otherwise it loads 5 into register d1, 0 into register d2, and branches to another location.
- `copy_data_with_offset`: Loads 14 into d1 and 544 into d2, calls another routine, adds d2 to d0, calculates an address in a3 using a1 and d0, and enters a loop that moves a byte from the address in a3 to the address pointed to by a1 while decrementing a1 and d4, then branches to another location.
- `process_word_and_shift`: It decrements d7 and branches if not zero; otherwise it sets d7 to 16, moves d6 to d0, shifts d6 right by 1, swaps d6, fetches a new 16-bit value from a pointer in a2 into d6, swaps d6 again, shifts d0 right by 1, and returns.
- `tiny_lsr_rts`: Small routine of 2 instructions that shifts d6 right by 1, then returns.
- `lookup_table_masked_value_3`: It moves d6 to d0, shifts d6 left by the value in d1, computes an offset using d1 and d3, rotates d7 left by the value in d6, fetches a 16-bit value from a table using a pointer in a2 into d6, rotates d7 left by the value in d6, increments d1, and returns the result of ANDing d0 with a value from a table at an offset determined by d1.
- `execute_nested_data_copy`: It uses a loop to iterate through a list, calling a routine via a pointer in a4, copying data from a5 to a1, and performing various conditional fetches and additions to a destination table based on values read from a5.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d3 to d6, then returns.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:HDWORM

- `fill_table_and_install_handler`: Program entry for the hard-disk worm module.
- `store_library_results_twice`: Clears a flag byte, calls a library function through a stored base and saves its 16-bit result, then repeats this for a second entry.
- `call_three_routines_return_zero_2`: Calls three routines in sequence and returns zero.
- `store_four_results_to_globals_2`: Calls one routine four times and stores each returned value into a separate fixed slot addressed from a base register.
- `call_function_pointer`: Calls a stored function pointer.
- `release_memory_via_library`: Calls a memory release function through a stored library base, then returns.
- `return_zero`: This routine is empty and simply returns a zero value to the caller.
- `poll_function_pointer`: Repeatedly calls a stored function pointer until it returns a nonzero value, doing a no-op memory write on each pass.
- `close_library_via_base`: Calls a library close function through a stored library base, then returns.
- `forward_call_and_return`: Calls one routine and returns, with no other work.
- `copy_byte_buffer_2`: Copies a sequence of bytes from a source to a destination, moving them in eight-byte blocks and then the remaining bytes one at a time.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `clear_globals_and_call_2`: Clears two global words, calls a routine, and returns the address of this routine.
- `loop_call_until_flag`: Calls a routine repeatedly until the condition flag from the last operation is set.
- `allocate_and_link_block`: Allocates a memory block through a stored base function, links it into a list headed by a global pointer, stores a size in its header, and returns the address past the header.
- `release_tracked_allocation`: Searches the allocation list for a block matching the given address.
- `jump_to_save_restore_global_value_2`: Jump stub: passes control straight on to save_restore_global_value_2.
- `jump_to_preserve_global_during_list_up`: Jump stub: passes control straight on to preserve_global_during_list_updates.
- `find_list_tail_2`: Walks a linked list to its last node and stores that node's address in a global variable.
- `save_restore_global_value_2`: Saves a global value, calls two routines, and then restores the global to its saved value.
- `preserve_global_during_list_updates`: Saves a global variable into a register, calls two routines to traverse and modify a linked list, and then restores the original value to the global variable.
- `insert_linked_node`: inserts a new node into a linked list by updating pointers and using a function pointer from a global variable.
- `remove_linked_node`: searches a linked list for a specific node to remove it, then attempts to allocate a new node if a global variable is zero.
- `unlink_and_release_list_node`: Walks a linked list to find a matching node.
- `allocate_and_link_node`: Allocates a memory node through a stored base function, links it at the head of a list headed by a global pointer, and clears its second word.
- `free_linked_node_chain`: Removes the head node from a global list, then releases it and every node chained after it through the memory release function.
- `tiny_ori_2`: Small routine of 1 instruction that merges 16 into d0.
- `allocate_and_initialize_buffer`: Scans a linked list to find the maximum value, then allocates a memory block based on that value and fills it with a longword pattern.
- `copy_sixteen_longs_then_call`: If the output pointer is nonzero, copies sixteen longwords from a fixed source area into it, then calls a routine.
- `process_hardware_state_and_decrement`: Modifies a word in a2, calls several routines with various offsets from a2, clears a long value in a0 at an offset derived from a1, and conditionally decrements a long value in a0.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_block_2`: Sets a global variable to a function address and then fills a large block of memory with zeros.
- `load_data_tables`: Iterates through a string to calculate offsets and copies multiple blocks of data from various source tables into a destination buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_return_minus_one`: Returns minus one without doing any other work.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_chained_calls`: Performs a sequence of indirect function calls through a pointer stored in a global variable and copies a block of data from one memory location to another.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_libraries_and_init_data`: Opens a sequence of system libraries through their function vectors, stopping early if any open returns nothing, then copies a block of data one longword at a time into a fixed buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_resource_handles_and_copy_data`: Attempts to open three library resources, and if successful, copies a block of data of size 6144 bytes from one memory location to another.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_custom_interrupt_vector`: Writes a specific address to the custom interrupt vector location and restores the previously saved registers before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `data_table_placeholder`: Contains only instructions that perform an or operation with zero on data registers, and does not call any routines or contain any loops.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:OCEAN#unpacked

- `open_libraries_and_init_data_2`: Opens three system libraries and, if successful, initializes a bitmap, opens a screen, copies a block of data of 3216 bytes into a buffer, loads a color palette, moves and brings the screen to the front, delays for 250 milliseconds, and then closes the screen and the libraries.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT].adf:TEAM17#unpacked

- `initialize_graphics_and_wait_for`: Opens three libraries, initializes a bitmap, opens a screen, and copies a block of data to a new memory location.
- `wait_for_disk_and_cleanup_libraries`: Loops while delaying and checking a bit in the CIAA peripheral register until a specific four-byte signature is found in a global variable, then closes a screen and three libraries before returning.
