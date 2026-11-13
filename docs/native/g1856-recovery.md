# Game 1856 (f29 retaliator (1989)(ocean)[cr][b]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01856). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903408 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 20 raw-word lines |
| Functions named and described | pass, 136 of 136 named (0 author symbols), 136 of 136 described (6 by rule, 99 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 157 of 157 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1536 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1757 of 1765 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 170783 |
| code | 7246 |
| data | 724848 |
| padding | 19 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| F29 Retaliator (1989)(Ocean)[cr][b].adf:bootblock | 1 | 81 | 3 |
| logged disk load from F29 Retaliator (1989)(Ocean)[cr][b].adf boot-block load | 1 | 604 | 49 |
| logged disk load from F29 Retaliator (1989)(Ocean)[cr][b].adf boot-block load | 1 | 678 | 61 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 379 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| F29 Retaliator (1989)(Ocean)[cr][b].adf bytes 899584-901120 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- f29 retaliator (1989)(ocean)[cr][b]/F29 Retaliator (1989)(Ocean)[cr][b].adf: custom-format (non-DOS) disk, 1661 non-empty blocks: matched against track-loaded segments
- reference run ref ends at a 68000 address error (odd PC an address after an address): the harness halted there, nothing after it ran (68000 HALTED: address error at odd PC an address after an address, vector 3 an address is not a handler in RAM (the 68000 double-faults and halts); run ended at frame 20)
- reference run ext ends at a 68000 address error (odd PC an address after an address): the harness halted there, nothing after it ran (68000 HALTED: address error at odd PC an address after an address, vector 3 an address is not a handler in RAM (the 68000 double-faults and halts); run ended at frame 20)
- f29 retaliator (1989)(ocean)[cr][b]/F29 Retaliator (1989)(Ocean)[cr][b].adf: the boot block reads 5632 bytes from disk offset 12288 to an address (found statically); recovered as a unit
- f29 retaliator (1989)(ocean)[cr][b]/F29 Retaliator (1989)(Ocean)[cr][b].adf: the boot block reads 204800 bytes from disk offset 18944 to an address (found statically); recovered as a unit
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 103 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- f29 retaliator (1989)(ocean)[cr][b]/F29 Retaliator (1989)(Ocean)[cr][b].adf: bytes 899584-901120 look like code no run loaded: recovered position-independently (load address unknown)
- f29 retaliator (1989)(ocean)[cr][b]/F29 Retaliator (1989)(Ocean)[cr][b].adf: 3 logged loads cover 211968 bytes of the image
- f29 retaliator (1989)(ocean)[cr][b]/F29 Retaliator (1989)(Ocean)[cr][b].adf: blocks 35-36 (bytes 17920-18944) outside the filesystem hold data, not code (opcode density 0.014, entropy 4.47)
- f29 retaliator (1989)(ocean)[cr][b]/F29 Retaliator (1989)(Ocean)[cr][b].adf: blocks 437-1756 (bytes 223744-899584) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.99)
- 1 never-loaded region(s), 1536 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 10752 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `clear_bit7_then_set_port` (F29 Retaliator (1989)(Ocean)[cr][b].adf boot-block load +an address to an address (5632 bytes)): Clears bit seven of a control byte, sets bits three through six, then clears one more bit chosen by the low bits of a data register plus three. Kind: bit-manipulation-based checksum or flag check. Pass path: the branch or state where DAT_00bfd100 retains expected bit patterns.
- `set_port_bits_clear_one` (F29 Retaliator (1989)(Ocean)[cr][b].adf boot-block load +an address to an address (5632 bytes)): Sets bits three through six of a control byte, then clears one bit chosen by the low bits of a data register plus three. Kind: bitmask-based check/trap. Pass path: The routine modifies a bit in a control/status word (an address) or a flag, likely used to validate a specific disk pattern or bit-sequence presence..
- `set_top_bit_then_call` (F29 Retaliator (1989)(Ocean)[cr][b].adf boot-block load +an address to an address (5632 bytes)): Sets bit seven of a control byte, calls a subroutine, then sets bits three through six in the same byte. Kind: sabotage path/loader protection. Pass path: sub_0596() executes with bit-set state.
- `step_head_to_target_track` (F29 Retaliator (1989)(Ocean)[cr][b].adf boot-block load +an address to an address (5632 bytes)): Compares a target value with a drive-control byte and steps the byte by two toward the target, calling a subroutine after each step, until the two are equal or a bit condition fails. Kind: track/head position check. Pass path: the loop terminates when the current head position (represented by _DAT_00001206) matches the target track/head position (in_D0w), effectively bypassing the need for head movement or extra steps..
- `spin_in_infinite_loop_3` (F29 Retaliator (1989)(Ocean)[cr][b].adf boot-block load +an address to an address (5632 bytes)): Spins forever in an empty loop with no other operations, so it never returns. Kind: disk hardware check (CIAB_PRB). Pass path: the loop terminates or exits when the condition or hardware-dependent counter (e.g., an address.l) matches d0, effectively bypassing the infinite hang/loop if the disk check or checksum validates or if the loop is a way to wait for a specific ha.
- `process_bit_flags` (F29 Retaliator (1989)(Ocean)[cr][b].adf boot-block load +an address to an address (5632 bytes)): Tests bits in a register to set bits in a global variable, writes to a hardware register, and if certain conditions are met, writes 56319 to the disk length register twice. Kind: disk hardware/track check. Pass path: returns bVar3 (the shifted bitmask) and continues execution normally.

## Functions


### F29 Retaliator (1989)(Ocean)[cr][b].adf:bootblock

- `load_blocks_and_submit_io`: Copies several word blocks from a global area into memory, fills in two I/O request blocks and submits each through an exec library call, then writes a copper list of register and value pairs.
- `setup_copper_list_io`: Copies word blocks from a global area into memory, writes a copper list of register and value pairs with an end marker, and submits I/O requests through an exec library call.
- `empty_routine`: the routine contains no executable instructions.

### logged disk load from F29 Retaliator (1989)(Ocean)[cr][b].adf boot-block load

- `read_boot_block_chunks`: Repeatedly calls a routine with a count pair and advances an address while the low count is above 5632, then calls one more routine.
- `read_boot_block_chunks_2`: Repeatedly calls a routine with a count pair and advances an address while the low count is above 5632, then calls one more routine.
- `call_subroutine_then_stop`: Makes one subroutine call, then the decoded body ends in bad instruction data, so no further behaviour is recovered.
- `copy_run_then_check_limit`: Copies a run of bytes from a source pointer into a fixed memory block, calls a subroutine, then loops forever if a count plus one exceeds 159.
- `spin_in_infinite_loop`: Spins forever in an empty loop, so execution never leaves it.
- `stub_bad_instruction_data`: The decoded routine is reduced to bad instruction data, so its 87 instructions are not recovered and no behaviour can be named.
- `expand_bit_flags_then_fill`: Expands bits from a 32-bit register into a table of flag bytes, then writes output bytes from a source.
- `copy_memory_block`: This routine copies a fixed number of bytes from a source pointer to a destination pointer and increments a global variable.
- `copy_block_and_advance_counter`: Calls a subroutine, copies 128 longwords from a source pointer to a fixed buffer, and adds 512 to a global counter before returning.
- `spin_in_infinite_loop_2`: Spins forever in an empty loop, so execution never leaves it.
- `loop_color_value`: Writes 3840 to the first custom color register and then enters an infinite loop.
- `loop_indefinitely`: This routine performs an infinite loop and modifies a color register.
- `sum_longwords_into_checksum`: Sums 256 longwords starting from a fixed address and returns the total, as a simple checksum.
- `clear_bit7_then_set_port`: Clears bit seven of a control byte, sets bits three through six, then clears one more bit chosen by the low bits of a data register plus three.
- `set_port_bits_clear_one`: Sets bits three through six of a control byte, then clears one bit chosen by the low bits of a data register plus three.
- `set_top_bit_then_call`: Sets bit seven of a control byte, calls a subroutine, then sets bits three through six in the same byte.
- `write_control_values_then_call`: Writes two control values in sequence to a shared byte, calls a subroutine, and returns the data register unchanged.
- `step_head_to_target_track`: Compares a target value with a drive-control byte and steps the byte by two toward the target, calling a subroutine after each step, until the two are equal or a bit condition fails.
- `spin_in_infinite_loop_3`: Spins forever in an empty loop with no other operations, so it never returns.
- `run_control_sequence_until_clear`: Writes control values to a shared byte, waits until a status bit clears, then loops over a counter with subroutine calls.
- `start_disk_dma_transfer`: Waits for a status bit to clear, writes the disk sync word and disk control values, stores the sync word through an address register, then waits for the disk interrupt flag.
- `scan_sync_sector_headers`: Scans a track buffer for the disk sync word, skips runs of repeated sync words, and checks each header's two decoded values against stored values.
- `decode_mfm_long_pairs`: Decodes MFM data by combining pairs of longwords, masking alternate bits and shifting one half of each pair left by one, then writes the results to an output buffer.
- `start_blitter_transfer`: Waits for the blitter to be idle, sets its mask, modulo, pointer, and control registers, then starts a transfer sized from a width and a height.
- `xor_fold_longwords_masked`: XORs a run of longwords into one accumulator, then masks the high longword of the return value with alternate bits.
- `tiny_move_rts`: Small routine of 2 instructions that copies d2 to d1, then returns.
- `check_interrupt_status`: Reads the interrupt request register, checks specific bits, and if neither bit is set, writes a value to the interrupt mask register and returns from an interrupt.
- `set_interrupt_request_two`: Sets bit 0 of a global variable, writes 2 to the interrupt request register, and returns from an interrupt.
- `set_interrupt_request_four`: Writes 4 to the interrupt request register and returns from an interrupt.
- `save_and_interrupt`: Saves d0 to the stack, loads a longword from a specific address into d0, calls another routine, writes 8 to the interrupt request register, restores d0 from the stack, and returns from an interrupt.
- `set_interrupt_request_eight`: Writes 8 to the interrupt request register and returns from an interrupt.
- `check_interrupt_status_3`: Saves d0 to the stack, loads a longword from the interrupt status register into d0, and if bit 4 or bit 5 is set, branches to another routine; otherwise it sets bit 1 of a global variable, writes 64 to the interrupt mask register, restores d0 from the stack, and returns from an interrupt.
- `enable_interrupt_and_return`: Calls another routine, writes a value to the interrupt mask register, and returns from an interrupt.
- `disable_interrupts_and_return`: Sets the status register to disable interrupts, calls another routine, writes 16 to the interrupt mask register, and returns from an interrupt.
- `update_interrupt_mask`: Reads the interrupt request register, masks the value with 1920, writes the result to the interrupt enable register, and returns from an interrupt.
- `check_interrupt_status_2`: Pushes d0 to the stack, reads a long value from a hardware register into d0, and if bit 12 is set, writes 2048 to the interrupt mask register before returning from an interrupt; otherwise it returns.
- `set_interrupt_request_4096`: Writes 4096 to the interrupt request register and returns from an interrupt.
- `process_interrupt_flags`: Pushes d0 to the stack, reads a long value from a hardware register into d0, and if bit 14 is not set, performs a series of operations including calling another routine, writing 8192 to the interrupt mask register, and returning from an interrupt; otherwise it returns.
- `set_interrupt_request_8192`: Writes 8192 to the interrupt request register and returns from an interrupt.
- `set_interrupt_request_16384`: Writes 16384 to the interrupt request register and returns from an interrupt.
- `update_keyboard_bitmask`: Shifts d0 right by two, calls another routine if bit 1 is set, and if bit 1 is set after a second shift, it reads a byte from the keyboard serial register, performs bitwise operations, and clears or sets a bit in a table at offset 7654.
- `update_bit_table_and_counters`: Uses a byte from a table to set a bit in a bitmask table, updates a character table and an index in a global variable, and sets CIAA and CIAB control registers.
- `process_bit_flags`: Tests bits in a register to set bits in a global variable, writes to a hardware register, and if certain conditions are met, writes 56319 to the disk length register twice.
- `call_stored_function`: Loads a function pointer from a global variable, increments that global variable, calls the function, and returns.
- `populate_table_from_value`: Uses a loop to write four 16-bit words derived from a global variable into a table starting at a specific memory location, updates a global variable, and calls another routine.
- `decrement_and_call`: If a 16-bit global variable is non-zero, it decrements it, increments another global variable, and calls a routine via a pointer in a third global variable; otherwise it returns.
- `loop_until_nonzero`: Sets d0 to the value in a4, performs a bitwise OR on d0, calls another routine, and loops if the result is zero.
- `fetch_indexed_byte`: Compares two 16-bit values from a global variable, and if they are not equal, fetches a byte from a table using the second value as an index, updates that index in a global variable, and returns the byte in d0.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.

### logged disk load from F29 Retaliator (1989)(Ocean)[cr][b].adf boot-block load

- `loop_until_zero`: Calls another routine in a loop until that routine returns a zero value.
- `wait_for_byte_and_clear`: Loops until a byte in a global variable is non-zero and positive, then clears a different global byte and returns.
- `advance_random_and_divide`: Advances a 32-bit linear congruential random state and divides its high word by a divisor passed in d1, returning the quotient and the remainder.
- `advance_random_and_swap_halves`: Advances the shared random state and returns the value with its two 16-bit halves swapped.
- `execute_sequence_logic`: Writes values to several global variables, calls a routine three times with different parameters, performs a loop that merges data from a source structure into a destination structure, and then executes a series of conditional routine calls and pointer updates.
- `execute_sequence_and_wait`: Calls two different routines and then enters a loop that waits for a global variable to be combined with a constant to become non-zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_global_state`: Tests a 16-bit value at a global address and branches to another routine if zero; if a longword at a global address is negative and within a specific range, it adds a constant to it, moves several longwords between global addresses, performs arithmetic on d0, clears three 16-bit global variables, and if a byte at a global address is 69, sets a global longword to 31 before returning.
- `test_global_values`: Loads a 16-bit value from a global address into d1; if d1 is zero, it branches to another routine; if d1 is 2, it tests a 16-bit value at a global address and branches to another routine if it is negative; otherwise it branches to another routine.
- `set_global_flags`: Tests a 16-bit value at a global address, and if it is negative, writes 5 to one global variable and 1 to another; otherwise it returns.
- `set_global_word`: Sets a 16-bit global variable to one and returns.
- `decrement_and_update_list`: Clears a longword at a PC-relative address, initializes a loop counter to 4, and enters a loop that decrements a 16-bit value at a table offset; if the value is non-negative, it loads a pointer from a table, adds three longwords from that pointer to offsets 14, 18, and 22 of a structure in a1, and calls another routine before looping.
- `compare_value_and_dispatch`: Calls another routine, compares a 32-bit global variable against 500 and 2000, and if the value is between these limits, it loads 17 into d0, calls another routine, and branches to another routine; otherwise it branches to another routine.
- `check_status_and_dispatch`: Loads 18 into d0 and calls another routine, then checks a 32-bit global variable; if the value is zero or negative, it branches to another routine, otherwise it checks another 32-bit global variable, and if that is non-zero, it loads 21 into d0, calls another routine, and branches to another routine.
- `load_status_command_nineteen`: Loads 20 into d0, calls another routine, and then branches to another routine.
- `dispatch_status_command`: Checks a 16-bit global variable; if zero, it calls another routine, otherwise it loads 19 into d0 and calls another routine.
- `test_status_and_trigger`: Checks a 16-bit global variable; if zero, it branches to another routine, otherwise it loads 24 into d0 and calls another routine.
- `process_status_logic`: Loads 22 into d0 and calls another routine, then checks a 16-bit global variable; if zero, it branches to another routine, otherwise if negative it branches to another routine, otherwise it loads 26 into d0 and calls another routine.
- `check_status_and_call`: Checks a 16-bit global variable; if non-zero, it branches to another routine, otherwise it loads 27 into d0 and calls another routine.
- `evaluate_status_range`: Loads 25 into d0 and calls another routine, then checks a 16-bit global variable; if zero, it branches to another routine, otherwise if negative it branches to another routine, otherwise it loads 29 into d0 and calls another routine.
- `execute_conditional_status`: Checks a 16-bit global variable; if zero, it branches to another routine, otherwise it loads 30 into d0 and calls another routine.
- `handle_status_branching`: Loads 28 into d0 and calls another routine, then checks a 16-bit global variable; if zero, it branches to another routine, otherwise if negative it branches to another routine, otherwise it loads 32 into d0 and calls another routine.
- `process_status_update`: Checks a 16-bit global variable; if non-zero, it calls another routine with 33 in d0.
- `trigger_event_sequence`: Loads 31 into d0 and calls another routine, then checks a 16-bit global variable; if non-negative, it checks another 16-bit global variable and calls another routine with 42 in d0 if that value is non-zero, otherwise it calls another routine with 42 in d0.
- `update_status_and_trigger_event`: Calls a routine with the value 43 in register d0, then checks a 16-bit global variable; if the value is non-negative, it checks a bit in another global variable and performs conditional logic based on whether the value falls within specific ranges, potentially setting a bit and calling the same routine with the value 2 in register d0.
- `store_byte_and_call_routine`: Stores a byte into a global variable, loads the value three into a data register, calls another routine, and then branches to another location.
- `update_dynamic_table_entry`: Clears a bit in a global variable, calls another routine, and compares a 16-bit value from a global variable against a threshold.
- `process_offset_table_entry`: Calculates an address by adding an offset to a pointer from a global variable, calls another routine, and if a 16-bit global variable is less than 200, calls another routine with the value 37 in d0; otherwise, it branches to a subsequent instruction.
- `decrement_global_loop_alt`: Loads 36 into d0 and calls another routine, then checks a 16-bit global variable; if non-zero and the first global variable is non-negative, it checks a second global variable and a third global variable to decide whether to call another routine, decrement the first global variable, and loop if the result is non-zero.
- `set_d0_and_call_branch_2`: Loads 41 into d0, calls another routine, and branches to another routine.
- `decrement_global_loop`: Stores 20 into a global variable, loads 46 into d0 and calls another routine, then checks a 16-bit global variable; if non-zero and the first global variable is non-negative, it checks a second global variable and a third global variable to decide whether to call another routine, decrement the first global variable, and loop if the result is non-zero.
- `set_d0_and_call_branch`: Loads 39 into d0, calls another routine, and branches to another routine.
- `decrement_and_update`: Stores 20 into a global variable, loads 47 into d0 and calls another routine, then checks a 16-bit global variable; if non-zero it decrements a 16-bit global variable, potentially setting it to 65535 if it becomes negative, and calls another routine based on a 16-bit global variable check, finally calling another routine and branching.
- `set_d0_and_branch`: Loads 51 into d0, calls another routine, and branches to another routine.
- `set_d0_and_call`: Loads 52 into d0, calls another routine, tests a 16-bit global variable, and if non-zero loads 48 into d0 and calls another routine before branching.
- `update_state_logic`: Loads 49 into d0, calls another routine, then checks a 16-bit global variable; if it is non-negative and its lowest bit is set, it checks if the value is between 260 and 360 to set a global variable to 3 and call another routine, or if it is between 261 and 360 to set a global variable to 1 and call another routine; otherwise it sets the global variable to 3, loads 6 into d0, calls another routine, and returns.
- `store_and_call`: Stores a byte into a global variable, clears a 16-bit value in another global variable, loads 7 into d0, calls another routine, and branches to another routine.
- `clear_and_call`: Clears a 16-bit value in a global variable, clears a 16-bit value in another global variable, loads 4 into d0, calls another routine, and branches to another routine.
- `process_scaled_values`: Sets a global variable to 1, stores a value, calls another routine, and then performs calculations involving division and multiple calls to another routine based on a value from a global variable.
- `call_offset_pointer_routine`: Loads a long value from a global variable, adds 3280 to it, and calls the routine at that address.
- `loop_bit_test`: Performs a bitwise operation using a value from a global variable and a bitmask, then enters a loop that checks bits of a value and writes 255 to a memory location if a bit is set, decrementing a counter until it reaches zero.
- `store_shifted_byte`: Copies d1 to d2, doubles d2, and stores a byte from d2 offset into the memory location pointed to by a0 before branching back.
- `execute_conditional_jump`: Calculates an index from a 16-bit global variable, loads a word from a table at offset 214 of a structure in a2, compares d0 with 8, and calls a routine at an address in a3 based on the result, finally storing d7 into a global variable.
- `test_structure_offset`: Tests a 16-bit value at offset 38 of the structure in a1 and branches to another routine if the result is positive; otherwise it returns.
- `compare_status_values`: Loads a pointer from offset 44 of the structure in a1 into a3, and returns if the 16-bit value at offset 30 of the structure in a3 is equal to either 82 or 83; otherwise it returns.
- `compare_and_branch`: Compares a 16-bit value at offset 34 of the structure in a1 to d6 and branches if it is greater than d6, otherwise it returns.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies a1 to d7, then returns.
- `initialize_and_swap_pointers`: Clears three global variables, calls a routine, and then enters a loop that copies a 16-bit value from a table into a structure and updates a field in that structure if it is zero, before calling another routine twice with different counts from global variables.
- `calculate_table_indices`: Calculates three new addresses by adding a global variable to the product of a 16-bit value from d0 or d1 and 30, then stores these in three global variables.
- `swap_pointers_in_loop`: Decrements d7 and enters a loop that calls another routine, then swaps the long values pointed to by a1 and a2, repeating until d7 reaches negative one.
- `swap_pointers_in_loop_alt`: Decrements d7 and enters a loop that calls another routine, then swaps the long values pointed to by a1 and a2, repeating until d7 reaches negative one using a different initial d0 value.
- `fill_memory_with_pointer`: Calls another routine, then stores the value in a1 into four consecutive longwords at a memory location pointed to by a0, where the final value is a1 plus 200.
- `update_linked_list_pointer`: Moves two zero values into a structure at offset 54 of a1, loads a longword from a2 into d0, and if d0 is not zero, moves the value in a1 into the location pointed to by a2; otherwise it returns.
- `traverse_and_update_linked_list`: Enters a loop that compares 16-bit values at offset 14 of two structures, updates a pointer at offset 94 of a structure using a value from a1, and continues until a pointer becomes zero.
- `process_linked_list_with_callback_2`: Calculates four 16-bit values based on d0, d1, and d2, stores them in a table, and then enters a loop that checks if 16-bit values at specific offsets of a structure are within a range, calling another routine if a condition is met.
- `process_linked_list_with_callback`: Enters a loop that checks if a 16-bit value at offset 14 of a structure is within a specific range, calls another routine if a condition is met, and calls a function pointer in a3 if a value at offset 22 is within a specific range.
- `clear_and_copy_structure_fields`: Clears a 16-bit word at offset 106 and a longword at offset 112 of the structure in a1, then copies three longwords from offsets 14, 18, and 22 to offsets 116, 120, and 124.
- `set_structure_pointer`: Stores the constant value a fixed address into a structure at offset 102 of a1 and calls another routine.
- `clamp_structure_value`: Compares a 16-bit value at offset 18 of the structure in a1 to 65486 and, if the value is greater than or equal to 65486, sets it to 65486.
- `assign_structure_pointer_and_call`: Stores the address of a global variable into a structure at offset 102 relative to a1, then calls another routine and returns.
- `assign_pointer_and_call_subroutine`: Stores a pointer to a global variable into a structure at offset 102 of a1, then calls another routine and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_pointer_and_call_subroutine`: Increments a 16-bit value pointed to by a6, calls a subroutine, and then either moves a pointer from a stack offset into a0 or loads a value from a stack offset into a0 based on the status flags.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_interrupt_enable_sequence`: Writes interrupt-enable values to the interrupt register, one with the set-bit flag and several source bits, and calls a subroutine between two of the writes.
- `enable_saved_interrupt_mask`: Clears a stored counter, then writes a saved interrupt-enable value with the set-bit flag to the interrupt register and calls a subroutine.
- `set_color_zero`: Sets the color zero register to a constant value and returns.
- `update_audio_channel_counters`: Advances three counters, inverts one value and resets its period to ten when a countdown goes negative, and sets a flag when another counter reaches zero, then calls a subroutine.
- `set_global_and_spin`: Sets a global word to a value with its high byte filled with ones, then busy-waits until that word becomes zero.
- `write_saved_interrupt_values`: Writes two saved values to a shared register, then writes a third saved value with the set-bit flag, and returns.
- `perform_blitter_transfers`: Calculates blitter parameters based on a pointer from a global variable, sets several blitter registers including destination path, mode, and size, and then enters a loop that performs multiple blitter transfers by updating the destination path and size registers.
- `perform_blitter_block_transfer`: Sets blitter destination path, height, and mode registers, then enters a loop that updates the destination path, height, and size registers for a set number of iterations while calling another routine.
- `wait_for_blitter_2`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `setup_blitter_loop`: Calls another routine, sets two blitter control registers, calculates a value based on a register and a constant to set the blitter mode register, calls another routine, and then enters a loop that sets the blitter destination path, source path, and size registers while decrementing a counter.
- `execute_blitter_fill_2`: Sets d0, d1, and d4 to specific constants, loads a pointer from a global variable into a2, and jumps to another routine.
- `execute_blitter_fill`: Sets blitter data, mask, and control registers, then enters a loop that updates the destination path and size registers for a set number of iterations while calling another routine.
- `test_and_jump`: Tests a 16-bit global variable, calls another routine if the value is non-zero, then calls a second routine and jumps to another routine.
- `call_and_test_word`: Calls another routine, tests a 16-bit value at a global address, and returns if the value is zero; otherwise it branches to another routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `fetch_table_value`: Loads a pointer from a global variable, uses a byte from another global variable shifted left by two to index into a table, loads the resulting longword into d1, and calls another routine; if the result is zero, it sets a global word to one and returns.
- `clear_global_word`: Clears a 16-bit word in a global variable and returns.
- `loop_color_toggle`: Enters an infinite loop that repeatedly writes two different values to the color zero register.
- `update_list_of_values`: Compares d0 to a value in a table, and if d0 is greater than that value, subtracts the value from d0, updates the table with the new d0, and then updates the table again with a new value derived from d0 and a pointer from the table.
- `read_joystick_position_clamped`: Reads joystick port movement, converts the low bits into X and Y deltas, and clamps the resulting position to a 320 by 200 area.
- `read_mouse_movement_counters`: Reads mouse port movement and button state, updates horizontal and vertical counters clamped to a 320 by 200 range, and writes a button flag to a shared byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `spin_in_infinite_loop_4`: Spins forever in an empty loop, so execution never returns from this routine.
