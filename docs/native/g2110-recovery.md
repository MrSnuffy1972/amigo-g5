# Game 2110 (ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02110). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (921232 top-level bytes in 51 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 133 of 133 named (0 author symbols), 133 of 133 described (10 by rule, 10 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2076 of 2076 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4893 of 4893 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 112481 |
| code | 14804 |
| data | 772499 |
| padding | 12232 |
| unused | 9216 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:installu6 | 2 | 267 | 8 |
| endcli | 2 | 117 | 2 |
| Saddam virus | 1 | 4 | 1 |
| Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:shut.txt | 2 | 267 | 8 |
| Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:shut.exe | 1 | 86 | 5 |
| Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 378 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 689 | 27 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3049 | 63 |

## Cases handled

- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:INTRO_3.NEW: high-entropy data file (7.79 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:intro.new: high-entropy data file (7.83 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:paper.bmp: high-entropy data file (7.93 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:mainmenu.new: high-entropy data file (7.64 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:ULTIMA_Ba.MOD: high-entropy data file (7.79 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:INTRO_1.NEW: high-entropy data file (7.85 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:ULTIMA_Ga.MOD: high-entropy data file (7.80 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:ULTIMA_Fa.MOD: high-entropy data file (7.91 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:u: high-entropy data file (7.76 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:ultcli: high-entropy data file (7.68 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:intro_1a.new: high-entropy data file (7.79 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:ULTIMA_Da.MOD: high-entropy data file (7.84 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:intro_2.new: high-entropy data file (7.65 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:gypsy.new: high-entropy data file (7.83 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:ultima6: high-entropy data file (7.68 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:ULTIMA_Aa.MOD: high-entropy data file (7.83 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:game: high-entropy data file (7.80 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:vellum1.new: high-entropy data file (7.83 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:intro_2a.new: high-entropy data file (7.90 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:title.new: high-entropy data file (7.77 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:ULTIMA_Ea.MOD: high-entropy data file (7.72 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:intro_1b.new: high-entropy data file (7.75 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:blocks.new: high-entropy data file (7.90 bits/byte), classified as packed data
- runtime segment an address: 294 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 488 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 24 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 686 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:installu6: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:installu6

- `load_hunk_sections_with_relocations`: Reads AmigaDOS load-file records, allocates memory for each code and data section, copies the section contents, applies 32-bit relocation entries, and stops at the end marker.
- `count_down_wait_loop`: Counts a 16-bit register down to zero in a tight loop and returns, so it waits a fixed number of iterations.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_backward_run_data`: Decodes a packed stream backwards from the end of a buffer, writing literal bytes and copying earlier runs of output.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `extract_bits_from_stream`: Reads bits one at a time from a stream of longwords into an accumulated value over a fixed bit count, refilling from memory when the buffer empties.
- `stub_bad_instruction_data`: A four-instruction fragment that makes a trap call and adds to a longword, then runs into bad instruction data.

### endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_then_call_pointer`: Copies a block of fifteen longwords from a table at a fixed offset into a caller-supplied buffer when the buffer pointer is nonzero, then calls a routine through a stored function pointer.

### Saddam virus

- `return_failure_code`: A program entry that immediately returns minus one to its caller.

### Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:shut.txt

- `unpack_amiga_hunk_data`: Decodes a compressed data stream by identifying specific hunk type codes and allocating memory to store the unpacked results.
- `decrement_and_loop`: Decrements a 16-bit value in a register until it reaches zero.
- `return_only_stub`: A single return instruction that does nothing; it is reached only through its address being taken.
- `copy_memory_blocks`: Performs conditional memory copying and byte transfers between memory locations based on values derived from a pointer and a register.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `extract_bit_run_into_accumulator`: Reads a counted run of bits from a stream of longwords into an accumulator, tracking the carried comparison result and refilling the bit buffer from memory when it empties.
- `extract_bitstream_data`: Processes a bitstream by extracting bits using a count from a register and refilling a buffer from memory when the buffer is empty.
- `stub_bad_instruction_data_2`: A four-instruction fragment that makes a trap call, then runs into bad instruction data.

### Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:shut.exe

- `install_pointer_slots_and_call`: Stores the address of one global into four pointer slots, calls two routines, and touches the custom chip register block.
- `unpack_backward_store_end`: Decodes a packed stream backwards like its sibling routine, and additionally saves the final output position into a word at a second pointer.
- `test_register_bit`: Checks if the second bit of a register is set and returns if it is not.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `shift_bits_out_of_register`: Shifts a bit register one bit at a time for a counted number of steps, refilling it from memory with the carried-in bit when it empties.

### Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_copper_display_program`: Opens and closes operating-system libraries, clears a block of memory, copies table data into a copper list, points the copper at that list and waits in a loop on an interrupt flag.
- `copy_nested_word_table_block`: Copies a rectangular table of words row by row into a destination buffer, using nested counted loops over a fixed table offset.
- `fill_strided_table_entries`: Fills entries spaced a fixed stride apart with a byte taken from a lookup table, advancing through the table for a fixed number of iterations and updating a stored offset.
- `blit_rows_from_index_table`: Waits for the blitter to be free, sets its mask, modulo and size words, then runs a fixed number of blit operations whose source offsets come from a lookup table, advancing a stored index each time.
- `advance_frame_table_and_blit`: Counts down a frame timer; when it expires, reads the next code from a sequence, finds its column in a table of fixed-width rows, and copies the values to two destinations.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `advance_counter_and_check_limit`: Waits for the blitter, starts a blitter operation, calls two routines and increments a counter.
- `start_blit_with_masks_a`: Waits for the blitter to be free, sets its first and last word masks and control word, stores two address pointers, and starts a blit of a fixed size.
- `start_blit_with_swapped_pointers`: Waits for the blitter to be free, sets its mask and control words, stores the two address pointers in swapped order, and starts a blit of a fixed size.
- `tiny_ori_2`: Small routine of 1 instruction that merges 8 into d0.
- `refresh_slot_table_from_sequence`: Maintains eight slot records.
- `encode_four_byte_entry`: Writes a four-byte entry derived from two input values: one byte from a value offset by a constant, a flag in the fourth byte, and two bytes computed from the other input.
- `copy_record_bytes_to_buffer`: Copies seven bytes from a record into a buffer at a position computed from two inputs, placing them at a fixed row stride.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_state_block_and_call`: Clears a block of state bytes starting at a global address, then calls one routine.
- `build_pointer_table_from_offsets`: Adds a fixed base address to seven stored values and stores them in slots.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `reset_audio_state_from_entry`: Disables interrupts and DMA through their control registers, clears the four audio volume registers, copies a twelve-byte entry from a table into the state block, clears the state block, and sets three state words.
- `clear_state_return_input`: Clears a block of state bytes, sets one state byte to a fixed value and clears an audio volume register, then returns the incoming value unchanged.
- `call_routine_between_flag_set`: Clears a flag word, calls one routine, then sets the flag word to all ones.
- `set_pending_state_words`: Copies one stored word into another, stores the incoming register value into a second word, and sets a flag word to one.
- `step_audio_sequence_counter`: Advances a counter and, when it reaches its limit, resets it and calls a routine if a flag is set.
- `compute_table_pointer_entry`: Reads a selector from a record.
- `dispatch_flagged_update_routines`: Calls two routines unconditionally, then calls further routines for each nonzero flag among three words, and returns the result of a final conditional call.
- `advance_index_load_entry`: Increments a counter and, when it reaches its limit, resets it, advances an index with wraparound, and copies four longwords from the table entry at that index into four global variables.
- `load_flagged_table_values`: For each of four flags that is nonzero, loads a longword from a lookup table at an index stored in a global and writes it to a matching global variable.
- `decode_command_word_switch`: Decodes a 16-bit command word through a switch on its high bits: most cases store a value into a state field and return it, some return a sentinel, and one case updates timing and volume fields.
- `call_unresolved_jump_table`: Makes an indirect call through an address the decompiler could not recover from a jump table, then returns.
- `return_only_stub_2`: A single return instruction that does nothing and is reached only through its address being taken.
- `store_word_into_record`: Stores the incoming word into a fixed field of a record and returns.
- `apply_command_position_update`: Decodes the high byte of a packed command word: some values call routines, while others update a pair of stored position words and a record index, clearing a 61-byte block in one case.
- `store_table_entry_or_clear`: Looks up a longword in a table selected by a stored index.
- `copy_strided_record_rows`: Copies rows of a record block from a table into a destination buffer four longwords at a time, storing the destination address and a length word, and clears a byte when it finishes.
- `load_entry_fields_into_record`: Selects a table entry by a stored index and copies several word fields from it into a record, resetting the record's counters to zero.
- `copy_entry_fields_into_record`: Copies a group of word fields from a table entry passed in directly into a record and clears its counters.
- `clear_record_output_fields`: Clears an output word, two record fields and a stored index, then clears a flag byte.
- `run_four_update_routines`: Calls four routines in sequence and returns.
- `update_record_output_level`: Advances a record's sequence position and moves its level toward a target value using a division by a stored step count.
- `run_mode_processing_on_countdown`: Counts down a record's timer and, when it expires, reloads it and performs one of several byte operations chosen by a mode word, such as negating a byte, adding a constant to a range or copying bytes.
- `execute_indirect_jump`: An empty routine that performs an indirect jump to an unrecovered address.
- `negate_byte_and_advance_index`: Negates the byte at a record's current index, advances that index, and wraps it back to a start value when it passes the end.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_negative_one`: Returns a longword containing all bits set to one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_entry`: Calls one routine and checks the zero flag.
- `open_dos_library`: Opens the DOS library through the operating system's old-style open call and stores the library handle in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_dos_command_context`: Clears a block of workspace longwords, opens the operating-system and DOS libraries through function-pointer calls, and then either sets up a context from a configured entry or builds a quoted command string in newly allocated memory.
- `release_dos_resources`: Calls a stored cleanup routine if one is set, then either tears down a library handle set through close and unlock calls or releases memory and reply messages.
- `free_linked_node_list`: Walks a linked list of nodes, calling a release routine for each node, then clears the list head and the node count.
- `split_command_line_arguments`: Splits a command string into up to thirty-two argument pointers, honouring double-quoted arguments and star-prefixed escape codes that become control characters.
- `call_function_pointer`: Loads a function pointer from a fixed offset in a global data structure and calls the code it points to.
- `return_only_stub_3`: A single return instruction that does nothing; it is called from one routine.
- `empty_stub_routine`: A routine with no operations that returns immediately and is called from one routine.
- `resolve_handle_with_status`: Calls a routine to obtain a handle, returning minus one when the handle is zero.
- `emit_string_to_bounded_buffer`: Writes a string into a buffer while a remaining-length counter allows, calling a routine for characters past the limit, and returns the number of characters in the string.
- `prompt_and_dispatch_user_choice`: Reads characters until one of two command letters is pressed, then runs one of two paths: one reads and matches a path string and may run a sub-command, the other runs a list of steps through library call pointers.
- `allocate_record_if_unset`: When a record's pointer is empty or a flag bit is set, calls a routine to allocate a buffer and stores the result in two fields.
- `write_character_with_line_state`: Writes a character into an output buffer while tracking line state: a newline is written as a carriage return followed by the newline, and a column count and status flags are updated.
- `read_next_buffered_character`: Returns the next byte from a buffer, treating a carriage-return pair and an end-of-file marker specially.
- `read_buffered_character_refill`: Reads the next buffered character with the same logic as its sibling routine.
- `flush_pending_entries`: Walks a table of entries from the last to the first and calls a routine for each entry that is enabled and lacks a particular flag bit, then calls one more routine.
- `copy_three_bytes_into_record`: Copies three bytes from a source into a record and then locates a second three-byte group with a routine, storing it with a presence flag.
- `append_bounded_string`: Appends a source string to the end of a destination string, copying at most the number of characters given in a register, and terminates the result.
- `copy_string_returning_end`: Copies a null-terminated string into a destination buffer, including the terminator, and returns a pointer to the terminating position.
- `format_signed_integer`: Writes the decimal form of a signed value into a buffer: a minus sign is written first when the value is negative, digits come from a routine and are reversed into place, and a null terminator is added.
- `parse_signed_decimal_prefix`: Parses an optional sign and decimal digits from the start of a string into a stored integer and returns the number of characters consumed.
- `return_block_to_free_list`: Returns a block of memory to a sorted free list, rounding the size up to a multiple of four bytes with a minimum of eight.
- `call_with_error_check`: Calls a routine and returns minus one when it returns zero.
- `call_with_parameter_error_check`: Calls a routine and returns minus one when it returns zero.
- `allocate_and_link_node`: Allocates a node through a memory routine and, when that succeeds, stores a size word and links the node into a list, updating its head and tail pointers.
- `allocate_block_from_free_list`: Searches a free list for a block of at least the requested size, rounded to a multiple of four bytes with a minimum of eight.
- `flush_open_handle_list`: Walks a list of open entries, calling a routine for each entry that has a particular flag state and a changed value, then calls one further routine.
- `write_with_error_code`: Clears the error code, calls a stored hook first if one is set, then writes through a DOS function.
- `seek_within_file`: Clears the error code and seeks in an open file, recording the operating-system error code when the seek fails.
- `read_from_file_with_error`: Clears the error code, reads from an open file through a DOS call, and on failure records the operating-system error code and sets a general error code of five.
- `open_file_with_error`: Clears the error code, opens a file through a DOS call, and on failure records the operating-system error code with a general error code of two and returns minus one.
- `probe_lock_then_open`: Attempts to lock a name; when the lock succeeds it releases it and returns minus one.
- `delete_locked_file_then_open`: Attempts to lock a name; when the lock succeeds it releases the lock and deletes the file.
- `close_file_handle`: Calls a stored hook when one is set, closes the file through a DOS call, and returns zero.
- `compute_fixed_point_product`: Multiplies two 32-bit fixed-point values held in registers, combining the high-half and low-half 16-bit products into a 32-bit result.
- `divide_with_sign_dispatch`: Divides one signed 32-bit value by another by handling the signs of both inputs and delegating the magnitude work to routines.
- `normalize_fixed_point_divide`: Divides a 32-bit dividend by a 32-bit divisor using a normalised fixed-point method.
- `copy_bounded_line_and_prompt`: Copies up to seventy-nine characters from a stored string into a local buffer and terminates it.
- `copy_line_and_check_break`: Copies a null-terminated string into a local buffer and terminates it.
- `acquire_handle_and_setup`: Calls a routine to obtain a value, stores it in a local field and passes it with fixed arguments to a routine.
- `setup_with_local_buffer`: Passes a local buffer and fixed arguments to a routine and returns zero when the routine reports success, otherwise minus one.
- `return_zero_unreliable_body`: The decompiled body is only a return of zero, although the listing has seventy-seven instructions and several library calls, so this routine's behaviour is not reliably recovered.
- `lookup_slot_entry_by_index`: Returns the address of a slot entry for a valid index whose entry is in use.
- `check_drive_status_callback`: Reads a status word through a library call and, when a particular status bit pair is set and a callback is registered, runs the callback.
- `process_expired_pending_entries`: Walks a table of pending entries and, for each entry not yet marked, compares a timer with a threshold of twenty thousand units.
- `process_entry_list_paths`: Loops over a list of entries and, for each unmarked entry, builds a path from stored name fields and calls routines that format and record it before marking the entry done.
- `send_request_and_wait_reply`: Allocates a message structure, fills its fields, sends it through the operating-system message calls and waits for the reply.
- `iterate_open_entries`: Traverses a linked list of entries and calls another routine for each entry that meets specific bitwise flag criteria and has a non-matching value.
- `release_slot_entry_if_ready`: Looks up an entry and returns minus one when it is absent.
- `emit_string_to_bounded_buffer_2`: Writes a string into a buffer while a remaining-length counter allows, calling a routine for characters past the limit, and returns the number of characters in the string.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `stub_undecoded_single_instruction`: A single-instruction routine called from one place.
- `build_device_path_name`: Builds a path in a buffer: it appends a colon when a device flag is set and the colon is missing, appends a separator when the name does not already end in one, and appends a period followed by the result of a routine when a suffix flag is set.
- `clear_slot_entry`: Releases a slot entry: when its pointer is set it calls a routine, then marks the entry's flag byte and value word as cleared and calls two further routines.
- `execute_with_error_check`: Calls another routine and returns a specific value if the result is zero or if a global variable is non-zero.
- `call_handle_with_mode_flag`: Obtains a handle through a routine and returns minus one when it is zero.
- `fill_info_block_from_flags`: Fills a file information block from a set of option bits and stored fields, calling routines to obtain values.
- `allocate_slot_and_store_result`: Searches a slot table from its fourth entry for a free slot and records an error if none is free.
- `format_string_with_arguments`: Interprets a format string with conversions for decimal numbers, pointer-like values, strings and fixed-width digit output, writing the characters through a routine.
- `append_string_unbounded`: Appends a null-terminated source string to the end of a destination string, including the terminator.
- `create_queue_node`: Requests a value from a routine and, when it is valid, allocates a node, stores the given pointer and a flag byte, and links it into a list.
- `multiply_fixed_point`: Performs a 32-bit fixed-point multiplication using the high and low 16-bit components of two input values.
- `read_line_into_buffer`: Reads bytes from an input stream into a buffer until a newline or end-of-input marker is reached, then terminates the buffer.
- `undecoded_routine_ten_instructions`: A ten-instruction routine referenced by address and called from two places.
