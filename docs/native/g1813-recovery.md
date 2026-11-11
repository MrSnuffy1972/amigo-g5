# Game 1813 (darkseed v1.0 (1992)(cyberdreams)(de)[cr toxic - prelux](disk 1 of 7)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01813). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (947024 top-level bytes in 30 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 45 raw-word lines |
| Functions named and described | pass, 387 of 387 named (0 author symbols), 387 of 387 described (27 by rule, 312 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1313 of 1313 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11306 of 11374 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 510614 |
| code | 39652 |
| data | 359826 |
| padding | 6724 |
| unused | 30208 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Darkseed v1.0 (1992)(Cyberdreams)(De)[cr Toxic - Prelux](Disk 1 of 7).adf:KillAGA | 2 | 1377 | 26 |
| Darkseed v1.0 (1992)(Cyberdreams)(De)[cr Toxic - Prelux](Disk 1 of 7).adf:tPX! | 2 | 267 | 8 |
| Darkseed v1.0 (1992)(Cyberdreams)(De)[cr Toxic - Prelux](Disk 1 of 7).adf:INSTALL | 1 | 454 | 20 |
| Darkseed v1.0 (1992)(Cyberdreams)(De)[cr Toxic - Prelux](Disk 1 of 7).adf:DS.exe | 13 | 233 | 7 |
| Darkseed v1.0 (1992)(Cyberdreams)(De)[cr Toxic - Prelux](Disk 1 of 7).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7796 | 250 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1207 | 74 |

## Cases handled

- darkseed v1.0 (1992)(cyberdreams)(de)[cr toxic - prelux](disk 1 of 7)/Darkseed v1.0 (1992)(Cyberdreams)(De)[cr Toxic - Prelux](Disk 1 of 7).adf:N03B: high-entropy data file (7.73 bits/byte), classified as packed data
- darkseed v1.0 (1992)(cyberdreams)(de)[cr toxic - prelux](disk 1 of 7)/Darkseed v1.0 (1992)(Cyberdreams)(De)[cr Toxic - Prelux](Disk 1 of 7).adf:N03C: high-entropy data file (7.66 bits/byte), classified as packed data
- darkseed v1.0 (1992)(cyberdreams)(de)[cr toxic - prelux](disk 1 of 7)/Darkseed v1.0 (1992)(Cyberdreams)(De)[cr Toxic - Prelux](Disk 1 of 7).adf:B3: high-entropy data file (7.74 bits/byte), classified as packed data
- darkseed v1.0 (1992)(cyberdreams)(de)[cr toxic - prelux](disk 1 of 7)/Darkseed v1.0 (1992)(Cyberdreams)(De)[cr Toxic - Prelux](Disk 1 of 7).adf:M00A: high-entropy data file (7.74 bits/byte), classified as packed data
- darkseed v1.0 (1992)(cyberdreams)(de)[cr toxic - prelux](disk 1 of 7)/Darkseed v1.0 (1992)(Cyberdreams)(De)[cr Toxic - Prelux](Disk 1 of 7).adf:M01A: high-entropy data file (7.67 bits/byte), classified as packed data
- darkseed v1.0 (1992)(cyberdreams)(de)[cr toxic - prelux](disk 1 of 7)/Darkseed v1.0 (1992)(Cyberdreams)(De)[cr Toxic - Prelux](Disk 1 of 7).adf:M03A: high-entropy data file (7.63 bits/byte), classified as packed data
- runtime segment an address (play-rs-m1500): 333 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 22 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 190 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- darkseed v1.0 (1992)(cyberdreams)(de)[cr toxic - prelux](disk 1 of 7)/Darkseed v1.0 (1992)(Cyberdreams)(De)[cr Toxic - Prelux](Disk 1 of 7).adf:DS.exe: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `run_trackdisk_io_request` (INSTALL): Adds a message port, opens the trackdisk device, sends an I/O request through the exec library, then removes the port and closes the device again. Kind: disk/track integrity check. Pass path: the branch where iVar2 is set to 0 and the loop executes (the 'success' path that clears a flag and returns a status code).
- `process_string_and_execute_chain` (runtime segment an address (play-rs-m1500)): Performs a series of string parsing, register increments, and function pointer executions, including clearing a global buffer and updating a global variable. Kind: manual/code-wheel lookup. Pass path: The loop that parses the protection text string and the subsequent execution of the chain if the string-based check (looking for specific characters/patterns) succeeds or fails, specifically leading to the execution of the code-wheel/manual.
- `open_trackdisk_device_and_request` (runtime segment an address (play-rs-m1500)): Opens the trackdisk device through library calls, adds and removes a message port, sends an I/O request and returns the result. Kind: track/sync check or manual/code-wheel lookup. Pass path: the branch where iVar2 != 900 (the loop exits or bypasses the check, or the condition (iVar2 != 900) or the loop-based check fails to trigger the error state).
- `execute_device_io_sequence` (runtime segment an address (play-rs-m1500)): Performs a sequence of operations using function pointers retrieved from a global variable, involving device-related library calls. Kind: track/sync check. Pass path: tst.l d0; beq.b an address.

## Functions


### Darkseed v1.0 (1992)(Cyberdreams)(De)[cr Toxic - Prelux](Disk 1 of 7).adf:KillAGA

- `initialize_program_startup_state`: Program entry point.
- `release_resources_and_reply`: Stores a parameter in a task record and calls an optional callback when one is set, then calls two other routines.
- `dispatch_dos_request_by_code`: Calls another routine and then dispatches on a request code.
- `return_immediately`: The routine executes a return instruction immediately upon entry, performing no other operations.
- `return_immediately_2`: The routine executes a return instruction immediately upon entry and performs no other operations.
- `measure_string_length_calling_routine`: Finds the end of a zero-terminated string, calls another routine for each character, calls one more other routine and returns the length of the string.
- `free_linked_list_nodes`: Frees every node of one linked list through the exec memory-free call and clears its head pointer.
- `allocate_record_or_set_error`: If a record's flag bit is clear, calls another routine to allocate its buffer and stores the result in the record.
- `write_to_dos_handle`: Checks the flags of a file handle and, when the handle is writable, writes data through the dos library, looping while writes succeed.
- `check_handle_and_write`: Checks the handle returned by another routine: a null handle records an invalid-argument error and returns minus one.
- `seek_dos_handle_checked`: Checks that a file handle is set and stores an invalid-argument error if not.
- `close_file_nodes_in_list`: Marks the state as set, then walks a linked list of file nodes.
- `parse_command_and_redirect_io`: Splits a command line into up to thirty-two arguments by skipping spaces, tabs and newlines, then opens the NIL device through the dos library for input and output, and looks up the current task through the exec library.
- `copy_name_and_find_task`: Copies up to seventy-nine bytes of a stored string into a local buffer, terminates it and looks up the current task through the exec library.
- `copy_name_and_print_break`: Copies up to seventy-nine bytes of a stored name into a local buffer, looks up the current task through the exec library and writes a break notice through the dos library.
- `find_node_by_index`: Returns the list node at a given index.
- `check_break_signal_state`: Reads the task's signal state through the exec library and tests the two break-signal bits.
- `find_byte_in_buffer`: Scans a buffer of a given length for a target byte, returning a pointer to its first occurrence or null when it is not found.
- `append_bounded_string`: Finds the terminating zero of a destination string and appends characters from a source string, stopping at the source's zero byte or when a count is used up, then writes a terminating zero.
- `copy_bounded_string_padded`: Copies a zero-terminated source string into a destination up to a count and then fills the rest of the count with zero bytes, as in a fixed-width string copy.
- `allocate_rounded_block_from_pool`: Rounds a requested size up to a multiple of four and checks it against a limit.
- `run_callbacks_then_close_nodes`: Calls each routine in one list of callbacks, then runs another routine on matching entries of another list, sets a state flag and processes a list of file nodes, closing files or calling a stored callback for each.
- `write_bytes_into_file_buffer`: Checks flag bits in a file node.
- `reset_node_or_check_all`: With no node given, walks a list of file nodes and calls another routine for each active node, returning a failure if any call fails.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `call_and_return`: The routine calls another routine and then returns.

### Darkseed v1.0 (1992)(Cyberdreams)(De)[cr Toxic - Prelux](Disk 1 of 7).adf:tPX!

- `process_executable_hunks`: Program entry for a loader that compares executable hunk type codes and processes each hunk, allocating and freeing memory through the exec library and calling other routines.
- `decrement_word_past_zero`: Repeatedly decrements a 16-bit data register, stopping after it has passed through zero to negative one, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_run_length_bytes`: Decodes a run-length compressed byte stream backward into a buffer.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `read_bits_from_backward_stream`: Shifts a bit buffer by a bit count and refills it with a word read backward from an input pointer when it empties, computing a bit from the bits shifted out and a flag.
- `stub_bad_instruction_data`: Calls a cache-invalidation trap and then runs into bad instruction data, so the routine is a stub with no usable logic in the decompilation.

### Darkseed v1.0 (1992)(Cyberdreams)(De)[cr Toxic - Prelux](Disk 1 of 7).adf:INSTALL

- `run_installer_entry_sequence`: Installer entry point.
- `call_library_vector_48_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_48`: Calls the library function at a fixed offset from the library base held in a global variable and returns its result.
- `copy_system_commands_to_ram`: Makes six calls to a library function, with text that copies system command files such as cd, copy, delete, execute and makedir into the RAM disk.
- `delete_ram_commands`: Makes five library calls, with text that deletes the copied command files from the RAM disk.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `read_line_with_backspace`: Reads input characters through library calls, waiting until one arrives.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `build_name_and_write_file`: Copies a name into a buffer, appends a second block of bytes from the same buffer, then opens a file through the dos library, writes to it and closes it.
- `find_first_passing_slot`: Calls other routines for up to four slots in turn and returns the index of the first slot whose checks pass, or minus one when none does.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `setup_device_request_for_unit`: Stores a unit number, finds the current task and calls library routines to set up a device request, storing an error code when setup fails.
- `run_trackdisk_io_request`: Adds a message port, opens the trackdisk device, sends an I/O request through the exec library, then removes the port and closes the device again.
- `verify_stored_device_record`: Stores a unit value, makes two library calls and compares a twelve-byte record for that unit with expected values, returning zero when they match and minus one otherwise.
- `compare_table_data`: The routine calls several functions via pointers, then compares values from a table against global variables to return either zero or a specific constant.
- `validate_disk_data_checksum`: Calls several functions via a pointer table to retrieve values and then compares a set of data from a table against specific global variables.
- `verify_stored_device_record_2`: Stores a unit value, makes two library calls and compares a twelve-byte record for that unit with expected values, returning zero when they match and minus one otherwise.
- `verify_stored_device_record_3`: Stores a unit value, makes two library calls and compares a twelve-byte record for that unit with expected values, returning zero when they match and minus one otherwise.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `examine_locked_disk_path`: Locks a path, examines it and unlocks it through the dos library, working with the labels darkseed_1 and df0.

### Darkseed v1.0 (1992)(Cyberdreams)(De)[cr Toxic - Prelux](Disk 1 of 7).adf:DS.exe

- `run_program_startup_calls`: Program entry point.
- `decrement_until_minus_one`: Repeatedly decrements a data register, starting from its existing value, until it reaches negative one, then returns.
- `accumulate_memory_values`: Iterates through a loop to check bit flags and adds values from a source to a destination memory block using an unrolled copy-like mechanism.
- `shift_and_test_bit`: Shifts a byte left and checks the carry flag or sign bit.
- `copy_memory_block_unrolled`: copies a block of memory from a source address to a destination address using an unrolled loop and a decrementing counter.
- `copy_memory_block`: copies a block of memory from a source address to a destination address using a decrementing counter.
- `test_global_bit`: tests a bit or a sign bit of a global variable against a register value.

### Darkseed v1.0 (1992)(Cyberdreams)(De)[cr Toxic - Prelux](Disk 1 of 7).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_engine_subsystems`: Initializes a table of function pointers by sequentially calling routines via pointers found at an offset from a register and stores the returned addresses into several global variables.
- `initialize_library_pointers`: Sequentially calls routines via pointers found at an offset from a register and stores the returned addresses into several global variables.
- `return_immediately_empty_stub`: Returns straight away without doing any work.
- `execute_conditional_function_chain_2`: Recursively calls itself if a certain global byte is non-zero, then executes a sequence of function pointers stored in a list.
- `search_byte_pair_and_set_flag`: iterates through a table or buffer looking for a matching pair of bytes, and if found, sets a bit in a byte at a fixed offset from a pointer and updates a global variable to a constant.
- `set_global_flag_and_value`: Calls a routine, sets a bit in a byte at an offset from a pointer, and sets a global variable to a constant.
- `update_record_bytes`: Updates certain bytes in a record or a global structure using a value returned from another routine.
- `update_global_value`: Updates a global variable or modifies it based on a bitset or a hardware register bit.
- `update_global_bitset`: modifies a global variable or a hardware register bit via a call to a routine that updates a global value.
- `update_string_parsing_state`: Updates a global pointer and state flags while checking string delimiters and range-validating memory pointers.
- `check_range_and_set_values`: Checks if a value is within a certain range and, if so, updates several memory locations with specific constants.
- `set_flag_if_set`: Checks a bit in a byte at an offset from a pointer and sets a bit in another byte if the first bit was set.
- `toggle_global_array_bit`: Performs an XOR operation on a value in a global array and, if a certain global variable equals a constant, toggles a bit in a byte at an offset from a pointer-indexed location.
- `return_from_subroutine_2`: the routine is empty and performs no operations before returning.
- `calculate_split_value`: If a global variable exceeds a certain threshold, it returns a value composed of the remainder and a quotient derived from a division operation.
- `calculate_scaled_value`: Calculates a value by multiplying a byte from a table by a constant and returns a concatenated result.
- `store_pointer_contents`: Reads bytes from a memory location pointed to by a register and stores them into several global variables.
- `modify_bitmask_and_recursively_branch`: Sets specific bits in a memory location and an entry in a table, then conditionally executes itself or another routine based on the value of a global variable or a character check.
- `modify_bit_in_table`: calculates an offset into a table and sets a specific bit in a word at that location, then conditionally executes another routine based on the value of a global variable.
- `branch_on_byte_value`: Checks if a byte in a global variable equals a certain value and calls another routine recursively or otherwise based on that value.
- `call_then_clear_flag_bit`: Calls a subroutine, then clears bit zero of the byte at offset five in the record reached through a pointer.
- `call_then_set_flag_bit`: Calls a subroutine, then sets bit zero of the byte at offset five in the record reached through a pointer.
- `call_then_toggle_flag_bit`: Calls a subroutine, then inverts bit zero of the byte at offset five in the record reached through a pointer.
- `update_state_and_recalculate_index`: Updates a global state variable and a table entry based on conditional bitwise modifications and pointer-based offsets.
- `update_record_fields_2`: This routine updates a record or a set of records by performing scaled subtraction and bitwise modifications, including clearing or setting bits and updating byte-sized or word-sized values within a record structure.
- `update_record_values`: Writes a word and a short to a record and calculates a new value for a record field using a scaled subtraction.
- `update_record_with_offset_value`: Clears a bit in a record, then conditionally sets a bit and updates a byte in that record with a new value.
- `step_point_to_free_cell`: Steps one point toward a target in increments taken from two record bytes, repeating while the bitmap cell it lands on is occupied, and returns the final coordinates.
- `select_nearest_unmarked_entries`: Computes the Manhattan distance from a point to each three-word entry of a global table, then repeatedly selects the nearest entry not yet marked in a bitmask byte of the record, updating a global variable with the selected index.
- `calculate_step_vector`: Calculates a vector or distance between two points and updates a global variable with a constant value.
- `halve_record_bytes`: Sets a byte in a record to one and then shifts two record bytes right by one bit if a certain condition is met.
- `halve_two_step_bytes`: Shifts two record bytes at offsets twenty-four and twenty-five right by one bit, halving them, and returns the input value unchanged.
- `probe_spiral_for_blocked_cell`: Walks outward in a spiral from a start cell, testing bitmap bits inside a 252 by 228 area, and returns the input pair as soon as it finds an occupied cell; it gives up after a fixed step limit, and returns early when a global flag is unable to be checked.
- `modify_record_data`: Iterates through a memory range, replacing certain bytes with a new value.
- `restore_removed_slots_with_byte`: Walks a terminated three-byte table and writes the input byte into every slot previously marked as removed, then calls two subroutines if any slot was restored.
- `update_record_pointers`: Clears certain bits in a memory range and updates a pointer-based structure with new word values.
- `call_then_set_record_bits`: Calls a subroutine, then sets bits three and two in the byte at offset thirteen of the record at A0.
- `update_global_state`: Calls several subroutines and conditionally modifies a byte or a global variable based on a certain value.
- `double_table_entry_plus_offset`: Looks up a byte in the global table at an index formed from the second input byte plus one hundred fifty-two, doubles it, and returns that value added to the first input word.
- `execute_conditional_state_logic`: Checks a byte or global variable against specific constants to execute different sequences of subroutines, modifies record flags, and iterates through a table-driven loop that updates a global variable and certain memory-resident pointers or values.
- `toggle_byte_and_recurse`: Calls a routine and then toggles the least significant bit of a byte at a location pointed to by a register.
- `toggle_global_byte`: Stores a specific byte value into a global variable, calls another routine, and then stores a different byte value into a global variable.
- `execute_and_loop`: Calls a routine and then enters an infinite loop.
- `toggle_global_byte_2`: Toggles a bit in a global byte by performing an XOR operation with one.
- `clear_global_variable_2`: Sets a global variable to zero and enters a loop that waits for a register to become zero.
- `modify_record_bits`: Checks a global variable and, if it meets certain criteria, clears a bit in a record and calls another routine.
- `set_byte_and_loop`: Sets a byte at an offset from a pointer and enters an infinite loop.
- `update_state_and_copy_instructions`: Updates a sequence of instructions by copying a block of memory and then modifies a global variable or table entry based on a conditional check of a global value.
- `execute_recursive_call`: Calls itself and then executes another routine.
- `update_record_fields`: Calls several routines while clearing a bit in a byte, setting a bit in a byte, and copying words or bytes between record locations.
- `update_global_pointers_2`: Loads a value from a table, performs a bitwise shift and OR, and copies a word from one table location to another before clearing a global variable.
- `copy_and_modify_record_data`: Modifies a byte in a record via a bitwise operation, then copies a word from one record location to another.
- `search_table_for_byte_value`: Scans a terminated table of three-byte records, comparing the third byte of each record with the input byte, and returns as soon as a match is found.
- `append_entry_and_update_count`: Calls a subroutine, writes the value 254 into a byte two before A0, stores the input byte in a table at the current counter index, sets a companion byte to the counter minus six or zero, and increments the counter.
- `remove_table_entry_by_value`: Calls a subroutine and clears a byte two before A0, then searches a byte table for the input value; when found, it shifts the following entries down by one, decrements the counter and adjusts a companion byte, otherwise it returns unchanged.
- `modify_and_copy_record_data`: Modifies a byte in a record via a bitwise operation, copies a word from one record location to another, and clears a bit in a record.
- `clear_global_bit`: Clears a specific bit in a global variable and clears a bit in a byte located at an offset from a global pointer.
- `clear_flag_and_set_byte`: Clears bit zero of a byte in a record and sets a byte at a certain offset to zero.
- `modify_flag_and_set_buffer_chars`: Clears a bit in a global flag, resets a byte in a resource buffer, and writes a sequence of ASCII characters to a buffer via repeated calls to other routines.
- `initialize_resource_buffer_and_map_data`: Allocates a memory block and initializes it with a descending and then ascending byte pattern.
- `toggle_and_branch_on_global`: Toggles a bit in a global variable and conditionally calls another routine based on a global variable's value.
- `patch_instruction_sequence`: Executes a sequence of routines and writes specific values into a sequence of instruction addresses.
- `update_subroutine_pointers`: Updates a sequence of instructions with values from a table and toggles a global variable bit within a loop.
- `iterate_and_call_functions`: Iterates through a list of function pointers and executes each one until a counter reaches a certain value.
- `expand_bitplane_bits_to_lookup_bytes`: Walks the bits of a multi-plane bitmap from the top bit down, gathering the same bit position from four consecutive source words into a 4-bit index per plane, then stores the lookup-table byte for each index into three output buffers.
- `update_subroutine_pointer`: Stores a byte from a table into a location in a sequence of instructions.
- `update_subroutine_pointer_2`: Updates a pointer in a sequence of instructions with a value from a table.
- `update_subroutine_pointer_3`: Writes a value from a table into a specific instruction address.
- `update_subroutine_pointer_4`: Sets an instruction or pointer to a value retrieved from a table.
- `generate_string_table_2`: Allocates memory and populates a sequence of memory locations with values derived from a source table.
- `generate_string_table`: calculates a sequence of values based on nibble-summing or bit-manipulation of a source table and writes a sequence of incrementing characters or a lookup table to a fixed memory area.
- `update_bitmask_from_string_diff`: Compares two strings and, if a difference is detected, updates a set of bitmasks or flags based on values retrieved from a lookup table.
- `tiny_ori_2`: Small routine of 1 instruction that merges 1 into d0.
- `clear_global_variable`: Sets a global variable to zero.
- `clear_bit_and_set_flag_2`: Clears the least significant bit of a byte in a global variable and then sets a flag or a value in a global variable.
- `tiny_suba`: Small routine of 1 instruction that subtracts a1 from a1.
- `clear_bit_and_set_flag`: Clears the least significant bit of a byte in a global variable and then sets a flag or a value in a global variable.
- `loop_with_bit_set`: Executes a loop that waits for a global variable to reach a certain value, and if a certain condition is met, sets a bit in a global variable.
- `format_string_buffer`: Converts an integer to a string in a fixed buffer and writes a specific character to a memory location based on a conditional check.
- `toggle_flag_and_assign`: Toggles a bit in a global variable and conditionally assigns a value from a memory location to a global variable.
- `return_constant_value`: The routine returns a constant 32-bit or 64-bit immediate value, depending on the register width, or a fixed bitmask, without performing any logic or accessing memory.
- `empty_loop_routine`: The routine is empty and contains an infinite loop.
- `execute_infinite_loop`: Executes a routine and then enters an infinite loop.
- `clear_global_bits`: Iteratively clears the least significant bit of a byte in a global structure.
- `load_global_values`: Copies two words from a global structure into two other global variables.
- `validate_and_update_pointer_target`: Checks if a value pointed to by a register matches a specific constant and updates it, then writes a byte to an offset of that pointer and updates a global variable.
- `clear_global_state`: Sets a global variable to zero and clears a bit in a byte at a specific offset in a global structure.
- `toggle_global_flag_and_recurse`: Toggles a global flag using a bitwise XOR and recursively calls itself a fixed number of times while calling two other routines.
- `validate_pointer_and_set_value`: Checks if a pointer target matches a constant and, if not, sets a global variable and a byte to a specific value.
- `execute_sequence_with_loop_2`: Executes a sequence of routines and enters a loop that checks a global variable against a constant.
- `advance_position_and_call_branch`: Sets a pointer-indexed word to a new value and calls subroutines, potentially branching based on a global variable.
- `reset_timing_words_and_state`: Sets a global variable to zero, calls several subroutines, and initializes several fixed global locations with specific values.
- `initialize_resource_buffer_with_pattern`: Validates a resource header and initializes a memory block with a descending and then ascending byte pattern.
- `validate_and_initialize_resource`: Checks a resource header for a specific magic signature and, if valid, initializes a memory buffer with a descending and then ascending byte pattern before copying data if certain bitmask conditions are met.
- `toggle_global_bit_8`: Calls a routine and then performs an XOR operation on a global variable with a constant value of one.
- `copy_global_word`: Calls another routine and copies a word from a global structure to a location pointed to by a register.
- `iterate_and_update_memory`: Iterates through a memory range, calling a routine with a decrementing counter and an incrementing pointer.
- `initialize_global_values`: Sets a global variable to zero and calls several subroutines.
- `clear_global_bitset`: Sets a global variable to zero, clears a bit in a byte at an offset from a pointer, and sets a global variable to a constant.
- `reset_global_variable`: Sets a global variable to zero and then calls another routine multiple times before setting a global variable to a constant.
- `tiny_move_ori_ori`: Small routine of 3 instructions that copies d4 to -(a1), then merges 0 into d3, then merges 0 into d0.
- `update_state_machine_branch`: updates a global state variable and jumps to a new instruction address based on the current value of a register.
- `tiny_move_ori_ori_2`: Small routine of 3 instructions that copies a6 to d2, then merges 0 into d5, then merges 10 into d0.
- `update_state_machine_branch_2`: Updates a global state variable and jumps to a new address based on the value of a byte-sized input or flag.
- `update_state_and_jump_target`: Updates a global variable and sets a jump target or return address based on the value of a byte.
- `empty_routine`: The routine is empty and performs no operations before returning.
- `load_scaled_byte`: loads a byte from a table using a scaled index into a global variable.
- `store_scaled_byte`: stores a byte from a table using a scaled index into a global variable.
- `dispatch_instruction_and_update_state`: Checks a byte or word against specific constants and executes a sequence of other routines that modify or clear global flags and reset timing or state variables.
- `execute_subroutine_and_return`: Calls a routine and then returns.
- `modify_record_and_loop`: Modifies a global variable, copies a word from a record, and executes a loop that decrements a counter while calling another routine.
- `modify_byte_and_clear_pointer`: Modifies a bit in a byte at an offset from a pointer, sets a pointer to zero, and clears a bit in a global variable.
- `iterate_with_counter`: Executes a sequence of routines while incrementing a register until a certain value is reached.
- `calculate_pointer_offset`: Loads two pointers from a table, uses a value from a register to index into that table, and performs arithmetic to update a second pointer.
- `recursively_decrement_counter`: Recursively calls itself while decrementing a register until a certain value is reached.
- `update_global_with_alternating_values`: Updates a global variable with two different constant values while performing recursive calls to itself and calling another routine.
- `process_input_string_and_state`: Parses an input string or character stream, updating a global buffer and managing a state machine that handles backspaces, newline or special control characters, and conditional pointer or global variable updates.
- `process_input_character`: Checks an input byte against certain characters and, if a match is found, searches a table and potentially updates a record or calls a routine.
- `return_scaled_global_index`: returns the value of a global byte multiplied by one hundred sixty-four as an integer.
- `return_global_times_seven`: returns the value of a global byte multiplied by seven as an integer.
- `copy_and_reformat_table`: Performs a fixed-length copy of bytes, followed by a stride-based copy of a table, and concludes by updating several global variables with data from a specific memory location.
- `copy_and_expand_table`: Iterates through a source table to copy or expand data into a destination, followed by a fixed-length copy of words and a series of global variable assignments.
- `copy_and_reformat_data`: Copies data from a source table using a stride, then performs a fixed-length copy of words and updates several global variables with values from a specific memory location.
- `copy_variable_stride_data`: Copies bytes from a source to a destination, where the amount of bytes skipped between copies is determined by a value read from a source buffer.
- `copy_memory_with_variable_stride`: Copies bytes from a source to a destination, where the amount of bytes skipped between copies is determined by a value read from a source buffer and a register.
- `tiny_move_ori_ori_ori`: Small routine of 4 instructions that copies (a6)+ to (a5), then merges 0 into d3, then merges 28 into d0, then merges 237 into (a3)+.
- `update_record_flags`: Iterates through a list of pointers, clearing a bit in a record and toggling several flags or bytes at offsets relative to those pointers.
- `convert_to_string_fixed_buffer`: converts an integer to a string of characters and appends a character, writing to a fixed memory location.
- `convert_to_string_pointer`: converts an integer to a string of characters and appends a character, writing to a location provided via a pointer.
- `update_global_state_and_records`: Updates a global variable and certain record fields based on a comparison of two bytes, then conditionally executes a loop that writes a sequence of calculated values into a memory range.
- `execute_and_clear_memory`: Calls several subroutines, writes a constant to a memory location, and then loops to clear memory at a pointer-indexed location while calling another routine.
- `reset_global_state`: checks a global variable against specific values and, if a condition is met, calls another routine and resets the global variable to a certain value.
- `fetch_byte_from_table`: Loads a word from a global variable to use as an offset, calculates an address relative to the program counter, and loads a byte from that address into a register.
- `reset_global_state_2`: Checks a global variable against a value, resets it, and writes a specific word to a memory location before calling other routines.
- `restore_global_variable`: Restores a global variable to its previous value before returning.
- `lookup_char_class_and_dispatch`: Takes the byte in the global character register, maps comma to a substitute code and the percent and greater-than symbols to the dollar code, then scans two sorted lookup tables for a matching entry and handles any match.
- `tiny_andi`: Small routine of 1 instruction that masks d0 with 0.
- `perform_empty_operation`: Performs a logical OR operation on a register with zero.
- `search_resource_tables`: Checks a global variable against values in a table and calls another routine if a match is found.
- `update_resource_metadata`: Reads a byte from a pointer, updates several global variables, and performs conditional checks on a table or array before writing a value from a pointer to a global structure.
- `call_grid_of_counts`: Calls a subroutine once for each cell of a grid whose column and row counts are read from a pointer in a register.
- `call_single_routine_return`: Calls one routine and returns; it adds nothing else.
- `update_resource_metadata_and_search_table`: Checks a global flag to perform a one-time initialization, then updates a global array or structure based on a value read from a pointer offset.
- `process_grid_counts`: Iterates through a grid or list, potentially writing a value to a global table and searching through a list of bytes.
- `execute_sequence_of_routines`: Executes a sequence of several subroutines and then assigns a routine address to a global variable.
- `execute_multiple_subroutines`: Executes a sequence of several subroutines and one additional routine.
- `execute_subroutine_sequence`: Executes a sequence of several subroutines.
- `search_and_copy_words`: Searches a list of words for a match and, if a certain condition is met, copies a word from a calculated offset to a memory location.
- `copy_sixteen_words`: Copies sixteen words from a source pointer to a destination pointer.
- `invert_and_store_bytes`: Calculates the bitwise complement of bytes from a source pointer and stores them in a destination pointer.
- `process_string_and_execute_chain`: Performs a series of string parsing, register increments, and function pointer executions, including clearing a global buffer and updating a global variable.
- `execute_conditional_function_chain`: Increments a global variable and executes a sequence of routines or function pointers based on the value of a global variable.
- `process_input_string`: Clears a global buffer with spaces and then reads characters from a source, handling backspaces by decrementing a pointer and replacing carriage returns or backspaces with spaces.
- `parse_string_buffer`: Parses a string buffer, replacing certain characters with spaces and handling backspaces or carriage returns while updating a global pointer.
- `store_uninitialized_value`: Stores the contents of an uninitialized register or memory location into a global variable.
- `copy_word_block`: copies a sequence of words from a source pointer or a register-relative offset to a fixed destination address, decrementing a counter until a certain number of words have been moved.
- `copy_word_block_2`: copies a sequence of words from a source pointer or a register-relative offset to a destination address in a fixed-size buffer.
- `copy_word_block_fixed`: Copies a fixed number of words from a source pointer offset by a certain amount to a destination address.
- `tiny_moveq`: Small routine of 1 instruction that loads 15 into d5.
- `process_nibble_masking`: Iterates through a buffer, masking and shifting bits to update a source buffer with a calculated value.
- `copy_word_block_4`: Copies a sequence of words from a source address to a destination address using a loop.
- `copy_word_block_3`: copies a sequence of words from a source pointer offset by a fixed amount to a destination address.
- `modify_nibble_in_block`: Iterates through a block of words, comparing and potentially modifying the lower nibble of each word based on a difference calculation.
- `blend_nibble_pairs_toward_each_other`: Walks two 16-bit word arrays one word at a time, and for each of four 4-bit nibble pairs moves the first array's nibble one step toward the second array's nibble when they differ by more than the threshold of 15, writing the adjusted result back into the first array.
- `modify_nibble_values`: Iterates through a block of words, comparing and modifying the lower nibble of each word based on the difference between source and destination values.
- `update_global_value_2`: Checks a global variable against several constant values and increments or decrements a register accordingly.
- `stub_return_only`: The routine performs a self-assignment of a global variable and returns.
- `update_window_dimensions`: Calculates and clamps a new window or or area boundary based on a previously called routine's return value.
- `return_empty_routine`: the routine performs no operations and immediately returns to its caller.
- `copy_words_to_address`: Copies a fixed number of words from a source pointer to a destination address.
- `tiny_ori_3`: Small routine of 1 instruction that merges 2 into d1.
- `iterate_and_store_result`: Decrements a counter and stores the return value of a routine into a pointer-indexed memory location until a zero return or counter expiration occurs.
- `clear_memory_on_loop`: Decrements a counter and sets memory at a pointer-indexed location to zero while calling another routine, provided the pointer-indexed value is non-zero.
- `return_without_work_stub`: The pseudo-code shows only a bare return, but the routine is reported to contain 8 instructions, so the decompiler output does not fully account for its body.
- `set_structure_flag`: Writes a constant value to a structure field at an offset from a register and returns a value from the stack.
- `clear_memory_blocks`: Iterates through a list of pointers and clears a fixed number of words at each location to zero.
- `initialize_global_pointers`: Sets a global variable to a routine address and sets another global variable to zero before entering an infinite loop.
- `copy_displaced_memory_blocks`: Copies blocks of bytes from a source or a source offset by a certain amount to a destination buffer.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 84 into d1, then merges 20 into d1.
- `set_global_constant`: Sets a global variable to a specific constant value.
- `tiny_ori_4`: Small routine of 1 instruction that merges 237 into d0.
- `set_constant_long`: Sets a global variable to a fixed 32-bit constant value.
- `set_constant_long_alt`: Sets a global variable to a different fixed 32-bit constant value.
- `copy_memory_with_offset`: Iterates through a list of pointers and copies a block of bytes from a source offset by a certain amount to a destination.
- `tiny_ori_ori_3`: Small routine of 2 instructions that merges 212 into d1, then merges 100 into d1.
- `execute_conditional_logic`: Checks a character and, depending on its value, executes a sequence of function pointers and copies a constant long value into a global variable.
- `execute_recursive_loop`: Executes a loop that calls another routine and checks a global variable, potentially calling another routine if a register contains a non-zero value.
- `copy_record_fields`: Checks if a word at an offset in a record is zero, then copies a word, a byte, and another word from a source record to a destination record.
- `reset_loop_state`: Clears a global variable and enters a loop that executes a sequence of subroutines and conditional checks until a routine returns zero.
- `execute_sequence_with_loop`: Executes a sequence of routines and checks a global counter, potentially entering an infinite loop if a certain pointer is null.
- `initialize_loop_and_state`: Clears a global variable and enters a loop that executes certain routines until a function returns zero, then sets a global variable to a constant.
- `initialize_resource_buffer`: Checks for a valid resource header and, if found, initializes a memory buffer by clearing it and then performing a bit-mask-based copy or bit-set operation.
- `initialize_resource_buffer_2`: Checks for a valid resource header, then initializes a memory buffer by filling it with a descending or ascending sequence of bytes.
- `initialize_string_buffer`: Initializes a memory buffer by filling it with a descending sequence of bytes followed by an ascending sequence of bytes.
- `initialize_memory_buffer`: Clears a block of memory by filling it with a descending sequence of bytes and then a sequence of zeroed bytes.
- `initialize_resource_buffer_3`: Checks a resource header for a specific magic signature and, if valid, initializes a memory buffer by filling it with a descending byte pattern followed by an ascending byte pattern, then copies a bitmask-filtered or direct stream of bytes from the resource into the buffer.
- `reset_global_pointer`: Sets a global variable to zero and calls a routine if a certain global variable is non-zero.
- `set_global_state`: Sets a global variable to a specific value and assigns a value to another global variable if certain conditions are met.
- `poll_until_non_zero_set_state`: Repeatedly calls another routine until it returns a non-zero value, then sets a global variable to a specific byte and subsequently to one.
- `reset_global_pointer_conditionally`: Calls a routine, sets a global pointer to zero, and if a certain global variable is non-zero, calls another routine and resets that variable.
- `execute_subroutine_and_clear_pointer`: Calls a subroutine and then sets a global pointer to zero.
- `execute_conditional_routine`: Checks if a global variable is non-zero and, if so, calls another routine.
- `check_global_and_call`: Checks if a global variable is non-zero and calls another routine if it is.
- `call_then_mask_word_bits`: Calls a routine and then returns the input word with bits six and seven cleared.
- `execute_two_routines`: Calls two other routines sequentially.
- `mask_upper_bits`: Clears the two most significant bits of a word and returns the result.
- `execute_two_subroutines`: Executes two other routines sequentially while updating a global variable with a constant value.
- `call_pair_poll_until_zero`: Calls two subroutines, then repeatedly calls a third until it returns zero.
- `search_list_for_value`: Iterates through a list of memory locations, comparing a byte at an offset to a value, and returns if a match is found.
- `store_result_if_positive`: Calls a routine and, if a counter or value is positive, stores the result of another routine into a memory location offset by a certain amount.
- `call_second_if_nonnegative`: Calls a subroutine, then calls a second one only when the input word is not negative.
- `free_list_elements`: Iterates through a list of pointers, calls a free routine on any non-zero pointer, and clears the pointer in the list.
- `search_and_update_buffer`: Iterates through a table or memory block to find a matching value.
- `format_string_buffer_2`: Allocates memory and writes a string or integer-converted string into a buffer, potentially copying bytes or checking a magic value in memory before updating a pointer or buffer content.
- `allocate_and_initialize_resource_buffer`: Allocates a memory block and initializes it with a descending and then ascending byte pattern.
- `copy_memory_block_3`: Copies a block of memory from a source pointer to a destination pointer using a loop.
- `copy_memory_block_2`: Copies a block of memory from a source pointer to a destination pointer, potentially calling another routine to update pointers or handle certain conditions.
- `increment_and_update_pointer`: Increments a byte at an offset, updates a pointer, and copies a value from a table if a certain condition is met; otherwise, it resets a value and calls a routine.
- `frag_moveq_lea_move_bmi_addq_rts`: Short code fragment that loads 0 into d4, then loads the address of a fixed memory address into a0, then copies an indexed entry from a0 to d0, then branches if the last result was negative, then loads the address of a fixed memory address into a0, then copies an indexed entry from a0 to d4, then adds 1 to d4, then returns.
- `update_address_and_check_condition`: Decrements a register and conditionally increments a global address or resets a global variable based on a comparison.
- `process_string_table`: Iterates through a table of bytes, checking for specific delimiters and updating a global counter or pointer.
- `format_numeric_string`: Converts a global variable to an ASCII string and increments a global counter and a certain memory location.
- `convert_byte_to_string`: Converts a byte value into two ASCII characters and stores them in a memory buffer.
- `set_constant_values`: Increments a global variable and sets a global variable to a constant value.
- `increment_registers`: Increments a global variable and another global variable, and sets a third global variable to a constant value.
- `set_hardware_registers`: Checks a value, then sets a global variable to a constant and updates hardware registers with constant values.
- `parse_string_buffer_length`: Iterates through a string buffer, counting characters and checking for space delimiters.
- `process_string_buffer`: Reads a string from a table, increments a counter, and updates a global variable or a counter if a certain length or space character is encountered.
- `parse_string_with_bounds`: Iterates through a string, updating a counter and a global address, and resets certain values if a space or a length limit is encountered.
- `update_blt_registers`: Calculates or retrieves values and writes a series of constants or calculated values into a set of hardware registers or memory locations.
- `setup_blt_parameters`: Calculates a value based on a global variable and writes several constants or calculated values into hardware registers or memory locations.
- `setup_blt_transfer`: Sets several hardware registers to specific values and then loops through a list to write certain values into those registers.
- `return_constant`: Returns a constant 64-bit value.
- `set_register_constant`: Writes a constant value to a global variable.
- `set_global_and_recurse`: stores a constant value into a global variable and then calls itself twice.
- `initialize_blt_state`: Sets a global variable to a constant, resets a certain memory location, and calls other routines.
- `set_multiple_registers`: Sets a global variable to a constant and calls other routines.
- `modify_byte_and_toggle_flag`: Performs a bitwise OR on a byte at an offset from a pointer and toggles a global flag.
- `toggle_global_bit`: toggles the least significant bit of a global variable using a bitwise XOR operation.
- `toggle_global_bit_2`: Toggles the least significant bit of a global variable using an XOR operation.
- `toggle_global_bit_3`: Performs an XOR operation on a global variable with a constant value of one.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `toggle_global_bit_4`: Flips a single bit in a global variable via an XOR instruction.
- `toggle_global_bit_5`: Executes an XOR instruction to invert a bit in a global variable.
- `toggle_global_bit_6`: Modifies a global variable by XORing it with one.
- `toggle_global_bit_7`: Uses an XOR instruction to change the value of a global variable.
- `toggle_and_reset_state`: Toggles a bit in a global variable and resets another global variable to zero.
- `toggle_and_loop`: Toggles a bit in a global variable and enters an infinite loop.
- `initialize_resource_buffer_4`: Sets a global variable to a fixed constant, calls several other routines, and writes a constant value to a global variable before returning a bitwise modified value.
- `set_global_values`: Sets a global variable to zero, sets another global variable to one, calls several subroutines, and returns a value masked with a constant.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_string_with_counter`: Iterates through a source string, copying bytes to a destination and incrementing a global counter if a specific byte value is encountered, or stopping if a terminator is found.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_named_file_exists`: Attempts to open a file using a pointer-based function and closes it if successful, setting a global flag to one if found or zero otherwise.
- `load_string_from_table`: Checks a file flag and, if certain conditions or a specific value in a global variable are met, copies a sequence of bytes from a table into a destination buffer.
- `execute_device_sequence`: Checks a global flag, calls a function pointer, and if successful, copies a block of bytes from a source to a destination and updates several global variables.
- `tiny_ori_ori_4`: Small routine of 2 instructions that merges 237 into d0, then merges 0 into d0.
- `execute_conditional_function`: Checks a bitmask and calls a function pointer retrieved from a global pointer, or retries the call if a certain condition is met.
- `call_library_function`: Calls a function pointer retrieved from an offset relative to a global pointer.
- `execute_offset_function`: Checks a global variable and calls a function pointer retrieved from a fixed offset of a global pointer, or retries if a certain condition is met.
- `call_function_pointer`: Calls a function pointer retrieved from a global variable.
- `call_indirect_function`: Calls a function pointer retrieved from an offset of a global variable.
- `call_indirect_function_offset`: Calls a function pointer retrieved from a different offset of a global variable.
- `retry_attempts_up_to_four`: Attempts to call a routine up to four times, returning the index of the first successful attempt or minus one if all fail.
- `tiny_ori_5`: Small routine of 1 instruction that merges 4 into d0.
- `update_global_pointers`: Updates several global pointers and checks a condition while potentially looping or clearing a value in a table.
- `open_trackdisk_device_and_request`: Opens the trackdisk device through library calls, adds and removes a message port, sends an I/O request and returns the result.
- `fill_zero_data`: Consists of a sequence of twelve logical OR operations with zero.
- `tiny_ori_ori_ori_ori`: Small routine of 4 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `tiny_ori_ori_ori_ori_2`: Small routine of 4 instructions that merges 0 into d0, then merges 112 into d2, then merges 0 into d0, then merges 108 into d2.
- `tiny_ori_6`: Small routine of 1 instruction that merges 3 into d0.
- `fill_zero_data_2`: Consists of two logical OR operations with zero.
- `execute_device_io_sequence`: Performs a sequence of operations using function pointers retrieved from a global variable, involving device-related library calls.
- `verify_file_lock_matches_entry`: Locks a named file through the DOS library, examines it and unlocks it.
- `calculate_scaled_value_2`: Calculates a scaled value using a global variable and certain offsets from a pointer, then stores the result in a structure.
- `calculate_scaled_value_full`: Performs a complete calculation involving scaling a global variable and storing a clamped result into a pointer-based offset.
- `find_valid_table_index`: Performs a series of checks against a table using a calculated index, potentially masking a byte and returning a pointer to a table entry.
- `store_byte_masked_with_two`: It masks a byte received in a register with the value two and stores the resulting byte through a pointer.
- `copy_global_variable`: Loads a value from a global variable into a register and stores that value into another global variable.
- `store_pow2_mask_and_flag`: Computes a power-of-two mask from a register value and stores it into a structure, sets a flag byte, and conditionally writes a constant into a hardware register.
- `reset_structure_and_set_mask`: Checks a byte in a structure, decrements a value, resets several fields in a structure, and sets a bitmask in a global structure.
- `init_struct_and_toggle_flag`: Initializes several fields of a structure with fixed values, then toggles and clears specific bits of a flag byte.
- `parse_control_command_stream`: Parses a stream of control characters to modify a state structure, including updating certain flags, adjusting a delay or timing value, and recursively processing further commands or characters.
- `process_control_character_command_2`: Processes a stream of control characters or command bytes to update a state structure, modify a global table, or recursively call itself.
- `process_control_character_command`: Checks a register for a specific control character and performs conditional updates to a structure or a global table.
- `update_byte_from_char`: Checks a register for a specific character and writes a modified version of another byte to a memory location.
- `process_command_characters_extended`: Checks a register for certain characters and either modifies bits in a byte or calls itself recursively.
- `set_bit_at_offset`: Sets a specific bit in a byte located at an offset from a pointer.
- `process_command_characters`: Checks a register for specific characters and modifies bits in a byte at an offset or calls itself recursively.
- `modify_bits_in_loop`: Iterates through a list of pointers, checking if a pointer matches a global variable and then clearing specific bits in a byte at an offset.
- `process_character_commands`: Evaluates a character in a register and either calls itself recursively or modifies bits and bytes at an offset from a pointer.
- `clear_bits_and_set_byte`: Clears a specific bit in a byte at an offset and stores a constant byte into another memory location.
- `handle_character_commands`: Checks a register against certain characters and either calls itself recursively or copies a byte from one pointer-indexed location to another, or modifies a bit in a byte at an offset from a pointer.
- `empty_recursive_routine`: The routine is empty and performs no operations before returning, though it is called in a way that would cause infinite recursion if executed.
- `process_character_side_effects_2`: Checks a register for a specific character and either copies a byte or modifies a bit in a byte at an offset from a pointer, or recursively calls itself.
- `process_character_side_effects`: Checks a character in a register and either copies a byte or modifies a bit in a byte at an offset from a pointer, or recursively calls itself if certain other characters are detected.
- `update_flags_on_char`: Checks a character in a register and modifies a bit in a byte located at an offset from a pointer if certain characters are detected.
- `update_structure_flags`: Checks a register for specific character values and modifies a bit in a structure field at an offset from a pointer.
- `update_flags_on_char_match`: If a register contains certain characters, it sets or clears a bit in a structure field and calls itself recursively.
- `recurse_on_char_match`: If a register contains a specific character, it clears a bit in a structure field and calls itself recursively; otherwise, it calls itself if the register contains another specific character.
- `check_char_and_recurse`: Checks if a register contains a specific character and calls itself if so.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `multiply_32bit_fixed_point`: The routine takes two 32-bit fixed-point values from registers and returns a 32-bit fixed-point product.
- `allocate_and_register_audio_server`: Allocates a block for the sound driver through the exec library, carves it into per-channel records at fixed offsets and registers an interrupt server for the audio interrupt; returns one if the allocation fails.
- `release_audio_server_and_memory`: Removes the audio interrupt server and frees the sound driver's memory block through the exec library, clearing the stored pointers.
- `close_audio_device`: Clears a bit in a global variable, iterates through a table of structures to clear flags, and calls a function pointer to close a device.
- `update_list_flags`: Iterates through a list of pointers, checking if a pointer matches a certain address, and then modifies a flag or bit in a nearby structure.
- `process_list_with_flags`: Loops through a linked list of structures, setting a flag byte and conditionally setting a bit while advancing pointers until a counter reaches zero.
- `decrement_and_clear_matching_structure`: Loops through a table of pointers, and if a pointer matches a provided address, calls another routine to decrement a counter and clear certain fields.
- `iterate_structures_and_toggle_flags`: Loops through a table of structures to call a routine if a field is non-zero, then loops through a second set of structures to clear a flag bit and set a byte to its maximum value.
- `update_flag_from_input`: Reads a hardware register for input state, then sets or clears a specific bit in a global variable based on that value.
- `check_global_flag_bit`: Checks a flag bit in a global variable and returns one when the bit is clear, otherwise returns zero.
- `update_function_result`: Calls a function pointer via a global variable and stores the returned value into a field of a second pointer or updates a pointer in a register.
- `execute_indirect_function`: Calls a function pointer located at an offset from a register.
- `free_indexed_list_entries`: Detaches the linked list held in the table slot chosen by the index argument, clears that slot, and releases each entry through a free-memory call.
- `release_slot_record_buffers`: Releases two buffers attached to a record, clears their pointer fields, and clears the record's first field.
- `reset_slot_table_state`: Performs a series of delay loops and clears a table of entries, freeing any active memory-allocated buffers and resetting certain global counters.
- `validate_and_populate_string_table`: Checks a memory buffer for a specific magic signature and, if valid, iterates through a list of pointers to populate a table with string or pointer data.
- `verify_and_populate_string_table`: Checks a memory buffer for a specific magic signature and, if valid, iterates through a list of pointers to populate a table with string or property data while checking for duplicates or null entries.
- `populate_object_property_table`: Iterates through a list of objects to populate a table with properties or pointers, checking for null or duplicate entries while updating a counter or index.
- `populate_object_property_table_2`: Iterates through a list of objects to populate a table with properties or pointers, checking for null or duplicate entries while updating a counter.
- `populate_scaled_data_table`: Iterates through a loop to calculate or retrieve values and store them into a table or array at an offset determined by a scaled index.
- `populate_scaled_table`: Iterates through a loop to calculate or retrieve values and store them into a table or array at an offset determined by a scaled index.
- `call_function_on_zero`: If a register is zero, it calls a routine located at an offset from a global pointer and returns one.
- `call_function_on_zero_false`: If a register is zero, it calls a routine located at an offset from a global pointer and returns zero.
- `signal_software_interrupt_if_flagged`: When a global flag byte is nonzero it raises an exec software interrupt through a cause call; otherwise it returns at once.
