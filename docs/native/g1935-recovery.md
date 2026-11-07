# Game 1935 (jetstrike (1993)(rasputin)[cr outlaws][h nomad]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01935). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906768 top-level bytes in 182 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 17 of 17 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 585 of 585 named (59 author symbols), 585 of 585 described (26 by rule, 109 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1474 of 1474 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10796 of 10796 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 182681 |
| code | 34490 |
| data | 594425 |
| padding | 47556 |
| unused | 47616 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| Mount | 2 | 117 | 2 |
| crunch | 2 | 267 | 8 |
| OUTLAWS.EXE | 2 | 267 | 8 |
| Port-Handler | 1 | 306 | 1 |
| null-handler | 3 | 203 | 10 |
| Ram-Handler | 2 | 10 | 1 |
| powerpacker.library | 2 | 788 | 51 |
| mathtrans.library | 17 | 830 | 97 |
| mathieeedoubbas.library | 3 | 632 | 60 |
| req.library | 5 | 4828 | 295 |
| Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:Jetstrike | 9 | 782 | 22 |
| Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:bootblock | 1 | 195 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 819 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 43 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 569 | 10 |

## Cases handled

- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:Jetlogo.aaf: high-entropy data file (7.72 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:jeticons.aaf: high-entropy data file (7.76 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:jetsound.aaf: high-entropy data file (7.88 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:Engine3.smp: high-entropy data file (7.79 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:Engine1.smp: high-entropy data file (7.90 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:Miscon.aaf: high-entropy data file (7.80 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:iceicons.aaf: high-entropy data file (7.69 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:Misc: high-entropy data file (7.75 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:Rockicons.aaf: high-entropy data file (7.73 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:Jetsoundext.aaf: high-entropy data file (7.84 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:training.aaf: high-entropy data file (7.76 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:Rockjet.aaf: high-entropy data file (7.74 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:Seaicons.aaf: high-entropy data file (7.77 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:aerolim.aaf: high-entropy data file (7.72 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:combatcombi.aaf: high-entropy data file (7.78 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:Icepic.aaf: high-entropy data file (7.69 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:display.aaf: high-entropy data file (7.68 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:Carrier.aaf: high-entropy data file (7.74 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:rockmap2.map.mp: high-entropy data file (7.73 bits/byte), classified as packed data
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:jetmap4.map.MP: high-entropy data file (7.75 bits/byte), classified as packed data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 495 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 43 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 541 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- jetstrike (1993)(rasputin)[cr outlaws][h nomad]/Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:c/crunch: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_long_block_then_call`: Copies sixteen longwords from a fixed offset of the base register into a caller-supplied buffer when that buffer pointer is non-null, then calls through an indirect code pointer.

### Mount

- `find_max_and_fill_table`: Walks a linked list to find the largest value in a field, then writes that maximum as a header and fills the following longwords with a stepping pattern.
- `copy_entries_then_call`: Copies a fixed number of four-byte entries from a source table into a destination buffer when the destination is not null, then jumps through a stored address.

### crunch

- `parse_hunk_relocations`: Parses an AmigaDOS hunk stream, comparing each record type word with the hunk codes and applying relocation entries against the loaded section bases.
- `count_down_word_register`: Counts a 16-bit register down until it wraps past zero, so the number of iterations equals the starting value.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_backward_lz_block`: Decodes a bit-packed compressed stream into a buffer filled from the end toward the start, emitting literal bytes or copying earlier output at a distance read from the stream.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `read_bits_from_backward_stream`: Loops over a count held in a register, pulling one bit per pass from a bit buffer that is refilled by reading longwords backward from a source pointer.
- `stub_undecompiled_short_routine`: A four-instruction routine whose decompilation timed out, so only its call relationships are known.

### OUTLAWS.EXE

- `decompress_lz_stream`: Processes a bit-packed compressed stream, decoding literal bytes and back-references into a buffer while tracking positions.
- `decrement_register_counter`: It decrements a counter held in a register until the counter reaches zero, acting as a simple delay or loop.
- `stub_empty_return`: Consists of a single return instruction that does nothing.
- `copy_backward_output_block`: It reads a length from a buffer, and if non-zero it copies that many earlier output bytes backward into the current output position while refilling its bit stream as needed.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `read_bit_count_backward`: Pulls a count of bits from a bit buffer that is refilled by reading longwords backward from a source pointer.
- `read_bits_backward_stream`: It pulls one bit per pass from a bit buffer that is refilled by reading longwords backward from a source, accumulating them into an output register.
- `invoke_trap_store_byte`: It invokes a system trap, stores an incoming byte to a memory location, and then falls through into trailing data.

### Port-Handler

- `load_hunk_records_into_table`: Walks a stream of hunk records and dispatches on each record type to store code, data, bss and symbol sections into a per-section table.

### null-handler

- `clear_handler_block_then_call`: Zeroes a block of nineteen longwords relative to the base register and stores a global pointer into it.
- `forward_block_fields_to_callee`: Reads two longword fields at offsets 12 and 16 of its first argument block and passes them, along with the original two arguments, to another routine.
- `link_handler_request_node`: Stores two arguments in a request block, links the block into a list through a pointer at offset 10 of the first element, and clears its two link words.
- `fetch_node_link_field`: Calls a routine on a structure 92 bytes into the argument block, calls it again on the same location, and returns the longword at offset 10 of the second result.
- `open_dos_and_walk_records`: Opens the DOS library through exec, raising an alert if that fails, then walks a list of records and dispatches on each record type code until a counter returns to zero.
- `stub_empty_return_2`: Consists of a single return instruction that does nothing.
- `call_library_function_via_base`: Loads a pointer from a global slot and calls a library function at a fixed negative offset from it.
- `dispatch_via_jump_table`: It indexes into a jump table stored in a library and dispatches to the selected routine.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `dispatch_via_jump_table_slot`: It indexes into a different slot of a library jump table and dispatches to the selected routine.

### Ram-Handler

- `copy_memory_then_dispatch`: Calls a library routine through the global base, then jumps through a jump table that could not be recovered.

### powerpacker.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `save_state_and_init_library`: As part of the PowerPacker shared library, saves a pointer argument into the state block at offset 36 and records the input in a global.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `release_state_when_count_zero`: As part of the PowerPacker shared library, decrements a 16-bit counter in its state block.
- `complete_or_mark_pending`: As part of the PowerPacker shared library, when the counter is zero it calls library routines to finish and returns the value saved at offset 36.
- `finish_and_return_saved_value`: As part of the PowerPacker shared library, reads the value saved at offset 36 of its argument, makes three library calls to release resources, and returns that value.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `forward_single_call_and_return`: As part of the PowerPacker shared library, calls one routine and returns, with no other processing of registers or memory.
- `unpack_backward_lz_block_2`: As part of the PowerPacker shared library, decodes a compressed stream into a buffer selected from a table by an index, filling it from the end toward the start.
- `return_immediately_2`: Executes three instructions and then returns without modifying any data or registers.
- `return_immediately`: It executes three instructions and then returns without modifying any data or registers.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `shift_bits_from_backward_stream`: Shifts a counted number of bits out of a bit buffer, refilling it by reading longwords backward from a source pointer whenever it empties.
- `refill_bit_buffer_backward`: Repeatedly loads longwords backward from a source pointer and shifts their bits into a bit buffer until a counter runs out.
- `copy_backward_match_run`: Decodes back-references in a buffer that fills from the end toward the start, reading a length and distance through helper routines.
- `copy_backward_match_store_pointer`: Copies a run of earlier output into a buffer that fills from the end toward the start.
- `fold_bytes_into_checksum_word`: Folds a zero-terminated byte string into a 16-bit value, rotating the running value by an amount taken from each byte before adding the next one.
- `compute_byte_string_checksum`: Walks a zero-terminated byte string, combining each byte with a rotated 32-bit running value on every pass.
- `xor_block_with_key`: XORs a run of longwords in place with a key held in a register.
- `clear_first_byte_then_call`: Writes a zero byte to the location held in the address register, then calls another routine.
- `call_routine_then_return`: It calls another routine and then returns.
- `setup_stack_frame`: This routine disables task switching, reads fields from an input buffer into a stack structure, and then re-enables task switching.
- `relocate_hunk_stream_sections`: Walks a stream of AmigaDOS hunk records until the end record, applying relocation records and handling bss records.
- `process_hunk_type_codes`: Processes a sequence of AmigaDOS hunk type codes, matching executable and relocation markers.
- `scan_hunk_type_codes`: Walks a zero-terminated sequence of AmigaDOS hunk type codes, skipping over each hunk's data by following its size field, and records the count of each hunk type encountered.
- `adjust_hunk_section_addresses_2`: Walks through a hunk data stream and rewrites the relocation entries so that each stored address is shifted by the loaded base.
- `adjust_hunk_section_addresses`: Walks through AmigaDOS hunk records and adjusts the addresses stored in each section table to account for the relocated load base.
- `record_hunk_section_addresses_2`: Walks through AmigaDOS hunk records and stores the address of each loaded section into a table.
- `record_hunk_section_addresses`: Walks hunk records and stores the address of each loaded section into a table, using the section pointer and the current record.
- `copy_longword_blocks_counted`: Copies longword blocks from a source pointer to a destination while counting down a 16-bit count held in a register.
- `adjust_relocated_hunk_addresses`: Walks through AmigaDOS hunk records, adjusting the section addresses stored in each table to account for the relocated load base and patching relocation entries.
- `decrement_until_zero`: This library code repeatedly decrements a value in a data register until it reaches zero, then returns.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 4 to a1, then returns.
- `adjust_hunk_section_addresses_3`: Walks through AmigaDOS hunk records, adjusting the section addresses stored in each table to account for the relocated load base and patchin.
- `copy_name_then_call_library`: Copies a zero-terminated name into a local buffer of about 108 bytes, then calls a library vector repeatedly with that buffer until a result ends the loop.
- `append_global_string_and_call`: Finds the end of a string in a local buffer, appends a zero-terminated string from a global, sets a flag byte, and calls a library vector.
- `read_powerpacker_signature`: Reads a four-byte header through library calls and checks it against the PowerPacker signature text, reading a size value when it matches.
- `find_task_then_call_handler`: If a library call returns two, looks up the current task through exec and calls a stored handler with its data.
- `find_task_then_call_handler_2`: Calls exec to find the current task, then calls a stored handler with its data, returning a failure code when the handler reports zero.
- `validate_stream_header_read`: Calls library vectors to read a value from the stream and checks it against an expected size, returning a negative error code on failure.
- `read_length_and_allocate_buffer`: Reads a four-byte length field through library calls, shifts stored flags, and allocates a buffer.
- `verify_chunk_length_and_read`: Shifts stored flags, allocates a buffer through the library, and reads a value that must match an expected length.
- `check_chunk_id_and_allocate`: Rejects a chunk whose two-byte identifier is the PowerPacker marker text with an error code.
- `allocate_and_verify_chunk`: Stores a size value, allocates a buffer through the library, and compares a value read from the stream with the stored size.
- `release_chunk_resources`: Reads a value through library calls and compares it with an expected count, then releases buffers with exec's FreeMem.
- `free_buffer_and_return_value`: Frees a buffer through exec's FreeMem, then returns a value saved in its argument block.
- `parse_header_with_forbid`: Disables task switching through exec, fills a stack structure with fields read from the input, and re-enables switching.
- `compute_header_buffer_sizes`: Derives buffer sizes from 16-bit header fields, adjusting them by fixed constants and storing the results in a stack structure.
- `wait_and_reply_port_messages`: Waits on a message port through exec, takes each message, and replies to it after checking its type value.
- `verify_entered_value_loop`: Repeatedly reads a value through a library vector and compares it with an expected value, clearing a byte buffer on each mismatch until a counter runs out.
- `read_byte_via_library`: Calls a library vector to fetch data, then returns the byte at the address register.

### mathtrans.library

- `mark_empty_routine`: This routine contains no executable instructions; it is empty and performs no operations on any data.
- `store_argument_in_global`: Copies the longword passed in the address register into a global slot and returns.
- `unlink_node_and_free`: If a counter field is zero, unlinks the structure from its list by rewriting its neighbours' pointers and frees it through exec's FreeMem.
- `mark_node_pending`: Sets the pending flag bit in the structure and returns zero.
- `open_library_on_first_use`: Increments a use counter in the structure.
- `return_failure_code`: Returns minus one with no other effect.
- `release_on_last_use`: Decrements a 16-bit use counter in the structure.
- `forward_single_call_and_return_2`: As part of the mathtrans shared library, calls one routine and returns, with no other processing of registers or memory.
- `refine_float_root_bits`: As part of the maths shared library, splits a single-precision value into exponent and mantissa, returning early for zero and negative inputs.
- `run_restoring_division_loop`: Repeatedly sets a bit selected by a count register and subtracts shifted remainder values, in the style of restoring division.
- `call_library_and_set_flags`: Calls a library vector and returns its result with one flag bit cleared and another set.
- `call_vector_sequence`: Calls a fixed sequence of library vectors at offsets from the base register, alternating between two entries and ending with a third.
- `forward_single_call_and_return_3`: As part of the maths shared library, calls one routine and returns, with no other processing of registers or memory.
- `conditional_float_dispatch`: Calls five library vectors and other routines, branching on a sign or zero test to choose which sequence runs.
- `branch_on_sign_and_call`: Calls library vectors depending on whether one argument is zero and the sign of another value.
- `branch_on_sign_and_call_2`: Calls library vectors depending on whether one argument is zero and the sign of another value.
- `mask_flags_after_branch_call`: Calls library vectors chosen by a sign test and a zero test, then returns the result of another routine with four flag bits cleared.
- `call_vector_and_mask_flags`: Calls one library vector, then returns the result of another routine with four flag bits cleared.
- `call_vector_twice_and_mask`: Calls one library vector twice, then returns the result of another routine with four flag bits cleared.
- `mask_return_value`: This library code calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `check_float_exponent_range`: Masks off the sign bit and tests the exponent range of a value.
- `run_restoring_division_loop_2`: Shifts a dividend and repeatedly subtracts a shifted divisor over a fixed count of steps, setting result bits as it goes.
- `shift_subtract_loop_with_counter`: Runs a shift-and-subtract loop driven by a 16-bit counter, adjusting a running difference on each pass.
- `reduce_angle_and_compute_trig`: As part of the maths shared library, reduces an angle argument into a principal range using fixed-point multiples of a quarter-turn constant, then selects a sign and evaluates the result through a short series of steps.
- `validate_and_trig_step`: Validates a byte argument against fixed bounds, then scales a value by a power of two and reduces it within a range before evaluating one step of a trigonometric series.
- `clamp_byte_to_fixed_point`: It validates a byte argument against fixed thresholds and returns early for out-of-range values.
- `adjust_dual_accumulator_range`: It rejects small byte arguments and otherwise performs the same range reduction as the preceding routine, but tracks two accumulators that are combined with shifting.
- `compute_trig_series_step`: Evaluates one step of a trigonometric series by shifting and adding table-scaled terms in a loop.
- `normalize_trig_angle`: It normalizes a fixed-point angle into a standard range using a table-scaled iterative loop, then invokes a chain of helper routines to complete the trigonometric reduction.
- `compute_trig_series_step_2`: Runs one iterative step of a trigonometric computation, adjusting an angle and an accumulator by table-scaled terms in a loop.
- `reduce_accumulators_and_branch`: It reduces a byte-scaled value into a target range using two shifting accumulators, unwinding any steps taken.
- `compute_trig_series_step_3`: Runs an iterative trigonometric step that updates several accumulators from table terms in a loop.
- `stub_empty_return_3`: Consists of a single return instruction that does nothing.
- `recursive_call_wrapper`: The routine contains no instructions and immediately returns.
- `call_two_routines_in_order`: Calls two routines in sequence and returns, with no other effect.
- `normalise_integer_with_exponent`: Normalises a 32-bit integer by shifting it in 16-bit steps while tracking an exponent base.
- `normalize_fixed_point_value_2`: It checks whether a signed 32-bit integer is negative, then normalises it by swapping its words and adjusting a shift count base.
- `call_three_routines_in_order`: Calls one routine, then two more, because the flag tested between them is always false.
- `call_one_then_three_routines`: Calls one routine and then three more, because the flag tested between them is always false.
- `call_with_incremented_byte`: If a byte argument is non-zero and does not overflow when incremented, calls one routine and then calls two others with the incremented value.
- `empty_return_routine_2`: The routine contains no instructions and immediately returns to its caller.
- `empty_return_routine_3`: This routine contains no instructions and simply returns without performing any operation.
- `return_immediately_3`: The routine executes five instructions and then returns without modifying any data or registers.
- `empty_stub_2`: The library code contains five instructions and performs no observable work; it ends immediately.
- `empty_routine`: The library code consists of four 68000 instructions and performs no observable work, immediately returning to its caller.
- `shift_value_by_exponent_field`: Returns at once for zero.
- `call_four_routines_in_order`: Calls four routines in sequence and returns, with no other effect.
- `accumulate_table_with_sign_switch`: Walks a table of longwords with a 16-bit count, adding or subtracting each entry depending on the sign of the running total.
- `subtract_table_from_accumulator`: It walks a table of longwords using a 16-bit counter, adding each entry to a running total.
- `normalise_float_sign_and_exponent`: Handles the zero and sign cases of a floating-point argument by calling routines, then normalises the exponent with bit shifts.
- `do_nothing_return`: It performs no operations and returns immediately.
- `refine_float_root_bits_2`: As part of the maths shared library, runs a fixed number of bit steps that compare shifted values against constants to refine a result.
- `run_restoring_division_loop_3`: Shifts a dividend and repeatedly subtracts a shifted divisor over a counted number of steps, setting result bits as it goes.
- `call_three_routines_in_order_2`: Calls three routines in sequence and returns, with no other effect.
- `normalise_float_sign_and_exponent_2`: Handles the sign of an argument by calling routines, and for non-zero values with a sign bit it shifts the value by an exponent taken from the low byte.
- `normalise_float_sign_and_exponent_3`: Calls routines to handle the sign and zero cases, then, if the argument is non-zero, shifts it by an exponent derived from its low byte.
- `stub_empty_return_4`: Contains no instructions and performs no operation.
- `return_nothing_3`: Contains no instructions and simply returns to its caller without modifying any data or registers.
- `return_nothing_2`: This routine contains no instructions and simply returns to its caller without modifying any data or registers.
- `increment_global_variable`: Loads a value from a global variable, adds a constant to it, and stores the result back to the same global variable.
- `return_empty_routine_2`: It contains no instructions and simply returns to its caller without modifying any data.
- `return_empty_routine`: This routine contains no instructions and simply returns to its caller without modifying any data.
- `return_no_operation`: This routine contains only a return and performs no operations on any data before handing control back.
- `call_pointer_and_return_value`: Calls the routine whose address is in the address register, then returns the value that was in the data register on entry.
- `call_indirect_return_d0`: This library code receives a function pointer in a register and a value in another register, calls the routine pointed to, and then returns the value that was in the data register on entry.
- `call_routine_keep_result`: Calls one routine and returns the value that was in the data register on entry.
- `call_and_return_d0`: This library code calls a routine through a pointer and then returns the value that was in the data register on entry without modifying it.
- `dispatch_float_return_d0`: This library code dispatches to one of several library vectors based on sign or zero tests, then returns the value that was in the data register on entry.
- `call_check_exponent_return_d0`: This library code calls a routine through a pointer, masks off the sign bit and checks the exponent range of a value, then returns the original data register value.
- `call_trig_series_return_d0`: This library code calls a routine through a pointer, evaluates one step of a trigonometric series, then returns the value that was in the data register on entry.
- `call_and_return_d0_2`: It calls the routine whose address is stored in a data register, then returns the value that was in that register on entry.
- `pass_and_return_d0`: Passes the incoming data register value to another routine and returns whatever that routine puts back into the data register.
- `store_result_via_pointer`: Calls one routine, then stores the data register value into the longword addressed by a second data register and returns.
- `call_then_three_return_d0`: This library code calls a routine through a pointer, then calls one more routine followed by three more, then returns the value that was in the data register on entry.
- `call_then_two_return_d0`: This library code calls a routine through a pointer, then calls one more routine followed by two more, then returns the value that was in the data register on entry.
- `invoke_callback_return_value`: Invokes the routine whose address is stored in a global variable and then returns the value that was in the data register on entry.
- `forward_register_value`: This routine forwards its input register value to another routine and returns that routine's result unchanged, acting as a pass-through wrapper.
- `return_d0_value`: This routine delegates to another routine and then returns the value that was in data register D0 when it was entered, leaving D0 unchanged.
- `forward_register_value_2`: This routine forwards its input data register value to another routine and passes that routine's returned value straight back to its caller.
- `call_indirect_return_d0_2`: It receives a function pointer and a value, calls the pointed-to routine, and returns the value that was in the data register on entry.
- `pass_d0_through_routine`: This routine passes the value in data register D0 straight through to another routine and then returns that same value unchanged.
- `return_nothing_4`: This routine is empty and simply returns to its caller without modifying any data or registers.
- `call_library_vector_keep_result`: Calls a library function through a pointer loaded from a global slot, then returns the value that was in the data register on entry.
- `invoke_indirect_return_value`: Invokes a routine at an address formed from a pointer in a global slot plus an offset in the address register, then returns the data register value.
- `call_library_vector_keep_pair`: Calls a library function through a pointer loaded from a global slot, then returns the two data registers as they were on entry.
- `invoke_indirect_return_concat_2`: It invokes a routine at an address formed from a global slot plus an offset, then returns the concatenation of two data registers.
- `call_indirect_and_return`: It invokes another routine located by adding an offset to a global pointer, then returns the value that routine placed in the data register.
- `call_sloted_routine_return_d0`: It calls another routine located at a slot in a global table minus an offset, then returns the value that routine placed in the data register.
- `invoke_offseted_routine`: It invokes the routine located at the address obtained by adding a negative offset to a global variable, then returns the combined contents of the two data registers.
- `invoke_indirect_concat_return`: It invokes the routine whose address is taken from a global slot plus a fixed offset, then returns the combined contents of the two data registers.
- `call_indirect_return_registers`: It invokes another routine whose address is formed by adding an offset to a global variable, then returns the combined contents of the D0 and D1 registers.
- `invoke_indirect_return_concat`: Invokes a routine at an address formed from a pointer in a global slot plus an offset in the address register, then returns the concatenation of the two input values.
- `call_indirect_at_offset`: Calls a routine at an address formed from a pointer in a global slot plus an offset in the address register, then returns the data register unchanged.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in a register by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### mathieeedoubbas.library

- `residentMI`: This routine is empty and performs no operations.
- `libInit2`: The library routine stores the incoming library segment list pointer into a global variable and returns.
- `MIExpunge`: If a counter field is zero, unlinks the structure from its list and frees it through exec's FreeMem, then returns the stored segment value.
- `Exp_InUse`: The library routine sets a flag bit within a structure field and returns zero.
- `MIOpen`: Returns minus one if the pending flag bit is set.
- `openFail`: The library routine returns the negative one value, signaling an open failure to its caller.
- `MIClose`: Decrements a 16-bit use counter in the structure.
- `DPFix`: Converts a double-precision value read through a pointer into a 32-bit integer by extracting its exponent and mantissa and shifting the mantissa.
- `dfi4`: Returns the data register D1 cleared to zero when bit 1 of the input is clear.
- `dfi6`: Returns zero with no other effect.
- `dfix1`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `DPFlt`: Converts a signed 32-bit integer into double-precision form by normalising its magnitude, then writes the two resulting longwords through a pointer with the sign kept.
- `DPAbs`: Copies a double-precision value from one pointer to another while clearing the sign bit of its high word, which gives the absolute value.
- `dcmp_1`: Small routine of 2 instructions that subtracts 1 from d0, then returns.
- `dcmp_2`: Small routine of 2 instructions that adds 1 to d0, then returns.
- `dtst_1`: Returns its data-register input minus one.
- `dtst_2`: Returns its data-register input plus one.
- `cc_1`: Calls the routine whose address is in the address register and returns with no other change to the registers.
- `cc_2`: Calls the routine whose address is in the address register, then returns the eight-byte value held in a stack slot.
- `cc_3`: The routine calls a function pointer held in a register and returns the value left on the stack at a fixed offset, performing no other operations.
- `MIIEEEDPFix`: Calls the double-to-integer conversion routine and returns with no other processing.
- `MIIEEEDPFlt`: Calls the integer-to-double conversion routine and returns the eight-byte value left on the stack.
- `MIIEEEDPCmp`: Hands control to the double-precision compare routine, which returns through the address-register call.
- `MIIEEEDPTst`: Hands control to the double-precision test routine, which returns through the address-register call.
- `MIIEEEDPAbs`: Calls the double-precision absolute-value routine and returns the eight-byte value on the stack.
- `MIIEEEDPNeg`: Hands control to the double-precision negate routine, which returns through the stack slot.
- `MIIEEEDPAdd`: Hands control to the double-precision add routine, which returns through the stack slot.
- `MIIEEEDPSub`: Hands control to the double-precision subtract routine, which returns through the stack slot.
- `MIIEEEDPMul`: Hands control to the double-precision multiply routine, which returns through the stack slot.
- `MIIEEEDPDiv`: Hands control to the double-precision divide routine, which returns through the stack slot.
- `MIIEEEDPCeil`: Negates the value, rounds it down with the floor routine, and negates the result again to give the ceiling.
- `MIIEEEDPFloor`: Converts a double to an integer and back, compares the result with the original, and adds or subtracts to finish a floor operation.
- `test_d6_d7`: Calls the double test routine, then the add or subtract routine depending on whether the sign of the data register matches the test result.
- `lt_zero`: Pushes a fixed constant onto the stack and calls the double subtract routine.
- `fptrap`: Forwards a floating-point trap to the operation table through a PC-relative jump.
- `fpoptab`: Jump stub: passes control straight on to badtrap.
- `return_fixed_error_code`: Returns the constant 224 with no other effect.
- `dfsub`: Flips the sign bit of the second operand and adds it to the first, so subtraction is carried out by the add routine.
- `dfadd`: Adds two packed double-precision values by unpacking sign, exponent and mantissa, aligning the smaller operand and combining the mantissas.
- `dfa11`: Shifts the smaller mantissa by the exponent difference, then adds or subtracts the two mantissas with sign handling and renormalises them.
- `dfa14`: Negates a mantissa pair and packs it as a double with the sign bit forced on when the exponent is positive.
- `dfa13`: Shifts a low-order mantissa right one bit at a time by a count, adds it to the other mantissa, renormalises the sum and stores the packed double, or zero when the exponent is out of range.
- `dfa5`: Subtracts one mantissa from the other, shifts the difference left or right until it is normalised, and stores a packed double with the sign bit set.
- `dfa8`: Builds quotient bits by repeatedly comparing a remainder with a divisor and shifting, then stores the packed result.
- `dfa7`: Stores the assembled double at the target address when the exponent is positive.
- `dfa9`: Writes zero to both longwords of the destination double and returns the stacked result.
- `dfauo`: Stores a signed maximum double when the exponent falls below the lower bound, and zero otherwise.
- `dfmul`: Multiplies two double-precision values by splitting the mantissas into 16-bit parts and accumulating the partial products.
- `dfm4`: The library routine clears two consecutive words at a pointer location and returns the values previously held on the stack.
- `dfmuo`: It compares a value against a small negative threshold and stores either a positive infinity pattern or all zeros into two consecutive words through a pointer.
- `dfdiv`: Divides one double-precision value by another by shift-and-subtract mantissa division, combining exponents and signs and storing the packed quotient.
- `dfd10`: Runs the quotient-bit loop of a double divide, rounds the mantissa and normalises it.
- `dfd4`: Rounds and normalises the quotient mantissa and stores a packed double with its sign.
- `dfsetz`: The library routine writes zero into two consecutive words at a pointer location and returns the values previously held on the stack.
- `dfdbyz`: Stores a signed maximum double with the sign taken from the first operand register.
- `dfduo`: It compares a value against a small negative threshold and stores either a sign-magnitude infinity pattern or two zero words into consecutive memory through a pointer.
- `dfneg`: Flips the sign bit of the double at the pointer in the data register, unless its exponent field is zero, so a zero stays unsigned.
- `dfcmp`: The decompiled body only returns the 32-bit value passed on the stack, and no comparison logic is visible in this pseudo-code.
- `dftst`: Returns the 32-bit value passed on the stack without changing it.
- `badtrap`: Small routine of 1 instruction that copies 224 to d1.

### req.library

- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `open_requester_libraries_and_state`: Opens the libraries and console device the requester needs and allocates a 196-byte state block, copied from a template.
- `return_immediately_stub`: Returns at once with no library calls or memory writes, so it is an empty entry point.
- `call_vector_then_return_zero`: Makes one library call through a fixed vector, then returns zero.
- `invoke_vector_return_zero`: The library routine invokes a stored routine vector and then calls a second routine before returning zero.
- `call_vector_then_call`: Invokes a stored routine vector and then calls a second routine before returning zero.
- `close_libraries_and_free_state`: Closes each opened library handle and the device handle when they are set, then frees the state memory and its buffer.
- `increment_counter_clear_flag`: Adds one to the 16-bit counter at offset 32 of the requester structure and clears bit 3 of the flags byte at offset 14.
- `decrement_counter_and_finish`: Decrements the 16-bit counter at offset 32.
- `finish_or_mark_pending`: If the counter at offset 32 is zero, it passes the value stored at offset 50 to a library vector and returns that result.
- `call_vector_return_stored_value`: Calls a library vector and then returns the 32-bit value stored at offset 50 of the same structure.
- `return_zero_stub`: Does nothing and returns zero, so it is an empty handler entry.
- `forward_then_return_argument`: Calls a follow-on routine, then returns the 32-bit value that was in the first data register on entry.
- `clamp_rectangle_to_bounds`: Clamps the position and size words of a rectangle to the limits given on the stack, then returns 1 when the rectangle fits inside those limits and 0 otherwise.
- `divide_scaled_words`: Computes a 16-bit quotient of two values after scaling them to fit, and returns minus one for a zero divisor.
- `divide_after_shifting_down`: Shifts a 48-bit numerator and its divisor right together until the divisor fits in 16 bits, then returns the quotient.
- `return_minus_one_result`: Returns a result word of minus one, with the stacked argument as the low word.
- `divide_scaled_minus_one`: Divides a 32-bit value by a scaled 16-bit difference, shifting both to fit, and returns the quotient minus one.
- `shift_divide_minus_one`: Shifts the numerator and divisor right together until the divisor fits in 16 bits, then returns the 16-bit quotient minus one.
- `empty_library_routine`: This routine is empty.
- `multiply_word_by_difference`: Multiplies a 16-bit factor by the difference of two 32-bit words, applies carry correction, and returns the high-order part.
- `run_requester_event_loop`: Lays out the requester window and its text from the supplied strings, then waits on the window's message port.
- `run_requester_text_loop`: Copies the prompt text into a buffer, sizes and places the requester window, and sets up its gadget block when the geometry values are non-zero.
- `run_requester_with_signal`: Allocates a signal for the requester, sets up its window and gadgets, then polls the message port for gadget clicks and keys.
- `run_requester_timed_wait`: Opens a timer port for the requester, starts a timed request, and polls the window port for gadget clicks, keys and timer completion.
- `poll_requester_messages`: Waits on the window's message port, takes each queued message, and returns a result from a gadget click or a matching key.
- `wait_for_requester_input`: Loops on the window port, reading each message and returning the gadget identifier for a gadget-up event or a result code from a matching key.
- `read_requester_key_result`: Reads queued input messages and maps the typed keys to result codes, returning as soon as a matching key or a gadget identifier is found.
- `release_requester_timer_resources`: Aborts any pending timer request, waits for it, closes the timer device, removes the port, and frees its signal and memory.
- `draw_requester_text_items`: Walks a null-terminated list of text items and draws each one through a rendering call, advancing until the terminating zero byte.
- `find_longest_string_width`: Scans a list of text items and returns the largest width reported for them, stopping at the zero terminator.
- `count_characters_to_newline`: Counts characters in a string up to a newline or the terminating zero and returns that count.
- `multiply_two_word_results`: Calls two routines that each return a 16-bit value and returns their product.
- `init_requester_gadget_box`: Writes the position, flags and text pointer fields of a gadget structure from the supplied coordinates, then links it to the next gadget in the list.
- `init_rectangle_structure`: Zeroes a four-word rectangle structure and sets its width and height from the two data-register arguments.
- `init_requester_text_item`: Sets the position, draw mode and text pointer fields of a text item from the supplied coordinates and links it into the list.
- `format_message_return_zero`: Formats a message through the system formatter using the arguments the caller prepared, then returns zero.
- `store_byte_if_valid`: Stores a single byte from a register into a pointed-to location when that location is not null, then returns.
- `read_process_task_value`: Looks up the current task, and if it is a process returns the positive 32-bit field at offset 184 from it.
- `init_requester_window_frame`: Stores the position, size and title fields into a requester structure, then initialises its gadget and border lists through two follow-on routines.
- `init_requester_gadget_item`: Sets the gadget type, flags and limit fields of a requester item and links it to the following item in the list.
- `build_requester_window_layout`: Allocates a requester structure, copies the caller's title and geometry into it, sizes the window from its text and gadget widths, and sets up its message port and border list.
- `build_number_entry_requester`: Allocates a requester for numeric input, sets its geometry and gadget widths from the caller's values, and sets up the prompt "Get a number" with its message port and gadget list.
- `run_text_entry_requester`: Opens a text-entry requester with the prompt "Get some text" and Ok and Cancel gadgets, then waits for input.
- `flag_out_of_range_entry`: Checks an entered value against its minimum and maximum.
- `close_window_and_menu`: Clears the menu strip and closes the requester window, then frees its two memory blocks.
- `draw_requester_box_outline`: Moves the drawing position and draws four lines with the rendering library to outline a box in the window's rastport.
- `init_requester_gadget_frame`: Sets the flags and position fields of a gadget pair from the flag bits and coordinates, then passes the state through a library call.
- `init_gadget_from_stack_args`: Sets the flags and position fields of a gadget pair from stacked arguments, applying a fixed offset and width, then returns.
- `adjust_gadget_position`: Shifts a gadget's position and width by the supplied deltas and marks it highlighted when flag 64 is set.
- `select_requester_font_height`: Looks up the requester's text font.
- `forward_to_follow_on_routine`: Calls a follow-on routine and returns nothing.
- `get_selection_value_pair`: Reads the 16-bit value and its companion word from the requester's current selection record.
- `read_selection_word_pair`: Returns two words from a selection record: the value at offset 20 and the companion at offset 24.
- `push_node_onto_list`: Pushes a node onto a singly linked list by storing the current head in the node and making the node the new head.
- `call_then_store_pointer`: Calls a stored routine, then reads a four-byte value from a structure field and stores the returned pointer back into that same field.
- `push_node_with_position`: Pushes a node onto a singly linked list, storing its two position words at offsets 4 and 6 before linking it in.
- `submit_zeroed_text_request`: Zeroes a ten-word local block, sets a flag and the requested value, then submits it through a library call.
- `retry_requester_step_sequence`: Makes a library call and, if it fails, runs six follow-on steps before trying again.
- `wait_frame_until_ready`: Waits for the next display frame and runs six follow-on steps between attempts, stopping once a library call reports success or the count runs out.
- `call_then_return_value`: Calls a stored routine and then returns the value that was in the data register on entry.
- `draw_text_with_pen`: Moves the drawing position, sets the pen colour, then draws the text through the rendering library.
- `strip_last_path_component`: Trims the last element from the path in the file name buffer at offset 1292, stopping at a colon or at the start of the path.
- `clear_flag_and_call_handler`: If the flag at offset 1672 is set, clears it and calls the handler stored in the structure at offset 364.
- `clear_flag_and_call_library`: If the flag at offset 1672 is set, clears it and calls a library vector through the stored base.
- `free_linked_node_list`: Walks a linked list, unlinking each node and returning its memory with the memory-free call until the list ends.
- `return_stack_value_pair`: Returns the 64-bit value passed on the stack unchanged.
- `init_list_head_node`: Initialises a list header by calling three follow-on routines, then pointing its tail back at itself and clearing its first word.
- `run_four_setup_steps`: Calls four follow-on routines in order and then returns.
- `free_stored_buffer`: Clears the buffer pointer at offset 344 and frees that buffer with the memory-free call when it was set.
- `call_stored_library_vector`: Calls a library vector through the library pointer stored at offset 368.
- `call_stored_window_vector`: Calls a window library vector through the pointer stored at offset 368.
- `call_two_stored_routines`: Calls two library routines in sequence through the pointer stored at offset 372.
- `fill_entry_list_fields`: Clears three counters, then loops over entries returned by library calls, copying their position words and name into the structure until no entry remains.
- `scan_directory_entries`: Reads directory entries one at a time, copying each name and its size and date fields into the requester buffers, and stops when no further entry is returned.
- `show_directory_read_error`: Draws the message "Couldn't read directory." in the requester window through the rendering library, after setting the pen and position.
- `check_flag_then_call_vector`: Checks a flag bit in a structure and, when it is clear, jumps through a stored routine vector before returning a pair of register values.
- `retry_text_render_sequence`: Tries a text rendering library call up to three times, running six follow-on steps between attempts, and stops as soon as the call succeeds.
- `retry_selected_render_sequence`: Chooses one of two text blocks from the flags, then tries the render call up to three times with six follow-on steps between attempts.
- `retry_alternate_render_sequence`: Chooses between two text blocks from the flags, then tries the render call up to three times with six follow-on steps between attempts.
- `retry_secondary_render_sequence`: Chooses a second text block from the flags, then tries the render call up to three times with six follow-on steps between attempts.
- `draw_rectangle_frame`: Passes four coordinate values to a rendering call, then makes four follow-on drawing calls to outline a rectangle.
- `draw_padded_frame_rect`: Draws a rectangle outline padded by two units horizontally and one vertically, using the same four follow-on drawing calls.
- `format_number_as_decimal`: Writes a 32-bit number as decimal ASCII digits backwards into a buffer, one digit per division by ten, ending at the buffer position supplied by the caller.
- `draw_list_item_row`: Copies a list entry's name into a row buffer, pads it with spaces to the column width, adds a size or directory marker, and draws the row.
- `build_list_row_entry`: Builds one list row from an entry's name padded to the column width with a size or directory marker.
- `pad_row_with_blanks`: Pads a list row with spaces up to the column width and writes the directory marker when the entry is a directory, then draws the row.
- `finish_list_row_draw`: Writes a trailing space into the row buffer and draws the finished row through two library calls.
- `find_list_entry_index`: Walks a linked list from the head stored at offset 28 and returns the zero-based index of the given node, or minus one when it is not in the list.
- `count_nodes_until_match`: Walks a linked list from a head pointer, counting nodes until it reaches the target node.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 255 into d0, then copies (a7)+ to a1, then returns.
- `draw_list_row_if_visible`: Works out the row number of an entry, and if that row is in the visible range it rebuilds the entry's padded name and draws it through the rendering library.
- `test_point_in_box`: Returns 1 when the point stored at offsets 1652 and 1654 lies inside the bounds given in the data registers, and 0 otherwise.
- `return_empty_value`: This routine is an empty function body that immediately returns without executing any instructions, touching any registers, reading or writing any memory, or calling any other routine.
- `call_stored_port_vector`: Calls a library vector through the pointer stored at offset 368 and returns nothing.
- `join_path_components`: Copies a base path into a buffer, adds a slash when it does not already end in a colon or slash, and then appends the second name.
- `collect_list_entries_from_events`: Reads queued input messages and, for each matching event, allocates a new list entry, fills its fields and pads its text to the column width.
- `pad_and_link_list_entry`: Fills a new list entry with blank padding up to its width, stores its position word, and links it onto the list head.
- `release_message_and_return`: Calls one library routine to release the pending message, then returns.
- `handle_list_selection_input`: Handles input on a file list: when the pointer is over the list it updates the selected entry, scrolls the view, copies the chosen name into the path buffers and updates the list gadget.
- `track_mouse_button_in_list`: Loops over the list interaction, reading the CIA port register for the mouse button bit.
- `copy_current_name_to_path`: Clears a flag byte and copies the name buffer at offset 2730 into the path buffer at offset 1292, then makes three follow-on calls.
- `update_list_selection_state`: Updates the list selection state when the pointer is over an entry, copies the chosen name and refreshes the list gadget, following the same steps as the other list handler.
- `find_list_entry_and_select`: Looks for the list entry matching a given pointer.
- `store_pointer_position_in_node`: Copies two 32-bit pointer position values into the node at offsets 16 and 20, then makes two follow-on calls.
- `commit_list_selection_state`: Commits the list selection.
- `add_list_entry_from_name`: Allocates a list entry, copies the source entry's name into it, and stores it in the first free slot of the list, up to the list's limit.
- `append_copied_list_entry`: Allocates a list entry, copies the source entry's position and name into it, and links it into the list.
- `spin_countdown_16bit`: Counts a 16-bit value down in a loop until it wraps past zero, as a short busy wait, and returns the original value.
- `spin_short_delay`: Decrements a 16-bit counter in a loop until it wraps to minus one, giving a short busy wait.
- `copy_entry_name_and_draw`: Draws the list entry through the rendering library and copies its name into the current name buffer, after checking the list for a match.
- `draw_listing_status_line`: Chooses a status line such as "Done." or a count of fonts, drawers, shown and hidden entries, and draws it through the rendering library.
- `repeat_call_until_zero`: Calls a follow-on routine repeatedly until the 16-bit value it returns is zero, then makes one final follow-on call.
- `call_until_word_zero`: Repeats a follow-on call until the word held in the data register reaches zero, then makes a final follow-on call.
- `unlink_last_list_node`: Walks a linked list to its last node and clears the pointer that refers to it, removing that node from the list.
- `clear_stored_pointer`: Clears the four-byte value stored at a register-held address, leaving it null.
- `compare_entry_names_ignore_case`: Compares two list entries: when their size fields match, it compares their names one character at a time without regard to case, stopping at the first difference or the end of both names.
- `merge_sorted_lists`: Merges two sorted linked lists using a comparison routine, relinking nodes in order until one list is exhausted, then attaches the remainder.
- `insert_into_sorted_list`: Inserts a node into a sorted linked list by walking the list and relinking the node at the position the comparison routine selects.
- `append_sorted_node_tail`: Relinks the list tail to the node in the second register, walking forward through the list to the insertion point.
- `store_list_link_pointer`: Stores the address of the new node in the list pointer slot, or in the value passed in the first register when that value is non-zero.
- `tiny_move_rts`: Small routine of 2 instructions that copies a0 to (a2), then returns.
- `validate_then_call_routines`: Verifies that a pointed-to integer is present and nonzero, then calls three stored routines in sequence before returning.
- `link_entry_if_unflagged`: When the entry's counter is zero and flag bit 2 is clear, it stores an entry value, marks the entry, and links it into the pending chain.
- `flush_when_below_limit`: Compares an entry counter with its limit and, when the counter is smaller, calls a library vector through the stored window pointer.
- `update_entry_count_and_flush`: Works out the difference between two counters, clamps it at zero and stores it, then flushes through a library call when the counter is below its limit.
- `tiny_movea_moveq_rts`: Small routine of 3 instructions that copies (a7)+ to a0, then loads 1 into d0, then returns.
- `test_font_file_match`: Decides whether an entry passes the font filter, returning 1 for a match and 0 otherwise, based on the sign of its size field, the flag bits and the listing mode.
- `test_info_file_match`: Decides whether an entry passes the info-file filter, using the sign of its size field and the flag bits, and returns 1 for a match and 0 otherwise.
- `tiny_moveq_movea_rts_2`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a0, then returns.
- `move_entries_between_lists`: Walks a list of entries and moves each one onto the list head chosen by the flag, updating the two counters as it goes.
- `relink_entries_into_list`: Moves entries from one list onto another in turn, then returns zero once the source list is empty.
- `reset_list_counters`: Zeroes the entry counter and list head fields, then makes three follow-on calls.
- `create_named_list_entry`: Allocates a list entry, copies the given name into it with its length and flags, links it onto the list, and flushes when the time since the last update exceeds a threshold.
- `link_entry_and_check_timer`: Links an entry into the pending list and updates its count.
- `check_elapsed_and_flush`: Reads the current timer value, works out the time since the stored start, and when that exceeds a threshold makes two follow-on calls.
- `branch_on_library_result`: Calls a library vector and runs three follow-on steps when it returns zero, or a single step otherwise.
- `forward_routine_result`: It is a thin pass-through routine: it invokes another routine, takes whatever value that routine leaves on the stack, and returns it unchanged.
- `flush_pending_entry_count`: When the leading text byte is empty, it updates the entry count and flushes through a library call once the count is past the limit.
- `fill_directory_name_buffers`: Clears a leading byte, then walks directory entries through library calls, copying each entry's name into the path and name buffers, and stops when no entry is returned.
- `toggle_info_file_visibility`: Flips the show-info-files flag when its mode bit is set, and records the matching "Hide .info files" or "Show .info files" label for the menu entry.
- `clear_empty_selection_state`: Clears the selection state when no items are selected, copies the name and path buffers into their destination locations, and shifts a table of entries.
- `commit_single_selection`: When the selection count is exactly one, it clears the name flags, updates the path and status buffers from the selection, and makes the usual follow-on calls.
- `commit_empty_selection_state`: When the selection count is zero or less, it clears the name flags, updates the path and status buffers, and makes the usual follow-on calls.
- `store_row_and_repeat_call`: Stores a row value from the list record, makes a follow-on call, then repeats a follow-on call until the returned word is zero.
- `select_row_if_flag_set`: When the flag at offset 1672 is set, it stores the current row value, makes a follow-on call and saves the row values back into the record.
- `scroll_list_region`: Calls a window library vector, then makes one follow-on call for each row in the given count.
- `save_list_scroll_state`: Copies the current list scroll values into the list record and installs three handler entries in it.
- `copy_names_if_changed`: Compares the current name and path buffers with their saved copies.
- `save_name_buffers_and_reset`: Copies the path and name buffers into their saved slots, zeroes the list counters, and makes three follow-on calls.
- `scroll_list_by_steps`: Steps the list position by the scroll amount, looping while the list is not at its end and calling the follow-on routines, then repaints the list.
- `scroll_list_one_step`: Moves the list position by one step when the scroll condition holds, calling the follow-on routines and repainting the list afterwards.
- `scroll_list_by_count`: Scrolls the list by the row count passed in the data register, stepping the position and calling the follow-on routines until it stops changing, then repaints.
- `dispatch_list_key_action`: Dispatches a list action on the key-state bits of the current entry.
- `dispatch_list_count_action`: Branches on a count value: non-positive counts commit the current selection to the path and status buffers, and other values run the matching follow-on steps.
- `dispatch_list_flag_action`: Checks the list flag bits and the entry state, then either commits the selection to the path and status buffers or runs the follow-on steps.
- `select_list_region_and_redraw`: Chooses the list region offset from the flag bits and pointer position, then calls the window library up to three times, running six follow-on steps between attempts.
- `commit_list_mode_selection`: When the count is positive it dispatches on the mode bits and commits the chosen entry to the path and status buffers.
- `select_list_region_by_count`: Chooses one of several list region offsets from a count and the mode flags, then redraws through the window library with up to three attempts.
- `select_list_region_by_flags`: Chooses one of several list region offsets from the flag bits and count, then redraws through the window library with up to three attempts.
- `select_list_region_on_decrement`: Decrements the count when it is positive.
- `return_nothing_stub`: Returns immediately without any work, so it is an empty routine.
- `build_volume_name_list`: Walks a list of volume or device names, allocating a node for each one that does not match an existing entry, writes it with a trailing colon, and links the node in.
- `build_name_list_entries`: Compares a name against the existing list entries and, when no match is found, allocates a new entry holding the name with a colon appended, then links it in.
- `add_colon_named_entries`: Scans a name string and adds a colon-terminated copy to the list when no existing entry matches, linking each new node in turn.
- `append_colon_named_entries`: Allocates list nodes for the remaining names in the source list, copies each name with a trailing colon, and links them in order.
- `compute_file_requester_layout`: Works out the file requester's layout from its minimum sizes and text metrics, storing the widths and heights of the list, gadgets and border fields.
- `build_file_requester_gadgets`: Creates the file requester gadgets labelled Ok!, Forget it, Get Dir, Parent and the Hide or Show .info files toggle, and lays them out in the window.
- `compute_list_window_layout`: Recomputes the list window geometry from the stored dimensions and text metrics, setting the list height and the widths of its gadget columns.
- `compute_list_row_metrics`: Works out the row metrics of the list from the font height and entry count, and stores them in the list record.
- `measure_list_rows_with_font`: Measures the list rows against the font through a library call, and stores the resulting row count and widths in the list record.
- `store_list_row_refs`: Copies two stored values from the entry into the list record, then makes two library calls to refresh the list.
- `build_font_requester_gadgets`: Creates the font requester labels and gadgets, including Ok!, the show and hide entries and the font size fields, then makes the library calls that set them up.
- `build_font_size_requester`: Builds the font size requester gadgets with the Font name and Size labels and an Ok gadget, then runs the layout calls.
- `reset_list_record_fields`: Zeroes three list record fields and makes a follow-on call.
- `get_port_message`: Takes the next message from a port with the message-get call, falling back to the free pool when none is queued, and returns its pointer or zero when there is none.
- `return_nothing`: This routine is empty and returns 0 without reading or writing any data, registers, or memory.
- `handle_requester_message_loop`: Runs the requester's main message-handling loop, dispatching window and timer events based on flag bits, then copies the requester's state fields into a result block and queues a series of numbered status messages.
- `process_window_messages_close_flag`: Loops over the window's message flags with the close-window check first.
- `process_window_messages_rawkey_alt`: Loops over the window's message flags, dispatching raw key events and gadget events to follow-on steps until a close event ends the loop.
- `process_window_messages_close_save`: Loops over the window's message flags.
- `process_window_messages_gadget_flags`: Loops over the window's message flags, dispatching gadget-up and menu events to follow-on steps until a close event ends the loop.
- `process_window_messages_menu_flags`: Loops over the window's message flags, dispatching menu and item events to follow-on steps until a close event ends the loop.
- `process_window_messages_timer`: Loops over the window's message flags, handling the timer event and dispatching the remaining events to follow-on steps until a close event ends the loop.
- `process_window_messages_full`: Loops over the window's message flags, dispatching gadget, key, menu and timer events to follow-on steps.
- `dispatch_requester_input_events`: Loops over the requester state block, testing an event flag word and dispatching each input event.
- `resume_requester_input_dispatch`: Re-enters the same flag-testing dispatch loop after one library call, so it continues processing input events from the requester state block.
- `allocate_named_requester_entry`: Compares two name strings and, when they differ, allocates a memory block and copies a 356-byte entry record from the caller's data into it.
- `build_requester_entry_defaults`: Allocates a memory block, copies the caller's 356-byte entry record into it, and replaces zero values in two short-value tables with the defaults 1 and 3.
- `initialise_requester_entry_fields`: Allocates a memory block and fills a requester entry record from the caller's data, copying its field tables and setting its header pointers.
- `free_requester_entry_resources`: Calls the library cleanup routine when an entry pointer is set, then frees the memory block held in a second field.
- `configure_file_requester_gadgets`: Copies the requester's title and text strings into its record and sets gadget positions and sizes from a field table.
- `update_requester_state_fields`: Copies the selected entry's name into its text field when a pointer is set, then calls library routines and copies state values from the entry table into the requester header.
- `release_requester_entry_state`: Clears the entry's active flag, calls library routines, and copies the entry's state fields into its header.
- `copy_entry_strings_to_buffers`: Copies two name strings from the entry into the buffers it points to, then copies a 356-byte record into a second buffer and frees memory through the exec free call.
- `get_process_window_pointer`: Finds the current task with the exec FindTask call.
- `copy_entry_status_value`: Stores the status value 15 in a field of the caller's block and copies a value from a fixed offset of the entry into a second field.
- `call_requester_library_hook`: Calls a library function through its function-table entry, using a pointer taken from the caller's block.
- `initialise_requester_layout_block`: Initialises a fixed-layout block by setting its size, position and flag fields, storing the caller's text pointer, and linking the block into the caller's pointer slot.
- `copy_flag_byte_to_entry`: Calls a routine, then copies one byte flag from the requester state block into a fixed offset of the entry.
- `copy_flag_bytes_to_entry`: Calls a routine, then copies two byte values from the requester state block into two fixed offsets of the entry.
- `clear_stack_buffer_bytes`: Zeroes a 14-byte local buffer on the stack and then calls another library routine.
- `initialize_requester_structure`: Initializes a requester structure by linking several internal buffers into a chain, then fills a series of request descriptors with font, palette, and input settings.
- `process_requester_input_events`: Copies a block of words from one area of memory to two others, then repeatedly reads records from a library request and dispatches their events to handlers based on status flags, updating stored coordinates and running cleanup tasks when the relevant conditions are met.
- `dispatch_input_flags_and_cleanup`: Tests input flag bits and dispatches matching events to handlers that read records through library calls.
- `dispatch_input_flags_loop`: Tests input flag bits 32 and 1024 and runs the matching handler.
- `poll_input_events_until_done`: Repeatedly reads input event records through library calls while the 1024 flag is set.
- `read_requester_input_events`: Reads 26-word input event records through library calls in a loop, stores each one in the requester block and checks its flag word to choose the next action.
- `dispatch_library_cleanup_tasks`: Dispatches cleanup tasks by invoking different library methods depending on which of several status flags are set, then returns a value assembled from registers and stack data.
- `call_load_r_g_b4`: Wrapper that calls the graphics library function LoadRGB4 and returns its result to the caller.
- `check_requester_flag_and_call`: Tests bit 128 of a flag word in the requester block.
- `dispatch_keyed_handler_table`: Scans a table of 16-bit keys, terminated by a negative value, for the key passed in the data register.
- `call_pointer_from_block`: Calls the routine whose address is stored in the first longword of the argument block.
- `stub_empty_return_5`: Does nothing and returns; the routine consists of a single return instruction.
- `call_load_palette_wrapper`: Calls the LoadRGB4 wrapper routine that loads a colour palette through graphics, then returns the value -1 to the caller.
- `stub_return_negative_one`: Does no work and returns the value -1 to its caller.
- `call_and_return_zero`: Calls another routine and then returns the value 0.
- `call_index_change_handler`: Invokes another index-change handler through a stored pointer and then returns zero.
- `call_second_index_change_handler`: Invokes a second index-change handler through a stored pointer and then returns zero.
- `run_index_change_handler`: Runs an internal routine and then returns zero, used when an index value changes.
- `load_palette_and_return_zero`: Calls the LoadRGB4 palette wrapper and one other routine, then returns 0.
- `select_requester_mode_code`: Reads a mode code from the entry.
- `store_requester_mode_value_checked`: Takes a mode value.
- `store_requester_mode_value`: Stores a mode value in a state field and returns 0.
- `update_palette_entry_loop`: Loops over a colour entry, recomputes a 4-bit value from its stored colour, and when the value has changed updates it through the graphics SetRGB4 call.
- `tiny_move`: Small routine of 1 instruction that copies d6 to d0.
- `call_get_r_g_b4`: Wrapper that calls the graphics library function GetRGB4 and returns its result to the caller.
- `compute_scaled_position_in_region`: Checks that a stored state value equals 104 and that a coordinate lies within a fixed range.
- `read_palette_colour_mode_four`: When the argument is 4, it reads two colour values through the GetRGB4 wrapper and stores them in state fields.
- `step_palette_between_values`: Loops from one value to another one unit at a time.
- `set_palette_colours_in_loop`: Repeats a counted loop that calls three internal routines and the graphics SetRGB4 call on each pass, then clears a state field.
- `combine_value_with_upper_mask`: It loads a 32-bit value from a global variable and returns it with the upper 16 bits set to all ones, combining that value with the constant 65535.
- `interpolate_colour_nibble`: Computes one 4-bit colour component by interpolating between two colour values over a step count, then stores the result in the table entry before the target pointer.
- `store_interpolated_value_step`: Computes half of a scaled difference, adds a base value to it, and stores the result in the table entry before the target pointer.
- `set_palette_colour_wrapper`: Passes its arguments to the graphics SetRGB4 call through the library function table and returns the registers unchanged.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 4 into d5, then merges 25 into (a0).
- `fill_rectangle_with_pen`: Sets the drawing pen with the graphics SetAPen call, then fills a rectangle with the RectFill call using the coordinates passed in.
- `update_when_index_changes`: If two index values differ, calls up to four internal routines.
- `draw_text_at_position`: Sets the drawing pen and draw mode through the graphics library, moves the pen, and draws text with the graphics Text call.
- `read_shifted_value_until_zero`: Calls an internal routine repeatedly until its second result is zero, then returns the last value read shifted right by four bits.
- `store_pen_pattern_and_draw`: Stores a nibble of input into a structure field, computes a repeated pen pattern from another value, invokes a library method, and returns the original input registers.
- `draw_with_temporary_flag`: Sets the graphics draw mode and sets one bit of an object's flag byte.
- `copy_word_table_to_buffer`: Copies a count of 16-bit words from a source table in the requester block into a destination buffer, looping until the count runs out.
- `copy_byte_block`: Copies a block of bytes from the source pointer to the destination pointer, with the count taken from the data register.
- `draw_grid_of_indexed_cells`: Loops over the rows and columns of a grid, calling two library routines for each cell and passing an incrementing cell index to the second one.
- `draw_row_of_indexed_cells`: Loops over rows, and for each row calls two library routines per cell, passing an incrementing cell index to the second.
- `draw_cells_from_count_table`: Walks a table of run lengths and calls two library routines for each cell with an incrementing index.
- `multiply_32bit_into_64bit`: Stores one longword from an indexed table entry at the target location, then multiplies two 32-bit values from the data registers into a 64-bit result.
- `multiply_two_32bit_values`: Multiplies two 32-bit values from the data registers into a 64-bit result, returned in a pair of registers.
- `round_combined_result_words`: Calls an internal routine that returns a 32-bit value, combines its words into a new 32-bit result, and adds one when the low word is negative.
- `divide_32bit_by_16bit`: Divides a 32-bit value by the low word of the second argument and returns the 16-bit quotient halves packed together.
- `walk_line_with_callback`: Steps along a line segment using an error term, and at each step calls a callback stored in the caller's record with the computed coordinates.
- `walk_line_with_offset_callback`: Walks a line segment in the same stepping pattern as its neighbour, calling a callback stored at a different offset in the caller's record with the computed coordinates.
- `walk_line_with_callback_2`: Splits a 64-bit value into its two 32-bit halves and invokes a callback stored in a record with those values.
- `match_name_against_pattern`: Matches a text string against an AmigaDOS-style wildcard pattern using the hash, question-mark and star tokens, ignoring letter case, and returns a status value from the result.
- `match_pattern_with_star_skip`: Matches a string against a wildcard pattern that contains a star, which lets the match skip ahead and retry from later positions.
- `match_pattern_segment_to_query`: Matches text against a wildcard pattern up to its question-mark token, comparing letters without regard to case, and returns a status value.
- `match_pattern_query_terminated`: Matches text against a wildcard pattern that ends at a question-mark token, comparing letters without regard to case, and returns a status value from the result.
- `stub_return_true`: Does no work and returns the value 1 to its caller.
- `test_string_is_empty`: Returns a flag that is set when the string argument's first character is a zero byte, meaning the string is empty.
- `walk_pipe_separated_items`: Walks a string whose items are separated by vertical bar characters, calls an internal routine for each item, and returns a status value showing whether the walk reached the end of the string.
- `combine_stack_and_constant`: This library routine combines a constant one into its upper half with a value passed on the stack into its lower half, then returns the combined result.
- `stub_return_stack_argument`: Does no other work and returns the value passed to it on the stack unchanged.
- `open_and_dispatch_slot_request`: Opens the exec library on first use, matches the caller's name against a table of named entries, and checks that the slot number lies in the entry's range.
- `dispatch_slot_request_by_name`: Matches the caller's name against a table of named entries, checks the slot number against the entry's range, clears the slots after it, and calls the matched entry's handler.
- `dispatch_slot_request_tail`: Checks that the slot number lies in the matched entry's range, clears the slots after it, and calls the entry's handler.
- `clear_slots_and_call_handler`: Clears the remaining slots of the request block from the current index up to sixteen, then calls the matched entry's handler and returns its result.
- `call_matched_entry_handler`: Calls the handler stored in the matched entry and returns its result.
- `return_constant_one`: This library routine ignores its inputs and simply returns the constant one.
- `set_palette_and_refresh`: Calls the palette-set library routine and then a second library routine in sequence, using the library base held in the argument.
- `set_palette_refresh_with_constant`: Stores a constant longword in a local slot, then calls the palette-set library routine and a second library routine.
- `build_requester_text_and_layout`: Clears a 46-byte local buffer and fills in the requester's text and position fields from the caller's record.
- `show_okay_query_requester`: Sets up a requester with the 'okay?' prompt text and layout values from the requester record.
- `show_cancel_requester`: Sets up a requester with the 'Cancel' label in its button fields and layout values from the record, then calls the requester routines to display it.
- `show_resume_requester`: Sets up a requester with a 'Resume' label, builds it through the requester routines and stores the returned value in the caller's result slot.
- `show_okay_button_requester`: Sets up a requester with an 'Okay' label, builds it through the requester routines and stores the returned value in the caller's result slot.
- `show_requester_without_label`: Sets up a requester from the layout values in its record with no label text, builds it through the requester routines and stores the returned value in the caller's result slot.
- `stub_return_error_code`: Does no work and returns the fixed error code 47 to its caller.
- `build_requester_and_store_result`: Calls two requester routines in order, the second producing a result that is stored through the caller's result pointer.
- `run_requester_if_ready`: Runs the requester routines when a field in the record is zero or a check passes, and stores the result through the caller's pointer.
- `release_then_build_requester`: Calls a requester library routine, then calls the requester build routine and stores its result through the caller's pointer.
- `store_requester_result`: Calls the requester build routine and stores its result through the caller's pointer, then returns 0.
- `build_requester_with_flags`: Clears a 34-byte local buffer, sets flag bits from the record, and checks the requester state through a library call.
- `build_requester_flagged_result`: Sets a flag bit when a record field is empty, checks the requester state through a library call, then builds the requester and stores the result through the caller's pointer.
- `build_requester_with_flag_set`: Sets a flag bit in a local block and checks the requester state through a library call.
- `store_result_return_zero`: Calls an internal routine, stores its result through the caller's pointer, and returns 0.
- `allocate_and_copy_requester_text`: Measures the requester's text string, allocates a memory block for it, and copies the text into the block.
- `allocate_requester_block_with_minimums`: Clears a 34-byte local block, raises two size values to minimums of 255 and 40 when they are too small, and allocates a memory block for the requester.
- `allocate_requester_buffer_with_text`: Allocates a memory block for the requester and copies its text into the block when a length is given.
- `copy_text_buffer_and_free`: Copies the requester text into an existing buffer when a length is given and fills in its fields.
- `build_requester_in_temp_buffer`: Raises a size field to 40 when it is too small, allocates a temporary memory block, and builds the requester into it.
- `free_requester_buffer_and_store`: Calls an internal routine and stores its result through the caller's pointer.
- `parse_signed_decimal_string`: Parses an optional plus or minus sign followed by decimal digits from a string, stores the value through the caller's pointer, and returns the number of characters consumed.
- `parse_signed_decimal_string_2`: This library routine parses the digits of a signed decimal string, storing the resulting integer through the caller's pointer and returning the number of characters consumed.
- `store_signed_value_from_string`: Stores a value through the caller's pointer, negating it when the string begins with a minus sign, and returns the number of characters consumed.

### Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:Jetstrike

- `open_system_libraries_at_startup`: Program entry code that opens the dos, graphics and intuition libraries by name through the exec library, and initialises drawing and timing calls.
- `handle_cancel_dialog`: Waits for a counter to reach zero, clears a 32-entry block, and stores a pointer in several fields of a structure.
- `copy_names_and_close_libraries`: Copies two name strings and a data block from the program's state into local buffers, then clears two state fields and closes a library through the exec CloseLibrary call.
- `process_string_and_update_pointers`: Waits for a counter to reach zero, clears a 32-entry block, and stores a pointer in several fields of a structure.
- `clear_block_and_write_newline`: Clears a 32-entry block, sets internal pointers and calls a library routine.
- `branch_on_word_and_call`: Compares a 16-bit field with 37 and calls one of two exec library routines through the global library base, depending on the result.
- `allocate_memory_block`: Calls the exec AllocMem routine through the global library base, passing its arguments on unchanged.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_copy_mem`: Library stub: loads its arguments and the exec library base, calls CopyMem and returns its result to the caller.
- `walk_byte_table_update_pointers`: Walks a zero-terminated byte table.
- `clear_field_and_call_library`: Clears a longword field in the program state and then calls a library routine through a pointer stored in that state.
- `process_descriptor_and_chain`: Reads a descriptor structure and, depending on the sign of its short value, writes a table entry or copies its fields.
- `process_table_and_cleanup`: Advances a pointer through a table based on a size check, then calls a routine and stores its result.
- `stub_empty_return_6`: Decoded from a gap in the program file.
- `process_string_and_table`: Runs a library routine, then processes a stored string by locating its end and replacing the terminating byte with a newline, while walking a table of paired longwords and invoking handlers based on several state fields.
- `check_table_entry_and_prepare`: Looks up a table entry by index and checks a four-byte tag.
- `reset_counters_and_call`: This routine calls another routine, clears two words through a pointer, and returns an input value.
- `signal_task_on_switcher`: Calls the exec FindTask routine and, when a task is found, sends it a signal through the exec Signal call.
- `decode_packed_back_references`: Part of a bit-packed decompressor.
- `decompress_bitstream`: Main routine of a packed bitstream decompressor.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Jetstrike (1993)(Rasputin)[cr Outlaws][h Nomad].adf:bootblock

- `boot_block_init_hardware`: Boot block entry point, called by the Kickstart strap.
- `reset_then_call_program_entry`: Calls the reset routine and then jumps through a global pointer to start the program.
- `enable_vblank_disable_bitplane_dma`: Writes to the custom chip's interrupt enable register to enable the vertical blank interrupt, and to its DMA control register to clear the bitplane DMA bit.
- `initialise_io_request_block`: Fills in the fields of a request structure with two longword values, a zero field and a word from its argument, then calls a library routine through the exec library base.
- `initialise_io_request_field`: Stores a word from its argument into a request field, then calls a library routine through the exec library base.
- `clear_memory_and_set_pointers`: Clears a block of memory word by word, then calls two library routines to allocate and set up its contents.
- `call_two_library_routines`: Calls a library routine at one offset and then another routine at a second offset through the library base.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `access_colour_register_entry`: Decompiler output was unavailable for this routine, so it is described only from its hardware use: it touches the colour register COLOR16 when executed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `build_audio_channel_table`: Clears a 32-longword block and builds a table of four entries from a source list, computing their pointers and fixed values.
- `install_handler_and_reset_state`: On its first call, saves the current value of a global handler pointer and replaces it with this routine.
- `advance_audio_sequence_step`: Sets a value for each of four channels, calls four routines, and advances a step counter through a table, wrapping to the start when the table ends.
- `empty_return_routine`: The routine contains no instructions and performs no operations on any data or registers.
- `advance_sound_channel_sequences`: Large routine that steps through sound channel sequence data, reading the next entry, handling repeat counts and updating each channel's sample state.
- `write_audio_channel_registers`: Checks the state of each of four sound channels, updates its stored pointer and length, and writes them to the audio location and length registers.
- `negate_sample_byte`: Negates one byte of sample data at an index computed from a counter and a base offset.
- `mix_two_sample_buffers`: When a count field is set and a counter is zero, averages two 32-byte sample buffers into a third, adding a bias of 128 to each sample and walking the source index with wrap-around.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_table_data_to_cell`: This routine calls another routine to transfer bytes from a table into a specific cell.
- `write_table_bytes_to_cell`: Looks up the index of a character in a table, then writes eight bytes from a second table into successive rows of a buffer, moving to the next cell after every 40 cells.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialise_display_and_blitter`: Opens the exec library, then sets the display window and fetch limits, blitter, copper and interrupt registers, and the DMA control value.
- `project_points_with_perspective`: Walks a list of six-word records and converts their two coordinates by dividing by a depth value, then adds offsets of 180 and minus 10.
- `process_consecutive_point_pairs`: Walks a table of point indices and, for each pair of neighbouring points, looks up their coordinates and calls a routine with them.
- `process_point_pairs_by_count`: Like its neighbour, walks a table of point indices and calls a routine for each pair of neighbouring points, using a count taken from a data register.
- `wait_blitter_set_registers`: Waits until the blitter is idle, then loads its size, modulo and pointer registers with values for the next operation.
- `shift_point_table_entries`: Moves nine six-word point records forward by one record, copying two values from each into the next position.
- `rotate_point_table_by_angle`: Rotates nine point records by an angle taken from a sine and cosine table, in two passes, then offsets them by 125 units.
- `copy_words_to_fixed_slots`: Copies eight 16-bit values from a table entry, located just before the current pointer, into fixed memory slots.
- `draw_line_with_blitter`: Computes the octant and error terms for a line between two points, then starts a blitter line draw after waiting for the blitter to finish.
- `program_blitter_twice_from_table`: Looks up a value in a table, then waits for the blitter and programs it twice with size and pointer values derived from the table.
