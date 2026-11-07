# Game 2431 (hero's quest (1990)(sierra)(disk 1 of 5)[cr neutron][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02431). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (913536 top-level bytes in 28 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 19 of 19 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 261 of 261 named (0 author symbols), 261 of 261 described (44 by rule, 78 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1576 of 1576 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5195 of 5195 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 13527 |
| code | 17674 |
| data | 774654 |
| padding | 6305 |
| unused | 101376 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| RunBack | 3 | 709 | 39 |
| CloseWB | 2 | 12 | 1 |
| ENDCLI | 2 | 117 | 2 |
| ECHO | 2 | 117 | 2 |
| Execute | 2 | 117 | 2 |
| ADD21K | 3 | 480 | 33 |
| Wait | 2 | 117 | 2 |
| Disk-Validator | 1 | 386 | 1 |
| Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron][a].adf:RESOURCE.DAT | 3 | 942 | 55 |
| Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron][a].adf:PROG | 66 | 29 | 2 |
| Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron][a].adf:HERO | 2 | 119 | 3 |
| Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron][a].adf:AMIGASND.DRV | 3 | 1030 | 74 |
| Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron][a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 487 | 27 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 384 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 2 |

## Cases handled

- hero's quest (1990)(sierra)(disk 1 of 5)[cr neutron][a]/Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron][a].adf:RESOURCE.000: high-entropy data file (7.70 bits/byte), classified as packed data
- hero's quest (1990)(sierra)(disk 1 of 5)[cr neutron][a]/Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron][a].adf:RESOURCE.001: high-entropy data file (7.83 bits/byte), classified as packed data
- runtime segment an address (play-rs-x25k): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 315 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 260 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (40 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- hero's quest (1990)(sierra)(disk 1 of 5)[cr neutron][a]/Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron][a].adf:PROG: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- hero's quest (1990)(sierra)(disk 1 of 5)[cr neutron][a]/Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron][a].adf:HERO: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_and_call`: Copies sixteen longs from a fixed offset in a block to a destination pointer when that pointer is non-null, then calls through a stored code pointer.

### RunBack

- `start_program_entry_call`: The program entry point, which makes a single call to one other routine.
- `run_background_task`: Parses a command line for an optional load delay of 0-9 seconds and a program name with its parameters, then starts that program as a background task.
- `clear_table_and_store_pointers`: Clears a block of forty longs inside a table entry, then stores a stack pointer and a global value into the two fields after the block before calling an unnamed routine.
- `call_hook_then_continue`: Calls a stored hook routine when its pointer is non-null, then calls a second routine and returns.
- `parse_command_line_and_run`: Opens the DOS and math libraries through library lookups, splits the command line into a null-terminated argument vector, and sets up the program's startup values before running it.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `run_loop_then_dispatch_globals`: Calls an unnamed routine ten times, then calls each stored hook and related routine whose global field is set.
- `append_string_limited`: Appends the source string to the destination string, copying at most a fixed count of bytes and always writing a terminator.
- `append_string_with_limit`: Appends bytes from the source string to the end of the destination, stopping at the terminator or after a count taken from the third argument, and always writes a terminator.
- `copy_null_terminated_string`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks a zero-terminated byte string to its terminator and returns the number of bytes before it.
- `copy_bytes_with_padding`: Copies bytes from a source pointer to a destination for a limited count, stopping early at a zero byte, then writes zero bytes to fill the rest of the count.
- `release_numbered_library_slot`: Checks a slot index from zero to nineteen and returns minus one with an error code when it is out of range or empty.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_routine_102`: Calls the library routine at a fixed offset from a library base held in a global variable.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_function`: Calls a function from a library using a pointer retrieved from a table.
- `call_library_function_2`: Executes a library function via an offset from a base address stored in a table.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_function_3`: Performs a call to a library routine using an offset from a pointer in a table.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_function_4`: Invokes a library function by accessing a pointer through a table and an offset.
- `call_library_function_5`: Calls a routine from a library using an offset from a pointer located in a table.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_second_library_routine_198`: Calls the library routine at a fixed offset from a second library base held in a global variable.
- `call_library_function_6`: Executes a library function using an offset from a pointer retrieved from a table.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_function_7`: Calls a library function via an offset from a base address found in a table.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### CloseWB

- `close_workbench_at_entry`: Program entry point that opens the intuition library, loops on a library call until it returns non-zero, then closes the Workbench and the library.

### ENDCLI

- `build_max_value_table_header`: Walks a linked list of entries to find the largest stored value, then writes a table header and entries into a buffer through a stored routine pointer.
- `copy_memory_block_and_call`: Copies a fixed number of words from a memory location to a destination pointed to by a stack pointer, then executes a function pointer stored in a register.

### ECHO

- `update_linked_list_and_copy_data`: Traverses a linked list to find the maximum value in a specific field, then updates a header and populates a buffer with a sequence of incrementing values.
- `copy_memory_block`: Copies sixteen bytes from a memory location offset from a pointer to a destination address on the stack.

### Execute

- `update_linked_list_and_copy_data_2`: Traverses a linked list to find the maximum value in a specific field, then updates a data structure with this value and populates a buffer using a loop.
- `copy_sixteen_words`: Copies sixteen words from a source pointer to a destination pointer.

### ADD21K

- `open_library_and_setup`: Program entry point that opens a library through a global structure and calls an error routine when the open fails.
- `set_viewport_and_refresh`: Opens the graphics and intuition libraries through a global structure and calls an error routine when an open fails.
- `clear_longs_and_store_pointers`: Clears forty-two longs inside a table entry, stores a stack pointer and a global value into the two fields after the block, then calls an unnamed routine.
- `call_indirect_if_flag_then_recurse`: The routine checks a global variable for non-zero and, if set, calls the function pointer stored there.
- `parse_dialogue_text`: Obtains two identifiers from a global table and stores them in global variables, then branches based on their values.
- `execute_callback_chain`: Executes a series of conditional function calls and a loop that runs a routine ten times.
- `initialize_game_state`: The routine runs a short loop ten times calling another routine.
- `copy_string_with_limit_2`: This routine finds the end of a destination string and copies bytes from a source pointer into that destination until a null byte is found or a maximum length is reached.
- `copy_string_with_limit`: Finds the end of a destination string, then copies bytes from a source pointer into that destination until a null byte is found or a counter reaches zero.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length_2`: Walks forward byte by byte through a zero-terminated string and returns the number of bytes from the start up to but not including the terminator.
- `copy_bytes_with_padding_2`: Copies bytes from a source pointer to a destination pointer for a specified count, stopping if a zero byte is found.
- `execute_table_entry`: This routine uses an index to locate an entry in a table, checks if the index is within valid bounds, and then calls a routine via a pointer from that entry if a specific bit is set.
- `call_offset_routine`: Calls a routine located at an offset from a pointer stored in a global structure.
- `call_offset_routine_2`: Calls a routine located at a different offset from a pointer stored in a global structure.
- `execute_indirect_function`: Performs a double-dereference to call a function pointer stored within a structure located via an offset from a base address in a register.
- `execute_indirect_function_2`: Performs a double-dereference through a pointer stored at an offset from a base address to call another routine.
- `call_indirect_via_table`: The routine loads a function pointer from a global variable table and calls another routine through it.
- `execute_indirect_function_3`: Retrieves a function pointer from a nested table structure using offsets from a base address and executes it.
- `call_indirect_via_offset`: Loads a pointer from a table using an offset and calls the routine located at that address.
- `execute_indirect_function_4_2`: This routine performs a double-dereference to call a function pointer stored within a nested table structure.
- `execute_indirect_function_4`: Performs a double-dereference to call a function pointer stored within a nested table structure.
- `execute_indirect_function_5`: Performs a double indirection to call a function.
- `execute_indirect_function_6`: Performs a double-dereference to call a function pointer stored within a nested table structure.
- `call_indirect_function_via_offset`: Fetches a function pointer from a nested table structure using offsets from a base address and executes it.
- `execute_indirect_function_7`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `execute_indirect_function_pointer`: Performs a double-dereference to call a function located via an offset from a pointer stored in a memory location.
- `execute_indirect_function_8_2`: This routine fetches a function pointer from a nested table structure using an offset from a base address and executes it.
- `execute_indirect_function_8`: Fetches a function pointer from a nested table structure using an offset from a base address and executes it.
- `call_library_slot_378`: Loads a function pointer from a table reached through the saved base register and jumps to it, forwarding the call to an Amiga library routine at vector offset minus 378.
- `call_library_slot_384`: Loads a function pointer from the base-register table and jumps to it, forwarding the call to an Amiga library routine at vector offset minus 384.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_alternate_base_library_slot`: Loads a function pointer from a second base-register table and jumps to it, forwarding the call to an Amiga library routine at vector offset minus 384.

### Wait

- `scan_entries_and_fill_table`: Program entry that walks a linked list of entries, keeping the largest value of one field, and then requests a block through the library sized from that value.
- `copy_sixteen_words_to_buffer`: Copies sixteen 32-bit words from a fixed offset in the base-register block into the buffer whose pointer is passed on the stack, skipping the copy when that pointer is null.

### Disk-Validator

- `compare_bitmap_pairs_for_mismatches`: Program entry that walks a range of block numbers and tests the matching bit in two bitmap word arrays, counting the blocks where the bits disagree.

### Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron][a].adf:RESOURCE.DAT

- `zero_twenty_global_words`: Calls another routine, then clears twenty consecutive 32-bit words in the global data area with a countdown loop.
- `initialize_object_state`: Stores a parameter value into one field of the object passed in, sets a word field to the constant forty, and points another field at a global variable.
- `delay_and_configure_game_object`: This routine contains a short busy wait and then queries a global structure, storing several fields into a stack frame.
- `allocate_tagged_resource_entry`: Allocates a resource entry sized from a count in the global data and, if that succeeds, sets its state fields and writes a four-character tag into its header.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `match_name_and_build_list`: Compares a stored name string against a second name and, on a match, walks a linked list of nodes, allocating and filling one record per node with pointers and values.
- `stub_bad_instruction_data`: The decompiled body is only a return, so the listed instructions could not be recovered.
- `copy_block_handling_overlap`: Copies the given number of bytes from a source pointer to a destination pointer, returning at once when both are equal.
- `copy_bytes_forward_or_reverse`: Copies a counted block of bytes between two pointers, returning at once when they are the same address.
- `stub_empty_return_routine`: The body is only a return with no work, so the routine is a stub.
- `tokenize_command_line_into_argv`: Builds an argument vector from a command string by copying a fixed string and the argument text into a buffer, then splitting the text on whitespace while keeping double-quoted sections together.
- `append_string_to_buffer`: Appends a source string to the end of a destination string, stopping after 32766 characters at most.
- `append_string_with_limit_2`: Appends a source string to the end of a destination string, copying no more characters than the count argument allows.
- `copy_string_pad_zeros`: Copies characters from a source string into a destination buffer, stopping at the source null terminator or when the count is used up.
- `fetch_icon_library_entries`: Opens the icon library, then looks up a window-related entry and calls two library vectors to fetch values, which are stored into the structure passed in.
- `call_optional_hook_and_finish`: If a global function pointer is set, calls the routine it holds, then calls a second routine and returns.
- `invoke_optional_hook_then_return`: Has the same body as the neighbouring hook caller: it calls the global function pointer when it is set, then calls a second routine and returns.
- `release_resource_table_entries`: Processes each resource entry through routine calls and then frees the entry table by its size.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `free_indexed_resource_entry`: Looks up a resource entry by index and fails with minus one, storing error code two in the global data, when the index is out of range or the entry is empty.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `switch_current_directory`: Calls the dos library directory-change routine through the library base kept in global data.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_exit`: Wrapper that calls the dos library function Exit and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_load_seg`: Wrapper that calls the dos library function LoadSeg and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_slot_108`: Calls the library routine at vector offset minus 108 through the library base stored in global data, passing no arguments.
- `call_library_slot_414`: Loads a library base from the global table and jumps to the vector at offset minus 414, forwarding whatever arguments are already in the registers.
- `call_library_slot_414_2`: Loads a library base from the global table and jumps to the vector at offset minus 414, forwarding the register arguments unchanged.
- `call_library_slot_198`: Loads a library base from the global table and jumps to the vector at offset minus 198, forwarding the register arguments unchanged.
- `call_library_slot_198_2`: Loads a library base from the global table and jumps to the vector at offset minus 198, forwarding the register arguments unchanged.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_slot_294`: Loads a library base from the global table and jumps to the vector at offset minus 294, forwarding the register arguments unchanged.
- `call_library_slot_294_2`: Loads a library base from the global table and jumps to the vector at offset minus 294, forwarding the register arguments unchanged.
- `call_library_slot_132`: Loads a library base from the global table and jumps to the vector at offset minus 132, forwarding the register arguments unchanged.
- `call_library_slot_210`: Loads a library base from the global table and jumps to the vector at offset minus 210, forwarding the register arguments unchanged.
- `call_library_slot_372`: Loads a library base from the global table and jumps to the vector at offset minus 372, forwarding the register arguments unchanged.
- `call_library_slot_372_2`: Loads a library base from the global table and jumps to the vector at offset minus 372, forwarding the register arguments unchanged.
- `call_library_slot_552`: Loads a library base from the global table and jumps to the vector at offset minus 552, forwarding the register arguments unchanged.
- `call_library_slot_552_2`: Loads a library base from the global table and jumps to the vector at offset minus 552, forwarding the register arguments unchanged.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_slot_378_2`: Loads a library base from the global table and jumps to the vector at offset minus 378, forwarding the register arguments unchanged.
- `call_library_slot_384_2`: Loads a library base from the global table and jumps to the vector at offset minus 384, forwarding the register arguments unchanged.
- `call_library_slot_384_3`: Loads a library base from the global table and jumps to the vector at offset minus 384, forwarding the register arguments unchanged.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_slot_78`: Loads a library base from a second global table and jumps to the vector at offset minus 78, forwarding the register arguments unchanged.
- `call_library_slot_198_3`: Loads a library base from a second global table and jumps to the vector at offset minus 198, forwarding the register arguments unchanged.

### Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron][a].adf:PROG

- `release_three_memory_blocks`: Calls another routine, then calls the system memory-free vector three times through a library base kept in global data.
- `decrement_until_minus_one`: Decrements a 16-bit counter held in a register until it reaches minus one, then returns.

### Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron][a].adf:HERO

- `decrunch_packed_data_backward`: Decodes packed data by reading bits from a stream that uses a sentinel bit, copying runs of earlier output backward into the destination.
- `fetch_next_bit_word`: Reads the longword just before the stream pointer, shifts it right by one bit and sets its top bit as a sentinel, then returns the result to the bit reader.
- `read_bit_count_from_stream`: Shifts bits out of the current stream word, loading the previous longword with a sentinel bit whenever the word runs out, and repeats for the requested number of bits.

### Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron][a].adf:AMIGASND.DRV

- `dispatch_callback_with_guard`: Sets a busy flag in global data and, if it was previously clear, stores a short value from a register and calls a routine chosen through an indirect pointer table indexed by that value.
- `parse_sound_sample_header`: Reads the header of a sound stream: a zero first byte fills a four-entry channel table from a bit mask, while a two-value first byte finds the end marker, computes the sample length and picks a playback period from the rate code (163 for 22000, 325 for 11000, otherwise 651).
- `clear_byte_and_set_status`: Clears the byte at the pointer passed in, stores the status value three in the structure, and returns that status.
- `find_sample_end_and_copy`: Scans the sound data for the terminator marker, works out the sample length and playback period from the rate code, and copies the sample bytes into the channel buffer when that buffer pointer is set.
- `decode_sound_command_block`: Reads a stream of bytes through a pointer, using a mask from global data to select up to four channel numbers.
- `check_bell_and_write_byte`: Checks if the first byte of a pointer is a bell character and if a byte at an offset from another pointer is zero.
- `store_byte_conditional`: Checks if a value at an offset from a pointer is not equal to four.
- `record_stream_end_or_byte`: If the byte is the 127 end marker, stores the distance from the stream start as a short in one field.
- `store_channel_value_word`: Writes the word passed in the data register into a fixed field of the channel structure, the same field that the byte-recording routine fills.
- `advance_sound_sequence_stream`: Advances a sound sequence by one step, decoding MIDI-style events such as note on, note off, control change and program change, and handling variable delay counts.
- `stop_channel_and_mark_idle`: Clears a global flag, sets a channel field to minus one and sets the global channel state to two, then returns zero.
- `advance_channel_state_machine`: Advances a small channel state machine: state two moves to three, and state three clears a flag and moves to four.
- `step_channel_timer_or_finish`: If the state is three, clears a flag, calls two other routines and moves the state to four.
- `tick_channel_timer`: Counts down a channel timer while its flag is set.
- `clear_sound_state`: Calls another routine, then if a global flag is clear, sets a channel field to minus one and the channel state to two.
- `decode_channel_sequence_events`: Decodes MIDI-style events from a channel sequence stream, dispatching note on, note off, control change and program change, and handling variable delay counts.
- `decode_sequence_until_wait`: Decodes MIDI-style sequence events from the stream, dispatching note, control and program events, until a pending delay or a stop event ends the pass.
- `save_sequence_position`: Decrements the channel delay counter, saves the current event bytes and the stream position into the channel structure, and returns.
- `decode_sequence_with_timer`: Decodes MIDI-style sequence events until a delay is pending, then counts that delay down or clears it depending on a flag.
- `decode_sequence_until_end`: Decodes MIDI-style events from the channel stream in a loop, dispatching note, control and program events and handling delay counts and the end-of-sequence marker.
- `decode_sequence_delay_and_save`: Decodes MIDI-style sequence events and adds the variable delay values that follow them to the channel delay counter.
- `accumulate_delay_and_save`: Reads sequence bytes, adding each small delay value to the channel delay counter and dispatching MIDI-style events.
- `skip_sequence_system_payloads`: Skips the payload of system messages in the family of status values from 240 upward, reading up to the terminator byte of 247 or stepping over a fixed number of bytes.
- `decode_sequence_with_system_skip`: Another variant of the sequence event decoder that skips system-message payloads and dispatches note, control and program events, adding delays to the channel delay counter.
- `decode_sequence_until_pending_delay`: Decodes sequence events while the delay counter is zero, skipping system-message payloads.
- `decode_sequence_note_control_events`: Decodes the note, control and program events of a sequence stream, adding delay bytes to the channel delay counter, and saves the stream position at the end of the pass.
- `parse_sequence_event_stream`: Iterates through a byte stream to decode musical sequence events, updating channel timers, note values, and envelope parameters.
- `decode_sequence_calling_library`: Decodes sequence events for a channel, with one routine call on its event path that resolves to a library call at vector offset minus 498.
- `decode_sequence_shared_entry`: Decodes sequence events for a channel and serves as a shared decoder entry, since six unnamed routines call it.
- `forward_to_helper_routine`: Makes a single call to another routine and returns.
- `store_bytes_and_adjust_offset`: Stores two byte values into the event fields of the structure, then sets the stream position field to the offset passed in relative to the stream start.
- `reset_channel_playback_state`: Sets a channel field to minus one, copies a saved position into the stream position field, and calls another routine.
- `clamp_level_and_scale`: Limits a 16-bit level field in the channel structure to 64, and when the channel flag is clear copies the level multiplied by four into a global field.
- `restore_saved_word_and_call`: Saves a word field from the channel structure, clears it, calls another routine four times, and then writes the saved value back.
- `call_helper_and_set_pending`: Calls another routine and then sets a pending flag byte in the channel structure.
- `arm_channel_timer`: Sets a channel flag byte to one and its timer count to eight.
- `configure_audio_channel_zero`: Disables interrupts, programs Amiga audio channel zero with the sample pointer, length, period and a volume scaled by the global level, then enables that channel's DMA and interrupt and re-enables interrupts.
- `configure_audio_channel_one`: Disables interrupts, programs Amiga audio channel one with the sample pointer, length, period and a volume scaled by the global level, then enables that channel's DMA and interrupt and re-enables interrupts.
- `configure_audio_channel_two`: Disables interrupts, programs Amiga audio channel two with the sample pointer, length, period and a volume scaled by the global level, then enables that channel's DMA and interrupt and re-enables interrupts.
- `configure_audio_channel_three`: Disables interrupts, programs Amiga audio channel three with the sample pointer, length, period and a volume scaled by the global level, then enables that channel's DMA and interrupt and re-enables interrupts.
- `set_audio_dma_control`: Builds a single-bit mask from the value in the data register and writes it to the DMA control register, then writes the same bit shifted seven places up to the interrupt enable register for the matching channel interrupt.
- `update_four_channel_slots`: Loops over the four channel slots of the driver.
- `update_channel_slots_from_index`: Continues the four-slot update loop from the current index, applying the same pending value and level-adjustment steps, and returns once every slot has been visited.
- `adjust_channel_level_step`: Subtracts a delta from a channel level field, advances the slot's counter when the level crosses its limit, and moves through the four slots, returning when all are done.
- `clear_remaining_channel_slots`: Walks the remaining channel slots, applying pending level adjustments and clearing each slot's state, and returns when all slots are done.
- `write_scaled_channel_volume`: Multiplies a level value by the global level, shifts the product right by six, and writes it to the volume register of the audio channel given by the index.
- `set_audio_channel_volume`: Scales the global level by a per-channel factor taken from a table, shifts the result, and writes it to the volume register of the audio channel given by the index.
- `clear_sound_flags`: Checks four bit flags in a channel byte and, for each flag that is set, calls two other routines.
- `advance_channel_zero_state`: Advances the playback state of channel zero.
- `load_channel_zero_sample`: Copies the sample pointer and two words from a sound block into the channel zero fields and records the channel bit of 128.
- `copy_channel_zero_parameters`: Copies a long word and two words from a source block into the parameter fields of channel zero, then records the channel bit of 128 in two fields.
- `advance_channel_one_state`: Advances the playback state of channel one.
- `load_channel_one_sample`: Copies the sample pointer and two words from a sound block into the channel one fields and records the channel bit of 256.
- `copy_channel_one_parameters`: Copies a long word and two words from a source block into the parameter fields of channel one, then records the channel bit of 256 in two fields.
- `advance_channel_two_state`: Advances the playback state of channel two.
- `load_channel_two_sample`: Copies the sample pointer and two words from a sound block into the channel two fields and records the channel bit of 512.
- `copy_channel_two_parameters`: Copies a long word and two words from a source block into the parameter fields of channel two, then records the channel bit of 512 in two fields.
- `advance_channel_three_state`: Advances the playback state of channel three.
- `load_channel_three_sample`: Copies the sample pointer and two words from a sound block into the channel three fields and records the channel bit of 1024.
- `copy_channel_three_parameters`: Copies a long word and two words from a source block into the parameter fields of channel three, then records the channel bit of 1024 in two fields.
- `remove_audio_handlers_and_free`: Restores the four audio interrupt vectors through the system vector routine, then passes the memory block given to it to the system free routine.
- `restore_audio_state`: Saves two fields of the channel structure, runs another routine and, unless it returns three, sets a busy flag and repeats another routine until the stream position stops advancing.
- `start_note_from_period_table`: Handles a note event by storing the note and its velocity in the channel record and looking up a playback period in a note table for that key.
- `release_or_start_note`: Handles a note event where a zero velocity byte releases a matching playing note by moving it into its release state.
- `start_indexed_note`: Stores a note into the channel record at the current index, looks up its playback period from the note table, sets the channel's active bit and records the note value.
- `set_flag_and_write_byte`: Sets one channel bit in the active-channel byte, stores a word into a channel field, calls another routine, and writes a byte into the per-channel table at the given offset.
- `release_matching_channel_note`: If the channel is not in its busy state and the stored key matches the byte at the pointer, copies an envelope value from the release table and moves the slot to state four.
- `release_note_if_matching`: Compares the byte at the pointer with the stored key for the slot and, when they match and a state is set, copies a value from the release table and moves the slot to state four.
- `tiny_moveq_move_rts`: Small routine of 3 instructions that loads 255 into d0, then copies 4 to d2, then returns.
- `initialize_audio_channels`: Allocates the audio channel control blocks, copies an eight-word table from global data into the first block, clears flags and stores global pointers into the channel structures.
- `fill_voice_record_for_channel`: Fills the voice record for a channel from its sound data, setting the sample pointer, period, length and envelope values and marking the channel active.
- `initialize_sound_channel`: Writes fixed envelope and volume defaults into a channel's voice record, copies one byte from the sound data into the channel state, and marks the channel active.
- `reset_voice_record_for_channel`: Clears the voice record for a channel, sets its sample pointer and period from the sound block, and sets its volume to the global level plus twelve, limited to 64.
- `initialize_sound_driver_state`: Copies one field to another inside the structure, sets a state word to three, copies a sound-block word into a field, and sets its volume to the global level plus twelve, limited to 64.

### Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_empty_sequence`: An empty routine that performs no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_dma_and_clear_memory`: Disables DMA transfers, clears a memory block, and updates global state variables.
- `process_complex_logic_tree`: Performs a series of conditional decrements on a global variable and executes multiple nested calls to another routine based on the variable's value.
- `save_state_and_enable_interrupts`: Saves the current value of the interrupt control register to a global variable, writes a new vector address, and enables interrupts by writing to the hardware register.
- `restore_saved_interrupt_state`: Restores a value from a global variable to the interrupt control register and then writes a specific enable value to it.
- `call_graphics_library_function`: Retrieves a function pointer from a table and executes it, storing the result in two global variables if the return value is non-zero.
- `call_library_function_8`: Calls a function pointer located at an offset from a global variable.
- `copy_and_initialize_memory`: Copies data from one memory location to another, then fills a subsequent memory range with specific pattern values.
- `tiny_ori`: Small routine of 1 instruction that merges 200 into d7.
- `tiny_ori_2`: Small routine of 1 instruction that merges 116 into d7.
- `setup_dma_transfer`: Stores a value from a global variable into a register, sets a memory address, and writes a value to the DMA control register.
- `restore_dma_register`: Restores a previously stored value to a DMA control register.
- `clear_memory_block`: Fills a large block of memory starting at a specific address with zeros.
- `decrement_and_toggle_flag`: Decrements a global variable and, if it reaches zero, toggles a bit in another global variable and resets the counter.
- `perform_blitter_operation_a`: Executes a loop that calls another routine and performs several assignments to global variables.
- `perform_blitter_operation_b`: Executes a loop that calls another routine and performs several assignments to global variables using different constant values.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `process_data_sequence`: Increments a global variable and, upon reaching a specific value, iterates through a table to find a matching byte, then executes a loop that calls another routine based on a value retrieved from a table.
- `execute_table_loop`: Increments a global variable and, when it reaches two, searches a table for a matching byte to determine an index for a loop that repeatedly calls another routine.
- `update_memory_offsets`: Retrieves a value from a table using a global index, multiplies it by a constant, and if the result meets a threshold, stores several calculated offsets into different global variables.
- `copy_memory_block_2`: Performs a block copy of data from one memory location to another using a loop, then updates a global variable and a specific memory location based on a conditional check.
- `shift_memory_data`: Decrements a global variable and, if it reaches zero, performs a block move of data within a table and updates a global variable based on a conditional check.
- `move_table_elements`: Decrements a global variable and, if it reaches zero, copies a block of data from one location in a table to another and updates a global variable based on a conditional check.
- `copy_data_forward`: Decrements a global variable and, if it reaches zero, copies a block of data from a source location to a destination location and updates a global variable based on a conditional check.
- `interleave_memory_blocks`: Decrements a global variable and, if it reaches zero, copies data from two different source locations into a single destination block using a loop, then performs a final update to a global variable.
- `decrement_counters_and_call`: Iterates through three parallel tables, decrementing values in one table and, if a value reaches zero, performing a replacement from another table and calling a routine.
- `calculate_complex_offset`: Performs arithmetic operations on two input values and a modified version of the first to calculate and store two large results into global variables.
- `tiny_move_3`: Small routine of 1 instruction that copies d2 to -(a7).

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_complex_address`: Performs arithmetic on input registers and stores the resulting values into two global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_complex_arithmetic`: Performs a series of arithmetic operations involving shifts and carries to calculate values, which are then stored in two global variables.
- `call_conditional_routine`: Checks if a word in a register is non-zero and calls another routine if it is.
- `call_conditional_routine_2`: Checks the zero flag and calls another routine if the flag is not set.
- `populate_data_tables`: Iterates through a loop to call another routine multiple times, storing the results and calculated values into several different memory tables.
- `set_memory_values`: Calls another routine and then stores a constant value into two word-sized locations at an offset from a pointer.
- `pack_bitfields_into_table`: Combines bits from two input registers and stores the resulting values into a table via a pointer.
- `process_data_and_update_table`: Iterates through a data block to find a specific value and then populates a table with calculated values based on a source list.
- `reset_globals_to_constant`: Writes zero to five global variables and writes a constant value to a sixth global variable.
- `update_audio_parameters`: Updates audio hardware registers and internal volume or pitch variables based on command flags.
- `update_sprite_attributes`: Updates coordinate and frame data in a destination structure by performing arithmetic on values extracted from a source table, while also interacting with audio and DMA hardware registers.
- `update_sprite_attribute_table`: Reads sprite data from a table based on a nibble-extracted index and updates a destination structure with coordinate and frame information.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
- `call_library_slot_144`: Calls the routine whose address is stored in a fixed memory location, then returns.
