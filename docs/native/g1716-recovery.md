# Game 1716 (venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01716). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (908896 top-level bytes in 13 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 178 raw-word lines |
| Functions named and described | pass, 98 of 98 named (0 author symbols), 98 of 98 described (3 by rule, 31 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1039 of 1039 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2192 of 2192 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 147512 |
| code | 8044 |
| data | 328128 |
| padding | 2300 |
| unused | 422912 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2).adf:Type | 2 | 117 | 2 |
| Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2).adf:venus | 2 | 1324 | 66 |
| Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 725 | 26 |

## Cases handled

- venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2).adf:00: high-entropy data file (7.87 bits/byte), classified as packed data
- venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2).adf:01: high-entropy data file (7.64 bits/byte), classified as packed data
- venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2).adf:02: high-entropy data file (7.62 bits/byte), classified as packed data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 148 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_complex_data` (runtime segment an address (play-rs-m1500)): Performs hardware initialization and bitwise operations, then executes a loop that manipulates code pointers and compares memory contents. Kind: disk hardware/track check. Pass path: the branch where the loop completes and returns CONCAT44(in_D0,in_D1) after the successful string/data comparison.

## Functions


### Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2).adf:Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: Copies sixteen longwords from a fixed offset in a base record into a destination buffer, but only when the destination pointer is set.

### Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2).adf:venus

- `decode_table_init_hardware`: Program entry routine that waits for a fixed sync byte in a data stream, then decodes a block of records into a table using arithmetic on each byte.
- `examine_timeout_three_instruction_routine`: The decompiler timed out, so only the structural facts are available: three instructions, entered by an address reference and called from the buffer preparation routine.
- `examine_timeout_six_instruction_routine`: The decompiler timed out, so only the structural facts are available: six instructions, reached as a gap that decodes as code.
- `examine_timeout_nine_instruction_routine`: The decompiler timed out, so only the structural facts are available: nine instructions, one call to another routine, and entry from the buffer preparation routine.
- `advance_fire_button_counter`: Reads a fire-button state byte and, when bit six is clear, steps a small counter that wraps to zero after six.
- `project_entries_into_bitplane`: Walks a list of entries and scales each pair of values by dividing by a counter that drops on every pass.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `interpret_command_stream_until_end`: Interprets a byte stream of commands until an end marker.
- `interpret_command_stream_until_end_2`: Interprets a byte stream of commands until an end marker, entered at the start of the same command loop.
- `interpret_command_stream_until_end_3`: Interprets a byte stream of commands until an end marker, called from the buffer preparation routine as part of the same command loop.
- `copy_indexed_block_rows`: Looks up a source block by an index byte from a shared table and copies eight bytes from it, spaced eighty bytes apart in the source.
- `dispatch_on_shared_word_sign`: Tests the sign of one shared word.
- `reset_shared_state_variables`: Zeroes a flag byte and several shared variables, sets one shared variable to fifteen, and stores a value from a second table into an indexed word location.
- `store_word_into_shared_variable`: Copies a word passed in a data register into one shared variable and then returns.
- `dispatch_command_into_slot_table`: Splits an input word into a four-way slot index from two bits and stores its command byte into that slot's fields.
- `copy_table_until_negative_word`: Sets two shared flags, then copies words from one source table into two shared tables in step.
- `do_nothing_return`: The routine contains thirteen instructions but performs no observable work: it returns to the caller immediately without touching memory, registers, or hardware.
- `tick_countdown_and_reload`: Counts down a timer held in a record and sets a state field to one or two while it runs.
- `decode_sequence_byte_commands`: Reads one entry from a sequence table and, for bytes below 240, adds an offset and reads again.
- `advance_counter_or_mark_finished`: When a status byte is not the terminal value of minus 127, it advances a two-word counter, wrapping back to its start value once the two words match, then calls a routine and sets a timer field to minus one.
- `step_sequence_position_and_decode`: Steps a position counter in a sequence record, reloading a repeat value when it reaches the end marker, then reads the next sequence byte.
- `load_sequence_pointer_and_decode`: Takes a seven-bit index from a parameter, uses it to look up a base pointer in a table and stores that pointer in the record.
- `store_parameter_and_advance_position`: Copies one parameter byte into a record field and increments a 16-bit position counter in the same record.
- `advance_position_through_sequence`: Increments a position counter in a sequence record and reads the entry it points to, looping past plain offset bytes.
- `setup_step_direction_and_decode`: When a shared gate flag is clear, it sets the gate and copies fields to derive a stepping value that reverses direction when two stored values meet, then clears the gate.
- `load_four_channel_entries`: Reads four 16-bit entries from a block selected by a record index and copies each into its own record field.
- `load_channel_entries_and_process`: Loads four channel entries from a selected block into record fields, as the neighbouring routine does, then continues with further field updates.
- `call_then_reset_state`: Calls one routine, then clears a word in the record at a fixed offset and sets a shared state variable to two.
- `populate_channel_data_structures`: Reads a series of short values from a data table and uses them as indices to resolve absolute memory addresses into a target structure.
- `parse_channel_data_table`: Iterates through a data table to extract and resolve a series of pointers or offsets.
- `tick_shared_timers_and_index`: Calls three helper routines, then decrements a shared delay counter.
- `tick_timers_set_audio_pointer`: Repeats the timer and index update of its neighbour and calls the same helper routines.
- `update_channel_record_state`: Advances the timing counters of a channel record and, when its step count runs out, moves to the next entry in its table.
- `dispatch_sequence_entry_type`: Resets three fields of a record, then reads the next entry from a table and dispatches on its type byte.
- `dispatch_sequence_entry_loop`: Loads a word from a record field, advances the record's counter, and dispatches on the type of the next table entry as its neighbour does.
- `accumulate_offset_and_decode`: Adds the lower twenty-four bits of a sequence entry to a base value and stores the sum into two record fields.
- `copy_word_then_decode_entry`: Copies a word from a command entry into two record fields, then advances the position and reads the next sequence entry.
- `copy_word_and_advance_position`: Copies a word from a command entry into a record field and increments the record's position counter.
- `decrement_hold_then_decode`: Decrements a hold counter in the record, or sets it to 255 when it is zero, then loads a new position from the entry.
- `set_hold_then_decode_entry`: Sets a hold counter in the record to minus one, advances the position and reads the next entry.
- `set_hold_and_dispatch_entry`: Sets a hold counter from a command entry, loads a position from the next entry, and dispatches on its type.
- `clear_record_word_field`: Clears a 16-bit field at a small fixed offset in a record and returns.
- `compute_indexed_table_sum`: Sums a record word, a signed byte from a second record and a table word selected by adding two byte fields, then stores the sum in the record.
- `compute_table_sum_and_store`: Adds a record word to a table word selected by a byte field and stores the sum in the record.
- `split_entry_byte_then_decode`: Splits a command byte into a masked field and a halved field stored in the record.
- `compute_scaled_sum_then_decode`: Stores a byte made of an entry value plus three times a stored record value into a record field.
- `store_entry_byte_then_decode`: Copies one byte from a command entry into a record field, advances the position and reads the next sequence entry.
- `store_four_entry_bytes_decode`: Copies four bytes from a command entry into consecutive record fields, advances the position and reads the next sequence entry.
- `decode_entry_unless_flagged`: Leaves at once when a flag byte in the record is set.
- `accumulate_entry_value_then_decode`: Adds a 16-bit entry value to a record accumulator and stores the sum, then subtracts half of the entry value from a second accumulator and stores that.
- `reset_accumulators_then_decode`: Sets an accumulator to a value from the entry and sets two fields to one.
- `step_oscillating_offset_value`: When a flag byte is set, it adds a step byte to an offset and returns the offset plus a base value.
- `decrement_timer_and_accumulate`: Checks a flag byte in a structure and, if it is set, adds a byte to a 16-bit accumulator and optionally mirrors the sum to another structure.
- `step_toward_target_on_tick`: Once a periodic counter in the record runs out, it reloads the counter and steps a 16-bit value toward a target, using a wrap-around window.
- `ramp_field_toward_target`: When a run flag in the record is set and its companion counter has expired, it moves a byte field toward a stored target by a step each pass.
- `dispatch_command_into_slot_table_2`: Splits an input word into a four-way slot index and stores a command byte into that slot's fields.
- `initialise_channel_record_fields`: Clears several fields of a record, computes a table pointer from a masked index, and when a bit flag is set it stores two shared values into an indexed table.
- `reset_state_flags_and_field`: Sets one shared variable to one, clears another shared variable, and clears a 16-bit field in the record before returning.
- `set_flagged_global_from_register`: Takes a word from a data register, sets bit six of it, and stores the result in a shared variable.
- `reset_shared_state_variables_2`: Zeroes a flag byte and several shared variables, sets one shared variable to fifteen, and stores a value from a second table into an indexed word location.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `scan_bitmask_and_call`: Repeatedly shifts a 32-bit mask right one bit at a time, calling a routine for each set bit and updating a small position counter.
- `scan_bitmask_and_call_2`: Runs the same bit-by-bit scan over a 32-bit mask as its neighbour, with a longer body that also writes to memory through a computed pointer.
- `decode_packed_stream_bits`: Decodes a packed bit stream by reading bit groups through the bit reader routines, which refill the buffer from the packed data as needed.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_empty_return_routine`: The body is only a return, so the routine does nothing.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_address_clear_hardware_copy_data`: Stores its own address in a global variable, clears a block of hardware registers, and copies a block of data from one memory location to another.
- `initialize_memory_and_copy_data`: Clears a block of memory at a hardware address and then copies a large block of data from one memory location to another.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_variable`: Checks a specific bit in a global variable, sets it to one if present, and then stores a constant value into that same global variable.
- `set_global_value`: Stores a constant value into a global variable.
- `initialize_hardware_and_unpack_data`: Clears hardware registers and initializes system state before performing a series of data decompression and memory copying operations.
- `decrement_table_values`: Iterates through a table of words, decrementing the high and middle nibbles and the low nibble of each entry until a zero value is encountered.
- `copy_table_data`: If a bit in a global variable is set, it copies data from two different locations in a table to several other memory destinations using a loop.
- `update_global_status`: Checks a bit in a global variable and updates that variable and another global with specific constant values.
- `execute_infinite_loop`: Executes an empty loop that never terminates, so control never returns to the caller.
- `update_global_pointers`: Stores a value into a global variable and performs bitwise operations on it.
- `update_hardware_and_status`: Sets a bit in a global variable, updates that variable and another with specific constant values, and stores a function pointer into a global variable.
- `decompress_bitstream_data`: Searches for a specific magic signature in memory before performing a bit-shifting decompression algorithm.
- `shift_bitstream_process_data`: Shifts a value left by one bit repeatedly, reloading the value from a pointer and adjusting the pointer when the value becomes zero.
- `toggle_interrupt_enable`: Modifies the interrupt enable bit in the INTENA register and updates the CIAA_PRA register.
- `save_register_context`: Stores a series of registers into a table in memory and performs conditional logic based on bits in a global variable.
- `process_complex_data`: Performs hardware initialization and bitwise operations, then executes a loop that manipulates code pointers and compares memory contents.
- `execute_recursive_initialization`: Calls two separate routines and then writes a constant value and a code pointer into a memory location relative to a register.
- `assign_routine_result`: Calls another routine and stores its return value into a global variable.
- `interleave_bit_data`: This routine performs bitwise interleaving of two data sources by masking bits and shifting them to create a single stream.
- `loop_decrement_value`: Decrements a value from two hundred and fifty-five down to zero in a loop.
- `set_bit_and_test_bit`: Sets the second bit of a byte pointed to by a register and checks if the fifth bit of a byte pointed to by another register is set.
- `modify_byte_and_wait`: Performs bitwise operations on a byte and then waits for a specific bit in another memory location to clear.
- `decrement_and_loop`: Decrements a value in a loop while performing bitwise operations on a byte and adding a value to a memory location.
- `write_and_wait_for_bit`: Stores a word into two separate memory locations and then waits in a loop until a specific bit in a third memory location is cleared.
- `mask_byte_and_wait`: Applies several bitwise masks to a byte and then waits for a specific bit in another memory location to clear.
- `initialize_hardware_and_wait`: Performs multiple bitwise operations on a byte and writes values to three memory locations before waiting for a bit to clear in a memory location.
- `return_immediately`: An empty routine that returns immediately.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
