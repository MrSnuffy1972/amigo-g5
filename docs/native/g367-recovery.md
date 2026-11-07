# Game 367 (JimPower v1.1 0068): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00367). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4062333 top-level bytes in 13 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 121 of 121 named (0 author symbols), 121 of 121 described (9 by rule, 54 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1165 of 1165 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1751 of 1751 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 294325 |
| code | 5910 |
| data | 3762097 |
| padding | 1 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 369 | 24 |
| Disk.1:bootblock | 1 | 6 | 1 |
| logged disk load from Disk.1 boot-block load | 1 | 80 | 3 |
| logged disk load from Disk.1 | 1 | 80 | 4 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 157 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 34 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1008 | 73 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- JimPower_v1.1_0068/JimPower_v1.1_0068_hdf/JimPower/Disk.1: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- JimPower_v1.1_0068/JimPower_v1.1_0068_hdf/JimPower/Disk.2: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- JimPower_v1.1_0068/JimPower_v1.1_0068.hdf: hard-disk image (AmigaDOS OFS, 4352 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree JimPower_v1.1_0068_hdf, which is recovered file by file; no block outside the filesystem holds data
- JimPower_v1.1_0068/JimPower_v1.1_0068_hdf/JimPower/Disk.1: the boot block reads 17408 bytes from disk offset 868352 to an address (found statically); recovered as a unit
- 5 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 53 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f150): 34 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 738 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- JimPower_v1.1_0068/JimPower_v1.1_0068_hdf/JimPower/Disk.1: 5 logged loads cover 274944 bytes of the image
- JimPower_v1.1_0068/JimPower_v1.1_0068_hdf/JimPower/Disk.1: 7 track-loaded blocks found in recovered runtime segments and classified through them
- JimPower_v1.1_0068/JimPower_v1.1_0068_hdf/JimPower/Disk.1: blocks 1731-1731 (bytes 886272-886784) outside the filesystem hold data, not code (opcode density 0.059, entropy 5.17)
- JimPower_v1.1_0068/JimPower_v1.1_0068_hdf/JimPower/Disk.1: blocks 1738-1759 (bytes 889856-901120) hold ASCII text
- 3 packed track region(s), 1509888 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `copy_longs_with_dma_setup` (runtime segment an address (f150)): Copies a block of longwords from a fixed source address into low memory with a count-down loop, after storing several shared words and saving the stack pointer. Kind: self-modifying code / loader decryption. Pass path: the loop completes and execution continues to u7__sub_0550().
- `initialise_channel_state_tables` (runtime segment an address): Clears the channel's flags and counters, copies four words from a table indexed by a record byte into shared words, and resets an eight-entry table of pointers and constants. Kind: disk hardware/channel state initialization. Pass path: the routine proceeds to initialize state tables and hardware registers (DAT_00dff09e) based on the loaded data, implying the check is an implicit part of the hardware state setup..

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `unpack_bitstream_data`: Reads a bitstream from memory and reconstructs bytes by shifting and masking bits.
- `update_state_and_hardware`: Writes various constants and routine addresses to global variables, modifies a global value, and sets a bit in a global variable based on a register value.
- `initialize_global_data`: Writes several constant values and a routine address into multiple global variables and decrements a global value.
- `update_state_variable`: Checks a character input against several constants and updates a global variable with specific values or increments it.
- `wait_for_bit_clear`: Performs a busy-wait loop until bit six of a global variable is cleared.
- `store_and_wait_for_bit`: Stores a word into a structure and then performs a busy-wait loop until bit six of a global variable is cleared.
- `store_constant_and_wait`: Stores a constant value through a pointer and waits for a specific bit in a global variable to be cleared.
- `store_offset_constant_and_wait`: Stores a constant value at an offset from a pointer and waits for a specific bit in a global variable to be cleared.
- `store_register_and_wait`: Stores a value from a register through an offset pointer and waits for a specific bit in a global variable to be cleared.
- `store_register_and_call`: Stores a value from a register through a pointer and then calls another routine.
- `store_offset_register_and_wait`: Stores a value from a register at an offset from a pointer and waits for a specific bit in a global variable to be cleared.
- `store_offset_constant_and_wait_alt`: Stores a different constant value at an offset from a pointer and waits for a specific bit in a global variable to be cleared.
- `wait_while_bit_set`: Performs a busy-wait loop while bit six of a global variable remains set.
- `set_interrupt_request`: Writes a constant value to a global variable.
- `update_interrupt_request_status`: Sets a global variable to a constant value and interacts with hardware registers to manage interrupt requests and port directions.
- `decompress_data_loop`: Repeatedly calls a routine through a pointer in a table until a value in that table is no longer negative.
- `call_indirect_function`: Calls a function through a pointer located at a specific offset within a table and returns the resulting values.
- `decompress_atom_data_stream`: Validates a data header for a specific signature and performs a bitstream decompression routine.
- `tiny_moveq`: Small routine of 1 instruction that loads 2 into d6.
- `process_bit_stream`: Iterates through a data buffer to perform bitwise operations and shifts based on the contents of the memory.
- `return_uninitialized_register`: This routine is empty and returns the current value of a data register to the caller.
- `return_from_subroutine`: This routine is empty and performs no operations before returning to the caller.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Disk.1:bootblock

- `load_disk_data`: Reads data from a disk device into memory using a bit-stream decompression loop.

### logged disk load from Disk.1 boot-block load

- `return_maximum_value`: The routine returns a constant value consisting of all bits set to one.
- `shift_packed_longword_right`: Returns the longword four bytes before the address register shifted right one bit, with the top bit set when a flag bit in a data register is set.
- `shift_bits_from_packed_longwords`: Shifts bits out of a register for a count taken from its argument, reloading from the previous packed longword whenever the register empties and feeding a flag into the top bit.

### logged disk load from Disk.1

- `unpack_bitstream_to_memory`: Reads a bitstream from a packed longword source and expands it into a destination buffer.
- `return_maximum_value_2`: This routine returns a value consisting of all bits set to one.
- `extract_bit_shifted_longword`: Checks a specific bit in a data register and combines it with a longword located four bytes before the address in a register, which is then shifted right by one bit.
- `extract_bits_from_packed_stream`: Shifts bits out of a register one by one for a specified count, reloading the register from a memory pointer whenever it becomes empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_event_id_actions`: Iterates through a list of identifiers and executes specific subroutines or modifies a global variable based on matching values.
- `initialize_global_pointers`: Stores the address of a global variable into a global variable and two locations relative to the user stack pointer.
- `write_constant_values`: Stores the value sixteen thousand three hundred eighty-four into a global variable and two offsets from the user stack pointer.
- `wait_and_store_values`: Polls a byte at an offset from the user stack pointer until a specific bit is cleared, then stores three values at various offsets from the user stack pointer.
- `reset_status_bytes`: Sets several global variables to specific constant values.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `process_flag_and_globals`: Checks a global variable for specific bits to set another global variable to nine, then performs several assignments from various memory locations to a global variable.
- `clear_and_assign_globals`: Sets two global variables to zero, calls another routine four times, and then assigns a series of constant values to a global variable.
- `update_state_comparison`: Compares a value from a register against a global variable and updates a status byte based on the comparison result.
- `update_data_structure`: Uses an index and a byte from a table to look up a pointer in a global table and updates several fields in the referenced structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `save_registers_and_trigger_trap`: Saves all data and address registers to the stack and writes the address of the next instruction into a specific memory location before triggering a software interrupt.
- `copy_longs_with_dma_setup`: Copies a block of longwords from a fixed source address into low memory with a count-down loop, after storing several shared words and saving the stack pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_bad_instruction_data`: Decodes as a few stores of zero and constants with repeated calls to an unnamed routine, which looks like data misread as code.
- `flush_pending_bit_flags`: Writes a constant to a shared word for each set bit of a pending flag byte, then clears that flag.
- `jump_to_stub_bad_instruction_data_3`: Jump stub: passes control straight on to stub_bad_instruction_data_3.
- `store_byte_then_call_routine`: Stores the low byte of the data register into a state byte, then calls an unnamed routine and returns.
- `decode_channel_command_byte`: Dispatches on the range of an input command byte: special values update the fields of a per-channel record and return, while ordinary values store the note index, reset several record fields and copy a value from a 16-entry lookup table into the record.
- `init_channel_tables_and_pointers`: Stores the caller's pointer parameters and a base value, then fills a 32-entry word table with a constant and clears a companion table.
- `jump_to_stub_bad_instruction_data_3_2`: Jump stub: passes control straight on to stub_bad_instruction_data_3.
- `compare_byte_sets_direction_flag`: Saves the input's low byte and the byte above it, then sets a flag to one, minus one or zero depending on how the new byte compares with the previously saved byte.
- `stub_bad_instruction_data_2`: Decodes as a few stores of zero and constants with repeated calls to an unnamed routine, which looks like data misread as code.
- `sync_channel_parameters`: Extracts data from a memory buffer to update specific fields within a structure pointed to by a global table.
- `set_dma_control`: Stores a word with a specific bit set into a global variable and calls another routine.
- `reload_countdown_and_poll_flag`: Decrements a countdown field in a record and, when it goes below zero, reloads it from a shared word.
- `tiny_addq`: Small routine of 1 instruction that adds 4 to a0.
- `interpret_sequence_command_byte`: Reads a command byte from a sequence stream and dispatches on its low four bits.
- `mark_done_and_advance_counter`: Sets a state byte to idle and advances a counter pair, resetting it to a stored start value when it reaches its limit.
- `interpret_sequence_with_loop_jump`: Variant of the sequence command reader that handles one command type by decrementing a repeat counter and jumping into a shared dispatch path.
- `interpret_sequence_table_jump`: Sequence command reader that, for one command type, loads a pointer from a table indexed by the command byte and jumps into the shared fetch path.
- `store_byte_and_advance_counter`: Stores the low byte of a value from the caller's frame into a state byte and increments a position word in the record.
- `set_state_byte_to_idle`: Sets one status byte in the record to the all-ones value and returns.
- `run_sequence_command_loop`: Loops over command bytes from a sequence stream, advancing the record's position for each one and dispatching on the low four bits.
- `dispatch_sequence_commands_until_end`: Reads command bytes from a sequence stream and dispatches on their low four bits, advancing the record's position after each one.
- `dispatch_sequence_table_command`: Sequence command reader that first uses the command byte as an index into a pointer table to set the record's stream pointer and state byte, then runs the usual low-nibble dispatch.
- `dispatch_sequence_commands_with_table`: Sequence command reader in which one command type loads a pointer from a table and jumps into the shared fetch path.
- `load_eight_words_from_table`: Reads eight 16-bit words from a record pointed to by the caller's frame.
- `jump_to_clear_frame_byte_flag`: Jump stub: passes control straight on to clear_frame_byte_flag.
- `update_entity_attribute_list`: Updates a sequence of entity attributes by reading values from a data table based on an index.
- `resolve_entity_attribute_pointers`: Reads a sequence of sixteen-bit indices from a table and resolves them into absolute addresses using a secondary lookup table.
- `clear_frame_byte_flag`: Clears one byte in the caller's frame and returns.
- `populate_entity_attribute_list`: Reads a sequence of sixteen-bit values from a table and uses them as indices to resolve absolute addresses from a secondary lookup table.
- `advance_channel_modulation_tick`: Advances per-channel modulation once per tick: it counts down timers, adds a step to a running value and reverses that step when a count reaches zero, then writes a scaled level to an output word.
- `update_channel_slide_and_output`: Per-channel tick that counts a delay down and calls a table-selected routine when it expires.
- `update_channel_slide_or_dispatch`: Per-channel tick that sets a flag bit in a frame word and clears a record flag.
- `copy_record_value_and_dispatch`: Copies a stored word from the record into a shared word, sets a flag byte from the caller's frame, advances a position counter and calls a table-selected routine.
- `store_sum_and_dispatch_next`: Clears a record byte, adds two frame values into a stored position word, writes the result to the output pointer, advances a counter and calls a table-selected routine.
- `store_word_and_dispatch_next`: Stores a word from the caller's frame into two record words, advances a counter and calls a table-selected routine.
- `update_channel_slide_with_gate`: Per-channel tick that uses a gate byte: when the gate is zero it sets the gate and calls a table-selected routine, otherwise it runs the step and slide logic and writes the output word.
- `update_countdown_and_dispatch`: Updates a countdown byte in the record, either setting it to the all-ones value or decrementing it from a frame value.
- `calculate_target_and_dispatch`: Calculates a new value by adding a scaled offset to a base value, stores a word and a byte, and then calls a routine from a table.
- `advance_channel_slide_without_dispatch`: Per-channel tick that clears a record byte and runs the timed step and slide logic, then writes the scaled value to an output word.
- `set_channel_pitch_from_table`: Looks up a word in a 64-entry table indexed by a record byte plus a frame byte, scales it by a frame value and stores it as the channel's target.
- `lookup_channel_value_from_table`: Looks up a word in a 64-entry table indexed by a frame byte, scales it by a record value and stores it as the channel's target.
- `clear_state_and_dispatch`: Clears several bytes in a structure, increments a counter, and calls a routine from a table using a value loaded from a table.
- `set_envelope_params_and_dispatch`: Stores a frame byte and a shifted copy of it into the record, sets an enable byte, and resets two record fields when the period field is zero.
- `compute_scaled_offset_and_dispatch`: Stores a byte equal to a frame value plus three times a record value, advances a counter and calls a table-selected routine.
- `store_byte_and_dispatch_next`: Stores a frame byte into a record byte, advances a counter and calls a table-selected routine.
- `store_four_bytes_and_dispatch`: Stores four frame bytes into four record bytes, advances a counter and calls a table-selected routine.
- `update_repeat_counter_and_dispatch`: Updates a repeat counter in the record: it is set to idle, decremented, or reloaded from a frame value depending on its state.
- `update_tick_and_dispatch`: Copies bytes to a structure, increments a counter, updates a value by adding a constant, and calls a routine from a table.
- `update_slide_and_dispatch_2`: Adds two shortwords, updates a value in a structure, increments a counter, and executes a routine selected from a table using a byte retrieved from a pointer.
- `advance_channel_slide_with_repeat`: Per-channel tick that handles a repeat counter, a delay countdown and the timed step and slide logic, then writes the scaled value to an output word.
- `select_record_table_and_dispatch`: Saves the current position and counter fields, selects a pointer by indexing a table with a frame byte, sets the record counter from a frame word and calls a table-selected routine.
- `reset_channel_and_dispatch`: Copies a longword and a word from one location to another, increments a counter, and calls a routine from a table.
- `set_target_and_dispatch`: Stores a word into a structure and conditionally stores it into another location, then increments a counter and calls a routine from a table.
- `accumulate_offset_and_dispatch`: Adds a frame longword into a position accumulator, writes it to an output pointer, reduces a level word by a shifted frame value, advances a counter and calls a table-selected routine.
- `reset_position_and_dispatch`: Clears a flag byte, sets the position accumulator from a frame longword, writes it to an output pointer, sets two output words to one, advances a counter and calls a table-selected routine.
- `run_channel_tick_slide_gate`: Per-channel tick that uses a gate byte and counter reload like its neighbours, runs the timed step and slide logic, then writes the scaled value to an output word.
- `update_entity_state_and_dispatch`: Calculates new coordinate or property values using fixed-point arithmetic and updates an entity structure.
- `increment_and_dispatch`: Increments a counter in a structure and calls a routine from a table using a value loaded from a table.
- `advance_channel_tick_dma_flags`: Per-channel tick that counts timers, steps and slides a value, and writes the scaled result to an output word, using the same logic as its neighbours.
- `update_slide_and_dispatch`: Adds two shortwords together, stores the result in two locations, increments a counter, and calls a routine from a table.
- `compare_table_byte_and_dispatch`: Compares a frame byte with a byte read from memory at an address given by a frame word, advances the command counter by two on a match or by one otherwise, and calls a table-selected routine.
- `run_channel_tick_interrupt_gate`: Per-channel tick that tests a gate byte in the record and calls a table-selected routine when it is clear, otherwise it runs the timed step and slide logic and writes the output word.
- `select_voice_record_and_countdown`: Picks one of three fixed voice records by testing bit flags in a shared word, copies a field from that record into a shared word, and decrements the record's countdown.
- `dispatch_or_jump_on_compare`: Compares a frame byte with a record byte: if the frame byte is smaller it sets the command counter from a frame word, otherwise it increments the counter, then calls a table-selected routine.
- `step_repeat_counter_and_dispatch`: Steps a repeat counter in the record, reloading it from a frame value when it expires, sets the command position from a frame word and calls a table-selected routine.
- `process_command_and_dispatch`: Stores a byte and combines a byte with a shifted value into a word, calls another routine, and then performs a table-based dispatch.
- `set_channel_target_and_dispatch`: Stores a frame word as the channel's target value, advances a counter and calls a table-selected routine.
- `clear_channel_state_and_dispatch`: Clears a few record bytes and a target word, advances a counter and calls a table-selected routine.
- `decode_channel_note_command`: Decodes a command byte stored in a channel record: a special marker sets a gate counter, and negative values choose one of several setup branches that store bytes or table-based words.
- `clear_frame_byte_and_longword`: Clears one byte in the caller's frame and a longword at a fixed offset from it, then returns.
- `stub_bad_instruction_data_3`: Decodes as a few stores of zero and constants with repeated calls to an unnamed routine, which looks like data misread as code.
- `set_shared_word_and_call`: Stores the input word into a shared word with one flag bit set, then calls an unnamed routine and returns.
- `initialise_channel_state_tables`: Clears the channel's flags and counters, copies four words from a table indexed by a record byte into shared words, and resets an eight-entry table of pointers and constants.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_decompiler_timed_out`: No pseudo-code could be produced for this nine-instruction routine.
- `return_all_ones_status`: Returns the all-ones value in the full 32-bit width and does nothing else.
- `stub_bad_instruction_data_4`: Decoded as bad instruction data that the decompiler truncated at once, so no behaviour can be recovered from this block.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
