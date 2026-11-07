# Game 263 (HeadOverHeels v1.0 1674): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00263). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (249672 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 349 of 349 named (0 author symbols), 349 of 349 described (10 by rule, 102 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4110 of 4110 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 215365 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6095 of 6095 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12260 |
| code | 18484 |
| data | 218925 |
| padding | 3 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| HeadOverHeels.slave | 1 | 67 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6026 | 343 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| AMIGA (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 4048 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- HeadOverHeels_v1.0_1674/HeadOverHeels/AMIGA: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 215365 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### HeadOverHeels.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `install_patch_and_jump_entry`: Calls two function-pointer hooks taken from a structure, clears a memory range in steps through a helper, writes a jump instruction and a constant word into fixed memory locations, and then jumps through a stored entry pointer.
- `toggle_word_on_key_code`: Stores the complement of a key-code byte and, for one particular code, XOR-patches a fixed memory word with a constant.
- `decode_run_length_packed_block`: Unpacks a run-length encoded stream into memory: a positive control byte copies the following literal bytes, and a negative one repeats the next byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_init_sequence`: Sets up a stack-based call sequence with a series of address-setting steps, then calls helpers to run each step.
- `poll_counter_until_nonzero`: Repeatedly calls helpers that advance a shared counter until it becomes nonzero, then returns the value minus one.
- `poll_counter_and_return_decrement`: Like its neighbour, polls a shared counter in a loop and returns the previous value minus one.
- `return_shared_word`: Returns the current value of a shared word without changing it.
- `step_countdown_with_helpers`: Initialises a shared byte to 255 and steps it down through helper calls, branching on the result.
- `run_timed_helper_loop`: Clears a shared variable and calls a fixed set of helpers in a loop, passing state through pushed return addresses until a shared byte becomes nonzero.
- `compare_until_equal_or_return`: Calls a compare helper in a loop and returns as soon as it reports equality.
- `tiny_move_cmp_rts`: Small routine of 3 instructions that copies (a0)+ to d1, then compares d0 with d1, then returns.
- `wait_until_counter_advances`: Busy-waits until the difference between two shared values reaches at least four, then stores the current value back into the shared variable.
- `toggle_flag_and_walk_list`: Flips a high bit in a shared byte, then walks a linked list of records, flipping the same bit in each one and calling a helper for records whose low bits are still set.
- `dispatch_handler_from_table`: Looks up a handler in a shared table using an index derived from the argument, records the record's state byte in a shared variable, and calls the handler unless a flag bit in the record is set.
- `initialize_global_state_2`: Sets a global variable to a constant and performs a nibble table lookup.
- `parse_audio_stream`: Checks a bit in a memory structure and returns a value from an audio data stream parsing routine or a constant.
- `initialize_global_state`: If a global variable is zero, it sets that variable to a constant, writes two bytes to a memory location, and returns a value from an audio parsing routine.
- `update_global_from_input`: Tests a flag, adjusts an offset, and updates a global variable using a value from a record-loading routine.
- `traverse_and_blit_sprite_list`: Recursively traverses a linked list of nodes to perform conditional bitmasking and horizontal span drawing, followed by a secondary pass that blits masked sprite rows if certain bit-level conditions or list-node presence are met.
- `decrement_and_traverse_list`: Decrements a byte at a pointer offset and recursively traverses a linked list, performing bitwise masking and conditional sprite row blitting or horizontal span drawing if certain threshold conditions are met.
- `modify_memory_and_traverse_list`: Decrements a byte at a pointer offset, traverses a linked list of integers, and performs bitwise operations or conditional drawing routines.
- `toggle_attribute_bit`: Toggles a bit in a memory structure and executes a routine that processes a list of nodes and blits masked sprites.
- `modify_memory_byte`: Reads a byte from a memory location, masks certain bits, and if certain conditions are met, performs an XOR operation on a byte at a different offset.
- `process_nibble_table_lookup`: Performs a nibble-based table lookup and updates a global variable or audio stream value based on the result.
- `set_global_bit_if_carry`: Checks the carry flag and sets the second bit of a global variable if it is set.
- `set_global_bit_2`: Sets the second bit of a global variable if the carry flag is set.
- `keep_maximum_byte`: Raises a shared byte to the argument when the argument is larger, and otherwise leaves it unchanged.
- `store_register_to_memory`: Stores a byte from a register into a memory location relative to a pointer.
- `update_global_variable_gated`: Checks a bit in a global variable and, if not set, updates that global variable with a value from another routine and several registers before performing a tail branch.
- `process_clamped_list_entry`: Updates a global variable with a clamped value and iterates through a linked list to perform masked sprite blitting.
- `update_flag_on_carry`: Checks the carry flag and sets a bit in a global variable if the flag is set.
- `update_global_flag_on_condition`: Advances a pointer table and sets the second bit of a global variable if the carry flag is set.
- `set_global_bit`: Performs a bitwise OR to set the second bit of a global variable.
- `lookup_and_update_table`: Performs a nibble table lookup and updates a global variable or audio stream value based on the result.
- `select_audio_stream_value`: returns a constant or the result of an audio data stream parsing routine if a global variable is negative.
- `traverse_entity_list`: Iterates through a linked list of entities while checking or updating flags and offsets in a memory structure.
- `update_record_flags`: Clears a bit in a memory location and conditionally sets or clears bits in other memory locations based on existing flag values.
- `update_record_nibble_flag`: Updates the upper nibble flag of a record byte from a bit in the argument.
- `toggle_and_scan_table_rows`: Toggles a high bit in a shared byte, then scans a table in nested loops, calling a helper for each matching entry.
- `update_bcd_counter_state`: Decrements a binary-coded decimal counter in shared memory and, when it reaches zero, runs a decimal-adjust pass over its digits.
- `update_bcd_counter_and_list`: Decrements a binary-coded decimal counter in shared memory and compares it against threshold values before updating a status word.
- `return_masked_status_with_calls`: Masks a shared status byte to its low seven bits and, when that is nonzero, calls helpers to update state before returning the masked value.
- `scan_characters_with_helpers`: Reads characters from a source pointer, calling helpers for each one, and returns a word value once a terminating character is reached.
- `set_shared_flag_bit_zero`: Sets the lowest bit of a shared flag byte and returns.
- `run_state_checks_with_helpers`: Calls helpers and tests bits of a shared word and a record field to choose between several state updates.
- `update_record_flags_and_counter`: Clears one flag bit and sets another in a record byte, decrements a counter held in the record, and calls helpers depending on a bit of a shared flag.
- `reset_global_and_poll_helpers`: Clears a shared variable, calls a series of helpers, and branches on whether the shared variable has reached a sentinel value of minus one.
- `countdown_and_dispatch_step`: Decrements a shared counter and, while it stays in range, computes a complementary value and calls helpers to act on it.
- `update_record_flag_from_helper`: Calls a helper, clears bit four of a record byte, and sets it again when bit one of the helper result is zero.
- `combine_nibble_and_check_result`: Forces the high nibble of the argument on and stores it in a shared variable.
- `update_maximum_value`: Compares a byte to a global variable and updates the global variable if the byte is larger.
- `set_shared_byte_to_two`: Stores the value 2 into a shared variable and returns.
- `decrement_shared_counter_and_shift`: Decrements a shared byte and clears it when it reaches zero.
- `return_input_or_process_flags`: Returns the argument unchanged when a bit of a shared flag is clear, and otherwise runs a sequence of helpers on the record and shared state.
- `call_helper_when_negative`: Returns early when a shared byte is in a set range and nonzero.
- `expand_bits_to_signed_bytes`: Expands the four low bits of a shared byte into four signed byte values, writing each into a table walked backwards.
- `poll_helper_until_flag_set`: Calls a helper repeatedly until it reports a flag of one, then returns the value it produced.
- `scan_bytes_until_condition`: Walks bytes from a source pointer, comparing them against an argument and calling helpers as it goes.
- `process_stack_argument_bytes`: Works through bytes at a pointer using a stack argument, calling helpers for each one.
- `transform_block_with_helpers`: Processes bytes at a pointer using two counters and calls helpers for each byte.
- `call_through_stored_pointer`: Calls the routine whose address is held in a shared pointer variable, then returns.
- `run_sequence_with_helpers`: Calls seven helpers and manipulates bits of a counter byte during the sequence.
- `call_conditional_pointer`: Checks if a global variable is below a threshold and, if so, calls a routine through a stored pointer.
- `load_stored_routine_pointer`: Calls a helper, then loads a routine address from a fixed location into the shared pointer variable.
- `store_argument_and_pointer`: Stores its byte argument into a shared variable, then sets the shared pointer variable to a fixed routine address.
- `store_scaled_index_and_pointer`: Stores a scaled index computed from its byte argument into a shared variable, then sets the shared pointer variable to a fixed routine address.
- `store_shifted_index_and_pointer`: Stores its byte argument shifted left by three into a shared variable, then sets the shared pointer variable to a fixed routine address.
- `set_top_flag_bit`: Sets the top bit of a shared byte and, if it was already set, runs the rest of the routine through a helper.
- `run_table_until_end_marker`: Stores two argument bytes into shared variables, then calls a helper for each entry of a table until a terminator value of 255 is reached.
- `tiny_addi`: Small routine of 1 instruction that adds 255 to d5.
- `skip_to_indexed_string_entry`: Chooses a table start from the argument, then skips terminator-separated entries until the requested index is reached.
- `duplicate_eight_longwords`: Copies eight consecutive longwords from the source into a destination, writing each source longword to two consecutive destination longwords.
- `combine_flag_results_with_helpers`: Calls helpers to obtain two values, clears the low nibble of the first, and calls a further helper when the low nibble or a low bit of the second value is set.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 0 into d2, then loads 254 into d1.
- `clear_low_nibble_with_check`: Clears the low nibble of a byte value and, when the low nibble or a low bit of the other inputs is set, calls a helper to produce the final result.
- `mask_low_nibble_checked`: Clears the low nibble of its first argument and, when the low nibble or a low bit of the other arguments is set, calls a helper for the result.
- `call_three_helpers_in_order`: Calls three helpers in sequence and returns, with no other work.
- `write_offset_byte_triplet`: Takes a 16-bit value in D1 and writes its low byte, that byte plus 12, and the high byte plus 24 to the same shared output byte in sequence, leaving the last value stored.
- `clear_256_longwords`: Zeroes 256 consecutive longwords starting at a fixed memory address, counting a 16-bit index down from 255 until it wraps to minus one.
- `lookup_nibble_table_byte`: Returns the byte from a 16-entry table at the shared global base, indexed by the low four bits of D0.
- `lookup_triple_index_table`: Multiplies the low byte of D0 by three and returns the table byte at that scaled offset in a constant lookup table.
- `update_table_and_counters`: Updates a byte in a table using a value from a lookup routine and increments two bytes in a memory structure.
- `apply_lookup_deltas_to_record`: XORs the low nibble of a looked-up value into the record byte at offset 15, then adds two register-supplied deltas to the record's bytes at offsets 9 and 10.
- `read_cyclic_byte_sequence`: Advances the counter byte at the pointer and returns the buffer entry at that index.
- `advance_fixed_point_accumulator`: Adds a shifted multi-word fixed-point value into two 16-bit accumulator words with carry, and when the result's top bit is clear it increments the shared byte counter, propagating carry through adjacent bytes.
- `invoke_step_routine_twice`: Calls the same unnamed subroutine two times in succession and returns, performing no other work.
- `reset_record_marker_bytes`: Runs the same two subroutines, clears the top bit of a flag byte in the record, and sets the two bytes that follow it to all ones.
- `run_conditional_init_then_sweep`: Returns early when a preceding check fails; otherwise it writes several shared globals, calls a series of subroutines, and loops over a counter stepping from 80 to 180 in steps of 20, calling a routine each pass.
- `stub_immediate_return`: Contains only a return instruction, so calling it performs no work.
- `loop_call_while_flag`: Calls a subroutine and repeats the call while the zero flag passed in was set, forcing that flag to one on each pass.
- `poll_status_and_dispatch`: Clears the lowest bit of three shared bytes, reads a status byte, and when that status is not all ones with its low bit set, transfers control through a stored routine pointer.
- `set_record_bits_from_flag`: ORs bit one into the byte at the address register, and also sets bit zero when the zero flag is set.
- `xor_invert_shared_byte_bits`: Replaces the shared byte with the input byte XORed against the inverted upper seven bits of its previous value, so bit zero of the result equals bit zero of the input.
- `blend_byte_with_conditional_invert`: Blends the input into the shared byte so bits one to seven keep the old value and bit zero comes from the input, then ORs the input in.
- `return_void`: This routine is empty and performs no operations before returning.
- `set_object_position_from_direction`: Uses a shared direction code to choose a case, writes adjusted offsets from constant tables into the record's position and state bytes, copies them into shared bytes, and returns a status pointer.
- `finish_sprite_state_setup`: Loads a table entry, copies three bytes from a stack argument into shared bytes, clears a record's 32-bit field and sets two of its bytes to all ones, then calls two subroutines.
- `stub_no_op_global_copy`: Assigns the shared global byte to itself and returns, so it changes nothing and performs no work.
- `copy_tile_block_if_flagged`: When a bit in the shared status byte is set and a table-derived value is nonzero, it copies a 12-column by 24-row byte block into a fixed work buffer with a 24-byte row stride, then calls a subroutine and sets a shared global to 64.
- `copy_tile_block_from_input`: Stores the input byte in a shared global, calls a subroutine, copies the same 12-column by 24-row block into the fixed work buffer, calls another subroutine and sets a shared global to 64.
- `conditional_call_on_negative_flag`: Returns the constant minus 55 by default.
- `gate_status_bit_then_call`: Returns the input unchanged when bit zero of the shared status byte is clear.
- `remap_nibbles_into_work_buffer`: Runs a flag-toggling state check, then either returns early or copies a 12-column by 24-row block into a fixed work buffer, remapping each byte's two 4-bit values through constant substitution values and leaving zero and fourteen unchanged.
- `store_index_or_remap_copy`: When the input is not minus one, stores it in a record field, clears bit one of a flag byte and sets a shared global to 223.
- `stub_return_without_work`: Contains only a return instruction, so calling it performs no work.
- `set_direction_flags_from_status`: Derives four shared direction flag bytes from the low two bits of the status global, clearing or setting the corresponding bits.
- `set_record_direction_flags`: Writes direction flags into a record from the low two bits of the input, setting two record flag bytes to one when the matching input bit is clear.
- `read_shared_status_value`: Returns the shared global value as its result, with no other work.
- `return_shared_byte_halved`: Returns the shared global byte shifted right by one bit.
- `call_stored_handler_pointer`: Stores the address of a routine in a shared pointer, passes that address to a subroutine, restores the saved pointer, runs three subroutines, then calls through the shared pointer.
- `copy_block_then_dispatch_handler`: Copies a memory block with the shared block copy routine, runs several further subroutines, and finishes by calling through the shared handler pointer.
- `call_shared_handler_pointer`: Calls the routine whose address is held in the shared handler pointer and returns.
- `copy_24_byte_block_pair`: Copies a 24-byte block from one buffer into another around a subroutine call, copies the block back, assigns the shared global to itself, and clears four shared bytes spaced four apart.
- `clear_four_shared_slots`: Clears four shared byte slots spaced four bytes apart and assigns the shared global to itself.
- `stub_return_empty_hook`: Contains only a return instruction, so calling it has no effect.
- `dispatch_through_handler_pointer`: Calls through the shared handler pointer and returns.
- `swap_byte_ranges`: Swaps bytes pairwise between two buffers, looping over a count taken from the D1 input down to zero inclusive.
- `copy_memory_block`: Copies a block of memory from the address in a0 to the address in a1 one byte at a time, using a dbra loop counter, and returns.
- `endless_loop_with_busy_wait`: Repeatedly advances a shared global by sixteen, calls several subroutines, and busy-waits in a nested countdown loop of twenty by 65536 steps.
- `load_palette_bank_from_table`: Uses the low byte of D0 times 32 as an offset into a constant table and copies eight longwords into colour registers COLOR00 to COLOR07.
- `load_palette_bank_from_global`: Uses the shared global byte times 32 as an offset into the same constant table and copies eight longwords into colour registers COLOR00 to COLOR07.
- `load_palette_from_word_index`: Uses the input word shifted left by five bits as an offset into the constant palette table and copies eight longwords into colour registers COLOR00 to COLOR07.
- `compute_offsets_and_release_slot`: Derives offset values from the low bits of the shared status byte, stores them in shared globals and calls two subroutines.
- `find_first_slot_and_clear`: Scans a table forward in two-byte steps for the first nonzero entry, then clears that entry and its following byte when the low flag bits of that byte are set.
- `copy_block_forward_by_count`: Reads a count byte at the pointer and copies that many bytes, starting just after the count, to a position shifted forward by the count.
- `copy_strided_longwords_to_global`: Calls a subroutine, then copies four longwords from the input buffer into consecutive shared longword slots, advancing the source by eight bytes for each copy.
- `copy_strided_longwords_to_pointer`: Copies four longwords from the source pointer to the destination pointer, advancing both pointers by eight bytes per step.
- `select_table_entry_by_nibble`: Stores the input byte in the shared global, then scales the low nibble of the input by eight and stores the resulting table address in shared globals.
- `snapshot_record_position_fields`: Saves the record's first two longwords into the fields at offsets 24 and 28, subtracts six from the byte at offset 31, and, when a flag bit is clear, copies the byte at offset 8 into offset 32.
- `insert_node_into_linked_list`: Copies the input bytes into a node buffer, optionally duplicates a fourteen-byte header, walks a linked list to find the insertion point, and splices the new node in by updating its forward and backward links.
- `splice_node_into_linked_list`: Selects a list start from a shared state value, walks the linked list to the insertion point, and splices the record into the list by updating its forward and backward links.
- `link_node_after_list_walk`: Calls two subroutines, walks the shared linked list until its end or a matching entry, then links the node at the address register into the list by updating its forward and backward pointers.
- `link_record_into_list`: Chooses a starting point from a record flag bit, walks the shared linked list to the insertion point, then links the record's four-byte node into the list and returns its forward link.
- `link_node_after_setup_calls`: Calls two subroutines, walks the shared linked list to the insertion point, and links the node at the address register into it by updating forward and backward pointers.
- `unlink_node_from_list`: When a flag bit is set, calls a subroutine and steps the node pointer forward by six bytes.
- `unlink_node_with_flag_call`: When a flag bit is set, calls a subroutine and steps the node pointer forward by six bytes.
- `unlink_list_node_at_address`: Removes the node at the address register from a doubly linked list by joining its predecessor and successor.
- `test_flag_and_adjust_offset`: Returns the record's bit-three flag masked from its flag byte.
- `tiny_addq`: Small routine of 1 instruction that adds 4 to a0.
- `compute_sprite_corner_offsets`: Reads two position bytes and a size byte from a source record, uses flag bits to choose the sign and spacing of each offset, and writes four corner coordinate bytes plus two derived bytes into a destination record.
- `compare_bounding_boxes_separation`: Compares two bounding boxes stored as word coordinates along three axes using strict separation tests on each axis pair.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `tiny_ori_and`: Small routine of 2 instructions that merges 0 into d0, then masks d0 with d0.
- `initialise_frame_state`: Sets default shared values, clears sixteen longwords of a table, and calls a set of subroutines depending on bits eleven and ten of a shared word before restoring that word.
- `initialise_frame_state_pointer_variant`: Resets the same shared values and clears the same sixteen-longword table as its sibling, starting from a pointer reset.
- `combine_buffers_and_search_table`: Computes sums and differences from two source buffers, rotates bit patterns across four words, and when a flag is set, searches a table of word entries for a matching key.
- `add_signed_3bit_delta`: Sign-extends the low three bits of the input, treating bit two as the sign, and returns the byte at the address register plus that signed value.
- `decode_command_stream`: Reads command bytes from a stream: a byte of 255 ends the command, bytes of 192 or more write a run of three bytes with accumulated offsets into a buffer, and other values set a mode byte before the result is chosen by its low bit.
- `select_status_result_code`: Calls two subroutines, then returns a result chosen from the shared status byte.
- `return_default_or_call`: Returns the constant 24 unless the shared global byte is zero, in which case it calls a subroutine and returns its result.
- `set_entry_coordinates_from_record`: Sets a shared index to one, derives coordinate globals from a byte in the caller's record, and calls subroutines.
- `shift_bit_into_accumulator`: Shifts a bit into a shared accumulator word.
- `draw_tile_row_edge_fixup`: Sets a drawing position from a row value times eight plus 36 and writes the input value.
- `draw_tile_row_from_pointer`: Repeats the same row drawing as its neighbour, writing the input value and, when a flag is set, two bytes read from the source pointer at successive offsets.
- `fix_edge_bits_and_draw`: Calls a subroutine, then when the masked bits of the shared byte are zero or all set, rewrites it to its low bit plus one.
- `find_table_record_by_id`: Sets the shared global to 128, then walks a length-prefixed record table until it finds an entry whose second byte matches the input value.
- `return_status_or_negated_field`: Calls a subroutine and returns its result when the shared byte is below sixteen.
- `or_byte_into_destination`: Calls a subroutine, then ORs the value in the D5 register into the byte at the destination pointer.
- `search_keyed_table_load_bits`: Walks a length-prefixed table from a fixed start, comparing the high twelve bits of each entry's key with the input.
- `search_keyed_table_from_pointer`: Walks a length-prefixed table from the address register, comparing the high twelve bits of each entry's key with the input word.
- `shift_out_bits_from_stream`: Shifts the shared bit buffer left once per count step.
- `toggle_status_bit_four`: Selects a bit from the shared status byte and toggles bit four to match it.
- `store_pair_after_leading_call`: Calls a subroutine, then stores two values returned by further subroutine calls into successive destination bytes.
- `store_two_returned_bytes`: Stores two values returned by subroutine calls into successive destination bytes, writes a derived byte from the source byte and a register value, then sets the shared byte from a bitwise combination of three registers.
- `store_two_returned_bytes_variant`: Repeats the same two-byte store and derived-byte computation as its neighbour, with the derived value taken from a shifted register.
- `calculate_scaled_offset`: Adds a byte to a value at a pointer, multiplies the result by eight, and adds eight.
- `invoke_step_routine_thrice`: Calls the same unnamed subroutine three times in sequence and returns.
- `build_byte_pair_table`: Clamps a count derived from two byte inputs, then fills a table with byte pairs in a counted loop.
- `store_halved_sum_and_difference`: Takes a value returned by a subroutine less six and stores half of its sum and half of its difference with the register input.
- `compute_difference_from_two_bytes`: Returns seven minus the byte two positions before the pointer, minus the byte one position before it.
- `set_global_from_row_index`: Sets the shared global byte to the input value multiplied by 192 plus a constant base, keeping the low byte of the result.
- `load_record_state_from_table`: Clears a record flag byte, loads two state fields from a three-byte table entry selected by the input, and sets further record flag bits from a source byte's pattern.
- `advance_frame_from_pointer_table`: Reads a state byte from the record and, when its high bits select a table entry, fetches the next frame value into the record's frame byte and updates the state.
- `set_frame_from_input_byte`: Stores the input byte as the record's frame value.
- `advance_frame_state_core`: Advances the record's frame from its state byte through a pointer table when the high bits are set, storing the frame value and updating the state.
- `reset_buffer_and_copy_tables`: Clears 296 longwords of a buffer and sets shared globals.
- `fill_and_copy_tables`: Clears a block of memory with zeros and then performs a reverse copy of longwords from two source tables into a destination based on a condition.
- `copy_reversed_tables_by_selector`: Chooses one of two source tables depending on whether the input equals six, copies the chosen table backwards into the destination in a counted loop, and returns the input minus six.
- `copy_reversed_sixteen_entries`: Copies sixteen longword entries backwards from two fixed source tables into the destination pointer.
- `build_column_byte_pairs`: When a status bit is clear, it clears the shared byte and fills a table backwards with byte pairs derived from a computed step.
- `swap_longword_halves`: Calls a subroutine and returns its 32-bit result with the upper and lower words exchanged.
- `set_object_offsets_from_code`: Sets shared status values from the input, selects offset values from a small code and writes three record offset bytes, then copies them into shared bytes and returns a pointer.
- `wait_status_then_set_flag`: Calls a subroutine while the shared status byte is nonzero, then stores the high byte of its stack argument XOR three with bit two set from bit zero.
- `copy_fixed_palette_bank`: Calls a subroutine, then copies eight longwords from a fixed table into colour registers COLOR00 to COLOR07.
- `copy_remapped_blocks_from_list`: Runs a fixed sequence of subroutine calls, then loops over a list of records.
- `remap_block_after_state_check`: Performs a conditional state update and a set of subroutine calls, then loops over a list of records, copying each selected 12-by-24 byte block into the work buffer with nibble remapping.
- `remap_block_from_setup_calls`: Makes three setup calls, then loops over a list of records, copying each selected 12-by-24 byte block into the work buffer with nibble remapping.
- `compute_flags_then_release_slot`: Derives a state code from comparing two record coordinate bytes and stores shared offsets from the status bits, calling subroutines along the way.
- `set_status_bit_from_input`: Clears bit two of the shared status byte and sets it from bit zero of the input byte.
- `clear_ten_longwords`: Zeroes ten consecutive longwords starting at a fixed memory address.
- `count_set_bits_in_table`: Counts the one bits across ten consecutive longwords starting at a fixed address and returns the total.
- `count_masked_status_bits`: Counts the set bits in the shared status byte with bit two cleared and returns the count.
- `count_status_set_bits`: Counts the set bits in the shared status byte, including bit two, and returns the count.
- `select_and_reduce_decimal_value`: Starts from zero, 501 or 1000 depending on a flag bit and the shared status byte, then reduces it modulo successive powers of ten.
- `reduce_value_to_decimal_digit`: Returns the input reduced modulo ten by successive division steps using the powers of ten from 1000 down to 1.
- `multiply_low_words`: Returns the product of the low word of the first input and the low word of the second input.
- `call_routine_over_grid`: Makes one initial subroutine call, then calls a subroutine at each point of a grid.
- `store_masked_adjusted_word`: Stores the input word with its low byte increased by three into the shared word, clearing the two lowest bits of each byte.
- `clamp_and_process_list_entries`: Checks the input bytes against range limits and, when they pass, adjusts a value and calls subroutines.
- `clamp_walk_list_single_call`: Makes one setup call, then checks the input bytes against range limits and adjusts a value when they pass.
- `clamp_and_walk_list_core`: Checks the input bytes against range limits and adjusts a value when they pass.
- `walk_list_calling_each_node`: Walks a linked list from the address register, storing each node in the shared global and calling a subroutine for it, until a null link ends the list.
- `blit_masked_sprite_rows`: Blends a sprite into a bitplane buffer row by row, writing each byte as the first source OR-ed with the second source AND-ed against the existing destination byte.
- `convert_chunky_pixels_to_bitplanes`: Converts four chunky source bytes per step into four bitplane bytes by looking up per-plane bit patterns in tables and OR-ing them.
- `draw_clipped_horizontal_span`: Clips a horizontal span against width limits of 56 or 74 depending on a flag bit.
- `return_byte_after_call`: Calls a subroutine and returns the input byte unchanged.
- `dispatch_clipped_span_draw`: Clips a span against the same width limits, then dispatches to one of several fill handlers through stored pointers based on the clipped length.
- `dispatch_span_handler_a`: Calls the span handler stored in one dispatch slot and returns.
- `dispatch_span_handler_b`: Calls the span handler stored in a second dispatch slot and returns.
- `invoke_single_setup_call`: Calls one subroutine, passing no data, and returns.
- `dispatch_span_handler_c`: Calls the span handler stored in a third dispatch slot and returns.
- `dispatch_span_handler_d`: Calls the span handler stored in a fourth dispatch slot and returns.
- `clear_buffer_or_dispatch`: When bit two of the input is clear, it returns the result of a subroutine call.
- `stub_return_no_effect`: Contains only a return instruction and performs no work.
- `tiny_clr_adda`: Small routine of 2 instructions that clears (a1), then adds 24 to a1.
- `clear_strided_longwords`: Zeroes one longword per row of a table and advances the destination by 24 bytes each step, repeating for the number of times given by the D7 input.
- `clear_strided_longword_pairs`: Zeroes two consecutive longwords per row of a table and advances the destination by 24 bytes each step, repeating for the number of times given by the D7 input.
- `copy_strided_longwords_to_rows`: Copies one longword per row from a source offset by four bytes, advancing the source by eight bytes and the destination by 24 bytes per step.
- `copy_longwords_to_rows_base`: Copies one longword per row from a source starting at its base, advancing the source by eight bytes and the destination by 24 bytes per step.
- `copy_longword_pairs_to_rows`: Copies two consecutive longwords per step from the source into each destination row, advancing the source by eight bytes and the destination by 24 bytes.
- `fill_rows_from_cyclic_table`: Selects a table base from the sign of a source byte and calls a subroutine.
- `fill_rows_from_cyclic_longwords`: Chooses one of two table offsets from the sign of a source byte and calls a subroutine.
- `fill_rows_cyclic_longwords_alt`: Chooses table base zero or 64 from the sign of a source byte and calls a subroutine.
- `test_bits_zero_two`: Checks whether bits zero and two of the byte at the address register are both set, then returns without changing anything either way.
- `return_word_after_call`: Calls a subroutine, then returns the input word unchanged.
- `store_call_result_and_registers`: Stores the result of a subroutine call into the shared global, then writes three register input bytes to it in turn, leaving the last input value stored.
- `return_clamped_byte_pair`: Calls a subroutine, then returns a word whose high byte is the smaller of the shared byte and the D2 input and whose low byte is the larger of the shared byte and the D5 input.
- `mask_word_bits`: Increments a word and applies a bitmask to clear specific bits.
- `copy_mask_tables_clear_columns`: Copies 336 longwords from a source into two table regions, then clears and sets bytes of mask columns in a bitplane pair for a row count derived from the input.
- `compute_frame_buffer_offset`: Maps a frame code in the shared global, masked to seven bits, to a buffer offset using piecewise scaling by constants 768, 576 and 896.
- `clear_memory_block`: fills a specified range of memory with zeros.
- `swap_word_halves_of_pair`: Calls subroutines to obtain two longwords, returning them with the upper and lower words of each exchanged.
- `return_immediately`: The routine performs no operations and returns to the caller immediately.
- `compute_sprite_step_offset`: Combines a record's flag byte with the low byte of a subroutine result, then returns a step of four to eight added to the input.
- `calculate_step_value`: Calculates a value based on bytes retrieved from a pointer and returns it.
- `compute_sprite_step_offset_gated`: When bit three of the record's flag byte is set it returns a subroutine's result.
- `return_immediately_2`: The routine performs no operations and returns to its caller immediately.
- `clear_nibble_in_fifty_longwords`: Clears one nibble, bits 16 to 19, in each of fifty consecutive longwords starting at a shared address.
- `search_word_table_after_setup`: When the zero flag is set, it stores a subroutine result and sets two shared globals to 96 and 26, with the first set to zero when the stored value is 14.
- `return_low_nibble_at_offset`: Returns the low four bits of the byte at offset three in the record.
- `search_global_table_for_byte`: Scans fifty-two word entries starting at the shared global for the value of the shared byte, returning as soon as a match is found.
- `search_global_table_for_word`: Scans fifty-two word entries starting at the shared global for the input word, returning as soon as a match is found.
- `update_entity_state_flags`: Iterates through a list of entries to find a match for a specific word and then modifies a global bitmask based on the resulting index.
- `return_default_or_call_negative`: Returns the constant minus 59 by default.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `run_fixed_call_sequence`: Makes a fixed sequence of subroutine calls, passing the constant 24 to two of them, and returns minus one.
- `run_fixed_call_sequence_short`: Makes a shorter fixed sequence of subroutine calls, passing the constant 24 to two of them, and returns minus one.
- `invoke_four_calls_minus_one`: Makes four subroutine calls in sequence and returns minus one.
- `pack_status_bits_copy_tag`: Packs the low bit of each of six shared words into a byte, increments the shared byte, and copies a short tag into the output.
- `copy_four_bytes_fill_four`: Copies four shared bytes to the destination, then writes the shared byte into the four bytes that follow the copy.
- `unpack_header_and_bit_stream`: Decrements a shared counter, unpacks header fields from the source into shared globals, and expands a bit stream into a buffer of words.
- `read_bits_then_unpack_header`: Shifts bits from an input bit stream into a buffer of words for a counted number of steps, then copies header fields into shared bytes.
- `return_scaled_global_plus_one`: Returns the shared global byte plus one, multiplied by 23.
- `or_rotating_bit_run_mask`: ORs into the byte at the address register a mask of bits formed by rotating a run of set bits by the input plus one, so the run wraps around the byte.
- `return_result_or_minus_one`: Calls a subroutine and returns its result when the zero flag is set.
- `run_seven_call_pairs`: Makes one subroutine call, then makes seven pairs of subroutine calls.
- `add_bcd_digit_to_byte`: Adds the input value to the byte at the address register using decimal-coded arithmetic and stores the result back in that byte.
- `decrement_bcd_byte_if_nonzero`: When the byte at the address register is nonzero, it decrements that byte using decimal-coded arithmetic and returns minus one.
- `lookup_table_byte_unless_flagged`: Returns the table byte at the index given by the low byte of the input, or the constant 3 when the shared global byte is nonzero.
- `store_input_and_return_global`: Calls a subroutine, stores the input byte in the shared global, calls another subroutine, and returns the global value.
- `load_global_from_record`: Loads the shared global byte from the first word of the ten-byte record selected by the low byte of the input.
- `find_entry_mask_flags`: Walks a linked list calling a handler for each entry until the handler reports a match.
- `mask_record_flag_with_global`: ANDs the shared global byte into one of two record flag bytes, chosen by bit one of the record's flag byte.
- `dispatch_object_state_variant`: Checks record flag bits and a shared value, then either scans a word table or sets record state and switches on a low nibble to run effect handlers that call subroutines and update shared globals.
- `compute_status_code_from_flags`: Derives a small status code from two flag bits of its inputs and ORs it into the shared status byte.
- `check_attribute_bounds`: Compares several values from a pointer-referenced memory location and returns a calculated difference or a constant if certain boundary conditions are met.
- `validate_attribute_range`: Checks if a calculated difference between memory values matches a certain threshold and validates other memory-based bounds before returning a value.
- `validate_attribute_bounds`: Compares differences between memory-based values against a threshold and returns a status code.
- `evaluate_pointer_offset_conditions`: Performs a complex series of checks against a global variable and values at certain offsets from a pointer, potentially updating a global variable or returning a value.
- `compare_pointer_offset`: Compares a global variable against a byte located at an offset from a pointer.
- `update_global_on_zero_flag`: Updates a global variable to a specific constant or another constant if the zero flag is set.
- `evaluate_complex_conditions`: Performs a series of checks against a global variable and values at certain offsets from a pointer, potentially updating a global variable or returning a value.
- `compare_global_with_offset`: Compares a global variable with a byte located at an offset from a pointer.
- `update_global_on_zero_flag_2`: If the zero flag is set, a global variable is updated to a constant value, or a different constant if a byte at a specific offset from a pointer exceeds a certain threshold.
- `compare_and_update_global`: Compares a global variable or a modified version of it against a byte at a specific offset from a pointer and may update a global variable to a constant value.
- `check_byte_at_offset_2`: Compares a global variable with a byte located at a certain offset from a pointer and may update a global variable.
- `check_byte_at_offset`: Compares a global variable with a byte located at a certain offset from a pointer.
- `compare_global_with_offset_2`: Compares a global variable with a byte located at an offset from a pointer and returns if they are not equal.
- `evaluate_byte_and_set_global`: Checks certain bit flags in a table and compares a global variable or a modified version of it against a byte at a different offset from a pointer, potentially updating a global variable to a constant value.
- `get_conditional_byte`: Returns a byte from a memory location or a byte from a different offset depending on a comparison with a global variable.
- `get_conditional_byte_global`: Returns a byte from a memory location or a global variable depending on a comparison with that global variable.
- `get_clamped_byte_low`: Returns a byte from a memory location or a constant value if a certain byte in a table is below a certain threshold.
- `get_clamped_byte_value`: checks if a byte at a pointer offset is below a certain threshold and returns either a byte from a further offset or a fixed value otherwise.
- `get_clamped_byte_offset`: Returns a byte from a memory location or a constant value if a byte at a specific offset in a table is below a certain threshold.
- `get_conditional_byte_offset`: checks if a byte at a specific offset from a pointer is below a certain threshold and returns either a byte from a different offset or a constant value.
- `calculate_byte_difference`: Calculates the difference between a register value and a byte from a memory location and returns the result.
- `do_nothing`: the routine is empty and performs no operations before returning.
- `calculate_attribute_type`: extracts a byte from a pointer-referenced memory location and uses its bitmask to determine a new value, which is then concatenated with a bit-shifted portion of a register and returned.
- `update_entity_link_and_flags`: Compares a record's byte at offset eleven with the input and, when they differ, walks a linked list of entries to find one in the matching position band.
- `stub_empty_decompile_listing`: The decompiler produced only a return for this routine, which has 154 instructions, so its behaviour cannot be recovered from the supplied evidence.
- `traverse_entity_linked_list`: Iterates through a linked list of entities, checking or updating certain flags and attribute-related offsets.
- `offset_record_position_and_scan`: Calls a subroutine, sets two record coordinate bytes six and seven above a base value, then walks a linked list and calls a subroutine for each flagged entry whose coordinates match.
- `scan_entries_for_collision_row`: Sets a record coordinate byte from its neighbour minus one, then walks a linked list of entries comparing coordinate bytes and calling a subroutine on matches.
- `compare_input_with_record_coordinate`: Compares the input byte with the record's byte at offset nine.
- `invoke_single_call_return`: Calls a subroutine and returns.
- `return_overlap_attribute_byte`: Calls a subroutine, then compares four byte fields of two records.
- `return_overlap_attribute_byte_direct`: Performs the same overlap comparison of record fields as its neighbour, without the leading subroutine call.
- `compute_record_edge_coordinates`: Computes four edge coordinate bytes for a record from its position bytes and a step of three or four chosen by a flag bit.
- `map_index_through_range_checks`: Maps the low byte of the input through a series of range checks that subtract fixed offsets, falling back to fixed values above a threshold.
- `process_state_loop_result`: Runs a loop of subroutine calls and reads a shared status value that selects one of several branches.
- `return_minus_61_or_call`: Returns the input unchanged unless the shared byte equals minus one.
- `copy_remapped_block_from_triples`: Loops over a table of three-byte records.
- `execute_sequence_of_calls`: Performs a series of repeated calls to other routines and a final call through a stored pointer.
- `loop_until_flag_clears`: Repeatedly calls a subroutine while the carry flag is set, then performs a sequence of calls and a final call through a stored pointer if the zero flag is set.
- `execute_initialization_sequence`: Performs a series of repeated subroutine calls, calculates bit counts from a table and a status byte, and reduces a value to decimal digits before conditionally calling another routine.
- `process_table_and_call`: Calculates a value based on a byte from a pointer and a register, stores it in a global variable, and then iterates through a table of routine addresses until a terminator is reached.
- `skip_conditional_processing`: This routine is empty as the conditional checks prevent any subsequent instructions from executing.
- `process_remapped_data_block`: Updates a byte via a pointer and enters a loop that modifies global variables based on values retrieved from a four-byte record structure.
- `iterate_and_update_globals`: Iterates through a table of data, performing various calls and updating two global variables based on values retrieved from a pointer.
- `execute_conditional_call`: Calls another routine and returns only if the result of that call is zero.
- `update_state_bitmask`: Calculates a value based on a table lookup and modifies a global variable and a bitmask using conditional logic.
- `process_state_changes`: Compares values within a structure and a global variable to update the global variable and modify a bitmask based on the results.
- `update_audio_parameters`: Compares a global value with a value from a structure and updates hardware registers and a global variable based on the difference.
- `initialize_audio_registers`: Sets specific word values in memory locations pointed to by registers.
- `return_empty_stack`: An empty routine that returns the value currently on the stack.
- `call_if_negative`: Checks if a global variable is negative and calls another routine if it is.
- `parse_audio_data_stream`: Iterates through a data stream to extract parameters and update a structure.
- `parse_data_stream`: Iterates through a data stream to extract values and update a structure.
- `return_uninitialized_stack_value`: The routine returns a value currently residing on the stack without performing any operations.
- `update_bitmask_state`: Checks bits in a global variable to update other global values and performs bitwise operations on a value returned from another routine.
- `update_entity_state_and_bitmask`: Iterates through a linked list of data structures to aggregate bitwise flags into a global variable.
- `execute_complex_routine`: A routine consisting of 180 instructions that is called by another routine.
- `dispatch_to_complex_routine`: Routine of 38 instructions with no decompiled body available, entered from outside the static control flow and executed in the reference run.
- `execute_complex_routine_tail_short`: Performs a tail branch to another routine using two instructions.
- `update_string_pointer_and_counter`: Decrements a counter or pointer-based length and updates a pointer or a secondary counter if the value reaches zero.
- `execute_complex_routine_tail_minimal`: Performs a tail branch to another routine using four instructions.
- `execute_complex_routine_tail_small`: Performs a tail branch to another routine using twelve instructions.
- `branch_to_complex_routine`: Performs a tail branch to another routine using a single instruction.
- `execute_tail_branch`: This routine performs a tail call to another routine.
- `branch_to_complex_routine_2`: This routine performs a tail branch to another routine.
- `execute_complex_routine_tail_seven`: Performs a tail branch to another routine using seven instructions.
- `dispatch_complex_routine_tail`: This routine is entered from outside the static control flow during execution and is called by the complex-routine dispatcher and the string pointer/counter updater.
- `increment_global_value`: Increments a global variable by a fixed amount and touches the interrupt request hardware register.
- `initialize_audio_and_bitmask_tables`: Initializes several global variables and hardware registers, then populates four memory tables by performing bitwise shifts and logical operations on a sequence of values derived from a single byte.
- `copy_linked_data`: Iterates through a list of pointers and copies data from the source to a destination structure.
- `parse_data_structure_2`: Calculates values based on a global variable and its contents to populate a structure passed via a pointer.
- `parse_data_structure`: Parses a data structure by checking for specific magic constants and uses an indirect jump to call a routine from a table based on the parsed results.
- `parse_data_stream_2`: Traverses a data stream to identify specific magic constants and manages a local stack for processing the results.
- `search_table_and_call`: Searches a table for a matching value and, if found within ten entries, executes a routine from a table using an indirect jump.
- `calculate_offset_difference`: Subtracts a value derived from a pointer's contents from a provided integer.
- `calculate_offset_difference_2`: Subtracts a value derived from a pointer's contents and a small constant from a register.
- `calculate_pointer_offset_difference`: Subtracts a value derived from a pointer's contents and a small constant from a register.
- `calculate_offset_difference_3`: Subtracts a value derived from a memory pointer and a bitwise check from a register.
- `update_pointer_and_calculate_difference`: Stores a pointer incremented by one into a global variable and returns the difference between a register and a value retrieved from memory.
- `update_pointer_and_calculate_difference_2`: Updates a global variable with an incremented pointer and performs a subtraction involving a memory-resident value.
- `search_magic_constants`: Iterates through a data stream looking for specific magic constants to perform table-based lookups and updates a stack-like structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_3`: This routine performs no operations and returns immediately to its caller.
- `perform_no_operation`: The routine is empty and performs no operations before returning.
