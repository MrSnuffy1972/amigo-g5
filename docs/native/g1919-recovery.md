# Game 1919 (indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01919). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2753344 top-level bytes in 128 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 18 of 18 units identical (bytes and relocations), 17 raw-word lines |
| Functions named and described | pass, 530 of 530 named (0 author symbols), 530 of 530 described (88 by rule, 427 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2430 of 2430 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 28835 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11988 of 11996 instructions identical in start and length, 5 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2175511 |
| code | 41978 |
| data | 409596 |
| padding | 27955 |
| unused | 98304 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| SetMap | 2 | 209 | 8 |
| More | 71 | 1986 | 97 |
| endcli | 2 | 117 | 2 |
| loadwb | 2 | 209 | 8 |
| Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 1 of 3).adf:Indy | 7 | 233 | 7 |
| d | 1 | 13 | 1 |
| TRISTAR | 2 | 104 | 4 |
| icon.library | 10 | 1375 | 109 |
| Disk-Validator | 1 | 10 | 2 |
| Disk-Validator | 1 | 9 | 1 |
| Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 1 of 3).adf:bootblock | 1 | 126 | 6 |
| Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 2 of 3).adf:bootblock | 1 | 54 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 168 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 138 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7230 | 275 |
| 98.LFL (raw code file, load address unknown) | 1 | 0 | 0 |
| 23.LFL (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar](disk 2 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 2 of 3).adf: file rooms/25.LFL (header 23083 bytes, blocks hold 5856): its blocks are judged as raw blocks outside the filesystem
- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar](disk 3 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 3 of 3).adf: file rooms/14.LFL (header 48283 bytes, blocks hold 17080): its blocks are judged as raw blocks outside the filesystem
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EB8) A7=an address (A7)=an address, slid to exec.library -an address): 366 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address (play-rs-m1500): 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 23 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 70 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2014 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar](disk 1 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 1 of 3).adf:Rooms/98.LFL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar](disk 2 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 2 of 3).adf:rooms/23.LFL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar](disk 2 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 2 of 3).adf: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.60)
- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar](disk 2 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 2 of 3).adf: blocks 13-15 (bytes 6656-8192) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.22)
- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar](disk 2 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 2 of 3).adf: blocks 17-18 (bytes 8704-9728) outside the filesystem hold data, not code (opcode density 0.010, entropy 6.33)
- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar](disk 2 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 2 of 3).adf: blocks 1131-1131 (bytes 579072-579584) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.89)
- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar](disk 2 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 2 of 3).adf: blocks 1727-1759 (bytes 884224-901120) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.45)
- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar](disk 3 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 3 of 3).adf: blocks 986-986 (bytes 504832-505344) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.09)
- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar](disk 3 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 3 of 3).adf: blocks 1364-1370 (bytes 698368-701952) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.84)
- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar](disk 3 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 3 of 3).adf: blocks 1375-1462 (bytes 704000-749056) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.68)
- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar](disk 3 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 3 of 3).adf: blocks 1471-1473 (bytes 753152-754688) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.19)
- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar](disk 3 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 3 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar](disk 1 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 1 of 3).adf:Indy: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 28835 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 10240 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `prompt_disk_insertion` (runtime segment an address): Displays a prompt requesting the user to insert a specific disk and press ENTER. It enters a loop that polls for a disk change or user input, attempting up to six retries before terminating. Kind: disk insertion/track check. Pass path: the branch where _DAT_00c1c996 != 0 (the loop breaks and execution continues).

## Functions


### SetMap

- `parse_amiga_hunk_relocations`: Iterates through a memory structure to identify and process AmigaDOS hunk types.
- `stub_empty_return_routine`: This routine is empty and contains only a return instruction.
- `decrement_integer_loop`: Decrements an integer value in a loop until it reaches zero.
- `tiny_ori`: Small routine of 1 instruction that merges 44 into d0.
- `copy_memory_blocks`: Performs a complex memory copy operation involving conditional loops and bit-shifting logic to move data between memory locations.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_cycle_counter`: Decrements a byte register once for each count unit, wrapping to a space character when it reaches zero, and returns.
- `shift_and_store_byte`: Shifts a word in a pointer by one bit and stores a byte from a table into a memory location.

### More

- `initialize_resource_loading_environment`: Checks for the existence of a specific resource or directory and initializes a structure containing function pointers and data buffers.
- `execute_callback_chain`: Checks several global flags and executes corresponding function pointers stored in a structure before freeing a linked list.
- `invoke_two_function_pointers`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `execute_callback_chain_2`: Checks a series of global flags and executes corresponding function pointers stored within a structure.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `split_arguments_and_open_streams`: Splits an argument string in place into up to 32 words, storing a pointer to each word and clearing the separators, then opens the input and output handles through DOS calls and records them.
- `allocate_and_init_block`: Allocates a block sized to the input plus a constant, stores that size inside the block, passes the global block and the block address to another routine, and returns the block address plus a constant offset.
- `allocate_and_init_block_2`: Allocates a block sized to the input plus a constant, stores that size inside the block, and passes the global block and the block address to another routine.
- `return_minus_one`: Returns minus one without other work.
- `free_linked_memory_blocks`: Walks a singly linked list whose head is held in a global, freeing each node with the size stored at its offset and following the next pointer until the list ends.
- `append_node_to_list`: Appends a node to a list whose head and tail pointers are held in the first structure by updating the next and head/tail pointers.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_seek`: Library stub: loads its arguments and the dos library base, calls Seek and returns its result to the caller.
- `call_delete_file`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `call_rename`: Library stub: loads its arguments and the dos library base, calls Rename and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `implement_more_utility`: Implements a command-line style pager that reads a file and displays its contents line by line.
- `visit_global_table_indices`: Loops twenty times calling one routine to get an index and passing the global table address offset by that index to a second routine, then loops a variable number of times passing each counter value to the second routine, and finally forwards two parameters to a third routine.
- `count_string_bytes`: Scans forward from a pointer byte by byte until it reaches a zero byte, counting the bytes examined, and returns that count as the length of the string.
- `copy_string_to_destination`: Copies a null-terminated string from the second argument into the first, byte by byte including the terminator, and returns the destination.
- `copy_string_to_pointer`: Copies a null-terminated string to a destination pointer for a specified number of iterations.
- `clear_byte_and_get_word`: Writes a zero byte to the memory location pointed to by an offset in the stack frame and returns a word from a different offset in the same frame.
- `copy_string_to_offset`: Calculates an offset by scanning for a null terminator and then copies a null-terminated string to that calculated position.
- `copy_string_with_limit`: Copies a null-terminated string to a destination offset, constrained by a maximum number of bytes to copy.
- `clear_first_byte_and_return`: Writes a zero byte through a pointer kept in the frame and returns a word stored at a fixed frame offset.
- `empty_stub_function`: This routine contains a block of instructions but returns without performing any observable effect on memory, registers, or I/O.
- `search_global_table`: Iterates through a table of structures until it finds a non-zero entry or reaches a specific memory limit, then passes the found pointer and two arguments to another routine.
- `forward_entry_arguments_conditional`: Checks a value at an offset from a register and, if it does not match a specific constant, passes three arguments to another routine.
- `forward_entry_arguments`: Passes two stored arguments and a table entry pointer to another routine before returning.
- `parse_stream_command`: Parses a command character from a data stream to execute specific buffer operations based on whether the command is a write, read, or advance.
- `process_stream_command`: Parses a command character from a data stream to execute specific buffer operations based on whether the command is a write, read, or advance operation.
- `update_stream_buffer_2`: This routine performs a seek operation, updates a buffer structure with status flags, and modifies internal pointers and data based on the result.
- `update_stream_buffer`: Routine that updates a buffer structure by setting status flags and modifying internal pointers based on the result of a seek operation.
- `update_stream_buffer_state`: Updates a record structure by conditionally modifying buffer pointers and state flags based on the result of another routine.
- `update_buffer_state_2`: Updates a buffer structure by checking a flag, calling another routine to modify internal data, and resetting specific fields within a pointer-based structure.
- `update_buffer_state`: This routine updates a buffer structure by checking a value, setting specific status flags, and modifying internal pointers and data.
- `update_stream_buffer_state_2_2`: Updates a record structure by conditionally modifying its buffer pointer and state flags based on a byte check.
- `update_stream_buffer_state_2`: Updates a record structure by conditionally modifying its buffer pointer and state flags based on a byte check.
- `update_stream_buffer_state_5`: Modifies a record structure by passing specific flags and a constant to another routine, then updating internal pointers and status bits based on the result.
- `update_stream_buffer_state_4`: Updates a record structure by conditionally resetting or incrementing values based on flag checks and performs file operations.
- `update_stream_buffer_state_3_2`: Updates a record structure by conditionally resetting a value or incrementing a pointer based on a flag check.
- `update_stream_buffer_state_3`: Updates a record structure by conditionally resetting a value or incrementing a pointer based on a flag check.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a6, then returns.
- `update_stream_record`: Modifies a record structure by updating its buffer pointers and state flags based on a conditional check.
- `clear_stream_record_fields`: Clears the record's count word, sets its buffer fields and state byte, then returns the record.
- `clear_object_links`: Calls one routine with the object when flag bit two is set, then clears its two pointer fields and flag byte, and finally passes a byte from the object to another routine.
- `lookup_open_table_entry`: Checks that an index lies within the table bounds and that the top bit of the entry's flag byte is set.
- `update_file_status_3`: This routine searches a table for a non-null entry, modifies status bits in a structure based on a parameter, and performs a file operation via a pointer.
- `update_file_status_2`: Updates a status structure based on bitwise operations and performs a file operation via a pointer.
- `update_file_status`: Routine that modifies a status structure using bitwise operations and performs a file operation via a pointer, updating error codes on failure.
- `update_file_access_state`: Updates a status structure by setting bitmasks based on specific flags and calls another routine using a pointer from that structure.
- `update_status_structure`: Updates a bitmask in a structure and calls another routine using a pointer from that structure.
- `manage_file_access_state_2`: Checks a bitmask in a structure to conditionally call another routine and updates a status value based on a bitwise comparison.
- `update_file_status_and_recurse_2`: Updates a bitmask in a structure based on a flag and conditionally calls another routine using a pointer from that structure.
- `update_file_status_and_recurse`: Updates a bitmask in a structure and conditionally calls another routine using a pointer from that structure.
- `fail_slot_with_invalid_mode`: Clears the slot's status byte, records an invalid-argument error code and returns minus one.
- `manage_file_access_state`: Checks a bitmask in a structure to conditionally call another routine and updates a status value based on a bitwise comparison.
- `open_file_and_store_result`: Attempts to open a file and stores the resulting handle or an error value into a structure.
- `return_stored_value`: Returns a longword stored at a negative offset from a base register.
- `clear_status_flag_and_recurse`: Clears a status bit in the second argument and calls the routine again with the modified arguments, passing the first argument unchanged.
- `read_file_into_buffer`: Attempts to read a specified number of bytes from a file handle into a buffer, returning an error code if the operation fails.
- `read_into_buffer_checked`: Calls a read library routine on the stored handle with the caller's buffer and length, and keeps the byte count.
- `write_to_handle_checked`: Routine that obtains a file handle and, when its flag bit is set, seeks to the end before writing the buffer.
- `seek_file_with_offset_table`: Obtains a file handle and, for an index below three, seeks to an offset taken from a global table, records any I/O error, then seeks back to the start and returns the result.
- `close_file_entry`: Closes a file entry's handle unless a flag bit says not to, records an I/O error and minus one if the close fails, and clears the entry's flag byte.
- `delete_file_with_error_handling`: Attempts to delete a file and updates global error state if the operation fails.
- `return_zero`: This routine is empty and simply returns zero to the caller.
- `open_file_handle`: Opens the file named in the global text buffer with a fixed mode value and stores the resulting handle in a global variable.
- `get_next_byte`: Retrieves a byte from a global variable or reads a single byte from a file handle into a stack slot.
- `read_byte_to_stack`: Reads one byte from a file handle into a stack slot and returns that byte.
- `write_byte_to_default_file`: Writes a byte argument to the file handle, opening the default file first if the handle is unset, and returns the byte.
- `forward_two_args_call`: Forwards its two arguments to one subroutine routine and returns nothing.
- `read_stream_byte_4`: Reads a byte from a stream by checking status flags, potentially calling another routine to refill a buffer, and updating a pointer and a remaining count.
- `read_stream_byte_3`: Manages a data stream by checking status flags, fetching a byte from a buffer, and updating the stream pointer and remaining count.
- `read_stream_byte_2`: This routine manages a data stream by checking status flags, fetching a byte from a buffer, and updating the stream pointer and remaining byte count.
- `fetch_stream_byte_2`: Retrieves a single byte from a data stream by updating stream status flags and managing a remaining byte counter.
- `read_stream_byte`: Reads a byte from a buffer by managing a stream structure, updating status flags, and decrementing a remaining byte count.
- `fetch_stream_byte`: Reads a byte from a memory buffer pointed to by a structure and increments the buffer's internal pointer.
- `forward_stream_to_helper`: Passes the stream pointer from the frame to a helper routine and returns.
- `write_byte_with_retry`: Writes a byte from its first argument into a buffer tracked by a state byte and a count field, updating the buffer position and counter.
- `reset_target_flags`: Returns zero immediately when the structure already has a target and no reset flag is set.
- `shift_global_variable_left`: Returns a global variable shifted left by three bits and performs no other operation.
- `call_with_value_twice`: Passes its single argument to another routine twice, as both parameters, and returns without further work.
- `find_node_by_distance`: Walks a linked list of nodes from a global head, moving by the requested distance and updating a global counter.
- `call_with_second_repeated`: Calls another routine with its first argument followed by its second argument repeated.
- `insert_or_update_list_node`: Inserts a node into a list anchored by a global variable at a position derived from a global counter, or updates an existing node.
- `check_signal_and_dispatch`: Reads the signal state for two bits, and when neither is set returns zero.
- `compute_signed_magnitude_division`: Divides two register inputs by a restoring division loop over 32 bit iterations on their magnitudes, giving the quotient the sign of the xor of the inputs.

### endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_stack_data`: Copies a sequence of words from a source pointer to a destination pointer and then calls another routine.

### loadwb

- `load_and_relocate_hunk`: Parses a data structure containing hunk type codes to load and relocate memory segments.
- `return_from_subroutine`: This routine is empty and performs only a return instruction.
- `decrement_integer_loop_2`: Decrements a value stored in a register by one in each iteration until the value reaches zero.
- `tiny_ori_2`: Small routine of 1 instruction that merges 188 into d0.
- `process_byte_data`: A complex routine that uses a byte cycle counter and bit shifting to move and manipulate data in memory.
- `decrement_byte_cycle_counter_2`: Decrements a byte register once for each count unit, wrapping to a space character when it reaches zero, and returns.
- `decrement_byte_with_space_wrap`: Decrements a byte in a register and replaces its value with a space character if it reaches zero.
- `execute_three_instructions`: A routine consisting of three instructions.

### Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 1 of 3).adf:Indy

- `free_blocks_and_clear_field`: Releases memory blocks through a system call in a loop and then zeroes a longword field in a structure.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `process_memory_relocations`: Performs complex memory operations involving multiple calls to other routines and arithmetic on memory contents.
- `copy_memory_block`: Copies a block of data from a source address to a destination address using a counter.
- `copy_memory_block_recursive`: Copies a block of memory from a source to a destination using a recursive algorithm.
- `copy_memory_block_2`: Copies a block of memory from a source address to a destination address using a loop.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d0.

### d

- `test_bit_and_store`: Checks a bitmask against a value and, if a condition is met, stores several 32-bit values to a stack-based location before performing a pointer calculation and a trap.

### TRISTAR

- `process_library_initialization`: Retrieves a pointer from a global variable and uses it to perform a loop that increments and decrements a counter while calling a routine via a pointer.
- `iterate_string_and_output`: Iterates through a null-terminated string using a pointer and performs two library calls, one of which is likely for outputting the string.
- `copy_memory_block_3`: Copies a block of memory from one location to another using a loop.
- `store_registers_to_memory`: Stores four registers into a series of global memory locations and performs several writes to memory using offsets from a stack pointer.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_list_node`: This library routine modifies a linked list or structure by updating pointers and setting a status bit.
- `set_status_bit`: Library code that sets a specific bit in a byte at a given offset from a base pointer and returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `store_library_pointers`: The library code copies values from a structure into two global variables and calls a routine via a pointer.
- `copy_two_pointers_from_table`: This library routine copies two 32-bit values from an address derived from a register into two global variables.
- `empty_library_routine`: This routine is empty and performs no operations before returning.
- `execute_empty_routine`: This routine performs no operations and immediately returns to its caller.
- `call_library_handler_2`: This library routine calls another routine and then returns.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `call_icon_init_routine_2`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `call_library_handler`: This library routine simply calls another routine and then returns.
- `recurse_with_incremented_parameter`: This library routine calls another routine with the same first argument and an incremented second argument, then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `call_icon_init_routine`: Library code that invokes a single initialization function to set up icon data structures and then returns.
- `dispatch_icon_handler`: This library routine invokes a specific handler routine via a pointer and then returns.
- `execute_recursive_handler`: This library routine calls another routine using a pointer retrieved from data, then returns.
- `call_other_routine`: The library routine calls another routine and then returns.
- `call_other_routine_2`: The library code calls another routine and then returns.
- `call_parse_numeric_string`: This routine calls a routine that parses a numeric string into an integer and then returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `return_immediately`: This library routine contains only a return instruction.
- `manage_icon_hierarchy_2`: This library routine performs a series of conditional calls to manage a hierarchy, including copying data blocks and updating pointers within icon structures.
- `copy_and_assign_structure`: This library routine copies a block of data from one structure to another and updates a pointer in a third location.
- `return_zero_stub_2`: This library routine is an empty stub that immediately returns the constant zero.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `rebuild_icon_hierarchy`: Traverses and updates the hierarchical structure of icon data by managing child pointers and associated metadata.
- `recursively_update_icon_hierarchy`: This routine traverses a hierarchical structure of icon data, updating child pointers and state members through recursive calls.
- `propagate_icon_changes_2`: This library code updates icon states and propagates changes by calling a function pointer and updating various structure members based on the results.
- `update_icon_state`: Library code that iterates through a list using a pointer, updating a global variable and a structure member based on the results of library function calls.
- `call_and_return_zero`: This library routine calls another routine and returns zero.
- `empty_routine_2`: This library routine contains four instructions and performs no observable work; it ends immediately with a return.
- `traverse_icon_structure_hierarchy_2`: Recursively traverses a hierarchical icon structure by following nested pointer tables and lists.
- `traverse_icon_structure_hierarchy`: Recursively traverses a hierarchical icon structure by following nested pointer tables and lists.
- `iterate_icon_structures`: Library code that traverses multiple levels of pointer tables, invoking a callback for each entry and updating a counter based on the results.
- `iterate_icon_table`: This library routine walks a null-terminated table of pointers, calling a routine for each entry and updating a counter.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `traverse_linked_list`: This library routine traverses a linked list by following pointers and executing a routine on each node.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `manage_icon_hierarchy`: Library code that performs a series of conditional library calls to manage a hierarchy, potentially updating a pointer or a value at a specific offset.
- `return_zero_constant`: Library code that performs no operations and returns zero.
- `return_zero_value`: This library routine performs no operations and returns zero to the caller.
- `update_indexed_table`: This library routine decrements a short counter through a pointer and uses the result to store two values into a table.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `recursively_update_icon_hierarchy_2`: Traverses a hierarchical structure by calling a routine on child nodes and managing pointer updates.
- `return_zero_library_call`: This library routine calls another routine and then returns zero.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `incremental_recursive_call`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `find_string_assignment`: Library code that iterates through a list of strings to find a specific character offset, specifically looking for an equals sign.
- `find_string_equal_sign`: Library code that iterates through a linked list and compares memory blocks to find a specific character offset.
- `find_equal_sign_offset`: Library code that iterates through a linked list to find a specific character offset within a string.
- `verify_string_integrity`: Library code that compares a calculated length or checksum against a provided value while iterating through a sequence.
- `validate_string_length`: Library code that searches for a null terminator and compares the resulting length against a target value.
- `search_list_checksum`: Library code that traverses a linked list while performing checksum calculations and memory comparisons.
- `search_linked_list`: This library routine performs a search or calculation by iterating through a list of values.
- `verify_string_length`: Library code that iterates through a sequence to verify if a calculated length matches a specific value.
- `parse_numeric_string_3`: Library code that parses a numeric string into an integer, copies data to a destination, and manages string offsets.
- `parse_numeric_string_2`: Library code that parses a numeric string into an integer and compares it against a value in a structure.
- `convert_numeric_string`: Library code that parses a string of digits into an integer and performs a subsequent operation if the value is non-zero.
- `parse_numeric_string`: Library code that converts a string of digits into a numeric value and performs a table lookup based on the result.
- `offset_index_value`: This library routine returns an offset value if the input is within a specific range, otherwise it returns negative one.
- `return_all_ones_2`: The library code returns a 32-bit value containing all bits set to one.
- `call_library_function`: Library code that calls a function via a pointer retrieved from a library offset.
- `call_library_function_offset`: Library code that calls a function via a different pointer retrieved from a library offset.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `assign_library_value`: The library code calls a routine and, if the result is non-zero, writes a constant value to a memory location and a pointer.
- `return_zero_2`: Library code that simply returns zero.
- `compare_and_zero_on_mismatch`: This library routine calls another routine and compares its return value to a provided parameter.
- `tiny_move_rts`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `validate_library_call`: This library routine calls another function and returns zero if the result does not match a provided parameter.
- `empty_routine`: This library routine is empty and returns immediately.
- `rotate_32bit_value_2`: Library code that performs bitwise rotation and arithmetic operations on a 32-bit value.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `find_null_terminator`: Library code that iterates through a string looking for a specific byte and returns a pointer to it, or returns zero if the null terminator is reached first.
- `return_zero_3`: This library routine is an empty stub that returns zero.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `copy_string_to_end`: Library code that finds the end of a source string and copies a second string to that location.
- `compare_memory_blocks`: Library code that compares two memory areas for equality up to a specified length and returns one if they match or zero if they do not.
- `return_constant_one`: Library code that returns the integer value one.
- `traverse_string_and_call`: Library code that iterates through a string and calls another routine when the end of the string is reached.
- `call_another_routine`: Library code that calls another routine before returning.
- `copy_and_truncate_string`: The library code copies bytes from a source to a destination for a specified count and then pads the remaining length of the destination with null bytes.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `validate_library_chain_2`: This library routine repeatedly calls a routine in a loop and checks a value at an offset from a pointer.
- `validate_library_chain`: The library code repeatedly calls a routine in a loop and checks a value at an offset from a pointer.
- `return_zero_stub`: This library routine returns the constant zero.
- `propagate_icon_changes`: Library code that recursively propagates icon changes through a hierarchy, performing 32-bit integer multiplication and updating a value at an offset within a structure.
- `return_negative_one_stub`: This library routine returns the constant negative one.
- `validate_icon_list`: This library code iterates through a linked list of icon structures, verifying members and updating structure offsets and pointers.
- `traverse_icon_list`: Library code that iterates through a linked list of icons, updating structure members and returning a value or negative one upon failure.
- `validate_and_update_icon_list`: Iterates through a linked list of icon structures, verifying specific member values and updating structure offsets.
- `return_all_ones`: This library routine returns a value consisting of all bits set to one.

### Disk-Validator

- `execute_initial_instructions`: A routine consisting of nine instructions that calls another routine containing a single instruction.
- `execute_single_instruction`: This routine consists of a single instruction and is called by another routine.

### Disk-Validator

- `execute_entry_routine`: This routine consists of nine instructions and serves as a program entry point.

### Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `close_system_libraries`: Closes several system libraries and manages the system state.
- `initialize_boot_environment_2`: This routine opens and closes several system libraries and enables DMA via the DMACON register.
- `execute_fourteen_instructions_3`: A routine consisting of eleven instructions that calls another routine to execute fourteen instructions.
- `execute_fourteen_instructions`: A routine consisting of fourteen instructions with no further information available.
- `execute_nine_instructions`: A routine consisting of nine instructions with no further information available.

### Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar](Disk 2 of 3).adf:bootblock

- `initialize_boot_environment`: A routine that calls several library functions, interacts with coprocessor registers, and enables a DMA channel via a hardware register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_callback_chain_3`: Calls two initial functions, modifies several global variables, and then executes a series of conditional function calls through pointers stored in a structure.
- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `process_game_loop_logic`: Executes a complex main loop containing conditional branching, bitwise manipulations, and multiple calls to subroutines.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `chain_seven_subroutine_calls`: Calls seven routines one after another with no branches, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `prompt_disk_insertion`: Displays a prompt requesting the user to insert a specific disk and press ENTER. It enters a loop that polls for a disk change or user input, attempting up to six retries before terminating.
- `request_disk_insertion`: Triggers a prompt requesting the user to insert a specific disk and press enter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_with_stored_argument`: Stores its argument in a shared word, then calls a library function with a fixed set of values and calls a helper when that call returns a nonzero result.
- `initialize_shared_tables_and_masks`: Resets shared flag and sentinel bytes, calls helpers, and clears several shared tables, setting the first hundred entries of one to one.
- `initialize_shared_state_defaults`: Sets default values for many shared tables and scalar globals, clears a block of per-slot records, and sets fixed limits such as 319 for a right edge.
- `write_slot_default_record`: Writes a fixed default record into the table slot given by the argument's low word, filling word and byte fields with default values.
- `allocate_and_initialize_structure`: Allocates a new structure, initializes its members with specific values, and populates it with a sequence of self-referencing pointers and data.
- `setup_four_slot_layout_tables`: Stores a vertical range from its arguments and fixed rectangle bounds for four slots.
- `set_constant_byte_pair`: Writes two fixed constant byte values, 77 and 151, into shared globals.
- `generate_bit_manipulated_table_3`: Populates a global variable through eight iterations of bitwise shifts, XOR operations, and arithmetic additions.
- `generate_bit_manipulated_table_2`: Iteratively populates a memory buffer using a sequence of bitwise shifts, XOR operations, and arithmetic additions.
- `generate_bit_manipulated_table`: Iteratively populates a memory buffer using a sequence of bitwise shifts, XOR operations, and arithmetic additions.
- `perform_complex_arithmetic`: Performs a series of bitwise shifts, additions, and multiplications on a global variable and stores results in a table.
- `multiply_and_concatenate_result`: Multiplies the low byte of its argument plus one by a byte from the high word, then returns the product as a 32-bit value with its low word shifted down by eight bits.
- `forward_argument_to_helper`: Passes its argument straight to a helper and returns.
- `execute_retry_loop`: Executes a series of tasks in a loop, attempting to retry operations based on returned status values and updating a global flag.
- `find_highest_priority_task`: Iterates through a table to find an entry with a non-zero pointer and a specific priority byte, then clears that entry and executes the retrieved pointer.
- `search_and_execute_task`: Searches a table for a valid pointer meeting specific byte criteria, clears the entry, and calls the retrieved function.
- `search_and_execute_task_2`: Iterates through a table to find a non-zero pointer with a zeroed byte value, clears the entry, and executes the function.
- `search_and_execute_task_3`: Loops through a table to find a valid pointer based on a byte condition, clears the entry, and calls the associated routine.
- `execute_priority_tasks`: Iterates through a table to find and execute routines based on specific priority values and pointer conditions.
- `assign_structure_pointers`: Iterates through a list of elements and stores the current element's address into a specific table location determined by a type byte.
- `update_pointer_table`: Traverses a linked structure and updates a table of pointers based on a type value found within the structure elements.
- `trace_unresolved_external_entry`: A routine consisting of 46 instructions that is called by two other routines and was reached during the reference run.
- `copy_longwords_until_count_ends`: Copies 4-byte words from a source pointer to a destination pointer, stepping both by one word each time, until the remaining length counter is no longer positive.
- `merge_linked_lists`: Iterates through a linked list and merges it into another list by updating pointers and adjusting size values.
- `traverse_and_process_list`: Traverses a linked list structure, performing operations on each node and updating the list head based on the results.
- `search_linked_list_2`: Searches through a linked list for a node that satisfies a size-based condition and calls another routine with the high words of the found pointers.
- `insert_and_merge_free_block`: Inserts a block into the address-sorted free list, then merges it with the following block when they touch and with the preceding block when they touch.
- `allocate_and_sort_memory_blocks`: This routine attempts to allocate memory blocks of varying sizes through repeated calls to another routine, stores the resulting pointers and sizes in a table, and then sorts the table in descending order using a bubble sort algorithm.
- `release_pool_block_list`: Iterates through a list of free blocks, calling a helper for each entry, and then calls the helper again with a pending block.
- `return_status_after_refresh`: Calls a helper to refresh state, then returns the shared status word.
- `poll_until_status_nonzero`: Repeatedly calls a routine in a loop until it returns a non-zero value, then calls a final routine.
- `increment_busy_counter`: Increments a shared busy counter, and when it changes from zero it sets a shared flag and calls a helper.
- `decrement_busy_counter`: Decrements a global variable and, if it reaches a value below one, resets it to zero and calls another routine.
- `call_helper_with_table_entry`: Looks up a table entry from the high word of its argument and calls a helper with the shared base and that entry's offset plus four.
- `update_sprite_bounding_boxes_2`: Calculates a vertical extent based on a list of values, clears a memory range, and updates a table of values using a loop.
- `compare_recorded_event_with_text`: When the argument is nonzero it reads a stored character from a fixed table, clears it, calls helpers, and compares it with the last recorded event; otherwise it executes several helper calls.
- `check_global_word_and_return`: Calls a routine to retrieve a global word and then returns.
- `loop_while_test_nonzero`: Calls a test helper, and while it returns nonzero calls a second helper and keeps its result.
- `wait_for_nonzero_event`: Calls a helper until it returns a nonzero value, then calls a second helper when that value equals 301 and returns the value.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `return_from_subroutine_2`: The routine performs an immediate return to the calling function.
- `call_helper_then_clear_flag`: Calls a routine and then sets a global variable to zero.
- `take_and_clear_pending_word`: Calls a helper, then returns the shared pending word and resets it to zero.
- `get_global_word`: Returns a global word if it is non-zero, otherwise calls another routine.
- `store_nonzero_field_word`: Stores the argument word into a field eighteen bytes into the record pointed to by the first argument, but only when that word is nonzero.
- `search_and_initialize_table`: Clears a range of memory, then searches a table for an entry matching specific byte criteria to update a global variable.
- `search_table_entries`: Iterates through a table to find an entry matching specific byte criteria and updates a global variable with a value from that entry.
- `update_global_via_pointer`: Checks a bit flag in a global variable and, if set, performs a recursive call before updating that global variable with a byte retrieved from a memory location pointed to by the return value.
- `search_and_set_global`: Searches a table for a matching index, retrieves a byte from that index, and then sets a global variable to one before calling another routine.
- `initialize_global_variable`: Sets a global variable to zero and then to one, and calls another routine if the variable is non-zero.
- `nop_padding_gap`: This routine is empty and consists of a single instruction that does not perform any operations.
- `load_record_header_and_words`: Loads one record at the index given by its argument, with each record thirty-eight bytes long.
- `write_record_bytes_and_columns`: Writes up to two byte fields of the record selected by its first argument, skipping values equal to minus 32768.
- `return_from_subroutine_3`: This routine performs no operations and immediately returns to the caller.
- `draw_palette_entry_via_library`: Rotates a four-entry index through a table, splits the selected 16-bit entry into three nibble fields, and calls a library function twice with those fields and a fixed value of 17, once for each of two bases.
- `read_two_words_from_block`: Copies two words from a fixed block of the shared state into the two output words given by its arguments.
- `call_and_return_2`: Calls another routine and then returns.
- `return_and_clear_shared_word`: Returns the current value of a shared word and sets that word to zero.
- `fetch_and_reset_word`: Reads the value of a global word into a register and resets that global word to zero before returning.
- `update_record_status_flags`: Updates two word fields within a record based on whether a provided byte matches specific values.
- `create_library_surface_pair`: Requests a library resource, stores its handle, and calls a library function twice with fixed sizes and offsets.
- `refresh_library_surface_pair`: Calls a library function once for each of two bases, using the stored handle plus an offset and a specific size.
- `reset_library_surface_offsets`: Calls a library function twice, once for each base, with sizes of 16 by 16 and zero offsets, using the stored handle plus 72.
- `position_library_surface_pair`: Calls a library function twice, once for each base, with sizes of 16 by 16 and offsets derived from a shared byte: its negated value plus one and its negated value.
- `position_library_surface_pair_2`: Calls a library function twice with specific dimensions and offsets derived from a shared byte.
- `reset_surface_offsets`: Calls a routine twice to reset surface offsets for two different bases using a stored handle and specific dimensions.
- `find_region_index_for_value`: Returns the index of the first of three ranges that contains the argument word, using each range's base and width.
- `set_shared_byte_to_constant`: Sets a shared byte to the constant 82.
- `publish_byte_and_set_flag`: Copies a shared byte into a shared word, calls a helper, and then sets a shared byte to one.
- `nop_sequence`: This routine is empty and performs no operations.
- `nop_padding_sequence`: This routine is empty and consists only of padding instructions used to align code or fill gaps in the executable.
- `find_matching_index_3`: Searches a table for an index where specific coordinate values fall within defined ranges and certain flags are set.
- `find_matching_index_2`: Decrements a counter to search a table for an index where multiple coordinate values fall within a specific range.
- `find_matching_index`: Decrements a counter to search a table for an index where multiple coordinate values fall within a specific range defined by another routine.
- `reset_active_entry_flags`: Sets a shared flag and calls a helper, sets both shared references to minus one, and clears a counter.
- `read_sound_size_value`: Reads and returns the contents of a global variable.
- `pass_shared_word_to_helper`: Passes a shared word to a helper and returns.
- `call_and_return_3`: This routine calls another routine and then immediately returns to its caller.
- `return_from_subroutine_4`: The routine performs no operations and immediately returns to the caller.
- `return_word_from_bytes`: The routine builds a 16-bit value from two consecutive bytes pointed to by its parameter and returns it.
- `initialize_resource_and_populate_table`: Attempts to acquire multiple library resources through a series of conditional checks, and upon success, performs a series of configuration calls and populates a sixteen-entry word table with sequential indices.
- `update_global_pointer`: Stores a global variable into a specific offset of another global variable.
- `cleanup_global_resources`: Performs a series of library calls and routine executions to release handles and update a global block.
- `execute_conditional_library_calls`: Performs a sequence of indirect function calls and library calls based on the return values and bit flags of previous operations.
- `release_three_stored_handles`: Calls another routine three times using a global variable as an argument.
- `poll_stored_handles_loop`: Loops up to thirty-two times, checking two stored handles and performing actions based on returned status values.
- `resolve_asset_metadata`: Retrieves and processes metadata from a data structure pointed to by a parameter.
- `check_resource_availability`: Performs several operations involving library calls and pointer arithmetic to determine if a specific resource is available.
- `free_double_bitplane_buffers`: Releases the two 32000-byte bitplane buffers that the allocation routine created.
- `install_callback_and_configure`: Sets two global variables to fixed values, stores a pointer to a global variable and a routine into a control block, and calls a library function.
- `call_library_with_global_pointer`: Passes a fixed argument of 5 and the address of a global block to a library function and returns.
- `build_room_file_name_list`: Walks a linked list of records, copying each name with a trailing colon into a table of up to nine string slots, then looks up a room file by name and sets a failure flag when neither lookup succeeds.
- `format_room_path_string`: Uses a formatter to construct a path string from a table of values, potentially shifting the table elements after processing.
- `update_mode_transition_state`: Clears three counters and a helper value, then compares the new mode with the mode stored last time.
- `call_another_routine_2`: The routine calls another routine and then returns.
- `call_another_routine_3`: The routine calls another routine twice with specific dimensions and offsets derived from a shared byte.
- `copy_plane_with_blitter`: Waits for the blitter, selects one of two plane buffers as source and the other as destination depending on a flag, and programs the blitter pointers, modulo, control and size registers for a copy 200 lines high and 20 words wide.
- `advance_byte_sequence_step`: Flips a buffer selector and runs a routine.
- `remap_sixteen_word_table`: Rebuilds a 16-entry word table in place by replacing each entry with the table value at the index it currently holds, then passes two stored buffers with table offsets to a library routine.
- `expand_changed_bitplane_rows`: Compares a 32-bit word from a source buffer with a previously stored value, and if they differ, packs the bits from the source word into four separate bytes distributed across four destination planes spaced 8000 bytes apart.
- `update_timer_logic`: Modifies a global variable and conditionally calls another routine based on the resulting value.
- `call_four_routines_in_sequence`: Calls four routines one after another with no arguments and returns.
- `call_and_return`: Calls another routine and then returns to its caller.
- `update_sprite_attribute_table`: Iterates through a table of 40 entries to update sprite-related data.
- `compute_row_span_address`: Computes an address from a base table entry plus a row offset and a column offset, then passes that address with a width value to a routine.
- `load_dma_channel_word`: Looks up a record by index and copies its 16-bit field at a fixed offset into a shared word that goes to the DMA control register.
- `load_interrupt_control_word`: Looks up a record by index and copies two 16-bit fields from it into a shared word, the second copy overwriting the first, with the result feeding the interrupt enable and request registers.
- `load_interrupt_request_word`: Looks up a record by index and copies one 16-bit field into a shared word twice, with the result feeding the interrupt enable and request registers.
- `load_word_from_index`: Loads a 16-bit word from a fixed offset within a record identified by an index into a global variable.
- `copy_word_for_register_index`: Uses the index in the data register to look up a record, then copies its 16-bit field at a fixed offset into the shared word that feeds the interrupt request register.
- `or_record_request_words`: Looks up two records by index, ORs their 16-bit fields at the same offset together, stores the result in the shared word and returns it.
- `store_library_result_in_record`: Looks up a record by the index in the data register, calls a library routine through a stored vector, and writes the 32-bit result into a fixed field of that record.
- `call_library_without_result`: Calls a library routine through a stored vector and passes its result back without storing it.
- `return_without_action`: The routine consists of a single return instruction and performs no work, no hardware register access, and no library calls.
- `call_unreached_library_slot`: Calls a library routine through a stored vector, but this block decodes from a gap in the file and is not reached from the program flow.
- `return_no_op`: The routine consists of a single return instruction and performs no work.
- `call_unreached_library_entry`: Calls a different library routine through a stored vector, in a gap of the file that is not reached from the program flow.
- `run_routine_four_times`: Calls one routine four times in a row with no arguments, then returns.
- `execute_routine_four_times`: Calls another routine four times in succession before returning.
- `execute_routine_four_times_2`: Executes a specific routine four times in succession before returning.
- `run_routine_four_times_wrapped`: Calls one routine once, then calls it four more times in a loop, and then once more, with no arguments.
- `call_then_run_four_times`: Calls one routine once, then runs the same routine four times in a loop.
- `update_audio_channel_state`: Updates audio channel data by copying values between buffers, adjusting a length field, and decrementing a byte value if certain conditions are met.
- `reset_audio_channel_fields`: Resets an audio channel record by setting its length field to 124 and clearing a volume-like field, copies a control word from the record, runs a short busy loop, and decrements a shared timer entry when it is set.
- `advance_four_channel_timers`: Iterates through four channel slots to add increment values to position fields and decrement two timer fields, resetting the position fields to zero when a timer reaches zero.
- `clamp_channel_length_field`: Computes two channel fields from a record: the first is raised to at least 124, and the second is limited to a stored maximum and becomes zero when a shared flag is set.
- `run_channel_command_script`: Decrements a delay counter and, once it expires, runs a stored script of offset and value pairs: small offsets are written straight into the channel register area, larger ones count down a per-entry timer, and a terminator either stops the script or restarts it.
- `traverse_null_terminated_string`: Iterates through a null-terminated string and increments a global variable by one after the loop.
- `store_register_to_global`: Stores the value of a register into a global variable.
- `update_sprite_bounding_boxes`: Calculates and updates the minimum and maximum vertical extents for a set of sprite rows.
- `initialize_audio_system`: Allocates memory, executes indirect jumps, and performs several conditional library calls to set up audio resources and a table of pointers.
- `release_audio_device_resources`: Undoes the audio setup by closing the audio device and freeing the request structure and the eight 64-byte blocks, each only when it is present, and clears the setup flag as it goes.
- `refresh_two_indexed_slots`: Passes two indices from its argument to a self-assignment on each of two shared table entries, then calls a routine with both indices.
- `clear_two_indexed_slots`: Clears the shared table entries for two indices from its argument and calls a routine for each index.
- `clear_indexed_slot`: Clears the shared table entry for the index it receives and calls a routine with that index.
- `update_object_slot_descriptors`: Updates two indexed entries in a shared table by copying attribute data from a source structure into new descriptor blocks.
- `run_pending_flag_handlers`: Calls one routine for each of two pending slot indices when that index is not the sentinel value of minus one.
- `release_matching_slot_entries`: Compares two shared slot entries with its argument.
- `save_slot_state_snapshot`: Clears a shared flag, copies three shared slot values into saved copies, calls a routine, and then stores a further shared counter into a saved copy.
- `restore_slot_state_snapshot`: Calls a routine with zero when the first saved slot index is valid, calls it again for the second saved index when that index is valid and different, and then restores a saved counter.
- `manage_object_lifetimes_and_dispatch`: Decrements reference counters in a table and manages the lifecycle of objects by updating their properties and dispatching nested calls.
- `invoke_handler_with_parameter`: Passes its parameter, a stack address, and a function reference to a handler routine and returns.
- `execute_indirect_function`: Fetches a function pointer from a global table and executes it.
- `call_raw_put_char`: Library stub: loads its arguments and the exec library base, calls RawPutChar and returns its result to the caller.
- `process_null_terminated_string`: Iterates through a null-terminated string and calls another routine for each character.
- `return_immediately_2`: This routine performs no operations and returns immediately to its caller.
- `copy_registers_to_table`: Copies several registers and a jump table address into a table and then calls a routine through an indirect pointer.
- `call_indirect_via_offset`: Calls a routine through a pointer located at an offset within the provided parameter.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `get_next_buffer_byte`: Increments a pointer within a structure if it is less than a limit and returns the byte at that pointer.
- `update_buffer_status`: Checks status flags in a structure, potentially iterates through a list of objects, and updates buffer pointers and status bits based on the result of a calculation.
- `search_and_process_table`: Calls a routine to find a null entry in a table and, if found, passes that entry to another routine for processing.
- `find_matching_string_entry`: Iterates through a table of strings to find a match for a provided string, then updates a structure with a retrieved index and a status flag.
- `find_character_in_string`: Searches for the first occurrence of a specific byte within a null-terminated string and returns a pointer to it.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `manage_file_handle_table`: Iterates through a table of structures to find an empty entry or an existing match.
- `read_file_with_error_check`: Attempts to read data from a file and calls a routine to handle potential input/output errors.
- `safe_memory_copy`: The routine copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `process_manx_object`: Allocates a memory block, performs several operations using a pointer to a structure, and conditionally calls functions via offsets within that structure.
- `return_from_subroutine_5`: This routine performs a single return instruction to exit the current execution flow.
- `initialize_manx_object`: Allocates a memory block based on a count multiplied by six bytes and writes the MANX marker into its header.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `parse_command_string`: Parses a string into individual tokens by handling whitespace and quoted substrings.
- `copy_string_with_limit_3`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches a specific limit.
- `copy_string_with_limit_2`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `initialize_window_structure`: Opens a file, retrieves a pointer from a nested structure, and populates a structure with values from a table and a library call.
- `copy_register_to_register`: copies the contents of one data register to another.
- `format_into_buffer_with_result`: Formats its arguments through a formatter routine into the buffer given as its first argument, writes a terminating zero after the output, and returns the formatter's result.
- `store_byte_increment_pointer`: Stores a byte from its argument into the address held in a shared pointer, advances that pointer by one, and returns the stored byte.
- `dispatch_formatted_string_handler`: Passes a formatted string, a stack pointer, and a handler reference to a dispatcher routine.
- `convert_number_to_digit_text`: Performs a division and returns a 64-bit value formed by concatenating the original first input with the quotient left in a register, then recursively calls itself without performing any other operation, while referring to a sixteen-symbol digit table.
- `parse_format_string`: Parses a formatted string containing specifiers for integers, floating-point numbers, and strings.
- `perform_signed_division`: Reads two signed inputs, determines the sign of the result, calls another routine, and returns a 32-bit value combining the result and the second input.
- `perform_signed_division_2`: Reads two signed inputs, determines the sign of the result, calls another routine, and returns a 32-bit value combining the result and the sign.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: Calls itself recursively without performing any other operation.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `forward_call_to_handler`: Invokes a single handler routine and returns without passing any arguments.
- `execute_buffer_and_return`: A 21-instruction routine that calls a routine to return immediately and another to process a data buffer.
- `return_immediately_3`: This routine performs no operations and returns to its caller immediately.
- `increment_and_store_byte_2`: Increments a pointer stored in a global variable and stores a byte from an offset of another pointer into the location pointed to by that in.
- `increment_and_store_byte`: Increments a pointer stored in a global variable and stores a byte from an offset of another pointer into the location pointed to by that incremented pointer.
- `iterate_and_call_loop`: Walks a shared table in steps of 22 bytes, calling one routine with each entry's address until the end of the table is passed.
- `clear_and_traverse_structure`: Recursively traverses a structure based on bit flags, performing linked list removal, file deletion, and file closing before zeroing out the structure's fields.
- `process_buffer_stream`: Manages a data buffer by checking status flags, updating pointers, and writing bytes to a memory location.
- `find_null_entry`: Iterates through a table of 22-byte structures to find the first entry where a specific byte is zero, then clears the first four bytes of that entry.
- `update_status_structure_2`: Checks a value via another routine and updates various fields in a structure based on whether the result is zero or non-zero.
- `iterate_linked_list_nodes_2`: Traverses a linked list by following pointers stored in each node and executes another routine using the node address and a data offset.
- `traverse_and_call_list`: Walks a linked list of nodes, calling another routine with each node and its data offset, and clears the list head when the walk ends.
- `insert_linked_list_node`: Inserts a new node into a linked list by updating pointers and storing a provided value and a global pointer within the new node.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `remove_from_linked_list`: Searches a linked list for a specific node and removes it by updating the pointer of the preceding node.
- `check_table_entry`: Validates an index against a global table and calls another routine if the entry is valid, returning a boolean result.
- `delete_file_wrapper`: Attempts to delete a file and, if the operation fails, retrieves an error code from a library function.
- `write_file_with_error_check`: Attempts to write data to a file and calls a routine to handle potential input/output errors.
- `invoke_conditional_callback`: Calls a routine with a specific size, and if a bit in the return value is set, calls a function from a global variable if it is not zero.
- `invoke_pointer_and_forward`: Invokes a stored global function pointer when it is set, then forwards a shared four-byte block and its size to another routine, passes the returned value onward, and finally calls a third routine before returning.
- `call_function_pointer`: Invokes a stored global function pointer when it is set, then calls another routine before returning.
- `process_global_state_updates`: Runs one update pass by calling a helper for each entry of a table, invoking a stored callback when set, clearing or writing several shared pointers, and when a status bit is set calling a supervisor-level library routine before finishing.
- `nop_padding_sequence_2`: This routine is empty and consists of instructions that perform no operation.
- `close_file_handle`: Validates an index into a table of file structures, calls a routine to close the file if a specific bit is not set, and then clears the entry in the table.
- `call_close_2`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir_2`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delete_file_2`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `call_input_2`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err_2`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock_function`: Calls a function via a pointer located at a fixed offset from a global variable.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `call_open_2`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output_2`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read_2`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek_2`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `execute_indirect_function_2`: Loads a function pointer from a global variable and executes it.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_write_2`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `execute_indirect_function_3`: Loads a function pointer from an offset within a global variable and executes it.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_request_block`: Initializes a new block of memory by storing several parameters and a pointer to another routine into the allocated space.
- `free_request_block`: Frees a request block: when its link field is set it first frees the chained block, then writes a fill byte to a field, stores a fill word at another offset, and invokes two other routines on the block.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `execute_indirect_jump`: Loads a function pointer from a global variable using a specific offset and executes it.
- `execute_indirect_function_4`: Loads a function pointer from a global variable using a calculated offset and jumps to it.
- `execute_indirect_jump_2`: Fetches a function pointer from a global variable using a different offset and executes it.
- `execute_indirect_function_5`: Fetches a function pointer from a global variable using an offset and executes it.
- `execute_indirect_function_6`: Loads a function pointer from an offset within a global variable and executes it.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `execute_indirect_jump_3`: Retrieves a function pointer from a global variable via a calculated offset and jumps to it.
- `execute_indirect_function_7`: Fetches a function pointer from a global variable using a calculated offset and executes it.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `execute_indirect_function_8`: Loads a function pointer from a global variable using a specific offset and executes it.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `execute_indirect_function_9`: Loads a function pointer from a location offset from a global variable and executes it.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_function_2`: Loads a library base from a global variable and calls a function at a specific offset within that library.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_function_3`: Calls a function located at a specific offset from a library base stored in a global variable.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_function_4`: Executes a function at a fixed offset relative to a library base retrieved from a global variable.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_function_5`: Uses a global variable to find a library base and calls the function at the corresponding offset.
- `call_library_function_6`: Retrieves a library base from a global variable and performs a call to a function at a specific offset.
- `call_library_function_7`: Calls a library function by calculating an offset from a base address stored in a global variable.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_function_8`: Performs an indirect call to a function at a specific offset from a library base held in a global variable.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
