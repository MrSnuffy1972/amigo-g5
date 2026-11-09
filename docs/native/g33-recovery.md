# Game 33 (BeachVolley v1.0 1249): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00033). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2063849 top-level bytes in 65 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 110 raw-word lines |
| Functions named and described | pass, 380 of 380 named (0 author symbols), 380 of 380 described (52 by rule, 260 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 7151 of 7151 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 109750 bytes, 5 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14030 of 14494 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 560247 |
| code | 76612 |
| data | 1426937 |
| padding | 53 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 473 | 51 |
| volley | 2 | 30 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1141 | 47 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8408 | 215 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2130 | 54 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |
| OSEmu.400 (raw code file, load address unknown) | 1 | 0 | 0 |
| animation.code (raw code file, load address unknown) | 1 | 149 | 1 |
| final.code (raw code file, load address unknown) | 1 | 238 | 1 |
| intro.code (raw code file, load address unknown) | 1 | 342 | 1 |
| score.code (raw code file, load address unknown) | 1 | 191 | 1 |
| volley.code (raw code file, load address unknown) | 1 | 1374 | 1 |

## Cases handled

- BeachVolley_v1.0_1249/BeachVolley_v1.0_1249.hdf: hard-disk image (AmigaDOS OFS, 2240 blocks): its filesystem parses and all 58 files in it are byte-identical to the extracted tree BeachVolley_v1.0_1249_hdf, which is recovered file by file; no block outside the filesystem holds data
- BeachVolley_v1.0_1249/BeachVolley_v1.0_1249_hdf/BeachVolley/data/dnv0: high-entropy data file (7.74 bits/byte), classified as packed data
- BeachVolley_v1.0_1249/BeachVolley_v1.0_1249_hdf/BeachVolley/data/dnv1: high-entropy data file (7.82 bits/byte), classified as packed data
- BeachVolley_v1.0_1249/BeachVolley_v1.0_1249_hdf/BeachVolley/data/dnv2: high-entropy data file (7.77 bits/byte), classified as packed data
- BeachVolley_v1.0_1249/BeachVolley_v1.0_1249_hdf/BeachVolley/data/dnv3: high-entropy data file (7.75 bits/byte), classified as packed data
- BeachVolley_v1.0_1249/BeachVolley_v1.0_1249_hdf/BeachVolley/data/dnv4: high-entropy data file (7.75 bits/byte), classified as packed data
- BeachVolley_v1.0_1249/BeachVolley_v1.0_1249_hdf/BeachVolley/data/dnv5: high-entropy data file (7.75 bits/byte), classified as packed data
- BeachVolley_v1.0_1249/BeachVolley_v1.0_1249_hdf/BeachVolley/data/dnv6: high-entropy data file (7.79 bits/byte), classified as packed data
- BeachVolley_v1.0_1249/BeachVolley_v1.0_1249_hdf/BeachVolley/data/dnv7: high-entropy data file (7.84 bits/byte), classified as packed data
- BeachVolley_v1.0_1249/BeachVolley_v1.0_1249_hdf/BeachVolley/data/introp2.cru: high-entropy data file (7.75 bits/byte), classified as packed data
- BeachVolley_v1.0_1249/BeachVolley_v1.0_1249_hdf/BeachVolley/data/terrain1.cru: high-entropy data file (7.62 bits/byte), classified as packed data
- BeachVolley_v1.0_1249/BeachVolley_v1.0_1249_hdf/BeachVolley/data/terrain5.cru: high-entropy data file (7.77 bits/byte), classified as packed data
- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 938 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m3000): 5046 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m3000-unpk04): 832 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m3000-unpk02): 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- BeachVolley_v1.0_1249/BeachVolley_v1.0_1249_hdf/BeachVolley/data/OSEmu.400: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BeachVolley_v1.0_1249/BeachVolley_v1.0_1249_hdf/BeachVolley/data/animation.code: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BeachVolley_v1.0_1249/BeachVolley_v1.0_1249_hdf/BeachVolley/data/final.code: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BeachVolley_v1.0_1249/BeachVolley_v1.0_1249_hdf/BeachVolley/data/intro.code: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BeachVolley_v1.0_1249/BeachVolley_v1.0_1249_hdf/BeachVolley/data/score.code: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BeachVolley_v1.0_1249/BeachVolley_v1.0_1249_hdf/BeachVolley/data/volley.code: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 2 never-loaded region(s), 39290 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- intro.code (raw code file, load address unknown)

## Copy protection found

- `restore_saved_custom_registers` (runtime segment an address): Writes five saved values back to a global variable, clearing one bit of the first and setting the top bit in four of them. Kind: checksum trap / register-based sabotage. Pass path: the code continues normally if the register/register-state-related values match the expected state, otherwise it triggers a crash or branch (as seen in sub_0000 or sub_1616).
- `configure_interrupt_masks` (runtime segment an address (play-rs-m3000)): Sets several hardware registers to maximum values, stores four values from a table into two global variables, and then overwrites the two global variables with the same values from the table. Kind: disk hardware/interrupt configuration check. Pass path: the branch following the call where the code continues normal execution (e.g., an address or the instruction following the move/tst).

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `execute_chained_calls`: Executes a sequence of calls through an indirect pointer, potentially calling up to four other routines, and stores a 16-bit register value into a structure field.
- `patch_jump_slots_and_call`: Writes no-operation instruction pairs and absolute jump instructions into fixed code locations, with each jump pointing to an unnamed routine, then calls four unnamed routines.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 60 into d0, then merges 0 into d0.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 200 into d0, then merges 0 into d0.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 70 into d0, then merges 224 into d0, then merges 0 into d0.
- `patch_jump_table_instructions`: Overwrites several locations in a global table or structure with enough instructions to form a trap or illegal instruction, then replaces one entry with a jump to itself.
- `patch_jump_table_slots`: Writes a series of trap instructions and a routine pointer into a table located at an offset from a global pointer.
- `patch_code_slots_then_call`: Writes no-operation instruction pairs and absolute jump instructions into six fixed code locations, then calls four unnamed routines.
- `tiny_ori_ori_3`: Small routine of 2 instructions that merges 204 into d0, then merges 0 into d0.
- `tiny_ori_ori_4`: Small routine of 2 instructions that merges 166 into d0, then merges 0 into d0.
- `tiny_ori_ori_5`: Small routine of 2 instructions that merges 104 into d0, then merges 0 into d0.
- `tiny_ori_ori_6`: Small routine of 2 instructions that merges 252 into d0, then merges 0 into d0.
- `patch_code_slots_with_jumps`: Writes a constant word pair into one memory location, then writes no-operation pairs, a load-small-constant instruction and absolute jump instructions into fixed code locations.
- `patch_function_pointers`: Overwrites a table or certain memory locations with jump instructions and new routine pointers.
- `patch_instruction_slots`: Writes specific instruction opcodes and target addresses into a memory-mapped table or buffer relative to a base pointer.
- `patch_code_slots`: overwrites a table or code area with branch instructions and a pointer to another routine.
- `patch_two_slots_then_call`: Writes no-operation pairs and two absolute jump instructions into fixed code locations, then calls three unnamed routines.
- `tiny_ori_ori_7`: Small routine of 2 instructions that merges 28 into d0, then merges 0 into d0.
- `patch_code_slots_2`: Overwrites a series of memory locations with specific instructions and replacement routine addresses, effectively redirecting execution or patching existing code branches.
- `patch_code_set_global`: Writes encoded instruction words and no-operation pairs into fixed code locations, then calls three unnamed routines and one more routine.
- `patch_code_then_call_routines`: Writes the same encoded instruction words and no-operation pairs into fixed code locations, then calls ten unnamed routines and two small constant routines.
- `store_register_to_memory`: Copies a word from a data register into a memory location pointed to by an offset from a base address held in an address register.
- `tiny_ori_ori_8`: Small routine of 2 instructions that merges 14 into d0, then merges 0 into d0.
- `tiny_ori_ori_9`: Small routine of 2 instructions that merges 244 into d0, then merges 0 into d0.
- `store_register_to_offset`: Copies a word from a register into a memory location calculated by adding an offset to a value held in another register.
- `store_word_result_in_field`: Has the same code as the previous routine, but is decoded as an address taken by a pc-relative push.
- `store_word_register_after_call`: Calls one unnamed routine, then stores the 16-bit value left in a data register into a structure field.
- `store_word_register_after_call_2`: Calls one unnamed routine, then stores the 16-bit value left in another data register into the same structure field.
- `store_fixed_word_after_call`: Calls one unnamed routine, then stores the fixed value 258 into a structure field.
- `store_fixed_word_after_call_2`: Calls one unnamed routine, then stores the fixed value 1,986 into a structure field.
- `store_fixed_word_after_call_3`: Calls one unnamed routine, then stores the fixed value 4,500 into a structure field.
- `store_fixed_word_after_call_4`: Calls one unnamed routine, then stores the fixed value 12,626 into a structure field.
- `store_fixed_word_after_call_5`: Calls one unnamed routine, then stores the fixed value 12,801 into a structure field.
- `store_fixed_word_after_call_6`: Calls one unnamed routine, then stores the fixed value 14,867 into a structure field.
- `store_fixed_word_after_call_7`: Calls one unnamed routine, then stores the fixed value 16,340 into a structure field.
- `store_fixed_word_after_call_8`: Calls one unnamed routine, then stores the fixed value 17,295 into a structure field.
- `store_fixed_word_after_call_9`: Calls one unnamed routine, then stores the fixed value 17,300 into a structure field.
- `wait_while_global_bit_set`: Busy-waits while bit six of a global flag word stays set, then returns.
- `write_call_stubs_from_table`: Walks a zero-terminated list of offsets and, at each offset from a base register, writes a subroutine-call instruction encoding followed by a word argument.
- `write_absolute_call_slots`: Walks a zero-terminated list of offsets and, at each offset from a base register, writes an absolute subroutine-call instruction followed by a 32-bit argument.
- `write_nop_then_call_slots`: Was decoded from an unreached gap, so it may be data.
- `write_nop_pairs_at_offsets`: Walks a zero-terminated list of offsets and writes a pair of no-operation instructions at each offset from a base register.
- `call_routines_by_flag_chain`: Calls through an indirect pointer, then makes between one and four calls to unnamed routines depending on whether a value is zero, chained in nested branches.
- `compare_null_terminated_strings`: Compares two zero-terminated byte strings and returns zero when they are identical, or minus one at the first mismatch.
- `decompress_checked_bit_stream`: Unpacks a compressed block through the bit reader routines, either copying earlier output bytes backwards or writing literal bytes built from bit fields.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `stub_endless_loop`: The pseudo-code is only an endless loop, with unreachable blocks removed by the decompiler, so nothing beyond the loop is recoverable.
- `compare_result_without_effect`: Calls one unnamed routine and compares its byte result with a global byte, but both branches return with no effect.
- `return_incoming_byte_unchanged`: Returns the low byte of the incoming data register unchanged and has no other effect.

### volley

- `initialize_library_and_loop`: Closes and opens a library, then calls a routine that loops 32,000 times, potentially incrementing a global variable if a file library routine returns zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `add_global_loop_or_close`: Calls a file library routine and, when the result is zero, adds 256 to a global variable in a loop of 32,000 passes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_complex_hardware_configuration`: Performs a large sequence of operations including blitter, copper, and audio configuration, modifies memory via pointers, manages counters, and manipulates certain hardware or or certain registers.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `modify_memory_via_pointer`: Calculates an address using a base register and an offset, then performs a bitwise OR on the byte at that location.
- `configure_audio_channel_three`: Writes a sequence of constants to global variables and fixed memory fields, interleaved with calls to six unnamed routines and a small routine that adds to a register.
- `delay_busy_loop_long`: Busy-waits by counting a 16-bit counter down through 15,000 steps for each of 100 outer passes, then returns.
- `delay_busy_loop_long_2`: Has the same code as the previous routine: it busy-waits through 15,000 steps for each of 100 outer passes, then returns.
- `delay_busy_loop_short`: Busy-waits by counting a 16-bit counter down through 400 steps, then returns.
- `adjust_two_counters_below_limit`: When a frame counter is below 599, it decrements one global word while that word is at least 95 and increments another global word while it is below 46.
- `step_frame_counter_and_adjust`: Advances a frame counter word and, while it lies from 599 up to 988, decrements three global words, calls an unnamed routine and adjusts two more words.
- `configure_blitter_three_passes`: Sets a blitter mask and register values, then runs a three-pass loop that writes blitter pointer registers and calls a fixed low-memory routine on each pass.
- `adjust_counter_pair_limit`: When a counter word is below 255 and a second word is at most zero, it increments the second word and returns.
- `update_global_variables`: Stores a value from a register into a memory location and updates several global variables and pointers.
- `configure_blitter_four_passes`: Sets blitter mask and register values, then calls a fixed low-memory routine four times with changing pointers, and adjusts two counter words when one is below a limit.
- `start_blitter_passes_and_wait`: Sets blitter control and pointer values and calls a fixed low-memory routine four times, then waits until a busy flag in a global word clears.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `configure_blitter_from_rectangle`: Computes a blitter size from two words of a structure and stores position and offset values, then programs the blitter source, destination and modulo registers over four passes.
- `interleave_three_tables_into_global`: Copies three word tables into global memory in an interleaved pattern: for each of sixteen entries it writes a running offset word, starting at 384 and rising by two, followed by a table entry.
- `save_custom_registers_then_set`: Copies five global register values into fields, then writes a routine address and two constants to a global variable.
- `restore_saved_custom_registers`: Writes five saved values back to a global variable, clearing one bit of the first and setting the top bit in four of them.
- `acknowledge_request_flag_bit`: If bit five of a global word is set, it increments that word, calls an unnamed routine and sets the word to 32; otherwise it sets the word to 4,063 and returns.
- `set_copper_list_pointers`: Writes four address pairs into fixed fields, waits until a global word reads minus one, then stores another address and writes a fixed constant.
- `swap_copper_list_buffers`: Toggles a flag to choose between two buffer addresses, waits for a global counter to reach a threshold and then for a handshake value, and writes four address pairs derived from the chosen buffer.
- `tiny_ori_addq`: Small routine of 2 instructions that merges 2 into d1, then adds 8 to d0.
- `select_audio_volume_from_table`: Scans a table of up to 128 bytes for the first entry greater than the current value, then clears several global words and loads a byte from another table field.
- `clear_audio_dma`: The routine writes to the four audio volume registers and to the DMA control register, setting the audio channels off and clearing DMA enables.
- `advance_audio_channel_step`: Advances a step counter that cycles through five values.
- `apply_channel_command_nibble`: Decodes the low nibble of a command byte in a structure: codes one and two call an unnamed routine and then decrement or increase a word with limits, code twelve stores a byte, and code fifteen stores a nibble in a global word.
- `check_pointer_then_return`: The pseudo-code only compares a pointer with the address of a global variable and returns, so it has no visible effect.
- `load_audio_channel_from_table`: Copies a long from a source buffer into a channel structure, then uses the high nibble of a control byte to select a 30-byte table entry and copies its fields into the channel.
- `tiny_ori_2`: Small routine of 1 instruction that merges 2 into d0.
- `set_sentinel_then_call_routines`: Stores a minus-one sentinel into a global long and calls several unnamed routines in sequence, one of which is a jump-table entry that passes control elsewhere.
- `empty_return_routine`: This routine is empty and performs no operations before returning.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `add_global_loop_or_close_2`: Has the same code as the earlier routine that calls library routines at fixed offsets: it calls a library routine and, when the result is zero, adds 256 to a global variable over 32,000 passes.
- `add_global_loop_passes`: Adds 256 to a global variable in a loop of 32,000 passes, then returns.
- `load_address_routine`: This routine contains 14 instructions and is called by a routine that adds a global loop pass.
- `execute_single_instruction`: This routine executes a single instruction and returns.
- `decompress_bit_stream_block`: Unpacks a compressed block through the bit reader routines, copying earlier output bytes backwards or writing literal bytes built from bit fields.
- `decompress_bit_stream_fragment`: A fragment of the same bit-stream decoder that enters through a jump label, decoded from an unreached gap so it may be data.
- `decompress_bit_stream_fragment_2`: Another fragment of the same bit-stream decoder, decoded from an unreached gap so it may be data.
- `decompress_bit_stream_fragment_3`: A further fragment of the same decoder, reached through a jump label and decoded from an unreached gap.
- `count_global_full_wrap`: Repeatedly stores a 16-bit counter into a global variable, counting down from minus one through the full 16-bit range until it wraps to zero, then returns.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `decompress_bit_stream_block_2`: Unpacks a compressed block of the same kind as the other decoders in this file, with a final countdown into a global variable when the final value differs from the header.
- `rotate_right_extend_long`: Returns a 32-bit word read from memory, shifted right one bit, with the top bit forced set.
- `rotate_bits_through_extend`: Rotates a 32-bit register right one bit at a time for a count taken from a data register, reloading from memory whenever the register empties.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_and_initialize_game_state`: Clears a large number of 16-bit global variables, resets several counters, and configures hardware registers including the DMA controller and the video beam position.
- `set_ciaa_bit`: Sets bit 0 of the CIAA Port A Data Register and then clears a 32-bit value at a memory location.
- `clear_memory_block`: Fills a memory range starting at a fixed address and ending at a fixed address with zeros using a loop.
- `setup_and_clear_buffer`: Disables interrupts, saves the stack pointer to a global variable, calls another routine, then clears a block of memory using a loop, writes a constant to a global variable, and returns from an interrupt.
- `compare_and_return`: Compares a long value in register d0 with a constant and returns if they are not equal.
- `zero_data_word`: Performs a logical OR operation on register d0 with zero.
- `tiny_ori_ori_10`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d2.
- `tiny_ori_4`: Small routine of 1 instruction that merges 0 into d1.
- `tiny_ori_5`: Small routine of 1 instruction that merges 1 into d1.
- `tiny_ori_6`: Small routine of 1 instruction that merges 1 into d3.
- `swap_data_structures`: If a specific word in memory is zero, it toggles a bit in another word, swaps two sets of data structures by exchanging their contents, updates a global word with a value from a table, and swaps two other data structures.
- `swap_and_render_sequence`: Swaps several words and a longword between two structures, then performs a series of conditional loops and calls to update global variables and hardware registers.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `update_blitter_parameters`: Calculates various offsets and values based on a structure in a0, writes to several hardware registers, and calls another routine multiple times within a loop.
- `null_operation_padding`: This routine performs a logical OR operation on the data register zero with zero and then continues into the subsequent code.
- `initialize_global_state`: Clears or sets various 16-bit words in multiple global variables and then calls five different routines before performing a conditional branch based on two global words.
- `execute_render_sequence`: Calls several routines to clear a global word, update a global word, and perform two separate calls to another routine with different values in d0.
- `set_audio_volume`: Runs a nested loop to perform a delay, then writes a value to the AUD3 Length register and sets the AUD3 Volume register to 1.
- `update_counters`: Compares a word in memory against several constants and, based on the result, increments one of two global counters and writes a value to a third global variable.
- `copy_structured_data_2`: Calculates two source addresses based on values in a table, then calls another routine twice to copy data from those locations.
- `copy_structured_data`: Runs a nested loop that copies 10 words from one memory area to another, using specific offsets for each source and destination pointer increment.
- `perform_blitter_commands`: Clears several global words, then performs a series of calls to another routine with various structures and values to update several global variables and hardware registers.
- `set_hardware_parameters`: Writes a value to a specific memory location, clears a global word, and calls two other routines.
- `prepare_sprite_structures`: Updates several destination structures by adding offsets to values from source structures, performs conditional checks on global variables, and calls another routine four times.
- `manage_sprite_rendering_state`: Checks multiple global variables and performs conditional blitter calls by setting a flag in a global variable and calling another routine based on comparisons of structure values.
- `execute_complex_animation_logic`: Updates a counter and toggles a flag, then performs several conditional blitter operations and structure copies based on various global variable states and counters.
- `performs_logical_or_on_d0`: Performs a bitwise logical OR operation on the contents of register d0 using the constant zero.
- `perform_logical_or`: Performs a bitwise OR operation on register d0 with zero.
- `reset_game_state`: Checks several global words and a bit in a global variable; if conditions are met, it clears a large number of global words, sets specific values in two globals, and calls several routines.
- `update_audio_and_jump`: Branches to an offset, clears a 32-bit word in a global variable, calls another routine, clears a 16-bit word in a global variable, calls another routine, loads a pointer from a table into a1, moves a word from that pointer to a global variable, and branches to another offset.
- `update_sprite_table_and_counters`: Iterates through a table of data to update a structure's pointer, width, and height values, while managing several counters and calling check_value_range and check_joystick_button.
- `check_value_range`: Compares two 16-bit values from two different memory locations and, if the second value plus 30 is greater than the first value and the first value is greater than or equal to the second value, sets two 16-bit words in a global table to 1.
- `initialize_sequence_data`: Calls several routines with different table addresses, clears many global words, and populates structures with values from a table.
- `update_object_properties_and_iterator`: Iterates through a table of data structures, copying longwords and words from selected entries into a destination structure based on a counter.
- `verify_offset_bounds`: Compares a 16-bit word from a memory location with a 16-bit word from another location and, if the second value plus 30 is greater than the first value and the first value is greater than or equal to the second value, sets a 16-bit word in a global variable to 1.
- `clears_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `tiny_ori_7`: Small routine of 1 instruction that merges 117 into d0.
- `initialize_and_render_sequence`: Writes constants to a structure, calls another routine eight times with different addresses, and then copies a word from one location to another with an offset.
- `process_sprite_sequences`: Adjusts values in a structure based on a global word, clears numerous global words, calls another routine, populates several structures with coordinates and dimensions, calls another routine multiple times with different structures, executes two loops, calls another routine, and updates a structure with a pointer and two words based on a global counter before calling another routine.
- `modify_registers_sequentially`: Performs a series of bitwise OR operations on registers d0 and d6 using various constant values.
- `check_joystick_button`: Tests a word in a global variable and a byte from a hardware register, then writes a word to a global variable if the byte is not negative.
- `test_ciaa_bit`: Reads a byte from a hardware register, masks it with a constant, and writes a word to a global variable if the result is zero.
- `compare_boundary_values`: Compares values from a structure in a0 and a table to set four 16-bit global variables, returning 1 if two specific bitwise AND results are non-zero.
- `nullify_data_register_zero`: Performs a logical OR operation on data register zero with zero, effectively leaving the register unchanged.
- `zero_d0_register`: Performs a logical OR operation on the d0 register with zero, effectively leaving its value unchanged.
- `zero_data_padding`: This routine performs a logical OR operation on register d0 with zero, effectively performing a no-operation.
- `set_d1_bit`: Performs a bitwise OR operation on register d1 with 1.
- `set_bit_zero_in_d1`: Sets the least significant bit of register d1 to one.
- `zero_data_padding_2`: This routine consists of a single instruction that performs a logical OR operation on a data register with zero, effectively performing a no-operation.
- `perform_logical_or_operation`: Performs a logical OR operation on register d0 using a zero constant.
- `performs_logical_or_operation`: Performs a logical OR operation on register d0 with the constant zero.
- `perform_logical_or_on_data_register_zero`: Performs a bitwise OR operation on data register zero using a zero constant.
- `clear_d0_register_3`: Clears the d0 register by performing a logical OR operation with zero.
- `populate_data_table`: Calculates an offset based on a global variable, then loops to fill a destination buffer with incrementing 16-bit values and values from a source table.
- `set_d2_sequence`: Sequentially performs bitwise OR operations on register d2 with eight different constant values.
- `update_hardware_registers`: Copies several hardware register values into a structure in a0, updates two global pointers, and writes specific values to two hardware registers.
- `configure_interrupt_masks`: Sets several hardware registers to maximum values, stores four values from a table into two global variables, and then overwrites the two global variables with the same values from the table.
- `handle_joystick_input`: Checks a bit in a CIAA register, modifies a hardware register, calls another routine, and if a specific byte in a global variable matches a counter, it enters a loop that increments a global variable and a counter for 32000 iterations.
- `update_audio_and_hardware_status`: Reads the Amiga audio status register to update two global status words based on bitwise transformations of the audio channel flags.
- `adjust_global_counters`: Reads a value from a table using an offset, performs several conditional checks on global variables, and updates a global counter and other global words through calls to other routines.
- `update_blt_parameters`: Calculates a value by shifting and masking a word from a global variable, stores it in a custom register at offset 64 of the hardware base, and then enters a loop that calls another routine three times to update memory using values from two tables.
- `wait_for_blitter_2`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `tiny_ori_8`: Small routine of 1 instruction that merges 1 into d6.
- `initialize_data_tables`: Stores four 32-bit values into a table by splitting them into word components, checks a hardware register, and then writes three values to hardware registers.
- `set_audio_registers`: Writes four specific values to audio hardware registers and one value to a DMA control register before returning.
- `configure_audio_hardware`: Sets several hardware registers for audio period, volume, and length, performs a loop of 200 iterations, and then sets a register to 1 before returning.
- `play_audio_sequence_one`: Sets audio hardware registers for channel 3 and calls another routine to create a delay.
- `play_audio_sequence_two`: Sets audio hardware registers for channel 3 with a different period value and calls another routine to create a delay.
- `play_audio_sequence_three`: Sets audio hardware registers for channel 1 and calls another routine to create a delay.
- `play_audio_sequence_four`: Sets audio hardware registers for channel 3 with a different address and period and calls another routine to create a delay.
- `configure_and_play_audio_sample`: Sets the audio channel enable, volume, and period, calls a delay routine, then updates the audio sample pointer, length, volume, and period before calling the delay routine again and resetting the audio channel enable.
- `play_audio_sequence_six`: Sets audio hardware registers for channel 3 with a different period and length and calls another routine to create a delay.
- `play_audio_sequence_seven`: Sets audio hardware registers for channel 3 with a different length and calls another routine to create a delay.
- `execute_delay_loop`: Decrements a counter from 300 to zero in a loop and then returns.
- `update_audio_parameters_and_state`: Clears several global variables and copies values from a structure into them, then updates a parameter structure and calls three routines to configure audio.
- `update_dynamic_parameters`: Updates a structure with various 16-bit values based on comparisons of a global variable against several thresholds, then calls another routine to process these values.
- `initialize_structure_values`: Clears several global words and then conditionally populates a structure in a0 with different sets of four 16-bit values based on a global variable's value.
- `update_audio_structure_parameters`: Calculates and updates various fields in a structure pointed to by a0, including coordinate offsets and timing parameters, based on values from a table and a global variable.
- `update_pointer_offset`: Loads two addresses into registers, copies a 16-bit word from an offset of 2 of the second address to an offset of 2 of the first address, and then adds 8 to the value at an offset of 2 of the second address.
- `check_status_flags`: Compares a 16-bit value at a global variable to 2 and another global variable to 1; if neither match, it sets a global variable to 1 and clears another global variable before returning.
- `search_and_increment_table`: Iterates through a table using a pointer derived from a global variable until a zero word is found, copies that word to another global variable, and then increments a global variable by 2 if a specific condition is met.
- `calculate_position_offsets`: Updates several global variables by performing arithmetic on a value from a structure in a0, including a conditional calculation involving bit shifts and additions, and calculates a pointer offset.
- `calculate_division_result`: Performs a division using a value from a structure in a0 and a 16-bit value from a0, updates a global variable with the result, and clears another global variable.
- `compute_difference_and_store`: Subtracts a 16-bit word at an offset of 2 of a2 from a 16-bit word at an offset of 2 of a1, stores the negated result in a structure at an offset of 6 of a0, and ensures the value is at least 1.
- `update_pointer_from_table`: Iterates through a table until a value is reached, then uses the iteration count to load a 32-bit value from a table into a global variable and clears a 16-bit global variable.
- `modify_coordinate_and_counter`: Sets a register to a series of constant values and modifies a global variable by adding or subtracting values based on the register's content.
- `update_physics_and_color`: Updates a coordinate structure based on a state value, modifies a counter using a table of offsets, and adjusts the hardware color register and a global state variable based on the resulting position.
- `copy_data_structure`: Copies two words from a source structure to a destination structure, stores a long value at offset 4 of the destination structure, and sets a byte at offset 0 of a global variable to 1.
- `update_counter_based_on_conditions`: Checks a global variable against 2 or 4; if not matching, it checks a value at a pointer in a0.
- `update_position_and_state`: Checks several global status and mode variables to determine a signed offset, then adds this offset to both a global coordinate and a secondary global value.
- `toggle_status_flag`: Loads a 16-bit value from a global variable into d0, decrements d0 if a second global variable is zero, and then performs a bitwise XOR on that second global variable with 1 before returning.
- `set_d1_bit_2`: Performs a bitwise OR on register d1 with 1.
- `decrement_and_process_buffer`: Decrements a global 16-bit value; if it reaches 0, it sets it to 3 and searches through a table via a pointer in a1 to find a non-negative value, then copies that value to a structure, sets two words at offsets 16 and 18, and increments the pointer in a global variable.
- `update_joystick_position`: Checks four global variables for zero, then reads the JOY1DAT register.
- `adjust_values_from_joystick`: Reads the JOY1DAT register and performs bitwise operations to check specific bits.
- `update_mouse_position`: Checks four global variables for zero, then reads the JOY0DAT register.
- `fetch_object_data_from_table_2`: Checks several global status flags and, based on a bitmask in a register, decrements a counter in a global variable and fetches a 32-bit pointer from a table.
- `tiny_ori_9`: Small routine of 1 instruction that merges 32 into d6.
- `tiny_ori_10`: Small routine of 1 instruction that merges 36 into d6.
- `tiny_ori_11`: Small routine of 1 instruction that merges 168 into d6.
- `tiny_ori_12`: Small routine of 1 instruction that merges 172 into d6.
- `tiny_ori_13`: Small routine of 1 instruction that merges 32 into d5.
- `tiny_ori_14`: Small routine of 1 instruction that merges 36 into d5.
- `tiny_ori_15`: Small routine of 1 instruction that merges 232 into d5.
- `tiny_ori_16`: Small routine of 1 instruction that merges 244 into d5.
- `fetch_object_data_from_table_3`: Checks several global status flags and, based on a bitmask in a register, decrements a counter in a global variable and fetches a 32-bit pointer from a table.
- `fetch_object_data_from_table_4`: Checks several global status flags and, based on a bitmask in a register, decrements a counter in a global variable and fetches a 32-bit pointer from a table.
- `tiny_ori_17`: Small routine of 1 instruction that merges 32 into (a0)+.
- `fetch_object_data_from_table`: Checks several global status flags and, based on a bitmask in a register, decrements a counter in a global variable and fetches a 32-bit pointer and two 16-bit values from a table into a structure.
- `validate_and_reset_audio_state`: Checks if a global flag is set and a status variable is non-zero, then performs range checks on two values from a structure relative to offsets of 10, 5, 28, and 32.
- `clear_d0_register`: Performs a bitwise OR on register d0 with 0.
- `clears_data_register_d0_2`: Performs a logical OR operation with zero on register d0.
- `validate_range_bounds`: Compares values from two structures and a global variable to check if they fall within specific bounds, otherwise it sets a global word to one and returns.
- `validate_range_and_set_flag`: Compares values from two structures to check if they fall within specific bounds and if a global variable is negative.
- `check_structure_overlap`: Compares a word from one structure with a word from another structure plus a constant, and if the comparison fails a specific range check, it sets a global word to one and returns.
- `update_table_indices`: Checks a 16-bit flag; if not set, it increments two global variables by 23 and sets the flag.
- `manage_audio_sequences`: Checks several global flags and counters to determine if an audio sequence should be updated, decremented, or reset.
- `process_audio_sequence_2`: Decrements a 16-bit global variable and, if zero, clears several globals and performs conditional logic to copy values from a table into a structure, adjust a global pointer and counter, and call two other routines before updating a global variable using a byte from a table.
- `update_audio_state`: Decrements a 16-bit value in a global variable and, if it reaches zero, clears several global variables, modifies two other globals by subtracting 10, and performs a series of comparisons between values in two structures before calling two other routines.
- `process_audio_sequence`: Decrements a global word and, if it reaches zero, clears several globals.
- `decrement_sequence_counter`: Decrements a global word and, if it reaches zero, clears several globals and updates a structure.
- `decrement_and_update_counters_2`: Decrements a global word and, if it reaches zero, clears several globals and updates a structure; otherwise, it conditionally updates a structure based on the value of the decremented word, clears several globals, calls two routines, and finally subtracts a byte retrieved from a table using the decremented word as an index from two global values.
- `load_sequence_data`: Checks three global words; if they are zero, it clears many globals and sets a global word to 2.
- `performs_logical_or_operation_2`: Performs a logical OR operation on data register zero using a zero constant.
- `clear_data_register_d0`: Performs a logical OR operation on data register zero with zero, which has no effect on the register's value.
- `clear_d2_byte`: Performs a bitwise OR on register d2 with zero.
- `update_animation_frame_and_audio`: Checks several global variables and, if conditions are met, selects a set of coordinates from a table based on an index, updates a destination structure, and calls several routines including an audio update and a routine that modifies a hardware register.
- `clear_d0_byte`: Performs a bitwise OR on register d0 with zero.
- `process_animation_frame_sequence_2`: Checks several global flags and status variables before selecting a set of three values from a table based on an index.
- `clear_data_register_zero`: Performs a logical OR operation on data register zero with zero, effectively leaving its value unchanged.
- `clear_data_register_2`: Sets the least significant bit of data register 2 to zero.
- `compare_and_clear_globals`: Compares a word from a structure in a0 with a word from a structure in a1 plus ten, and if the comparison result is not less than zero, clears four global words and calls another routine before returning.
- `update_audio_state_logic`: Checks a global word and a global word for specific values, performs several comparisons between words in structures pointed to by a0 and a1 with various offsets and additions, and if certain conditions are met, clears three global words, sets two global words to one, calls another routine, clears a global word, and calls another routine before returning.
- `manage_audio_sequence_and_timers`: Checks multiple global status flags and decrements several counters, updating audio hardware parameters and playing specific sound sequences based on the remaining timer values.
- `clears_data_register_zero_2`: Performs a logical OR operation on data register zero with zero to ensure the register is cleared.
- `clear_data_register_d0_2`: Performs a logical OR operation on register d0 with zero to clear its contents.
- `zero_d0_register_2`: Performs a logical OR operation on the data register zero with zero, effectively leaving its value unchanged.
- `clear_d0_register_2`: Performs a logical OR operation on the data register zero with zero to clear its contents.
- `zero_data_padding_3`: This routine consists of a single instruction that performs a logical OR operation on a data register with zero, effectively leaving the register unchanged.
- `clear_data_d0`: Performs a logical OR operation on register d0 with zero, effectively ensuring the register contains zero.
- `performs_logical_or_operation_3`: Performs a bitwise logical OR operation on data register zero using a zero constant.
- `or_byte_d0_alt`: Applies a bitwise OR with zero to the byte in register d0.
- `manage_complex_state_transitions`: Evaluates multiple global words and compares a global word against eighty-eight, then performs a series of conditional updates to structure members in a0, calls another routine, updates several global words, performs calculations involving structures in a1 and a2 to update global words, and finally updates a global word with a calculated value before returning.
- `process_index_list`: Checks a 16-bit value; if non-zero, it updates a structure at a0 with a pointer and two constants.
- `update_structure_data`: Checks a 16-bit value; if non-zero, it checks if a value at a table offset is 255.
- `update_structure_data_3`: Checks if a word at a global variable is non-zero; if so, it checks if a word at a second global variable plus an offset is 255.
- `update_pointer_structure`: Checks a 16-bit value; if non-zero, it updates a structure at a0 with a pointer and two constants.
- `process_audio_sequence_and_state`: Checks several global flags and counters before iterating through an audio sequence structure by incrementing a counter and updating parameters based on the current index.
- `or_byte_d0_sub`: Executes an OR operation on register d0 using a zero immediate value.
- `or_byte_d0_var`: Performs a bitwise OR on the byte in register d0 with zero.
- `or_byte_d0_exec`: Ors the byte in register d0 with zero.
- `tiny_ori_18`: Small routine of 1 instruction that merges 0 into d6.
- `tiny_ori_19`: Small routine of 1 instruction that merges 156 into d6.
- `process_animation_frame_sequence`: Checks several global flags and a counter before iterating through a table to update a structure with specific values and offsets.
- `perform_null_operation`: Performs a logical OR operation on the least significant byte of data register zero with zero, effectively acting as a no-operation instruction.
- `zero_data_padding_4`: This routine contains a single instruction that performs a logical OR operation on a data register with zero and does not return.
- `tiny_ori_20`: Small routine of 1 instruction that merges 96 into d6.
- `tiny_ori_21`: Small routine of 1 instruction that merges 154 into d6.
- `update_sequence_parameters`: Checks a global word and if non-zero, either clears several global words or updates a structure with values from a table and calls two other routines.
- `zero_data_padding_5`: This routine performs a logical OR operation on data register zero with zero, effectively performing a no-operation.
- `or_byte_d0_jump`: Executes a bitwise OR on register d0 with zero.
- `or_byte_d0_trap`: Performs a bitwise OR operation on the byte in register d0 with zero.
- `set_bits_6_and_7_in_d6`: Performs a bitwise OR operation on register d6 with the constant ninety-six.
- `or_immediate_to_d6`: Performs a bitwise OR operation on register d6 with the constant 154.
- `process_audio_timing`: Checks several global variables and if specific conditions are met, updates a structure with a pointer and constants, calls two routines, and then decrements a global counter in a loop.
- `clear_data_register_d0_3`: Performs a bitwise OR operation on data register zero with zero.
- `tiny_ori_22`: Small routine of 1 instruction that merges 10 into d0.
- `manage_audio_state_and_timers`: Checks several global status flags and decrements counters, updating audio hardware parameters and playing specific sound sequences.
- `adjust_coordinate_ranges`: Clears a 16-bit flag and checks several global conditions.
- `apply_bitmask_to_word`: Performs a bitwise OR operation on a 16-bit word located at an offset of 76 from the sum of a6 and d0 using a constant mask.
- `tiny_ori_23`: Small routine of 1 instruction that merges 0 into -(a4).
- `manage_dynamic_offsets`: Clears a 16-bit flag and performs complex conditional checks on global variables and modes.
- `update_range_parameters`: Checks several global variables for non-zero values.
- `set_indexed_byte`: Sets a byte at the address calculated by adding d0 to the pointer in a2.
- `clear_global_word`: Clears a 16-bit word at offset 4 of the structure pointed to by a6.
- `search_and_update_table_pointer`: Checks a global word; if zero, it searches a table for the first non-zero word by incrementing an offset, updates a pointer with two words from a source, and sets a global flag to one.
- `update_global_value`: Calculates the absolute difference between two 16-bit words at offsets 2 of a3 and a2, then stores a value in a global table based on that difference and potentially increments it if a global flag is non-zero.
- `update_coordinate_offset_table`: Reads a structure from a pointer in a0, updates a global 16-bit value by adding or subtracting specific constants based on several conditional checks of global words and structure members, and performs arithmetic operations including multiplication and division to calculate a new value stored in a global variable.
- `calculate_squared_sum`: Clears registers d1 through d5, calculates the sum of the squares of two different absolute differences involving a table at a3 and two input structures in a1 and a2, and returns the result in d3 and d4.
- `calculate_toggle_sum`: Clears a 16-bit variable, then enters a loop that runs a number of times determined by a value in a table.
- `set_bit_zero_in_d1_2`: Sets bit zero of register d1 using a logical OR operation.
- `render_blitter_sprite_sequence`: Calculates blitter dimensions and source/destination coordinates based on input structure values, then performs a series of blitter operations to draw multiple sprite or tile segments using a sequence of calls to a drawing routine.
- `copy_table_entries`: Multiplies a value in d0 by 4 to find an offset in a table, then enters a loop that copies words from the table to a destination address.
- `copy_data_block`: Copies eight bytes from a source table to a destination table using a loop that runs four times, updating the source and destination pointers by large offsets each iteration.
- `subtract_from_globals`: Subtracts the value in register d0 from nine different global word variables.
- `toggle_address_table`: Waits for a global word to become zero, then selects one of two base addresses based on a global word, stores the result in a global pointer, and updates four global word variables with the high and low words of various calculated addresses before toggling a global word.
- `find_max_byte`: Iterates through a block of memory to find the maximum byte value, and if a certain condition is met, clears three global variables and stores a byte from a global variable into another global variable.
- `update_audio_channels`: Increments a global word and, if it equals 4, resets it to 0 and performs several operations including clearing flags in a table.
- `process_attribute_byte`: Reads a byte from a structure and performs different operations based on its value, such as adding or subtracting from a value in a4, updating a value in a5, or searching a table.
- `update_audio_registers`: Checks a6 against three specific addresses to return a pointer in a4, otherwise it calculates a value from a table using a byte from a structure, performs a loop to update several hardware audio registers, and updates a global counter and several other hardware registers.
- `update_structure_data_2`: Loads a pointer from a table, extracts a value to index another table, and updates several fields in a structure including a bitwise OR on a global variable.
- `tiny_ori_24`: Small routine of 1 instruction that merges 0 into d3.
- `load_level_assets_and_configure_hardware`: Checks a global variable to determine the current level and executes a sequence of hardware configurations and data loads specific to that level.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `set_bit_one_in_d0`: Sets bit 1 of register d0 using a logical OR operation with the constant 2.
- `set_bit_one`: Sets bit 0 of register d1.
- `do_nothing_2`: Performs a logical OR on register d0 with zero.
- `test_word_and_jump`: Tests a word in a global variable and returns if it is zero; otherwise it jumps to another routine.
- `decode_bit_stream`: Fetches a longword from the address in a0, XORs it with d5, and then performs a series of conditional bit extractions and jumps to copy bytes from a source to the destination pointed to by a2.
- `execute_jump_logic`: Performs a jump to a specific address with registers d1 and d4 set to 8.
- `dispatch_jump_logic`: Sets d1 to two, calls another routine, and if d2 is two or three it performs different jumps or sets d1 to eight and calls another routine before jumping to a specific address.
- `process_bit_stream`: Calls a routine to read bits, then iterates through a loop to copy bytes from a source to a destination based on the read values.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `bk_refill_bit_buffer_3`: Fetches a longword from the address pointed to by a0, performs an XOR with d5, and shifts the value in d0 right by one bit.
- `bk_read_bits_3`: Collects a specified number of bits from d0 into d2, refilling d0 from the memory pointed to by a0 using the bk_refill_bit_buffer logic whenever d0 becomes zero.
- `set_bit_pattern_loop`: Iterates through a table of words, calling a routine three times for each non-zero entry to modify values based on table contents.
- `execute_blitter_loop`: Sets blitter configuration registers, stores a pointer in a global variable, and executes a loop three times that calls a blitter routine and another routine.
- `set_bit_pattern_single`: Iterates through a table three times, calling a routine to modify values based on table contents.
- `tiny_ori_25`: Small routine of 1 instruction that merges 163 into d4.
- `execute_dual_pattern_loops`: Executes two separate nested loops that iterate through tables and call a routine to modify values using offsets.
- `tiny_ori_26`: Small routine of 1 instruction that merges 6 into (a2).
- `set_bit_pattern`: Calculates an offset into a table based on input values, sets a bit and three other bits at specific offsets within that table, and conditionally modifies bits based on a global variable.
- `clear_bit_pattern`: Calculates an offset into a table using input values, clears a bit at that offset, clears three other bits at specific offsets, and sets a final bit at a specific offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_level_assets_and_graphics`: Clears a memory block starting at 32000 and decrements a global variable, then executes a series of conditional calls to routines that initialize graphics structures and perform blitter operations based on the value of a global variable.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `run_nested_loops`: Executes a nested loop structure where the outer loop runs three times and the inner loop runs two times.
- `process_list_with_blitter`: Iterates through a list of pointers, calling a routine to wait for a raster line and then executing conditional blitter operations.
- `iterate_data_list`: Sets a global variable to 1 and iterates through a list of structures pointed to by a0, calling execute_conditional_blitter with the first two words of each structure and advancing the pointer by 4 bytes until a word containing all bits set is encountered.
- `perform_blitter_transfers`: Configures blitter registers, updates a global variable by shifting and ORing, and executes a nested loop that performs blitter operations and calls another routine.
- `perform_blitter_transfers_shifted`: Sets a global variable to all ones, configures blitter registers, and executes a nested loop that performs blitter operations and calls another routine after shifting the global variable right.
- `decrement_and_update_counters`: Decrements a value in a global variable, and if it becomes negative, sets it to all ones and increments five other global variables by two before calling another routine in a loop.
- `increment_and_update_counters`: Increments a value in a global variable, and if it reaches a threshold, sets it to zero and decrements five other global variables by two before calling another routine in a loop.
- `execute_conditional_blitter`: Checks a global variable and calculates a destination address, then configures blitter registers and executes a loop that performs blitter operations using a source and destination pointer.
- `update_pointer_and_register`: Writes a value to a hardware register, copies a value from a global pointer to another global pointer, and then writes a different value to the same hardware register.
- `restore_saved_interrupt_state`: Disables interrupts by writing to the interrupt control register, restores a previously saved value from a global variable, and then re-enables interrupts.
- `handle_interrupt_request`: Checks bit 5 of the interrupt status register and a global word, calls another routine if both are non-zero, and then writes a value to the interrupt mask register before returning via an exception return.
- `tiny_ori_27`: Small routine of 1 instruction that merges 104 into d0.
- `initialize_hardware_parameters`: Stores four pairs of 16-bit values into a table, updates a pointer in a global variable, sets a value in a hardware register, and then overwrites that same hardware register with a different 16-bit value.
- `configure_audio_hardware_2`: Clears a word at a memory location, sets several hardware audio registers, performs a loop that decrements a counter 400 times, and sets a hardware register to one.
- `fill_table_with_incrementing_values`: Fills a table starting at a specific address with values that increase by two for each entry, while copying 16-bit words from a source buffer, repeating this process 15 times.
- `set_display_and_loop_call`: Writes one and zero to two consecutive global memory locations, then enters a loop that calls another routine up to one hundred and twenty times if a specific hardware register bit is set.
- `perform_graphics_operations`: Writes a value to a global structure, calls three routines, populates a structure in a0 with four 16-bit words, calls two more routines, and finally calls another routine and returns with d0 set to a constant.
- `update_graphics_and_table_values`: Initializes several structures with width, height, and source address parameters, then enters a loop that performs multiple blitter operations to draw graphics while decrementing and updating values in a table based on data read from memory.
- `execute_nested_loops`: Runs a nested loop structure where an inner loop runs 2000 times and an outer loop runs 1000 times.
- `update_graphics_tables_and_blitter`: Initializes several structures with coordinate and size data, then executes three distinct loops that perform blitter operations to draw graphics based on values in various tables.
- `update_graphics_and_table_parameters`: Initializes several structures with coordinate and size data, then enters a loop that performs blitter operations based on values in a table.
- `update_graphics_and_scroll_tables`: Initializes several graphics structures with coordinates and dimensions, then enters a loop that updates these structures and calls a routine to perform blitter operations.
- `copy_short_block`: Copies 16 words from a source pointed to by a register to a destination pointed to by another register using a loop.
- `update_graphics_and_scroll_tables_2`: Initializes several graphics structures with coordinate and offset data, then enters a loop that toggles a flag and modifies table values to update screen parameters.
- `update_graphics_and_counters`: Initializes several structures with coordinate and size data, then enters a loop that decrements two primary counters while performing blitter operations to update screen graphics.
- `update_graphics_and_counters_2`: Initializes several graphics structures and counters, then enters a loop that performs blitter operations to draw sprites or backgrounds based on values in a table.
- `perform_blt_operations`: Sets several hardware bitplane registers, calculates a destination address based on a global variable and an offset, and calls another routine three times within a loop while updating source and destination pointers.
- `scatter_table_values_2`: Calculates an offset into a table, reads two bytes to determine multipliers, and calls a routine to copy data to a destination address.
- `scatter_table_values`: Copies 17 words from a source table to a destination table at specific increasing offsets, repeating this process four times.
- `update_blt_graphics`: Calculates screen coordinates and bitplane offsets based on input structure values, then performs multiple Blitter operations to draw graphics.
- `fill_table_with_four`: Loops 15 times to write the value 4 into every 4th byte of a table starting at a global variable, then returns.
- `check_and_call_conditional_routines`: Saves the current value of d0 to the stack, checks if a 16-bit word at a global address is non-zero to call another routine, checks if a second 16-bit word at a different global address is non-zero to call a third routine, restores d0 from the stack, and returns.
- `contains_zero_data`: Consists of eight consecutive instructions that perform a bitwise OR on register d0 with zero.
- `decrement_and_copy_table`: Decrements a global counter and, upon reaching zero, copies a sequence of 16 words from one location to another, then calls another routine.
- `decrement_and_set_d5`: Decrements a 16-bit word at a memory location, and if the result is not zero, sets a 16-bit word at another location to 2 and sets d5 to 1; otherwise it sets the word to 2 and sets d5 to 1.
- `process_bitmask_table`: Performs a nested loop to modify values in a table using bitwise operations and increments a global counter until it reaches a threshold.
- `contains_zero_data_2`: Consists of a single instruction that performs a bitwise OR on register d0 with zero.
- `tiny_ori_28`: Small routine of 1 instruction that merges 1 into d0.
- `load_asset_data_by_index`: Reads an index from a global variable and performs a series of data transfers based on that index.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `clear_data_register_zero_2`: Performs a logical OR operation on data register zero with zero, effectively ensuring the register contains zero.
- `clears_data_register_zero_3`: Performs a logical OR operation on data register zero with zero to ensure its value is zeroed.
- `clear_data_register_zero_3`: Performs a logical OR operation on data register zero with zero to ensure the register contains zero.
- `clear_data_d0_2`: Performs a logical OR operation on register d0 with zero, effectively leaving its value unchanged.
- `contains_zero_data_3`: Consists of two consecutive instructions that perform a bitwise OR on register d0 with zero.
- `clear_audio_and_data`: Loops through a table to compare bytes, and if a condition is met, clears four audio volume registers, a global variable, and two other global variables before copying a byte from a table to another global variable and returning.
- `process_audio_updates`: Increments a global word and, if it equals 6, resets it to 0 and performs several operations including clearing flags in a table.
- `update_audio_parameters`: Reads a byte from a structure in a6 and compares it against several constants to either update a value in a4, copy a word from a table to a structure in a5, or store a byte in a structure in a5.
- `update_audio_registers_and_counters`: Checks a pointer against several constants to return specific addresses, otherwise it calculates a value by shifting and adding a byte from a table, calls another routine three times to update audio channel registers, and updates a 16-bit register with the result.
- `update_sprite_attribute_structure`: Fetches a structure from a table using an index derived from a byte at offset 2, then updates various fields including a pointer, two short values, and a DMA controller register.
- `performs_logical_or_operation_4`: Performs a bitwise logical OR operation on the data register zero using a zero constant.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_jump_via_pointer`: Loads a pointer from the stack into a6, updates a memory location with the value in d0, and calls a subroutine with d0 and d4 initialized to specific values, before finally jumping to the address stored at the location pointed to by the value 25000.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately`: the routine is empty and performs no operations before returning.
- `return_immediately_2`: The routine is empty and performs no operations before returning.
- `do_nothing`: This routine is empty and performs no operations.

### animation.code (raw code file, load address unknown)

- `configure_hardware_registers`: This routine contains 149 instructions and modifies audio volume, audio length, certain CIAA and DMA controller registers.

### final.code (raw code file, load address unknown)

- `initialize_library_resources`: This routine contains 238 instructions and uses several library calls to manage libraries and modifies audio, CIAA, color, DMA, and video pointer registers.

### intro.code (raw code file, load address unknown)

- `setup_system_environment`: This routine contains 342 instructions and uses library calls to manage libraries and certain system states while modifying audio, certain CIAA, color, DMA, and BLT registers.

### score.code (raw code file, load address unknown)

- `update_audio_and_dma`: Modifies audio volume registers, updates certain CIA or disk status or button registers, and enables certain DMA channels.

### volley.code (raw code file, load address unknown)

- `manage_system_state_and_blitter`: Restricts or permits certain certain system-wide states via library calls and configures blitter-related registers and DMA or or keyboard or or button or or disk status or or DMA or or certain registers.
