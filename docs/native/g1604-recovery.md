# Game 1604 (plotting (1990)(ocean)[cr angels][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01604). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901122 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 107 of 107 named (0 author symbols), 107 of 107 described (6 by rule, 18 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 101 of 101 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 387866 bytes, 14 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1389 of 1389 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 38829 |
| code | 4202 |
| data | 441558 |
| padding | 3349 |
| unused | 413184 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 363 | 12 |
| waitmouse | 2 | 4 | 1 |
| clr | 3 | 19 | 1 |
| Plotting (1990)(Ocean)[cr Angels][a].adf:plot.demo | 1 | 99 | 3 |
| Plotting (1990)(Ocean)[cr Angels][a].adf:plot | 5 | 598 | 68 |
| Plotting (1990)(Ocean)[cr Angels][a].adf:bootblock | 1 | 10 | 1 |
| logged disk load from Plotting (1990)(Ocean)[cr Angels][a].adf | 1 | 91 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Plotting (1990)(Ocean)[cr Angels][a].adf:A (raw code file, load address unknown) | 1 | 0 | 0 |
| Plotting (1990)(Ocean)[cr Angels][a].adf:PRO.BIN (raw code file, load address unknown) | 1 | 204 | 14 |

## Cases handled

- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 3 64 KB pages each with >= 29491 distinct PCs executed, 51137 of its 52316 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 3 64 KB pages each with >= 29491 distinct PCs executed, 51137 of its 52316 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 3 64 KB pages each with >= 29491 distinct PCs executed, 51137 of its 52316 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 3 64 KB pages each with >= 29491 distinct PCs executed, 51137 of its 52316 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- prefix run f150 ignored: it logged 15320 PCs the canonical run never executed (not a prefix of it)
- plotting (1990)(ocean)[cr angels][a]/Plotting (1990)(Ocean)[cr Angels][a].adf: 79 executed PCs whose RAM was overwritten before any snapshot match its bytes 460304-460720: recovered as a unit run at an address
- 1 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 0 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- plotting (1990)(ocean)[cr angels][a]/Plotting (1990)(Ocean)[cr Angels][a].adf:plot.demo: TetraPack 2.2 (xfd) stream found by ancient scan (14616 -> 65536 bytes) but only 0 of 10 sampled windows are in reference RAM: not used
- plotting (1990)(ocean)[cr angels][a]/Plotting (1990)(Ocean)[cr Angels][a].adf:A: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- plotting (1990)(ocean)[cr angels][a]/Plotting (1990)(Ocean)[cr Angels][a].adf:PRO.BIN: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- plotting (1990)(ocean)[cr angels][a]/Plotting (1990)(Ocean)[cr Angels][a].adf: 1 logged loads cover 0 bytes of the image
- 1 never-loaded region(s), 387154 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_and_verify_data` (plot): Enters a loop that copies 428 longwords from a source to a destination while calculating an XOR checksum, and if the checksum matches a constant, it calls another routine to decrypt a table and perform bit manipulations on a hardware register. Kind: disk hardware check. Pass path: The routine proceeds to initialize data and then calls the jumptable at pcRam01005284 after verifying checksums and manipulating CIAB_PRB pins, indicating a successful hardware check and continuation of execution..
- `retry_nested_operation_limits` (plot): Clears a result word, then calls a routine to run an operation and retries it while counting failures, with an outer loop that repeats the whole attempt up to five times and saves intermediate values between tries. Kind: disk hardware/track retry limit check. Pass path: the loop breaks when iVar4 == 0, proceeding to reset counters and return.
- `pulse_ciab_port_bit3_low` (plot): Writes a value with every bit except the top one set to the CIA-B port B byte, calls a routine, clears bit 3, and calls another routine. Kind: bit-level hardware state check. Pass path: the routine completes without triggering a crash/reset, allowing the subsequent bitwise manipulation to proceed.
- `pulse_ciab_port_bit3_high` (plot): Sets bits 7 and 3 of the CIA-B port B byte, calls a routine, clears bit 3, calls another routine, then sets bit 3 again. Kind: disk hardware bit manipulation/timing check. Pass path: the code continues execution sequentially after the bit manipulation and subsequent calls.
- `disable_all_interrupts_and_dma_2` (PRO.BIN (raw code file, load address unknown)): Writes a value with every bit except the top one set to the interrupt enable and DMA control registers, twice, then xors a shared word with a fixed constant. Kind: anti-copy/timing protection. Pass path: the code continues to the instructions following the call (lea.l an address(a5), a0).

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_and_call`: When the output pointer passed on the stack is not null, copies 16 longwords from a fixed structure offset to it.
- `initialize_state_and_process_table`: Clears two longword fields at offsets 136 and 140 of the structure in a1 and a global field at offset 632, then calls a routine through a5 several times, storing its D1 results into the structure and global fields at offsets 600, 604 and 608 and branching on zero values.
- `store_result_into_structure_loop_2`: Loops while a counter at offset 152 of a structure is less than or equal to a byte from a table, calling another routine through a pointer and storing results.
- `initialize_global_variable`: Sets a global variable at offset 612 to negative one and then branches to another routine.
- `set_structure_flag_and_loop`: Sets a flag in a structure to negative one, then loops while a counter is less than or equal to a byte from a table, calling another routine and storing results.
- `index_byte_table_store_long`: Computes an index from a long at offset 220 of the structure in a1 shifted left two places plus a long at offset 224, reads the byte at that index from the table in a0 into d2, loads an address from offset 296 of the structure in a4 with a3 derived from it, sets d0 to 240, calls the routine whose address is in a5, then branches to another routine.
- `dispatch_event_bytes_loop`: Repeatedly calls a routine through a pointer in a5 and compares the result in d1 against the codes 72 and 78; for 72 it stores -1 as a long at offset 140 of the structure in a1, for 78 it stores -1 as a long at offset 612 of the structure in a2, and it advances a counter at offset 152 of a1 while that counter is at most a byte from a table in a0.
- `store_result_into_structure_loop`: Loops while the counter at offset 152 of the structure in a1 is at most a byte from a table in a0, calling a routine through a5 and storing values into fixed offsets of a1 and a2, then loads the constant 232 and a pointer at offset 336 of a2 and calls through it.
- `store_result_until_minus_one`: Sets the long at offset 616 of the structure in a2 to 1, then repeatedly calls a routine through a5 and stores d1 at offset 208 of a1, leaving the loop when d1 is -1, and sets the long at offset 624 of a2 to 5 when the loop ends normally.
- `set_long_value_and_call`: Sets a long value at offset 616 of a structure to one, then repeatedly calls another routine through a pointer and stores values from a register.
- `call_indirect_routine`: Loads a value into a register, uses it as an address to call another routine, then jumps to a location in another register.

### waitmouse

- `wait_for_mouse_button_release`: Busy-waits while bit 6 of a global variable is set, then returns zero.

### clr

- `call_library_jump_vectors`: Program entry that calls a library function through a global base pointer, then makes two calls through the pointer that call returned and one more through the global base, and returns zero.

### Plotting (1990)(Ocean)[cr Angels][a].adf:plot.demo

- `unpack_compressed_stream_to_memory`: Copies a block of packed data into a buffer, then reads bits to choose between copying a run of bytes from earlier in the output and emitting literal bytes, looping until the output pointer reaches a fixed limit.
- `refill_bit_buffer_word`: Reads a 32-bit word from memory just before the current input position, stores its low byte through the output pointer, and returns that word shifted right by one with the top bit set.
- `shift_bit_buffer_count`: Shifts a bit buffer right by a count held in a register, calling the refill routine whenever the buffer runs out, and then returns.

### Plotting (1990)(Ocean)[cr Angels][a].adf:plot

- `chain_two_routine_calls`: Four-instruction routine whose decompiled code timed out, so only its facts are known: it calls two other routines.
- `configure_dma_and_interrupts`: Writes the colour, DMA control and interrupt enable registers, then calls one other routine; the decompiler timed out, so the exact values are not shown.
- `setup_plot_boot_dma`: Refers to the text PLOT.BOOT and writes to the DMA control register, then calls two other routines.
- `copy_1001_longs_and_reset`: Sets a bit in a global flag word, calls a routine, then copies 1001 longs from a data block into a fixed low-memory table, writes a constant to a global word twice and xors a shared word with a constant.
- `disable_all_interrupts_and_dma`: Writes a value with every bit except the top one set to the interrupt enable and DMA control registers, twice, then xors a shared word with a fixed constant.
- `initialize_and_verify_data`: Enters a loop that copies 428 longwords from a source to a destination while calculating an XOR checksum, and if the checksum matches a constant, it calls another routine to decrypt a table and perform bit manipulations on a hardware register.
- `decrypt_table_data`: Enters a loop that performs bitwise operations and writes results back to a table starting at a0, then calls another routine.
- `verify_checksum_sequence_2`: Loads a word from a table, calls another routine to interleave its bits, and compares it to a constant; if they match, it enters a loop that XORs a sequence of interleaved words from the table into a register for 3157 iterations, then compares the final result against a third word from the table.
- `restore_registers_and_return`: Restores multiple registers from the stack, sets a register to negative one, and returns immediately.
- `interleave_and_mix_bits`: Saves d1 and d2 on the stack, keeps the even-position bits of the low word of d0 and the odd-position bits of the high word of d0 shifted up one place, combines them with an OR into d0, then restores d1 and d2 and returns.
- `divide_and_update_structure`: Divides d7 by d5, stores the quotient and remainder in a structure in a5 at offsets 684 and 690, updates a 16-bit word at offset 678 based on the quotient, and calls another routine or sets bit 2 of a byte at the address in a4.
- `clear_bit_and_branch_sequence`: Clears bit 2 of a byte at the address in a4, calls three subroutines, and branches to another routine based on a comparison of a 16-bit word at offset 676 of a structure in a5 with a constant.
- `chain_three_operations`: Calls one routine, then another routine with a 16-bit offset, then branches unconditionally to an address in the middle of another routine.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `invoke_and_fall_through`: Calls one routine and then falls through into the next routine without returning.
- `check_bit_and_clear`: Tests bit 4 of a byte at offset 3840 of a4, branches to an address if the bit is set, clears a 16-bit word at offset 676 of a5, then returns.
- `clear_bit_and_increment_counter`: Clears bit one of a byte at a memory location, calls another routine, and increments a 16-bit value at an offset from another register.
- `set_bit_and_countdown`: Sets bit 1 of a byte at an address in a4, calls another routine, decrements a 16-bit word at offset 676 of a5, then returns.
- `clear_and_set_bit`: Clears bit zero of a byte at a memory location, sets bit zero of the same byte, then calls another routine.
- `check_byte_offset_six_loop`: Saves d7, then repeatedly adds 70 to the byte at offset 6 of the structure in a6 until the sum equals that byte again, after which it restores d7 and returns.
- `configure_hardware_and_data`: Writes values to a structure in a6 and a location in a2, calls another routine, copies a longword from a table to offset 32 of a structure in a6, writes 16384 to a location in a3, and enters a loop that continues as long as bit 4 of a byte at offset 3072 of a structure in a4 is not set.
- `write_short_values`: Writes the 16-bit value 16384 to a memory location and the 16-bit value 1024 to another memory location, then restores registers and returns.
- `wait_for_bit_clear`: Tests a specific bit of a byte at an offset from a register, branches back if the bit is set, and returns when the bit is clear.
- `reset_status_byte`: Sets a byte at the address in a4 to 127, calls another routine, clears bit 3 of the byte at the address in a4, calls another routine, and calls another routine.
- `modify_bits_and_jump`: Sets bits 7 and 3 of a byte at the address in a4, calls another routine, clears bit 3 of that byte, calls another routine, sets bit 3 of that byte, and jumps to the address in a5.
- `lookup_uppercased_name_in_table`: Copies a text argument into a buffer converting lowercase letters to uppercase, derives a slot number by hashing the length and characters modulo 72, then walks the table entries in that slot comparing the name with each record.
- `lookup_entry_by_fixed_hash`: Computes a slot number by hashing a fixed run of bytes that starts near its own code, then walks the table entries in that slot, comparing names and copying a matched record out.
- `find_record_at_slot`: Takes a slot index, reads the first entry from the table at that slot, and walks the chain comparing the requested name with each stored name.
- `set_status_six_then_exit`: Sets a status word in the lookup context to 6, then calls one routine and returns.
- `set_status_seven_then_exit`: Sets a status word in the lookup context to 7, then calls one routine and returns.
- `match_text_then_copy_record`: Compares a text argument byte by byte against a stored name, and on a full match copies the record out; when the comparison fails it sets a status value and calls the record-copy routine.
- `scan_records_for_key`: Walks records one at a time, checks a count field in each, and compares a stored key with a fixed text.
- `copy_record_bytes_to_buffer`: Copies a record's stored bytes into the output buffer in a loop driven by a length field and a counter, and sets a status value if the first check fails.
- `copy_record_payload_loop`: Repeatedly copies a fixed-length payload from a record into the output buffer, advancing the output pointer and stopping when a counter goes negative.
- `set_status_one_then_exit`: Sets the status word in the lookup context to 1, then calls one routine and returns.
- `verify_table_checksum`: Temporarily zeroes one stored word, sums the negated 128 longs of the table, restores the word, and returns zero when the sum equals the saved value or minus one otherwise.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `set_status_five_then_exit`: Sets the status word in the lookup context to 5, then calls one routine and returns.
- `retry_position_lookup`: Splits a stored 16-bit value into a quotient and remainder by 22, adjusts the remainder and a side flag, then calls routines in a loop until a call reports success or a limit of five attempts is reached.
- `retry_nested_operation_limits`: Clears a result word, then calls a routine to run an operation and retries it while counting failures, with an outer loop that repeats the whole attempt up to five times and saves intermediate values between tries.
- `retry_operation_five_times`: Repeats a routine call up to five times per pass across five passes, saving the position on each failed attempt.
- `reset_counters_store_position`: Clears two retry counters and stores a position word built from two stored values, one of them multiplied by 128 and added to the other.
- `set_bit2_and_step_position`: Sets bit 2 of a byte at a hardware address when a flag is non-zero and clears it otherwise, then calls a routine.
- `clear_bit2_and_step_position`: Clears bit 2 of a byte at a hardware address, then calls a routine.
- `wait_until_position_reached`: Calls a routine, then loops until a position value equals a target, calling a routine on each pass where the position is not yet at the target.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `wait_for_port_bit_clear`: Calls a routine repeatedly while bit 4 of a byte at a hardware address stays set, then clears a counter word.
- `check_bit_and_clear_word_2`: Tests bit 4 of a byte at an offset of 2401 from a4; if set, it branches to another routine, otherwise it clears a 16-bit word at an offset of 1232 from a5 and returns.
- `advance_counter_until_limit`: Calls a routine and increments a counter word, repeating until the counter reaches 79.
- `compare_word_and_branch`: Compares a 16-bit word at an offset of 1232 from a5 with 79; if the value is less than 79, it branches to another routine, otherwise it returns.
- `clear_bit1_and_increment_counter`: Clears bit 1 of a byte at a hardware address, calls a routine, then increments a counter word.
- `set_bit1_and_decrement_counter`: Sets bit 1 of a byte at a hardware address, calls a routine, then decrements a counter word.
- `pulse_bit0_then_call_routine`: Clears and then sets bit 0 of a byte at a hardware address, producing a pulse, then calls one routine and returns.
- `wait_on_vertical_beam_position`: Busy-waits on the vertical beam position register with an empty loop body, reading that register twice per pass.
- `test_bit_and_restore_stack`: Pushes d0 to the stack, sets d0 to an address, and subtracts 1; if the result is zero, it branches to another routine, otherwise it tests bit 5 of a byte at an offset of 2401 from a4; if set, it branches to another routine, otherwise it restores d0 from the stack and returns.
- `set_status_four_then_exit`: Sets the status word in the lookup context to 4, then calls one routine and returns.
- `program_block_and_wait_ready`: Writes a sequence of control values into a register block at fixed offsets from a base pointer, then waits up to a fixed count for a ready bit in a status word.
- `write_control_word_bit14`: Writes a control word with only bit 14 set to a register at a fixed offset from a base pointer, then returns.
- `decode_sector_after_sync_word`: Scans a buffer for the standard Amiga disk sector sync word, then checks the sector header against expected values, and decodes 128 longs by combining odd and even bit fields under a mask.
- `find_matching_sector_and_decode`: Scans forward for the Amiga disk sync word and then for a sector header with the wanted number, checks two further header values, and decodes 128 longs under a mask into the output.
- `decode_sector_if_checks_match`: Compares two values from the header and, if they agree, decodes 128 longs from the buffer into the output by combining odd and even bit fields under a mask.
- `decode_128_long_sector_block`: Decodes 128 longs by merging each odd bit field with the adjacent even field under a mask, writes them to the output, clears a status counter and returns zero.
- `stub_return_failure_value`: Returns minus one immediately with no other work.
- `combine_and_shift_longwords`: Loads two longwords from a table pointed to by a0, applies a bitmask in d4 to both, shifts the first longword left by one bit, and returns the bitwise OR of the results.
- `compute_xor_checksum_run`: XORs together a run of longs starting at a pointer, for a count held in a register, and returns the result masked with a value from another register.
- `pulse_ciab_port_bit3_low`: Writes a value with every bit except the top one set to the CIA-B port B byte, calls a routine, clears bit 3, and calls another routine.
- `pulse_ciab_port_bit3_high`: Sets bits 7 and 3 of the CIA-B port B byte, calls a routine, clears bit 3, calls another routine, then sets bit 3 again.
- `copy_365_longs_to_buffer`: Copies 365 longs from a data block into a fixed low-memory buffer with a counted loop, then returns zero.

### Plotting (1990)(Ocean)[cr Angels][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### logged disk load from Plotting (1990)(Ocean)[cr Angels][a].adf

- `initialize_rendering`: Initializes hardware registers and memory locations for graphics rendering by setting up coprocessor pointers, clearing a display mode register, and writing coordinate values to a table.
- `frag_move_tst_bne_bsr_rts`: Short code fragment that copies d0 to the stack, then tests a fixed memory address, then branches if the last result was not zero, then calls another routine, then tests a fixed memory address, then branches if the last result was not zero, then calls another routine, then copies the top of the stack to d0, then returns.
- `update_table_values`: Decrements a counter in memory and branches to the end of the routine if it does not reach zero.
- `frag_subq_bne_move_moveq`: Short code fragment that subtracts 1 from a fixed memory address, then branches if the last result was not zero, then copies 1 to a fixed memory address, then loads 1 into d5.
- `process_bitmask_table`: Performs a nested loop to modify values in a table using bitwise operations and increments a global counter until it reaches a threshold.
- `frag_btst_ori`: Short code fragment that tests bit d0 of the memory just below a0 (moving a0 down), then merges 34 into d0, then merges 36 into d0, then merges 38 into d0, then merges 40 into d0, then merges 42 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### Plotting (1990)(Ocean)[cr Angels][a].adf:PRO.BIN (raw code file, load address unknown)

- `copy_1001_longs_and_reset_2`: Sets a bit in a global flag word, calls a routine, then copies 1001 longs from a data block into a fixed low-memory table, writes a constant to a global word twice and xors a shared word with a constant.
- `disable_all_interrupts_and_dma_2`: Writes a value with every bit except the top one set to the interrupt enable and DMA control registers, twice, then xors a shared word with a fixed constant.
- `verify_word_xor_checksum`: Reads a word from the stream and compares it with a stored word; if they match, it xors the next 3150 words together and checks the stored final word against the complement of that xor.
- `interleave_odd_even_bits`: Combines the low word of one value with the high word shifted left, using alternating-bit masks to place the bits into odd and even positions, and returns the combined pair.
- `divide_and_step_to_target`: Divides a 32-bit value by a 16-bit divisor, stores the remainder and a parity flag for the quotient, and updates a stored total.
- `check_bit_and_clear_word`: Tests bit 4 of a byte at an offset of 2401 from a pointer in a4; if set, it branches to another routine, otherwise it clears a 16-bit word at an offset of 1156 from a5 and returns.
- `clear_bit1_and_increment_counter_2`: Clears bit 1 of a byte at a hardware address, calls a routine, then increments a counter word.
- `set_bit1_and_decrement_counter_2`: Sets bit 1 of a byte at a hardware address, calls a routine, then decrements a counter word.
- `pulse_bit0_then_call_routine_2`: Clears and then sets bit 0 of a byte at a hardware address, producing a pulse, then calls one routine and returns.
- `busy_wait_on_memory_byte`: Empty busy-wait that reads a byte at a fixed offset from a base pointer and compares it with a shifted copy of itself.
- `program_registers_wait_ready`: Stores a few control and count values into a register block at fixed offsets, waits for a status bit to be set, writes a fixed value, then waits for a ready flag word before writing final values and returning.
- `wait_port_bit5_clear`: Busy-waits while bit 5 of a byte at a hardware address is set, then returns.
- `pulse_ciab_port_bit3_low_2`: Writes a value with every bit except the top one set to a byte at a hardware address, calls a routine, clears bit 3, and then calls two further routines.
- `modify_bits_and_jump_2`: Sets bits 7 and 3 of a byte at the address in a4, calls another routine, clears bit 3 of that byte, calls another routine, sets bit 3 of that byte, and jumps to the address in a5.
