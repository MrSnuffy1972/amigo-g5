# Game 2109 (ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02109). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (921232 top-level bytes in 51 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 133 of 133 named (0 author symbols), 133 of 133 described (10 by rule, 15 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2076 of 2076 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5275 of 5275 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 112481 |
| code | 16060 |
| data | 771247 |
| padding | 12228 |
| unused | 9216 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:installu6 | 2 | 267 | 8 |
| endcli | 2 | 117 | 2 |
| Disk-Validator | 1 | 386 | 1 |
| Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:shut.txt | 2 | 267 | 8 |
| Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:shut.exe | 1 | 86 | 5 |
| Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 378 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 689 | 27 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3049 | 63 |

## Cases handled

- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:INTRO_3.NEW: high-entropy data file (7.79 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:intro.new: high-entropy data file (7.83 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:mainmenu.new: high-entropy data file (7.64 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:paper.bmp: high-entropy data file (7.93 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:ULTIMA_Ba.MOD: high-entropy data file (7.79 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:INTRO_1.NEW: high-entropy data file (7.85 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:ULTIMA_Ga.MOD: high-entropy data file (7.80 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:ULTIMA_Fa.MOD: high-entropy data file (7.91 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:u: high-entropy data file (7.76 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:ultcli: high-entropy data file (7.68 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:intro_1a.new: high-entropy data file (7.79 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:ULTIMA_Da.MOD: high-entropy data file (7.84 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:intro_2.new: high-entropy data file (7.65 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:gypsy.new: high-entropy data file (7.83 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:Ultima6: high-entropy data file (7.68 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:ULTIMA_Aa.MOD: high-entropy data file (7.83 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:game: high-entropy data file (7.80 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:vellum1.new: high-entropy data file (7.83 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:intro_2a.new: high-entropy data file (7.90 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:title.new: high-entropy data file (7.77 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:ULTIMA_Ea.MOD: high-entropy data file (7.72 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:intro_1b.new: high-entropy data file (7.75 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:blocks.new: high-entropy data file (7.90 bits/byte), classified as packed data
- runtime segment an address: 294 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 488 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 24 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 686 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)[a2]/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:installu6: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:installu6

- `load_hunk_sections_with_relocations`: Reads AmigaDOS load-file records, allocates memory for each code and data section, copies the section contents, applies 32-bit relocation entries, and stops at the end marker.
- `count_down_wait_loop`: Counts a 16-bit register down to zero in a tight loop and returns, so it waits a fixed number of iterations.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_backward_run_data`: Decodes a packed stream backwards from the end of a buffer, writing literal bytes and copying earlier runs of output.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `extract_bits_from_stream`: Reads bits one at a time from a stream of longwords into an accumulated value over a fixed bit count, refilling from memory when the buffer empties.
- `stub_bad_instruction_data`: A four-instruction fragment that makes a trap call, then runs into bad instruction data.

### endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_then_call_pointer`: Copies a block of fifteen longwords from a table at a fixed offset into a caller-supplied buffer when the buffer pointer is nonzero, then calls a routine through a stored function pointer.

### Disk-Validator

- `scan_index_table_bit_masks`: A 386-instruction program entry whose decompiled output is cut short.

### Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:shut.txt

- `unpack_amiga_hunk_data`: Decodes a compressed data stream by identifying specific hunk type codes and allocating memory to reconstruct the original data.
- `decrement_and_loop`: Decrements a 16-bit value in a register until it reaches zero.
- `return_only_stub`: A single return instruction that does nothing and is reached only through its address being taken.
- `copy_memory_blocks`: Performs a series of memory copies and byte transfers between memory locations based on values read from a pointer.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `extract_bit_run_into_accumulator`: Reads a counted run of bits from a stream of longwords into an accumulator, tracking the carried comparison result and refilling the bit buffer from memory when it empties.
- `extract_bit_stream`: Extracts bits from a source buffer using a bit count and shifts them into an accumulated value, refilling from memory when the buffer is exhausted.
- `trigger_software_interrupt`: Executes a software interrupt trap.

### Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:shut.exe

- `install_pointer_slots_and_call`: Stores the address of one global into four pointer slots and calls two routines, touching the custom chip register block as well.
- `unpack_backward_store_end`: Decodes a packed stream backwards like its sibling routine, and additionally saves the final output position into a word at a second pointer.
- `test_upper_bits_no_effect`: Tests whether a register has any bits above its lowest bit, then returns in both cases.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `shift_bits_out_of_register`: Shifts a bit register one bit at a time for a counted number of steps, refilling it from memory with the carried-in bit when it empties.

### Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install)[a2].adf:bootblock

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
- `nop_instruction`: This routine is empty and performs no operations.
- `store_word_into_record`: Writes the low word of the data register into a fixed field of a structure addressed by the address register, then returns.
- `decode_channel_command_word`: Decodes a packed 32-bit command word into step and position values, stores the previous values in a per-channel record, and clears a run of bytes in that record.
- `copy_indexed_table_entry`: Looks up a long word in a global table using an index held in a record.
- `copy_block_from_table`: Copies a block of long words from a global table, selected by an index in a record, into a destination buffer four long words per loop pass.
- `load_table_entry_into_record`: Selects a 152-byte entry from a global table using a one-based index and copies several of its fields into the caller's record, scaling one field by four.
- `copy_entry_fields_to_record`: Copies the same set of fields from a source record into a target record, scaling one field by four and clearing several state words.
- `clear_record_status_fields`: Sets four state words and one flag byte to zero.
- `update_four_audio_channels`: Calls four channel routines in turn and references the audio location registers for all four Amiga channels.
- `update_channel_period_tick`: Advances one sound channel by one tick: applies slide and offset steps taken from lookup tables, clamps the result to a minimum of 113, and writes it and a sample pointer into the output record.
- `run_channel_effect_command`: Counts down a timer in a channel record and, when it expires, reloads it and runs one effect command chosen by a type field.
- `execute_indirect_jump`: Performs an indirect jump to an address stored in a table.
- `negate_byte_and_wrap`: Negates one byte in a buffer addressed by the record, advances a stored offset, and resets the offset to its start field when it passes the end limit.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_minus_one_status`: Returns the 32-bit value minus one and performs no other work.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `close_library_conditionally`: Calls a close routine and, when the zero flag is clear, calls a second routine and then invokes a library function through a vector loaded from a global base pointer.
- `open_dos_library`: Calls the exec library function OldOpenLibrary to open dos.library and stores the returned library handle in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_dos_and_port`: Clears a block of working memory, opens the DOS library, allocates memory and takes the current-directory lock.
- `release_dos_resources`: Optionally runs a callback, closes DOS handles and locks, frees memory, closes a library and replies to a message.
- `release_linked_list_nodes`: Walks a linked list of nodes, calls a routine through a stored pointer for each node to release it, then clears the list head and tail pointers.
- `parse_quoted_string_tokens`: Splits a text buffer into tokens and skips blanks, tabs and newlines between them.
- `call_library_function_pointer`: Loads a library base from a stack parameter and calls the function at a fixed negative vector offset from it.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `do_nothing_return`: It performs no operations and returns immediately without touching any data or registers.
- `validate_lookup_result_status`: Calls a lookup routine and returns minus one when it yields nothing.
- `copy_text_with_space_limit`: Copies a text string into an output buffer one character at a time while decrementing a remaining-space counter, calling a flush routine when the space runs out.
- `parse_device_and_path`: Reads characters from a buffer, converting lowercase letters to uppercase when a flag bit is set, and checks for colon and slash separators.
- `prepare_stream_buffer`: Obtains a new buffer through a routine when the stream has none or a reset flag is set.
- `write_byte_to_stream`: Writes one byte to a buffered stream unless the stream is in an error or flushing state.
- `read_byte_from_stream_entry`: Returns the next byte from a buffered input stream by entering the shared read routine at its start.
- `read_byte_from_stream`: Returns the next byte from a buffered input stream, refilling the buffer through a routine when it empties.
- `process_entries_in_reverse`: Walks a table of eight-byte entries from the last entry back to the first.
- `copy_three_byte_fields`: Copies a three-byte header from a source buffer, or from a default when none is set, into a fixed record.
- `append_limited_string`: Appends characters from a source string to the end of a destination string, copying at most the limit given in the data register.
- `copy_string_return_end`: Copies a zero-terminated string to a destination and returns a pointer to the position of the terminator, so callers can append after it.
- `format_signed_decimal`: Converts a signed number to decimal text: it writes a minus sign when the value is negative, collects the digits in a temporary buffer, then copies them back in reverse order and terminates the string.
- `parse_signed_decimal`: Reads an optional sign and then decimal digits from a string, accumulates the value and stores it through a pointer.
- `insert_block_into_free_list`: Returns a memory block to a sorted free list.
- `chain_calls_with_check`: Calls one routine and, if it returns nonzero, calls a second routine and returns minus one when a field in the context block is set.
- `forward_argument_with_check`: Passes its argument to a routine after a first check succeeds, then returns minus one when a field in the context block is set.
- `allocate_linked_list_node`: Allocates a node of the requested size plus twelve bytes through a library memory call, stores the size and a link field in its header, and appends it to a linked list.
- `allocate_block_from_free_list`: Allocates a block of at least the requested size from a sorted free list, rounding the size up to a multiple of four with a minimum of eight.
- `process_pending_list_entries`: Walks a linked list of entries and calls a routine on each entry that has one flag bit set, another flag bit clear, and differing stored values.
- `dos_call_with_error_code`: Clears the error state and calls a DOS vector function.
- `seek_file_with_error_capture`: Clears the error state and calls a DOS seek function.
- `read_file_with_error_capture`: Clears the error state and calls a DOS read function.
- `open_file_with_error_capture`: Clears the error state and calls a DOS open function through its vector.
- `open_file_if_unlocked`: Calls a lock function on a name first.
- `release_and_delete_file`: Releases a lock on a name when one is present, then deletes the file and calls the delete routine.
- `close_file_handle`: Calls a routine when a pending-state field is nonzero, then calls the DOS close function through its vector and returns zero.
- `multiply_fixed_point_values`: Multiplies two 32-bit fixed-point values from data registers by summing the cross products of their 16-bit halves, shifting them into the high word, and adding the low-by-low product.
- `divide_signed_32bit_values`: Divides one 32-bit signed value by another.
- `divide_with_quotient_and_remainder`: Divides a 32-bit dividend by a 32-bit divisor using normalised shift-and-subtract steps.
- `copy_string_and_print_line`: Copies up to seventy-nine characters from a length-prefixed string into a local buffer and terminates it.
- `echo_line_with_break_notice`: Copies an input string into a local buffer and terminates it.
- `check_intuition_action_result`: Looks up a library entry and passes it with two offset arguments and the values 250 and 60 to a routine.
- `set_context_and_run_request`: Stores a pointer into a context field, calls a routine with two offsets and the values 250 and 60, and returns zero only when the routine returns one.
- `stub_returns_zero`: The decompiled body contains only a return of zero.
- `get_table_entry_pointer`: Returns the address of an entry in an eight-byte-stride table when the index is in range and the entry is in use.
- `check_disk_change_status`: Reads a status word through a library vector and, when either of two status bits is set and a registered callback exists, invokes the callback.
- `poll_entries_with_timeout`: Loops over a table of entries that have a use flag set.
- `scan_entries_and_free`: Iterates a list of entries and, for each entry in use, copies a name into a stack buffer and calls several routines.
- `send_port_message_and_wait`: Allocates a message structure, fills in its node links and two data fields, and sends it through a message port.
- `process_linked_list`: walks a linked list and calls another routine for each entry where a specific bit is clear, another bit is set, and a stored value differs from a comparison value.
- `clear_entry_and_check_error`: Looks up an entry and returns minus one when none exists.
- `copy_string_with_limit`: copies characters from a source string to a destination buffer while decrementing a remaining space counter and calling another routine if the limit is reached.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `stub_no_pseudocode`: The decompiler produced no pseudo-code for this routine, so its behaviour cannot be recovered.
- `append_path_separators`: Appends a colon to a name when one is missing and the flag requests it.
- `clear_and_release_request_block`: Calls a release routine on an attached pointer when one is present, sets two state fields to minus one, and then calls two library functions through the global vector, one of which releases memory.
- `execute_chained_calls`: calls one routine and then another if the first returns zero, returning a specific value if a field in a global variable is non-zero.
- `execute_conditional_calls`: calls a routine and then conditionally calls another routine based on a bit flag in the first routine's result, returning a specific value if a global variable is non-zero.
- `build_request_record_flags`: Looks up a request in a table and, when it can be created, sets a size field of the output record to 1024 or 2048 depending on a sign test of a stack value.
- `claim_free_table_slot`: Searches an eight-byte-stride table from its fourth entry onward for a free slot.
- `expand_format_string`: Copies text from a format string into the output buffer and, when it meets a percent sign, processes a conversion specifier from the stack arguments.
- `append_string_unbounded`: Advances to the terminator of a destination string and copies a source string onto its end, including its terminator.
- `queue_input_character_node`: Reads one character through an input vector and, when the read succeeds, allocates a node and stores the character and a pointer in it.
- `multiply_fixed_point`: performs a 32-bit fixed-point multiplication using the cross products of 16-bit halves.
- `read_line_into_buffer`: Copies characters from a source buffer into a destination until it reads a newline or the end marker, replacing the newline with a zero terminator.
- `stub_no_pseudocode_2`: The decompiler timed out and produced no pseudo-code, so the behaviour cannot be recovered.
