# Game 2360 (spy vs spy - the island caper (1989)(wicked software)[cr vf][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02360). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902144 top-level bytes in 40 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 25 of 25 units identical (bytes and relocations), 124 raw-word lines |
| Functions named and described | pass, 924 of 924 named (287 author symbols), 924 of 924 described (59 by rule, 306 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1287 of 1287 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 22487 of 22531 instructions identical in start and length, 3 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 182844 |
| code | 85024 |
| data | 179837 |
| padding | 7463 |
| unused | 446976 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| setmap | 13 | 627 | 33 |
| delete | 2 | 117 | 2 |
| addbuffers | 2 | 117 | 2 |
| path | 2 | 117 | 2 |
| ex | 2 | 117 | 2 |
| info | 2 | 117 | 2 |
| dir | 2 | 117 | 2 |
| copy | 2 | 117 | 2 |
| makedir | 2 | 117 | 2 |
| list | 2 | 117 | 2 |
| cd | 2 | 117 | 2 |
| genim2 | 1 | 4256 | 221 |
| spy2 | 3 | 9563 | 227 |
| begin | 4 | 1202 | 71 |
| port-handler | 1 | 306 | 1 |
| disk-validator | 1 | 386 | 1 |
| ram-handler | 2 | 10 | 1 |
| serial.device | 4 | 976 | 83 |
| gb | 1 | 12 | 1 |
| parallel.device | 3 | 322 | 26 |
| printer.device | 19 | 3272 | 218 |
| arp.library | 3 | 85 | 7 |
| Spy vs Spy - The Island Caper (1989)(Wicked Software)[cr VF][a].adf:vf.obj | 2 | 96 | 4 |
| Spy vs Spy - The Island Caper (1989)(Wicked Software)[cr VF][a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 238 | 9 |

## Cases handled

- runtime segment an address: 181 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `report_malformed_line` (genim2): Sets the flag and raises a counter as in other handlers, then calls helpers that report a malformed line of input. Kind: sabotage path. Pass path: sub_4AF2: bra.w an address.
- `reset_game_state_fields_2` (genim2): Clears several internal state variables and certain pointers to zero or default values, and reinitializes a string buffer with a default file extension if a certain flag is set. Kind: manual/code-wheel/colour-card lookup. Pass path: the branch where iVar6 == 0, returning uVar4.
- `START_SOUNDS` (begin): Fills an 8192-byte area with a generated pseudo-random byte sequence, clears the music switch and music pointer variables, and clears two audio channel words before setting one audio period register. Kind: random_number_seed_initialization. Pass path: the routine initializes a PRNG or random-based state (RND1, RND2, RND3) and sets up hardware registers, which is a common setup for a protection-heavy loader or a routine that uses random-delay or random-pattern-based checks to obscure code.
- `EXPLODE` (begin): Starts the explosion sound on the fourth audio channel by pointing it at the random table, setting its length and period to a fixed value, clearing its state, and installing the explosion effect as the pending routine. Kind: sabotage path. Pass path: the instructions following the call continue execution normally, while the routine overwrites critical RAM/state (like pointers and counters) to cause a crash or a forced error state (EXPLODE).

## Functions


### setmap

- `start_from_cli_or_workbench`: Program entry point that checks the task's CLI pointer to tell a command-line start from a Workbench start.
- `release_resources_and_return_status`: Releases the library and message-port resources held in global variables, then returns the status value passed in as its argument.
- `call_two_library_vectors`: Calls two library functions one after another through jump-table offsets relative to the library base register, then returns without a result.
- `initialize_library_and_resources`: Attempts to initialize a library and checks the return status, subsequently releasing library and message-port resources if certain conditions or previously initialized pointers are met.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `release_signal_and_device_resources`: Releases the signal, device and library resources held in global variables, calling the release helper for each one that is set.
- `store_pair_and_invoke_helpers`: Saves its two arguments into global variables, then calls a helper with a fixed argument list that includes a global reference and constants.
- `load_and_install_keymap`: Reads the keymap name from the command line or the icon tool types, opens console.device and icon.library, and loads the named keymap file from the keymap directory.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_head`: Library stub: loads its arguments and the exec library base, calls AddHead and returns its result to the caller.
- `call_find_name`: Library stub: loads its arguments and the exec library base, calls FindName and returns its result to the caller.
- `call_find_task`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_alloc_signal`: Library stub: loads its arguments and the exec library base, calls AllocSignal and returns its result to the caller.
- `call_free_signal`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `call_put_msg`: Library stub: loads its arguments and the exec library base, calls PutMsg and returns its result to the caller.
- `call_wait_port`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_open_device`: Library stub: loads its arguments and the exec library base, calls OpenDevice and returns its result to the caller.
- `call_close_device`: Library stub: loads its arguments and the exec library base, calls CloseDevice and returns its result to the caller.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `call_open_resource`: Library stub: loads its arguments and the exec library base, calls OpenResource and returns its result to the caller.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `forward_with_stack_scratch_slot`: Forwards its parameter together with a global variable to a helper routine.
- `invoke_library_function_from_global`: Loads a library base from a global variable and calls the function at a fixed negative offset from it.
- `format_and_write_message`: Formats a message into a local 140-byte buffer with exec's formatting routine, then measures its length by scanning for the terminating zero byte.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### delete

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_and_call`: Copies a 16-long block from a fixed offset in a base register into the buffer whose address is passed on the stack, when one is supplied.

### addbuffers

- `allocate_and_prepare_command_block`: Finds the largest size in a list of entries and allocates a buffer that starts with a counting pattern of words.
- `copy_stack_data`: Copies a fixed number of words from a memory location into the stack.

### path

- `initialize_program_data_tables`: Calculates the size of a data structure, populates a table with an incrementing sequence, and copies or initializes memory blocks or pointers or or certain values.
- `copy_and_branch_data`: copies a fixed number of words from a memory location relative to a pointer into the stack and then calls another routine.

### ex

- `initialize_task_environment_2`: Calculates the size of a linked list or table and populates a memory buffer with a sequence of incrementing values.
- `copy_data_and_jump`: copies a block of 16-bit or 32-bit words from a memory location or table into the stack and then jumps to another routine.

### info

- `initialize_program_environment`: Traverses a linked list of pointers to find the maximum value, then initializes a certain memory block with an incrementing sequence of longwords.
- `copy_and_branch_to_subroutine`: copies a block of sixteen bytes from a memory location relative to a register into the stack and then jumps to another routine.

### dir

- `initialize_program_environment_2`: Iterates through a linked list of pointers to find the maximum value in a list, then populates a table with a sequence of incrementing longwords and copies a block of data from a source or destination buffer.
- `copy_stack_frame_and_branch`: copies a fixed number of words from a memory location (calculated from a register) onto the stack before calling another routine.

### copy

- `initialize_program_environment_3`: Iterates through a linked list or table to calculate a maximum value, then initializes a data structure or table with a sequence of incrementing values.
- `copy_data_and_branch`: copies a fixed number of words from a memory location to the stack before jumping to another routine.

### makedir

- `initialize_task_data_structures`: Iterates through a linked list to find a maximum value, then populates a table or structure with an incrementing sequence of words and copies a block of data from a fixed memory location.
- `copy_stack_frame_and_jump`: copies a fixed number of words from a memory location or a data structure onto the stack and then calls another routine.

### list

- `initialize_program_environment_4`: Calculates the maximum size of a linked list or table, initializes a buffer with a sequence of incrementing or fixed values, and copies a block of data from a fixed source to a destination or vice-versa.
- `copy_memory_block`: copies a fixed number of words from a source pointer to a destination pointer.

### cd

- `initialize_task_environment`: Iterates through a linked list or table to find a maximum value, then populates a memory block with a sequence of incrementing or fixed values and copies a block of data from a source buffer to a destination or vice versa.
- `copy_stack_frame`: Copies a fixed number of words from a source pointer or table to the stack.

### genim2

- `initialize_frame_state_and_process`: Copies a template block from a global area into the frame, then clears a block of words and several flags.
- `reset_game_state_fields`: Clears a group of state variables in the data block and copies one stored longword into another field.
- `process_pending_list_entries`: Returns early unless one flag is set and a second flag is clear.
- `run_six_helpers_then_zero`: Calls six helper routines in sequence, then zeroes one longword in the data block.
- `run_four_helpers_then_accumulate`: Calls four helper routines in sequence, then adds one stored longword into another field.
- `increment_flagged_counters`: If the lowest bit of the word at the argument pointer is set, increments that word and a second counter stored two longwords later.
- `release_node_fields_if_set`: Inspects a node structure and calls a helper when its first longword is set.
- `clear_node_and_release_links`: Calls a helper if the node's first longword is set, zeroes its third longword, then calls a helper with the node if its second longword is set.
- `fetch_next_input_byte`: Returns the next byte from a buffer held in the data block, or minus one once the end bound has been reached.
- `process_input_state_step`: When a flag is clear it scans a list of character pointers.
- `adjust_counter_from_input_char`: Sets a flag, and when a state flag is set it increments or decrements a signed counter according to whether the input character is a plus or a minus sign.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 10 into d1, then returns.
- `decrement_counter_and_update_flag`: Decrements a signed byte counter in the data block and sets a flag showing whether the result is still non-negative.
- `increment_counter_and_update_flag`: Increments a signed byte counter in the data block and sets a flag showing whether the result is non-negative.
- `clear_transient_input_flags`: Clears a set of transient flags in the data block, calling a helper first when a related flag is set and its conditions hold.
- `parse_hunk_type_records`: Compares values against AmigaDOS hunk type codes and branches on each one, calling helpers to read the matching record layout.
- `copy_record_with_optional_bytes`: Copies a header byte and the leading fields of one record into another.
- `store_pointer_offset_sixteen`: Stores the caller-supplied pointer advanced by sixteen bytes into a pointer field in the data block.
- `append_matching_entry_to_list`: When the state flags allow it, compares a byte from the entry with a stored byte.
- `emit_pending_list_record`: When a state flag is set, writes a pending record to a list by copying fields from the source entry.
- `initialize_new_list_node`: Initialises a new list node from a pool pointer, setting its link, tag and count fields.
- `consume_buffer_bytes`: Reserves a number of bytes from a buffer pointer and its counter in the data block.
- `reset_list_state_and_check`: Clears two list-state words in the data block, calls helpers that update them, and returns early when both values are still zero.
- `call_helper_when_flag_clear`: Returns if a flag in the data block is set.
- `advance_scan_and_call_helpers`: Adjusts a scan position in the data block and calls four helper routines with the resulting values.
- `run_six_scan_helpers`: Runs a scan step that calls six helper routines using the current position and a byte value.
- `compare_and_advance_scan_position`: Compares a stored value with the current scan state and advances the position when they match, calling one helper.
- `forward_to_scan_helper`: Calls a single scan helper routine and returns.
- `run_scan_helpers_on_entry`: Calls two scan helper routines in sequence on the current list entry.
- `call_two_scan_helpers`: Calls two scan helper routines and returns.
- `call_scan_helper_with_pointer`: Calls one scan helper routine with a pointer argument.
- `run_four_scan_helpers`: Calls four scan helper routines in sequence on the current scan state.
- `chain_two_scan_helpers`: Calls one helper and then a second helper, returning the result of the second call.
- `pop_list_state_until_zero`: Calls a helper repeatedly while the current list entry is nonzero, then steps two list-state words back by 2 and 8.
- `step_back_list_state`: Decrements two list-state words in the data block by 2 and 8.
- `update_state_and_list`: Updates a flag and a counter, then saves a set of registers to a memory location before traversing a list to append a value.
- `pop_eight_byte_list_entry`: Decrements an 8-byte list counter at a fixed offset and, when the previous value was large enough, loads the two longwords stored for that entry.
- `tiny_movea_rts`: Small routine of 2 instructions that copies a4 to a0, then returns.
- `classify_token_character`: Skips newline, tab, space and comma characters and classifies letters, digits and symbols into token classes.
- `scan_digit_run`: Returns at once on angle-bracket characters, and otherwise scans a run of digit characters from the input pointer.
- `perform_no_operation`: This routine is empty and performs no operations.
- `scan_dollar_prefixed_token`: Skips a run of digit characters, and when the next character is a dollar sign it records a buffer start in the data block and writes a terminator.
- `dispatch_by_character_offset`: Jumps through a computed address derived from the negated character value to a handler in the program image, then returns.
- `match_directive_keyword`: Compares the leading characters of the current token with a four-character keyword and a double-underscore prefix.
- `set_hash_marker_flag`: Sets a marker byte to all-ones when the current character is a hash sign and a stored flag is set, then calls helpers depending on the flags and the marker state.
- `dispatch_record_attributes`: Checks a state flag and the attribute bits of the current record, calling helpers accordingly and returning early for some attribute values.
- `check_record_type_and_call`: Tests bit 4 of a record's attribute byte.
- `forward_to_helper_routine`: Calls a single helper routine and returns.
- `return_void`: the routine is empty and performs no operations before returning.
- `handle_record_by_type`: Calls a helper, then when the zero flag is set and the attribute checks pass, it tests a flag and the record type byte and returns for one type.
- `count_matching_record_field`: When a record's attribute bit is clear, compares a field with a stored value for type-one records and increments a counter on a match.
- `append_marker_to_buffer`: Appends a double-plus marker, a single-bit-set marker byte, and a type byte retrieved from an entry into a list buffer before adding a zero terminator.
- `store_name_buffer_and_length`: Stores a leading byte from a state field into a buffer in the data block, sets a length byte from the difference of two characters, and records the buffer address.
- `skip_invalid_record`: Performs no operations and then calls a routine that reports a malformed line of input.
- `skip_invalid_record_nop`: An empty routine that performs no operations before calling another routine to report a malformed line of input.
- `check_pending_flag_and_helper`: Returns early for one combination of a flag and a character, otherwise it calls a helper with a value from the state block.
- `save_registers_to_list`: saves several registers to a memory location and appends a value to a linked list if certain conditions are met.
- `store_equals_marker_and_value`: Stores the equals character into a byte field of the data block, then stores the argument into the longword that follows it, after a helper call.
- `compare_and_pack_result`: Combines a comparison outcome with a helper's return value and returns both as one packed result.
- `increment_entry_count`: Increments the entry counter in the data block and returns a result pair, returning early when the input value is nonzero.
- `loop_until_entry_count`: Calls helpers in a loop controlled by the entry counter in the data block until the loop condition ends.
- `scan_until_end_tag`: Loops over input until it finds the four-character ENDC tag, using the entry counter to track progress.
- `copy_entry_counter_to_slot`: Reads the entry counter from the data block and stores it into a related slot in a loop.
- `match_if_conditional_directive`: Checks the leading two characters of the token against the IF directive and calls helpers to process a match.
- `skip_blank_and_comment_chars`: Advances the input pointer past newline, tab, space and asterisk characters, calling helpers for other characters.
- `scan_symbol_until_delimiter`: Reads characters until a newline, tab, space or period is reached, mapping each one through a lookup table in the data block.
- `return_stored_pointer_value`: Returns minus one together with the longword stored at a fixed offset in the data block.
- `initialise_limit_and_store_result`: Sets a 16-bit limit field in the data block to eight thousand, calls a helper and stores the value it returns in a pointer field.
- `store_byte_at_pointer`: Stores the low byte of the argument at the address held in the pointer register.
- `update_flags_on_selection`: When a flag is set, updates a related flag and a 16-bit counter in the data block and calls a helper.
- `process_pending_list_entries_2`: Returns if a flag is set.
- `reset_state_and_run_helpers`: Clears a byte flag, sets a state byte to 20, and calls a long sequence of helpers, clearing the flag again between calls.
- `advance_block_list_pointer`: Moves a pointer through a block list, switching to the next block when the current block's end is reached, and returns the position.
- `do_nothing`: This routine is empty and performs no operations before returning.
- `fill_table_with_minus_one`: Fills a table in the data block with the value minus one, then clears a second table and returns.
- `parse_operand_from_text`: Parses an operand from the input text by calling helper routines for each character class and stores the result in the data block.
- `store_word_at_pointer`: Stores a 16-bit argument at the address held in the pointer register.
- `dispatch_stored_jump_target`: Loads a stored code address from the data block, clears that slot and jumps to the address.
- `dispatch_token_handler`: Skips to the next non-blank character, then either calls a helper or jumps through a stored handler address when one is set.
- `update_position_offset`: Computes the offset of the current pointer from a base stored in the data block and adds it to two position counters.
- `push_registers_to_stack`: saves a set of registers to a memory-based stack or context structure and updates a flag or counter.
- `find_period_in_name`: Scans the characters of a stored name for a period, recording its position and the remaining length in the name block.
- `scan_identifier_chars`: Reads characters from the input until a delimiter is reached, using a lookup table in the data block to classify each character.
- `scan_quoted_string`: Reads characters after an opening double or single quote until the matching quote is reached.
- `store_pointer_when_present`: When a pointer field is set, calls a helper and stores its result in a second field, then calls another helper.
- `find_entry_by_length`: Walks a linked list comparing a stored byte with each entry's length field, returning the matching entry or the code three when the list is empty.
- `call_two_module_helpers`: Calls two helper routines and returns.
- `initialise_list_pointers_and_store`: Clears a pointer field, calls helpers, and stores the returned values into related fields when the state flag is clear.
- `store_pointer_pair_at_offset`: Stores the argument into one pointer field and the argument advanced by sixteen bytes into another.
- `parse_hunk_record_header`: Stores a record code in the data block and compares it against AmigaDOS hunk type codes, calling helpers for each record kind.
- `register_jump_return_slot`: Stores the current position into a table entry, sets a return handler pointer in the data block, and returns a position count unless a byte flag is two.
- `check_state_flags`: Checks if certain flags or bytes in a global structure are non-zero and returns if so.
- `classify_symbol_type`: Returns a value by symbol type: for type byte four it checks a marker and returns the constant 24, for type byte five it computes a second value, and otherwise it returns the incoming value.
- `initialise_list_head`: Sets a word counter to zero, points a list head at it, and clears a byte field in the data block.
- `append_plus_marker_record`: Appends a double-plus marker with the argument word to a list buffer, then writes a zero terminator and advances the list position.
- `append_plus_marker_with_type`: Appends a double-plus marker, a marker byte of all-ones and a type byte taken from the entry to the list buffer, then writes a zero terminator.
- `update_global_state`: updates several global or pointer-based values and calls another routine using a pointer-to-pointer, returning a value from a register.
- `reset_state_unless_flag_zero`: Returns the helper result when the zero flag is set.
- `process_list_and_loop`: Modifies a flag and a counter, then iterates through a list to append a value, followed by a loop that decrements a value derived from a pointer.
- `save_registers_to_memory`: Saves a set of registers to a memory location, updates a flag, and traverses a list to append a value.
- `report_malformed_line`: Sets the flag and raises a counter as in other handlers, then calls helpers that report a malformed line of input.
- `dispatch_conditional_helper_calls`: Calls a helper, then chooses between further helper calls depending on a state byte and the zero flag.
- `init_record_pointer_plus_fourteen`: Stores the current address in its own slot, advances it by fourteen bytes, and copies a record field depending on a helper result.
- `init_list_node_copy_field`: Stores a self-referencing pointer advanced by fourteen bytes into the record, reads a size from the argument and calls a helper, then copies a field when the test passes.
- `walk_list_entries_with_helpers`: Examines the first entry of a list, calling a helper when its link is set.
- `clear_field_and_release`: When a field of the entry is nonzero, zeroes it and calls a helper routine.
- `visit_matching_list_nodes`: Walks a list of nodes, calling a helper for each child link and invoking a callback when the node's type byte matches or is zero.
- `visit_child_list_entries`: For a node with a child list, recursively visits the child entries, calls a callback, and visits the next sibling when one exists.
- `visit_node_then_siblings`: Calls a helper for the node's child when it is set, invokes a callback on the node, then recurses into the next sibling.
- `release_first_list_entry`: Calls a helper, then calls another helper when the first entry of a list is nonzero.
- `invoke_callback_when_flagged`: Calls the supplied callback when bit five of the entry's attribute byte is set.
- `visit_child_entries_then_release`: For a node with a child list, recursively visits its children, calls a helper for the node, and visits the next sibling when present.
- `walk_sibling_chain`: Calls a helper for the node's first child when it is set, calls a second helper for the node, then recurses into the next sibling.
- `invoke_callback_when_bit_four`: Calls the supplied callback when bit four of the entry's attribute byte is set.
- `append_first_list_entry`: Sets the head and tail pointers of a list in the data block to a new entry when the flag is clear, zeroing the entry's link and size fields.
- `compute_list_entry_index`: Computes an entry offset from a base value and the current position, then reads a size field from the list tail when the flag is clear.
- `append_list_entry_fields`: Appends a new entry to the list tail, storing its link, size and two field values from the arguments.
- `match_list_entry_key`: Compares its key with a stored value and, on a match, reads the entry's length and checks a field.
- `advance_list_head_entry`: Moves the list pointer to the next entry when one exists and clears its size field.
- `set_key_to_minus_one`: Sets a stored key in the data block to minus one and returns zero.
- `skip_strings_and_call_helper`: Skips a counted number of null-terminated strings, then calls a helper for each following string until an empty string is reached.
- `call_helper_for_strings`: Skips over strings while a count is still nonzero, then calls a helper for each remaining string until an empty one.
- `emit_newline_after_wait`: Waits while a status word equals 130, then appends a newline character to the output buffer and advances the output pointer.
- `emit_space_after_helper`: Calls two helper routines and returns a result pair whose second part is the character 32, a space.
- `tiny_moveq`: Small routine of 1 instruction that loads 32 into d1.
- `call_helper_pass_through_pair`: Calls a helper routine and returns its two incoming register values unchanged.
- `lookup_nibble_table_entry`: Looks up a byte in a table in the data block, indexed by the low four bits of the argument, with helper calls before and after the lookup.
- `lookup_nibble_entry_with_helper`: Calls a helper, then looks up a table byte indexed by the low four bits of the argument and returns it with a further helper call.
- `lookup_nibble_entry_after_helper`: Calls a helper, looks up a table byte indexed by the low four bits of the argument, then calls another helper and returns the merged result.
- `lookup_low_nibble_byte`: Looks up a byte in a table in the data block using the low four bits of the argument, calls a helper and returns the value.
- `rotate_nibbles_and_dispatch`: Rotates a 32-bit value one nibble at a time and dispatches each nibble through a computed jump, skipping leading zero nibbles.
- `search_threshold_table`: Searches a table of longword thresholds for the first entry greater than the argument and calls a helper with the matching index.
- `search_threshold_table_and_dispatch`: Searches a table of longword thresholds for the matching entry and dispatches through the jump target that entry names.
- `store_word_from_pointer`: Stores a word from a pointer-based address into another memory location.
- `walk_list_and_release_entries`: Calls helpers, then when the list head is set walks its first entry, calling helpers for links and tags.
- `dispatch_on_entry_type_byte`: Calls helpers and branches on a type byte of the entry.
- `return_helper_result_with_space`: Calls two helper routines and returns the result of the first with the character 32, a space, as its second part.
- `emit_repeated_entries`: When a flag is set, calls a helper once for each unit of a count stored in the entry.
- `fill_marker_buffer`: When the given entry is present, fills a 128-byte buffer in the data block with a marker sequence that starts with the value 127 followed by all-ones bytes.
- `find_matching_entry_byte`: Checks an entry's flag bit and whether its threshold byte is not above the argument, then scans the entry's data bytes for a match.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_and_release_stored_block`: When a stored block pointer is set, clears it, subtracts the block's recorded size from a running total and calls a helper to free it.
- `append_to_list_tail`: Walks a list from its head to the last entry, calls a helper, and links the new entry after the tail.
- `set_buffer_limit_and_store`: Sets a 16-bit limit field to ten thousand two hundred and forty, calls a helper and stores its result in a pointer field.
- `release_stored_buffer_block`: When a stored block pointer is set, releases its contents through a helper, subtracts its recorded size from a running total and calls a release helper.
- `subtract_block_and_release`: Calls a helper when a stored block's first longword is set, subtracts the recorded block size from a running total and calls a release helper.
- `initialise_output_buffer_state`: Clears two state bytes and a counter, then points two list heads at a buffer area in the data block.
- `flush_when_count_set`: When a counter in the data block is nonzero, calls three helper routines to flush pending state.
- `clear_counter_and_notify`: When a counter in the data block is nonzero, clears it and calls a helper if the old value differs from a stored value.
- `write_chars_until_buffer_full`: Copies the low byte of the argument into a buffer in the data block, advancing its pointer until the end is reached.
- `reset_buffer_pointer`: Resets a buffer pointer in the data block to its base when it has moved, and calls a helper when it has.
- `advance_output_counter`: Increments a 16-bit counter in the data block and calls a series of helpers, with the path chosen by a flag byte.
- `advance_index_until_limit`: Clears a field, increments a 16-bit index in the data block, and calls a helper when the previous index differs from the limit.
- `emit_characters_from_index`: When a flag is set, loops emitting characters from a counted index in the data block, calling helpers and wrapping the index when it goes negative.
- `emit_null_terminated_string`: Calls a helper for each character of a null-terminated string until the terminator is reached.
- `format_decimal_word`: Checks a 16-bit value against ten thousand and calls helper routines to write its digits, with a carry from a byte shift choosing the branch.
- `stub_return_only`: Contains only a return instruction and does nothing.
- `compare_mode_and_return`: Compares a mode word with three and returns without any action in either case.
- `copy_string_with_comma`: Copies a null-terminated string into a destination buffer and then appends a comma character.
- `check_hunk_record_type`: Compares values with AmigaDOS hunk type codes and calls helpers for the matching record, skipping records when a state flag is set.
- `compare_flag_and_return`: Compares a flag in the data block with zero and returns without any action in either case.
- `check_flag_and_return`: Compares a flag in the data block with zero and returns without any action.
- `return_void_2`: The routine is empty and performs no operations before returning.
- `advance_position_by_argument`: Adds the argument to a position counter in the data block when a mode byte is not two and a flag is set, then calls helpers.
- `compare_mode_two_and_return`: Compares a mode byte with two and returns without any action in either case.
- `copy_bytes_and_advance_position`: Adds the count to a position counter in the data block, then copies that many bytes from the source to the destination pointer.
- `emit_space_if_mode_two`: When a mode byte is two, calls helpers and returns the result with the character 32, a space, appended.
- `clear_pending_flag_after_helper`: When a pointer field in the data block is set, calls a helper and clears a pending flag.
- `advance_two_counters_and_clear`: Clears a byte at the pointer, increments two counters in the data block, and calls helpers when a flag is set.
- `clear_longword_field`: Clears a longword field in the data block.
- `check_hunk_block_value`: Returns the incoming value unless a flag is set, in which case it compares the value with AmigaDOS hunk type codes and returns the matching result.
- `release_field_and_clear_pointer`: When a pointer field in the data block is set, calls a library helper and clears the field.
- `repeat_helper_while_remaining`: Subtracts a fixed amount of eight thousand nine hundred and sixty-six from the argument until nothing remains, calling a helper on each pass.
- `find_line_end_offset`: Scans characters from the input until a newline is reached and returns the offset where the line ends, or minus one when none is found.
- `do_nothing_2`: this routine is empty and performs no operations.
- `copy_and_advance_pointer`: Increments a pointer or a global variable and copies a certain number of bytes from a source to a destination.
- `traverse_linked_list`: Traverses a linked list of pointers, checking for a null terminator and performing string-like operations on the data.
- `update_block_data_2`: Checks a condition, calculates a new address, and copies a sequence of bytes (including padding) to a destination.
- `update_block_data`: Calculates a new address by adding two pointers, copies a sequence of bytes from a source to a destination, and pads the end of the destination with null bytes.
- `execute_recursive_branch`: Checks a byte against a value at an offset, then recursively calls itself and may store a register value into a memory location if a loop condition is met.
- `do_nothing_3`: the routine is empty and performs no operations.
- `verify_and_store_data`: Performs a recursive check against several memory-based flags and, if certain conditions are met, stores a value from a register into a memory location.
- `copy_padded_record_name`: Copies a record's name bytes to a destination, first making an unnamed call when a counter is below the padded name length, then zero-fills the tail to a four-byte boundary.
- `store_register_after_loop`: Executes a loop checking a register value and then stores a register's contents into a memory location.
- `check_register_value`: checks if a register equals a specific value and calls another routine if it does not.
- `do_nothing_4`: The routine performs no operations and returns.
- `call_three_routines_in_order`: Executes three unnamed routine calls in sequence and returns.
- `scan_linked_list_records`: Performs a conditional check on a byte and then enters a loop that scans a linked list of records until a certain value or a null pointer is encountered.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning to the caller.
- `scan_until_minus_one_result`: When a field of a pointer block is nonzero, it calls an unnamed routine, repeats calls until the returned value is minus one, then makes one final call.
- `set_pointer_field_plus_ten`: Stores a pointer plus ten bytes into a field at an offset within a block referenced by another pointer.
- `set_block_field_plus_ten`: Stores a value equal to a pointer plus ten into a field at an offset of six within that pointer's block.
- `find_matching_record_index`: searches a linked list of records for an entry that matches a specific type and value, or a type and a word field.
- `update_memory_with_pointer_and_data`: Stores a value into a global variable, calculates a size, and copies a sequence of bytes from a source to a destination, followed by zeroing certain global variables and checking a pointer.
- `append_string_copy_with_comma`: Copies a null-terminated string to a destination, appends a comma, and then appends a second copy of the original string.
- `reset_buffer_and_pointer`: Calls a library routine, then clears a 16-bit value and sets a pointer to a new address.
- `flush_output_then_call`: Calls the library routine at vector offset minus 48, then calls an unnamed routine.
- `allocate_memory_resources`: Allocates memory via a library call and initializes a pointer and certain memory-resident or global values, returning a pointer or a specific error code.
- `reset_game_state_fields_2`: Clears several internal state variables and certain pointers to zero or default values, and reinitializes a string buffer with a default file extension if a certain flag is set.
- `find_path_separator`: scans a string for a forward slash or colon and returns a pointer to the position immediately following the last found separator, or a failure value if none are found.
- `reset_counter_range`: resets a certain range of a counter or index and calls another routine if a certain condition is met.
- `flush_counter_when_nonnegative`: When a counter is non-negative, it calls a routine, calls a library routine, and resets a pair of counters.
- `handle_break_signal_flag`: Polls task signals through a system vector and, when the break bit (bit 12) is set, ORs 127 into a status byte, issues a second signal call, and runs an unnamed routine if a stored pointer is set.
- `print_null_string`: Iterates through a null-terminated string in memory, processing or checking each character until a null terminator is encountered.
- `call_two_library_functions`: Calls two library functions in sequence through the library base, passing the low word of the state pointer to the second call.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies 32 to (a3)+, then returns.
- `allocate_segment_block`: Allocates a block of the requested size plus eight bytes, records that size in the block's first longword, stores the block address shifted and plus one in an output longword, and clears its link field.
- `allocate_counted_block`: Allocates a block of the requested size plus four bytes through the system vector.
- `terminate_name_around_call`: Writes a zero byte just past a counted name, calls another routine, and then writes a byte of value 11 at the same position.
- `scan_to_colon_or_end`: Advances through a string until it reaches a colon or the terminating zero byte, then returns.
- `copy_string_and_lookup`: Performs a library lookup and, if certain conditions are met, copies a string from a source pointer to a destination pointer before performing further string or library operations.
- `run_library_lookup_sequence`: Runs a library lookup sequence: a call at vector offset minus 84, then a call at minus 102 when that succeeds, a close at minus 90, a call at minus 30, and a final call at minus 36 when a state field is empty.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `normalise_library_result`: Calls the library routine at vector offset minus 42 and returns zero for a non-negative result, otherwise passing the negative result through.
- `stub_single_call_return`: Calls another routine and then returns.
- `call_library_vector_minus_24`: Calls a library function at a specific offset from the library base.
- `call_library_vector_minus_42`: Calls the library function at vector offset minus 42 from the library base.
- `copy_name_and_check_library`: Copies a name into a fixed buffer, appending it after the existing path when the name starts with a period or is empty, and stops at a limit character.
- `flush_when_nonzero`: When the data register value is nonzero, it calls the library routine at vector offset minus 48.
- `call_library_vector_minus_66`: Calls the library routine at vector offset minus 66 and returns.
- `execute_conditional_function`: Checks a byte in a global or offset-based memory location and calls a function pointer if that byte is zero.

### spy2

- `START_CODE`: Jump stub: passes control straight on to START.
- `DUMMY_SPRITE`: Busy-waits until the value in the vertical position register or a certain range relative to a register value is met.
- `ATARI_COPY`: Copies 4000 steps from a source, writing words into four separate arrays spaced 8000 bytes apart.
- `COLOUR_COPY`: Copies sixteen words from a source pointer into a global area, shifting each word left by one bit.
- `LOAD_FILE`: Opens the DOS library through the exec library, stores its base address in a global, and then calls two further library functions through that base.
- `SETUP_SCREEN`: Sets the current and alternate screen pointers, copies 56 words of plane pointers into the second copper list, and programs the display window, data fetch, modulo, control and copper registers.
- `WHICH_PLANES`: Writes four bitplane addresses into a table, splitting each address into low and high words.
- `BLIT`: Waits for the blitter to finish, then writes saved register values to the masking, pointer, modulo, control, and size registers.
- `WAIT_BLIT`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `SWAP_SCREEN`: Waits for the blitter and then loops forever.
- `CLEAR_SCREEN`: Clears a block of memory by writing zero longwords through an address register, four at a time.
- `MYINT2`: Calls another routine, clears bit six of a serial control byte, waits in a short loop, and then writes eight to the same byte.
- `ADD_MATRIX`: Calculates a bit position and index based on a global variable and sets or clears a bit in a keyboard matrix table.
- `DOWN_STROKE`: Clears a specific bit within a byte at a memory location determined by a base address and an offset.
- `INIT_KEY`: Writes an all-ones longword value into four consecutive global longwords.
- `INKEY`: Tests the bit in a global bitmap that corresponds to a given code, returning a specific value if the bit is set or clear.
- `SCAN_JOY`: Reads the fire-button register and the joystick and mouse port movement registers, and stores derived direction and button words in globals.
- `HANDLE_MOUSE`: Sets two global words to minus one or zero depending on specific bits of the button register.
- `DO_MOUSE`: Decrements a four-byte counter in a state record by rotating the byte order and steps a separate counter toward zero based on a comparison.
- `LAST_MOUSE0`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `LAST_MOUSE1`: Calls a routine to find memory and then assigns the returned address to several global pointers.
- `KLEFT1`: Small routine of 1 instruction that merges 0 into d0.
- `ALLOCATE_MEMORY`: Locates available memory blocks and stores the returned bases into three global variables for the onscreen, twoscreen, and background buffers.
- `CLEARD0`: Writes zero into eight thousand consecutive longwords starting at a given address.
- `FIND32K`: Calls a library function to allocate memory and enters an infinite loop if the returned pointer is null.
- `DECO_PIC`: Copies a certain number of words from a source, then unpacks run-length encoded data into a destination buffer.
- `DOS_BASE`: Calls a function, then repeatedly calls a sound routine while waiting for a specific bit in a global variable to toggle.
- `DOS_HANDLE`: Calls a function pointer from a table, then executes a loop that triggers sound or music updates while waiting for a specific bit in a global variable to change.
- `ONESCREEN`: Calls a routine via a pointer, starts audio, and loops a sequence of sound calls while waiting for a specific bit in a global variable to toggle.
- `TWOSCREEN`: Calls a routine via a pointer in a global variable, starts sounds, and then executes a loop that repeatedly calls a sound-creation routine and toggles a music switch while waiting for a bit change in a global variable.
- `SCREEN2`: calls an external routine to clear or reset a screen, starts sound playback, and executes a loop that waits for a specific bit in a global variable to change while decrementing a counter.
- `SCREEN1`: Calls a routine via a pointer, starts a sound routine, and executes a loop that plays a sequence of sounds while waiting for a specific bit in a global variable to change.
- `COPPER1`: Calls a routine from a pointer in a global variable, starts sounds, and loops a certain number of times while waiting for a bit in a global variable to change.
- `COPPER2`: Calls a routine from a pointer in a global variable, starts sounds, and loops a certain number of unable-to-determine number of times while waiting for a bit in a global variable to change.
- `COPPER_LIST2`: Executes a routine that triggers sound effects and loops while waiting for a specific bit in a global variable to change.
- `E_KISS`: Executes a routine via a pointer, starts any active sounds, and then loops a specific number of times, repeatedly triggering new sounds and waiting for a bit-change in a global flag.
- `E_RUMBLE`: calls a routine via a pointer in a table, initiates a sound, and loops a specific number of times to trigger new sounds while waiting for a change in a global flag.
- `TEST`: Triggers a sound effect, then repeatedly plays music notes while waiting for a status flag to flip on and off.
- `START_SOUNDS`: Fills a memory area with values from a pseudo-random generator, initializes several audio channel records with envelopes and dummy pointers, and configures certain hardware registers including DMA and interrupt enable bits.
- `GO_SOUND`: Fills a sound channel record with a sample pointer, two word parameters, a cleared status word, a flag set to one and a second pointer.
- `HIGH_PING`: Calls one library routine and returns, passing back nothing.
- `MEDIUM_PING`: Calls a routine to play a medium-frequency sound effect.
- `LOW_PING`: Calls a routine to play a specific low-frequency sound effect.
- `DROP_PING`: Performs an indirect call by loading a function pointer from a data structure and jumping to it, serving as a dispatch mechanism for a handler stored in a data table.
- `TAKE_PING`: Calls a routine to populate a two-part record with a pointer, two values, and initialized counters.
- `MEDIUM_BEEP`: Triggers a routine that fills a record structure with a pointer, two values, and specific flags.
- `BANG`: Invokes a routine to initialize a channel record with a pointer, two values, and a flag set to one.
- `OTHER_SMACK`: Performs a call to a routine that populates a data structure with a pointer and two 16-bit values.
- `GRAVESOUND`: Requests the filling of a two-part record containing a pointer, two values, and zeroed counters.
- `BOOM`: Calls a routine to set up a record with a 32-bit pointer, two 16-bit values, and a flag.
- `BURY`: Plays a sound effect by calling a routine that configures a sound channel.
- `SPLASH`: This routine invokes the sound system and then returns without performing any other action.
- `RUMBLE`: Calls a sound routine and returns without performing any further operations.
- `FEET`: calls a routine that configures a sound channel.
- `KISS`: Calls the sound routine and then returns without doing anything else.
- `SUB`: Calls a sound routine and then returns.
- `HANDLE_SOUNDS`: Calls one routine, then calls up to three more in turn, each only when the previous step left the zero flag set, and returns nothing.
- `HANDLE_REQUEST`: If the record's field is zero, searches four global words for the first positive one.
- `NEW_SOUND`: Matches a sound request's ID against stored request slots and marks the matching slot, or updates the stored current value when the flag bit is clear.
- `MYINT6`: When a master enable flag is set, counts down a multi-step counter, calls routines for audio channel envelopes, and writes an interrupt-request value to a global.
- `CHANGE_ENVELOPE`: Steps a voice's volume envelope by reading entries from a sequence its record points to.
- `X8912`: Selects a wave table, wave length, and frequency for the first three audio channels from lookup tables, or configures a noise area if the mode bits specify.
- `PSGW`: Writes the low byte of a register into a global area at an offset determined by another register.
- `INITMUSC`: Sets a hardware register to a certain value, performs three writes to a sound chip, and calls another routine twice.
- `PLAYMUS`: Switches an audio channel, enables a voice, and calls routines to output certain or certain audio signals before switching to another channel.
- `VOICEISOF`: Updates a global structure or table with new values or pointers or increments a value or checks a condition or certain bytes from a table.
- `NEXTSEQ`: Decrements a counter and updates a pointer or certain bytes if the counter reaches a certain value or a new pointer is required.
- `VOICISON`: Counts down a voice's note timer and, when it expires, advances to thenext note.
- `DEGATEIT`: Sets a byte at a specific offset relative to a pointer to zero.
- `CHECK2`: Modifies a hardware register or a global variable and calls a routine to write to the sound hardware.
- `CALCNOTE`: Calls a routine and extracts bits from the result to decrement a count or return a modified value.
- `OUTVC1`: Calls a sound output routine.
- `OUTVC2`: Calls a routine that handles audio output or sound triggering.
- `SNDOUT`: Calls a routine that writes to the sound hardware three times in a row.
- `NOTEMODS`: Modifies a register or a global variable based on the lower bits of a register and a value from a certain address.
- `RESET`: Issues three sound hardware write calls in sequence with no other register or memory modifications.
- `SAVE_INT2`: Performs a sequence of memory allocations, hardware initializations, screen setups, and copies a block of data from one location to another.
- `TEMP_MUSIC`: Executes a series of initializations and copies data from a source buffer to a destination buffer.
- `REG7`: Initializes several global pointers and copies a large block of data between two memory locations.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `MUSINF_1`: Performs a series of setup routines including memory allocation and copying data from a source pointer to a destination pointer.
- `execute_subroutine_zero`: Executes a block of code that copies data from a source address to a destination address and enters a loop.
- `MUSINF_3`: Performs memory allocation and copies a fixed amount of data from one memory area to another.
- `MUSIC_DELAY`: Initializes system components and copies a block of data from one pointer to another.
- `MASTER_ENABLE`: Sets up system-level or screen-level variables and copies data from a source buffer to a destination buffer.
- `START`: Performs memory allocation, initializes hardware, copies screen buffers, and enters a main game loop that checks an abort flag.
- `READALL`: Clears a screen buffer, then performs a long sequence of copying picture data from various files into buffers using specialized grab and save routines, finally setting a color register.
- `MYINT3`: Increments a global variable, clears a certain flag, and writes a constant value to another global variable.
- `GRAB`: Decrements a register while calling another routine for a fixed number of iterations.
- `GRABOBJ`: Small routine of 2 instructions that copies 20 to d3, then subtracts 1 from d3.
- `GRABOBJ1`: Decrements a register while calling another routine until the register reaches a certain value.
- `GRABLAND`: Calls a routine or procedure for a fixed number of iterations using a loop counter.
- `GRABMAP`: Decrements a register while calling another routine until a certain value is reached.
- `PLAYINIT`: Resets or initializes a wide range of global game state variables, including player coordinates, energy levels, and equipment, while copying level-specific terrain and map data from tables into active memory buffers.
- `PLAYINIT0`: Sets automatic-control flags for the players based on player count, copies level-specific map and terrain data into working buffers, resets player or item properties (such as energy, altitude, and position), and initializes rocket and trap-list data.
- `GETA4`: Small routine of 3 instructions that clears d1, then copies (a4)+ to d1, then returns.
- `XGET_STICK`: Uses a level value to index into tables and copies values into certain global variables.
- `MY_VERY_OWN_STACK`: Resets several global status flags and copies a block of memory from a source table to a destination screen buffer.
- `XSTICK1`: Resets several global status flags, copies a block of memory from a source table to a screen buffer, and updates a counter-based input handler that triggers sound effects and state changes based on certain directional or trigger-based input or certain counter-based timing conditions.
- `YSTICK1`: Updates a certain amount of memory by copying a source table, checks for certain button triggers, and updates a global variable or value based on a joystick move or a counter or a sound trigger.
- `PARMS`: A loop that copies a table to a second screen buffer and reads trigger and or joystick input to navigate a menu.
- `DO_GAME`: Initializes game state, manages player or spy movement and collision detection, and updates a countdown timer and screen buffer.
- `READPIX`: Runs a picture decode and copy routine, then sets two global variables to zero and a large value before returning.
- `STUFFIT`: Repeats until its count register reaches zero.
- `STUFFSPY`: Picks a random position for each spy and converts map positions to window coordinates.
- `SETUP_VOLCANO`: Stores certain fixed values into three global color variables and mirrors them into their duplicate copies.
- `COPY_COLOURS`: Copies certain color values into global variables, choosing between two different sets or source pointers depending on whether a register value is even or odd.
- `COL1A`: Calls a routine that generates a random number.
- `COL2A`: Invokes a routine to compute a random value.
- `COL3A`: Triggers a routine responsible for random number generation.
- `RNDER`: Calls a routine to generate a random value.
- `RNDER2`: Calls a routine to generate a random value and returns the result concatenated with a register value as a 64-bit value.
- `GETRAND`: Repeatedly picks random slab coordinates from a random routine until the chosen cell reads as zero, then returns a pointer to that cell.
- `SETXY`: Adds a fixed offset to a value, then stores the quotient in a memory location via a pointer and stores the remainder in another memory location via a pointer.
- `CLEAROUTCRAP`: Sets several global counters and fuel variables to a sentinel value of all ones.
- `L_TENMINS`: Updates rendering parameters, including screen buffer pointers, sprite coordinates, and depth-based offsets, for a spy's view if certain visibility or position conditions are met.
- `L_ONEMINS`: Checks if a spy's window or certain map coordinates overlap with the current view, and if so, calculates screen-space coordinates and buffer offsets to draw a spriter and hands or hands/sprites.
- `L_TENSECS`: Calculates and updates screen buffer pointers, clipping-relative drawing coordinates, and sprite/hand drawing parameters based on the current active spy window and map coordinates.
- `L_ONESECS`: Calculates and sets the screen-space drawing coordinates and buffer pointers for a spy sprite based on its current map position, altitude, and swamp depth.
- `L_S1FUEL`: Calculates a screen-space or or buffer-relative position and scale-related values for a fuel object, then calls a routine to draw a sprite.
- `L_S2FUEL`: Checks if a fuel item's map coordinates and altitude-adjusted depth fall within the current visible window, then calculates a screen-space or buffer-space position to render a sprite and hands.
- `SLABX`: checks if a spy's map coordinates match a specific window and, if so, calculates a new screen-space drawing position and buffer offset before calling a sprite-drawing routine.
- `OLD_BUSY`: Checks if a spy's map coordinates match the current viewing window and, if so, calculates and updates screen-space drawing coordinates, buffer pointers, and depth/swamp-related offsets.
- `SAVE_INT3`: Checks if a spy's map position or window visibility matches a specific coordinate and updates a global rendering buffer, screen, and plot coordinates.
- `DRAWSPY`: Checks if a spy's map coordinates fall within a visible window and, if so, calculates a screen-space position and depth-based offset to render the spy sprite and its hands.
- `DRAWHANDS`: Draws a hand sprite for a player when that player's counter, depth and swamp values are within limits.
- `DRAWBUTTONS`: Calls a single-button drawing routine six times in a row.
- `ONEBUTTON`: Draws one item button sprite on the second screen, using the ITEMON buffer when the flag is unable to be set and ITEMOFF otherwise.
- `SWAPSCREEN`: Jump stub: passes control straight on to SWAP_SCREEN.
- `DRAWTIME`: Checks if certain time-related global variables are non-zero, calls a comparison routine, and calls a character drawing routine if the value is non-zero.
- `HLINE`: Passes the current register values straight to a horizontal line routine and returns.
- `METER`: Restores one unit of energy on a timer when the player's state allows it, and sets the refresh flag.
- `FLASH`: Chooses between two drawing paths based on the counter's low bit and the upper bits of the chosen player's hand value, calling another routine in each path.
- `DRAWPIECE`: Sets a global variable, calls a sprite routine, and then sets the global variable to a different value.
- `DRAWSEL`: If a certain flag is set, it checks a bit in a counter and calls a character routine several times, skipping certain calls based on that bit.
- `COMPARETWICE`: Compares a 16-bit value against a stored two-word pair and updates the pair if the value or any part of the pair or if any part of the pair differs.
- `RSPRITER`: Calculates a bitmask by inverting a bit-shifted version of the sprite source, then applies this mask and the shifted source pixels to the even and odd bitplanes of the screen buffer.
- `RBACKER`: Copies or merges words from a source pointer into a destination buffer, including bit-shifting or masked bitwise operations across multiple planes.
- `copy_four_plane_words`: Copies words from a source pointer into a screen buffer at large offsets, using bitwise operations to mask and merge data into multiple planes, and branches to another routine upon completion.
- `RSAVE_BUFF`: Reads four bitplane-sized words per cell, row by row, and writes them consecutively into a buffer of four-word records.
- `RDRAW_BUFF`: Reads consecutive four-word records and writes their words back into four bitplane-sized blocks, row by row.
- `RSTUFF_BUFF`: Writes a size header for a rectangle, then packs each cell into the buffer as the OR of its four bitplanes followed by the individual planes.
- `RSTUFF_BUFF_SHORT`: Writes a size header for a rectangle, then copies each of the four bitplane words of every cell into the buffer in order without combining them.
- `RSPRITER_AM`: Programs the blitter's channel pointers, masking, and size, then starts it for a masked sprite copy, waiting for the blit to finish.
- `SPRITER`: Calls a shared subroutine and returns the two input registers unchanged.
- `SPRITER_AM`: A thin wrapper that calls a blitter sprite routine using the current register values.
- `BACKER`: Calls a shared subroutine and returns the two register values unchanged.
- `X`: Sets a specific bit in a byte at a certain memory location using a bitmask derived from a register.
- `WIDTH`: sets a specific bit within a byte located in a memory buffer using a bitmask, then advances a pointer to the next character.
- `BUFFER`: sets a single bit within a byte at a memory location pointed to by a register, then increments the pointer.
- `SCREEN`: sets a single bit within a byte at a memory location pointed to by a register, then increments the pointer to the next byte.
- `MINTERMS`: updates a byte in a memory table by setting a specific bit based on the lower three bits of a register.
- `PUTCHAR`: Calls a routine to draw a character and returns.
- `NEXTCHAR`: Looks up the character cell in the font table from the low nibble of the input and copies eight rows into four bitplanes at the target.
- `CALCAD`: The routine performs no functional operations or data manipulations before returning.
- `PLOTPOINT`: Applies a clear mask and then a set mask to four words spaced 8000 bytes apart, using the data register values.
- `LEFT_RIGHT`: Checks if a global flag is zero and if the least significant bit of a register is cleared, returning immediately if both conditions are met.
- `UP_DOWN`: Compares a global variable with zero and returns in either case, making no change.
- `DRAW_LINE`: Works out the horizontal and vertical distances between two coordinates and their directions, then steps from one point to the other in increments computed by division, calling routines at each step.
- `GO_LEFT`: Iterates through a memory range, applying a bitwise mask or clear-and-set operation to a word-aligned buffer and several or offset-based memory locations.
- `HORLINE`: Draws a horizontal line by masking or setting bits in a bitplane-based framebuffer, using a bitmask or a constant bit pattern to modify memory or skip pixels via a secondary routine.
- `POINT`: Combines the upper half of a 32-bit register with a 16-bit register and masks the result to keep only certain bits for a pixel position and bit index.
- `GET_RANDOM`: Updates a three-word random state using addition, subtraction, and a bitwise rotation or folding technique.
- `READ_TRIGS`: Scans the joysticks, then sets each player's trigger word from the fire button for that player's control mode, reading the keyboard when the mode calls for it.
- `MOVE`: Determines the next movement direction for a spy using a combination of joystick input, AI logic, or random movement, then checks for collisions with map objects such as walls or fuel, and updates movement-related flags and collision counters.
- `set_spy_direction_four`: Called from the MOVE branch, it stores 4 into the spy direction variable and writes 2 through an address register pointer, then returns.
- `run_spy_turn_logic`: The 585-instruction body is represented here only by a fragment that stores 3 into the spy direction variable and writes 1 through a pointer.
- `CHECKMEET`: Compares the two spies' map rows and horizontal positions to decide whether they share a screen window, recording which spy holds it and where.
- `CHECKMEETEND`: Tail of the meeting check: when the counter held in a data register is greater than one, it sets both buffer row markers to an invalid all-ones value, then returns its argument unchanged.
- `SETTEMPS`: Chooses player one or player two from its input value and copies that player's window and map coordinates into the current spy position variables.
- `DIRFIX`: Maps the spy direction (one to four) to one of four frame buffer addresses for player one or player two, advances that player's frame counter with wrap at four, and resets the counter to zero for any other direction.
- `TEST_OPTIONS`: When the cooldown flag is zero it polls the option test routine; otherwise it decrements the flag, so repeated option input is held off for a number of frames.
- `TEST_M`: Polls the input handler, sets a cooldown value, and toggles the music switch.
- `JOYMOVE`: Scans the joystick, waits for button states in a polling loop, sets the abort flag on a second press, and when the brain flag is clear derives the spy direction from the joystick lookup table entry for the active player's mode.
- `SETWINDOW`: Copies one of the two saved window positions into the current spy window coordinates; window two is used when the window state is two or the argument is nonzero, except when the state is one.
- `GETWINDOW`: Copies the current spy window coordinates back into the window slot for player one or player two, using the same selection rule as the window fetch routine.
- `LITTLE_DELAY`: Spins a 16-bit counter from a fixed start down until it wraps, producing a short pause, then returns its argument unchanged.
- `FEET_FLAG`: Per-frame redraw: draws the fuel gauge, flash, and selector, redraws each player's land and objects unless its window is active, draws buttons, time and ropes, starts a death sound once per player when energy drops below zero, and updates the meter.
- `DRAWMOVE`: Performs the same per-frame drawing sequence as the playfield redraw, called from the game loop for the movement phase, with the fuel, land, object, button, time, rope and meter steps.
- `DRAWLAND`: Stores the landscape window position, sets the background pointer and upper row, redraws slabs when the row or column has changed, then programs the blitter registers for the window and waits for each blit to finish.
- `DRAWLAND2_0`: Sets the blitter control mode, blits the landscape, draws a slab, then programs the blit offset and size registers and waits for each blit to complete.
- `DRAWLAND3_0`: Performs the same blitter setup and slab drawing as the first landscape blit routine, used as a second entry point for the landscape draw path.
- `ONE_SLAB`: Computes the source address for one landscape slab from its index and row, sets the blitter control and mask registers, and waits for the blit to finish.
- `DRAWSLAB`: Draws one landscape slab, then draws it a second time when the slab index is zero or lies in a middle band that excludes its ends, and the address is aligned to eight.
- `DRAWBACK`: Copies a buffer of 32-bit word pairs into the back-screen area, advancing the row pointer by a fixed stride for each pair, for a count taken from the width value.
- `DRAWOBJ`: Scans the map cells of the current window row by row, skipping empty and excluded tile codes, and draws each object sprite selected from its tile code through the sprite routine.
- `DRAWLAND5_2`: Draws the cover sprite over a window at one of two horizontal positions chosen by the player argument, using the cover buffer and the second screen buffer, then calls the sprite drawing routine.
- `DRAWROPES`: For each player whose rope-line flag is set, draws the pending line and clears the flag, so each line is drawn only once.
- `DRAWFUEL`: Reads a player's fuel value and draws the gauge as a row of horizontal line segments, filling the segments that are used and leaving the remainder empty.
- `CLEAR_WINDOW`: Clears a window-sized region of the second screen buffer with four blitter fills, waiting for each to finish and using the window offset for each row.
- `SEL_FLAG`: Reads the trigger state and uses per-player delay counters to throttle it, then updates the selection state from the trigger bits.
- `BUSY`: Per-frame state handler for one spy: after throttling on the player delay counters it reads the trigger state, calls the joystick reader, and calls the trap-adding, hand-placement, state and sound routines as the spy's situation requires.
- `BUSY4_A`: Clears the player's hand item and places it on the map at the player's cell.
- `DELETE_TRAP`: Searches the trap list of up to 100 six-byte entries for one that points at the given map cell.
- `ADD_TRAP`: Searches a list for an empty slot and adds a new entry containing a pointer or value, while selecting a flag based on an input byte.
- `COUNTER1`: When no window is active it reads the joystick and the swamp and depth values, then for input indices zero to three sets one of four map display widths and shows the map.
- `clear_flash_and_move_delay`: Clears both flash counters, then for the chosen player moves any shared delay value into that player's delay slot and clears the player's count.
- `BUSY6_4`: Per-spy movement routine that calls the joystick reader, the random number routine and the hand-placement routine.
- `SETSAFE`: Sets the safe counter of the chosen player to four.
- `KILLDOUBLE`: Stores the global counter minus twenty into the time slot of the chosen player, leaving the other player's slot unchanged.
- `GETPOS`: Small routine of 2 instructions that shifts d1 right by 6, then shifts d2 right by 4.
- `GETPOS1`: Sets the player's horizontal sprite position from a column value scaled up, plus a fixed offset, and stores a combined row and column value in the paired position variable.
- `SETSTATE`: Sets the state counter of the chosen player to the value passed in a data register, first moving any pending shared delay into that player's delay slot.
- `GETBTIME`: tests if a value in a certain register is zero and returns if so or otherwise returns without performing any operations.
- `SHOWMAP`: Positions a sprite at a fixed column and at a row taken from the argument, selects the map buffer for the current level, and draws it with the sprite routine at width six.
- `ABSD2`: the routine executes no operations before returning to the caller.
- `DRAWROPE`: Computes the two endpoints of a player's rope line from the player's plot position and facing direction, stores them in that player's line slot and sets the draw-line flag.
- `CLEANTRAIL`: Sets all six trail entries of both players to an invalid all-ones value.
- `DELAY1`: Loops while the input is held, swapping screens, copying the back buffer and drawing horizontal line bands and cloud sprites each pass, then clears the sound counter and returns when input ends.
- `SUBCLOUDS`: Draws the cloud sprite at a horizontal position from the first argument and a vertical position from the second, using the cloud buffer at width one.
- `COPYBACK`: Copies the whole front screen buffer into the back screen buffer, four words per step.
- `PARADROP`: Computes a count from the demo timer, draws the sprite twice per pass while the count is above one, then pauses in a busy wait.
- `GETMODE`: Copies the front screen to the back, redraws the menu blocks and sprites, then loops on a demo countdown while joystick up and down input moves the selected item within its range.
- `DRAWITEMS`: Draws seven menu item frames as pairs of line segments.
- `SELECTITEM`: Empty routine that returns at once without doing any work, so it is a stub.
- `DRAWLINE`: Thin wrapper that calls the line drawing routine and returns its argument unchanged.
- `RSPRITERBOTH`: Draws a sprite into the front screen buffer through a mask: each row's sprite words are shifted by the bit offset from the position argument, combined with the screen words, and written across the four bitplanes.
- `RDRAW_BOTH`: Copies a block of four-plane words from the buffer into the front screen at the position given by the buffer offset, advancing by a row stride between rows.
- `RCLEARBLOCK`: Fills a block of screen rows by setting the first bitplane to all ones and clearing the other three planes.
- `PX1`: Clears the pending-kill flags, enables brain control, reads joystick movement, and then either holds the spy still or chooses its next step from its direction and map state.
- `PX2`: Same pseudo-code as the first player's movement routine, which the decompiler shows with identical content.
- `BRAINMOVE`: Computer-controlled movement for one spy: reads joystick state, handles the trouble and nudge flags and the stand-still condition, then picks a step using the random and mound search routines and sets the spy state.
- `GETD0D1`: Sets the spy direction from a four-bit mask: bit zero gives three, bit one gives four, bit two gives one, bit three gives two, and no set bit gives zero.
- `BRAINBUSY1`: Sets a global variable or a pointer-referenced value to zero or a fixed constant based on a register value.
- `reset_spy_two_busy_state`: Clears the second player's F counter and stores the fixed busy value in the pointed-to slot, without the zero test that its caller applies.
- `RANDMOVE`: With a one-in-thirty-two random chance, picks a direction from two random bits, sets the spy direction, and loads a random nudge value from zero to seven into the chosen player's nudge counter.
- `FIND_A_MOUND`: Scans a small block of map cells starting at the spy's cell for an occupied cell that passes a code test, records its position in the spy's position slots, and stores a bad-move flag for the chosen player.

### begin

- `jump_to_init`: Jump stub: passes control straight on to INIT.
- `E_EXPLODE`: Sets bits in a status value, calls the stored routine, starts the sounds, then runs ten dive and explode passes, each waiting on a status bit, and finally restores the saved interrupt value.
- `E_DIVE`: Runs the same pseudo-code as the explosion sequence: set bits, call the stored routine, start sounds, ten dive and explode passes waiting on a status bit each, then restore the saved value.
- `TEST`: Runs the same ten-pass dive and explode sequence as the effect handlers, starting the sounds first and restoring the saved interrupt value at the end.
- `START_SOUNDS`: Fills an 8192-byte area with a generated pseudo-random byte sequence, clears the music switch and music pointer variables, and clears two audio channel words before setting one audio period register.
- `SHOOT`: Stores a flag derived from the processor overflow and negative status bits, then hits bad instruction data.
- `SHOOT2`: Writes a fixed length value to the fourth audio channel length register.
- `EXPLODE`: Starts the explosion sound on the fourth audio channel by pointing it at the random table, setting its length and period to a fixed value, clearing its state, and installing the explosion effect as the pending routine.
- `DIVE`: Sets the second audio channel's period and volume registers, points it at the triangle wave table, clears its state, and installs the dive effect as the pending routine.
- `MYINT6`: Interrupt handler: when the master enable is set, counts down the music delay and, on each third tick, plays the next music step and updates the sound chip, then steps each of the four envelopes and clears the interrupt request bit.
- `advance_music_playback_step`: Fragment of the music interrupt handler: resets the music delay, plays the next music step and updates the sound chip when music is switched on, steps each of the four envelopes, and clears the interrupt request bit.
- `CHANGE_ENVELOPE`: Advances one sound envelope: when its segment counter is zero it loads the next segment from the envelope table unless a stop marker is reached, then applies the segment's period and volume deltas and writes the results to the audio channel.
- `X8912`: Plays the next note on a music channel: selects the wave table or the noise source from the channel record, writes the audio channel registers, and either resets the envelope state or hands off to the envelope routines.
- `DO_ENVEL1`: When the channel one restart flag is set, loads the envelope start values, then writes the audio channel one registers from the wave and frequency tables and hands off to the noise or tone completion routine.
- `XNOISE1`: Starts a noise effect on channel one: loads the random noise table with a fixed length, sets the channel's noise period and volume from the music record, and hands off to the tone completion routine.
- `XDONE1`: Completes a channel one note: writes the audio channel one registers from the wave and frequency tables, applies the envelope values when the record's flag is set, and then passes control to the channel two envelope routine.
- `DO_ENVEL2`: Restarts the channel's envelope when its restart flag is set, clears that flag, and hands off to the channel two completion routine.
- `XNOISE2`: Starts a noise effect on channel two in the same way as the channel one noise routine, then clears the channel's restart flag.
- `XDONE2`: Clears the restart flag held in the channel record it is given.
- `PSGW`: Writes one byte to the sound chip register chosen by the index argument.
- `INITMUSC`: Sets the sound chip mixer register to its initial mode, writes it, then silences both music voices.
- `PLAYMUS`: Plays the next step on both music voices in turn: selects each channel, enables its voice, and outputs its values to the sound chip.
- `VOICEISOF`: Silences the voice and resets its sequence state, then reads the first sequence entry and loads its duration, pitch offset and volume fields.
- `NEXTSEQ`: Steps the voice's sequence pointer by one three-byte entry; past the end of the sequence it wraps back to the start, then loads the entry's duration, pitch offset and volume fields.
- `VOICISON`: Counts down the voice's note timer.
- `NEXTNOTE`: Handles a note event for a voice: while a note is sounding it counts it down and writes the output, otherwise it steps the sequence pointer, looks up the duration in the duration table, and writes the new note value.
- `NEXTNOTO`: Steps the voice's sequence pointer and takes the duration from the duration table.
- `CONTSONG`: Reads the duration from the duration table for the current sequence position and returns low codes unchanged.
- `resolve_voice_note_value`: Clears the voice's gate byte, then returns the low note byte directly when its high nibble is zero.
- `store_computed_note_value`: Runs the sound chip check and note calculation routines, then stores the computed note and the data-register value in the voice record.
- `DEGATEIT`: Clears the gate byte of the voice record it is given.
- `CHECK2`: Selects the sound chip mixer setting for the active channel from the voice state bits, writes it, and sets the channel's envelope value.
- `SNAREA`: Sets the mixer register for the first channel to a noise configuration, sets its envelope value, and writes the chip registers in sequence.
- `CHANNELB`: Chooses the mixer and envelope settings for the second channel from the voice state, writes them to the sound chip, and sets the channel envelope value.
- `SNAREB`: Sets the mixer register for the second channel to a noise configuration, sets its envelope value, and writes the chip registers in sequence.
- `CALCNOTE`: Takes the modifier value, extracts its upper nibble, and when its low three bits are nonzero lowers the nibble by one or two steps, returning the adjusted value.
- `OUTVC1`: Sends the first voice's output to the sound chip.
- `OUTVC2`: Sends the second voice's output to the sound chip.
- `SNDOUT`: Writes three values to the sound chip registers in sequence, using the register write routine.
- `NOTEMODS`: Applies a modifier to a note byte: a modifier below eight is added to the low nibble, and a larger one is subtracted, with the high nibble adjusted on overflow or borrow.
- `NOTEMOD5`: Lowers a note by a negative modifier, handling the carry from the low nibble and adjusting the saved high bits, then writes the result to the note output register.
- `NOTEMOD6`: Combines the low nibble of the note with the saved high bits and writes the result to the note output register.
- `XXMOD5`: Works like the negative modifier routine but first masks the modifier to its low three bits.
- `XXMOD1`: Combines the note byte with the saved high bits and writes the result to the note output register.
- `RESET`: Writes three sound chip register values in sequence to reset them.
- `INIT`: Allocates memory, starts the sound system, sets up and clears the screen, decodes and copies the picture data, builds the fade mask table, blits the title pieces, and runs timed waits before the jets move in.
- `BLIT_ME_HONEY`: Combines a source plane set with a mask over the whole screen, one word at a time across four bitplanes, using AND then OR into the destination.
- `PIXEL_PIECE`: Calls the piece blit routine sixteen times in succession, once for each piece row.
- `BLIT_ME_HONEY_PIECE`: Blits one picture piece through a mask: for each of twenty rows and four bitplane words it clears the masked bits in the destination and then ORs in the source bits.
- `MOVE_THE_JETS`: Copies the first screen buffer to the second, then runs two passes of the jet drawing routine across the screen, each pass stepping the position forward and blowing a hole every eighth step.
- `DO_RESTORE_OF_JET`: Copies a 32-row block of background words from the second screen back to the first screen, at the column given by the jet's saved x offset and the row argument.
- `SHOW_JET`: Waits for the raster position, disables the master enable, restores the background, advances the jet frame index, then draws 32 rows of five masked words into both screen buffers.
- `WAIT_FOR_RASTER`: Busy-waits until the video beam position counter falls inside a window of lines starting at the given value.
- `BLOW_A_HOLE`: Advances the hole index modulo eight and, while the hole position is inside the screen bounds, moves the hole graphic twice.
- `MOVE_A_HOLE`: Copies a block of four by three words through a mask into the destination planes, then either fires the shot routine when its flag is clear or counts the flag down.
- `ATARI_COPY`: Copies a picture of four bitplanes into the screen buffer, one word per step across the planes.
- `COLOUR_COPY`: Converts sixteen palette values by shifting each one left a bit and setting a fixed pattern, then stores them in the colour register table.
- `LOAD_FILE`: Opens the DOS library through the exec library, stores its base and a file handle, then calls two further DOS library routines.
- `SETUP_SCREEN`: Assigns the two screen buffers, copies the bitplane pointer list into the copper list, and programs the display window, data fetch, plane modulo and copper pointer hardware registers.
- `WHICH_PLANES`: Writes four bitplane addresses into the plane pointer list, each 8000 bytes after the previous one, starting from the base address.
- `BLIT_A_SPY`: Starts a blit of the spy sprite with a fixed control value, then issues three follow-up blit setups for the remaining planes.
- `BLIT`: Writes the blitter mask, modulo and control registers from its arguments, then waits for the blit to finish.
- `BLIT2`: Writes the blitter plane pointers and size from its arguments, then waits for the blit to finish.
- `WAIT_BLIT`: Polls the blitter status until its busy bit clears.
- `SWAP_SCREEN`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `CLEAR_SCREEN`: Sets every word of the screen buffer to zero, four words per step.
- `ALLOCATE_MEMORY`: Allocates six 32K memory buffers for the loading screen, the two screens, the title, the jet and the credits images.
- `RESCIND_MEMORY`: Releases six 32K memory buffers by calling the free routine six times.
- `FIND32K`: Requests a block of memory from the system allocator and returns its address, or loops forever if the allocation fails.
- `LOSE32K`: Wrapper that calls the exec library function FreeMem and returns its result to the caller.
- `DECO_PIC`: Loads a picture file, copies its header words, then decodes a run-length compressed picture into the bitplanes, where a marker byte introduces a repeat of the following byte.

### port-handler

- `load_hunk_descriptor_table`: Fills a hunk descriptor from a table of hunk addresses and sizes, storing the base pointers, list heads and end pointers, and compares hunk type codes for executable loading or relocation.

### disk-validator

- `validate_disk_module_entry`: Program entry of the disk validator module.

### ram-handler

- `execute_indirect_call`: Performs an indirect call to a function pointer retrieved from a global address offset.

### serial.device

- `stub_serial_bad_instruction_entry`: Entry of the serial device module that hits bad instruction data at once, so it has no real behaviour.
- `initialise_serial_buffer_fields`: Initialises a serial buffer: when a pointer is given it calls a system library routine to release it, then allocates a buffer of the requested size through the library and resets its pointer, count and size fields.
- `check_flag_and_call_handler`: The routine tests a byte at an offset from a pointer in A1 against a mask and, if the test passes, calls a handler routine obtained from a table via A6.
- `clear_flag_and_dispatch_handler`: Clears a status byte of the request record and, when its status bits are zero, calls a handler through the device base table.
- `disable_interrupts_around_call`: Clears an interrupt enable bit set, calls the routine that follows, then sets the same bits again.
- `unlink_request_from_queue`: Clears the record's flag and, depending on its status code, unlinks it from one of two request lists and adjusts the queued request count.
- `finish_request_record`: Unlinks the record from the active list when it is the list head, or unlinks it and decrements the queued count when its flag is set, then marks it complete and calls the completion routine when its bit is clear.
- `complete_queued_request`: When the record's queued flag is set, unlinks it from its list, decrements the queued count, marks it complete, and calls the completion routine unless its bit is set.
- `finish_active_request_record`: Completes a request record: when it is the active list head it clears the head and related flags, calling a routine for one status code; otherwise it unlinks a queued record and decrements the count.
- `unlink_flagged_entry`: Unlinks an entry from a linked list if a specific flag bit is set and decrements a list counter.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `finish_request_with_library_calls`: If the record's count field is nonzero it sets a flag and returns zero.
- `close_request_with_library_calls`: Calls a series of library routines to close a request, unlinks the record from its list, and returns the saved value.
- `queue_request_record`: Links a request record into the queued list when its size is nonzero, marking it queued, incrementing the count and calling a library routine.
- `take_next_queued_request`: Takes the next record from the pending list, or the head of the queue when the pending list is empty, unlinks it, clears its counters and returns it.
- `transfer_serial_bytes_to_buffer`: Loops while the outstanding byte count is nonzero, reading bytes from the transmit buffer with wrap-around, storing them on the receive side, and calling the status routine when its flag is set.
- `store_received_serial_byte`: Stores one received byte into the receive buffer, advances its pointer and count, and checks the byte against a table of trigger values, calling the status routine when a trigger is met.
- `service_pending_request_list`: Clears the flags of the record at the head of the list and calls the completion routine, then moves to the next queued record while the queued count is positive, returning the status value.
- `store_masked_serial_byte`: Clears one bit of a value chosen by a control byte, stores the result at the current buffer pointer, advances the buffer count, and calls the status routine when its threshold condition holds.
- `stub_empty_serial_routine`: Empty routine that returns at once, so it is a stub.
- `identify_routine`: This routine is not empty; it contains ten instructions but the decompiler has not produced meaningful pseudo-code.
- `select_serial_status_code`: Clears a control flag and sets the status byte of the request record to one of three codes, chosen from two flag bits.
- `reset_serial_status_counters`: Sets the status byte from the control flags, clears the record's counters, and clears the low control bits.
- `set_serial_status_and_reset`: Sets the status byte to one of two codes from the control flags, resets the record's counters, and returns to the caller's loop.
- `handle_serial_timeout_event`: Checks the serial status register and, when a timeout condition holds, sets the timeout flag, records the time from the request record, and calls the device routine.
- `store_incoming_serial_byte`: When the incoming byte matches the record's expected value it handles the wait flag.
- `process_serial_receive_flags`: Checks the receive flag bits and, when the receive path is active, sets the pending flag and the read pointer before storing the byte.
- `store_serial_byte_if_room`: Stores a received byte into the receive buffer when there is room and the ninth data bit is clear, advancing the buffer pointer and count and calling the status routine when the remaining space falls below a threshold.
- `route_serial_byte_by_flag`: Checks a flag and the ninth data bit, then either marks the receive as pending or stores the received byte in the buffer, following the same path as the byte store routines.
- `mark_serial_receive_pending`: Sets the receive-pending flag, records the read pointer if none is stored, and calls the device routine through the base table.
- `store_serial_byte_threshold_path`: Stores the byte in the receive buffer when there is room, advances the buffer pointer and count, and when the remaining space falls below the threshold takes the pending path.
- `notify_serial_receive_ready`: Records the read pointer when none is stored, makes a library call, then calls the device routine through the base table.
- `update_serial_line_status`: Copies a status byte from the record into the device state, clears its flag, updates a port output bit, and writes a fixed interrupt control value when the flag is clear.
- `set_serial_line_status_flag`: Copies a status byte into the device state, sets its flag bit, sets a port output bit when the control flag is present, and writes a fixed interrupt control value when the flag is clear.
- `clear_serial_status_byte`: Sets the status byte of the record to zero.
- `start_serial_timeout_timer`: When its timer flag is clear, sets the flag, resets the timer counter, loads the start time less ten and calls the device timer routine.
- `check_serial_timer_expiry`: Clears the timer flag and, when it was set and the status word has no pending bits, sets the receive-pending bit, records the read pointer, and calls the device routine.
- `dispatch_serial_status_change`: Reads a status word, masks it with the device mask, and compares the result against the record's status values, setting or clearing the matching status bits.
- `mark_serial_transmit_ready`: Sets the transmit-ready flag, records the read pointer if none is stored, and calls the device routine through the base table.
- `activate_serial_request`: Clears the record's count and, when the device is idle and no other request is active, makes the record the active request and starts it, calling the status routine when the check passes.
- `set_active_serial_request`: Marks the record as active and stores it as the current request, writing a fixed interrupt control value.
- `do_nothing_routine`: this routine is empty and performs no operations before returning.
- `set_serial_error_status`: When two status bits in the port register are both set, sets the record's status byte to a fixed error code.
- `report_serial_line_state`: Reports the device line state: when the record's pending byte is set it writes the combined mask and clears the byte, otherwise it checks the active flag and the queued request before updating the state.
- `check_serial_request_timeout`: Sets the request timeout to a fixed count, reads the head byte, and when the request's time count is exceeded clears its timeout flag and proceeds with the request.
- `process_serial_line_bits`: Compares the received count with the record's limit, masks the port bits against a stored mask, sets or clears the no-data flag, and calls the port check routine when the flag is set.
- `write_serial_data_byte`: Combines the status word with a bit from the device record, writes the result to the serial data register, advances the transmit pointer and count, and when the record's bit is set copies the next byte through the filter table.
- `dequeue_serial_request`: When the queued count is nonzero, takes the next record from the queue list, makes it the current request, decrements the count, and marks it as active.
- `set_serial_global_flag`: Sets a global variable to one, as a simple signal to the device.
- `send_pending_serial_byte`: Clears the record's pending byte and writes it, combined with a stored mask, to the device output.
- `reset_serial_device_state`: Disables interrupts, flushes the request lists through a helper routine, clears the device flags, and writes a set of default configuration values into the device record.
- `finish_serial_request_handler`: Calls a helper routine, then calls the device handler when the record's status bits are clear.
- `copy_serial_state`: The routine copies a 32-bit value from an offset within a structure pointed to by a register to two other offsets in the same structure, then clears a 32-bit field at a third offset.
- `flush_serial_request_list`: Marks the device as flushing when a request is queued, calls the helper for the active head request when one is present, and calls the device handler when the record's status bits are clear.
- `clear_flag_and_signal_device`: Clears a device flag and calls the helper for the active head request when present, then writes a fixed interrupt control value and calls the device handler when the record's status bits are clear.
- `disable_interrupts_during_helper`: Disables the selected interrupt bits, calls the helper routine, re-enables them, then calls the device handler when the record's status bits are clear.
- `clear_serial_flag`: The routine calls another routine twice and then clears a 32-bit field in a structure pointed to by an address register.
- `drain_serial_request_list`: Repeatedly unlinks the first record from the list it is given and calls the helper routine for it, until the list is empty.
- `do_nothing_5`: The routine is empty and performs no operations.
- `set_serial_flags_and_maybe_call`: The routine sets a flag byte in a structure pointed to by a1 and writes a constant to the next byte.
- `record_serial_status_snapshot`: Copies the port register bits, a status flag and the current counter into the request record, clears the record's status flags, and calls the handler when its status bits are clear.
- `queue_serial_request_if_busy`: Clears the record's flag and, when it is flagged and the device has pending requests, links the record into the waiting list, then calls the device handler.
- `mark_active_serial_request`: Calls a helper, then either records the request as the active one and writes a fixed interrupt control value, or calls the device handler and clears the active request.
- `clear_active_serial_request`: Calls the device handler and clears the active request pointer.
- `complete_active_serial_request`: Clears the record's flag and, when it was set and the active request has status ten, sets the completion flag, clears the active request, writes a fixed interrupt value and calls the device handler.
- `send_serial_command_byte`: Sets the device flag, stores the given value and a command code of nine in the device record, calls the helper routine, and sets the status to a fixed code when the helper reports a nonzero state.
- `update_serial_interrupt_flag`: Sets or clears the record's interrupt flag from the device mode bit.
- `finish_serial_transfer_flags`: Clears the transfer flag and, when the device is idle, the request list is empty and the record has no pending bit, masks interrupts and calls the device handler.
- `wrap_helper_with_interrupt_mask`: Writes a fixed interrupt control value, calls the helper routine, writes the enable form of the value, then calls the device handler when the record's status bits are clear.
- `validate_serial_timing_parameters`: Checks the record's character size, stop-bit and data fields against their valid ranges, and the rate value against a fixed range, then stores the derived setting and calls the helper routines to apply it.
- `compute_serial_mask_values`: Builds a bit mask from the record's bit fields and the device flag, stores it, then calls two helper routines to fetch the matching port values.
- `decrement_indexed_table_lookup`: The routine checks a flag byte in a structure pointed to by a register and conditionally decrements a short index.
- `set_serial_status_code_five`: Sets the record's status byte to the fixed code five.
- `compute_serial_divisor_setting`: Scales a stored rate value by seven, divides a fixed clock constant by it (shifting the scaled value when it is too large for the divider), stores the result as the divisor, and sets the transmit-ready flag when the device mode code is eight with the enable bit set.
- `set_serial_divisor_from_quotient`: Divides a 32-bit value by a 16-bit divisor argument, stores the quotient as the device divisor and writes it to the serial period register, marking the transfer flag when the mode code is eight with the enable bit set.
- `allocate_serial_unit_and_setup`: Allocates a unit structure through a library call and, if allocation and a device probe succeed, fills in buffer pointers, buffer sizes and two handler addresses, stores a status word and returns 1.
- `setup_serial_unit_buffers`: Stores buffer sizes, buffer pointers and two handler addresses into the unit structure, calling subroutines around the updates, then sets the global status word and returns 1.
- `fill_serial_request_block`: Writes a fixed byte and a pointer to a global variable into a request block, stores two saved register values in it, and links the block pointer into a second structure.
- `set_serial_device_io_request`: The routine writes a constant byte to an offset in a structure pointed to by a0, stores a pointer to a global variable at another offset, and copies two caller-provided values into two further offsets.
- `set_serial_device_request`: The routine fills a request block in memory with a pointer to a global variable and several byte fields, then calls a function through a pointer obtained from a table.
- `configure_serial_request_buffers`: Sets buffer pointers and sizes in the unit structure, calls request-setup subroutines and a library function, then sets the global status word and returns 1.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `stub_return_zero`: Does nothing except return zero, so it is a stub with no behaviour beyond its return value.

### gb

- `stub_bad_instruction_data`: Only a few instructions decode cleanly: they XOR a word and a byte into fields of structures held in registers.

### parallel.device

- `check_parallel_request_state`: Checks the busy and flag bits of the parallel port against the request's status, returns the error code 99 when they conflict, and otherwise updates the status bits, a counter and two saved fields before returning a success code.
- `update_parallel_status_flags`: Clears one status byte, copies a busy bit into a flag, increments a counter when the request is idle, and copies two saved fields into the request when its bit is set.
- `dispatch_parallel_io_request`: Dispatches a parallel request by its command field: it clears or sets pending flags for some commands, starts a transfer through a library call for others, marks the request, and passes it on to a follow-up routine through the device vector.
- `start_parallel_transfer_if_idle`: For command type three, clears a global bit and, unless a transfer is already pending, starts one through a device vector and marks the request pending.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `reset_parallel_request_and_dispatch`: Sets the request's status byte to a fixed value, then dispatches it by command type in the same way as the general dispatcher.
- `queue_or_start_parallel_request`: Clears the request's count field when it has none.
- `activate_parallel_request_record`: Clears a flag on the request, queues it on the device list when a transfer is already active, otherwise makes it the active request, records its count and buffer fields, and sets the global status byte to 0 or 255 according to the request type.
- `begin_parallel_request_transfer`: Marks a request as the active transfer, copies its buffer pointer and resets the transferred count, clears a busy bit, and sets the global status byte to 0 or 255 depending on the request type.
- `service_parallel_transfer_queue`: Advances the active parallel transfer by one byte.
- `advance_parallel_transfer_byte`: Moves one byte of a parallel transfer that is passed in a register between the global status byte and the request buffer, updates the transferred count, checks it against an eight-entry byte table and the length limit, and completes the request through a device vector when the stop condition is met.
- `complete_parallel_request`: Records the transferred count in the finished request, clears the active-transfer bits, calls a device vector to reply to the request and clears the active pointer.
- `read_parallel_byte_into_request`: Fetches one byte from a source pointer into the device status, counts it toward the transfer, checks it against an eight-entry byte table and the length limit, and completes the request through a device vector when the transfer ends.
- `acknowledge_parallel_status_update`: Calls a device vector, then clears the status byte, copies a busy bit into a flag, increments a counter when the request is idle, and copies two saved fields back into the request when its bit is set.
- `stub_return_immediately`: Returns without doing anything, so it is an empty stub routine.
- `finish_active_parallel_request`: When the active-request field is set, calls a device vector to finish that request, then returns zero.
- `finish_active_parallel_request_2`: When the active-request field is set, calls a device vector to finish that request, then returns zero.
- `drain_parallel_pending_queue`: Removes each request from the pending list in turn and calls a handler for it until the list is empty, then clears the pending count.
- `clear_parallel_pending_count`: Sets the device's pending-request count to zero.
- `set_parallel_request_and_notify`: Sets a flag bit on the request, writes a fixed status value into it, and calls a library function through the device's vector table.
- `exchange_parallel_control_bits`: Swaps three-bit control fields between the global control byte and the device control byte, stores the updated byte into the request, and calls two subroutines to apply it.
- `copy_parallel_request_state`: When the active-transfer bit is clear, copies the request's flag byte and, if its bit one is set, two saved fields into the device.
- `copy_parallel_flags_and_fields`: Copies the request's flag byte into the device state and, when its bit one is set, copies two saved fields across.
- `open_parallel_unit_and_register`: Allocates a parallel unit structure, claims the required system resources through library calls, initialises its pending-request list and handler fields, then calls a device vector and returns its result.
- `return_null_pointer`: The routine returns a null pointer.

### printer.device

- `allocate_printer_unit_structure`: Allocates a printer unit structure through a library call, stores its pointer in a global variable and records the caller's parameter in it, then calls a library function on it.
- `forward_to_printer_helper`: Passes control straight to one subroutine without doing any other work.
- `process_printer_io_request`: Marks the printer request busy, unlinks it from its queue when it is queued, stores the caller's status value, and calls device vectors to hand the request on.
- `execute_printer_initialization`: Calls a routine through a pointer, stores the result in a global variable, and updates a structure member with a value from a register.
- `initialise_printer_unit_resources`: Opens the libraries and timer device the printer needs, fills a 1024-entry table of words with a fixed pattern, sets up list pointers, and returns the unit on success.
- `release_printer_unit_resources`: Calls a release subroutine several times and then a cleanup vector, returning zero, to undo the setup done by the initialiser.
- `chain_three_printer_helpers`: Calls three subroutines in sequence and returns.
- `store_printer_status_value`: Stores a value from a register into a memory location pointed to by a pointer in a data structure.
- `set_printer_status_bit_zero`: Sets the lowest bit of a status byte held in the unit structure.
- `clear_printer_status_bit_zero`: Clears the lowest bit of a status byte held in the unit structure.
- `drain_printer_pending_requests`: Repeatedly calls a subroutine for as long as the printer's pending-request list still has an entry.
- `advance_request_list`: This routine is empty.
- `forward_to_printer_helper_2`: Calls one subroutine and returns, with no other work.
- `run_printer_signal_loop`: Initialises two request lists and the unit's signal bits, then waits in a loop for signals.
- `poll_printer_signal_events`: Waits for signal bits in a loop, clearing printer status bits for two signal sources and handing each queued request to its handler when the matching signal is set.
- `service_printer_signal_loop`: Runs the same signal loop as the routine before it, starting from the queued-request check: it hands queued requests to their handlers and clears status bits for two signal sources.
- `mask_and_store_byte`: Clears a bit in a register based on a control byte and writes the resulting byte to a buffer pointer.
- `queue_printer_io_request`: Marks the printer busy and places a new request on the first or second queue, depending on whether the busy bit was already set.
- `return_from_subroutine`: The routine executes a return or an empty branch/return sequence.
- `start_printer_request_record`: Sets the second busy bit of the printer status, stores two caller parameters into the request record, sets a request type of three, and calls two subroutines to hand it on.
- `wait_for_printer_completion`: Returns the request's status byte when it is set, or zero when the printer is not busy.
- `wait_for_printer_status_code`: Waits for printer signals and handles events in a loop until the request reports a status code or the printer is idle, then returns that code or zero.
- `handle_printer_signal_events`: Handles printer signal events in a loop.
- `dispatch_printer_signal_events`: Waits for printer signals and dispatches each event to its handler, clearing busy bits for two sources, and returns a status code when the request finishes.
- `wait_for_printer_signal_status`: Waits for printer signals in a loop, handling each event, and returns the request's status code when one is set, or zero when the printer is no longer busy.
- `initialise_printer_request_fields`: Sets a fixed request length of nine, copies a device setting into the request, clears a counter and a flag byte, and calls a library function on the request's port.
- `clear_printer_request_and_notify`: Clears a status byte in the request and calls a library function on its associated port.
- `send_printer_character_unless_bell`: Unless the character field holds the bell control code seven, calls a library function on the unit's port and then a device subroutine to output it.
- `send_printer_character`: Calls a library function on the unit's port and then a subroutine to output the stored character.
- `check_printer_request_status`: Reads the request's status byte.
- `wait_for_printer_idle_status`: Loops waiting for printer signals and handling their events until the printer is no longer busy, returning any status code a handler reports or zero when idle.
- `await_printer_events_until_idle`: Handles printer signal events one at a time and returns zero once the printer is no longer busy, or the status code a handler reports.
- `run_printer_event_wait_loop`: Repeatedly waits for printer signals and passes completion events to their handlers, returning a status code when one is reported or zero when the printer is idle.
- `wait_for_printer_event_status`: Waits for printer events and dispatches them in a loop, returning a status code when one is reported or zero once the printer is no longer busy.
- `start_printer_buffered_output`: Chooses one of two output buffers alternately unless a mode byte is set, stores the choice in the printer state and calls a library function to begin output.
- `start_printer_alternating_buffer`: Alternates between two output buffers, stores the chosen one in the printer state, and calls a library function to start output.
- `show_printer_error_requester`: When the request's flag bit is clear, copies its flag byte and, if bit one is set, two saved fields into the device.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `chain_two_printer_helpers`: Calls two routines in sequence and returns.
- `copy_printer_result_field`: Clears a result field in the request, runs a subroutine, and if that succeeds copies a stored value into the result.
- `chain_two_printer_helpers_2`: Calls two routines in sequence and returns.
- `insert_printer_request_to_list`: Inserts a new request into a linked list and sets a status bit before copying a null-terminated string into a lookup table.
- `start_printer_transfer_request`: Returns the error code of a preliminary check when it fails.
- `transfer_printer_text_chunks`: Copies the caller's text into a 256-byte buffer in pieces, taking its length from the request or by scanning for a terminator.
- `transfer_printer_text_in_registers`: Copies the caller's text into a 256-byte buffer in pieces and passes each piece to a subroutine, splitting at a special byte when enabled.
- `transfer_printer_text_until_done`: Repeatedly copies printer text into a buffer and passes each piece to a subroutine, splitting on a special byte when set, until the caller's length is used up or an error occurs.
- `buffer_printer_bytes_and_flush`: Copies printer bytes one at a time into a 256-entry buffer, flushes the buffer to a subroutine when it is full or the text ends, and handles the special-byte split before returning the status.
- `split_printer_text_on_marker`: Splits the text at the special byte: each run before the byte goes to a subroutine, the byte itself is forwarded as a marker, and the loop continues until the text is exhausted or an error occurs.
- `send_printer_text_runs`: Walks the text looking for the special byte, sending each run to the printer through subroutines and buffering characters into the 256-byte chunk.
- `copy_remaining_printer_bytes`: Copies the remaining buffered bytes into the printer buffer one at a time, then stores the caller's result value in the request, zeroing it on error.
- `clear_printer_result_status`: Sets a result field in the request to zero and returns the status byte it was passed.
- `translate_printer_character_codes`: Maps each input character through a thirteen-entry substitution table chosen by a mode value, with several special cases, and passes each resulting character to a subroutine while shifting a ten-entry buffer.
- `translate_printer_buffer_codes`: Applies the same thirteen-entry substitution to a counted run of input characters, writing each mapped character to the output and passing non-negative results to a subroutine.
- `translate_printer_text_table`: Runs the same thirteen-entry substitution over a counted run of characters, selecting each mapping from a fixed table of letters stored in the routine, and passes each mapped character on to a subroutine.
- `scan_printer_character_mapping`: Scans each input character against the thirteen-entry substitution table, writes the mapped characters to the output and hands each one to a subroutine.
- `map_printer_characters_inline`: Maps input characters through the thirteen-entry substitution table in an inline loop, writes the results and passes each one to a subroutine, returning when the counted run is finished.
- `apply_printer_character_mapping`: Applies the substitution table to each input character, handling the last mapping entry as a separate case, passes mapped characters to a subroutine, and returns once the counted run is complete.
- `map_printer_characters_pending`: Applies the substitution table to input characters, with a pending case handled after the last mapping entry.
- `substitute_printer_character_table`: Substitutes input characters from the table without the pending case, writes each result and passes it to a subroutine, and returns once the counted run is complete.
- `map_printer_characters_formfeed`: Maps input characters through the substitution table and emits a form-feed character for one particular code.
- `expand_printer_escape_sequences`: Walks the printer's input text, copying plain characters to the output.
- `expand_printer_escape_bytes`: Converts the printer's input bytes one at a time, expanding escape sequences, table strings and newline handling into the output buffer.
- `dispatch_printer_escape_code`: Looks up the byte after an escape in a small table.
- `expand_printer_escape_stream`: Processes printer input bytes, handling escape sequences that open with a bracket or parenthesis, expanding table strings and newline codes into the output buffer.
- `scan_printer_control_codes`: Scans printer input for a set of eight control codes and passes matches to a subroutine.
- `expand_printer_newline_text`: Starts by handling a newline code, emitting carriage return and line feed or line feed alone depending on a mode flag.
- `emit_printer_line_and_text`: Sets a line-break code, passes it to a subroutine and then copies the following printer text into the output buffer, expanding table strings and escape sequences until the request ends or the buffer is full.
- `emit_printer_crlf_and_text`: Writes a line feed and a carriage return into the output buffer and then copies the following printer text, expanding table strings and escape sequences until the request ends or the buffer is full.
- `emit_printer_linefeed_and_text`: Writes a line feed into the output buffer and then copies the following printer text, expanding table strings, escape sequences and newline codes until the request ends or the buffer is full.
- `expand_printer_sequence_entry`: Copies printer text into the output buffer, expanding table strings for codes from 160 upwards and passing escape-sequence results to a subroutine.
- `expand_printer_table_text`: Copies plain printer bytes into the output buffer and expands table strings for codes from 160 upwards, with escape sequences and newline codes handled as they appear.
- `continue_printer_table_string`: Resumes copying a printer table string from its current position, reading backslash-digit escapes as decimal values and counting the bytes written.
- `copy_printer_table_string_remainder`: Copies the remainder of a printer table string into the output buffer, reading backslash-digit escapes as decimal values, and then continues with the rest of the printer text.
- `copy_printer_table_string_entry`: Copies one printer table string entry into the output buffer, reading backslash-digit escapes as decimal values, and then continues with the rest of the printer text.
- `parse_printer_backslash_escape`: Reads a backslash-digit escape from the printer text, stores its decimal value as one byte of output, and continues copying the text until the request ends or the buffer is full.
- `parse_printer_decimal_byte`: Parses up to three decimal digits from the printer text into one byte, stores it in the output buffer, and then continues copying the remaining text.
- `store_printer_byte_and_continue`: Stores the current printer byte in the output buffer and then continues copying the text, expanding table strings and escape sequences until the request ends or the buffer is full.
- `store_printer_byte_and_dispatch`: Stores the current printer byte in the output buffer, then dispatches escape sequences and table strings in the rest of the text until the request ends or the buffer is full.
- `finish_printer_text_request`: Subtracts the bytes handled from the remaining count, clears the chunk counter, and returns the stored status word.
- `copy_printer_expansion_into_output`: Runs a subroutine that produces up to 256 bytes into a local buffer and copies them to the output when they fit.
- `copy_printer_buffer_to_output`: Copies a counted run of bytes from the local buffer into the output buffer when they fit.
- `mark_printer_text_for_resume`: Sets the resume position to the byte before the current one and returns one, so the text is processed again later.
- `copy_printer_local_bytes_unchecked`: Copies a counted run of bytes from the local buffer into the output buffer without a size check and returns zero.
- `copy_printer_buffer_run`: Copies bytes from a local buffer into the output buffer in a loop until the counted length is reached, then returns zero.
- `parse_printer_escape_parameters`: Reads the numeric parameters of an escape sequence: digits build each parameter, a semicolon starts the next one, and a space or quote sets a state value.
- `read_printer_escape_parameter_digits`: Accumulates decimal digits into the current escape parameter, handling separators and quote codes as the parameter parser does, and returns the byte that ends the sequence.
- `scan_printer_escape_arguments`: Scans the arguments of an escape sequence, accumulating digits and recording the separator state, and returns the byte that ends the argument list along with the argument count.
- `finish_printer_escape_parameter`: Accumulates the digits of an escape parameter until a quote or other terminator is reached, records a state value on a quote, and returns the byte that ends the parameter list.
- `accumulate_printer_escape_digit`: Adds a decimal digit into the current escape parameter, continues scanning the argument list, and returns the byte that ends it.
- `read_printer_buffer_byte`: Returns the next byte of a buffer and advances its index, or returns zero when the index passes the end of the buffer or the output is over its limit.
- `return_register_value`: Returns the value currently held in a data register.
- `scan_back_to_escape_byte`: Scans backwards through the buffer for an escape character, stops at the start of the buffer, then records the stop position and sets a found flag.
- `step_back_to_escape_byte`: Steps back one byte at a time through the buffer until it reaches an escape character or the start of the buffer, then sets a found flag.
- `reset_printer_state_and_prefix`: Clears the printer's state bytes and counters.
- `save_printer_setting_words`: Copies six printer setting words from the device configuration into a save area.
- `compare_printer_setting_words`: Compares six current printer setting words with the saved copy.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `format_printer_settings_block`: Copies four bytes of printer settings from the request into a local area, runs a formatting subroutine that expands them into a 256-byte buffer, and emits the result through a subroutine when it is produced.
- `send_printer_setup_sequence`: Sends a short setup sequence to the printer: a control value of one when a flag is clear, then a buffer taken from the device, then a value of three when the flag is clear.
- `emit_printer_command_sequence`: Looks up a printer command string in a table by its code, applying special cases for two codes and copying the string byte by byte.
- `copy_printer_command_string`: Copies the printer command string for a given code into the output buffer with the same special cases.
- `copy_printer_command_with_argument`: Copies a printer command string and, when the code falls in a numeric range, appends the argument byte.
- `copy_printer_command_tail`: Copies the rest of a printer command string, mapping stop markers to zero, and returns the number of bytes written.
- `copy_printer_command_bytes`: Copies printer command bytes up to the terminator and returns the byte count, or minus one if a stop marker is reached.
- `handle_printer_status_request`: For request codes zero and one, marks the request's flag byte as set when it is clear and then calls a device vector with fixed arguments.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d2 to d0, then copies (a7)+ to d2, then returns.
- `setup_printer_graphics_render`: Copies the graphics request's region, density and option fields into render state, works out the printable width and height from the source bitmap and the device limits, and returns an error code when the request exceeds the printable area.
- `begin_printer_graphics_output`: Marks the render as active, sends an opening control sequence through a device vector, and selects the output mode from the device's configuration bits before continuing the graphics output.
- `render_printer_graphics_rows`: Sets the graphics mode flag from the device configuration and the source settings, then processes the bitmap through subroutines while honouring the device's width and height limits.
- `check_printer_graphics_bounds`: Works out the width and height of the requested rectangle and returns error code four when it is smaller than the required area.
- `measure_printer_bitmap_source`: Derives the width from a source bitmap header, multiplied by eight, and the height from the same header.
- `clamp_printer_graphics_extents`: Obtains two extent values from subroutine calls and clamps each to a device limit before continuing the graphics output.
- `clamp_printer_output_dimensions`: Obtains a dimension from a subroutine call, clamps it to the device's first limit, and then clamps the second dimension to the device's second limit.
- `clamp_printer_graphics_dimensions_pair`: Obtains a dimension from a subroutine call, clamps it to the device's first limit, and then clamps the second dimension to the device's second limit before continuing the graphics output.
- `clamp_printer_row_width`: Obtains a width from a subroutine call and clamps it to the device's first limit before the row output continues.
- `render_printer_graphics_strip`: Obtains a width from a subroutine call, clamps it to the device's first limit, and then processes the graphics strip through further subroutine calls.
- `clamp_printer_graphics_height`: Obtains a height from a subroutine call, clamps it to the device's second limit, and then continues the graphics output through subroutines.
- `select_printer_graphics_pass_count`: Sets a pass count of six or twelve and a working count of seven or fourteen, chosen by a global flag, and then continues the graphics output.
- `prepare_printer_graphics_row_range`: Sets the row range for the graphics output from the request's offset and size fields and passes the rows on through subroutines.
- `prepare_printer_graphics_column_range`: Sets the column range from the request's offset and size fields, sets the mode flag, and loops over the bitmap data through subroutines.
- `clip_printer_graphics_rectangle`: Computes the right and bottom edges of the graphics rectangle from its offset and size and, when the source coordinates are negative, clamps them, calling subroutines before the rectangle is output.
- `scale_printer_graphics_coordinate`: When a flag bit is set, applies a 16-bit fixed-point scaling to one coordinate using a subroutine for the multiplication, and stores the result back before continuing.
- `scale_printer_graphics_extent`: When a second flag bit is set, applies the same 16-bit fixed-point scaling to the other extent and then continues the graphics output.
- `select_printer_graphics_minimum`: Computes several extents through subroutine calls and keeps the smaller of two candidate values before the graphics output continues.
- `clip_printer_graphics_width`: Clamps the requested width to the device's width limit, using the smaller of the two values as the output extent.
- `clip_printer_graphics_height`: Clamps the requested height to the device's height limit and records the clipped extent for the row output.
- `clear_printer_colour_table`: Sets the row extent values from the device's limits and clears a table of colour entries used by the graphics output.
- `read_printer_colour_palette`: Reads each colour register through the graphics library and splits its 12-bit value into four-bit red, green and blue components stored in a table.
- `send_printer_graphics_block`: Sets a fill pattern value and sends the graphics block through a device vector, retrying with an alternate vector when the first call returns zero.
- `finish_printer_graphics_block`: Sends a closing control vector for the graphics block and then, depending on the mode flags, runs one of the output subroutines, returning its status.
- `select_printer_graphics_writer`: Runs one of several output subroutines chosen by two mode flags and stores the status code it returns.
- `flush_printer_graphics_rows`: Runs the output subroutine selected by a mode flag and, at intervals set by a device value, sends a control vector and reports the status.
- `emit_printer_graphics_row_group`: Runs one of two output subroutines depending on a mode flag, then sends a control vector at each row interval set by a device value and passes its status on.
- `emit_printer_graphics_row_status`: Runs the output subroutine, then sends a control vector at the device's row interval and passes the resulting status on.
- `send_printer_graphics_row_marker`: Sends a control vector when the row count is a multiple of the device's row interval, and passes the status on.
- `forward_with_zero_arguments`: Calls a routine with two zero arguments and returns.
- `convert_printer_colour_to_level`: Converts a colour's three channels to one value.
- `reduce_printer_colour_to_minimum`: When a device bit is set, reduces a three-channel colour to its smallest channel.
- `compute_printer_pixel_position`: Works out the pixel coordinates from row and column offsets, reads the pixel through a subroutine, and returns a sentinel value when the position lies outside the bitmap.
- `read_printer_graphics_pixel`: Reads one pixel through the graphics library at the computed position and stores its colour in the output, using a fallback colour when the position lies outside the bitmap.
- `store_printer_colour_index`: Stores the output colour looked up from a palette table, or a packed colour value when the table is not in use.
- `pack_printer_pixel_plane_bits`: Stores a colour value and, depending on a device setting, inverts it.
- `emit_printer_pixel_nibble`: Selects the pixel's colour bits from a packed value, passes them to an output subroutine, and stores the result in the pixel accumulator.
- `accumulate_printer_bit_totals_conditional`: Stores the colour bit under a mode condition, passes the value to an output subroutine, and adds the bit to four running totals.
- `accumulate_printer_bit_totals_mode3`: Stores the colour bit when the mode is three, passes the value to an output subroutine, and adds the bit to four running totals.
- `accumulate_printer_bit_totals`: Stores the colour bit, passes it to an output subroutine, and adds it to four running totals.
- `accumulate_printer_bit_totals_2`: Stores the colour bit, passes it to an output subroutine, and adds it to four running totals.
- `accumulate_printer_bit_totals_3`: Stores the colour bit, passes it to an output subroutine, and adds it to four running totals.
- `store_printer_palette_totals`: Stores a palette entry from the table into the output and adds the current colour to four running totals.
- `initialise_printer_plane_totals`: Works out a plane size as the product of two request dimensions, falling back to their sum when the product is zero, and sets four running totals through subroutine calls.
- `send_printer_colour_planes`: Sends the four colour planes to the printer in turn, each through a subroutine and a device vector with its plane index.
- `send_printer_plane_sequence`: Sends the colour planes one after another through a device vector, storing the status of each step and stopping early when one fails.
- `send_printer_upper_planes`: Sends the remaining colour planes through a device vector, checking each step's status and stopping early when one fails.
- `send_printer_final_plane`: Sends the last colour plane through a subroutine and a device vector and returns its status, or zero when the plane is not sent.
- `return_zero_2`: The routine returns a zero-valued integer.
- `advance_printer_row_counter`: Increments the row counter and, when the count reaches the device's row interval, sends two control vectors and returns their status.
- `send_printer_control_vector`: Sends a fixed control vector and returns its status, or zero when that status is zero.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `render_printer_bitmap_rows`: Walks the bitmap row by row, calling subroutines to obtain each row's data.
- `render_printer_bitmap_block_rows`: Loops over the bitmap rows.
- `process_printer_bitmap_blocks`: Obtains a block height from a subroutine, processes the block's pixels in nested loops and advances the block counters, returning early when a subroutine reports an error.
- `advance_printer_block_columns`: Processes the pixels of each column block in turn, advances the column counter and the block offset, and returns the status when a subroutine fails.
- `advance_printer_block_rows`: Advances the row and column counters across the block grid, processing each block, and returns the status when a subroutine fails.
- `advance_printer_bitmap_offset`: Advances the bitmap offset by the block height, obtains a new row width through a subroutine at each block boundary, and returns zero when the bitmap ends.
- `advance_printer_row_offset`: Advances the current row offset by the row step and returns zero when it reaches the end.
- `render_printer_bitmap_strips`: Walks the bitmap in strips.
- `render_printer_strip_blocks`: Obtains a row width from a subroutine, resets the block counters, and processes each block in the row through nested loops, reporting any error status.
- `process_printer_strip_columns`: Resets the block counters and processes each column of the current strip, reading the block count from a subroutine, and stops with the status when an error is reported.
- `advance_printer_strip_column_index`: Advances the column index through the strip, processes the blocks of each column, and returns the status when a subroutine reports an error.
- `advance_printer_strip_block_row`: Advances the row position through the strip by one block, processes the blocks of the next row, and returns the status when a subroutine fails.
- `advance_printer_strip_offset`: Steps the strip counter down, moves the block offset by the row step, and processes blocks until the strip ends, returning the status on error.
- `advance_printer_row_limit`: Adds the row step to the offset and returns zero when it reaches the bitmap height.
- `render_printer_bitmap_strip_loop`: Steps through the bitmap a strip at a time while the offset is inside its height, processing each strip's blocks through nested loops and returning zero when the bitmap ends.
- `render_printer_bitmap_while_rows`: Obtains a row count from a subroutine and processes the blocks of each row while the count is non-zero, then obtains the next count.
- `render_printer_block_rows_loop`: Processes blocks row by row through nested loops, passing each block through a subroutine and advancing the row offset until the bitmap is complete.
- `update_pointer_and_counter`: Updates a global counter, modifies a pointer-based value through multiplication and addition, and enters a loop that checks a pointer target.
- `update_printer_logic`: Updates a global counter and modifies a pointer-based value or a global variable through a multiplication and addition operation.
- `update_printer_row_counter`: Increments a global counter and checks if a target or a pointer-based value has reached a certain threshold or limit.
- `decrement_row_counter`: Decrements a pointer's target and returns a value or continues a loop based on whether the target reaches zero.
- `reset_printer_status_counters`: Clears two or more status-related or counter-related global or pointer-based variables to zero before beginning a processing loop.
- `process_printer_buffer_loop`: Iteratively checks a pointer-based value and updates a target, potentially returning a status or zero.
- `process_loop_with_pointer`: Executes a loop that increments a global counter and updates a pointer-based address, while also performing conditional checks on a return value.
- `compute_pixel_position_loop`: Calculates a pixel coordinate and executes a loop that decrements a counter while checking a pointer's target against a value.
- `send_printer_color_planes`: Executes a loop that updates a counter and checks a pointer-based value, potentially returning a status or or error code.
- `process_printer_loop`: Increments a global counter, decrements a global value, and executes a loop that updates a pointer or checks a counter.
- `process_printer_buffer`: Iteratively decrements a pointer-based value and checks a global counter against a pointer-based address.
- `accumulate_scaled_values`: Iteratively adds a doubled value to a pointer's target until the target is no longer less than one, then subtracts a doubled second parameter from that target.
- `compare_plane_values`: Compares a provided byte against a value retrieved from a table or a calculated offset, returning a boolean result.
- `compare_table_value`: Performs a comparison between a byte retrieved from a table using bitwise-derived indices and a provided byte.
- `negate_boolean_results`: Calls two different routines and returns the negation of the first return value based on the status of a second return value.
- `rotate_and_mod_value`: Performs bitwise rotations and modulo-like operations on a 32-bit value, potentially updating a register or a return value.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `recursive_call`: The routine calls itself and then returns.
- `call_and_return_d1`: The routine calls another routine and returns the value found in the D1 register.
- `negate_on_flag`: The routine calls another routine, checks a second return value, and returns the negated first return value if the flag is non-zero.
- `negate_on_flag_extended`: Calls another routine and returns its first return value, or the negation of that value if the second return value is non-zero.
- `call_read_pixel`: Library stub: loads its arguments and the graphics library base, calls ReadPixel and returns its result to the caller.
- `call_get_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls GetRGB4 and returns its result to the caller.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `match_and_update_string_buffer`: compares a string from a table or pointer against a global buffer and, if a match is found, updates a global pointer and copies the matching string into the global buffer.
- `compare_and_copy_string_2`: Compares a string against a global buffer and, if a match is found, updates a pointer and copies the string to a global buffer.
- `return_error_status`: An empty routine that returns a constant negative value.
- `compare_and_copy_string`: Compares two strings, and if a match is found, certain memory or library-based resources are updated and a string is copied to a global buffer.
- `string_compare_and_copy`: Compares a string against a global buffer, potentially releases resources, and copies a string to a global buffer if certain conditions are met.
- `cleanup_printer_string`: Iterates through a string, compares characters, and if a certain condition is met, releases resources and copies a string to a global buffer.
- `cleanup_printer_resources`: Checks a pointer, calls library routines to release or lock resources, and copies a string or buffer using a loop.
- `call_dos_functions`: Calls two different library routines and returns a value of negative one.
- `execute_callback_and_recurse`: Checks a global value, calls a function pointer from a table, and then calls itself with a value from a pointer.
- `compare_case_insensitive_chars`: Checks if two characters are equal after optionally converting them to a common case range.
- `return_null_pointer_2`: The routine returns a null pointer or zero-value integer.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg_2`: Calls a function pointer located at an offset from a base pointer, specifically targeting a library routine.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `return_zero`: The routine is empty and returns a zero value to the caller.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 255 into d0, then returns.

### arp.library

- `tiny_moveq_rts_4`: Small routine of 2 instructions that loads 34 into d0, then returns.
- `populate_block_list`: Iteratively calls a function to populate a list or table with return values while decrementing a counter.
- `allocate_library_block`: Library code that allocates a memory block, writes a link word, and manages a list of blocks while potentially calling a callback.
- `update_library_state`: Library code that updates a field in a structure, calls other library functions, and potentially sets a status flag or a pointer.
- `poll_until_flag_then_count`: Library code from arp.library that repeats a library call until its condition flag clears, then increments a 16-bit counter in a structure.
- `clear_slot_then_call_library`: Library code from arp.library.
- `allocate_block_with_link_word`: Library code from arp.library that, when given a non-zero size, allocates a memory block and writes a link word into its first slot pointing just past that slot.

### Spy vs Spy - The Island Caper (1989)(Wicked Software)[cr VF][a].adf:vf.obj

- `run_entry_with_interrupt_toggle`: Disables interrupts by modifying the hardware register, calls a routine, modifies a global to point to a self-referencing table, and calls another routine before re-enabling interrupts.
- `transform_bit_stream_to_memory`: Iteratively processes bits from a global variable and uses a secondary counter-driven loop to reconstruct and store a byte sequence into a memory buffer.
- `xor_and_shift_register`: Xors a global variable with a value from a pointer and returns a right-shifted version of that value with a bit set.
- `shift_and_decrement_counter`: Repeatedly shifts a register right and decrements a counter until the register or counter reaches zero.

### Spy vs Spy - The Island Caper (1989)(Wicked Software)[cr VF][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialise_runtime_state_and_chips`: Opens an exec library, writes the interrupt, DMA and copper pointer registers, waits for the blitter to go idle, clears a block of memory word by word and then calls several subroutines to finish setup.
- `advance_countdown_and_pop_queue`: Decrements a byte-sized countdown timer and, if it reaches zero, resets it to four.
- `initialise_eight_entry_table`: Fills an eight-entry table with pointers that step by a fixed stride and matching word offsets that step by eighteen.
- `fill_mirrored_word_pairs`: Reads a word from a pointer-chained table, writes it and its complement offset by a fixed amount into four paired slots, and advances the pointer, wrapping to the start at a sentinel value.
- `run_twenty_calls_eight_blocks`: Makes twenty calls to a subroutine for each of eight blocks, advancing a pointer by twenty after every block and resetting it when the stored count is unable to reach a certain value.
- `search_table_and_setup_blit`: Finds the index of a value in a byte table, waits for the blitter to go idle and then programs its registers with mask words, a source address derived from the index and a fixed size.
- `setup_blitter_scroll_step`: Steps a scroll offset down by four, wrapping it to a fixed value and toggling a bit pattern when it goes negative, then waits for the blitter and programs its size and pointers for the copy.
- `run_blitter_copy_loop`: Waits for the blitter to go idle, programs its control and size registers with a constant, then runs eight copy passes that transfer source words to the destination.
- `do_nothing_routine_2`: the routine contains ten instructions but performs no state-altering or computational operations before returning.
