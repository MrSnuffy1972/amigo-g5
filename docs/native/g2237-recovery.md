# Game 2237 (wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02237). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 51 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 26 of 26 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 277 of 277 named (0 author symbols), 277 of 277 described (40 by rule, 31 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1639 of 1639 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8454 of 8463 instructions identical in start and length, 2 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 16316 |
| code | 25550 |
| data | 841647 |
| padding | 14023 |
| unused | 3584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Setmap | 13 | 653 | 31 |
| Else | 2 | 117 | 2 |
| isdosdev | 7 | 45 | 2 |
| Type | 1 | 426 | 2 |
| SetPatch | 24 | 833 | 45 |
| addbuffers | 2 | 117 | 2 |
| multidie | 7 | 45 | 2 |
| wipeout | 7 | 45 | 2 |
| remove | 7 | 45 | 2 |
| EndCli | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| lab | 1 | 2 | 1 |
| Cls | 2 | 20 | 1 |
| FastMemFirst | 7 | 207 | 6 |
| CD | 2 | 117 | 2 |
| mouse | 2 | 4 | 1 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| memavail | 7 | 45 | 2 |
| setpatchV38.31 | 2 | 893 | 76 |
| Version | 11 | 574 | 18 |
| d | 1 | 13 | 1 |
| gb | 1 | 4 | 1 |
| Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:InstallWingCommander | 2 | 3734 | 67 |
| Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:Wing | 8 | 45 | 2 |
| Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:bootblock | 1 | 126 | 2 |

## Cases handled

- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:PLANET0.PKT: high-entropy data file (7.70 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:ENDGAME.PKT: high-entropy data file (7.92 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:FONTS.PKT: high-entropy data file (7.88 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:PCSHIP03.PKT: high-entropy data file (7.95 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:PCSHIP01.PKT: high-entropy data file (7.96 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:MIDGAME2.PKT: high-entropy data file (7.94 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:DEMO.PKT: high-entropy data file (7.97 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:MIDGAME0.PKT: high-entropy data file (7.96 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:PCSHIP02.PKT: high-entropy data file (7.93 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:DEMOSAMP.PKT: high-entropy data file (7.97 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:MIDGAME3.PKT: high-entropy data file (7.93 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:PCSHIP00.PKT: high-entropy data file (7.96 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:MIDGAME1.PKT: high-entropy data file (7.91 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:c/multidie: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:Wing: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Setmap

- `main_parse_cli_workbench_args`: Program entry point.
- `release_task_resources_and_return`: Stores its argument in a slot of a per-task record, then closes or frees each resource whose field is set through library calls, replying to a message port where one is held.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `forward_conditional_global_updates`: Forwards up to four global values to a subroutine, each guarded by a flag, then forwards a fixed global and the incoming parameter to the same subroutine.
- `store_pair_call_setup_helpers`: Stores its two parameters in two globals, calls a subroutine with eight arguments that include a fixed size pair, then calls a second subroutine with a constant.
- `load_keymap_icon_or_argument`: Opens the console, icon, intuition and keymap libraries, takes the keymap name from a tool-type entry or the command line, and loads the matching keymap file from the devices directory.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `forward_global_param_slot`: Passes a global value, its argument and the address of an eight-byte stack slot to another routine and returns.
- `call_library_vector_from_global`: Calls a library function through a base pointer read from a global, at a fixed negative offset, and returns with nothing else.
- `format_text_stack_send`: Formats text into a 140-byte stack buffer through the exec formatting routine, measures its length by scanning for the terminator, and passes the buffer and length to another routine when it is non-empty.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Else

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If a destination pointer is non-null, copies sixteen longwords from a fixed block into it, then calls a function pointer held in a register.

### isdosdev

- `startup_loop_then_restore_word`: Calls subroutines and a function pointer, runs a short loop of subroutine calls, clears a word, then restores a saved value to a fixed location when a process field matches a specific pattern.
- `busy_wait_countdown_loop`: Counts a local value down in a loop until it passes below zero, a plain busy-wait delay, then returns.

### Type

- `type_command_dump_files`: Entry point of a file-typing command.
- `write_hex_dump_lines`: Reads bytes one at a time and writes them as hex digits in lines that begin with a four-digit offset, using a digit table and dos output calls, until the input ends.

### SetPatch

- `start_setpatch_cli_or_workbench`: Program entry point for the patch command.
- `release_task_resources_return_b`: Stores its argument in a slot of a per-task record, then closes or frees each resource whose field is set through library calls, replying to a message port where one is held.
- `conditionally_forward_parameter`: Calls one subroutine, then when a global flag is set passes its result and the incoming parameter to a second subroutine, and finally calls a third subroutine.
- `install_exec_vector_patches`: Installs patches into exec vectors such as the memory allocator and the floating-point exception handlers, reading patch options from the command arguments and skipping any vector already in use.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_stored_handler_return_value`: Calls a handler through a pointer stored at a fixed offset in a global record and returns the value left in a data register.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_function_via_base`: Calls one library function through the library base held in a global, at a fixed negative offset, then returns.
- `execute_indirect_function_2`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `execute_indirect_function_3`: Loads a function pointer from a location within a global data structure and jumps to it.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `execute_indirect_function_4`: Fetches a function pointer from a global table and executes it.
- `copy_routine_into_allocated_memory`: Checks a memory-size field and, when conditions are met, allocates a block either at a fixed address or from memory of a suitable type.
- `store_word_checksum_then_call`: Sums twenty-three words from a record, stores the one's complement of the sum in a field, then calls through an address the decompiler could not resolve.
- `hook_disk_resource_mask_irq`: If a resource record is present and recent enough, masks an interrupt by clearing its enable bit, calls two resource subroutines, and re-enables it when the nesting count returns to zero.
- `call_hook_and_record_owner`: Calls a hook through a stored pointer and, when it returns nonzero, records the current task pointer in a shared slot.
- `call_hook_if_owner_matches`: Returns when a recorded owner differs from the current task and is not minus one.
- `call_hook_if_minus_one`: Returns unless its argument is minus one, in which case it calls a stored hook.
- `install_once_guarded_handler_patch`: Writes the interrupt-enable mask to disable interrupts, then, on the first call when a four-letter signature is absent from low memory, stores the signature and a flagged entry address and calls a handler through a table.
- `call_handler_table_entry_b`: Calls one entry of a handler table held in a global, then returns.
- `fill_sixteen_entry_address_table`: Writes a computed offset from a pointer and a signed byte into a global, then fills a sixteen-entry table with successive addresses two bytes apart.
- `call_sixteen_helpers_return_last`: Calls a series of sixteen subroutines in a row and returns the result of the last one.
- `return_register_value_unchanged_2`: Returns its register input unchanged.
- `call_fourteen_helpers_return_last`: Calls a series of fifteen subroutines in a row and returns the result of the last one.
- `return_register_value_unchanged`: Returns its register input unchanged.
- `set_function_if_slot_range`: Returns when a slot number is above a limit.
- `clone_record_or_free`: Allocates a copy of a record through a library call and copies its slot number and its table of value pairs into the new block.
- `release_counted_entries_and_return`: Calls a library routine once for each remaining count, then returns the value at its pointer with the top bit set.
- `release_entry_return_flagged_value`: Makes one library call and returns the value at its pointer with the top bit set.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `empty_stub_routine`: The routine contains four instructions and performs no observable work; it does not access memory, registers, or call other code, and immediately returns to its caller.
- `call_check_then_two_followups`: Calls a check routine with a field from a record.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `scan_words_high_byte_marker`: Advances a word pointer until it reaches a word whose high byte matches a fixed marker value, then returns that pointer.
- `advance_past_variable_length_entry`: Works out the length of a variable-length entry from its first byte, either a one-byte signed displacement or a two-byte one when the low byte is zero, and returns a pointer to the entry that follows.
- `compute_displaced_pointer_from_word`: Reads a signed word displacement stored two bytes into an entry and returns the address it points to, measured from that entry.
- `compute_chained_offsets_into_globals`: Makes seven subroutine calls, each stepping through offsets from the result of the one before, and stores three of the results in shared globals.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_exec_dispatch_function`: Calls the exec dispatch function through the library base held in a global, then returns.
- `forward_two_params_stack_slot`: Passes its two parameters to another routine together with the address of a fixed stack slot, then returns.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_at_pointer`: Stores the low byte of its data register at the address held in a pointer register, then returns.

### addbuffers

- `install_handler_block_from_list`: Scans a list for the largest value, calls library subroutines to fill a block of table entries with an incrementing pattern, installs a handler address into the block and copies sixteen-word blocks around the calls.
- `copy_data_and_call_routine`: Copies a block of data from a table to a destination pointer and then calls another routine.

### multidie

- `reset_install_state_at_entry_2`: Program entry for Ambermoon_install that calls routines and a library function in a loop from 5 down to 2, then clears a word at a fixed location.
- `decrement_until_minus_one_2`: Decrements a local variable or register value until it reaches negative one and then returns.

### wipeout

- `reset_install_state_at_entry_3`: Program entry that calls routines and a library function in a loop from 5 down to 2, then clears a word at a fixed location.
- `decrement_register_to_negative_one`: Decrements a 16-bit register until its value reaches negative one and then returns.

### remove

- `reset_install_state_at_entry_4`: Performs a series of delay loops, then checks if a specific memory word matches a constant.
- `decrement_word_until_minus_one`: Decrements a 16-bit value stored in a register until it reaches negative one.

### EndCli

- `install_handler_block_list_b`: Scans a list for the largest value, calls library subroutines to fill a block of table entries with an incrementing pattern, installs a handler address into the block and copies sixteen-word blocks around the calls.
- `copy_data_to_stack`: Copies a sequence of words from a memory location to a stack if the stack pointer is not null.

### Echo

- `traverse_and_populate_linked_list`: Traverses a linked list to find the maximum value in a sequence of elements.
- `copy_stack_data`: Copies a block of sixteen bytes from a memory location to the stack and then calls another routine.

### lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Cls

- `open_dos_call_console_op`: Opens a library through the exec base, calls two library functions on it, and closes it when the second call reports a failure.

### FastMemFirst

- `start_fastmemfirst_cli_or_workbench`: Program entry point with the same start-up shape as the other command entries: it allocates a state record, opens libraries, handles CLI or Workbench start-up, then frees its record and returns a status.
- `move_fast_memory_node_first`: Walks the memory-region list looking for a fast-memory entry in a fixed address range with a matching type.
- `call_library_base_e`: Calls one library function through the library base held in a global, then returns.
- `execute_indirect_function`: Calls a function through a pointer located at a specific offset within a global variable.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.

### CD

- `install_handler_block_list_d`: Scans a list for the largest value, calls library subroutines to fill a block of table entries with an incrementing pattern, installs a handler address into the block and copies sixteen-word blocks around the calls.
- `copy_stack_data_2`: Transfers sixteen bytes from a source address to the stack before executing a subroutine.

### mouse

- `wait_fire_port_bit_clear`: Busy-waits while bit six of the joystick port register is set, then returns zero.

### EndIf

- `return_zero_stub`: Returns zero and does nothing else.

### If

- `initialize_dynamic_data_structure`: Traverses a linked list to find a maximum value and then populates a structured data block with a sequence of incrementing values.
- `copy_memory_block`: Copies sixteen words from a source address to a destination address pointed to by a stack value.

### memavail

- `reset_install_state_at_entry`: Calls several routines in a loop from five down to two, clears a word at a fixed location, and if a specific memory value is found, swaps it with a global variable and clears that global.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### setpatchV38.31

- `setpatch_check_kickstart_cache`: Program entry for a v38 patch installer.
- `allocate_and_copy_patch_block`: Checks a library result and a system flag.
- `allocate_copy_patch_block_b`: Returns zero unless a flag is set.
- `allocate_copy_patch_block_c`: Allocates memory, copies fifty-one words of a fixed block into it, calls library functions to install and release the block, and returns one.
- `probe_memory_size_and_mask`: Checks a low-memory byte and, if it is not a fixed value, probes memory downward from a fixed high address to find where the self-addressing test stops matching, then updates a fixed-address byte with a bit mask.
- `release_patch_entries_and_clear`: Releases a pending entry if one is recorded, then walks six slots, releasing each one that is set through a library call and clearing it.
- `complete_request_or_forward_handler`: Checks a request state word and, when it matches the completion case, copies two stored fields, clears a flag and calls a library vector.
- `finish_request_or_forward_handler`: Repeats the same state check and completion steps as its neighbour, clearing the flag and calling the library vector on a match.
- `reply_message_or_forward_handler`: Runs the completion steps directly when a status byte matches, copying fields, clearing flags and calling a library vector that replies to the message.
- `finish_unflagged_request_or_forward`: When a flag word at a fixed offset is zero, it copies a stored field, clears a flag byte and calls a library vector.
- `call_cia_routine_twice`: Calls the same unnamed routine twice in a row and returns zero.
- `claim_flagged_structure_once`: Fetches a structure through a library call and, if the pointer is non-null and its flag byte is negative, sets that flag, passes the structure to a library routine and then clears the flag again.
- `forward_to_unnamed_routine`: Two-instruction wrapper that transfers control to one other routine with no other visible effect.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_through_fixed_vector`: Five-instruction relay that calls a single jump-table entry, which passes control straight on to a fixed address elsewhere in memory.
- `cache_console_device_handle`: Calls a library routine and, when it returns a non-null pointer, saves that pointer, a value read from its sixth longword, and the caller's base pointer into three global slots.
- `dispatch_in_range_or_forward`: Compares a value with lower and upper bounds held in globals.
- `pass_through_saved_vector`: Jumps straight through a saved vector pointer, handing control to the previously installed routine without changing any arguments.
- `dispatch_below_limit_or_forward`: Compares the data register with an upper bound stored in a global.
- `store_offset_from_library_result`: Chooses one of two fixed offsets depending on whether a field equals a particular value, adds it to the pointer returned by a library call, and stores the result in a global when that call succeeded.
- `clear_matching_slot_entry`: Uses the data register, scaled by four, as an index into a table.
- `clear_slot_or_forward`: Looks up a table entry by the scaled data-register index and, if it matches the stored global, sets its two following fields to minus one.
- `clear_slot_then_forward_nested`: Performs the same indexed table check, and on a match clears the two following fields.
- `jump_through_saved_vector_again`: Two-instruction routine that jumps through the saved original vector pointer, passing control to the earlier handler unchanged.
- `sequence_intena_calls_return_one`: Calls three library routines in sequence and stores the constant 64 in a global between two of those calls.
- `save_argument_into_global_slot`: Copies its input register into a global variable and returns one.
- `refresh_pattern_buffer_on_change`: Reads a value through a library call and compares it with a cached copy for the selected index.
- `stub_single_instruction_handler`: The facts describe a one-instruction routine reached through a handler pointer in data, with two unnamed callers.
- `update_pattern_cache`: Compares a value from another routine to a table entry, and if they match a specific constant, fills a memory block with a repeating pattern.
- `update_pattern_cache_and_fill`: Calls an unnamed routine for a value and, when it differs from the cached value for the indexed entry, updates the cache.
- `add_library_result_twice`: Calls a library routine and, when it returns a non-zero value, adds that value into one global slot twice.
- `update_on_matching_message_code`: Checks a stack argument and the second argument against two fixed codes, and when both match and a flags word has bit three or four set, copies one stored field into another.
- `copy_field_on_code_match`: When the second argument equals a fixed code and a flags word has bit three or four set, copies one stored field into another.
- `clear_flag_bit_on_target`: Chooses a target pointer from a library call when a flag bit is set, otherwise uses a fixed low address.
- `search_list_by_word_key`: Calls a library routine, then walks a linked list of nodes looking for one whose stored halfword matches the key in the data register.
- `search_linked_list`: Traverses a linked list to find a node whose halfword matches a key, using library calls to manage the search process.
- `search_list_for_matching_word`: Calls a library routine and scans a linked list for a node whose stored halfword equals the data-register key.
- `search_list_with_two_calls`: Makes two library calls around a scan of a linked list of nodes, matching each node's halfword against the key in the data register.
- `search_list_unless_second_arg`: Returns the second argument unchanged when it is non-zero.
- `traverse_linked_list_with_hooks`: Executes a hook routine, traverses a linked list to find a node matching a specific halfword value, and executes a second hook routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `search_list_return_two_default`: Calls a library routine, scans a linked list for a node whose halfword matches the data-register key, and calls a second library routine.
- `search_list_returning_two`: Runs a linked-list scan with a library call before and after it, matching the data-register key against a stored halfword.
- `search_list_signed_minus_one`: Returns minus one at once when the data register holds minus one.
- `traverse_list_with_hooks`: Executes a hook routine, traverses a linked list to find a node matching a specific halfword value, and executes a second hook routine.
- `search_list_zero_default`: Calls a library routine, scans a linked list for a node whose halfword matches the data-register key, and calls a second library routine.
- `search_list_with_two_hooks`: Runs a linked-list scan for a halfword matching the data-register key, bracketed by two unnamed routine calls.
- `search_list_return_argument_default`: Calls a library routine, scans a linked list for a node whose halfword matches the data-register key, and calls a second library routine.
- `return_stack_value_if_present`: Calls a library routine and returns the first stacked argument when its result is non-null, otherwise returns the third argument.
- `return_parameter`: Returns the input parameter unchanged.
- `locate_word_entry_in_table`: Picks a base pointer from a fixed offset or a library call, depending on a flag bit, then scans forward through halfwords for a stored value and saves that position.
- `call_library_vector_636`: Library stub: loads a library base from a global variable and calls the library function at offset minus 636 of that base, returning its result to the caller.
- `check_patched_branch_or_forward`: Reads the first word at a stored patch target.
- `check_patched_branch_then_forward`: Performs the same opcode check on a stored target.
- `relay_to_unnamed_routine`: Calls one unnamed routine and returns.
- `convert_c_string_to_bcpl`: Measures a null-terminated C string and copies it into a newly allocated buffer as a length-prefixed string, with a zero after the last character.
- `write_bcpl_string_to_buffer`: Measures a null-terminated C string and writes it into a caller-supplied buffer as a length-prefixed string, placing the length byte first.
- `copy_bytes_backwards_terminated`: Copies a run of bytes backwards from the end of one buffer into another, stores a byte passed in a data register in front of the copy, and writes a zero terminator.
- `stub_immediate_return`: A three-instruction routine that performs no work and returns straight away to its single unnamed caller.
- `forward_library_call_pair_result`: Calls a library routine and returns its two result registers together as one 64-bit value.
- `call_set_signal`: Library stub: loads its arguments and the exec library base, calls SetSignal and returns its result to the caller.
- `drain_flagged_list_nodes`: Increments a global activity counter and decrements a per-structure count.
- `branch_on_sign_decrement_counter`: When the sign flag is set it calls two library routines; otherwise it decrements a 16-bit counter in the caller's structure.
- `call_alert_and_init_semaphore`: Calls two library routines that the facts identify as exec.Alert and exec.InitSemaphore, then a third library routine, and returns.
- `unlink_first_flagged_node`: Removes the first node from a linked list in the caller's structure.
- `walk_list_until_terminator`: Walks a linked list from the head pointer in the caller's structure, calling a library routine on each node and relinking as it goes.
- `advance_count_clear_status_bit`: Increments a per-structure count and, once it is non-zero and a pointer is set, clears bit four of a status byte and calls two library routines.
- `clear_status_bit_or_count`: When a pointer in the caller's structure is set, it clears bit four of a status byte, calls two library routines and tail-calls a third.
- `check_stack_bounds_then_forward`: Compares the address of a local variable with the lower and upper stack bounds stored in the task structure, and checks a flag word for minus one.
- `check_stack_bounds_then_call`: Performs the same stack bound and flag-word test as its neighbour.
- `check_stack_bounds_forward_result`: Applies the same stack bounds and flag-word test, and when it triggers it calls the saved original routine and returns the byte it produces.
- `check_stack_bounds_then_relay`: Tests the local-variable address against the stack bounds and the flag word for minus one.
- `save_a1_and_base_globals`: Stores the A1 argument and the A6 base register into two global slots and returns one.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `relay_call_to_neighbour`: Three-instruction routine that transfers control to one other unnamed routine.
- `call_unnamed_routine_after_helper`: Eleven-instruction routine that calls a two-instruction helper and one unnamed routine.

### Version

- `startup_allocate_and_open_dos`: Program entry that finds the current task, allocates a memory block and opens dos.library, returning an error code on failure.
- `swap_task_slot_with_cleanup`: Stores its argument in a per-task slot and returns that slot's value after running cleanup calls.
- `parse_args_check_library_version`: Reads command-line arguments and prints a usage line when asked.
- `call_library_then_unnamed_routine`: Calls a library routine with the first argument and a fixed flag value of 128, then passes the second argument to one unnamed routine.
- `parse_signed_decimal_string`: Converts a string to an integer by reading an optional plus or minus sign and then decimal digits.
- `compare_null_terminated_strings`: Compares two null-terminated strings byte by byte.
- `measure_null_terminated_length`: Walks a null-terminated string and returns the number of characters before the terminator.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_exec_dispatch_vector`: Calls the exec.Dispatch routine through the library base stored in a global, with no arguments set up here.
- `forward_with_stack_pointer_argument`: Passes its two arguments to one unnamed routine together with the address of a stack location, then returns.
- `call_exec_vector_198`: Calls the exec library routine at offset 198 from the exec base held in a global, and returns its result.
- `call_exec_vector_210`: Calls the exec library routine at offset 210 from the exec base held in a global, and returns its result.
- `call_exec_vector_414`: Calls the exec library routine at offset 414 from the exec base held in a global, and returns its result.
- `call_exec_vector_444`: Calls the exec library routine at offset 444 from the exec base held in a global, and returns its result.
- `call_exec_vector_450`: Calls the exec library routine at offset 450 from the exec base held in a global, and returns its result.
- `call_exec_vector_552`: Calls the exec library routine at offset 552 from the exec base held in a global, and returns its result.
- `format_text_and_emit_length`: Formats text into a 140-byte stack buffer with the exec formatted-output routine, then measures the resulting string.
- `store_byte_at_output_pointer`: Stores the low byte of the data register at the location given by the address register and returns.

### d

- `apply_bitwise_and_mask`: Performs a bitwise AND operation between a byte and the value pointed to by a register, storing the result back at that memory location.

### gb

- `null_entry_point`: This routine is empty and contains no executable instructions.

### Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:InstallWingCommander

- `startup_clear_state_open_dos`: Program entry for the installer that zeroes a large global state block and works out a stack-based layout.
- `release_install_state_and_reply`: Stores its argument in a stack slot and runs cleanup: it calls an optional callback, two unnamed routines and, when a flag is set, a further library routine.
- `walk_entry_chain_with_delay`: Checks remaining stack space before walking a chain of entries, calling a set of helper routines on each one.
- `query_volume_info_with_lock`: Locks a path, queries volume information with the dos library and unlocks it, with memory allocated and freed around the call.
- `find_table_entry_by_id`: Scans a table of sixteen-byte entries and returns the first entry whose stored identifier matches the argument, or zero when none does.
- `process_typed_table_entries`: Walks a table of sixteen-byte entries and, for entries whose type byte is 2 or 4, calls helper routines with the entry's identifier.
- `copy_file_and_set_protection`: Opens a file with the dos library, examines it and reads its contents into allocated memory, then writes them out.
- `scan_string_against_install_buffer`: Copies a string into a fixed buffer in the install state, then scans it to the end and compares its characters with the argument string.
- `create_directory_if_missing`: Examines a path with a lock first and creates the directory when the examine does not find it.
- `stub_return_without_work`: The entry is eight instructions long, but its pseudo-code shows only a return.
- `stub_no_op_return`: Eight-instruction entry whose only effect is an immediate return to its single caller.
- `free_linked_memory_blocks`: Frees each node of a linked list through the exec free-memory call, clears the list head, then releases the remaining entries of a second list.
- `split_slash_path_into_segments`: Splits a path on forward slashes by writing a zero at each separator and recording the start of each segment in an array of up to sixteen entries.
- `split_path_into_name_parts`: Copies a path into a 63-byte local buffer and scans it backwards for a dot, slash or colon to separate the name from the directory and extension parts.
- `append_path_separator_if_needed`: Finds the end of a path string and appends a separator byte from a global when the last character is not already a slash, colon or backslash.
- `copy_string_return_end_pointer`: Copies a null-terminated string from the source to the destination, including the terminator, and returns the address where the terminator was written.
- `format_decimal_digits_to_string`: Repeatedly calls an unnamed division helper and stores each remainder as an ASCII digit.
- `format_octal_digits_to_string`: Converts an unsigned value to octal digits by taking three bits at a time, reverses them into the output, adds a terminator and returns the length.
- `format_hex_digits_to_string`: Converts an unsigned value into lower-case hexadecimal digits using a sixteen-character lookup table.
- `parse_signed_decimal_into_pointer`: Reads an optional plus or minus sign and decimal digits from a string, stores the signed value through the second pointer, and returns the number of characters consumed.
- `allocate_buffer_or_set_error`: When a flag bit is clear, it allocates a buffer through an unnamed routine and stores it in two fields of the structure.
- `uppercase_string_in_place`: Walks a null-terminated string and converts each lower-case letter to upper case in place.
- `write_buffered_data_to_handle`: Writes data through the dos library after checking the handle's mode bits, and records any dos error in a global.
- `check_handle_mode_then_call`: Checks that a handle exists and has bit one set, otherwise it records an error code and returns minus one.
- `read_from_handle_checked`: Checks a handle's mode bits, then reads data from it through the dos library and records dos error codes in globals.
- `seek_handle_checked`: Validates a handle, then seeks within it through the dos library using the mode argument, recording any dos error.
- `close_handle_and_clear_slot`: Closes an open file handle through the dos library and returns zero; an invalid handle returns minus one.
- `open_install_file_with_protection`: Adds a node to the open-file list, allocating one when no free node exists, then opens the named file through the dos library and applies protection bits.
- `divide_fixed_point_values`: Divides one 32-bit value by another using 16-bit halves and a shift chosen from the divisor's size.
- `multiply_32bit_via_halves`: Multiplies two 32-bit values from the data registers by combining the cross products of their 16-bit halves with the low-half product.
- `stub_empty_return_routine`: The pseudo-code shows only a return, while the facts list 25 instructions and one unnamed call, so only the return is claimed.
- `close_or_notify_open_handles`: Sets a status flag and walks the open-file list.
- `parse_command_line_into_args`: Splits the command line into up to thirty-two arguments, treating quoted text as a single argument.
- `copy_message_text_to_stack`: Copies up to seventy-nine characters of a string from the caller's state into a stack buffer and terminates it.
- `print_break_message_to_output`: Copies a message of up to seventy-nine characters into a stack buffer, looks up the current task and writes the text, with a break prefix, through the dos library.
- `get_list_node_by_index`: Returns the node at the given index in the list rooted in the state, or null when the index is negative or past the count.
- `check_break_signal_and_report`: Calls SetSignal to read the signal mask.
- `find_byte_in_buffer`: Scans a buffer of the given length for a target byte and returns a pointer to the first match, or null when the length runs out without one.
- `append_string_to_buffer`: Appends a source string to the end of a destination string, copying no more characters than the count passed in the data register and always writing a terminator.
- `concatenate_strings_unbounded`: Finds the terminating zero of the destination string and copies the source string, including its terminator, onto that point.
- `copy_string_padded_to_count`: Copies a null-terminated source into the destination, stopping at its terminator or at the count, then fills the rest of the count with zero bytes.
- `move_byte_block_overlap_safe`: Copies a block of bytes of the given length, choosing forward or backward order so that overlapping source and destination regions are copied correctly.
- `release_pool_block_checked`: Frees a block from the state's memory pool after checking that its address lies inside a recorded region, recording error code twenty-two when it does not.
- `free_pool_block_via_exec`: Performs the same region check as its neighbour, then returns the block to exec through the free-memory call.
- `allocate_pool_block_from_region`: Rounds the requested size up to a multiple of four and searches the recorded pool regions for a free block of that size.
- `allocate_pool_block_or_alloc`: Uses the same rounding and pool search as its neighbour.
- `close_all_open_streams`: Runs the registered callbacks, then closes or notifies each node in the stream list and sets the closing flag.
- `append_byte_to_fill_buffer`: Stores a byte at the current fill position of a buffer and increments the count.
- `return_status_from_flags`: Calls one unnamed routine with a stack-frame argument, then returns minus one when bits four or five of a flag word are set and zero otherwise.
- `read_line_into_buffer`: Reads characters from the stream into a buffer until a newline or end of input, then terminates the buffer.
- `read_next_byte_from_stream`: Returns the next byte from a buffered stream, or minus one at end of input.
- `compute_stream_position`: Works out a stream position from a read result: when a flag is set it subtracts the number of newline characters counted in the buffer, and otherwise it subtracts the remaining buffered count.
- `compute_stream_position_from_node`: Follows the same position calculation as its neighbour for a node passed in its argument, using the flag and buffered count stored in the node.
- `reset_stream_buffer_state`: Resets a stream's buffer pointers and counters to their starting values, clears a status byte and runs a flush step through unnamed routines.
- `check_stream_access_mode`: Checks a stream's mode bits for the access requested.
- `check_stream_access_and_write`: Checks the stream's mode bits in the same way as its neighbour, returning zero when access is refused.
- `check_stream_access_and_read`: Checks the stream mode bits for a read request and returns zero when access is refused.
- `update_stream_node_state`: With a null node it walks the stream list and returns minus one if any helper reports a problem.
- `flush_and_release_stream_node`: Returns minus one for a null or unopened node.
- `close_stream_node_if_open`: Returns minus one for a null or closed node.
- `open_stream_from_mode_string`: Parses a two-character access mode made of read, write, append and optional plus signs into flag bits, then calls another routine to open the file.
- `find_or_allocate_free_node`: Finds the first unused node in a list, or allocates and zero-fills a new node when none is free, then passes the node to an unnamed routine.
- `format_string_with_arguments`: Walks a format string, reading flags, width and conversion letters, and converts the matching argument values into text using six unnamed helpers.
- `emit_format_text_characters`: Walks a format string, passing ordinary characters to an output routine and treating a doubled percent sign as one literal percent sign.
- `delete_file_checked`: Calls the dos library delete routine.
- `stub_empty_return_two`: A 36-instruction routine whose pseudo-code is only a return, with four unnamed callers.
- `call_unnamed_routine_then_return`: Calls one unnamed routine and returns, with no other work.

### Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:Wing

- `free_blocks_restore_patch_a`: Program entry for the Wing executable that calls two unnamed routines and frees a short run of memory blocks through exec.FreeMem.
- `busy_wait_16bit_countdown`: Loops, decrementing a 16-bit counter until it wraps past zero to minus one.

### Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `iterate_records_calling_library`: Walks a sequence of length-prefixed records until a zero byte, calling one library routine for each record and a second library routine after it.
