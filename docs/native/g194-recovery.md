# Game 194 (MicroMachines v1.1 0357): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00194). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1791570 top-level bytes in 41 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 18 of 18 units identical (bytes and relocations), 154 raw-word lines |
| Functions named and described | pass, 244 of 244 named (0 author symbols), 244 of 244 described (6 by rule, 237 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1313 of 1313 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 11644 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3706 of 3708 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 9241 |
| code | 12476 |
| data | 1769848 |
| padding | 5 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 310 | 22 |
| FRT | 1 | 2891 | 176 |
| GAM | 1 | 0 | 1 |
| LV1#unpacked | 1 | 18 | 1 |
| LV2#unpacked | 1 | 18 | 1 |
| LV3#unpacked | 1 | 21 | 1 |
| LV4#unpacked | 1 | 22 | 1 |
| LV5#unpacked | 1 | 21 | 1 |
| LV6#unpacked | 1 | 19 | 1 |
| LV7#unpacked | 1 | 20 | 1 |
| LV8#unpacked | 1 | 22 | 1 |
| LV9#unpacked | 1 | 19 | 1 |
| MID | 1 | 0 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 312 | 27 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 6 |
| MD2#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- MicroMachines_v1.1_0357/MicroMachines_v1.1_0357.hdf: hard-disk image (AmigaDOS OFS, 1952 blocks): its filesystem parses and all 21 files in it are byte-identical to the extracted tree MicroMachines_v1.1_0357_hdf, which is recovered file by file; no block outside the filesystem holds data
- MicroMachines_v1.1_0357/MicroMachines_v1.1_0357_hdf/MicroMachines/data/ICN: IMP: File Imploder-packed, unpacked with ancient (36620 -> 61440 bytes)
- MicroMachines_v1.1_0357/MicroMachines_v1.1_0357_hdf/MicroMachines/data/LV1: IMP: File Imploder-packed, unpacked with ancient (86532 -> 103120 bytes)
- MicroMachines_v1.1_0357/MicroMachines_v1.1_0357_hdf/MicroMachines/data/LV2: IMP: File Imploder-packed, unpacked with ancient (60366 -> 77124 bytes)
- MicroMachines_v1.1_0357/MicroMachines_v1.1_0357_hdf/MicroMachines/data/LV3: IMP: File Imploder-packed, unpacked with ancient (53222 -> 65448 bytes)
- MicroMachines_v1.1_0357/MicroMachines_v1.1_0357_hdf/MicroMachines/data/LV4: IMP: File Imploder-packed, unpacked with ancient (76090 -> 88296 bytes)
- MicroMachines_v1.1_0357/MicroMachines_v1.1_0357_hdf/MicroMachines/data/LV5: IMP: File Imploder-packed, unpacked with ancient (39336 -> 50772 bytes)
- MicroMachines_v1.1_0357/MicroMachines_v1.1_0357_hdf/MicroMachines/data/LV6: IMP: File Imploder-packed, unpacked with ancient (65154 -> 75932 bytes)
- MicroMachines_v1.1_0357/MicroMachines_v1.1_0357_hdf/MicroMachines/data/LV7: IMP: File Imploder-packed, unpacked with ancient (43110 -> 54964 bytes)
- MicroMachines_v1.1_0357/MicroMachines_v1.1_0357_hdf/MicroMachines/data/LV8: IMP: File Imploder-packed, unpacked with ancient (49834 -> 62796 bytes)
- MicroMachines_v1.1_0357/MicroMachines_v1.1_0357_hdf/MicroMachines/data/LV9: IMP: File Imploder-packed, unpacked with ancient (67428 -> 80716 bytes)
- MicroMachines_v1.1_0357/MicroMachines_v1.1_0357_hdf/MicroMachines/data/MD0: IMP: File Imploder-packed, unpacked with ancient (15388 -> 20596 bytes)
- MicroMachines_v1.1_0357/MicroMachines_v1.1_0357_hdf/MicroMachines/data/MD1: IMP: File Imploder-packed, unpacked with ancient (18190 -> 22606 bytes)
- MicroMachines_v1.1_0357/MicroMachines_v1.1_0357_hdf/MicroMachines/data/MD2: IMP: File Imploder-packed, unpacked with ancient (8160 -> 11644 bytes)
- MicroMachines_v1.1_0357/MicroMachines_v1.1_0357_hdf/MicroMachines/data/MD3: IMP: File Imploder-packed, unpacked with ancient (15856 -> 19400 bytes)
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 60 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- MicroMachines_v1.1_0357/MicroMachines_v1.1_0357_hdf/MicroMachines/data/MD2#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 11644 bytes, recorded as UNVERIFIED DATA (0 bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `initialize_hardware_and_tables_b`: Stores a pointer in a global variable and calls a function through an offset in its table.
- `conditionally_clear_byte_and_dispatch`: Conditionally clears a byte at a fixed offset from an address register when a high-word test does not match, writes a fixed constant to a global variable, stores a byte argument into another global variable and compares it against that variable, and calls a routine through a pointer from a global table.
- `store_constant_and_dispatch_table_entry`: Writes a fixed 32-bit constant to a global variable, stores a byte from a data register into another global variable and compares it back, then calls a routine through a pointer loaded from a global table.
- `conditionally_clear_byte_offset`: Clears a byte at a fixed offset from an address register, but only when the high-word test of that address does not match a particular value.
- `tiny_clr_rts`: Small routine of 2 instructions that clears (a1), then returns.
- `store_constant_to_global`: Writes a fixed 32-bit constant into a global variable and returns.
- `store_byte_to_global`: Copies a byte from a data register into a global variable and then compares it back against the same variable, returning in both branches of the comparison.
- `store_byte_then_return`: Stores a byte passed in a data register into a global variable, then compares it against that variable and returns without any further effect.
- `store_byte_and_compare`: Stores a byte passed in a data register into a global variable, then compares it against that same variable and returns.
- `store_byte_then_return_2`: Stores a byte passed in a data register into a global variable and then returns.
- `compare_byte_and_return`: Compares a byte passed in a data register against a global variable and then returns; the comparison result has no effect because both branches simply return.
- `empty_return_stub`: Contains no operations and only returns immediately.
- `install_patched_routine_entries`: Calls a routine from a global table to obtain a value, clamps it to a maximum of a fixed address, and passes it to another table entry.
- `store_constant_to_hardware_global`: Copies a global variable onto itself, then writes a fixed 16-bit constant into a global variable, touching the vertical position hardware register in the process.
- `install_patched_code_and_dispatch`: Calls an indirect table entry to obtain a value, clamps it to a maximum, passes it to another indirect table call, and conditionally writes several fixed instruction words and an address-based jump into global memory before returning the saved registers.
- `store_parameters_to_frame_offsets`: Stores two parameters into fixed offsets of an address-register-based structure and also copies a stack value into a global variable.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `store_routine_address_to_global`: Stores the address of another routine into a global variable and returns.
- `store_constant_to_global_2`: Stores a fixed 32-bit constant into a global variable and returns.
- `call_indirect_through_table_entry`: Loads a pointer from a global table, then calls a routine at a fixed offset within it, and returns the data register unchanged.
- `return_immediately`: This routine is empty: it consists only of a return instruction and performs no work.

### FRT

- `halt_on_bad_data`: Stops execution immediately because the decompiler found undecodable instruction bytes at this point, so the routine performs no useful work and is effectively a trap for invalid code.
- `clear_globals_and_call_setup`: Clears two global variables and a block of 648 longword pairs of global storage, calls several other routines, and calls a routine from a table indexed by a word from a data register.
- `run_sequenced_callback_dispatch`: Calls a long sequence of other routines, clears ten words of memory through a pointer held in a global variable, and loops calling routines until a returned value is zero.
- `copy_tables_and_store_globals`: Calls several other routines in sequence, stores two register values into global variables, and calls four more routines.
- `dispatch_two_callbacks`: Clears a global variable, stores two address-register values into globals, calls the routine addressed by one of them, then clears the global and calls through it.
- `poll_flags_until_terminal_bit`: Loops waiting for a global variable to become non-zero, calls another routine on each pass, and returns 1 when the variable equals a fixed value or 0 when a particular bit is set.
- `wait_flag_then_set_sentinel`: Waits for a global variable to become non-zero, calls another routine, and returns early if a bit is set in that variable; otherwise it writes 255 to a global variable.
- `wait_then_set_flag`: Loops until a global variable is set, optionally returning early when a bit mask on another global variable is clear and that variable is non-zero.
- `poll_then_store_ff_2`: Waits for a global variable to become non-zero, calls another routine, and returns early when a bit is set; otherwise it writes 255 to a global variable.
- `poll_then_store_ff`: Repeatedly checks a global variable, returning early on a bit-test condition, otherwise waiting for it to become non-zero before calling another routine.
- `wait_then_set_flag_2`: Loops until a global variable becomes non-zero, calls another routine, and exits the loop if the variable is zero; otherwise it sets the global variable to 255 and returns.
- `wait_range_then_store`: Spins in an infinite loop if a global variable is non-zero and above a fixed threshold, otherwise waits for it to become non-zero, calls another routine, and stores 255 into a global variable.
- `poll_masked_flag_store_2`: Waits for a global variable to become non-zero, calls another routine on each pass, and writes 255 to a global variable when a bit-test exit condition is met.
- `poll_masked_flag_store`: Waits on a global variable with a bit-test exit condition, calling another routine on each pass, and writes 255 to a global variable when the loop completes.
- `wait_then_set_flag_variant`: Loops until a global variable becomes non-zero, calls another routine, and exits if the variable is zero; otherwise it sets a global variable to 255 and returns.
- `spin_until_zero_store`: Spins forever when a global variable is non-zero, otherwise waits for it to become non-zero, calls another routine, and then stores 255 into a global variable.
- `wait_then_run_sequence`: Repeatedly waits while a global variable is zero, then runs another routine that performs a six-step sequence and sets the global variable to 255 once it changes.
- `spin_nonzero_then_store`: Enters an infinite loop when a global variable is non-zero, otherwise waits for it to change, calls another routine, and stores 255 into a global variable.
- `poll_bit_then_store_ff`: Waits for a global variable to become non-zero, calls another routine, and returns early if a particular bit is set; otherwise it writes 255 to a global variable.
- `poll_bit_then_store`: Returns early when a bit is set in a global variable, otherwise waits for that variable to become non-zero, calls another routine, and writes 255 to a global variable.
- `wait_until_flag_equals_at_sign`: Busy-waits until a global variable becomes nonzero, calls another routine, and repeats until the global variable equals the ASCII code for an at sign.
- `advance_counter_and_dispatch`: Calls several other routines, increments a global counter, and resets it to zero once it exceeds a fixed limit.
- `select_mode_and_dispatch`: Calls a sequence of other routines, then sets a global variable to 1 if bit 2 is set and to 2 if bit 3 is set.
- `dispatch_five_calls_conditionally`: Calls another routine five times in sequence, then conditionally overwrites a global variable with 1 if its bit 2 is set and with 2 if its bit 3 is set, then calls another routine twice more before returning.
- `invoke_seven_sequential_routines`: Calls the same other routine seven times in succession and then returns.
- `dispatch_six_calls_conditionally`: Calls another routine six times, then conditionally overwrites a global variable with 1 if its bit 2 is set and with 2 if its bit 3 is set, then calls another routine twice more before returning.
- `invoke_six_then_clamp_flag`: Calls another routine six times, then if a global variable equals 150 it sets that variable to 255, and finally calls one more routine before returning.
- `dispatch_conditional_routine_calls`: Calls another routine twice, and between the calls it checks a global variable against one specific value and, when it matches, replaces that variable with a different value.
- `invoke_six_routines_sequentially`: Calls another routine six times in sequence and returns, with no conditional logic or direct data access.
- `store_digit_char_and_dispatch`: Calls other routines, then stores the low byte of a global variable plus the ASCII code for zero into a fixed memory location, and finishes with a further sequence of calls.
- `advance_two_bit_counters`: Calls several other routines and then, for two global variables each holding a value compared to 1 or 2, resets it to zero and advances it modulo four, skipping a value that would equal the other variable.
- `call_routine_six_times_2`: Calls another routine seven times in sequence and then returns.
- `call_routine_six_times`: Calls another routine six times in sequence and returns without any other operation.
- `initialize_grid_and_wait_flags`: Stores a word from a register into a global variable, calls several routines, clears global variables, and loops waiting on a global flag before returning.
- `dispatch_copy_and_string_calls`: Calls other routines, branching on the low word returned by its first call to decide whether to make an extra call, and returns after a final call.
- `forward_single_string_measure_call`: Makes a single call to another routine and returns.
- `dispatch_four_list_walk_calls`: Makes four calls to other routines in sequence and returns, performing no data handling of its own.
- `conditionally_call_emit_routine`: Makes one call, then calls another routine only if a global variable is non-zero, followed by a final call before returning.
- `conditional_call_return_default`: Calls three other routines, then returns a fixed default value unless a global signed word is non-negative, in which case it returns the result of another call.
- `call_if_word_nonnegative`: Reads a word through an address register and calls another routine only when that word is non-negative, otherwise returning immediately.
- `invoke_four_update_routines`: Calls four other routines in sequence and returns.
- `update_counters_and_clamp_value`: Calls two routines unconditionally and calls two more routines when a global word is non-negative.
- `dispatch_four_nested_calls`: Calls another routine four times in succession and then returns, performing no other work of its own.
- `increment_word_capped_conditionally`: If the word pointed to by one address register is non-negative, it increments the word pointed to by another address register, clamping it at a maximum of 150 so it never reaches 151.
- `store_word_then_forward`: Stores a word from a data register through an address register pointer, then makes a single call to another routine and returns.
- `forward_call_to_another_routine`: Makes a single call to another routine and then returns to its caller, passing no data of its own.
- `forward_call_to_routine`: Makes a single call to another routine and returns.
- `increment_word_capped_alt`: Performs the same conditional increment as another routine: if one pointed-to word is non-negative, it increments the word at the other pointer, capping it at 150.
- `write_word_then_chain`: Writes a word from a data register through an address register pointer, then calls another routine and returns.
- `forward_to_another_routine_2`: Makes a single call to another routine and then returns, performing no other operations.
- `forward_call_to_routine_alt`: Makes a single call to another routine and returns, performing no other operations.
- `set_global_from_flags`: Tests bits 2 and 3 of the incoming data register and, for each set bit, stores a fixed constant into a global variable (65532 for bit 2, 4 for bit 3).
- `compute_shifted_index_wrapped`: Shifts a global variable right by six bits and subtracts ten from the result if it exceeds ten, returning the reduced value.
- `advance_and_wrap_index`: Adds a global variable to itself, wraps the result into a fixed range by adding or subtracting a constant, and keeps the low six bits as the return value.
- `wrap_index_and_lookup`: Adds a fixed offset to a negative 16-bit input, masks the low six bits into a global variable, and when that value is zero calls another routine, doubles its result, and clears the global variable if the table entry it indexes is zero.
- `update_global_state_from_flags`: Calls a helper, then if a global variable is not two and a table entry indexed by it is negative, calls helpers and, when a flag bit and a zero global are set, claims a slot in a table and stores a computed value into a global variable.
- `claim_table_slot_if_negative`: When a global variable is negative, calls helpers and, if a flag bit and a zero global are set, marks a free table slot as used and stores a value derived from its index into a global variable.
- `advance_state_after_slot_claim`: Unless a global variable equals two, calls helpers and, when a flag bit and a zero global are set and a table slot is free, marks the slot used, stores a derived value into a global variable, and advances a state global by one, resetting it to four if it no longer equals two.
- `claim_slot_and_set_state_four`: Unless a global variable equals three, calls helpers and, when a flag bit and a zero global are set and a table slot is free, marks the slot used, stores a derived value into a global variable, sets another global to four, and then advances a state global by one.
- `step_global_state_machine`: Advances a state global through a small sequence of values, applying flag-driven updates to a table entry and calling helpers at each step.
- `set_global_to_ff`: Stores the constant 255 into a global variable and returns.
- `store_negative_two_after_calls`: Calls four other routines in sequence, stores the 32-bit value minus two through an address register pointer, then calls one more routine and returns.
- `run_loops_then_store_constant`: Calls another routine twice, stores a fixed 32-bit constant through an address register, and then calls another routine once more.
- `call_four_routines_in_sequence`: Calls four other routines one after another and then returns.
- `call_three_routines_sequentially`: Calls three other routines in sequence and returns.
- `call_six_routines_then_store`: Calls six other routines in sequence, stores the 32-bit value minus two through an address register pointer, then calls one more routine and returns.
- `call_routines_then_store_constant`: Calls five other routines, stores a fixed 32-bit constant through an address register, and then calls one more routine before returning.
- `iterate_two_nested_ranges`: Runs two nested counted loops over index ranges held in registers, calling another routine twice per iteration of each loop.
- `set_bits_in_column_table`: Walks four rows of a table through a pointer computed from a global variable and two register arguments, setting or clearing one bit per row according to successive bits of a shift register value.
- `clear_two_longword_buffers`: Zeroes two buffers of longwords, alternating writes between them, over a fixed count of iterations, using pointers loaded from a global variable.
- `wait_then_store_longword`: Loops while a hardware-referenced word's low-order bits are below a threshold, then copies a longword from a pointer held in a global variable into a fixed offset of an address register-based structure and clears a global variable.
- `copy_table_block_and_words`: Calls another routine, copies a block of longwords from one fixed address range to another in a counted loop, copies a block of words in a second counted loop, and then calls another routine.
- `fill_word_grid_with_decrement`: Clears a fixed word in memory, then writes a sequence of words into a block of memory, each value decreasing by a constant step, and finally calls another routine.
- `fill_grid_with_constant`: Calls a routine, then writes a fixed 16-bit value through an address register in a 17-by-32 nested loop, calling another routine at each row boundary.
- `fill_grid_then_store_constant`: Writes a fixed 16-bit value through an address register in a 16-by-32 nested loop, calling another routine at each row boundary and once more at the end.
- `call_routine_then_loop_forever`: Calls another routine once and then enters an infinite loop that never returns.
- `test_value_against_constants`: Reads a 16-bit value passed in a data register and compares it with zero and with a second constant, but the comparison result is discarded and the routine returns without storing or changing any data.
- `clear_global_and_trap`: Sets a global variable to zero, executes a trap instruction with a fixed vector number, and then calls another routine before returning.
- `infinite_idle_loop`: Spins forever in an empty loop with no other work.
- `forward_to_another_routine_alt`: Calls another routine once and returns, performing no other operations.
- `call_two_routines_sequentially`: Calls two other routines in sequence and returns.
- `clear_flag_then_copy`: Sets a global byte to zero and then calls a copy routine.
- `forward_to_copy_routine`: Makes a single call to another routine and returns without any other work.
- `clear_global_words_after_call`: Calls a copy routine, then writes four words of all-ones followed by eight words of zero into global variables.
- `clear_globals_and_call_sequence`: Zeroes two global variables, makes three calls to other routines, then zeroes the same two global variables again before returning.
- `clear_table_and_iterate_ranges`: Zeroes a block of longwords starting at a fixed address, then calls other routines in a fixed sequence and runs two nested-style counted loops that call another routine with a stepped index and a fixed address-derived value.
- `process_pointer_table_entries`: Loops ten times over a table of words pointed to by a global variable, branching on each word's value (zero, one, or other) and calling a routine per branch.
- `forward_to_another_routine`: Makes a single call to another routine and returns without any other work.
- `fill_hardware_block_fields`: Waits until a status bit in a hardware-referenced structure clears, then writes fixed constants and computed values into several fields of that structure, including a value derived from a register argument.
- `wait_then_start_blit`: Busy-waits until the blitter-busy flag clears, then writes a fixed control value, a constant word, and a source pointer taken from a register to the blitter registers and triggers the operation.
- `blit_two_passes_offset`: Scales a word argument from a data register by eight and adds a constant to form an address offset, then waits for the blitter to be idle and starts two successive blitter operations using two different constant offsets from that base.
- `initialize_structure_and_set_bits`: Calls a copy routine to initialise a structure and clears a longword through an address register, then calls another routine that sets the high bit of two fields.
- `initialise_structure_and_copy`: Calls a copy routine, clears a longword through an address register, then copies 72 longwords from a fixed source table into the following structure fields and clears two further longwords and a trailing longword.
- `set_high_bit_in_two_fields`: Reads a longword at a fixed negative offset from an address register, sets its bit 7, and writes the result back there and to a second longword at a fixed positive offset.
- `pack_four_byte_fields`: Computes a sum from two word arguments and stores it as a byte through a pointer, then writes a half-value of an adjusted third argument and a derived sum as further bytes, with a fourth byte built from carry-like bit tests of the sums.
- `write_fixed_four_words`: Loads a pointer from a global variable at a fixed offset and stores four constant 32-bit words through it in sequence.
- `write_fixed_four_words_alt`: Loads a pointer from a global variable at a fixed offset and stores four different constant 32-bit words through it in sequence.
- `store_negative_two_longs`: Loads a pointer from a global variable at a fixed offset and writes the same constant 32-bit value to the location it points to and to the location it points to at a fixed offset from it.
- `store_sentinel_through_pointer`: Loads a pointer from a global variable's structure and writes the longword value a fixed address through it and through a second stored pointer, then calls two other routines.
- `write_fixed_words_and_computed_field`: Writes a series of fixed 32-bit and 16-bit constants through a pointer loaded from a global variable, computes a value from another global variable modulo 704 and stores it in a half-word field, then writes further constants through the address register and calls other routines.
- `start_blit_from_table`: Reads a 32-bit pointer from a table indexed by a scaled word argument, waits for the blitter to be idle, then loads blitter registers with constants, a computed size from another argument, and a source address derived from the table pointer plus an offset from a second argument, and starts the operation.
- `start_blit_table_offsets`: Reads a pointer from a table indexed by a scaled word argument, waits for the blitter to be idle, then computes a size from two word arguments and a constant, and starts a blitter operation with a source address from that pointer plus an offset derived from another argument.
- `start_blit_indexed_table_entry`: Reads a pointer from a table indexed by a scaled word argument, waits for the blitter to be idle, then computes a destination address from the quotient and remainder of an argument divided by four combined with a constant, and starts a blitter operation with a source address from that pointer plus an offset from another argument.
- `start_blit_mod_four_offset`: Reads a pointer from a table indexed by a scaled word argument, waits for the blitter to be idle, then computes a destination address from the low two bits of a second argument scaled by a constant, and starts a blitter operation with a source address from that pointer plus an offset from the first argument.
- `call_routine_per_string_byte`: Walks a null-terminated byte string through an address register, calling another routine for each nonzero byte and advancing the pointer until the terminator.
- `measure_then_emit_string`: Optionally redirects its source pointer to a fixed address depending on a bit of a global variable, scans to the string terminator, then calls another routine for each byte from the start of the string until the terminator is reached.
- `measure_then_call_string`: Computes the length of a null-terminated string by scanning to its terminator, then calls another routine once for each character until a null byte is reached, updating the pointer from the value returned in the address register each time.
- `call_routine_per_string_byte_2`: Walks a null-terminated byte string, calling another routine once for each byte until the terminating zero byte is reached.
- `invoke_routine_per_string_char`: Calls a routine and then walks a null-terminated string, invoking a routine held at a fixed code address once per character until the terminating null byte.
- `walk_null_terminated_list`: Repeatedly calls another routine while the byte at the current pointer is non-zero, advancing the pointer each time it returns, and returns when a zero byte is reached.
- `call_routine_per_string_char`: Walks a null-terminated string, invoking a routine held in an address register once per character until the terminating null byte is reached, advancing the pointer from the address register after each call.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_bitplane_block_fifteen_rows`: Copies a block of bytes from a source table into five interleaved destination positions spaced 40 bytes apart, repeating across sixteen rows with a stride of 80 bytes in the source.
- `copy_bitplane_block_eight_rows`: Copies a block of bytes from a source table into five interleaved destination positions spaced 40 bytes apart, repeating across eight rows with a stride of 80 bytes in the source.
- `copy_interleaved_block_eight_rows`: Copies a block of bytes from a source table into five destination positions spaced 176 bytes apart, repeating across eight rows and advancing the destination by 880 bytes per row.
- `copy_longwords_with_count`: Reads a count derived from a field of a source structure, then copies that many longwords from a source pointer to a destination pointer.
- `setup_blitter_fill_request`: Waits until a busy flag clears, then writes fixed values into a hardware-style control block, setting a mask, a size, a modulus and a source address derived from a global pointer.
- `setup_blitter_copy_request`: Waits until a busy flag clears, then writes fixed values into a hardware-style control block, setting a mask, a size, a modulus and a source address derived from a different global pointer offset.
- `zero_globals_and_call_routines`: Clears two global variables to zero and then calls five other routines in sequence.
- `fill_table_with_stepped_values`: Calls another routine, then writes 208 consecutive longword entries into a fixed table, each one 200 greater than the previous value starting from a constant.
- `fill_longword_table_stepped`: Writes a running 32-bit value into consecutive longword entries of a table, adding 200 to the value after each write, for 208 entries.
- `build_display_list_structures`: Writes a series of hardware-style register and value longword and word pairs into a buffer, stores pointers to several sub-sections into a structure, and fills thirty-two word pairs with incrementing values followed by an end marker.
- `build_copper_style_list`: Writes a series of hardware-style register/value word pairs into a buffer through a pointer, stores pointers to several sub-sections in a structure, and fills a run of 32 word pairs with incrementing values before terminating the list with two end markers.
- `poll_and_reset_serial_control`: Stores two values into global variables, polls a status register until a particular bit is set, then writes control words and several global variables, returning the polled status value.
- `write_control_word_register`: Writes a fixed 16-bit value into a control word at a fixed offset from an address register.
- `store_word_at_offset`: Stores the 16-bit constant 256 into a word at a fixed offset of the address register A6 and returns.
- `fill_words_with_increments`: Loops a fixed count taken from a register, writing pairs of 16-bit words through the pointer in A0 for each iteration: a high and low half of a 32-bit value from D0, plus a running 16-bit value from D7 that advances by 2 and 4 per iteration.
- `read_serial_byte_and_store`: Reads a global variable and, if bit 3 is set, clears it and waits until bit 3 is set again before reading the keyboard serial register through the CIA. It then shifts and inverts the received bits into a global variable and stores a flag byte into a table indexed by that value, finally setting a global variable to 8 and acknowledging an interrupt request.
- `increment_global_and_dispatch`: Increments a global byte variable twice, calls another routine, conditionally calls a function through a code-pointer table if the variable is non-zero, then sets the variable to a fixed constant and returns two register values.
- `dispatch_through_function_table`: Calls two routines through a table of code pointers indexed by a global variable, each time masking bit 4 out of the returned byte when that bit is set.
- `call_routine_twice_wrapper`: Calls another routine and returns, performing no other operations of its own.
- `call_bit_and_mask_routines`: Calls one routine and then calls another routine that builds a bitmask, returning after the second call.
- `read_port_bit_into_mask`: Calls a routine to obtain a value, clears bit four of it, and sets bit four again when a bit of a global variable is clear.
- `build_bitmask_from_pairs`: Reads four byte pairs from a global table, and for each pair whose second byte's bit is set in a working register, sets the bit given by the first byte in an accumulated result, which it returns.
- `build_bitmask_from_table`: Iterates four times over byte pairs read through the pointer in A0, and for each pair whose second byte is non-zero in a table, sets the bit numbered by the first byte in a 32-bit accumulator, which it returns in D0.
- `empty_routine_placeholder`: This routine is empty: it contains no operations beyond returning to its caller.
- `call_indexed_routine_then_clear`: Calls a routine, loads a longword from a global table indexed by a word from a data register and passes it to that routine, calls another routine, then writes a byte of all-ones to a global variable.
- `decompress_lz_stream`: Decompresses a stream whose header begins with the magic value 'IMP!' by copying literal bytes and back-references read backwards from the end of the packed data into the output buffer, using a bit-by-bit shift register to decode the control codes.
- `init_track_segment_tables`: Walks a 32-entry table of 8-byte records read from the input block, filling a parallel array of per-entry state with relocated pointers and bit-masked fields, then fixes up a second pointer table by adding the block base to each non-zero offset (once, when a magic check matches).
- `advance_audio_playback_state`: Processes a set of global state variables, conditionally calling helpers per bit of a status word, and when a counter wraps it reloads a pointer into a table and stores a byte index into a global variable.
- `clear_audio_channel_state`: Clears a global byte and writes the values zero and fifteen into several global variables, then returns.
- `clear_word_and_globals`: Writes zero to a memory location through an absolute address, then stores the constant 15 and then zero into a global variable four times in sequence.
- `reset_audio_channel_volumes`: Clears a byte at a fixed location and writes zero and fifteen to several global variables, leaving the audio-related state set to those values.
- `advance_sequence_state_machine`: Clears a set of object fields, then either counts down a pending delay and returns, or reads the next byte-encoded command from a stream: a high-bit byte sets a repeat count, otherwise it decodes parameters and dispatches on the low nibble of the second byte through a switch, updating object fields such as velocity, timers, indices and table-derived values.
- `store_nibbles_and_pointer_fields`: When a word argument is nonzero, stores its low and high nibbles into two fields of a structure and sets two other fields to fixed values; it then stores a six-bit value from a byte pointer and, unless that value is the terminator, calls another routine, finally writing a pointer computed from a table word.
- `store_negated_word_and_pointer`: Stores the negation of a word argument into a field of a structure, then stores a six-bit value from a byte pointer, calls another routine unless it is the terminator, and writes a pointer computed from a table word.
- `store_word_and_pointer`: Stores a word argument unchanged into a field of a structure, then stores a six-bit value from a byte pointer, calls another routine unless it is the terminator, and writes a pointer computed from a table word.
- `store_indexed_table_value_and_sign`: Reads a six-bit value from a byte pointer; if it equals the terminator it sets a sign-adjusted field from the argument, otherwise it stores the value into two fields, looks up a table entry by it, and optionally adjusts the sign of the argument before writing a pointer computed from a table word.
- `store_lookup_nibbles_and_pointer`: Sets a field to a fixed value and, when the word argument is nonzero, stores a table entry indexed by its low nibble and the high nibble into fields; then stores a six-bit value from a byte pointer, calls another routine unless it is the terminator, and writes a pointer computed from a table word.
- `decode_object_fields_from_byte`: Stores a sentinel value into one word field of the object, then splits an input word into a 4-bit field and a flag word, storing those into the object.
- `set_field_split_nibble_store`: Sets a word field at a different offset of the object in an address register to all ones, then splits a data register value into a nibble field and a flag field.
- `store_default_offset_and_pointer`: Writes the constant two to a word in a global-addressed table, clears a word field of the object in an address register, and stores a masked byte from a pointer.
- `store_masked_byte_and_pointer`: Reads a byte through a pointer, stores its low six bits as a word into a field of the object in an address register, and when those bits are not all ones calls another routine.
- `split_register_into_nibble_fields`: Splits a word from a data register into a low nibble or a high nibble with a flag word, storing the results into fields of the object in an address register.
- `store_sentinel_and_read_byte`: Stores a sentinel word and an argument word into fields of a structure addressed by a register, increments a counter field, then reads a byte from a stream, masks its low six bits into a field and, unless the masked value is 63, calls another routine.
- `clamp_and_store_limited_value`: Clamps a word argument to a maximum of 64 and stores it into a field of a structure, then reads a byte from a stream, stores its low six bits into a field, and calls another routine unless that value is 63.
- `store_sentinel_and_read_stream`: Stores a sentinel word into a field of a structure, then reads a byte from a stream and stores its low six bits into a field, calling another routine unless the value is 63.
- `dispatch_through_indexed_jump_table`: Takes a word argument, shifts it right by four, scales the result to a longword offset and makes an indirect call through a table of routine addresses, then returns.
- `clamp_minimum_and_store_field`: Raises a word argument to a minimum of two, unless it is zero in which case the field store is skipped, and stores the result into a field of a structure.
- `initialize_record_fields_conditionally`: When a word argument is nonzero, it stores the low nibble and high nibble of that value into two fields of a structure and sets two other fields to fixed values.
- `initialize_fields_then_store_pointer`: Sets one word field of the object in an address register to all ones, then splits a data register value into a nibble field and a flag field.
- `store_pointer_from_offset_table`: Reads a byte through a pointer, stores its low six bits as a word into a field of an object, and when those bits are not all ones, calls another routine before storing a pointer into the object's first field, computed from the input pointer plus a word offset taken from a second structure.
- `initialize_fields_and_read_stream`: Stores a sentinel value and an argument word into fields of a structure, increments a counter field, then reads a byte from a stream and stores its low six bits into a structure field.
- `set_flag_and_decode_record`: Sets or clears bit 1 of a global variable depending on whether an argument word is zero.
- `store_negated_word_and_reset_field`: Stores the negation of a word argument into a field of a structure, sets another field to a fixed value, then stores a six-bit value from a byte pointer, calls another routine unless it is the terminator, and writes a pointer computed from a table word.
- `store_negated_word_and_initialize_2`: Stores a word argument into one field of a structure when it is nonzero, then extracts the low six bits of a byte from a stream into another field and recurses until that value equals 63.
- `initialize_record_fields_with_offset`: Stores a word argument into a field of a structure, sets another field to 1 and a third to 65535.
- `set_flag_bit_and_read_stream`: Sets or clears bit 1 of a global variable depending on whether an argument word is zero, then reads a byte from a stream into a structure field, recursing until its low six bits equal 63.
- `set_flag_and_read_byte`: Sets bit 1 of a global variable, then reads a byte from a stream and stores its low six bits into a structure field.
- `store_negated_word_and_initialize`: Stores the negation of a word argument into a field of a structure and sets another field to 65535.
- `init_object_fields_from_stream`: Stores a word from a register into a field of the object pointed to by an address register, sets two further fields to fixed values, and reads a byte from a stream pointer to set a 6-bit count field.
- `copy_record_fields_to_table`: Stores a word from the D0 register into a field of the object pointed to by the A0 register, then loads a word from a table indexed by D0 into another field.
- `copy_audio_channel_registers`: Saves a word field from a structure and clears it, stores it in a global variable, and loops calling another routine until a value returned in D1 is zero.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `update_object_motion_state`: Advances a per-object state record each frame: applies a velocity to a position field with clamping to a limit, adjusts a second value toward a bound, steps a cyclic index and selects a position from a table, computes a derived offset from a global table and a divisor, and writes a clamped result and a sum into a linked display record.
- `advance_counter_and_reset`: Decrements a word counter at a fixed offset from the base register when it is zero, first reloading it from a neighbouring word; once a second counter reaches 65 it clears four global variables and two words in the structure.

### GAM

- `decode_entry_data_failure`: The decompiler could not decode the instructions at this location, so the routine body is unrecoverable and it contains no meaningful operations.

### LV1#unpacked

- `clear_bit_and_toggle_bits`: Clears one bit in a byte through a pointer and toggles single bits in two other memory locations, using register-derived bit positions.

### LV2#unpacked

- `stores_longword_and_sets_bits`: Writes the four bytes of a longword to a structure at a fixed offset, sets bits in two bytes selected through pointers, toggles a bit in a third byte, adds a constant to another byte, then performs a bounds check that raises a trap if the value is negative or exceeds a stored limit.

### LV3#unpacked

- `mask_and_toggle_words`: Sets the low twelve bits of a 16-bit word addressed by one pointer register, then XORs a 32-bit value stored at a fixed offset from a second pointer register with a constant mask.

### LV4#unpacked

- `clear_bit_and_toggle_bit`: Clears one bit in a byte addressed through one pointer, toggles a bit in a byte addressed through another pointer, and stores a masked value into a longword through a third pointer, then adds a constant to a longword at the toggled byte's address.

### LV5#unpacked

- `toggle_and_clear_bits`: Toggles one bit in a byte addressed through a pointer, clears a bit in another byte, masks a 16-bit word through a pointer, clears the global variable's bits with a mask, and adds a constant to a byte before a pointer.

### LV6#unpacked

- `copy_word_at_offset`: Reads a 16-bit word from a fixed offset relative to the address register and writes the same value back to that location, leaving memory unchanged.

### LV7#unpacked

- `toggle_bit_and_clear_bit`: XORs a word in memory with a constant, toggles one bit of a byte at a computed pointer offset chosen by a register, and clears one bit of a byte pointed to by another address register.

### LV8#unpacked

- `xor_word_at_offset`: XORs a 32-bit word at a fixed offset from an address register with a constant, storing the result back in place.

### LV9#unpacked

- `halt_on_bad_data_2`: Stops execution immediately when the decoder encounters bytes that do not form a valid instruction, so control flow is truncated at this point.

### MID

- `decode_invalid_instruction_stream`: The decompiler could not decode the instruction bytes at this location, so the routine body is unrecoverable and it performs no observable data operations in the recovered output.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `single_instruction_stub`: A one-instruction routine whose decompilation timed out, so its exact operation is not recovered from the available evidence.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_tables`: Sets a global signature word and an adjacent word, calls other routines to perform setup, and writes a set of global variables.
- `initialize_tables_and_dispatch`: Checks a global signature word and traps and sets it if it differs, then clears a block of 10 twelve-byte records and a range of words, sets several global variables, and calls a helper that returns an index used to jump through a table of routine pointers.
- `tag_global_record_if_unset`: Calls another routine, then if a global pointer is non-null and its first longword does not equal a fixed four-byte tag, writes the tag, zeroes one field and points another field at a location 256 bytes past the record.
- `copy_record_words_by_key`: Extracts a four-byte key from a source location, then searches a list of three-word entries for a match.
- `store_record_and_copy_words`: Stores three input values and a pointer into a record, then copies a word count's worth of words from a source pointer into a destination buffer, provided the new end address stays within a limit held in a global variable.
- `copy_words_to_destination`: Copies a word count taken from a structure into a sequence of words, reading from a source pointer stored in the same structure and writing to the destination address register, one word at a time.
- `return_without_action`: The routine consists of a single instruction and performs no work of its own, so it only returns control to its caller.
- `initialize_hardware_and_copy_block`: Writes fixed values to a global variable and to a hardware port register, saves two address-register values into globals, and calls several routines.
- `set_clear_byte_flags`: Computes a byte address from an uninitialized address register plus a constant offset, sets four bits in that byte and clears two others, leaving the remaining bits unchanged.
- `set_clear_byte_bits`: Computes a byte address from an input address register plus a constant offset, sets two bits in that byte, clears one bit and sets it again, leaving the other bits unchanged.
- `unnamed_single_instruction_stub`: The routine is a single-instruction stub reached only by address-taken reference and one unnamed caller, and the decompiler output is unavailable.
- `tail_call_to_bit_setter`: Contains only a single call to another routine that computes a byte address from an address register plus an offset and then sets, clears and sets bits in that byte, followed by an immediate return.
- `store_constant_and_call_routine`: Writes a fixed 32-bit constant to a location at a negative offset from a pointer loaded from a structure, then calls another routine.
- `divide_index_and_compare`: Divides a value in a data register by eleven, compares the quotient's low word against a word in a structure pointed to by an address register, and if they differ, calls another routine with a value combining the remainder and quotient before calling a second routine with no arguments.
- `adjust_bits_and_wait_until_match`: Sets bit 2 of a byte at one offset from a base pointer and updates bits 0 and 1 of that byte in a loop, calling another routine to compare a value against a stored word until they match.
- `poll_and_loop_bits`: Sets and clears bits in a byte through a pointer in a loop, calling another routine to compare a value against a stacked word and repeating until the values match or fall below the threshold.
- `store_word_and_handle_mismatch`: Stores a word from a data register into a structure at a fixed offset, then compares it against a stacked word.
- `set_bit_write_constants_wait`: Sets bit 3 in a byte at one offset from a base register, writes the constant 2 and the constants 84 and 64 to two other offsets, busy-waits until bit 1 of a byte at another offset is set, then clears bit 0 of the first byte.
- `return_immediately_stub`: An empty routine consisting of a single return instruction that does nothing.
- `sets_flags_waits_clears_bit`: Sets bit 3 in a byte at offset 3840 from a base register, writes the value 2 to a byte at offset 3328, and writes 100 and 8 to bytes at offsets 1536 and 1792.
- `sets_bits_waits_then_clears_word`: Sets bit 1 of a byte at a fixed offset from one address register, then repeatedly toggles bit 0 of that byte while a global variable has bit 4 set, calling another routine on each pass.
- `call_routine_and_clear_word`: Calls another routine, then stores a zero word into a structure field at a fixed offset from an address register.
- `configures_disk_sync_transfer`: Writes a set of 16-bit control words to hardware registers offset from a base register, including a sync word and a DMA-related value, and stores a 32-bit pointer from a second base-relative location into a register-offset slot.
- `decode_sector_with_retries`: Sets a retry counter, then repeatedly scans words for a sync marker and matches a header against a key value.
- `decode_sector_checksum_loop`: Scans a sequence of words for a sync marker and matches a header against a value from a counter-driven loop.
- `decode_sector_into_buffers`: Scans a sequence of 16-bit words for a sync marker, matches a header whose key bits equal a value from a counter-driven loop, then de-interleaves the odd and even bit fields of the following data into an output buffer while XOR-accumulating a checksum.
- `return_immediately_no_op`: The routine is a single instruction that does nothing beyond returning to its caller, performing no data reads, writes, or calls.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_2`: This routine is empty and performs no operations before returning to the caller.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `nop_instruction_stub`: This routine is empty and performs no operations.
- `return_immediately_3`: The routine is empty and performs no operations before returning.
- `return_no_op`: The routine consists of a single instruction that returns immediately, performing no work.
- `nop_operation`: The routine is empty and performs no operations.
