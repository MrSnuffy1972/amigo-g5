# Game 2430 (hero's quest (1990)(sierra)(disk 1 of 5)[cr neutron]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02430). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (913536 top-level bytes in 28 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 19 of 19 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 261 of 261 named (0 author symbols), 261 of 261 described (44 by rule, 62 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1576 of 1576 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5195 of 5195 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 13527 |
| code | 17674 |
| data | 774652 |
| padding | 6307 |
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
| Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron].adf:RESOURCE.DAT | 3 | 942 | 55 |
| Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron].adf:PROG | 66 | 29 | 2 |
| Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron].adf:HERO | 2 | 119 | 3 |
| Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron].adf:AMIGASND.DRV | 3 | 1030 | 74 |
| Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 487 | 27 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 384 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 2 |

## Cases handled

- hero's quest (1990)(sierra)(disk 1 of 5)[cr neutron]/Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron].adf:RESOURCE.000: high-entropy data file (7.70 bits/byte), classified as packed data
- hero's quest (1990)(sierra)(disk 1 of 5)[cr neutron]/Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron].adf:RESOURCE.001: high-entropy data file (7.83 bits/byte), classified as packed data
- runtime segment an address (play-rs-x25k): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 315 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 260 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (40 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- hero's quest (1990)(sierra)(disk 1 of 5)[cr neutron]/Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron].adf:PROG: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- hero's quest (1990)(sierra)(disk 1 of 5)[cr neutron]/Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron].adf:HERO: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_words_to_buffer`: Copies sixteen 32-bit words from a fixed offset in the base-register block into the buffer whose pointer is passed on the stack, skipping the copy when that pointer is null.

### RunBack

- `record_timed_out_entry_routine`: The decompiler timed out, so no pseudo-code exists for this program entry.
- `run_program_as_background_task`: Parses a command line for an optional load delay and a program name with its parameters, then starts that program as a background task.
- `clear_forty_longs_and_link`: Clears forty consecutive 32-bit words in a table selected by an index passed on the stack, stores the address of the stack frame and a global variable into two fields after the block, then calls another routine.
- `call_optional_hook_then_finish`: If a global function pointer is set, calls the routine it holds, then calls a second routine and returns.
- `open_libraries_and_split_text`: Opens the dos and math libraries by name and, when both open, splits a text buffer into words by writing zero bytes between runs and building an array of pointers to them.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `run_back_loop_handler`: Calls another routine ten times, then calls an optional function pointer from global data and, depending on several other global values, calls further routines with arguments taken from those values.
- `append_string_to_buffer`: Appends a source string to the end of a destination string, stopping after 32766 characters at most.
- `append_string_with_limit`: Appends a source string to the end of a destination string, copying no more characters than the count argument allows.
- `copy_string_to_buffer`: Copies bytes from a source string into a destination buffer until a zero byte has been copied, including the terminator, and returns the destination pointer.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns its length in bytes, excluding the terminator.
- `copy_bytes_with_padding`: Copies bytes from a source pointer for up to a count taken from a 32-bit parameter, stopping early at a zero byte, then writes zero bytes to fill the rest of the count.
- `release_indexed_table_entry`: Checks an index into a twenty-entry table of six-byte records and, if the record is in use and its flag bit is clear, releases the record's handle through a library vector and clears its pointer.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_slot_102`: Loads a library base from global data and jumps to the vector at offset minus 102, forwarding the register arguments unchanged.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_144_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_slot_84`: Loads a library base from global data and jumps to the vector at offset minus 84, forwarding the register arguments unchanged.
- `call_library_slot_30`: Loads a library base from global data and jumps to the vector at offset minus 30, forwarding the register arguments unchanged.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_slot_60`: Loads a library base from global data and jumps to the vector at offset minus 60, forwarding the register arguments unchanged.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_slot_90`: Loads a library base from global data and jumps to the vector at offset minus 90, forwarding the register arguments unchanged.
- `call_library_slot_48`: Loads a library base from global data and jumps to the vector at offset minus 48, forwarding the register arguments unchanged.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_slot_198`: Loads a library base from a second global table and jumps to the vector at offset minus 198, forwarding the register arguments unchanged.
- `call_library_slot_198_2`: Loads a library base from a second global table and jumps to the vector at offset minus 198, forwarding the register arguments unchanged.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_slot_210`: Loads a library base from a second global table and jumps to the vector at offset minus 210, forwarding the register arguments unchanged.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### CloseWB

- `wait_then_close_workbench`: Opens a library by name through the system's old open routine, polls a function in it until that returns non-zero, then calls the workbench close routine through the global base.

### ENDCLI

- `scan_entries_and_fill_table`: Program entry that walks a linked list of entries, keeping the largest value of one field, and then requests a block through the library sized from that value.
- `copy_and_call_routine`: Copies a block of sixteen bytes from a table to a destination and then calls another routine.

### ECHO

- `update_linked_list_and_copy_data`: Traverses a linked list to find the maximum value in a specific field, then updates a header structure and performs a series of memory copies between a source buffer and a destination buffer based on the calculated maximum.
- `copy_data_and_call_function`: Copies sixteen bytes from a memory location offset from a pointer into a destination address on the stack.

### Execute

- `update_linked_list_and_copy_data_2`: Traverses a linked list to find the maximum value in a specific field and then updates a header structure.
- `copy_sixteen_words`: Copies sixteen words from a source pointer to a destination pointer.

### ADD21K

- `configure_display_from_library`: Opens a library by name through a global name string, falling back to a second open attempt if the first fails.
- `open_graphics_and_intuition`: Opens the graphics and intuition libraries by name, sets a flag in a structure reached through the first base and calls another routine with the values 640 and 256.
- `clear_forty_two_longs`: Clears forty-two consecutive 32-bit words in a table selected by an index passed on the stack, stores the address of the stack frame and a global variable into two fields after the block, then calls another routine.
- `call_indirect_if_flag_then_recurse`: Checks a global variable for a non-zero value to call an indirect function pointer, then calls another routine.
- `parse_dialogue_text`: Obtains two identifiers from a global table and stores them in global variables, then branches based on their values.
- `execute_callback_chain`: Executes a series of function pointers and nested calls stored in a table or structure.
- `initialize_game_state`: Runs a short loop ten times calling another routine, then checks several global flags to execute specific routines or library calls based on their values.
- `copy_string_with_large_limit`: Copies bytes from a source to a destination until a null terminator is found or a very large counter expires, ensuring the destination is null-terminated.
- `copy_string_with_limit`: Finds the end of a destination string, then copies bytes from a source pointer into that destination until a null byte is reached or a counter reaches zero, ensuring the destination is null terminated.
- `copy_string_to_buffer_2`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length_2`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `copy_bytes_with_padding_2`: Copies bytes from a source pointer to a destination pointer for a specified count, stopping if a zero byte is found, and fills the remaining destination area with zero bytes.
- `lookup_and_execute_function`: Validates an index against a table, checks if a function pointer at that index is non-zero, executes it if the high bit is not set, and then clears the entry.
- `call_library_slot_36`: Loads a library base from global data and jumps to the vector at offset minus 36, forwarding the register arguments unchanged.
- `call_library_slot_126`: Loads a library base from global data and jumps to the vector at offset minus 126, forwarding the register arguments unchanged.
- `call_library_slot_144`: Loads a library base from global data and jumps to the vector at offset minus 144, forwarding the register arguments unchanged.
- `call_library_slot_54`: Loads a library base from global data and jumps to the vector at offset minus 54, forwarding the register arguments unchanged.
- `call_library_offset_thirty`: Calls a function via a pointer located at a specific offset within a library structure.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested table structure using an offset from a base address and executes it.
- `call_library_slot_108`: Loads a library base from global data and jumps to the vector at offset minus 108, forwarding the register arguments unchanged.
- `call_library_slot_414`: Loads a library base from global data and jumps to the vector at offset minus 414, forwarding the register arguments unchanged.
- `call_library_slot_414_2`: Loads a library base from global data and jumps to the vector at offset minus 414, forwarding the register arguments unchanged.
- `call_library_slot_198_3`: Loads a library base from global data and jumps to the vector at offset minus 198, forwarding the register arguments unchanged.
- `call_library_slot_294`: Loads a library base from global data and jumps to the vector at offset minus 294, forwarding the register arguments unchanged.
- `call_library_slot_132`: Loads a library base from global data and jumps to the vector at offset minus 132, forwarding the register arguments unchanged.
- `call_library_slot_210_2`: Loads a library base from global data and jumps to the vector at offset minus 210, forwarding the register arguments unchanged.
- `call_library_slot_372`: Loads a library base from global data and jumps to the vector at offset minus 372, forwarding the register arguments unchanged.
- `call_library_slot_552`: Loads a library base from global data and jumps to the vector at offset minus 552, forwarding the register arguments unchanged.
- `call_library_slot_552_2`: Loads a library base from global data and jumps to the vector at offset minus 552, forwarding the register arguments unchanged.
- `call_library_slot_378`: Loads a function pointer from a table reached through the saved base register and jumps to it, forwarding the call to an Amiga library routine at vector offset minus 378.
- `call_library_slot_384`: Loads a function pointer from the base-register table and jumps to it, forwarding the call to an Amiga library routine at vector offset minus 384.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_alternate_base_library_slot`: Loads a function pointer from a second base-register table and jumps to it, forwarding the call to an Amiga library routine at vector offset minus 384.

### Wait

- `update_linked_list_buffer`: Traverses a linked list to find the maximum value in a specific field and then updates a data structure by writing a header and copying a block of words.
- `copy_multiple_words`: Copies a fixed number of words from a source pointer to a destination pointer.

### Disk-Validator

- `compare_bitmap_pairs_for_mismatches`: Program entry that walks a range of block numbers and tests the matching bit in two bitmap word arrays, counting the blocks where the bits disagree.

### Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron].adf:RESOURCE.DAT

- `zero_twenty_global_words`: Calls another routine, then clears twenty consecutive 32-bit words in the global data area with a countdown loop.
- `initialize_object_state`: Stores a parameter value into one field of the object passed in, sets a word field to the constant forty, and points another field at a global variable.
- `delay_and_configure_game_object`: Performs a busy wait, queries a global structure to copy several fields into a stack frame, and executes a sequence of library calls and initialization routines.
- `allocate_tagged_resource_entry`: Allocates a resource entry sized from a count in the global data and, if that succeeds, sets its state fields and writes a four-character tag into its header.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `match_name_and_build_list`: Compares a stored name string against a second name and, on a match, walks a linked list of nodes, allocating and filling one record per node with pointers and values.
- `stub_bad_instruction_data`: The decompiled body is only a return, so the listed instructions could not be recovered.
- `copy_block_handling_overlap`: Copies the given number of bytes from a source pointer to a destination pointer, returning at once when both are equal.
- `copy_bytes_forward_or_reverse`: Copies a counted block of bytes between two pointers, returning at once when they are the same address.
- `stub_empty_return_routine`: The body is only a return with no work, so the routine is a stub.
- `tokenize_command_line_into_argv`: Builds an argument vector from a command string by copying a fixed string and the argument text into a buffer, then splitting the text on whitespace while keeping double-quoted sections together.
- `copy_string_with_limit_3`: Copies bytes from a source pointer to a destination pointer with a large limit and ensures a null terminator is placed at the end.
- `copy_string_with_limit_2`: Copies bytes from a source pointer to a destination pointer until a null terminator is reached or a specified counter reaches zero.
- `copy_string_pad_zeros`: Copies characters from a source string into a destination buffer, stopping at the source null terminator or when the count is used up.
- `fetch_icon_library_entries`: Opens the icon library, then looks up a window-related entry and calls two library vectors to fetch values, which are stored into the structure passed in.
- `call_indirect_hook_if_set`: If a global pointer slot is non-zero, calls the routine whose address it holds.
- `call_indirect_if_flagged_recurse`: Calls the routine held in a global pointer slot when that slot is non-zero, then makes a no-argument call to another routine.
- `process_resource_descriptor`: Walks a descriptor held in a register-based structure, calling a routine for each non-zero field and storing one value through a pointer field.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `release_resource_slot`: Checks that a slot index is in range and that the slot is in use, otherwise sets an error code and returns failure.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `call_library_function_indirect`: Calls a function through a pointer retrieved from a library vector.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_exit`: Wrapper that calls the dos library function Exit and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_load_seg`: Wrapper that calls the dos library function LoadSeg and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_indirect_routine_via_table`: Loads a routine address from a table held in a global block and calls it with no arguments, then returns.
- `dispatch_library_vector_414`: Jumps through the library vector at offset -414 from the library base pointer held in a global slot, which in turn calls an unnamed routine.
- `call_library_vector_414`: Calls the library function at vector offset -414 from the library base pointer held in a global slot and returns the result.
- `dispatch_library_vector_198`: Jumps through the library vector at offset -198 from the library base pointer held in a global slot.
- `call_library_vector_198`: Calls the library function at vector offset -198 from the library base pointer held in a global slot and returns its result.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `dispatch_library_vector_294`: Jumps through the library vector at offset -294 from the library base pointer held in a global slot.
- `call_library_vector_294`: Calls the library function at vector offset -294 from the library base pointer held in a global slot and returns its result.
- `call_library_vector_132`: Calls the library function at vector offset -132 from the library base pointer held in a global slot and returns its result.
- `call_library_vector_210`: Calls the library function at vector offset -210 from the library base pointer held in a global slot and returns its result.
- `dispatch_library_vector_372`: Jumps through the library vector at offset -372 from the library base pointer held in a global slot.
- `call_library_vector_372`: Calls the library function at vector offset -372 from the library base pointer held in a global slot and returns its result.
- `dispatch_library_vector_552`: Jumps through the library vector at offset -552 from the library base pointer held in a global slot.
- `call_library_vector_552`: Calls the library function at vector offset -552 from the library base pointer held in a global slot and returns its result.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_378`: Calls the library function at vector offset -378 from the library base pointer held in a global slot and returns its result.
- `dispatch_library_vector_384`: Jumps through the library vector at offset -384 from the library base pointer held in a global slot.
- `call_library_vector_384`: Calls the library function at vector offset -384 from the library base pointer held in a global slot and returns its result.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_78`: Calls the library function at vector offset -78 from a second library base pointer held in a global slot and returns its result.
- `call_library_vector_198_2`: Calls the library function at vector offset -198 from a second library base pointer held in a global slot and returns its result.

### Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron].adf:PROG

- `run_startup_and_free_memory`: Program entry for this hunk.
- `decrement_until_minus_one`: Decrements a 16-bit register variable in a loop until it reaches minus one, then returns.

### Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron].adf:HERO

- `unpack_compressed_hunk_data`: Program entry for this hunk.
- `read_shifted_previous_longword`: Reads the 32-bit value just before the address register, shifts it right by one bit, sets its top bit and returns the result.
- `shift_bit_stream_backwards`: Shifts a bit register right one place.

### Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron].adf:AMIGASND.DRV

- `deferred_callback_dispatcher`: Checks a one-bit flag in a global byte.
- `initialize_sound_stream_state`: Initialises a sound channel state block.
- `set_status_and_clear_byte`: Stores zero through the address register and sets a status word in the structure to three, then returns that status word.
- `parse_sound_stream_header`: Scans forward through a byte stream to a terminator byte, recording its length.
- `decode_sound_command_block`: Initialises a sound command block in a caller-provided structure, decodes a stream of bytes through a mask from a global variable to select up to four channel values, writes them into a 16-byte buffer and terminates it, clears several command fields, and returns a status word from the structure.
- `check_bell_and_write_byte`: Checks whether the byte at the stream pointer is the bell character and whether a flag in the structure is clear.
- `store_byte_conditional`: Checks a global variable against four.
- `store_byte_or_mark_end`: Reads a byte from the stream pointer.
- `store_word_field_value`: Stores the word held in the data register into a field of the address-register structure and returns.
- `advance_sound_stream_decoder`: Advances the sound stream decoder by one step.
- `reset_playback_state`: Clears a global word, sets a field in the structure to minus one, sets a channel state byte to two and returns zero.
- `advance_channel_state_machine`: Advances a channel state machine driven by a state byte.
- `service_channel_timer_step`: When the channel state byte is three, it clears a flag, calls two routines and moves the state to four.
- `countdown_channel_timer_step`: Counts down a channel timer byte and, when it reaches zero, reloads it and decrements a global count.
- `clear_sound_state`: Calls a routine, then if a global variable is zero it writes the value minus one into a word field of the structure and sets a channel state byte to two.
- `run_sound_stream_decoder_step`: Reads the next command from a sound byte stream, first handling a pending flag by calling a routine.
- `parse_sound_command_bytes`: Parses command bytes from the sound stream until a data byte or a timing limit is reached, handling command codes such as end markers and jumps.
- `record_channel_position_and_wait`: Decrements a wait counter in the structure, stores the current command and command value bytes, and records the stream offset relative to the stream base.
- `scan_sound_commands_until_end`: Loops over sound command bytes, handling the end-of-stream code and the other command codes.
- `process_sound_command_stream`: Reads sound command bytes in a loop, dispatching command codes for end, repeat and skip to their handlers.
- `accumulate_sound_wait_counter`: Skips the operand bytes of a command, adds the following wait byte to the channel's wait counter, and stores the command and offset back into the structure.
- `dispatch_sound_control_codes`: Dispatches a sound control code.
- `skip_sound_command_operands`: Skips the operand bytes that follow a sound command, which are one, two or three bytes depending on the code, or a run ending at a terminator byte.
- `advance_sound_stream_past_operands`: Advances the stream pointer past the operand bytes of a two-byte or three-byte sound command, then stores the channel position and wait value in the structure.
- `parse_next_sound_command`: Advances past a single operand byte for one command code.
- `handle_midi_channel_message`: Compares a channel number with the channel index stored in the structure.
- `handle_midi_status_messages`: Handles MIDI-style note-off, control change and program change status codes.
- `handle_midi_control_program_messages`: Handles the control change and program change status codes.
- `handle_midi_program_change`: Handles the program change status code by calling a routine that consumes its data byte.
- `forward_to_stream_handler`: Calls one routine with no arguments and returns, forwarding the call to the stream handler.
- `store_channel_bytes_and_offset`: Stores two byte values into the structure and sets its position word to the argument minus the stream base.
- `reset_sound_driver_state`: Sets a word field to minus one, copies a saved position word into the stream position field, calls a routine, then sets a flag byte and clears a wait word in the structure.
- `clamp_and_scale_value`: Caps a 16-bit value in the structure at 64 and, when a flag byte in the structure is zero, stores four times that value in a global word.
- `restore_saved_word_and_call`: Saves a word field of the structure, clears it, calls a routine four times, then restores the saved word.
- `call_routine_then_set_flag`: Calls a routine, then sets a flag byte in the structure to one.
- `arm_channel_countdown`: Sets a channel flag byte to one and a countdown byte to eight in the structure.
- `configure_audio_channel_zero`: Disables interrupts through the exec library, programs audio channel zero's pointer, length, period and volume from a structure, sets its DMA enable bit and interrupt enable bit, then re-enables interrupts.
- `configure_audio_channel_one`: Disables interrupts through the exec library, programs audio channel one's pointer, length, period and volume from a structure, sets its DMA enable bit and interrupt enable bit, then re-enables interrupts.
- `configure_audio_channel_two`: Disables interrupts through the exec library, programs audio channel two's pointer, length, period and volume from a structure, sets its DMA enable bit and interrupt enable bit, then re-enables interrupts.
- `configure_audio_channel_three`: Disables interrupts through the exec library, programs audio channel three's pointer, length, period and volume from a structure, sets its DMA enable bit and interrupt enable bit, then re-enables interrupts.
- `set_audio_dma_control`: Builds a one-bit channel mask from the input value, writes it to the DMA control register, and writes a shifted copy of the mask to the interrupt enable register.
- `step_sound_channel_slots`: Walks four channel slots in a sound structure, copying each slot's pending value from a table.
- `update_sound_slots`: Iterates through sound slots to manage active status, decrement byte values, and update data from a table based on specific conditions.
- `step_sound_slot_levels`: Subtracts a value from a slot's level and advances the slot index when the level falls below its threshold.
- `retire_finished_sound_slots`: Walks the four sound slots.
- `write_audio_register_value`: Multiplies an input value by a scale factor from a global variable, shifts the result right six places, and stores it into the audio channel register selected by an index.
- `set_audio_channel_scaled_level`: Scales a global value by a second copy of itself and by a per-slot byte from a table, shifts the result, and stores it into the audio channel volume register chosen by the slot index.
- `clear_sound_flags`: Checks four bits of a flag byte in the structure.
- `advance_audio_channel_zero_setup`: Advances the setup of audio channel zero in steps.
- `continue_audio_channel_zero_setup`: Continues the setup of audio channel zero.
- `copy_sound_driver_fields`: Copies a 32-bit pointer and two 16-bit values from a source block into fixed fields of a channel block, then sets two channel fields to 128.
- `advance_audio_channel_one_setup`: Advances the setup of audio channel one in steps.
- `continue_audio_channel_one_setup`: Continues the setup of audio channel one.
- `copy_sound_header_fields`: Copies a 32-bit pointer and two 16-bit values from a source block into fields of the channel block, then sets two channel fields to 256.
- `advance_audio_channel_two_setup`: Advances the setup of audio channel two in steps.
- `continue_audio_channel_two_setup`: Continues the setup of audio channel two.
- `copy_sound_parameters`: Copies a 32-bit pointer and two 16-bit values from a source block into fixed fields of a channel block, then sets two channel fields to 512.
- `advance_audio_channel_three_setup`: Advances the setup of audio channel three in steps.
- `continue_audio_channel_three_setup`: Continues the setup of audio channel three.
- `copy_sound_params`: Copies a 32-bit pointer and two 16-bit values from a source block into fixed fields of the channel block, then sets two channel fields to 512.
- `shut_down_sound_driver`: Shuts the sound driver down.
- `restore_audio_state`: Saves a word from the object and a position word, then calls a routine.
- `dispatch_sound_channel_command`: Reads a two-byte command from the sound stream.
- `handle_sound_command_pair`: Reads a two-byte command pair.
- `load_channel_sound_table_entry`: Loads the sound table entry for the given channel from the structure, copies its flag byte into the channel block, computes a level and a period from the entry's fields, and returns.
- `set_flag_and_write_byte`: Sets one bit in a channel flag byte selected by an index, writes a 16-bit value into the structure, calls a routine, then stores an input byte into a byte table indexed by a short parameter.
- `update_midi_status`: Compares a byte from a pointer with a value in a table and, if they match, copies a byte from a secondary table into a destination buffer.
- `compare_and_copy_byte`: Compares a byte at a pointer with a value from a table using a word offset, then optionally copies a byte from a second table to a destination based on a control byte.
- `tiny_moveq_move_rts`: Small routine of 3 instructions that loads 255 into d0, then copies 4 to d2, then returns.
- `initialize_sound_channel_blocks`: Allocates a memory block through the exec library and, if that succeeds, copies a seven-entry table from a global block into it.
- `fill_sound_channel_record`: When the playback flag is clear, fills a channel record in the driver's record table from a sound table entry.
- `initialize_sound_channel`: Initialises a sound channel record in the driver's table, writing fixed values to its fields and copying a byte from the channel block into the record.
- `reset_sound_channel_record`: Resets a sound channel record by clearing its first word, flag byte and counter.
- `initialize_sound_driver_state`: Copies a pointer field, sets a state word to three, copies a value from the driver block into a field, and caps a level field at 64.

### Hero's Quest (1990)(Sierra)(Disk 1 of 5)[cr Neutron].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_sequence_of_calls`: Executes a series of subroutine calls and updates a global variable through several assignments and a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_dma_and_clear_memory`: Disables DMA transfers, clears a memory block, and updates global state variables.
- `execute_complex_logic_flow`: Performs a series of conditional checks and recursive calls to another routine while modifying a global variable and several hardware and memory values.
- `save_state_and_enable_interrupts`: Saves the previous value of the interrupt control register, stores a new vector address and writes the master-enable and interrupt-enable bits.
- `restore_saved_interrupt_state`: Restores a value from a global variable to a register and writes a specific value to the interrupt enable register.
- `call_library_function_pointer`: Calls a function pointer retrieved from an offset of a global variable and stores the result in two word-sized locations if the return value is non-zero.
- `call_library_function`: Calls a function pointer located at an offset from a global variable.
- `copy_and_initialize_memory`: Copies data from one memory location to another, then fills a subsequent memory range with specific pattern values.
- `tiny_ori`: Small routine of 1 instruction that merges 200 into d7.
- `tiny_ori_2`: Small routine of 1 instruction that merges 116 into d7.
- `setup_dma_transfer`: Stores a value from a global variable into a register, sets a memory address, and writes a value to the DMA control register.
- `restore_address_register`: Restores a value from a global variable into a register.
- `clear_memory_block`: Fills a large block of memory starting at a specific address with zeros.
- `decrement_and_toggle_flag`: Decrements a global variable and, if it reaches zero, toggles a bit in another global variable and resets the counter.
- `perform_blitter_operation_a`: Executes a loop that calls another routine and performs several assignments to global variables.
- `perform_blitter_operation_b`: Executes a loop that calls another routine and performs several assignments to global variables using different constant values.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `process_data_sequence`: Increments a global variable and, upon reaching a specific value, iterates through a table to find a matching byte, then executes a loop that calls another routine based on a value retrieved from a table.
- `execute_table_loop`: Increments a global variable and, when it reaches two, searches a table for a matching byte to determine an index for a loop that repeatedly calls another routine.
- `update_memory_offsets`: Retrieves a value from a table using a global index, multiplies it by a constant, and if the result meets a threshold, stores several calculated offsets into different global variables.
- `copy_memory_block`: Performs a block copy of data from one memory location to another using a loop, then updates a global variable and a specific memory location based on a conditional check.
- `shift_memory_data`: Decrements a global variable and, if it reaches zero, performs a block move of data within a table and updates a global variable based on a conditional check.
- `move_table_elements`: Decrements a global variable and, if it reaches zero, copies a block of data from one location in a table to another and updates a global variable based on a conditional check.
- `copy_data_forward`: Decrements a global variable and, if it reaches zero, copies a block of data from a source location to a destination location and updates a global variable based on a conditional check.
- `interleave_memory_blocks`: Decrements a global variable and, if it reaches zero, copies data from two different source locations into a single destination block using a loop, then performs a final update to a global variable.
- `decrement_counters_and_call`: Iterates through three parallel tables, decrementing values in one table and, if a value reaches zero, performing a replacement from another table and calling a routine.
- `calculate_complex_offset`: Performs arithmetic operations on two input values and a modified version of the first to calculate and store two large results into global variables.
- `tiny_move_3`: Small routine of 1 instruction that copies d2 to -(a7).

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_complex_offsets`: Performs arithmetic operations on two input registers and stores the resulting values into two global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_complex_arithmetic`: Performs a series of arithmetic operations involving shifts and carries to calculate values, which are then stored in two global variables.
- `call_conditional_routine`: Checks if a word in a register is non-zero and calls another routine if it is.
- `call_routine_if_not_zero`: Checks the zero flag and calls another routine if the flag is not set.
- `populate_data_tables`: Iterates through a loop to call another routine multiple times, storing the results and calculated values into several different memory tables.
- `set_memory_values`: Calls another routine and then stores a constant value into two word-sized locations at an offset from a pointer.
- `pack_bitfields_into_table`: Combines bits from two input registers and stores the resulting values into a table via a pointer.
- `process_data_and_update_table`: Iterates through a data block to find a specific value and then populates a table with calculated values based on a source list.
- `reset_globals_to_constant`: Writes zero to four global variables and writes a constant value to a fifth global variable.
- `update_audio_parameters`: Updates audio channel properties and volume levels by modifying specific memory locations based on command flags.
- `update_sprite_attributes`: Updates coordinate and frame data in a destination structure by performing arithmetic on values extracted from a source table, while also interacting with audio and DMA hardware registers.
- `update_sprite_attribute_table`: Reads sprite data from a table based on an index derived from a byte, then updates a destination structure with position and attribute values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
- `call_library_vector_144`: Calls the library function at vector offset -144 through a pointer stored at a fixed runtime address, then returns.
