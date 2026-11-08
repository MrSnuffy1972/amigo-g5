# Game 2088 (secret of monkey island, the (1990)(lucasfilm games)(disk 1 of 4)[cr skr][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02088). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (975184 top-level bytes in 41 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 25 of 25 units identical (bytes and relocations), 19 raw-word lines |
| Functions named and described | pass, 668 of 668 named (0 author symbols), 668 of 668 described (45 by rule, 521 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 6091 of 6091 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 46704 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 18236 of 18241 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 236541 |
| code | 55516 |
| data | 667779 |
| padding | 9204 |
| unused | 6144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Secret of Monkey Island, The (1990)(LucasFilm Games)(Disk 1 of 4)[cr SKR][a].adf:LoadSeq | 1 | 20 | 2 |
| Run | 2 | 117 | 2 |
| Delete | 2 | 117 | 2 |
| Assign | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| Lab | 1 | 2 | 1 |
| Skip | 2 | 117 | 2 |
| Execute | 2 | 117 | 2 |
| Copy | 2 | 117 | 2 |
| Makedir | 2 | 117 | 2 |
| CD | 2 | 117 | 2 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| Secret of Monkey Island, The (1990)(LucasFilm Games)(Disk 1 of 4)[cr SKR][a].adf:Main | 2 | 233 | 8 |
| disk-validator | 1 | 386 | 1 |
| SkidRow | 2 | 233 | 8 |
| icon.library | 10 | 1375 | 109 |
| Secret of Monkey Island, The (1990)(LucasFilm Games)(Disk 1 of 4)[cr SKR][a].adf:IconX | 12 | 806 | 37 |
| Secret of Monkey Island, The (1990)(LucasFilm Games)(Disk 1 of 4)[cr SKR][a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 32 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7452 | 235 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 777 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5741 | 218 |
| music.dat (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- secret of monkey island, the (1990)(lucasfilm games)(disk 1 of 4)[cr skr][a]/Secret of Monkey Island, The (1990)(LucasFilm Games)(Disk 1 of 4)[cr SKR][a].adf: file Rooms/disk01.lec (header 411115 bytes, blocks hold 27328): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 20 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2211 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 257 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 3148 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- secret of monkey island, the (1990)(lucasfilm games)(disk 1 of 4)[cr skr][a]/Secret of Monkey Island, The (1990)(LucasFilm Games)(Disk 1 of 4)[cr SKR][a].adf:Rooms/music.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- secret of monkey island, the (1990)(lucasfilm games)(disk 1 of 4)[cr skr][a]/Secret of Monkey Island, The (1990)(LucasFilm Games)(Disk 1 of 4)[cr SKR][a].adf: blocks 2-54 (bytes 1024-28160) outside the filesystem hold data, not code (opcode density 0.017, entropy 6.70)
- secret of monkey island, the (1990)(lucasfilm games)(disk 1 of 4)[cr skr][a]/Secret of Monkey Island, The (1990)(LucasFilm Games)(Disk 1 of 4)[cr SKR][a].adf: blocks 63-65 (bytes 32256-33792) outside the filesystem hold data, not code (opcode density 0.020, entropy 6.05)
- secret of monkey island, the (1990)(lucasfilm games)(disk 1 of 4)[cr skr][a]/Secret of Monkey Island, The (1990)(LucasFilm Games)(Disk 1 of 4)[cr SKR][a].adf: blocks 958-958 (bytes 490496-491008) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.80)
- secret of monkey island, the (1990)(lucasfilm games)(disk 1 of 4)[cr skr][a]/Secret of Monkey Island, The (1990)(LucasFilm Games)(Disk 1 of 4)[cr SKR][a].adf: blocks 970-974 (bytes 496640-499200) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.84)
- secret of monkey island, the (1990)(lucasfilm games)(disk 1 of 4)[cr skr][a]/Secret of Monkey Island, The (1990)(LucasFilm Games)(Disk 1 of 4)[cr SKR][a].adf: blocks 979-1759 (bytes 501248-901120) outside the filesystem hold data, not code (opcode density 0.015, entropy 6.85)
- secret of monkey island, the (1990)(lucasfilm games)(disk 1 of 4)[cr skr][a]/Secret of Monkey Island, The (1990)(LucasFilm Games)(Disk 1 of 4)[cr SKR][a].adf:Main: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 46704 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Secret of Monkey Island, The (1990)(LucasFilm Games)(Disk 1 of 4)[cr SKR][a].adf:LoadSeq

- `copy_code_block_low_memory`: Copies a fixed block of code longwords from the routine's own image into a fixed low-memory location, then returns zero.
- `write_marker_when_arguments_valid`: If neither of two arguments is minus one, writes a fixed marker longword just below the stack argument; otherwise it returns without writing.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_and_invoke_routine`: If its pointer argument is non-null, copies sixteen longwords from a fixed offset of a base-register table into it, then calls a routine through a saved pointer.

### Delete

- `build_command_packet_buffer`: Finds the largest argument size in a linked list, allocates a buffer sized from it, fills it with a marker sequence and a copy of a template structure, then calls a routine to attach a handler; returns zero on success and minus one when allocation fails.
- `copy_memory_block_3`: Copies a sequence of words from a source address to a destination address and then calls another routine.

### Assign

- `update_linked_list_buffer_2`: Traverses a linked list to find the maximum value in a specific field and then expands a data buffer by writing a header and copying data from a source table.
- `copy_memory_block_and_call`: Copies a fixed number of words from a memory location to a destination buffer and then executes a function pointer.

### Echo

- `update_linked_list_and_copy_data_2`: Traverses a linked list to find the maximum value in a specific field, then expands a data structure by writing a header and a sequence of incrementing values.
- `copy_sixteen_longwords_to_buffer`: When a destination pointer is non-null, copies sixteen longwords from a fixed offset in the data area into that destination, then makes an indirect call through a saved routine pointer.

### Lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Skip

- `build_marked_entry_table`: Scans a linked list of entries for the largest value held in a field, then allocates a table sized from that value and fills it with a running marker pattern.
- `copy_memory_block_4`: Copies a fixed number of words from a source address to a destination address.

### Execute

- `update_linked_list_and_buffer`: Traverses a linked list to find the maximum value in a specific field and then updates a buffer with a new header and a sequence of incrementing values.
- `copy_memory_block_and_call_2`: Copies a fixed number of words from a memory location to a destination address and then calls another routine.

### Copy

- `update_linked_list_and_copy_data_3`: Traverses a linked list to find the maximum value in a specific field, then writes a header and populates a buffer with incrementing values.
- `copy_memory_block_5`: Copies sixteen bytes of data from a source location to a destination location via a loop.

### Makedir

- `allocate_and_initialize_buffer`: Traverses a linked list to determine a maximum value, then allocates a memory block and populates it with a sequence of incrementing values.
- `copy_data_block`: Copies a fixed number of words from a source memory location to a destination memory location.

### CD

- `update_linked_list_and_copy_data`: Traverses a linked list to find the maximum value in a specific field, then updates a header and performs a series of memory copies between a table and a destination buffer.
- `copy_memory_block`: Copies sixteen bytes of data from a memory location offset from a pointer to a destination address.

### EndIf

- `return_zero_status`: Contains only a return of zero, with no other work.

### If

- `update_linked_list_buffer`: Traverses a linked list to find the maximum value in a specific field, then expands a data buffer by writing a header and copying a block of data.
- `copy_memory_block_2`: Copies a fixed number of words from a memory location to a destination address.

### Secret of Monkey Island, The (1990)(LucasFilm Games)(Disk 1 of 4)[cr SKR][a].adf:Main

- `load_executable_hunks`: Walks the hunk list of an executable image, allocating memory for each section through the exec library and copying its data in.
- `busy_wait_countdown`: Counts a 16-bit register down from its starting value until it wraps past zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_backward_packed_data`: Decodes a packed byte stream from the end of a buffer toward its start.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bit_stream_with_carry`: Shifts a multi-word bit register one bit per step, pulling bits in from the next input word when the current word is exhausted and carrying the shifted-out bit forward.
- `stub_bad_instruction_data`: This block is bad instruction data rather than real code, so no behaviour can be recovered from it.

### disk-validator

- `check_disk_track_bitmaps`: Walks the track bitmaps of a disk, testing one bit in each of two stored words per position and counting every position where the two results differ.

### SkidRow

- `reconstruct_amiga_hunk_data`: Parses a buffer containing AmigaDOS hunk type codes to allocate memory and relocate data.
- `decrement_and_loop`: Decrements a 16-bit value in a register until it reaches zero.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `process_memory_blocks`: Iterates through memory using a pointer and performs conditional data movement and copying based on values read from memory.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `shift_bit_stream_with_carry_2`: Computes a count from the difference of two register values, then shifts a multi-word bit register one bit per step.
- `unpack_bit_stream`: Decodes a bitstream by extracting variable-length bit sequences and updating a bit register and a status flag.
- `execute_trap`: Triggers a processor trap instruction.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code that adds one to a 16-bit value at a fixed offset from a base pointer and clears bit three of a byte at another offset from the same pointer.
- `decrement_library_refcount`: Library code that subtracts one from a 16-bit counter at a fixed offset from a base pointer.
- `unlink_node_or_set_pending`: Library code that, when a counter at a fixed offset is zero, removes a node from a doubly linked list by relinking its neighbours and then calls a library function through the library base.
- `set_flag_bit_return_zero`: Library code that sets bit three of a byte at a fixed offset from a base pointer and returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `open_library_or_alert`: Library code that stores two values in a structure, calls a library function through the base to open a library by name (dos.library appears in its text), and records the returned handle there.
- `copy_two_values_to_globals`: Library code that copies two 32-bit values from fixed offsets of a structure addressed by a register into two global cells, then returns.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `jump_to_library_handler`: Library code that transfers control straight to another routine, which then does the work.
- `call_library_handler`: Library code that calls one other routine and then returns, with no work of its own.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `invoke_icon_loader`: This library routine calls a helper routine to load icon data into an allocated buffer and then returns.
- `call_library_handler_2`: This library routine simply calls another routine and then returns.
- `call_library_handler_3`: This library routine calls another routine and then returns.
- `initialize_icon_record`: Library code that allocates a 168-byte record through a library call, links it to a sub-structure, and calls other routines to initialise it.
- `trigger_icon_initialization`: This library routine calls a single initialization function to set up icon data structures and then returns.
- `call_handler_pointer`: This library routine fetches a function pointer from a data structure and calls it.
- `execute_library_handler`: This library routine calls another routine via a pointer and then returns.
- `call_other_routine`: The routine calls another routine and then returns.
- `call_find_segment_routine`: This routine calls another routine and then returns.
- `call_format_copy_count`: This library routine calls a routine that parses an optional decimal copy count from a text field and then returns.
- `store_value_into_returned_block`: Library code that calls a routine through a function pointer table in the base, passing the first argument, then stores the second argument at a fixed offset in the pointer that routine returns.
- `chain_calls_release_on_failure`: Library code that calls one routine and, if it returns a non-zero handle, passes that handle to a second routine with the caller's arguments.
- `fill_record_from_local_buffer`: Library code that obtains a record from one routine, asks a second routine to fill a local buffer, and copies its eleven longwords and several fields into the record.
- `restore_record_fields_from_frame`: Library code that copies eleven longwords and several fields from a frame on the stack back into a record.
- `return_zero_stub`: Empty library routine that returns zero without doing any work.
- `copy_record_to_stack_header`: Library code that builds a small header on the stack holding two constants, copies eleven longwords and several fields from a record into it, and then calls another routine with the first two arguments and a pointer to that stack header.
- `load_icon_info_from_file`: Library code that reads an icon file header through library calls, checks it against a fixed magic word and version value, and then reads its fields and any optional data blocks into memory it allocates.
- `read_icon_header_fields`: Reads the fields of an icon header that has already been located.
- `read_optional_icon_data_blocks`: Library code that reads the remaining icon fields, including optional pointer lists and an extra image record, allocating memory for each.
- `read_longword_list_into_array`: Library code that reads a list of longwords into an array until a counter in the frame runs out, then reads one more field into the structure.
- `call_then_return_zero`: Library code that calls one other routine and then returns zero.
- `return_immediately`: Contains only a return, with no other work.
- `load_icon_with_allocated_buffer`: Library code that allocates a 260-byte path buffer, reads an icon header and its fields into the caller's record through library calls, and reads the optional data blocks.
- `write_icon_info_to_file`: Library code that writes an icon's header and fields through library calls, measuring each optional string before writing it, then releases its handle.
- `write_optional_icon_data_blocks`: Library code that writes the optional pointer lists and extra fields of an icon record, measuring each string first, then restores a saved value in the record.
- `write_string_list_to_file`: Library code that measures a null-terminated list of pointers, then writes each entry with its measured length and one optional extra entry.
- `write_pointer_table_entries`: Library code that walks a null-terminated table of pointers, measuring and writing each entry through library calls, then writes one optional extra entry.
- `walk_node_list_to_limit`: Walks a chain of nodes that starts at an offset in a structure, calling another routine on each node until the chain ends or reaches a stored limit.
- `iterate_linked_list_nodes`: Walks a linked list, calling another routine on each node and fetching the next node through another routine, until the chain ends or wraps back to its starting node.
- `call_with_offset_argument`: Library code that calls another routine with the same first argument and the second argument increased by a fixed amount, then returns.
- `allocate_icon_record`: Library code that allocates a 168-byte record through a library call, links it to a sub-structure, and calls other routines to initialise it.
- `decrement_and_refill_counter`: This library routine decrements a 16-bit counter and calls a refill routine if the counter reaches zero.
- `return_zero`: Library code that returns zero.
- `refill_counter_and_store_pair`: Decrements a 16-bit counter behind a pointer and, when the counter runs out, calls a refill routine and returns zero if that fails.
- `decrement_counter_and_store_pair`: Decrements a 16-bit counter behind a pointer, then stores two values from the arguments into the table entry selected by the counter.
- `allocate_small_icon_block`: Library code that allocates a 94-byte block through a library call, initialises its fields, and calls other routines to attach it to the caller's structure.
- `call_and_return_zero`: This library routine calls another routine and then returns zero.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `incremental_recursive_call`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `find_value_after_equals_sign`: Searches a null-terminated list of pointers for an entry whose name matches the key, then returns the text after the equals sign in that entry.
- `find_value_after_equals_sign_2`: Scans a null-terminated list of strings for an entry in which an equals sign follows the name, and returns the text after it.
- `find_value_after_equals_sign_3`: Scans a null-terminated list of pointers for an entry whose name matches, and returns the text after the equals sign.
- `find_segment_in_separated_list`: Scans a string whose entries are separated by a separator character, and returns one when an entry has the same length as the key and matches it.
- `find_segment_in_separated_list_2`: Scans a string whose entries are separated by a separator character, and returns one when an entry has the key's length and matches it.
- `find_segment_in_separated_list_3`: Scans a string whose entries are separated by a separator character, and returns one when an entry matches the key, or zero when none matches.
- `search_delimited_list`: This library routine scans a list for a specific value by iterating through segments and checking lengths.
- `search_separated_string`: Library code that scans a string for a specific entry by checking lengths and content within a delimited list.
- `format_copy_count_label`: Parses an optional decimal copy count from a text field, defaulting to one when none is present.
- `parse_copy_count_from_text`: Reads the decimal digits that follow a fixed prefix in a text field and stores the count it finds.
- `parse_decimal_count_suffix`: Reads a run of decimal digits from a string and checks that a fixed suffix follows them.
- `parse_decimal_count_suffix_2`: Reads a run of decimal digits from a string and checks that a fixed suffix follows them, writing the count when it does.
- `convert_digit_char_to_value`: Returns the numeric value of a character when it is a decimal digit, or minus one for any other character.
- `return_minus_one`: Returns minus one with no other work.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `allocate_icon_work_block`: Library code that allocates a 96-byte block, sets its first word to ten and stores the same value in a caller's word, then calls another routine to initialise the block.
- `return_zero_constant`: This library routine returns zero.
- `check_call_result_matches_value`: Library code that calls one routine with three arguments and returns the value it sends back when that value equals the fourth argument, or zero otherwise.
- `tiny_move_rts`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `forward_library_call`: This library routine calls another routine with three arguments and returns zero if the result does not match a fourth argument.
- `return_void_constant`: This library routine returns without performing any operations.
- `divide_longword_with_remainder`: Divides a 32-bit value by a divisor, using native division when the divisor fits in 16 bits and a 16-step shift-and-subtract loop when it does not.
- `shift_subtract_division_steps`: Performs sixteen shift-and-subtract steps, shifting the dividend left one bit at a time and subtracting the divisor from the upper half when it fits, so the quotient bits collect in the lower half.
- `multiply_32_bit_integers`: Computes the product of two 32-bit register values using 16-bit partial products and returns it.
- `call_rawdofmt_formatter`: Calls the exec library's formatted text routine through the library base stored in a global cell, with its arguments already in the registers, and then returns.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `measure_string_length`: Returns the length of a null-terminated string by counting characters until the zero byte is reached.
- `find_char_in_string`: Scans a null-terminated string for a given character and returns a pointer to its first occurrence, or null when it is not found.
- `return_zero_2`: This library routine is empty and simply returns zero to the caller.
- `copy_c_string`: Copies bytes from a source string into a destination buffer, including the terminating zero byte.
- `append_c_string`: Finds the terminating zero byte of the destination string and copies the source string, including its own terminator, onto the end.
- `compare_strings_up_to_count`: Compares two strings for up to a given number of characters.
- `return_one_status`: Contains only a return of one, with no other work.
- `scan_string_up_to_limit`: Walks a string for up to a given number of characters, calling another routine when it reaches the terminating zero byte before the limit runs out.
- `call_another_routine`: Library code that calls another routine and then returns.
- `copy_string_with_zero_padding`: Copies up to a given number of characters from a source string into a buffer.
- `return_from_subroutine_2`: This routine is an empty function that performs no operations and immediately returns to its caller.
- `initialise_self_linked_header`: Sets up a two-longword list header in place: its first longword points four bytes past its own start, the next longword is cleared, and the third longword points back at the header.
- `validate_request_and_forward`: Returns one when the request pointer is null.
- `walk_entry_list_applying_helpers`: Walks a linked list of entries, applying two other routines to each one.
- `check_entry_list_all_succeed`: Walks a linked list of entries, calling two other routines on each one, and returns one when every entry passes and the list ends.
- `return_zero_constant_2`: This library routine returns a zero value to the caller.
- `allocate_entry_with_data`: Allocates a 20-byte cleared entry, fills in a data block obtained from another routine, and attaches it to the entry's pointer field.
- `return_negative_one`: Library code that returns a value of all ones.
- `build_entry_list_with_blocks`: Builds a linked list of 20-byte entries, allocating a data block for each through other routines and linking each new entry to the previous one.
- `build_entry_chain_with_blocks`: Allocates entries one after another, attaching a data block to each and linking each new entry to the previous one.
- `build_linked_list`: This library routine iteratively builds a linked list by calling other routines to allocate and link data blocks.
- `return_all_ones`: Returns a value consisting of all bits set to one.

### Secret of Monkey Island, The (1990)(LucasFilm Games)(Disk 1 of 4)[cr SKR][a].adf:IconX

- `start_iconx_program`: Opens the libraries, allocates a state block and a message port, and splits the command line into arguments, handling quoted strings and star escapes for special characters.
- `release_iconx_resources_and_reply`: Restores the state saved by the entry routine: it stores the parameter in the state block, closes the library and file handles it used, frees the memory, replies to the waiting message, and returns the saved parameter.
- `open_icon_window_and_process`: Opens the icon library and, when a window is needed, opens a console window with a fixed specification.
- `handle_iconx_request`: Processes one icon request by clearing a buffer, looking up a handler and asking it for a result.
- `initialize_icon_library_functions`: Processes a pointer through another routine, creates a library with the system's library-making call, builds its function table, and enables it.
- `parse_icon_path_string`: Walks a list of string nodes, appending the usable names into a 260-byte path buffer.
- `append_string_pair_to_buffer`: Copies one string into a destination buffer, appends a second string to it, and then passes the position after the combined text to another routine.
- `measure_string_length_2`: Returns the length of a null-terminated string by counting bytes until the zero byte is reached.
- `copy_null_terminated_string`: Copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `skip_string_end_then_call`: Advances a pointer to the terminating zero byte of a string, then calls another routine with that position and the original second argument.
- `parse_signed_decimal`: Parses a decimal integer from a string, accepting an optional plus or minus sign, and returns its signed value.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_with_stack_slot_pointer`: Calls another routine with the two arguments it received plus a pointer to a slot in its own stack frame, then returns.
- `forward_to_library_function`: Calls a library function through the library base stored in a global cell, with its arguments already in the registers, and then returns.
- `execute_indirect_function`: Loads a function pointer from a table and executes it.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `format_text_and_send_buffer`: Formats text into a 140-byte stack buffer through the exec library's formatting routine, measures the resulting string, and passes the buffer and its length to another routine when the length is non-zero.
- `store_byte_at_pointer`: Stores the low byte of the data register at the address held in an address register, then returns.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_via_pointer`: Stores a byte from a register into a memory location pointed to by another register.

### Secret of Monkey Island, The (1990)(LucasFilm Games)(Disk 1 of 4)[cr SKR][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_value`: Returns a constant value of all bits set to one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `close_library_if_loaded`: Calls another routine, and when a flag shows that a library handle is held, closes it through the close-library wrapper and a library function at a fixed offset from the base.
- `open_dos_library`: Opens the DOS library by its name through the exec library's old open call, and stores the returned handle in a global cell.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `modify_memory_values`: Subtracts a byte value from two different memory locations and performs a word-sized move.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_resource_environment`: Clears a memory block and initializes several global pointers and state variables.
- `handle_interrupt_reply`: The routine conditionally calls a function pointer from a structure, calls an empty stub, calls a function pointer from a global variable, calls another routine, and if two fields in a structure are non-zero it calls a function pointer from a global variable, calls another function pointer from a global variable, and calls another function pointer from a global variable.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `return_immediately_2`: This routine is empty and performs no operations before returning to its caller.
- `traverse_linked_list`: iterates through a linked list and executes a function pointer stored in a global variable for each element before clearing two values in a structure.
- `tokenize_string_content`: parses a string into tokens by skipping whitespace and handling quoted substrings, storing pointers to these tokens in an array before passing the results to other routines.
- `store_globals_and_set_flag`: Calls another routine twice and then returns.
- `call_clear_stored_pointers`: Calls a routine that clears several global variables by moving their contents to a global variable and then resets the original variables to zero.
- `set_handler_flag`: Calls another routine twice and then stores a short value into memory at an offset from a pointer held in a register.
- `clear_pointer_offset_word`: The routine calls another routine twice, then stores a zero value at a specific offset from a pointer derived from the first call.
- `get_pointer_offset_word`: The routine calls another routine and returns a word stored at a specific offset from the resulting pointer.
- `clear_multiple_globals`: This routine clears several global variables and memory locations to zero and sets a global variable to specific constant values.
- `clear_multiple_globals_2`: This routine resets a series of global variables and memory locations to zero and assigns specific constant values to a global variable.
- `initialize_state_and_clear_flags_2`: The routine stores the result of another routine into a global variable, clears two global variables, and calls another routine before returning.
- `initialize_memory_blocks_3`: Sets several global pointers based on values found within a memory block and performs a loop to zero out a series of memory segments.
- `assign_global_pointers`: Copies a global pointer to two other locations and updates a global variable with a specific constant value.
- `clear_stored_pointers`: Checks several global variables for non-zero values, moves their contents to a global variable, and resets the original variables to zero.
- `update_indexed_data_2`: The routine checks a byte in a table using an index derived from a register and, if zero, copies two words from another table into a global variable and clears a value at an offset.
- `jump_to_initialize_audio_and_script_st`: Jump stub: passes control straight on to initialize_audio_and_script_state.
- `clear_multiple_globals_3`: This routine performs a bulk reset of various global variables and memory locations to zero and sets a global variable to specific constant values.
- `branch_on_table_state`: Checks a value in a table using an index derived from a register and performs conditional calls to another routine based on that value and a global variable.
- `map_input_indices_to_pointers`: Iterates through a sequence of values retrieved from a source array and uses them as indices to look up and store corresponding memory addresses into a destination structure.
- `update_joystick_button_states`: Iterates through a series of input values from a source array and updates a corresponding structure.
- `update_indexed_data`: Checks a byte in a table using an index derived from a register and, if zero, copies two words from another table into a global variable and clears a value at an offset from that variable.
- `clear_audio_and_state_variables`: The routine writes zero to a set of global variables and to several memory locations, including audio volume registers and DMA and interrupt control registers, effectively resetting state.
- `initialize_audio_and_script_state`: Clears a set of global variables and audio volume registers, resets DMA and interrupt state, and then loads a block of eight 16-bit entries from a table pointed to by a global variable and a register-derived offset into RAM.
- `initialize_state_from_input`: The routine stores constants and the low word of an input value into global variables, computes an address using a table indexed by the high byte of the input and stores the result into a global variable, and clears two other global variables.
- `check_conditional_state`: The routine checks a value in a table and performs conditional calls to another routine based on the state of a global variable.
- `call_and_return_stack`: The routine calls another routine and returns the value that was on the stack.
- `update_conditional_table_entry_3`: Checks a value within a table and, if specific conditions are met, updates several entries in another table using data from the original source.
- `update_conditional_table_entry_2`: Evaluates a condition based on a value in a table and, if met, updates a table entry with a new value, a byte, and a short.
- `update_conditional_table_entry`: Evaluates conditions based on values in a structure and, if met, updates a table entry with a new value, a byte, and a short.
- `initialize_state_and_clear_flags`: The routine stores the result of another routine into a global variable, clears two global variables, and calls another routine before returning a concatenated value.
- `initialize_memory_block_2`: Sets three integer values at specific offsets from a pointer, updates a global pointer, and zeros out a series of memory blocks.
- `initialize_memory_block`: Sets three integer values at specific offsets from a pointer, updates a global pointer, and then uses a loop to zero out a series of memory blocks.
- `set_conditional_state_2`: Checks a bit in a structure and, if not set, updates a global pointer and several other values.
- `set_conditional_state`: Checks a bit in a structure and, if not set, writes a byte to a memory location; it also conditionally updates a pointer and several other values.
- `clear_audio_and_state_variables_2`: Writes zero to a set of global variables and to several memory locations, including audio volume registers and interrupt control registers.
- `initialize_engine_state_and_tables`: Clears global variables and tables, initializes game state constants, and processes dialogue text, coordinate offsets, and script state flags.
- `execute_multiple_callbacks`: Executes a sequence of various routines, including table dispatches and linked list traversals.
- `clear_specific_memory_locations`: Calls another routine and then writes zero to three different word-sized memory locations.
- `copy_registers_to_memory`: Copies the current contents of several data and address registers into a memory block pointed to by a parameter.
- `decrement_table_values`: Iterates through a table and subtracts a value from specific entries, marking them with a new status byte if they reach zero.
- `search_table_for_value`: Iterates through a table to find a matching value based on several criteria, including range checks and bitwise comparisons, and returns a processed character.
- `decrement_and_process_counter`: Decrements a byte in a memory location repeatedly while it is non-zero, calling another routine if a value in a secondary table is not negative.
- `process_dialogue_text_and_coordinates`: Iterates through a list of text elements to extract character data and associated coordinate offsets.
- `update_input_coordinates`: Calculates coordinate values by applying an offset to a table entry and performs clamping within a specific range.
- `return_register_value`: Returns the value currently held in a data register.
- `execute_recursive_logic`: Performs a recursive call based on an input parameter and updates a local stack variable.
- `dispatch_via_table`: Checks a value against a range and executes a function from a table, or calls functions via pointers stored in a global structure.
- `update_global_pointer`: Checks a value in a structure and, if it matches specific constants, calls another routine and stores the result in a global variable.
- `return_void`: The routine is empty and performs no operations before returning.
- `traverse_linked_list_2`: Iterates through a linked list and calls another routine if specific bitwise conditions and value comparisons are met for each node.
- `return_uninitialized_register`: This routine returns the value currently held in a data register without performing any operations.
- `execute_double_call`: Calls another routine twice in succession.
- `return_immediately_3`: This routine performs an immediate return to its caller without executing any instructions.
- `return_undefined_byte`: This routine is empty and returns an uninitialized byte value.
- `clear_and_call_pointers`: Clears a word in a structure and conditionally calls two different functions stored in a table based on the values of two other memory locations.
- `call_routine_with_offset`: Checks a value in a structure and, depending on whether it is one or two, calls another routine with a pointer and a specific offset, storing the result in a global variable.
- `return_register_value_2`: An empty routine that returns the value currently held in a register.
- `update_state_from_table`: Clears a series of words in a table, checks a condition against a list of entries, and if not met, decrements a byte and updates several words from a table based on an index.
- `initialize_global_tables`: Clears a global byte, calls several other routines, and conditionally calls another routine based on a global short.
- `initialize_game_state_2`: Clears and initializes a large set of global variables and tables, sets several constants, and calls other routines to set up the engine.
- `reset_object_fields_by_index`: The routine uses an index from a parameter to clear and set multiple bytes and words in a large structure referenced via an uninitialized pointer, initializing fields conditionally based on a flag in the parameter.
- `process_script_command_loop`: Initializes a global flag and enters a loop that fetches and processes script data using several helper routines.
- `process_node_and_dispatch`: Calls another routine with a calculated offset and a constant argument, then calls another routine with no arguments.
- `initialize_and_process_nodes`: The routine writes a constant byte to a global variable and loops three times calling two other routines with a counter.
- `fill_struct_and_process`: Computes an offset from a parameter to write several fields into a global structure.
- `fill_struct_array_with_params`: The routine loops over a fixed count of entries, computes an address using the loop index and a parameter, and writes the parameter and part of a second parameter into bytes within each entry.
- `find_matching_range_and_store`: The routine initializes a word in a structure to a sentinel, scans three entries for a range containing a parameter value, stores the matching entry index, forces a default index if a flag is set, and returns the stored index.
- `compute_bitmask_table`: Copies a byte to a word, then populates a table by iterating through a list of words and performing bitwise OR operations based on the bits of a loop counter.
- `execute_script_commands`: The routine calls another routine with a global address and an argument, followed by other calls that may be conditional based on a value in a table.
- `set_global_pointers`: Stores a calculated address into a global variable and sets a word in another global variable to a constant value.
- `return_immediately_4`: This routine performs no operations and returns to its caller immediately.
- `initialize_input_state`: Checks multiple status flags through function pointers and, if successful, resets a series of data structures and populates a table with sequential indices and default values.
- `update_state_variables`: Updates several global variables by calling other routines and storing the returned values or pointers into specific memory locations.
- `write_constant_data`: Stores two longwords containing specific constant values into a global memory area.
- `transform_bitfield_tables`: Performs iterative arithmetic on two memory tables by repeatedly doubling existing values and adding a boolean result derived from comparing two specific bits within each element.
- `perform_repeated_arithmetic`: Executes two loops that repeatedly multiply values in a table by two and add a bitwise comparison result to the product.
- `perform_repeated_multiplication`: Performs a series of arithmetic operations on a value in a table, involving a multiplication by two and a conditional sign check.
- `clamp_and_step_position`: Clamps a word in a global variable to a range defined by two other global words, aligning the value to an 8-byte boundary and stepping it by 8 when a flag is clear, otherwise snapping it.
- `clamp_and_compute_bounds`: Ensures a word in a global variable is at least a constant, otherwise it adjusts it using another word from a global variable, then derives two bounds from the word by shifting and adding constants and stores them in two global words.
- `search_linked_list_for_bytes`: The routine initializes two global words based on a flag parameter using another routine that builds a long from bytes, then walks a linked structure by repeatedly updating global pointers and comparing two bytes from the parameter against bytes in the structure, returning the matching node pointer or zero.
- `assign_script_pointer_if_missing`: The routine checks a table entry indexed by a parameter; if the entry is zero it calls another routine and writes a value from a global variable into that table entry.
- `update_script_entry_with_flag`: Conditionally calls another routine twice based on two global words, and if a global pointer is non-zero obtains a byte from another routine and writes several indexed bytes and words into tables derived from a global base, clears three indexed bytes, then calls another routine.
- `copy_string_or_dispatch`: The routine copies a null-terminated string from a parameter into a buffer in a global area, or dispatches to another routine depending on two global flags.
- `process_script_command`: Checks a global short against a parameter and updates it, then clears a global word.
- `clear_array_elements`: If a certain condition is met, it iterates through an array and sets certain memory locations to zero if they do not equal a certain value.
- `clear_pointer_table`: If a flag in a global variable is non-zero, the routine iterates over a count stored in a global variable and clears each entry in a pointer table.
- `increment_counter_and_dispatch`: The routine increments a short counter in a global variable and branches on its value.
- `initialize_script_state`: The routine initializes several global variables and a small table with values obtained from other routines, updates a running accumulator stored in a global variable, and sets up a fixed-size descriptor in memory using values read from globals.
- `load_script_resources`: Reads multiple data blocks from a source into various global buffers and populates a table with six 16-bit values derived from the lengths of these loaded resources.
- `zero_multiple_tables`: The routine zeroes several groups of words and bytes referenced through global pointers, using counts from global variables, and also clears two fixed-size byte areas in global memory.
- `update_state_and_counters`: Compares a parameter against a global short, updates it, and clears a global word.
- `process_script_state`: Decrements a global short counter and, when it reaches zero, performs a series of calls and stores data from global areas into a structure.
- `get_stored_word`: Returns a word stored at a specific offset within a global structure.
- `process_input_and_xor`: Calls another routine with a pointer to a local buffer, updates a global word based on the result, and returns a word formed by XORing a global byte with the buffer contents.
- `manage_linked_list`: If a global pointer is non-zero, it calls another routine and clears the pointer, then recursively processes a list to insert a value into a structure.
- `process_input_short_xor`: Calls another routine to fill a small buffer, updates a global word based on the result, and returns a word formed by XORing a global byte with the buffer contents.
- `return_uninitialized_byte`: This routine is empty and returns an uninitialized byte from a register.
- `clear_word_at_offset`: Stores a zero word into a memory location calculated from a global pointer and a fixed offset.
- `interpolate_buffer_values`: Calculates and stores two hundred linearly interpolated byte values into a destination buffer based on three input parameters.
- `update_flag_and_xor`: Updates a global word with a boolean result and returns the result of an XOR operation between a global byte and a value from another routine.
- `process_script_state_flags`: The routine clears a global byte and conditionally calls another routine based on two global shorts and a global byte.
- `set_script_object_field`: The routine clears a global word, computes an index from a parameter and a global short, sets a byte in a table at that index, writes the index and two other globals, and calls another routine with an address built from globals.
- `update_script_object_state`: The routine updates fields of a script object stored in a global variable, computes offsets from several object fields and writes them to other object fields, and conditionally iterates over a count stored in a global variable calling another routine for each entry and updating the object fields with the returned values.
- `decode_script_byte`: Reads a byte from a pointer stored in a global variable and increments that pointer.
- `clear_byte_at_offset`: Stores a zero byte into a memory location calculated from a global pointer and a fixed offset.
- `decompress_variable_length_bitstream`: Iteratively reads a compressed bitstream from a source pointer and expands it into a destination buffer using a lookup table.
- `decompress_bitstream_to_buffer`: Decompresses a variable-length encoded bitstream into a destination buffer using a lookup table.
- `decode_compressed_stream`: Decompresses a bitstream using a variable-length encoding scheme and writes the resulting bytes to a destination buffer.
- `process_conditional_index`: Resets several global values, checks if a global short is within a specific range, compares a value from a table to a global byte, and conditionally updates a global byte and calls another routine.
- `initialize_memory_blocks`: Calls another routine with a pointer from a global location, sets a global byte to one, clears a sequence of bytes in a table, and fills a second table with the value two hundred.
- `decompress_data_stream`: Decompresses a custom bitstream into a destination buffer using a lookup table for character expansion.
- `copy_interleaved_data_blocks`: Copies data from a source buffer to a destination buffer using a fixed stride, where the number of elements and the size of each element are determined by values read from the source buffer.
- `initialize_memory_blocks_2`: Sets a byte to one at a specific location, clears a block of memory, and fills another range with a constant value.
- `clear_byte_flag`: Sets a byte in global memory to zero.
- `store_byte_via_index_and_recurse`: The routine reads a byte from a global variable, uses it as an index to store a byte from the first argument into a table located via a global base, increments the index byte and writes it back, and if the index exceeds a threshold it calls another routine with a fixed offset.
- `copy_palette_entries`: If a byte in global memory is non-zero, the routine copies a number of bytes from a table indexed by words in global memory into a destination area in global memory.
- `search_table_for_range_match_2`: Iterates through a table to find an index where a value satisfies a bitmask check, a bit flag check, and falls within a specific numerical range.
- `search_table_for_value_2`: Iterates through a table to find an index where a value matches a specific range defined by two other tables.
- `search_table_for_range_match`: Iterates through a table to find an index where a value satisfies a bitmask check, a bit flag check, and falls within a specific numerical range defined by two other tables.
- `decrement_and_table_walk_byte`: Copies a byte from a global variable to another global variable and repeatedly decrements it until it reaches zero.
- `iterate_and_clear_counter`: The routine checks a global counter byte and, if it is non-zero, iterates from zero up to that count.
- `iterate_and_update_script_object`: The routine conditionally clears a parameter byte, reads several bytes from tables indexed by a global byte, checks range conditions, loops over a range updating a script object fields and calling another routine for each matching index.
- `reset_and_process_nodes`: The routine clears several byte arrays using counts from global variables, zeroes a 200-byte block, then iterates over two fixed ranges of tables.
- `select_behavior_by_byte`: Reads a byte from a global variable to branch between different routine calls or updates a byte in a global variable and returns a computed value.
- `fill_memory_chunks`: Repeatedly calls another routine to fill memory in chunks of up to a maximum size.
- `pass_parameter_to_routine`: Calls another routine using a value retrieved from a global memory location and a provided parameter.
- `process_xor_data`: Performs XOR operations on parts of a parameter with a global byte, calls another routine, and updates several global variables based on the result.
- `reset_and_mask_data`: Clears two words in global memory, calls another routine, and then performs a bitwise AND on a range of two hundred bytes.
- `return_uninitialized_register_2`: The routine returns the value currently held in a data register without performing any operations.
- `process_indexed_table`: Calculates an offset from a table, performs arithmetic on a value, and iterates through a loop to call another routine with constructed values.
- `call_indirect_pointer`: The routine calls another routine using a pointer retrieved from a global variable.
- `combine_four_bytes`: The routine concatenates four bytes from a pointer into a single 32-bit value and returns it.
- `xor_buffer_with_key`: The routine performs a bitwise XOR operation on a buffer using a key from a global variable, processing the data in chunks.
- `copy_c_string_2`: The routine copies a null-terminated byte string from a source pointer to a destination pointer, advancing both pointers until the terminating zero byte is written.
- `process_sprite_range_and_copy`: Copies a null-terminated string into a stack buffer, computes a value from two global variables, and calls two other routines using that value and a derived offset.
- `update_object_pointer_table`: Copies a sequence of bytes from a source table into a destination buffer and populates a table of sixteen word entries with derived pointer values.
- `fill_script_buffers`: The routine loops a fixed number of times, writes a byte into a global byte, calls another routine, and copies a null-terminated string from a global area into a buffer in a global variable.
- `store_rectangle_and_metadata`: The routine normalizes two coordinate values using a fourth parameter, then stores rectangle bounds and several metadata bytes and words into multiple global arrays indexed by a parameter.
- `clear_object_table_and_draw`: Clears a small table of word entries to a sentinel value and then, depending on a flag stored in a global variable, calls another routine multiple times with coordinates derived from a global position and size values together with several metadata bytes and words loaded from global variables.
- `update_display_state`: The routine calls another routine with values computed from global words, then conditionally calls other routines based on a global short flag and another global short.
- `process_script_tree_node`: Reads a short entry from a table indexed by a parameter and, if it is not -1, loads related shorts and a flag byte from several tables.
- `find_containing_rectangle`: The routine searches a fixed set of rectangles stored in global memory for one that contains the point given by a parameter and returns its index or a sentinel value if none matches.
- `initialize_script_entry`: The routine consists of 18 instructions and is called by two unnamed routines.
- `adjust_coordinate_differences`: Calculates the absolute difference between two sets of coordinate fields by subtracting the second parameter from the first where negative, then calls another routine four times using various combinations of those resulting fields.
- `recursive_call_loop`: The routine calls itself indefinitely.
- `update_global_pointer_2`: The routine checks a parameter and, depending on its value, calls another routine and stores the result in a global variable.
- `return_register_byte`: The routine returns a single byte from a register.
- `execute_recursive_call`: This routine performs a single recursive call to itself before returning.
- `update_data_table`: copies bytes from a calculated source address into a destination buffer and populates a table with sixteen word values derived from the source data.
- `check_bitmask_matches`: iterates through a list of values and sets bytes in a global array to one if they satisfy specific bitwise comparison conditions.
- `execute_conditional_sequence`: performs a series of calls to other routines based on the state of two global variables.
- `call_external_function`: calls another routine located at a specific memory address.
- `dispatch_stored_routine`: checks a global variable for a non-zero value and, if present, calls the routine pointed to by that value before clearing it.
- `forward_parameter_to_helper_2`: Immediately passes its input parameter to another routine and returns.
- `process_single_node`: The routine computes an offset from the parameter, checks a table entry for non-zero, and if present calls another routine with that entry and then clears the table entry.
- `process_script_nodes`: Clears a global byte and repeatedly calls another routine while comparing a parameter with a returned value and checking a returned short.
- `search_script_nodes_with_flag`: Clears a global byte and repeatedly calls another routine while comparing a parameter with a returned value and checking a returned short.
- `select_max_priority_node`: The routine scans a list of nodes up to a count from a global variable, finds the node with the highest priority byte after masking, stores the selected node in a global variable, clears the node and its flag, and calls another routine with the stored node.
- `find_zero_flag_node`: The routine walks a list of nodes up to a count from a global variable, selects the first node whose flag byte is zero after masking and whose identifier differs from a table value, stores the node in a global variable, clears the node, and calls another routine with it.
- `find_zero_flag_node_alt`: The routine walks a list of nodes stored at a fixed offset, selects the first node whose flag byte is zero after masking, stores the node in a global variable, clears the node, and calls another routine with it.
- `find_zero_flag_node_alt2`: The routine walks a list of nodes up to a count from a global variable, selects the first node whose flag byte is zero after masking, stores the node in a global variable, clears the node, and calls another routine with it.
- `select_max_priority_node_2`: The routine copies a byte from a global variable to another global byte, then loops incrementing a global byte and dispatching to one of four node-selection helpers based on its value, stopping when a flag byte becomes non-zero.
- `process_script_entries`: Calls another routine with two global pointers, then loops a number of times taken from a global short, calling another routine each iteration with a global pointer and a global pointer, and finally calls a routine with a global pointer and a constant.
- `rotate_bytes`: takes a 32-bit value and returns it with its four bytes rotated to new positions by shifting and masking.
- `register_script_command_pointers`: The routine walks a list of script command entries and stores the address of each entry into several global tables indexed by a byte from the entry and by values read from global variables.
- `process_script_command_list`: The routine processes a script command list pointed to by a global variable, iterating over its entries and invoking another routine with derived parameters for each entry.
- `parse_script_resource_opcodes`: Iterates through a buffer of data to interpret a sequence of command bytes.
- `find_maximum_chain_sum`: The routine walks a list of linked structures using a global pointer and a global count, calling another routine on each node's second word and accumulating the sum per chain.
- `insert_node_with_byte_rotation`: The routine computes a rotated value from a node field and decides whether to insert a new node before or after an existing node based on a distance check.
- `copy_words`: copies a block of words from a source pointer to a destination pointer for a specified count.
- `search_range_for_flagged_node`: The routine scans a range of nodes bounded by a base address and a limit, using a rotated value from each node to step through the range.
- `process_node_chain`: Walks a linked list of nodes bounded by a limit and updates each node's fields using several helper routines, then returns the final pointer.
- `process_node_chain_2`: The routine builds a chain of nodes from global data using another routine and then walks the chain to find a node matching a parameter, returning a result from another routine.
- `insert_node_with_byte_rotation_2`: Ensures a parameter is at least eight, aligns it down to an eight-byte boundary, then walks a linked list of nodes from a global pointer to find an insertion point.
- `link_node_and_update_neighbors`: It updates predecessor and successor links, merges or adjusts neighbor link values using rotated sums, and returns a rotated value derived from a predecessor.
- `collect_and_sort_fixed_point_samples`: Allocates a temporary buffer to accumulate fixed-point samples and their associated pointers from a global source, sorts the resulting table by sample value using a bubble sort, builds a linked list from the sorted pointers, and stores the sum of each sample and its pointer into an output array.
- `execute_callback_list`: iterates a number of times determined by a global variable and executes a function pointer stored in a global variable during each iteration.
- `calculate_complex_modulo`: performs a complex mathematical operation involving bit shifts, rotations, and modulo arithmetic on two input values.
- `traverse_linked_list_3`: Traverses a linked list of pointers, potentially allocating and zero-initializing a new block of memory if the list is empty, before calling another routine with the list head.
- `update_global_state`: Stores a parameter into a global variable, clears a second global variable, and calls two other routines.
- `calculate_string_length`: Calculates the length of a null-terminated byte string by iterating through it until a zero byte is found.
- `process_structured_data`: Checks flags within a data structure, calls other routines based on specific bitwise conditions, and returns a status value.
- `multiply_32bit_integers`: Performs a 32-bit multiplication of two values by combining the results of 16-bit partial products.
- `copy_byte_string`: Copies a null-terminated byte string from a source address to a destination address.
- `clear_state_and_fetch_value`: Clears a global word, loops calling another routine until a global word becomes non-zero, optionally calls another routine and captures its result, then returns a combined value formed from that result and a global word.
- `update_script_state_and_dispatch`: The routine updates several global words based on other globals, clamps a computed offset, and returns early if a script counter is zero.
- `increment_counter_and_copy_word`: Reads a short from a global variable, increments it, and if the previous value was zero sets a byte flag, calls another routine, and copies a word from one global location to another.
- `decrement_counter_and_copy_word`: Reads a short from a global variable, decrements it, and if the result is zero or the original was less than one clears a word and a byte flag, calls another routine, and copies a word between two global locations.
- `copy_cstring_to_buffer`: Copies a null-terminated byte string from a source address derived from a table and a parameter into a fixed buffer in a global variable, then writes a terminating zero byte.
- `initialize_script_object`: The routine checks a global flag and branches to set up script object globals, writes several constants into global words, updates a global entry indexed by a parameter, and loops calling another routine while a global word is zero.
- `handle_script_string`: The routine measures the length of a null-terminated string in a global buffer, clears its last byte, and calls another routine with the buffer pointer.
- `process_script_string`: The routine measures the length of a null-terminated string in a global buffer, clears its last byte, and calls another routine with the buffer pointer.
- `copy_string_and_call`: Copies up to seventy bytes from a source to a global table and then executes a function pointer stored in a global variable.
- `clear_global_flag`: Checks if a byte in a global variable is non-zero and, if so, calls another routine before setting that byte to zero.
- `fetch_offset_word`: calls another routine and returns a word located at a specific offset from the returned pointer.
- `store_routine_result`: Clears a global word, calls another routine to obtain a value, stores that value in a global variable, and calls a third routine.
- `fetch_and_increment_indexed_value`: Calls another routine and compares a value stored at a global offset with a value at a different global offset.
- `fetch_offset_word_2`: This routine calls another routine and returns a word located at a specific offset from the pointer returned by that call.
- `process_state_logic`: Checks a global variable against specific constants to either toggle a byte, perform complex data transfers involving multiple memory locations and function calls, or execute a specific routine.
- `return_immediately_5`: This routine is empty and performs no operations before returning to the caller.
- `copy_memory_block_6`: Copies a specified number of bytes from one memory location to another using a loop.
- `return_uninitialized_value`: returns an uninitialized value from a register.
- `clear_global_byte`: Sets a byte at a specific offset in a global structure to zero and calls another routine.
- `return_immediately_6`: This routine performs no operations and returns control to its caller immediately.
- `execute_callback_with_data`: Stores a value and a pointer into global memory, executes a routine with those parameters, and returns a value from a global variable.
- `interpolate_values`: Calculates the absolute difference between two sets of coordinates and uses a loop to step through and call another routine for each incremented step.
- `set_global_index`: Sets two different global variables to a provided value, ensuring the value is at least zero, and calls another routine if the value is zero.
- `combine_and_store_values`: Returns a value formed by combining a parameter and a second value, while storing the first parameter into a global variable.
- `set_flag_conditionally`: Checks a byte in a global variable and executes a series of calls based on its value before setting that byte to one.
- `reset_script_state`: Clears a fixed range of bytes in a global area, sets a global byte to 255, and then scans a range of flags to set a global index and call three helper routines for matching entries.
- `fetch_byte_from_pointer`: Loads a byte pointer from a global variable, increments the stored pointer value, and returns the byte pointed to by the original pointer.
- `fetch_word_from_pointer`: Loads a byte pointer from a global variable, advances the stored pointer, reads two consecutive bytes from the original address and returns them as a little-endian word.
- `process_byte_stream_loop`: Loops while a byte in a global variable is not -1, fetches a byte from a pointer stored in a global variable, stores it into a global byte, copies a global word, sets a byte in a table indexed by the loop byte, and calls a function pointer selected by the fetched byte.
- `process_pending_command`: Checks a global byte flag and, if it is not -1, calls three helper routines to update pointers and resolve a script object reference.
- `update_pointer_from_index`: If a byte flag in a global variable is not 255, it loads an indexed pointer from a table using that byte as an index and adds an offset from a global variable, storing the result back into a global pointer variable.
- `store_pointer_delta_to_table`: If a byte flag in a global variable is not 255, it computes the difference between two global integer values and stores the result into a table entry indexed by the flag byte.
- `call_other_routine_twice`: Calls another routine two times in succession.
- `resolve_script_object_reference`: Resolves a script object reference by reading a flag byte from a global variable and using an index to look up a script entry.
- `clear_matching_entries_2`: Iterates through a table and clears a byte field if specific status flags or comparison conditions are met.
- `clear_matching_entries`: Iterates through a table of entries and clears specific fields if certain status bytes or comparison conditions are met.
- `process_matching_entry_by_id`: Scans a range of entries for a matching identifier and type, clears related fields, and updates a global byte.
- `reset_matching_identifiers_2`: Compares a value against a table of shorts to reset specific bytes and identifiers, then iterates through a second table to mark matching entries with sentinel values.
- `reset_matching_identifiers`: Compares a value against a list of entries and resets specific bytes and identifiers to zero or sentinel values if they match.
- `mark_matching_entries`: Iterates over a range of short entries and, when an entry matches a value from a global variable, writes sentinel values to related locations in global memory and marks the entry.
- `update_script_entry_2`: Updates tables indexed by global bytes, stores a byte from a parameter into a global byte, calls three helper routines, and conditionally restores a global index based on table comparisons.
- `dispatch_script_operations`: This routine calls five different helper routines to manage script pointers, process pending commands, match entries by identifier, update script entries, and find the next non-zero entry in a table.
- `find_next_nonzero_entry`: Scans a byte table starting at index one for the first non-zero entry and returns its index.
- `update_table_and_flags_2`: Iterates through a loop to conditionally set a bit in a table and increment a byte, then increments several bytes in a separate table, and clears a bit and a byte in another table based on a global word.
- `update_table_and_flags`: Iterates through a loop to conditionally set a bit in a table and increment a byte, then increments several bytes in a separate table, and finally clears a bit and a byte in a table based on a global word value.
- `increment_table_and_clear_flag`: Increments a series of bytes in a table based on a counter and, if a specific global value is not 255, clears a bit in a byte and sets another byte to zero.
- `copy_string_and_dispatch`: Copies a null-terminated string from a pointer into a local buffer and then calls another routine with the buffer and a constant offset, followed by a call with the buffer and a parameter.
- `process_script_command_list_2`: Calls another routine with a global address and an argument, calls another routine with a global address, conditionally calls another routine with a constant when a global word equals a specific value, then calls another routine and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_with_constant`: Executes several calls and then sets a local word to all ones before passing it to another routine.
- `process_with_build_long`: Calls a routine that constructs a 32-bit value from four consecutive bytes pointed to by its parameter.
- `parse_script_token_stream_3`: This routine is called by three different parsing routines and contains 48 instructions.
- `parse_script_token_stream`: Iterates through a data stream to parse tokens and control characters, performing recursive calls based on specific byte values.
- `parse_dialogue_control_stream_2`: Iterates through a character stream to interpret control codes and command tokens.
- `parse_dialogue_control_stream`: Iterates through a character stream to interpret control codes and command tokens.
- `parse_script_token_stream_2`: Iterates through a character stream to interpret control codes and command tokens.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `update_script_entry`: Initializes a script buffer pointer, updates several fields in a global entry indexed by the parameter, and then scans a null-terminated string to trim trailing spaces.
- `forward_parameter_to_helper`: The routine immediately forwards its parameter to another routine and returns.
- `convert_short_to_ascii`: Obtains a short value, writes a minus sign to a pointer-based buffer if the value is negative, and then repeatedly divides the absolute value by decreasing powers of ten to write decimal digits into the buffer.
- `parse_script_byte_stream`: Walks a byte stream pointed to by its parameter, copying non-control bytes to a buffer via a global pointer and handling control bytes by dispatching to another routine or writing bytes to a global buffer, terminating on a null byte.
- `search_table_and_dispatch`: The routine obtains a word via a function call and searches a range of entries in three byte-indexed tables for a match, then calls another routine with a computed offset if found, otherwise calls another routine with a fixed offset.
- `conditional_dispatch_by_value`: Obtains a short value and either calls another routine with a fixed offset if the value is zero, or calls two other routines using values derived from global memory.
- `select_and_update_pointer`: The routine checks a parameter field and either loads a fixed address or adds a constant to a global pointer, then calls another routine with the resulting pointer.
- `concatenate_four_bytes`: The routine concatenates four bytes from a pointer into a single 32-bit value.
- `set_pointer_from_table`: The routine computes an address from a table indexed by the high word of the parameter, adds a constant, stores the result into a global variable, and if the result is zero replaces it with a fixed offset from a global variable.
- `update_pointer_from_table`: Compares a parameter against a global word, then either sets a global pointer to a specific address or increments it based on a value retrieved from a table.
- `invoke_lookup_helper`: The routine is empty except for a single call to another routine and an immediate return.
- `lookup_parameter_in_tables`: The routine masks a byte from a global variable with the low word of the parameter, stores the result, then searches two different tables for a match and writes several fields in a global variable before returning a status code.
- `update_range_bounds`: The routine checks bounds using a short from a table indexed by a parameter and a global variable, clamps parameters, and conditionally ORs a value into a range of words in a global table and updates two byte fields in a per-entry structure for a range of indices.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_strings_and_check_file_2`: If a byte in a global variable is zero, the routine copies strings from a table referenced via a global variable and checks file information via a library call.
- `reset_structure_and_seek_2`: Checks a pointer stored in a global variable and, if non-zero, calls another routine with that pointer and then clears the global variable.
- `get_word_from_global`: The routine returns a word stored at a fixed global offset.
- `clear_word_at_global`: The routine writes zero to a word at a fixed global offset.
- `call_reset_routine`: Calls another routine using a pointer from a global variable and the passed parameter.
- `call_indexed_routine`: Checks a flag byte in a structure and conditionally calls another routine using a derived index.
- `xor_decrypt_buffer`: The routine loops while a length counter is positive, clamping each chunk to a maximum size, calls another routine with a pointer, a chunk size and a pointer from a global variable, then XORs each byte of the buffer with a byte from a global variable and advances the pointer, updating the remaining length.
- `call_other_routine_2`: The routine calls another routine and then returns.
- `fetch_script_byte`: The routine calls another routine using a pointer stored in a global variable.
- `reset_word_and_increment_counter_2`: The routine retrieves a word from a global location, and if it is zero, it retrieves another word, stores a boolean result in a global word, and increments a global short counter.
- `reset_word_and_increment_counter`: The routine clears a word at a fixed global offset and increments a short counter at another fixed global offset by one.
- `xor_byte_with_word`: Calls another routine, stores a flag derived from the result into a word in a global variable, then returns the result xored with a byte from a global variable.
- `xor_decrypt_two_bytes`: The routine calls another routine with a stack buffer, a size of one and a pointer from a global variable, stores a flag derived from the returned short into a global variable, XORs two bytes from the stack buffer with a byte from a global variable and returns the combined result.
- `xor_decrypt_four_bytes`: The routine calls another routine with a four-byte stack buffer, a size of one and a pointer from a global variable, stores a flag derived from the returned short into a global variable, XORs each of the four bytes from the stack buffer with a byte from a global variable and returns the combined 32-bit result.
- `update_sprite_animation_state`: The routine reads a short from a global table indexed by a byte in a global variable, loads a pointer from the resulting address, copies several bytes from that pointer into global variables and a local buffer, then calls another routine with a computed offset and stores the result.
- `compute_sprite_position`: Validates two parameters and, if a range lookup succeeds, computes two 32-bit positions from a table indexed by a global word and stores them through pointers in a global structure, adjusting them with an offset and a nibble from the second parameter.
- `add_offset_and_compute_position`: Adds a global short to each of two parameters and passes the results to another routine.
- `compute_sprite_position_2`: The routine subtracts two parameters, takes absolute values, determines a maximum, then loops while accumulating sums and conditionally calls another routine with an updated short value.
- `find_matching_range_index`: The routine iterates through a table of three entries to find an index where a parameter falls within a specific range, returning the index or a default value.
- `dequeue_message_index`: The routine reads a short index from a global variable, compares it with another global short, and if they differ it returns and clears an entry from a global table indexed by the short, then increments the index modulo sixteen.
- `first_segment_empty_rts`: The first empty code slot in this runtime segment contains only a return instruction, so it returns immediately without touching registers or memory.
- `perform_empty_operation`: The routine performs no operations and returns immediately.
- `dequeue_and_update_state`: Clears a global word, calls another routine and stores its result in a global word, calls another routine, and returns the stored word.
- `reset_message_state`: The routine clears three global words used for message handling and then calls another routine.
- `set_dispatch_flag_zero`: Sets a byte in a global structure to zero and then calls another routine.
- `set_dispatch_flag_one`: Sets a byte in a global structure to one and then calls another routine.
- `update_indexed_table_entry`: checks a field of its parameter and, if non-zero, reads a short from a global variable, increments it modulo sixteen, stores the parameter field into a table indexed by the original short, and updates the index variable when the new value differs from another global variable.
- `process_message_loop`: Repeatedly calls another routine with a global pointer and a buffer until it returns zero or a counter reaches a limit.
- `process_message_and_reply`: The routine obtains a message pointer from a global table, and if the pointer is non-zero it copies several fields from the message into a caller-supplied buffer, adjusts a word field according to the message type and some global state, updates a few global variables, then releases the message and returns a flag indicating success.
- `copy_object_data_from_table`: The routine computes a pointer into a table using a parameter index and copies bytes from that entry into several global variables.
- `store_table_entry_bytes`: The routine stores two bytes derived from the parameters into a table slot indexed by the parameter value.
- `update_object_state`: The routine copies two words from a global structure, queries two flags, sets a global word based on the results, updates two global shorts when a byte and a char change, and then selects a byte from a small table to build an indirect call.
- `read_object_words_from_pointer`: The routine reads an integer from a global variable, uses it as a base address, and copies two words from fixed offsets into the two output pointers.
- `return_immediately_10`: This routine performs no operations and returns immediately to the caller.
- `toggle_flag_bit_one`: The routine tests bit one of a global byte, returns whether it was set, and if set clears the bit by XORing the byte.
- `toggle_flag_bit_two`: The routine tests bit two of a global byte, returns whether it was set, and if set clears the bit by XORing the byte.
- `handle_object_state_flags`: The routine updates word and byte flags at fixed offsets in a structure based on a code value in a parameter block, modifies the code value in the parameter block, and returns the updated code.
- `allocate_and_initialize`: The routine calls a function via a global variable table, stores the returned pointer into a global variable, and then calls another function via a pointer stored in a global variable.
- `free_and_output`: The routine calls a function via a pointer stored in a global variable and then calls another function via a global variable table.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `call_indirect_via_table_2`: An empty routine that calls another routine indirectly through a pointer stored in a global variable.
- `conditional_indirect_dispatch`: Tests a byte in a structure and calls one of two other routines indirectly based on the result, then returns.
- `store_handler_parameter`: The routine stores the incoming short parameter into two global shorts, handling a sentinel value, and calls another routine when the parameter is zero.
- `call_handlers_and_compute`: Calls two indirect handlers obtained from a global pointer, then calls another routine with a value built from stack data, a global short and the parameter.
- `reset_state_and_dispatch`: Clears a word flag in a global structure, calls another routine, then calls another routine unconditionally.
- `wait_and_invoke_handlers`: The routine waits for a word change, then invokes two indirect handlers obtained from a global pointer via offsets, waits again, and invokes two more indirect handlers via offsets.
- `toggle_state_flags`: The routine conditionally calls another routine and clears a global byte, copies and toggles a global byte, calls another routine, and if a global short is non-zero it calls another routine, flips a global short, and calls another routine.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `call_decode_table_initialization`: Fetches a value from a memory location via an offset and passes it as an argument to another routine.
- `process_bit_parity_loops`: The routine executes two loops that iterate over arrays of values, updating each entry by adding a boolean result of a bitwise comparison between two specific bits to twice the original value.
- `store_byte_to_offset`: Stores a single byte into a memory location calculated from a register and a fixed offset.
- `build_lookup_table_from_bits`: Calls another routine, copies a byte from a global variable into a global word, calls another routine, and then builds a table of two hundred fifty-six entries by combining words from a table based on the bits of each index.
- `return_immediately_stub_2`: The routine consists of a single return instruction and performs no work, no register or hardware access, and no library calls.
- `initialize_three_structures`: The routine calls another routine three times with three different constant arguments.
- `decode_and_dispatch_script`: Reads a byte from a global variable to index a table of routine pointers, stores the selected pointer in a global variable, and if a flag is set and the index is non-zero it calls another routine with an offset address, scales the returned word, and calls another routine with the scaled value.
- `sync_short_table_entries`: Iterates over a fixed number of short pairs, compares two tables and updates the second with the first when they differ, then calls another routine with an indexed value.
- `copy_data_to_buffer`: Stores a parameter into a global word and copies sixteen bytes from a source area into a destination area.
- `copy_bytes_via_table`: The routine copies 16 bytes from a source area addressed by a global variable pointer and an index table into a destination area referenced through a global variable, using an offset derived from another global variable.
- `copy_string_and_call_2`: The routine copies a null-terminated string into a local buffer and then calls another routine using that buffer and a provided parameter.
- `process_data_loop`: The routine performs a series of conditional checks and nested calls, eventually writing several bytes into a destination area based on a loop counter and a parameter.
- `fill_table_entries`: The routine iterates forty times to store two different byte values into specific offsets within a table structure.
- `initialize_game_state`: The routine attempts to obtain a resource via an indirect call through a global table and stores the result in a global structure.
- `execute_table_function`: The routine checks if a value is within a specific range and then calls a function retrieved from a table.
- `cleanup_and_exit`: Performs multiple function calls via pointers in a global table, clears memory, and closes libraries before returning a constant.
- `open_libraries_and_set_flags`: The routine obtains library bases via indirect calls, stores them into global variables, checks a flag in one library structure and writes two word values into global variables accordingly, then calls another routine and stores a stack value into a global variable before returning a success flag.
- `close_libraries_twice`: The routine performs two close-library calls via indirect function pointers derived from a global variable, using the same offset twice.
- `allocate_memory_blocks`: The routine calls two functions via a library base stored in a global variable, obtains a memory pointer via an indirect call, stores it and several offsets into multiple global variables, repeats the allocation for a second set of pointers, and returns a flag indicating success.
- `free_memory_twice`: The routine performs two free-memory calls via indirect function pointers derived from a global variable, using the same offset twice.
- `initialize_interrupt_structure`: Writes constant bytes and pointers into a structure in memory, stores a self-reference as a code pointer, and calls an indirect routine obtained from a global variable.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `check_current_directory_info`: The routine calls two functions via a library base stored in a global variable, then calls another function via the same base and returns a boolean indicating whether a field in a structure pointed to by a global variable equals a constant.
- `copy_strings_from_table`: The routine disables interrupts with Forbid and enables them with Permit, then iterates a table referenced via a global variable.
- `check_file_info`: The routine obtains a pointer from a global variable and calls a library function through it.
- `copy_strings_and_check_file`: If a byte in a global variable is zero, copies strings from a table referenced via a global variable, checks file info via a library call through a pointer from a global variable, initializes format state by clearing a counter and storing an argument, then iterates a table referenced via a global variable and for each entry calls routines to copy strings and check file info, returning on error.
- `fill_buffer_from_value`: The routine receives a byte pointer and a parameter containing a byte value and a repeat count.
- `execute_repeated_function`: The routine iterates a number of times based on a value in a global variable and calls a function through a pointer in a global variable during each iteration.
- `check_structure_values`: Compares two 16-bit values from a structure against specific constants and returns one based on the result of another routine.
- `return_zero_constant_3`: The routine contains no operations and returns the constant zero.
- `accumulate_script_chunks`: The routine loops while a count parameter is positive, clamping the count to a maximum, calls another routine with a pointer, a clamped count and a parameter, adds the returned short to an accumulator, subtracts the returned value from the count and advances the pointer, repeating while the returned value equals the clamped count.
- `allocate_and_process_resources`: The routine calls exec.AllocMem and exec.FreeMem and invokes several helper routines to copy strings, check a file, accumulate script chunks, dispatch conditionally, compute an adjusted offset, reset a structure and seek, and process an entry while clearing flags.
- `decrement_flagged_entries_2`: Clears two 16-bit fields in a structure and then iterates through a table to decrement values in another table if certain conditions are met.
- `decrement_flagged_entries`: The routine loops over a range defined by a byte in a global variable, comparing each byte in a table with a register value.
- `reset_state_and_mask_table`: The routine clears two global words, calls another routine, clears a global byte, and then masks a range of bytes in a table to keep only the high bit.
- `invoke_handler_twice`: The routine calls another routine twice in succession and then returns.
- `initialize_pointers_and_counter`: The routine stores the address of a structure within the same block into a global variable and writes the constant three into another global variable.
- `free_memory_and_clear_flags`: The routine calls exec.FreeMem and then returns.
- `register_entry_and_return_index`: The routine calls another routine to obtain a word, builds a combined value from that word and a global byte, and if the byte is below a limit it increments the global byte, stores a byte derived from the parameter into a table, increments a byte in another table, and returns a word derived from the parameter.
- `call_various_functions`: The routine calls several functions using various offsets and parameters, including a conditional call based on a value in a global variable.
- `clear_field_and_call`: The routine calls two functions and then stores zero into a field of a structure pointed to by a register.
- `execute_empty_call`: The routine performs no operations and immediately returns to the caller.
- `fetch_offset_value`: returns a 16-bit value from an offset within a structure pointed to by a register.
- `set_flag_and_branch`: Calls another routine, then branches based on a global byte to call a different set of other routines, before storing a value into a global byte and returning.
- `clear_flag_conditionally`: The routine conditionally calls another routine if a global byte is non-zero, then stores zero into that global byte and returns.
- `copy_byte_flag`: copies a byte from one memory location to another offset within the same block.
- `toggle_byte_flag`: reads a byte from one offset within a block, XORs it with one, and writes the result to another offset within the same block.
- `load_script_object_state`: The routine copies words from a script object entry into global variables, conditionally adjusts a field, calls another routine, clears a global word and then iterates over a fixed range updating global words and writing back bytes to the object entry.
- `clamp_and_update_bounds`: The routine clamps two 16-bit values read through a global base pointer, updates several 16-bit and 32-bit fields derived from those values and from entries in a table, and then calls another routine.
- `compute_and_call_routine`: The routine builds an address from several values read through a global base pointer and calls another routine with that address and two additional arguments derived from the same base pointer.
- `copy_palette_and_trigger_update`: The routine copies two blocks of 20 bytes from the arguments into a global area and mirrors them into a global variable table, writes a few constants into a global variable, calls several indirect routines from a table, and stores a returned value back into a structure.
- `store_value_from_global`: Reads a 32-bit value from a global variable and writes it to a location computed from another global variable and a fixed offset.
- `return_no_operation`: The routine consists of a single instruction that returns immediately, with no other work performed.
- `return_immediately_stub`: The routine consists of a single return instruction and performs no work, so it only transfers control back to its caller.
- `set_flag_and_copy_word`: Tests a bit in the parameter, writes a 16-bit flag to a global variable and a constant to a global variable, then copies a 32-bit word from a global variable to another global variable.
- `wait_for_word_change`: Spins in a loop until a 32-bit value in a global variable differs from another 32-bit value in a global variable, then returns.
- `update_short_entries`: If a condition on the parameter and a global short holds, it writes the parameter's high word to a global short and calls an indirect function from a global table; it then stores the parameter's low word into a global table entry indexed by the parameter's high byte and into a similarly indexed global variable.
- `invoke_indirect_handler`: Calls an indirect function obtained by adding a constant offset to a pointer stored in a global variable.
- `call_indirect_via_table`: The routine calls another routine indirectly through a pointer stored in a global variable.
- `compute_and_store_range_constants`: Computes a value based on the input parameter and stores it in a global variable, then selects one of two constant words based on whether the parameter is below a specific threshold and stores that word in a global variable.
- `decode_and_copy_bytes_3`: The routine uses a global base pointer to index four tables and decodes nibble-based indices from a word buffer, writing bytes and words into a destination buffer in a repeated loop controlled by a counter.
- `initialize_decode_tables`: The routine stores the argument into a global variable and fills several tables derived from that base with values computed from the bits of two 5-bit counters.
- `return_constant_2`: Returns the constant 24576 in d0 to the caller without any other effect.
- `update_global_pointer_3`: The routine adds a constant offset from a table to a global variable, increments the global variable, and returns zero.
- `return_byte`: Returns the byte pointed to by the parameter.
- `return_word_from_bytes`: The routine builds a 16-bit value from two consecutive bytes pointed to by its parameter and returns it.
- `return_long_from_bytes`: The routine builds a 32-bit value from four consecutive bytes pointed to by its parameter and returns it.
- `setup_blitter_and_wait`: The routine programs the Amiga custom chip registers for a blitter copy, writes computed 32-bit sums and 16-bit constants into a structure pointed to by a base address, then calls another routine repeatedly and spins until a flag bit is cleared.
- `update_blitter_registers`: stores three values into a structure pointed to by a register and waits for a specific bit in a memory location to clear.
- `wait_for_bit_clear`: Enters a loop that repeatedly checks a specific bit within a byte at a memory location pointed to by a register until that bit is no longer set.
- `animate_dma_blitter_coordinates`: Updates DMA controller registers through a series of loops to perform linear interpolation of coordinate values.
- `update_interpolated_coordinates`: Calculates and writes a series of intermediate coordinate values to a structure using a loop that decrements a counter.
- `decode_and_write_bytes`: Repeatedly processes a byte stream using a global variable as a key and a global table as a lookup, updating a 32-bit state and writing transformed bytes through a pointer that is advanced each iteration.
- `return_from_subroutine_3`: The routine is empty and performs a return from a subroutine.
- `return_immediately_7`: This routine performs no operations and returns immediately to its caller.
- `interpret_script_and_manage_resources`: This routine consists of 225 instructions and is called by manage_linked_list and seven other unnamed routines.
- `parse_nested_data_structure`: Recursively traverses and parses a hierarchical data structure by reading bytes from a pointer and managing a stack of state information.
- `process_script_resource_block`: Processes a script resource block by recursively traversing sub-blocks or linked data and updating internal flags, counters, and pointers.
- `tokenize_string_input`: Parses a string into an array of pointers by splitting on whitespace or handling quoted substrings.
- `return_from_subroutine_4`: The routine contains a single instruction and performs a return to the caller.
- `adjust_string_bytes`: The routine walks a zero-terminated byte string pointed to by the argument and, for each byte, checks a flag in a global table indexed by the byte value.
- `skip_bytes_by_table`: The routine examines the first byte of the buffer pointed to by the argument and, while a flag in a global table indexed by that byte is set, advances the pointer by one.
- `format_and_reverse_buffer`: Repeatedly calls another routine to convert values to numeric characters and stores them in a buffer, which is then reversed in place.
- `reverse_engineer`: Repeatedly calls another routine, stores a byte from a register through a pointer, and then reverses a 12-byte buffer in place before terminating it with a null byte.
- `convert_number_to_ascii_2`: Converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `convert_number_to_ascii`: The routine converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `expand_and_parse_data`: Expands packed nibbles from a lookup table into a reversed byte sequence and performs signed integer parsing on a string.
- `reverse_nibbles_from_table`: The routine expands a packed nibble value using a global lookup table and writes the resulting bytes in reverse order to a destination.
- `parse_signed_integer_2`: Converts a numeric string into a signed short integer by iterating through characters and applying a sign based on a leading character.
- `parse_signed_integer`: Iterates through a string to calculate a signed integer value based on numeric characters and a potential negative sign.
- `parse_data_stream_2`: The routine iterates through a data stream, reading bytes and using them as indices into a lookup table to determine if they represent control characters.
- `parse_nested_data_stream_2`: Recursively processes a data stream by identifying specific marker bytes to handle nested structures.
- `parse_recursive_data_stream`: Iterates through a data stream by recursively calling another routine to process nested structures.
- `return_immediately_9`: This routine is empty and performs an immediate return to its caller.
- `parse_nested_data_stream`: Iterates through a data stream by checking bytes against a lookup table to determine if they should be skipped or processed.
- `parse_bytecode_stream`: Iterates through a data stream, reading bytes and using them as indices into a lookup table to determine if they represent control characters or operands.
- `parse_data_stream`: Iterates through a data stream, checking bytes against a table and performing conditional jumps or pointer increments based on specific byte values.
- `return_immediately_8`: This routine performs an immediate return to the caller without executing any instructions.
- `store_byte_to_buffer`: The routine increments a counter stored in a global variable, writes a byte from the argument to the location pointed to by a global pointer, and then increments that global pointer.
- `initialize_format_state`: The routine clears a 16-bit counter in a global variable, stores an argument into a global variable, calls another routine with the argument and a stack address, clears a byte through a global pointer, and returns the counter.
- `store_registers_to_pointer`: The routine copies the contents of D1 through D7, A1 through A6, and the value from the stack into successive entries of the pointer passed in A0, and then returns.
- `set_first_element_and_return_concat`: The routine reads a field from the second argument, forces a zero value to one, stores the first word of the first argument into an element of the first argument, and returns a 32-bit value formed by concatenating the adjusted field with the read word.
- `manage_resource_table_entries`: The routine scans a table of short entries using a global count to find the first zero entry.
- `seek_file_wrapper`: The routine obtains a value from another routine and if non-zero calls another routine with a value loaded from an offset, then checks a global word and returns an error code if set, otherwise returns the result.
- `find_memory_block`: The routine searches a linked list of memory blocks for a block of at least a requested size, splitting a larger block if needed and updating a global counter.
- `compute_adjusted_offset`: The routine checks a flag byte in a structure and conditionally calls another routine with a derived index.
- `reset_structure_and_seek`: The routine conditionally adjusts a parameter using a short field from a structure based on flag bits, then clears several fields in the structure and masks flag bits.
- `process_script_resource_block_2`: Iterates over a nested count structure, decrementing a short counter and advancing a byte pointer stored in a structure, reading a byte or calling another routine when the counter reaches zero, writing each byte to an output buffer and returning the outer loop index when a sentinel value is encountered.
- `execute_unreached_code`: This routine consists of thirteen instructions and calls three other routines.
- `fetch_byte_from_block`: Decrements a short counter in a structure and either calls another routine or advances a pointer to return a byte from the structure.
- `forward_call_wrapper`: The routine calls another routine and returns its result unchanged.
- `parse_command_and_prepare_struct`: Examines a command string and a structure to decide flags and an index, obtains a value from another routine using a derived index, and then clears several fields in the structure while writing the obtained value and computed flags into it.
- `fetch_byte_from_script_block`: Checks a flag in a script block and may invoke other routines, decrements a short counter in the block and either returns a byte from a pointer inside the block after advancing the pointer or recursively processes the block.
- `iterate_linked_list_and_process`: Walks a linked list starting from a global variable, tests flag bits in a byte at offset 19 of each entry and compares two words in the entry, conditionally calling another routine, then calls another routine unconditionally.
- `dos_write_wrapper`: The routine checks a signal flag via another routine, clears a global word, calls a function pointer obtained from a global variable and on error stores an error word from another function pointer into globals and returns the result.
- `dos_seek_wrapper`: The routine checks a signal flag via another routine, clears a global word, calls a function pointer from a global variable and on error records an error word, then computes a return value based on parameters and a function pointer result.
- `dos_read_wrapper`: The routine checks a signal flag via another routine, clears a global word, calls a function pointer obtained from a global variable and on error stores an error word from another function pointer into globals before returning the result.
- `dos_open_wrapper`: The routine checks a signal flag via another routine, clears a global word, calls a function pointer from a global variable and if the result is zero records an error word and returns a failure value.
- `dos_lock_open_wrapper`: The routine checks a signal flag via another routine, clears a global word, attempts a lock via a function pointer and on success opens via another pointer, recording an error on failure.
- `reopen_resource_on_failure`: The routine checks a signal flag via another routine, clears a global word, and attempts to open a resource via a function pointer.
- `conditionally_call_and_close`: Checks a global variable and conditionally calls another routine, then calls a function via an indirect pointer from a table.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `divide_fixed_point`: Performs a normalized fixed-point division of two 32-bit values, handling sign logic and bit shifting to maintain precision.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `lookup_table_entry`: The routine checks a parameter against a global variable and a table entry and stores a byte through a pointer.
- `parse_number_with_skip`: The routine returns zero when the argument is zero, otherwise it calls another routine with the argument, then calls another routine with the result and a stack location, and returns the value stored there.
- `free_linked_list`: The routine walks a linked list stored in a global variable, calls another routine for each node, and clears two global pointers.
- `allocate_linked_node`: The routine allocates memory with AllocMem, initializes fields of the new node from a parameter and a global variable, links it into a list stored in global variables, and returns a pointer to the node's data area.
- `process_indexed_entry_with_flag`: The routine obtains an index from another routine, tests a flag byte at index plus one, optionally calls another routine, then calls another routine with a value from index plus two and returns a fixed address if a global word is non-zero.
- `parse_encoded_data_stream_4`: Parses a stream of encoded data by reading character identifiers to determine how to extract and store subsequent values.
- `parse_encoded_data_stream_3`: Parses a stream of encoded data by reading character identifiers to determine how to extract and store subsequent values.
- `parse_encoded_data_stream_2`: Parses a stream of encoded data using character identifiers to determine the type of value to extract.
- `parse_encoded_data_stream`: This routine acts as a multi-mode data parser that reads values from a stream based on a leading character identifier.
- `accumulate_linked_list_values`: Iteratively traverses a linked structure by calling a function pointer to retrieve values and updating a counter.
- `parse_signed_fixed_point`: The routine checks for sign characters and adjusts a value before iteratively calling a function pointer to accumulate data and store the result through a pointer.
- `extract_fixed_point_data_2`: Iteratively calls a function pointer to accumulate values, performs a fixed-point product, and stores the result through a pointer.
- `extract_fixed_point_data`: The routine iteratively calls a function pointer to accumulate values, performs a fixed-point product, and stores the result through a pointer.
- `store_word_and_advance_2`: The routine stores a 16-bit value from a memory location through a pointer and updates a destination pointer with a new value.
- `store_and_increment`: Stores a 16-bit value from a memory location through a pointer and updates a destination pointer with a new value.
- `accumulate_variable_length_bits`: The routine iteratively extracts bits from a stream via a function pointer and accumulates them into a local variable.
- `parse_bitstream_and_store`: The routine iteratively extracts bits from a stream via a function pointer, accumulates them into a value, and stores either a 16-bit or 32-bit result through a pointer based on a condition.
- `store_word_via_pointer`: Stores a 32-bit value through a pointer located at a destination address and updates a secondary pointer.
- `decode_variable_length_integer_2`: Reconstructs an integer from a stream of encoded bytes by checking specific bit flags to determine if additional data segments must be shifted and combined.
- `decode_variable_length_integer`: Reads a sequence of values using a function pointer and reconstructs a single integer by shifting and masking bits based on flags found in a lookup table.
- `decode_variable_bitstream_2`: Reads a bitstream using a function pointer and uses bitwise flags to determine how to shift and combine values before storing them.
- `decode_variable_bitstream`: The routine reads a bitstream using a function pointer and uses bitwise flags to determine how to shift and combine values before storing the result through a pointer.
- `store_word_conditionally`: The routine checks a condition to determine whether to store a 16-bit or 32-bit value through a pointer and then updates a destination pointer.
- `store_word_via_pointer_2`: Writes a 32-bit value to a location pointed to by a pointer and updates another pointer.
- `convert_decimal_string`: The routine checks a bit in a table to determine if a numeric conversion is required.
- `parse_numeric_string`: Iteratively calls a function to retrieve values and converts them into a decimal integer stored in a global variable.
- `read_byte_stream`: The routine iteratively calls a function pointer to retrieve bytes, stores them through a pointer while incrementing a counter, and handles an end-of-stream signal.
- `increment_pointer_or_set_max`: The routine checks a short value and either sets a destination word to a maximum value or increments a counter and returns the result.
- `increment_and_return`: Increments a value and returns the result.
- `extract_byte_sequence`: Repeatedly calls a function to fetch values and stores them as bytes into a buffer.
- `return_zero_3`: An empty routine that returns zero.
- `store_word_and_advance`: Stores the 16-bit value through the destination pointer and returns the advanced position.
- `parse_format_specifier`: Parses a format specifier string to extract flags, width, precision, and length, then retrieves arguments from a pointer to produce formatted output.
- `scan_format_string`: The routine walks a string looking for percent signs, skips escaped percent signs, and for each format specifier calls a parser and then a function pointer.
- `allocate_memory_block`: The routine checks a flag in a structure and a global variable, then obtains a block via a memory allocator and stores the result into two fields of the structure.
- `iterate_table_and_call`: The routine loops over a count stored in a global variable, reads entries from a table indexed by the loop counter, and conditionally calls another routine with a value from the table.
- `execute_callback_chain`: The routine calls several function pointers stored in a global structure and a local structure, then returns a value composed of a parameter and a value from a table.
- `copy_string`: Copies a null-terminated string from a source address to a destination address.
- `insert_memory_block`: The routine inserts or updates a memory block descriptor in a linked list stored in global variables, adjusting a global counter and returning a status code.
- `execute_conditional_dos_read`: The routine checks a return value from another routine and, if non-zero, calls a function pointer with a value loaded from an offset.
- `copy_memory_block_7`: Copies a specified number of bytes from a source address to a destination address.
- `process_pending_request_and_allocate`: The routine checks a global variable for a pending request and executes it via another routine before clearing the variable.
- `allocate_and_initialize_list_node`: Traverses a linked list starting from a global offset until it finds a node with a non-zero short field or reaches a null pointer.
- `process_entry_and_clear_flags`: The routine conditionally obtains a value from another routine using a derived index, then calls another routine with fields from a structure.
- `clear_table_entry`: Obtains a pointer from another routine and, if the pointer is non-null, checks a flag byte in the pointed-to structure.
- `set_task_signal`: A routine consisting of 16 instructions that calls a function to set a signal.
