# Game 223 (AbandonedPlaces2 v2.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00223). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3375432 top-level bytes in 19 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 509 of 509 named (0 author symbols), 509 of 509 described (17 by rule, 491 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2701 of 2701 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 3280390 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3920 of 3927 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 13474 |
| code | 13580 |
| data | 3348365 |
| padding | 13 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| AbandonedPlaces2.Slave | 1 | 1005 | 72 |
| AP2 | 3 | 2245 | 182 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 268 | 211 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 395 | 35 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 6 |
| ap2.000 (raw code file, load address unknown) | 1 | 0 | 0 |
| ap2.001 (raw code file, load address unknown) | 1 | 0 | 0 |
| ap2.002 (raw code file, load address unknown) | 1 | 0 | 0 |
| ap2.003 (raw code file, load address unknown) | 1 | 0 | 0 |
| ap2.dir (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- 18 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 202 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 316 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- AbandonedPlaces2_v2.0/AbandonedPlaces2/data/ap2.000: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AbandonedPlaces2_v2.0/AbandonedPlaces2/data/ap2.001: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AbandonedPlaces2_v2.0/AbandonedPlaces2/data/ap2.002: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AbandonedPlaces2_v2.0/AbandonedPlaces2/data/ap2.003: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AbandonedPlaces2_v2.0/AbandonedPlaces2/data/ap2.dir: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 5 never-loaded region(s), 3280390 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### AbandonedPlaces2.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `invoke_struct_routines`: It stores a pointer into a global variable, then invokes several routines located at fixed offsets within the pointed-to structure before returning.
- `patch_jump_table`: It stores a computed value into two fields of a jump table, then calls the routine referenced at the start of that table before returning.
- `dispatch_indirect_jump`: It performs an indirect jump through an unrecovered jump table, dispatching to one of many possible targets before returning.
- `clear_bits_in_global`: It reads a global variable, clears several of its bits using a mask, and returns the modified value.
- `read_interrupt_status`: It reads an interrupt request status register and returns its value unchanged.
- `store_value_and_interrupt`: It stores a value taken from the stack into a global variable and signals an interrupt request.
- `return_zero_constant`: The routine immediately returns zero without performing any operations or modifying data.
- `set_flag_in_struct`: The routine receives a pointer in a register and sets bit 1 in the byte at offset 14 of the pointed-to structure.
- `return_immediately_6`: This routine contains no instructions and simply returns to its caller without touching any data or registers.
- `copy_structures_with_helpers`: It reads a field from a structure, copies data between two structures in a loop using a helper routine, runs another helper repeatedly, and returns a computed flag.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `find_node_by_string`: It walks a linked list of nodes and returns the first node whose stored string matches a provided input string, or a null pointer if none matches.
- `store_register_to_global`: It copies the value from a data register into a global variable and performs no other observable work.
- `store_global_then_jump`: It stores a global variable into another location, then performs an indirect jump through an unrecovered jump table before returning.
- `copy_byte_between_structures`: It copies a single byte from one structure into another and returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_global_constant`: It compares a global variable to a specific constant value and returns 4 if they match, otherwise it returns 1.
- `initialize_pointer_chain`: Writes a sentinel value into a pointer and stores the address of the next entry into three fields in two different structures.
- `copy_nested_pointers`: Copies 32-bit values from pointers referenced at offsets four and eight of a source structure into fields of a destination structure.
- `check_collision_flags`: XORs a global state word with a value from a linked list node and checks for specific bit flags.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 8 to a7, then returns.
- `copy_word_if_valid`: Checks if two pointer arguments are non-null.
- `compute_and_store_byte_logic`: Calls a function pointer from a global structure and combines its result with register values using bitwise logic.
- `negate_nonzero_flag`: The routine receives a value in a register and returns zero if the value is non-zero, and returns a negative one if the value is zero.
- `compare_bit_pattern`: Computes a bit pattern from an input byte and compares it against a global variable, then writes a value to a stack address and returns the computed byte.
- `empty_return_routine_9`: This routine contains no instructions; it simply returns without performing any operation.
- `manage_interrupt_enable`: It enables or disables interrupts by writing to the interrupt enable register depending on a passed flag, then stores a stack value into a global variable before returning.
- `calculate_table_offset`: Reads values from a global table and performs arithmetic operations including division, multiplication, and addition.
- `update_flag_based_state`: It reads a flag byte from one structure and clears a byte in another, then branches based on those flags to either clear a field, set a value, or append a digit character to a buffer.
- `negate_short_toggle_bit`: Negates a short value at a specific offset in a structure and toggles the high bit of a byte at an offset in another structure.
- `set_flag_from_bit`: Reads a bit from a global variable using an index derived from a byte in another memory location.
- `do_small_work`: This routine executes three instructions and then returns.
- `empty_return_stub`: This routine contains no instructions and simply returns without performing any work.
- `clear_and_process_flag`: It computes a bit mask from a structure flag and clears that bit in a global byte.
- `return_immediately_7`: It contains no instructions and simply returns to its caller without touching any data or registers.
- `do_nothing_stub`: Performs no work at all, returning immediately without modifying any memory, registers, or calling another routine.
- `return_immediately_8`: This routine contains no instructions and simply returns to its caller without modifying any data or registers.
- `clear_offset_and_save_register`: It clears a word-sized field located at a fixed offset from a pointer held in a register, then stores the contents of another register into a global variable before returning.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `dispatch_via_address_table`: It compares an address against a global value and dispatches to an indirect call through a different address when they differ.
- `store_routine_pointer`: Assigns a routine pointer to a global location and then returns to the caller.
- `initialize_expansion_structures`: It calls several routines through pointers stored in a global structure to obtain descriptor addresses, sets specific flag and pointer fields within those descriptors, and stores global pointers into fixed RAM locations.
- `invoke_indirect_routines`: It reads a pointer from a structure and invokes the routine located there, then reads another pointer and calls that routine before returning.
- `allocate_and_queue_message`: It sets up a message buffer on the stack, allocates a message from a library port, and then inserts that message into a list before returning.
- `invoke_scan_table_jump`: Invokes several routines, stores their results into fields of a structure, then scans a table of words until a match is found before jumping through a pointer.
- `store_values_dispatch_jumptable`: Performs two other operations, then stores a fixed negative value and a register value into consecutive fields of a structure.
- `copy_buffered_data`: Copies a computed number of bytes from one buffer to another, adjusting the source and destination lengths as it goes.
- `copy_byte_block_2`: Copies a variable-length block of bytes from one location to another, adjusting the source and destination pointers and remaining counts as it goes.
- `search_and_dispatch_table`: It calls a helper routine, copies values into fixed offsets of a structure, and invokes additional helpers.
- `branch_result_store_dispatch`: Invokes a routine and branches on whether its result is negative to store either the value or a negative constant into a structure field.
- `invoke_branch_scan_jump`: Invokes several routines, branches on the sign of a result to store different values, then scans a table of words until a match is found before jumping through a pointer.
- `sign_branch_store_dispatch`: Invokes a routine and branches on the sign of its result, calling different code paths before storing the outcome into a structure field.
- `check_field_store_dispatch`: Checks a field of a structure and, if nonzero, invokes a stored routine.
- `compute_and_dispatch_table`: It reads several fields from a structure, adjusts a value based on a control field, and compares it against another field to set a pair of result values.
- `execute_3_instructions`: This routine executes three instructions and then returns.
- `copy_value_through_pointer`: The routine loads a value from a global variable into a register, stores that value through a pointer, and then returns.
- `empty_return_routine`: This routine contains no instructions and simply returns to its caller.
- `rotate_string_store_length`: It rotates a null-terminated byte string one position to the right by shifting each byte forward and writes the original first byte into the last position, then stores the length of the string minus one as a byte just before the string in memory.
- `store_context_on_result`: Calls a routine and, if it returns zero, invokes a second routine with a fixed value before returning zero.
- `check_pointer_dispatch`: Checks whether an input pointer is nonzero, reads a field from it, and calls routines through a jumptable based on that value.
- `normalize_file_path`: It takes a path-like string, strips any directory prefix and trailing colon-separated segments, and copies the remaining file name into a buffer.
- `call_routine_twice`: Calls a routine twice in a row, checking a flag between the two invocations, and returns the value produced by whichever call set that flag.
- `compare_byte_sequences`: It walks through two byte sequences in parallel, comparing each pair of bytes and returning immediately when a mismatch occurs.
- `decode_invalid_instructions`: It decodes a stretch of data that does not correspond to valid instructions, so the analyzer stops here without recognizing any further control flow.
- `return_immediately`: It contains no instructions and simply returns to its caller without touching any data or registers.
- `invoke_via_jumptable`: Invokes a routine through a global pointer, runs another routine, and then jumps through a pointer stored in a table before returning.
- `check_movep_opcode`: Compares a short at a computed offset against the instruction movep opcode, and if it matches, jumps through a pointer stored at another computed address.
- `write_custom_register`: Writes a single byte to a custom hardware register, then calls another routine and returns the value left in a register.
- `spin_infinite_loop`: It enters an unconditional infinite loop that never reaches its return and never modifies the vertical blank register.
- `flip_and_cancel_bits`: It applies the same bit flip to the byte pointed to by a pointer twice in a row, so the two operations cancel each other out.
- `dispatch_via_result_table`: It invokes a routine through a table and inspects the upper half of the returned value.

### AP2

- `initialize_boot_entry`: Initializes a global variable with a magic value and a pointer, records its own address into a table, then calls a function through that table and reads two 32-bit words from a computed offset.
- `loop_system_trap_handler`: Clears a global flag byte, then repeatedly invokes a system trap and calls another routine in an infinite loop.
- `copy_string_and_init`: Stores a pointer, copies a string from one location into another until a zero byte, triggers a system call, and then runs a flagged handler or finishes setup.
- `scan_table_for_zero`: It scans a table of short values, advancing by fixed strides until it finds a zero entry and then stepping past it, repeating until another zero terminates the search.
- `scan_table_and_count`: It scans a table of short values, advancing by fixed strides until it finds a zero entry and then stepping past it, repeating until another zero terminates the search.
- `process_token_normalize_path`: Reads a token and, if it is negative, sets up a series of pointers and flags before returning.
- `copy_token_bytes_terminate`: Reads a token and, if it is negative, sets up a series of pointers and flags before returning.
- `conditionally_copy_token_with_period`: Only acts when a flag is set, reading a token and, if negative, setting up pointers and flags before returning.
- `parse_text_tokens`: Reads tokens from a text stream, storing each token length and its starting address into parallel arrays, and stops once four tokens have been collected.
- `relink_table_entry`: Reads a value from another routine, stores it into a global, writes a stack value through a computed pointer, and then returns.
- `init_struct_and_clear`: Clears a global, zeroes a block of bytes counted down in a register, then fills several fields of a structure with pointers and a fixed value.
- `clamp_value_or_wait_flag`: Returns a register value directly when it falls within a small positive range.
- `count_and_reset_flag`: Waits for a flag bit to clear, then counts how many times it is set before resetting it.
- `find_set_bit_or_wait_flag`: Scans bits of a flag variable and returns as soon as it finds an unset bit, marking it set.
- `clear_flag_bit_range`: Clears a single bit in a global flag byte when the passed value falls within a valid range, otherwise waits for a flag and drains a counter.
- `reset_global_flag`: Clears a global flag byte by storing zero into it, then returns.
- `enable_dma_channels`: It stores a fixed word into a global variable, setting a high bit that enables DMA channels along with another channel bit.
- `call_routine_four_times`: It invokes another routine four times in a row and then returns, with no visible effect on memory or hardware.
- `loop_until_result_fifteen`: Runs a routine in a loop, repeatedly reading its result until that value equals fifteen, then returns.
- `enable_interrupts_after_init`: Runs an initialization routine and then writes a value with bit 15 set into a hardware interrupt-enable register, enabling interrupts.
- `copy_nonzero_words`: Copies up to fifteen words from one global block into another, skipping any source word that holds zero.
- `copy_words_conditional`: It copies up to seven words from a global variable into a separate block of memory, skipping any source word that holds zero.
- `is_empty_routine`: This routine contains no instructions and simply returns without touching any data or registers.
- `save_flag_before_routine`: Reads a global flag byte, runs a routine, and returns the flag value that was captured before the routine ran.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `store_global_value`: It loads a value from a global variable, stores that value back into the same global variable, and then returns.
- `do_nothing_return`: This routine contains five instructions and does not call any library functions, touch hardware registers, or reference any data.
- `do_trivial_return`: It executes five instructions and then returns.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `do_nothing`: This routine performs no operations; it contains only a return and touches no registers, memory, or hardware.
- `empty_return_routine_2`: This routine contains no instructions and simply returns without performing any operation.
- `execute_five_instructions`: It performs a sequence of five instructions without returning a value, with no library calls, hardware register access, or text references to indicate what it does.
- `empty_routine`: This routine contains no instructions and only returns.
- `copy_variable_value`: The routine loads the value stored at a global variable into a register and writes it back to a different global variable through a pointer, then returns.
- `return_immediately_2`: It is a short routine that performs five instructions and then returns without calling anything or touching hardware registers.
- `return_immediately_3`: This routine contains five instructions but does not perform any visible work; it returns without reading or writing data, calling anything, or touching hardware registers.
- `empty_returning_routine`: This routine performs five instructions but executes none of them, since the body is empty.
- `run_routines_then_enable_interrupts`: Runs two routines in sequence and then writes a value with bit 15 set into a hardware interrupt-enable register before returning the original register value.
- `set_and_clear_flag`: Sets a global flag byte to eight, calls another routine to clear a bit in it, sets it to eight again, and returns.
- `run_pending_cleanup_hook_2`: Checks a global flag and, when it is set, jumps through a function pointer stored in a table to run a cleanup routine before clearing the flag.
- `iterate_string_chars`: Walks a null-terminated string starting at the pointer in A0, repeatedly invoking a helper routine for each character until that routine reports failure or the string ends.
- `mark_empty_routine`: This routine contains no instructions and performs no operations.
- `scan_string_values`: Initializes a global variable to a negative value, then walks a null-terminated string character by character while repeatedly invoking another routine on each value it produces, and finally advances past the string's terminator.
- `check_field_drain_flag`: Checks a field in a global structure and, when it is zero, runs the shared flag-draining sequence before returning a stored value.
- `format_value_if_changed`: If a global short already holds the passed value it returns zero; otherwise it stores the value, formats it as a short string into a buffer, runs a routine, and returns a combined value.
- `dispatch_handler_by_value`: Branches on a passed value, either running the shared flag-draining sequence or invoking a handler from a table indexed by that value.
- `store_d0_to_global`: It stores the value held in register D0 into a global variable and then returns.
- `format_value_if_changed_2`: If a global short already holds the passed value it returns zero; otherwise it stores the value, formats it as a short string into a buffer.
- `store_pointer_and_count`: Stores a pointer into a global slot, then stores either zero or three hundred into another global depending on a flag.
- `store_three_hundred`: Stores the value three hundred into a global variable and then calls another routine.
- `validate_and_read_result`: Initializes globals based on a flag, then validates a passed value and reads a result from a structure when it is in range.
- `store_pointer_and_zero`: Stores a pointer into a global slot, then stores zero into another global depending on a flag.
- `store_zero_global`: Stores zero into a global variable and then calls another routine.
- `skip_null_terminated_string`: Skips through a null-terminated string until reaching its terminator, calls another routine, and returns two register values combined.
- `walk_string_call_helpers`: Walks a null-terminated string by advancing a pointer until the terminator is reached, then invokes two helper routines.
- `walk_string_loop_helpers`: Walks a null-terminated string, then repeatedly calls helper routines until a counter reaches a limit.
- `traverse_string_then_loop`: Walks through a null-terminated string by advancing a pointer until it reaches the terminator, then repeatedly invokes a helper routine and reads its result, looping while an accumulated value stays below a stored limit.
- `concat_strings_into_buffer`: It copies a null-terminated string from a source address into a global buffer, inserting a slash when the source is not already terminated.
- `skip_whitespace_count_token`: It skips leading spaces and an optional dash or quote, then counts the characters of the following token until a delimiter.
- `empty_return_routine_3`: This routine contains no instructions and does nothing.
- `wait_blitter_then_store_words`: It waits for the blitter to finish its current operation through another routine, then writes a shared variable into three consecutive words at a fixed address.
- `sync_blitter_and_store`: Writes a fixed value into a global variable, then reads two 32-bit words from a data structure and stores them into global variables.
- `initialize_and_load_data_block`: Initializes a block of global state by copying a fixed string into a table, then reads a header that identifies the type of data being loaded and sets up the appropriate structures.
- `count_nonzero_words_scaled`: It counts the number of non-zero words pointed to by a register, then returns that count plus one multiplied by four.
- `handle_hunk_type`: Checks a returned value, triggers a system call, stores a pointer, and if a field matches a value and another flag is set it jumps through a computed pointer.
- `count_flagged_dispatches`: Waits for a status flag to clear, then repeatedly calls the dispatch routine while counting how many times the flag remains set and stores that running count, and finally increments a counter in a global variable.
- `dispatch_flagged_events`: Waits for a status flag to clear, then repeatedly calls another routine while counting how many times the flag stays set, storing that running count, and finally writes a computed sequence to a table.
- `setup_dispatch_table`: Waits for a status flag to clear, then counts how many times it remains set while storing the running count, and finally writes a sequence of values into a table.
- `return_sentinel_stub`: The routine has a single instruction and no branch or call, so it simply returns the value already held in the D0 register.
- `loop_until_value_clears`: Loops calling another routine until the returned value's low thirty bits become zero, then returns the value that was originally in register D0.
- `return_stacked_value`: An empty routine that immediately returns the value left on the stack by its caller without touching any data.
- `locate_table_slot`: Walks a table of entries with nested loops to find the slot whose stored range contains a computed address, then sets up a chain of return addresses.
- `walk_list_and_setup`: Walks a linked list until it finds a zero entry, then reads fields from a data structure, stores them nearby, records return addresses, and returns.
- `setup_struct_and_return`: Reads fields from a data structure and stores them into nearby locations, then records two return addresses and calls another routine for each.
- `find_string_table_index`: Searches a table of null-terminated strings for one matching a given input string, and if none matches it returns a negative marker.
- `compare_string_table_entries`: Walks a table of fixed 32-byte records, each holding a null-terminated string, and compares one against an input string while treating letters case-insensitively.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `lookup_table_entry`: Indexes into a table of four-byte entries using a shifted register value as the offset and returns the located entry, while also touching the joystick and disk status port along with a color register.
- `reset_and_relink_table_entries`: Walks a table of linked lists and, for each entry, clears a field, records a pointer difference, and stores a computed address built from the base pointer plus that difference.
- `dispatch_handler_if_flag_set_2`: Checks a global flag and, when it is set, jumps through a computed pointer in a handler table to run a routine.
- `compare_string_table_entries_2`: Walks a table of fixed 32-byte records, each holding a null-terminated string, and compares one against an input string while treating letters case-insensitively.
- `locate_table_slot_and_setup_stack`: Walks a table of entries with nested loops to find the slot whose stored range contains a computed address, then sets up a chain of return addresses in a stack and stores several global values.
- `validate_table_entries_loop_2`: Walks a table of entries, comparing computed values and running a shared flag-draining sequence, returning early when a mismatch occurs.
- `store_pointer_difference`: Given a non-null pointer, records the difference between a supplied address and the pointer, then stores a computed address built from the pointer plus that difference.
- `conditional_loop_with_counter`: Initializes an accumulator to zero, compares an incoming value against a stored limit to possibly jump to a shared exit, then runs a helper routine in a loop that decrements a counter until it reaches negative one.
- `loop_until_counter_negative`: Runs a routine in a loop, decrementing a register each time until it reaches negative one, then returns.
- `validate_table_entries`: Walks a table of entries comparing computed values, and on a mismatch installs a jump table and sets several flags and registers before returning.
- `validate_table_entries_loop`: Walks a table of entries comparing computed values, running the shared flag-draining sequence and returning when a mismatch occurs.
- `process_flagged_event`: When a global flag is clear, it waits for that flag to be set, drains it by looping while it stays set, stores a computed address, then writes several values to consecutive words of a pointed-to buffer and updates several globals before returning.
- `process_flagged_event_2`: When a flag is clear it drains a status flag, records a return address, stores several return addresses, updates several globals, and then returns.
- `initialize_dispatch_table`: Waits for a status flag to clear, then counts how many times it remains set while storing the running count, and finally writes a sequence of fixed handler addresses into a table and initializes several global variables.
- `wait_and_dispatch_flags`: Disables interrupts and waits for a global counter to reach a target range, then invokes a series of stored routine pointers and a CPU trap.
- `initialize_and_dispatch_routine`: Stores a system pointer, runs several routines, copies a value into multiple global locations, zeroes a global byte, and then jumps through a stored routine pointer before returning.
- `configure_dispatch_structure`: Stores a routine address into a global and writes a sequence of fixed addresses into a structure, then sets several global flags and registers before returning.
- `invoke_indirect_routine`: Stores a supplied address into a global variable, then jumps through that stored address to invoke a routine at a fixed offset from it, and finally returns the original argument.
- `store_and_chain_handlers`: Saves incoming register values into a stack frame, records a chain of routine pointers and flag values, then invokes each handler in turn before returning the previously saved value.
- `store_and_run_handlers`: Stores incoming register values into a stack frame, then writes a sequence of routine pointers and flag values before invoking each stored handler.
- `set_flag_and_wait`: Writes a fixed value into a field of a structure, then spins in a loop until a specific bit within another field of that structure becomes set.
- `invoke_stored_routines`: Invokes two stored routine pointers in sequence and returns a value passed in a register.
- `copy_structure_bytes`: Copies two bytes from fixed offsets of one structure into matching offsets of another structure, sets a third field to a fixed value, and returns that stored value.
- `chain_two_routines_return_d0`: It invokes two other routines in sequence and then returns the value that was loaded into the D0 register before the calls.
- `copy_two_bytes_and_set_flag`: Copies two adjacent bytes from a source buffer into a destination buffer, then writes the value 127 into a third location and returns that value.
- `shift_and_return_flag`: Copies a global flag value into a location, then shifts a register value left by four bits and returns the result.
- `pack_status_flags`: Combines five processor status flag bits into a single byte by shifting each into its own position and ORing the results together.
- `initialize_parallel_port`: Writes the byte 255 to a hardware register on a parallel interface chip, initializing or enabling that port.
- `loop_until_status_clear`: Loops calling a routine that sets interrupts and reads a status value until that value becomes zero, then returns a register value.
- `dispatch_set_flags`: Reads a global flag byte, masks it against a second flag byte, then dispatches to each registered handler whose corresponding bit is set before restoring the flag state.
- `dispatch_bitmask_callbacks`: Checks a bitmask against a value and executes a series of function pointers from a table for every bit that is set.
- `process_flag_handlers`: Combines a global flag with a value passed in a register, masks it with another flag, then walks a table of routine pointers, invoking each handler whose corresponding bit is set.
- `dispatch_flagged_routines`: It inspects a status word and, for each of the lower nibble bits that are set, jumps through a table of routine pointers and executes the matching routine.
- `invoke_bit_dispatched_routines`: It walks a table of routine pointers, calling each entry whose slot corresponds to a bit that is set in the input register.
- `dispatch_set_flags_3`: Masks a global flag against another value, then walks a table of routine pointers and invokes each handler whose corresponding bit is set.
- `dispatch_flagged_routines_2`: Masks a set of flags against another value, then walks a table of routine pointers, invoking each entry whose corresponding bit is set.
- `dispatch_set_flags_2_2`: It masks a global flag against another value, then walks a table of routine pointers, invoking each one whose corresponding bit is set in the input.
- `dispatch_flagged_routines_3`: It masks a global flag against another value and then walks a table of routine pointers, invoking each entry whose corresponding bit is set in the masked value.
- `invoke_flagged_routines`: Iterates over a table of routine pointers, invoking each entry whose corresponding bit is set in a masked value derived from the input registers.
- `dispatch_set_flags_2`: Masks a global flag against another value, then walks a table of routine pointers invoking each one whose corresponding bit is set.
- `clear_global_memory`: Copies sixteen words from a zeroed source area into a global variable using a decrementing loop counter.
- `zero_global_memory`: Copies sixteen words from a global variable into a zeroed destination area using a decrementing loop counter.
- `store_pointer_global`: Stores a pointer passed in a register into a global location and returns.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 64 into d0, then returns.
- `index_and_clear_flag`: Reads a value, stores a register into a table indexed by that value, then clears a flag bit based on the same value.
- `clear_flag_bit`: Clears the bit indexed by the passed value in a global flag byte, treating negative indices as requests to clear the entire flag.
- `set_flag_bit_from_value`: If the passed value is negative it clears a global flag bit; otherwise it sets the bit at the position given by the low five bits of that value in the same global variable.
- `swap_and_mask_48bit`: It swaps the two 16-bit halves of a 32-bit pair, masks off the high bit of each half, and then sets the top bit of the resulting 48-bit value.
- `swap_and_mask_48bit_2`: It swaps the two 16-bit halves of a 32-bit pair, clears the high bit of each half, and sets the top bit of the resulting 48-bit value.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `set_interrupts_and_read`: It writes a global variable combining the interrupt-enable bit and the vertical-blank-disable bit, then reads that variable back and returns it together with the value passed in a register.
- `scan_array_for_value_2`: Advances a pointer past array entries until it finds one matching a specific value, then steps past that entry and repeats until a counter is exhausted.
- `scan_array_for_value`: Iterates over a bounded array, advancing a pointer past entries until it finds one matching a specific value, invoking a routine for each skipped entry.
- `process_tagged_data_blocks`: Processes a stream of tagged data blocks based on a type code, copying, zeroing, or relocating the referenced memory as needed.
- `process_hunk_types`: Handles AmigaDOS hunk type codes by returning immediately for executable and relocation types, and by copying, zeroing, and relocating hunk data for the remaining types.
- `store_scaled_table_value`: Computes a scaled offset from two stored values, indexes a table through a pointer, and writes a masked value plus a zero into the pointed location, then stores a computed offset when the scaling factor is nonzero.
- `initialize_system_state_2`: Initializes several global variables and pointers, zeroes a block of bytes, configures a hardware register, and returns a value built from a byte.
- `set_system_flags`: Writes fixed values into three global variables and returns a value built from one of them.
- `process_input_bits_2`: Checks a flag and a counter, shifting a block of bytes when a counter is nonzero, then looks up a table entry and stores a computed byte when a condition holds.
- `shift_global_array_down_2`: Decrements a global counter and shifts each element of a block of half-words one position earlier in a global array, then resets the counter to zero when it reaches one.
- `empty_return_routine_7`: Contains only a return instruction and performs no operations, reading or writing no data.
- `initialize_system_state`: Initializes several global variables and pointers, zeroes a block of bytes, configures a hardware register, and returns a value built from a stored byte.
- `process_input_bits`: Inspects individual bits of an input value and, when set, updates a global variable, modifies a hardware register, and writes computed bytes into a table.
- `shift_global_array_down`: Shifts a block of half-words down a global array by decrementing a counter and copying each element forward, then clears the counter when a flag register is nonzero.
- `empty_return_routine_4`: This routine performs no operations; it contains only a return instruction and does not read or write any data.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d5.
- `clear_three_global_variables`: Clears three consecutive global variables to zero and returns the original contents of a data register unchanged.
- `zero_three_globals`: An empty routine that zeroes three global variables and returns without performing any further operations.
- `swap_globals_and_scale_delta`: Swaps two stored word values, writes a fixed mask into a global, derives flag bits from it, then computes a signed difference scaled by a divisor and adds the result to another stored value.
- `call_twice_return_byte`: Invokes two other routines in sequence, then returns the byte value currently held in a global variable.
- `store_registers_to_globals`: Stores three incoming register values into three global variables and returns without performing any further operations.
- `init_globals_from_registers`: Initializes a set of global variables from the incoming word values in the registers, then returns.
- `add_scaled_difference`: Computes the signed absolute difference between two input values, scales it by a global factor, and adds the result to the value stored through a pointer.
- `store_maximum_of_pointers`: Replaces the value at the first pointer with the larger of the two pointed values, then with the second value if that is greater, effectively storing the maximum back through the pointer.
- `copy_variable_length_block`: Copies a variable-length byte block from a source pointer to a destination pointer, using a length encoded in a control byte.
- `tiny_ori_3`: Small routine of 1 instruction that merges 63 into d0.
- `tiny_ori_4`: Small routine of 1 instruction that merges 199 into d0.
- `copy_byte_block`: Copies a variable-length byte block from a source buffer to a destination buffer using a small opcode table that selects how many bytes to move and how to advance the pointers.
- `tiny_ori_5`: Small routine of 1 instruction that merges 20 into d0.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 1 into d1, then merges 1 into d2.
- `copy_backward_and_dispatch`: Copies a block of bytes backward from one buffer to another in fixed-size chunks, then reads a byte from the source to compute a table index and dispatches to a routine based on that value.
- `dispatch_from_masked_index`: Derives a table index from a masked register value and calls the matching entry, falling back to a second table when the index is zero.
- `dispatch_from_offset_index`: Builds a table index from a masked register value offset by a constant, then calls the matching entry or a fallback table.
- `dispatch_from_table_offset`: Computes a small index from a register value, adds three, and if the result is nonzero calls the routine found in a jump table at that offset.
- `call_table_entry_offset`: Calls a single entry from a jump table using a fixed negative offset, then returns the result.
- `dispatch_from_flagged_index`: Combines a register value with a flag bit to form a table index, then calls the matching entry or a fallback table.
- `route_by_masked_index`: Adds a constant to the low eleven bits of a register and, when the result is nonzero, indexes a jump table using a shifted byte value and calls the entry.
- `route_dual_index_call`: Decodes two small indices from a register value, then branches to a table-driven routine using a fallback table when both indices are zero and a second table otherwise, calling the selected function pointer and returning its result.
- `dispatch_from_table`: Computes a table index from input values, then either returns a value or calls a function pointer retrieved from a jump table based on that index.
- `copy_backward_then_dispatch`: Copies a block of bytes backward through two pointers in chunks, then dispatches to a table entry based on a register value.
- `open_cached_library_once`: On first use it opens a cached library through a stored function pointer and saves the returned handle in a global variable, then returns the two incoming data register values unchanged.
- `close_cached_library_when_open`: When the cached library handle is non-zero it closes that library through a stored function pointer and clears the handle, then returns the two incoming data register values unchanged.
- `run_conditional_helper_sequence`: It invokes a series of helper routines, and when one of them reports a non-negative result it runs two further helpers before returning.
- `init_and_invoke_cached_library`: Opens a library through a stored function pointer on first use and caches the returned handle in a global variable.
- `invoke_routines_from_cached_handle`: It calls a helper, reads a cached handle, and when the result is non-negative it invokes two routines through pointers derived from that handle before returning.
- `open_cached_library`: Opens a library through a stored function pointer on first use, caching the returned handle in a global variable.
- `close_cached_library`: Closes a previously opened library when its handle is non-zero, then clears the cached handle.
- `open_library_and_check`: Opens a library through a stored function pointer, stores the handle, and returns whether the operation succeeded.
- `run_pending_cleanup_hook`: Checks a global flag and, when it is set, jumps through a function pointer stored in a table to run a cleanup routine before clearing the flag.
- `invoke_handler_if_flag_set`: Checks a global flag and, when it is set, jumps through a computed pointer stored in another global variable to invoke a handler routine.
- `dispatch_handler_if_flag_set`: Checks a global flag and, when it is set, jumps through a computed pointer in a handler table to run a routine.
- `invoke_handler_if_active`: Reads a value from a global variable and, if it is nonzero, jumps to a handler address stored in a table to execute it.
- `occupy_handler_slot`: Occupies a single instruction slot in a contiguous block of handler entry points and is invoked by ten other routines via taken addresses.
- `occupy_handler_slot_two`: Consists of a single instruction and is reached through taken addresses plus a handler pointer, being called by seven unnamed routines.
- `occupy_handler_slot_three`: A one-instruction routine entered via a taken address and a handler pointer, invoked by a single unnamed caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `describe_single_instruction`: Contains a single instruction that decodes as bad instruction data, truncating control flow immediately after entry.
- `do_nothing_2`: The routine contains no instructions; it is empty.
- `empty_routine_2`: The routine contains no executable instructions; it is empty.
- `halt_baddata`: A single instruction that is not valid 68000 code, so the disassembler cannot interpret it and control flow stops here.
- `mark_empty_routine_2`: The routine contains no executable instructions; it is empty.
- `empty_routine_stub_3`: It is an empty routine that returns immediately without performing any operations.
- `describe_unreached_instruction`: A single unreached instruction that decodes as bad instruction data and truncates control flow upon entry.
- `mark_empty_routine_3`: The routine contains no executable instructions; it is an empty routine.
- `is_empty_routine_2`: This routine is empty, containing only a single instruction that decodes as invalid data.
- `empty_routine_3`: The routine contains no executable instructions; it is an empty routine.
- `invalid_byte_gap`: The routine contains no valid instructions; the bytes at this location decode as an invalid opcode.
- `empty_routine_4`: Contains no executable instructions; the routine is empty.
- `mark_unreached_gap`: This routine contains no actual executable code; it is a one-byte gap that only decodes as code because it falls between other routines.
- `mark_empty_routine_4`: The routine contains no executable instructions; it is completely empty.
- `store_table_byte`: Reads a single byte from a table using an index held in a register and stores the resulting byte through a pointer.
- `run_empty_stub`: It runs a sequence of sixteen instructions that perform no data operations and return without modifying memory.
- `return_empty`: This routine returns immediately without modifying memory or performing any data operations.
- `empty_routine_38`: Contains no instructions and performs no operations.
- `noop_data_word`: The routine contains no executable instructions; it is a single data word with no code to execute.
- `empty_routine_5`: The routine contains no valid instructions; it is an empty routine.
- `empty_routine_6`: The routine contains no instructions; it is empty.
- `return_immediately_9`: This routine returns immediately without executing any instructions or modifying any memory.
- `empty_routine_7`: The routine contains no executable instructions; it is empty.
- `empty_return_routine_10`: An empty routine that contains only a return and performs no operations.
- `empty_routine_8`: The routine contains no executable instructions; it is empty and performs no operations on any data.
- `empty_data_gap`: This routine contains no executable instructions; it is an empty gap that decodes as code.
- `empty_routine_9`: The routine contains no executable instructions; it is empty.
- `emit_illegal_trap`: The routine executes a single illegal instruction (trapv) as its only operation, with no further instructions, no memory access, no library calls, and no visible caller.
- `mark_empty_routine_5`: The routine contains no executable instructions; it is an empty routine with no code to describe.
- `noop_routine`: Contains no executable code; the routine is empty.
- `empty_code_gap`: The routine contains no executable instructions; it is an empty gap that decodes as code.
- `empty_routine_10`: The routine contains no executable instructions; it is an empty routine with no code to execute.
- `empty_routine_11`: The routine contains no executable instructions; it is empty.
- `mark_empty_routine_6`: This routine is empty, containing no instructions to execute.
- `empty_routine_12`: The routine contains no executable instructions; it is empty.
- `decode_spurious_instruction`: Decodes as a single instruction that is never reached because the surrounding bytes are data rather than executable code.
- `mark_empty_routine_7`: The routine contains no executable instructions; it is a single empty routine with no code, no library calls, no register access, and no data manipulation.
- `invalid_data_gap`: The routine contains no valid instructions; the bytes at this location do not decode as executable 68000 code.
- `halt_baddata_2`: The routine contains no executable instructions; it consists solely of an instruction that the disassembler could not decode as valid code.
- `mark_empty_routine_8`: An empty routine that contains no instructions to execute.
- `empty_data_word`: The routine contains no executable instructions; it is a single data word with no code body.
- `store_nothing`: The routine contains no executable instructions; it is a single data word with no code.
- `halt_baddata_3`: The routine contains no valid instructions; the code segment here holds non-executable data rather than a real routine, so nothing is executed or modified.
- `mark_empty_gap`: This is an empty routine consisting of a single bad instruction that decodes as invalid code, so it performs no operations.
- `return_immediately_4`: A single-instruction routine that immediately returns without touching any registers, memory, or hardware.
- `halt_at_bad_data`: The routine begins with a single instruction that is immediately followed by data which does not decode as valid code, so the executable's control flow cannot continue past it.
- `mark_empty_block`: The routine contains no valid instructions; it is an empty block that executes nothing.
- `mark_empty_routine_9`: The routine contains no executable instructions; it is empty.
- `mark_empty_routine_10`: The routine contains no executable instructions; it is completely empty.
- `mark_empty_routine_11`: The routine contains no executable instructions; it is a zero-length stub.
- `mark_empty_routine_12`: The routine contains no executable instructions; it is a completely empty routine.
- `store_nothing_else`: This routine contains no executable instructions; the bytes at this location are data rather than code.
- `mark_empty_routine_13`: The routine contains no executable code; the analyzer only encounters bad instruction data at its entry and stops, so nothing is read, written, or called.
- `decode_bad_data`: This is an empty routine consisting of a single instruction that decodes as bad instruction data, so it performs no observable action.
- `empty_routine_stub_4`: It contains no instructions and returns immediately without reading or writing any memory.
- `empty_routine_13`: This routine is empty, containing no instructions.
- `empty_routine_14`: The routine contains no executable instructions; it is an empty routine that does nothing.
- `empty_routine_15`: The routine contains no executable instructions; its body is empty and it performs no operations on any data.
- `empty_data_gap_2`: This routine is empty; it contains no instructions and performs no operations on any data.
- `halt_baddata_4`: The routine contains no executable instructions; it is a single instruction that decodes to invalid data, so the disassembler flags it as bad instruction data and truncates control flow.
- `empty_routine_noop`: The routine contains no executable instructions; it is a single data word with no code.
- `halt_baddata_5`: The routine contains no valid instructions; it begins immediately with data that the disassembler cannot decode as code, so it executes nothing.
- `bad_data_only`: The routine contains no valid instructions; it consists solely of invalid or bad instruction data.
- `halt_baddata_6`: The routine contains no valid instructions; the bytes at this location decode as an invalid opcode and are treated as data rather than executable code.
- `copy_byte_via_pointer`: Reads a byte from a global variable and stores it through a pointer.
- `empty_routine_16`: The routine contains no executable instructions; the referenced bytes are treated as data rather than code, so nothing is performed.
- `store_bad_data`: The routine contains a single instruction that is not valid 68000 code; the bytes are data rather than executable instructions.
- `empty_routine_17`: The routine contains no executable instructions; it is a single empty entry point that performs no operations on any data.
- `mark_empty_routine_14`: The routine contains no executable instructions; it is a completely empty routine with no code body.
- `store_unused_data`: The routine contains no decodable instructions; the bytes at this location are data rather than executable code, so nothing is read, written, or called.
- `halt_baddata_7`: The routine contains no valid instructions; the bytes at its location are non-executable data, so the disassembler cannot recover a real body and the routine is effectively empty.
- `halt_baddata_8`: The routine consists of a single instruction that is data rather than executable code, so it cannot be decoded as a normal operation.
- `halt_baddata_9`: A single wordless instruction is located immediately after the preceding routine ends.
- `store_bad_data_2`: The routine contains no executable instructions; it holds only data that the disassembler could not decode as valid 68000 code.
- `execute_single_instruction`: The routine contains a single instruction and no further code.
- `execute_single_instruction_2`: The routine contains a single instruction and no other code.
- `mark_unreached_gap_2`: The routine contains a single instruction that is not valid 68000 code; it is unreached data that only decodes as an instruction because it sits in a gap between real routines.
- `halt_baddata_10`: The routine contains a single instruction that is not valid 68000 code, so it cannot be decoded as a meaningful operation.
- `execute_single_instruction_3`: The routine contains a single instruction and no other code.
- `mark_bad_data`: The routine contains a single instruction that is not valid 68000 code; it is effectively an empty routine with no executable behaviour.
- `flag_bad_instruction`: The routine contains a single instruction that is data rather than executable code, so it cannot be decoded as a meaningful operation.
- `empty_routine_18`: The routine contains a single instruction and no other code, so it performs no data manipulation, library calls, or register access.
- `copy_byte_across_memory`: The routine contains a single instruction that reads a byte from a memory location and stores it back to another memory location.
- `halt_baddata_11`: The routine contains a single instruction that decodes to invalid 68000 data and is never reached from any caller.
- `halt_baddata_12`: The routine contains no executable instructions; it holds only data bytes that the disassembler cannot decode as valid 68000 code.
- `contains_non_code_data`: The routine contains no executable instructions, only data that the decompiler cannot interpret as code.
- `store_invalid_bytes`: The routine contains no valid instructions; the bytes at this location are data rather than executable code, so nothing is executed or modified.
- `identify_baddata`: The routine contains only a single instruction that is not valid 68000 code, so it is a data segment misinterpreted as code rather than an executable routine.
- `empty_routine_19`: The routine contains no instructions; it is empty.
- `store_no_instructions`: The routine contains no executable instructions, only data that the decompiler could not interpret as code.
- `empty_data_segment`: The routine contains no executable instructions, only data that the disassembler could not decode.
- `empty_routine_20`: The routine contains no executable instructions; the code segment here holds only non-code data.
- `empty_data_gap_3`: The routine contains no executable instructions; it is a gap of non-code data that the disassembler misidentifies as code.
- `misaligned_data_region`: The routine contains only a single instruction that is not valid 68000 code, so it is a data region misinterpreted as executable rather than a real routine.
- `mark_empty_routine_15`: This routine contains no instructions; it is empty.
- `empty_routine_21`: This routine is empty, containing no instructions.
- `recovered_no_code`: No usable instructions were recovered in this routine; the surrounding bytes appear to be data rather than executable code, so nothing can be said about what it does.
- `mark_empty_routine_16`: This routine contains no executable instructions; it is a single empty routine with no code, calls, or data operations.
- `empty_gap_data`: This routine is empty, containing no instructions.
- `hit_bad_data`: Encounters an immediate byte of 92 (backslash) at the start of the routine, which the disassembler flags as bad instruction data and truncates control flow.
- `unreferenced_data_gap`: This routine is a gap of unreferenced instruction data that the disassembler cannot interpret as valid code.
- `empty_data_region`: The routine contains no executable instructions; only non-code data is located at this address.
- `halt_single_instruction`: This routine is an isolated one-instruction gap that decodes as code but is never reached.
- `halt_baddata_13`: The routine contains no valid instructions; the code segment here is data, not executable code.
- `is_empty_routine_3`: This routine is empty; it contains no instructions and performs no operations.
- `empty_routine_22`: This routine is empty.
- `halt_single_instruction_2`: A single instruction is located at a spot where the disassembler could not decode valid code, so the routine contains only one instruction and no further logic.
- `misaligned_data_gap`: This is not a real routine; it is a gap of unreferenced data that the disassembler misinterprets as code.
- `mark_empty_routine_17`: An empty routine with no executed instructions.
- `empty_data_region_2`: The routine contains no executable instructions; the code segment here holds only non-code data, so nothing is performed.
- `noop_routine_2`: The routine contains no executable instructions; it is a single empty routine with no data access or library calls.
- `empty_routine_23`: The routine contains no executable instructions; it is a zero-length stub that performs no operations on any data.
- `empty_gap_routine`: This routine contains no executable instructions; it is a one-byte gap that only decodes as code because of the surrounding disassembly, so it performs no operation on any data.
- `empty_routine_stub`: The routine contains no executable instructions; it is an empty stub with no code, calls, registers or data operations.
- `is_empty_routine_4`: This routine is empty; it contains no instructions, only bad instruction data that the disassembler decoded as a single instruction.
- `empty_routine_24`: This routine contains no executable instructions; it is a single empty entry point that performs no operations.
- `halt_baddata_14`: The routine contains no valid instructions; the bytes at this location decode as an invalid opcode and are treated as data rather than code.
- `halt_baddata_15`: The routine contains no executable instructions; it holds only data that the disassembler cannot decode as valid code.
- `empty_routine_25`: The routine contains no executable instructions; it is an empty routine with no code, data references, library calls, or register access to report.
- `empty_data_region_3`: The routine contains no valid instructions; it is a single data region with no executable body.
- `halt_baddata_16`: The routine contains no valid instructions; the code segment here holds non-executable data rather than machine code.
- `unreachable_gap_data`: This routine is only unreachable gap data that a disassembler happens to decode as an instruction; it performs no operations and contains no verifiable behaviour.
- `execute_trap_abort`: Executes a trap instruction that aborts control flow.
- `empty_data_word_2`: The routine contains no executable instructions; it is a single data word with no code.
- `is_empty_routine_5`: This routine is empty, containing only a single instruction that performs no operation.
- `mark_data_region`: Contains no instructions; the bytes in this range are data rather than executable code.
- `store_invalid_bytes_2`: The routine contains only a single instruction that is not valid 68000 code, so the bytes at this location are data rather than executable instructions.
- `unreached_gap_bytes`: This routine is a one-byte gap that decodes as code but is never reached.
- `empty_routine_26`: The routine contains no executable instructions; it is a single empty routine that does nothing.
- `halt_baddata_17`: The routine contains no valid instructions; the bytes at this location decode as an invalid opcode.
- `mark_empty_block_2`: The routine contains no executable instructions; it is a single empty block that performs no operations on any data.
- `empty_noncode_routine`: The routine contains no executable instructions; it consists solely of non-code data, so it performs no operations on any data or registers.
- `halt_with_trap`: A single illegal instruction (trap #v) is executed, which the disassembler flags as bad instruction data.
- `is_empty_routine_6`: The routine contains no instructions; it is empty.
- `empty_data_segment_2`: The routine contains no valid instructions; the code segment here holds only non-executable data rather than any operation.
- `empty_routine_27`: The routine contains no executable instructions; it is empty and performs no operations.
- `mark_bad_data_2`: The routine contains no executable instructions; it holds only non-code data that the disassembler flagged as invalid.
- `empty_routine_28`: The routine contains no executable instructions; it is an empty routine with no data to process.
- `contains_no_code`: The routine contains no valid instructions; the code segment here holds only non-executable data rather than any code.
- `decode_gap_instruction`: This routine is a single instruction that decodes from stray data in a gap between code segments rather than being called from anywhere.
- `empty_routine_29`: The routine contains no executable instructions; the code segment holds only non-instruction data at this location.
- `empty_routine_30`: This routine contains no executable instructions, only data that the decompiler flagged as invalid.
- `decode_gap_data`: This routine is a gap of unreferenced data that Ghidra misinterprets as code; it contains no genuine instructions, makes no library calls, and touches no registers or memory.
- `halt_baddata_18`: The routine contains no executable instructions; it holds only data that disassembly tools flagged as invalid, so nothing is actually performed.
- `halt_baddata_19`: This routine is empty, containing no instructions to analyze.
- `empty_stub_2`: The routine contains no executable instructions; it is an empty stub with no operations performed on any data.
- `empty_datablock`: The routine contains no executable instructions; the listed instructions are invalid data rather than code, so the routine performs no actions.
- `misaligned_gap_data`: This routine is not real code.
- `do_nothing_3`: The routine contains no instructions; it is empty.
- `mark_data_region_2`: The routine contains no executable instructions; it is a pure data region with a single bad-instruction marker and no code body.
- `decode_gap_instruction_2`: This routine is a single instruction that decodes from stray data in an unreached gap rather than from real code, so it performs no meaningful operation.
- `empty_data_region_4`: The routine contains no executable instructions; it holds only data that was misinterpreted as code.
- `halt_baddata_20`: A single instruction that is not valid 68000 machine code, so it cannot be decoded as an operation.
- `skip_bad_bytes`: This routine contains no executable instructions; the bytes at this location decode as an invalid 68000 opcode and are simply skipped.
- `empty_routine_31`: The routine contains no instructions and performs no operations.
- `halt_baddata_21`: The routine contains no executable instructions; it is a block of data flagged by the disassembler as invalid code rather than a functioning routine.
- `empty_data_gap_4`: The routine contains no executable instructions; it is a gap that decodes as code but performs no operations.
- `mark_noncode_data`: The routine contains no executable instructions; it holds only non-code data.
- `halt_baddata_22`: The routine contains no valid instructions; the code segment holds only non-executable data at this address.
- `halt_baddata_23`: This routine contains no valid instructions; the bytes at this location decode as an invalid 68000 opcode.
- `halt_baddata_24`: The routine contains no executable instructions; it consists solely of a single data word that the disassembler cannot decode as valid 68000 code.
- `empty_data_word_3`: The routine contains no executable instructions; it is a single data word with no code.
- `empty_routine_32`: The routine contains no executable instructions; the region holds only non-code data.
- `halt_baddata_25`: The routine contains no valid instructions; the bytes at this location decode as an invalid opcode.
- `copy_table_byte`: Reads a single byte from a table whose address is passed in a register and stores it into a byte addressed by another register.
- `halt_baddata_26`: The routine contains no recognizable instructions; it consists solely of unrecognized data bytes that Ghidra flags as bad instruction data, so it performs no detectable operation on any data or registers.
- `empty_data_gap_5`: This routine is empty, containing no instructions.
- `mark_empty_routine_18`: The routine contains no instructions; it is an empty routine.
- `empty_gap_code`: This routine contains no executable instructions; it is a gap in the binary that the disassembler decodes as code.
- `empty_routine_33`: The routine contains no instructions; it is an empty routine with no code, no data access, and no library calls.
- `halt_baddata_27`: The routine contains no valid instructions; the bytes at this location decode as an invalid opcode and the code is never reached.
- `flag_bad_data_region`: The routine contains no decodable instructions; the bytes at its location are non-executable data, so nothing is executed or modified.
- `decode_gap_data_2`: This routine does not contain executable code; it is a gap in the runtime segment that Ghidra decodes as instructions.
- `read_bad_instruction`: The routine contains only a single instruction that does not correspond to valid 68000 code, so the surrounding data cannot be decoded as executable instructions.
- `decode_invalid_data`: This routine contains a single instruction that decodes as invalid 68000 data.
- `halt_baddata_28`: The routine contains no valid instructions; the bytes at this location decode as an invalid opcode and the code is never reached.
- `mark_empty_routine_19`: This routine contains no executable instructions; it is an empty routine.
- `halt_on_bad_data`: The routine contains only a single instruction that appears to be data rather than executable code, so it does not perform any readable operation on memory.
- `empty_routine_34`: This routine is empty, containing no instructions or data.
- `be_an_empty_stub`: This routine is an empty stub: it contains no executable instructions and performs no operations on any data.
- `store_nothing_else_2`: This routine contains no executable instructions; the bytes at this location are data rather than code, so there is nothing for it to do.
- `store_bad_data_3`: This routine contains no executable instructions; it holds only bad instruction data.
- `halt_baddata_29`: The routine contains no executable code, only an invalid or non-executable instruction.
- `empty_byte_gap`: This routine contains no executable instructions; it is a one-byte gap that decodes as code only because it sits between other routines.
- `is_empty_data`: The routine contains no valid instructions; the bytes at this location are not executable code but data.
- `halt_baddata_30`: The routine begins by executing a single instruction that the disassembler cannot decode as valid 68000 code, which causes the decompiler to stop immediately and emit a truncation warning.
- `false_code_gap`: This routine is not real code.
- `empty_routine_35`: This routine is empty and contains no instructions, no library calls, no register or memory accesses, and no referenced text.
- `no_op_routine`: This routine is empty: it contains no instructions, no library calls, no register or memory access, and no referenced text.
- `empty_gap_routine_2`: The routine contains no executable instructions; it is an empty gap that decodes as code but performs no operations.
- `halt_baddata_31`: The routine contains no valid executable instructions; the disassembler flags the region as bad instruction data and the decompiler truncates control flow immediately.
- `call_handler_and_return`: Calls a handler routine through a pointer and returns the value left in a register by that handler.
- `call_and_return`: Calls another routine and returns its result.
- `forward_routine_result`: Calls another routine and returns its result.
- `call_and_return_routine`: Calls another routine and returns its result.
- `forward_routine_result_2`: Invokes another routine and forwards its return value back to the caller.
- `delegate_to_helper`: Calls another routine and returns its result.
- `increment_a6_offset32`: Reads a 16-bit value located 32 bytes past a base pointer held in register A6 and increments it by one.
- `describe_as_empty`: Contains two instructions that decode to a no-op sequence followed by an immediate return, so it performs no operations other than returning.
- `decrement_a6_offset32`: Reads a 16-bit value located 32 bytes past a base pointer held in register A6 and decrements it by one.
- `increment_word_at_offset`: It increments a 16-bit value located at offset 32 from a base pointer held in register A6.
- `empty_routine_36`: The routine contains no executable instructions; the disassembler reports a bad instruction and truncates control flow at the entry point, so the routine performs no operations on any data, registers, or memory.
- `halt_baddata_32`: This routine is a single instruction that is not valid 68000 code.
- `recover_data_block`: The routine's disassembly contains non-instruction data rather than valid 68000 code, so no executable instructions or referenced data can be recovered.
- `halt_single_instruction_3`: The routine contains a single instruction and then stops, doing nothing else.
- `empty_routine_stub_2`: The routine executes a sequence of sixteen instructions that ultimately perform no data operations, ending in a return without modifying memory or registers.
- `empty_return_routine_8`: A routine that contains exactly three instructions and performs no operations, returning immediately to its caller.
- `empty_return_routine_5`: The routine contains eight instructions that decode to a no-op sequence followed by an immediate return, so it performs no operations other than returning to the caller.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a5, then returns.
- `store_byte_pointer`: Stores a byte from a register through a pointer, then loads a byte from that same pointer into a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_global_to_ff`: The routine stores the byte value 255 into a global variable and then returns to the caller.
- `return_global_word`: The routine returns the 16-bit value held in a global variable to the caller.
- `read_byte_global`: Reads a single byte from a global variable and returns it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_constant_and_loop`: It stores the constant 32767 into a global variable, invokes a callback through a function pointer, issues a software trap, and then loops forever calling callbacks from a table indexed by a shifting register.
- `invoke_software_trap`: The routine invokes a 68000 software trap via a trap instruction and then returns to the caller.
- `invoke_structured_callbacks`: The routine conditionally copies a global variable into another global, calls two routines through pointers stored in a structure, saves register values into globals, calls a third routine through a pointer, clears a global, and increments a counter before returning.
- `discard_and_return_registers`: The routine discards the value in a register by looping a number of times equal to its low byte and then returns the original register values to the caller.
- `loop_until_absolute_diff_large_enough`: Computes the absolute value of the difference between two passed values and repeats that subtraction until the result is not smaller than the other passed value.
- `save_values_and_fire_traps`: Stores a passed 32-bit value and a passed pointer into a global variable, then issues three hardware trap calls while saving the high half of the value and two derived constants into globals.
- `store_self_as_callback`: Stores the address of itself into a function pointer located 70 bytes past a global variable, then returns.
- `load_words_from_offset_pointer`: Reads two consecutive 32-bit words from fixed offsets past a passed pointer and stores each into a separate global variable.
- `return_immediately_5`: Contains no instructions and simply returns to the caller without touching any data.
- `fire_trap_loop_callback`: Calls several helper routines, issues a hardware trap, then loops calling a callback until a global flag becomes nonzero.
- `copy_constants_and_loop_callback`: Copies twelve 16-bit words into a destination and then repeatedly calls a callback through a table entry.
- `merge_constants_and_copy`: It merges several constant bit patterns into a register, copies twelve 16-bit words into a destination, and then repeatedly calls a callback before returning.
- `copy_words_and_loop`: Copies twelve 16-bit words from a source pointer into a destination pointer starting two words past a passed address, then repeatedly calls another routine until a register value becomes zero.
- `copy_twelve_words_pointers`: Copies twelve 16-bit words from a source pointer into a destination pointer that starts two words past a passed address, advancing both pointers each iteration.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d1, then merges 104 into d1, then merges 208 into d1.
- `store_pending_value_and_increment`: Sets a flag in a global variable and stores a value from a second global into a local slot.
- `invoke_callbacks_and_flag`: Reads a global flag and, when set, moves its value into another global and clears the flag, then invokes two callbacks through pointers stored in a structure.
- `run_once_while_positive`: Performs a single iteration of a loop guarded by a passed value and returns both passed values combined into a single 64-bit result.
- `store_repeated_value_pair`: Writes the same 16-bit value into two consecutive 32-bit slots of a structure pointed to by a passed address, then stores that value at a fixed memory location and a nearby location, and finally writes a different 16-bit value into another fixed location.
- `store_incremented_value_pair_2`: Writes a passed 32-bit value as two consecutive 16-bit halves into a destination starting two words past a passed address, then advances that destination pointer by those two words.
- `store_incremented_value_pair`: Writes a passed 32-bit value as two consecutive 16-bit halves into a destination starting two words past a passed address, advancing that destination each iteration while adding a passed increment to the value.
- `store_value_and_fire_traps`: Stores a passed 32-bit value and its high half into global variables, then issues two hardware trap calls and saves a derived constant into a global.
- `store_self_as_pointer`: It writes the address of itself into a function pointer located at a fixed offset from a global variable, then returns to the caller.
- `decode_compressed_stream`: Decodes a compressed byte stream by reading marker bytes and copying literal or back-referenced data into an output buffer.
- `decode_and_copy_buffer`: Reads a byte from a source buffer and, depending on its value, copies a computed number of bytes from a second source into a destination buffer.
- `decode_and_copy_compressed_bytes`: Copies a run of bytes from one location to another using a compressed length encoding, decoding either a small one-byte length or a larger two-byte length and then copying that many bytes.
- `dispatch_by_size_jumptable_2`: It derives a size from a register and, when nonzero, indexes a jump table by a shifted form of that value to invoke a callback.
- `dispatch_by_combined_size`: It derives a size from two registers, dispatches to a callback through a jump table when nonzero, and otherwise compares two values before falling through to another indexed callback.
- `dispatch_by_computed_size`: It derives a size from a register and dispatches to a callback through a jump table when that value is nonzero.
- `call_callback_directly`: It reads a value from a register and immediately calls a callback through a jump table, returning the result.
- `dispatch_indexed_routine`: It derives a small index from a register value combined with a flag bit, adds four, and if that result is nonzero calls a routine from a jump table indexed by the low three bits.
- `dispatch_by_computed_size_2`: Adds five to the low eleven bits of a value and, when the result is nonzero, indexes a dispatch table using the low three bits to call a size-based callback.
- `dispatch_by_two_sizes`: It derives two sizes from a register, dispatches through one jump table when either is nonzero, and otherwise compares two values before falling through to another indexed callback.
- `dispatch_by_size_jumptable`: Computes two sizes from register values, then dispatches to one of several callbacks through a jumptable based on those sizes.
- `copy_bytes_dispatch_size`: Copies a block of bytes from one memory location to another, decrementing both pointers by eight each iteration, then branches through a size-based jumptable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `do_nothing_4`: The routine contains no instructions and does nothing; it returns immediately without touching any data, registers, memory, or hardware.
- `empty_routine_37`: The routine contains no instructions and performs no operations; it returns immediately without reading or writing any data, registers, or hardware.
- `do_nothing_5`: The routine contains no instructions and performs no operations; it returns immediately without modifying any data or registers.
- `do_nothing_6`: The routine contains no instructions and performs no operations; it returns immediately without reading or writing any data.
- `empty_return_routine_6`: An empty routine that performs no operations and returns immediately.
- `do_nothing_7`: The routine contains no instructions and performs no operations; it is empty.
