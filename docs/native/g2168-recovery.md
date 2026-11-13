# Game 2168 (black crypt (1992)(electronic arts)[cr trsi](disk 1 of 3)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02168). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 27 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 544 of 544 named (0 author symbols), 544 of 544 described (34 by rule, 178 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 250 of 250 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16168 of 16171 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 654708 |
| code | 51280 |
| data | 186199 |
| padding | 6885 |
| unused | 2048 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 458 | 12 |
| Stack | 2 | 117 | 2 |
| avail | 11 | 376 | 12 |
| runback | 3 | 709 | 39 |
| endcli | 2 | 182 | 3 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:TRSI! | 1 | 116 | 3 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:InstallCrypt | 3 | 3235 | 57 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:BlackCrypt | 3 | 2384 | 104 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bcdfp | 3 | 6602 | 223 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bcdfq | 2 | 1120 | 70 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bcdft | 5 | 160 | 15 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- black crypt (1992)(electronic arts)[cr trsi](disk 1 of 3)[a]/Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:TRSI!: CrunchMania Addr (xfd) stream found by ancient scan (13088 -> 32446 bytes) but only 0 of 7 sampled windows are in reference RAM: not used
- black crypt (1992)(electronic arts)[cr trsi](disk 1 of 3)[a]/Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bcdft: CrunchMania Normal (xfd) stream found by ancient scan (83200 -> 180004 bytes) but only 0 of 86 sampled windows are in reference RAM: not used
- black crypt (1992)(electronic arts)[cr trsi](disk 1 of 3)[a]/Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:TRSI!: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- black crypt (1992)(electronic arts)[cr trsi](disk 1 of 3)[a]/Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bcdft: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_then_call_pointer`: When the destination pointer is non-null, it copies sixteen 32-bit words from a fixed block in the address register area into that destination, advancing both pointers each step.
- `initialize_entity_structure`: Initializes a large structure by zeroing various offsets, fetching values from tables into specific offsets, and performing a loop to copy a sequence of bytes from one memory location to another.
- `update_entity_attribute_from_table_2`: Fetches a byte from a table using a value from offset 0 of a structure, stores it at offset 20 of that structure, and calculates a new value for offset 24.
- `update_entity_attribute_from_table`: Calculates a target offset in a structure by adding a value from offset 12 to a constant and stores it at offset 16, then calculates a source offset from offset 20 and stores it at offset 80.
- `update_entity_state_and_call_handler`: Updates a structure in a0 by setting a byte at an offset calculated from a1, then copies several longwords from a structure in a1 to a structure in a0.
- `branch_to_offset`: Adds the value at offset 84 of a1 to 1 and then branches to a different location.
- `test_longword_and_branch`: Tests a longword at offset 40 of the structure in a1 and branches to another routine if the value is non-zero, otherwise it branches to a third routine.
- `modify_bitmask_in_structure`: Performs a bitwise AND between a longword in d1 and a longword at offset 48 of a1, and if the result is non-zero, performs a bitwise OR on the value at offset 48 of a1 with a mask.
- `update_entity_pointer_and_call_subroutine`: Copies a longword from offset 64 to offset 108 of a structure, prepares arguments for a subroutine call using values from offset 48, 52, and 132 of two different structures, and stores the returned longword into offset 28 of the first structure before conditionally branching.
- `reset_entity_state_and_update_pointers`: Updates a pointer in a table using values from a structure, calls two routines, and clears several fields in a structure at a calculated offset in a0.
- `update_structure_and_decrement_counters`: Calls several routines using values from a structure in a2, stores a longword from d1 into a global variable, and conditionally decrements a value at an offset in a table pointed to by a0.

### Stack

- `fill_table_from_list_maximum`: Walks a list of entries to find the largest value in one field, then makes two calls through a function pointer.
- `copy_stack_words`: Copies a fixed number of words from a source pointer to a destination pointer on the stack.

### avail

- `setup_task_and_open_libraries`: Allocates a record and returns error code 20 if that fails.
- `store_task_slot_and_release`: Stores its argument in a slot of the current task record and returns that same argument.
- `sum_sizes_of_matching_entries`: Walks a linked list of entries and adds up the difference between two fields of each entry whose flag word shares at least one bit with the argument.
- `print_memory_usage_report`: Prints a memory table with available, in-use, maximum and largest columns, and a usage line naming the chip, fast or total memory.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `forward_args_with_stack_slot`: Passes its two arguments on to another routine together with the address of a fixed stack slot, then returns.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `measure_buffer_and_pass_on`: Makes one indirect call through a global function pointer, then counts the characters in a 140-byte stack buffer up to its null terminator.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### runback

- `call_one_unnamed_routine`: The routine is a single instruction that passes control to one other routine.
- `start_program_in_background`: Parses a command line with an optional load delay and a program name with parameters, then starts that program as a background task.
- `clear_table_and_store_pointers`: Zeroes forty 32-bit words in a block whose offset is computed from a stack argument.
- `call_flagged_hook_then_continue`: Checks a global pointer and, when it is non-zero, calls the function it points to.
- `open_resources_and_init_slots`: Opens resources through library-style calls and stores their handles in global slots, calling an error routine if the first open fails.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `loop_ten_then_dispatch_globals`: Calls one routine ten times in a loop, then checks several global slots.
- `append_string_to_destination`: Appends a zero-terminated source string to the end of a destination string, copying at most 32767 bytes.
- `append_string_with_count_limit`: Appends bytes from a source string to the end of a destination string, stopping at the source terminator or when the count argument runs out.
- `copy_string_to_destination`: Copies a zero-terminated source string into a destination buffer, including the terminator, and returns the destination pointer.
- `measure_zero_terminated_string`: Walks a zero-terminated byte string and returns how many bytes come before the terminator.
- `copy_string_with_zero_padding`: Copies bytes from a source string into a destination for a count taken from a 32-bit argument, stopping at the first zero byte.
- `release_indexed_slot_entry`: Looks up an entry in a table of twenty slots by index; an out-of-range index or empty slot stores error code 3 and returns minus one.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_function_102`: Loads a library base from a field in the global block and calls the routine 102 bytes below that base, then returns.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_function_84`: Loads a library base from a field in the global block and calls the routine 84 bytes below that base, then returns.
- `call_library_function_30`: Loads a library base from a field in the global block and calls the routine 30 bytes below that base, then returns.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_function_60`: Loads a library base from a field in the global block and calls the routine 60 bytes below that base, then returns.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_function_90`: Loads a library base from a field in the global block and calls the routine 90 bytes below that base, then returns.
- `call_library_function_48`: Loads a library base from a field in the global block and calls the routine 48 bytes below that base, then returns.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_function_198`: Loads a library base from a field in the global block and calls the routine 198 bytes below that base, then returns.
- `call_indirect_function`: Loads a function pointer from a global table and executes it.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_function_132`: Loads a library base from a field in the global block and calls the routine 132 bytes below that base, then returns.
- `call_library_function_210`: Loads a library base from a field in the global block and calls the routine 210 bytes below that base, then returns.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### endcli

- `scan_list_and_update_header`: Iterates through a linked list to find the maximum value in a specific field and then writes a header block containing this value and a sequence of incrementing integers.
- `copy_fixed_size_data`: Copies a fixed number of longwords from a source pointer to a destination pointer.
- `update_state_and_jump`: Modifies a value in a2, updates a pointer in a1, calls a routine in a5 multiple times with various parameters from a2 and a1, decrements a value in a table at a0, and finally performs an indirect jump using a6.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:TRSI!

- `call_three_routines_at_entry`: A program entry of 91 instructions that calls three other routines.
- `select_half_or_input_word`: Returns the first input word unchanged unless a second argument equals one, in which case it returns half of a third word.
- `mask_word_with_table_entry`: Looks up a 16-bit entry in a fixed word table, indexed by the second argument, and returns that entry ANDed with the first argument.

### Port-Handler

- `setup_hunk_record_from_table`: Fills a record from a table entry chosen by the first argument, sets several following pointer and size fields from computed addresses, and branches on the record's type value to pick one of three base offsets.

### Disk-Validator

- `format_counter_as_digit_char`: A program entry of 386 instructions that checks a counter field and, when it is ten or more, calls a routine and turns the result into a digit character; otherwise it uses a space.

### Ram-Handler

- `copy_memory_then_call_pointer`: Calls the exec CopyMem routine through a global library base, then makes an indirect call to a target it could not recover from the jump table, and returns.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:InstallCrypt

- `clear_global_block_and_init`: Zeroes eighty 32-bit words in a block of global state and stores a register value into a field.
- `process_intuition_message_loop`: Takes a lock on the current directory, calls several library routines, and then loops over messages from a port.
- `open_graphics_and_intuition_libs`: Clears five global handle slots and opens the graphics and intuition libraries through the exec base, storing each handle and calling an error routine if an open fails.
- `draw_install_path_prompt`: Draws the install path prompt with graphics calls that move the pen, set its colour and write text.
- `show_failure_message_and_close`: Shows an installation failure line with graphics text and a short delay when the result is above 400, then closes the window and releases any handles still set.
- `copy_name_then_append_colon`: Copies a name from an indexed table entry into a buffer, using the length stored in the entry and substituting zero when that length is above 33.
- `scan_entries_for_failure`: Walks a linked list of entries and, for each entry with a non-zero flag, calls a routine with a stored field and a second routine with the argument.
- `prompt_insert_disk_and_wait`: If a check on the given path fails, it draws a prompt asking the user to insert a disk, with a short delay, and copies the path name into a buffer ending in a colon.
- `copy_file_with_progress_display`: Examines a source file, then copies it to a destination named from a table entry, opening, reading and writing with the dos library.
- `install_disks_and_write_config`: Drives the install sequence: it checks the destination with the dos library, creates directories, and copies disk contents through open, read and write calls.
- `forward_arg_with_stack_slot`: Passes its argument to another routine together with a fixed block address and the address of a stack slot, then returns that routine's result.
- `copy_byte_array`: Copies a given number of bytes from a source pointer to a destination pointer, one byte at a time, and returns the destination pointer.
- `compare_zero_terminated_strings`: Compares two zero-terminated byte strings and returns one when the first sorts after the second, minus one when it sorts before, and zero when they are equal.
- `return_without_action`: The routine does nothing and returns at once, with no calls and no effect on its arguments.
- `open_standard_streams_with_tag`: Allocates a block with a library call and returns early if that fails.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `parse_quoted_arguments_into_buffer`: Allocates a buffer with a library call and, when that succeeds, copies a name and an argument string into it.
- `append_string_with_count_limit_2`: Appends a source string to the end of a destination string, copying at most the count given in the third argument.
- `read_icon_window_entry`: Opens a library and, when the lookup succeeds, calls icon library routines to read a window entry and store its values in the caller's record.
- `append_string_to_destination_2`: Appends a zero-terminated source string to the end of a destination string, with no length limit, and returns the destination pointer.
- `append_string_to_buffer_2`: Finds the end of a string and appends a second string to that location.
- `append_string_to_null_terminated_string`: finds the end of the first string and copies the second string to that location, effectively appending it.
- `copy_string_or_pad`: Copies characters from a source string into a destination buffer up to a count given as the third argument.
- `multiply_32_bit_values`: Multiplies two 32-bit register values modulo 2 to the power 32, using 16-bit partial products.
- `format_string_with_percent_codes`: Copies literal bytes from a format string into a destination buffer and expands percent codes using flags, widths and digit tables for upper and lower case number output.
- `calculate_string_length_2`: Calculates the number of bytes between the start of a string and its null terminator.
- `call_routine_per_table_entry`: Walks a table of 22-byte records between two fixed global addresses and calls a routine on each record in turn.
- `write_byte_to_buffered_record`: Writes one byte to a buffered record after checking its status flags, and a null record gives minus one.
- `close_record_and_reset_fields`: Closes a valid record that has a non-zero status, calling routines to flush or release its buffer, and writes a set numeric field as five decimal digits first.
- `close_record_and_reset_fields_2`: Closes a record with a non-zero status, calling routines to flush or release its buffer, and writes a set numeric field as five decimal digits first.
- `return_zero_value`: Returns zero without doing anything else, and it makes no calls.
- `tiny_addq_addq`: Small routine of 2 instructions that adds 2 to a5, then adds 8 to d0.
- `apply_to_record_or_all`: With no argument, it walks the record table and calls a routine with a minus-one argument on each active record that lacks certain flag bits, returning minus one if any such call returns minus one.
- `scan_record_table_for_failure`: Scans the record table from its start and calls a routine with a minus-one argument on each active record that lacks certain flag bits.
- `forward_call_and_return`: Calls one routine and returns without using its result or changing anything else.
- `call_and_return_pair`: Calls another routine and returns a 64-bit pair made from the original second register and the value the call left in the second register.
- `apply_sign_around_call`: Records the sign of its two register arguments, calls another routine on the values, and negates that result when the signs call for it.
- `negate_result_by_sign`: Calls another routine and then negates its second result when the first register argument is negative.
- `recursive_call`: The routine calls itself indefinitely without performing any other operation.
- `divide_32_bit_quotient_remainder`: Divides a 32-bit dividend in the first register by a 32-bit divisor in the second.
- `init_buffer_record_fields`: Initialises a record's fields, calling another routine to choose the buffering mode.
- `unlink_and_free_list_entry`: Searches a global linked list for the given entry and unlinks it when found, then calls a library routine on it.
- `walk_list_and_clear_head`: Walks a global linked list and makes a library call for each entry with the same argument register, then clears the list head and returns zero.
- `traverse_and_clear_list`: Iterates through a linked list, calling a function pointer for each node, and then clears a global variable.
- `allocate_and_link_list_record`: When given a non-zero argument, it allocates a record through a library call and, if that succeeds, links the record onto the front of a global list and stores the argument in it.
- `copy_string_simple_2`: Copies characters from a source string to a destination buffer until a null terminator is reached.
- `check_entry_is_interactive`: Checks a numbered table entry: an out-of-range or empty index stores error code 3 and returns minus one.
- `seek_entry_and_record_error`: Checks a numbered table entry, storing error code 3 and returning minus one when the index is out of range or the entry is empty.
- `delete_file_or_record_error`: Calls the dos library DeleteFile routine with its argument.
- `return_zero_2`: The routine is empty and simply returns a zero value to the caller.
- `write_record_and_report_error`: Checks a numbered table entry, storing error code 3 for an invalid index or code 6 when the entry's mode bits are clear.
- `check_flag_bits_and_return`: Calls a library routine that returns a flag word and tests bit 4096.
- `write_global_word_to_output`: Writes the four bytes of a global variable to the standard output through the dos library, makes one more library call, and calls another routine with the argument one.
- `run_list_hooks_then_finish`: Walks a global linked list, calling a function pointer stored four bytes into each entry and advancing to the next entry.
- `release_resources_and_unlock`: Releases global resources in order: when a table exists it calls a routine for each index up to a global count, then calls library routines that free and release handles and records.
- `do_nothing_2`: The routine is empty and performs no operations.
- `close_indexed_entry_handle`: Checks a numbered table entry, storing error code 3 and returning minus one when the index is out of range or the entry is empty.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:BlackCrypt

- `clear_state_block_and_start`: Zeroes sixty-seven 32-bit words of global state, stores a register value in a field, and tests a flag bit that may call a library routine.
- `handle_workbench_and_disk_loop`: Closes and reopens the Workbench interface around a lock on the game disk volume, taking the current directory first.
- `call_routine_and_branch`: Sets d2 to negative 1, calls a routine at an offset from the program counter, and then branches to another routine.
- `push_value_and_call`: Calls a routine, pushes the word 4 onto the stack, pushes a longword from an offset of the program counter onto the stack, calls another routine, cleans the stack, and branches to another routine.
- `push_value_and_call_alternate`: Calls a routine, pushes the word 5 onto the stack, pushes a longword from a different offset of the program counter onto the stack, calls another routine, cleans the stack, and branches to another routine.
- `prompt_disk_and_dispatch_loop`: Calls a routine to prompt for a disk insertion, updates a status structure with a code of 6, and then enters a loop that evaluates a status code from a structure to perform various system operations including loading segments, delaying, or unloading segments.
- `dispatch_event_loop_handler`: Executes a loop that evaluates a status code from a structure to perform various system operations including loading segments, delaying, unloading, and handling disk insertion or input events.
- `execute_system_sequence`: Calls two routines, copies a 16-bit word from d0 to d2, and branches to another location.
- `reset_and_reload_segment`: Calls two routines, clears a longword at offset 82 of a structure pointed to by a4, pushes two addresses onto the stack to call a third routine, sets a longword at offset 82 of the structure to 33799, and branches to another location.
- `dispatch_event_loop`: Enters a loop that evaluates a status code from a structure to perform various system operations including loading segments, delaying, unloading segments, or polling input events.
- `open_screen_window_and_config`: Clears a set of global handles and opens the graphics and intuition libraries, then loads a configuration file and sets up bitmap and colour data.
- `close_screen_window_and_free`: Closes the screen and window handles when their global fields are set, calling intuition routines and library free routines through stored base pointers.
- `delay_then_unload_segment`: Sets a status value of six in a record, makes one library call, and passes a stored handle to another routine.
- `poll_input_events_until_done`: Loops calling an input routine and switches on the event type.
- `dispatch_and_load_process`: Calls a dispatch routine, then checks a byte at an offset of a structure in a6; if certain conditions are met, it calculates a new value by shifting and adding 16384, stores it in a global variable, and calls a routine to load a segment and create a process.
- `clear_register_and_branch`: Clears register d2 and branches to another location.
- `execute_jump_table_call`: Pushes a value onto the stack, calls another routine, cleans the stack, and branches to another location.
- `branch_after_call`: Calls a routine, clears d2, and branches to an earlier instruction in this routine.
- `execute_jump_table`: Performs a conditional jump based on a comparison and uses a table to determine the destination of a jump.
- `copy_uppercase_name_then_colon`: Copies a name from an indexed table entry into a buffer, using the stored length and substituting zero above 33.
- `scan_entries_for_failure_2`: Walks a linked list of entries and, for each entry with a non-zero flag, calls a routine with a stored field and a second routine with the argument through a 120-byte stack buffer.
- `prompt_insert_disk_wait_messages`: Returns a stored value when a global flag is set; otherwise it checks the path and, when that fails, copies the path into a buffer with a colon, shows the PLEASE INSERT DISK prompt with a delay, and loops over port messages until a message of class 32768 triggers another check.
- `wait_for_library_result`: Repeatedly calls one library routine, then reads a second library routine's result, and loops until that result is non-zero.
- `load_segment_and_create_process`: Loads a code segment and, when that succeeds, creates a process from it, storing the process and segment handles and clearing three fields.
- `set_entry_status_six`: Looks up an entry through a subroutine, stores its field into a global slot, and sets its status to six after comparing its state with a value from the caller's record.
- `load_run_and_unload_segment`: Loads a segment through the dos library and, when the load succeeds, calls a function pointer stored in a global with arguments derived from the segment's length.
- `call_routine_per_character`: Makes two calls, then calls another routine once for each character of a zero-terminated string argument before returning.
- `do_nothing_routine`: The routine is empty and performs no operations before returning.
- `call_routine_per_character_2`: Calls one routine with its argument, then calls another routine once for each character of a zero-terminated string argument before returning.
- `call_two_routines_in_order`: Calls two routines one after the other and returns, without any other work.
- `clear_blit_area_then_call`: Calls one routine, then makes a call through a global block pointer at minus 300 bytes and returns.
- `write_menu_labels_then_return`: Calls two routines with fixed numeric arguments several times, then returns five.
- `copy_menu_tiles`: Calls call_routine_per_character_2 and call_routine_per_character multiple times using various word constants and pointers to process several strings.
- `render_save_menu`: Calls two different character routines with specific pointers and constants, then checks a byte at an offset of a structure in a4 to decide whether to call a fourth character routine before returning a value in d0.
- `process_save_game_strings`: Calls another routine four times with specific word constants and pointers, then sets d0 to 6 and branches to another location.
- `render_menu_options`: Calls a routine to display a sequence of text strings based on a conditional check of a global byte.
- `render_menu_text_strings`: Calls a routine for each character in a sequence of four text strings, passing a specific word value and a pointer to each string.
- `copy_text_tiles`: Calls call_routine_per_character_2 four times with different word constants and pointers to process a set of strings.
- `poll_input_for_slot`: Polls a message port until it finds a message of class eight with code 104 whose position maps onto a slot, and returns that slot number.
- `dispatch_by_event_code`: Clears a blit area through a graphics function pointer, then dispatches on a numeric code: codes one, two, three, four and six each call a different subroutine.
- `dispatch_input_and_branch`: Calls a routine, moves the result from d0 to d2, and branches to another location.
- `dispatch_and_branch`: Calls a routine, moves the result from d0 to d2, and branches to another location.
- `call_external_routine`: Calls a routine, moves a word from d0 to d2, and branches to another location.
- `call_menu_routine`: Calls a routine, moves a word from d0 to d2, and branches to another location.
- `dispatch_input_event`: Calls a routine, saves d0 in d2, and branches to another location; if d0 is not less than 7, it uses d0 to jump to a location in a table; it then calls another routine, polls input for a slot, and calls another routine before returning.
- `process_menu_strings`: Calls another routine, calls another routine, and if a byte at an offset of a5 is not zero, calls another routine before branching to another location.
- `copy_config_tiles`: Calls call_routine_per_character four times with different word constants and pointers to process a set of strings.
- `prompt_keyboard_input`: Displays a series of text strings and enters a loop that waits for keyboard input, storing up to six characters into a local buffer while filtering for specific characters and checking for an escape key to abort.
- `display_instruction_text`: Copies a string from a data table to a buffer, calls a routine to display the text character by character, and then branches to another routine.
- `display_input_prompt_and_process_sequence`: Copies a string to a buffer, calls a routine to display it, and then calls a routine to process the character sequence.
- `display_input_prompt_sequence`: Copies two text strings into a buffer at an offset from a pointer in a5, then calls another routine to process each character of that buffer.
- `display_input_prompt_and_validate`: Copies a string to a buffer, displays it, and then enters a loop that waits for keyboard input to validate a character sequence.
- `display_input_prompt_and_validate_2`: Copies strings to a buffer located at an offset from a5, calls a routine to process each character, and enters a loop that reads input characters to validate them against a set of criteria before storing them in a sequence.
- `calculate_jump_target`: Compares a word in d0 to 6, and if it is not equal, adds d0 to itself and loads a word from a table using d0 as an offset before jumping to the resulting address.
- `validate_input_character_sequence`: Checks if a character from a structure matches a specific byte value and evaluates a sequence of characters stored in a buffer.
- `clear_and_branch`: Clears d4 and branches to another location.
- `validate_and_store_input_character`: Checks if a character in a register already exists within a buffer by looping through the buffer, then checks if the character is a vowel.
- `store_byte_and_call_libraries`: Stores a byte from register d2 into a buffer at an offset determined by d5, increments d5, and then calls two library functions using pointers retrieved from a global table.
- `branch_to_offset_11a6`: Branches to a specific offset within another routine.
- `branch_to_offset_1090`: Branches to a specific offset within another routine.
- `copy_config_tile_bytes`: If a word in d6 is zero, it copies 6 bytes from a source buffer at an offset of negative 6 from a5 to a destination buffer at an offset of negative 32586 from a4, then calls three other routines before returning.
- `copy_and_poll_input`: Copies two longwords from a location offset from a4 to a location offset from a5, calls three routines, and if d3 is zero, calls another routine and branches back to the start of this routine.
- `handle_menu_selection`: Calls prompt_keyboard_input and enters a loop that branches based on the returned value.
- `process_item_pointer`: Checks a byte at an offset of a4, and if it is zero, pushes a word and a pointer onto the stack, calls another routine, cleans up the stack, and branches to another location.
- `push_string_and_call_routine`: Pushes a 16-bit value onto the stack, pushes the address of a string onto the stack, calls another routine, cleans up the stack, and branches to a different location.
- `manage_game_save_file`: Clears a screen area and prompts the user to insert a disk, then attempts to open a file for writing or reading.
- `execute_disk_io_loop`: Pushes the current stack pointer to the stack, calls four consecutive routines, adjusts the stack pointer, and branches to a nearby location.
- `manage_disk_io_menu`: Clears a screen area and enters a loop to poll for input to manage a menu for reading or writing data to a disk.
- `handle_disk_io_menu`: Enters a loop that polls for input to manage a menu for reading or writing data to a disk.
- `clear_area_and_set_flag`: Clears a blit area, sets a register to one, and branches to a later instruction to update two values from a structure using a pointer in a5 before returning.
- `branch_on_comparison`: Compares a word in d0 to six, branches if it is less than six, otherwise it adds d0 to itself, loads a word from a table using d0 as an index, and jumps to a location from that table.
- `branch_to_offset_4928`: Performs a short branch to a location 4928 bytes ahead in the instruction stream.
- `copy_two_longwords`: If d3 is equal to 1, copies two longwords from a location offset from a5 to a location offset from a4.
- `load_pointer_from_structure`: Loads a pointer from offset 8 of a structure in a5 into a3, and if a3 is not zero, moves a3 to a2 and branches; otherwise it returns.
- `process_string_sequence`: Loads a pointer from a structure in a4, then iterates through a string by calling a routine for each character and calculating the string length to advance the pointer.
- `process_event_sequence`: Checks a value at an offset of a structure pointed to by a4, and if it is 1 and d3 is not 0, calls call_routine_per_character_2, call_two_routines_in_order, poll_input_for_slot, and clear_blit_area_then_call.
- `store_registers_into_globals`: Writes the first two data-register values and a third register into three fixed global locations.
- `copy_block_to_strided_rows`: Copies eight bytes from a source block into a destination at offsets from zero to 280 in steps of 40, then repeats the copy six times with the destination advanced by 8000 bytes each time.
- `scan_global_array_twice`: Scans a global byte array in two passes, two bytes at a time.
- `run_sixteen_step_loop`: Runs a loop sixteen times, making two calls through a function pointer, one routine call and one call through a second pointer on each pass.
- `decrement_nibbles_in_word_list`: Walks a list of 16-bit words, with the count taken from the first register, and decrements each of the three low-order nibbles that is non-zero.
- `execute_function_list`: Executes a loop or a fixed number of times, calling indirect function pointers and another routine.
- `increment_nibbles_where_differing`: Walks two lists of 16-bit words together for a count taken from the first register.
- `copy_byte_array_2`: Copies a specified number of bytes from a source pointer to a destination pointer.
- `compare_string_bytes`: The routine compares two byte strings character by character, returning a status code based on the first difference or equality.
- `return_empty_routine`: This routine is empty and performs no operations before returning.
- `handle_workbench_disk_access`: The routine interacts with the Workbench interface, locks the game disk volume, parses strings, opens icon libraries, and processes input events and state changes.
- `parse_string_tokens`: Parses a string or quoted-string, extracting tokens separated by whitespace and storing them into a global table.
- `parse_string_tokens_2`: Parses a source string into individual tokens, handling quoted strings and skipping whitespace, then stores pointers to these tokens in a destination array.
- `copy_string_with_limit`: Finds the end of a destination string and copies characters from a source string, decrementing a counter until a null terminator or a zeroed counter is reached.
- `copy_string_with_limit_2`: Finds the end of a destination string and then copies characters from a source string while decrementing a counter.
- `load_icon_data`: Loads an icon from a library and stores a pointer or value from a table into a parameter-relative or global location.
- `append_string_to_buffer`: Advances a pointer to the end of a destination string and appends characters from a source string until a null terminator is copied.
- `copy_string_to_null`: Locates the null terminator of the first string parameter by incrementing a pointer until a zero byte is found, then copies the second string to that location.
- `copy_string_or_pad_2`: Copies a source string to a destination, or if a count is zero, fills the destination with null bytes, ensuring the destination is null-terminated.
- `allocate_and_init_record`: Calls a library routine for an identifier and returns zero when that is minus one; otherwise it allocates a record, calls a library routine on it and returns zero if the allocation fails.
- `reset_record_and_release`: If a record's pointer field is set, it calls a library routine on it.
- `calculate_string_length`: Calculates the number of bytes between a starting pointer and the null terminator of a string.
- `setup_self_linked_header`: Stores the record's own address plus four in its first word, clears its second word, and stores the record address in its third word.
- `copy_string_simple`: Copies characters from a source string to a destination buffer until a null terminator is encountered.
- `execute_shutdown_callbacks_2`: Iterates through a linked list of function pointers, executing each one and updating the list pointer to the next element.
- `execute_shutdown_callbacks`: Iterates through a list of indices to invoke a helper routine, then executes a series or conditional function pointers stored in global memory.
- `return_empty_routine_2`: the routine is empty and performs no operations before returning.
- `close_file_with_validation`: Checks if a file handle or index is valid and within a certain range before calling a library function to close a file and clearing a status flag.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bcdfp

- `clear_state_block_and_start_2`: Zeroes 494 32-bit words of global state, stores a register value in a field, and tests a flag bit that may call a library routine.
- `open_graphics_intuition_libraries`: Opens the graphics and intuition libraries and, when either open fails, calls an error routine with a message from the text table.
- `initialize_block_and_release`: Calls one library routine, then passes a pointer from a global slot to another routine and makes two more library calls with that pointer before returning.
- `copy_list_records_by_flag`: Walks a linked list of 20-byte records held in a table, copying each record into a buffer with a routine that takes a 20-byte count and advancing the buffer pointer.
- `initialize_four_entity_structures`: Loops four times over entity structures, clearing their 16-bit arrays and setting a few byte fields.
- `initialize_entity_arrays`: Loops four times filling per-entry 16-bit fields with values computed from the loop index, and sums attribute values from a global table into a per-entry long.
- `save_character_data_to_file`: Writes character configuration data to a file: it opens the file, writes padding and header blocks, and writes four character slots with their tables and status fields.
- `wait_for_library_call`: Repeatedly calls a function pointer from a global structure until a second function pointer from the same structure returns a non-zero value.
- `run_library_call_state_zero`: Sets a global state word to zero, makes one library call through a global block and calls a routine.
- `run_library_call_state_two`: Sets a global state word to two, makes one library call through a global block and calls a routine.
- `run_library_call_state_eight`: Sets a global state word to eight, makes one library call through a global block and calls a routine.
- `run_library_call_state_three`: Sets a global state word to three and stores its argument in a global, then makes a library call through a global block and calls a routine, with a further call when the state word is not six.
- `run_library_call_state_seven`: Sets a global state word to seven and stores its argument in a global.
- `run_library_call_state_ten`: Sets a global state word to ten and stores two arguments in globals.
- `transfer_dungeon_save_files`: Reads and writes save data through the dos library, opening files on the game disk and save paths, transferring blocks, and waiting briefly.
- `check_disk_space_and_prompt`: Locks the game disk volume through the dos library and checks the disk status in a loop, selecting one of two prompts for a status of one or two.
- `dispatch_sprite_by_type`: Loops over a sprite record and dispatches on its type value.
- `dispatch_sprite_logic_variant_zero`: Loads a word from an offset of a pointer in a4, performs a bitwise operation to isolate the lower two bits after adding 65535, pushes the result to the stack, calls three different routines, adjusts the stack pointer, and branches to another location.
- `dispatch_sprite_logic_variant_one`: Loads a word from an offset of a pointer in a4, adds 1, performs a bitwise operation to isolate the lower two bits, pushes the result to the stack, calls three different routines, adjusts the stack pointer, and branches to another location.
- `call_and_branch`: Calls another routine and then branches to another location.
- `dispatch_sprite_logic_2`: Performs a multi-way branch based on a value from a global variable at an offset from a pointer in a4, potentially calling process_entry_field_calls and swap_buffers_and_wait_sync twice with a value from the stack.
- `setup_sprite_rendering_loop`: Stores zero in a global field and makes a series of subroutine calls.
- `call_routine_nine_times`: Calls one routine nine times in a loop and returns, without other work.
- `call_sprite_offset_calculator`: Calls a routine that calculates a memory offset by multiplying a register value by a constant and adding a base, storing the result in a global variable.
- `store_sprite_offset_and_call`: Computes a value from a global index and a second global field, stores it into a per-entry field of a global table, then calls subroutines with the index.
- `configure_sprite_rendering_params`: Calls subroutines with fixed numeric arguments and with a value computed from a global index, stores its argument into a global index field, then makes further subroutine calls.
- `dispatch_sprite_rendering`: Checks a global index against a field and calls a subroutine when they differ.
- `call_sprite_logic_zero`: Clears the stack top, calls another routine, adjusts the stack pointer, and branches to a previous location in a loop.
- `call_sprite_logic_zero_alt`: Clears the stack top, calls another routine, adjusts the stack pointer, and branches to a previous location in a loop.
- `call_sprite_logic_one`: Pushes 1 onto the stack, calls another routine, adjusts the stack pointer, and branches to a previous location in a loop.
- `call_sprite_logic_one_alt`: Pushes 1 onto the stack, calls another routine, adjusts the stack pointer, and branches to a previous location in a loop.
- `call_sprite_logic_two`: Pushes 2 onto the stack, calls another routine, adjusts the stack pointer, and branches to a previous location in a loop.
- `call_sprite_logic_two_alt`: Pushes 2 onto the stack, calls another routine, adjusts the stack pointer, and branches to a previous location in a loop.
- `call_sprite_logic_with_three`: Pushes the word three onto the stack, calls a routine at a relative offset, increments the stack pointer by two, and branches to another routine.
- `call_sprite_logic_with_three_alt`: Pushes the word three onto the stack, calls a routine at a relative offset, increments the stack pointer by two, and branches to another routine.
- `call_sprite_logic_with_four`: Pushes the word four onto the stack, calls a routine at a relative offset, increments the stack pointer by two, and branches to another routine.
- `dispatch_sprite_logic`: Calls check_and_update_sprite_condition, then calls store_word_in_state, swap_buffers_and_wait_sync, and call_two_routines_then_stop.
- `check_range_overlap`: Searches ten entries in a global table of 16-bit coordinate pairs for one whose box contains the argument, testing the low half of the argument within thirteen units of the entry's first value and the high half within nine of its second.
- `check_and_update_sprite_condition`: Checks values in a global table indexed by parameters, increments or decrements specific fields if conditions are met, and calls helper routines for further processing.
- `update_sprite_conditions`: Calculates offsets into a table, compares values, and if they differ, decrements one value and increments another in a table before calling call_helpers_by_tens_value twice.
- `call_helpers_by_tens_value`: Reads a 16-bit value from a per-entry table, divides it by ten, and makes three subroutine calls when the quotient is zero or two subroutine calls otherwise.
- `process_entry_field_calls`: Calls a subroutine with the address of an entry field in a per-entry table, then calls another subroutine with a value derived from the entry index.
- `clear_entry_flag_and_refresh`: Reads a value for the current entry and, when it is non-zero, checks two flag bits returned by subroutine calls.
- `clear_entry_flag_and_refresh_2`: Checks two flag bits from subroutine calls and clears either the entry's byte at the index before a stored value or its first byte, then calls subroutines to refresh the entry.
- `clear_entry_byte_and_refresh`: Clears the entry byte just before a stored index, then calls subroutines with a value of one to refresh the entry.
- `store_entry_byte_and_refresh`: Reads a subroutine value for the current entry and, when it is not twelve and a second subroutine returns non-zero, stores that second value as a byte in the entry and writes a zero byte after it.
- `search_short_table`: Searches a global table of 26 16-bit values for the argument and returns its index plus 65 when found, or zero when not found.
- `calculate_string_offset`: Calculates an offset by multiplying a word from offset -29132 of a4 by 26 and adding it to a base address, calls a routine to measure a string length at that location, and returns if the length is zero or twelve.
- `update_field_and_sync`: Calculates an offset using a value from a global variable and d2, writes the byte 32 to that offset, then repeats a similar calculation to clear a byte at a different offset, calls a routine with a value from the stack, calls a routine to store state, calls a routine to swap buffers, and calls the first routine again with a value from a global variable before branching to another routine.
- `allocate_memory_and_copy`: Copies a byte from a source pointed to by a0 to a destination pointed to by a2, then calls a routine using a pointer from a system library.
- `allocate_and_open_graphics_library`: Allocates a block through the exec library and returns one when that fails.
- `process_integer_list`: Iterates through a list of integers until a terminator value is found, invoking a library function for each entry.
- `process_pair_list_with_library`: Calls a library routine at offset minus 30 through a global base, then walks a list of pairs until the second word of a pair is minus one, calling a library routine at minus 42 for each pair.
- `empty_stub_2`: This routine is empty and performs no operations.
- `cleanup_allocated_memory`: Walks two tables of function pointers and global slots, calling a routine for each entry that is set.
- `enable_dma_and_interrupts`: Copies two coordinate words from one state block into another and disables all interrupt and DMA bits through the custom chip enable words.
- `setup_copper_and_dma_registers`: Disables all interrupt sources, loads a copper list pointer pair from a state block into the copper pointer registers, and sets the DMA control word.
- `wait_for_vertical_blank`: Polls the interrupt request word until its vertical blank bit is set, then writes that bit back to the interrupt request register to acknowledge it.
- `wait_for_vertical_blank_frames`: Waits for the vertical blank bit once per frame in the requested count, acknowledging each one through the interrupt request register.
- `wait_for_flag_repeatedly`: Repeatedly clears a flag byte in a state block and spins until a handler sets it non-zero.
- `forward_call_to_subroutine`: Calls one other subroutine and returns without doing any work of its own.
- `build_copper_list_entries`: Fills a copper list with move and wait instructions.
- `copy_table_into_buffers`: Copies seventeen long words from a table in a base block into a buffer, then copies seventeen words from a later point in the same table into a second area of the buffer.
- `get_state_field_pointer`: Returns the address of one fixed field inside the shared state block.
- `store_word_in_state`: Stores the word passed in the data register into a fixed word field of the shared state block.
- `initialize_copper_list_slots`: Sets a state flag from the subroutine result.
- `store_state_and_copy_table`: Stores the subroutine result as a counter and sets a companion word to minus one.
- `update_sprite_motion_frame`: Toggles a frame bit and increments a frame counter, then applies pending table data to a copper list.
- `dispatch_through_unrecovered_pointer`: Jumps through a pointer whose target was not recovered, so it only forwards control to code that is not present here.
- `acknowledge_selected_interrupt_requests`: Reads the interrupt request word, keeps only the copper, vertical blank and blitter bits, and writes them back to clear those requests.
- `acknowledge_ports_interrupt`: Calls one subroutine, then writes the ports interrupt bit to the interrupt request register to acknowledge it.
- `jump_through_unrecovered_pointer`: Forwards execution through a jump pointer that was not recovered, so its behaviour depends on code missing from this dump.
- `cycle_copper_pattern_timer`: Counts down a timer while enabled and toggles a phase word each time it expires.
- `reload_copper_table_timer`: Sets the frame timer to a fixed value, copies six words from a global table into the copper list, and forwards to an unrecovered jump target.
- `flush_pending_copper_entries`: Copies eleven words of pending data into the copper list when a flag is set, and fills six entries from a table when a stored index is non-negative.
- `initialize_player_input_state`: Resets the input-state fields of a state block and sets a timer to a tenth of a global byte value.
- `feed_table_word_into_copper`: Every third frame, when a flag is set, takes the next word from a cursor in a table and writes it to a fixed copper list slot.
- `store_result_and_forward`: Stores a subroutine result and a second returned value into the state block, calls one more subroutine, and forwards to an unrecovered jump target.
- `check_regions_on_state_change`: Compares two stored state bytes with newly read values.
- `scan_regions_for_position_flags`: Scans a list of rectangular regions for one with the first flag bit set that contains the current position.
- `update_second_state_flag`: Compares a stored state byte with a newly read value and, when it changed to non-zero, scans the region list for a match on the high flag bit.
- `apply_second_state_change`: Stores a newly read state byte and a second returned value, then checks a region list for a match on the high flag bit when the new byte is non-zero.
- `scan_regions_for_second_flag`: Walks a list of rectangular regions until it finds one with the high flag bit that contains the current position.
- `store_state_and_tick_counter`: Stores a subroutine result and a second returned value into the state block, calls one more subroutine, and decrements a non-zero counter in the block.
- `store_state_and_run_tail`: Runs an extra subroutine call before storing a result and a second value into the state block, then decrements a non-zero counter.
- `decode_keyboard_serial_byte`: Handles the keyboard serial register handshake by clearing the previous acknowledge and reading a byte when the strobe bits are set.
- `set_key_bitmap_bit`: Sets one bit in a held-key bitmap in the state block, using the low three bits of the input to select it.
- `read_key_bitmap_byte`: Returns one byte of the held-key bitmap from the state block.
- `clear_buffer_and_reset_counters`: Resets several counters and flags in a state block and sets one byte through a pointer to a fixed value.
- `scroll_bitplane_buffer_frame`: Advances a wrapping offset by two bytes out of forty-two and copies byte pairs for eight bit planes from two source addresses into a ring buffer at that offset.
- `advance_scroll_buffer_countdown`: Decrements a countdown held at the address passed in a register when a flag is set.
- `advance_copper_table_cursor`: Moves a table cursor forward by one row unless it has reached its limit.
- `call_and_return_first_word`: Calls one subroutine, calls a second one, and returns the word produced by the first call.
- `read_next_text_character`: Reads the next byte from a circular text buffer, moving backwards with wrap-around at eighty entries.
- `clear_text_active_flag`: Clears a flag in the state block and returns zero.
- `scan_string_for_nonzero_result`: Walks a zero-terminated byte string, calling a subroutine on each character and setting an active flag.
- `write_byte_to_text_ring`: Stores one byte at the write index of a circular text buffer and then advances the index with wrap-around.
- `set_text_write_index`: Sets the write index of a circular text buffer to the value passed in a register and returns zero.
- `copy_and_process_data`: Calls a routine, clears a memory area pointed to by a value from a structure in a5, loops to find a null terminator in a string, uses that length to copy seven bytes from a source table to a destination table, and finally writes 2400 to a location in a5 and performs a multiplication and assignment.
- `copy_data_and_calculate`: Copies eight words from a source table to a destination table using a loop, sets a word at offset 974 of a structure in a5 to 2400, and stores a product of two values into offset 1084 of that structure.
- `decrement_timer_array`: Decrements each non-zero counter in a block of nine words by one.
- `initialize_motion_and_bounds`: Sets velocity, position and clamping bounds in a state block and stores two bytes read from the argument structure.
- `step_position_within_bounds`: Either clears two blocks of object data or advances the position by velocity or by a scaled step with fractional accumulators.
- `clamp_position_to_bounds`: Clamps the x and y values passed in registers to the stored bounds and saves them as the current position.
- `clamp_y_and_store_position`: Clamps the vertical value passed in a register to its stored bound, saves it with the horizontal value as the current position, calls two subroutines, and forwards to an unrecovered jump target.
- `offset_and_clamp_position`: Adds the stored offset to the current position and clamps the result to the stored bounds.
- `clear_object_data_pointers`: Clears two blocks of object data through pointers held in the state block and forwards to an unrecovered jump target.
- `compute_wrapped_byte_deltas`: Stores two newly read bytes as the previous values and returns the signed differences from the old values.
- `compute_direction_and_delta`: Returns two values: the complement of the first input, negated when a flag is set, and the second input minus a stored value, wrapped into the signed byte range.
- `return_immediately`: The routine contains instructions but performs no data manipulation or hardware access before returning control to the caller.
- `record_position_history_entry`: Stores the current position in a four-entry history ring, steps the ring index back with wrap-around, and writes a subroutine value or zeros into two output slots.
- `advance_history_index_and_write`: Steps a four-entry index with wrap-around, reads a value from a subroutine, and writes it into four slots of a buffer at fixed offsets.
- `pack_coordinates_with_flags`: Adds two offsets to the stored position and packs the sums into a single long word.
- `read_fire_button_pressed`: Returns minus one when a bit of the cached port A status byte is clear, which means the fire button is pressed, and zero otherwise.
- `read_mouse_counters_and_store`: Temporarily disables the master interrupt enable while it reads the mouse counter word into two state bytes.
- `store_position_with_interrupts_masked`: Temporarily disables the master interrupt enable, stores two input words as the current position, and re-enables interrupts if they were enabled before.
- `store_velocity_components`: Stores two input words into the velocity fields of a state block.
- `negate_and_store_values`: Negates two 16-bit values in d0 and d1, subtracts 12 from each, and stores the results into a table at offsets 162 and 164 from a base address.
- `store_registers_to_table`: Stores the values of d0, d1, a0, and the stack pointer into a table at offsets 168, 170, 172, and 174 from a base address.
- `compare_d0_value`: Loads a word from a relative offset into a0, compares d0 against the values one, two, three, and four, and either branches to another routine or sets d0 to negative one and returns.
- `get_byte_from_offset_c4`: Returns the byte stored at offset 196 of the structure pointed to by a0.
- `get_byte_from_offset_c5`: Returns the byte stored at offset 197 of the structure pointed to by a0.
- `get_word_from_offset_82`: Returns the word stored at offset 130 of the structure pointed to by a0.
- `get_word_from_offset_84`: Returns the word stored at offset 132 of the structure pointed to by a0.
- `update_scaled_value`: Stores a 16-bit word from d0 into offset 178 of a structure, calculates a new value by multiplying d0 by 46 and dividing by 100 into offset 176 of the same structure, and modifies the interrupt enable register based on a bit test of a value from the stack.
- `update_status_and_copy`: Calls another routine, checks a byte at offset 200 of a5, and if it is not zero, writes a word of 1 to a0 incremented, clears a word at a0 incremented, writes two words from a5 to a0, and decrements a byte at offset 200 of a5.
- `clear_list_node`: Pushes a word from a6 plus 28 onto the stack, sets a word at offset 154 of a6 to 4096, checks a word at offset 190 of a5, and if zero, clears two longwords at a0, checks bit 14 of a value from the stack, and either returns or sets offset 154 of a6 to 32768 and returns.
- `transfer_and_clear_list_node`: Decrements a 16-bit value at offset 190 of a structure, traverses a table using a value from offset 188 and offset 182, and if a comparison with 128 is met, copies a 32-bit doubleword from a table to a destination and clears the source entries, otherwise it returns.
- `pop_queued_entry_or_marker`: Produces a result record according to the queue state: a fixed code when queueing is disabled, a different fixed code when nothing is pending, the next queued entry's words when one is popped, or a code with the current position otherwise.
- `clear_queue_entries`: Resets the queue head, tail and count words and zeroes 128 two-long entries of a buffer.
- `enable_queue_processing`: Sets a state word to one, which enables queue processing.
- `disable_queue_processing`: Sets a state word to zero, which disables queue processing.
- `store_mask_and_test_bit`: Stores the input word, masked to a fixed set of bits, as a control word.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `clear_bits_in_structure`: Clears specific bits in a 16-bit word at offset 194 of a structure by performing a bitwise AND with the inverse of a masked version of d0.
- `set_bits_in_structure`: Sets specific bits in a 16-bit word at offset 194 of a structure by performing a bitwise OR with a masked version of d0.
- `enqueue_entry_with_position`: When the input bits match the stored control word and the queue is not full, it appends an entry to the end of a list.
- `append_position_record_at_pointer`: Records a buffer pointer in the ring of queued entries and advances the ring head.
- `scale_value_to_counter`: Multiplies the input by a stored word and divides by sixty.
- `copy_byte_within_structure`: Copies a byte from offset 201 of a structure to offset 200 of the same structure.
- `copy_region_list_with_offsets`: Copies a linked list of rectangle entries into the state block, keeping each entry's left and top edges.
- `render_text_string_with_blitter`: Draws a text string into two bitplane buffers by copying each character's glyph from two font tables.
- `return_immediately_2`: The routine performs no operations and returns to the caller immediately.
- `initialize_render_buffer_state`: Sets two buffer pointers in the state block to their alternate positions and clears the flags and counters.
- `swap_buffers_and_wait_sync`: Copies six long words from the current buffer into a staging area and requests an update.
- `call_two_subroutines`: Calls two other subroutines in order and returns.
- `compute_global_offset_and_call`: Calls one subroutine, stores a global address computed from the low word of a register as a scaled offset, and then calls another subroutine.
- `process_entries_until_negative`: Walks a list of five-word entries, calling a subroutine for each one, until it reaches an entry whose first word is negative.
- `blit_sprite_rows_with_masks`: Sets up blitter masks and widths for a sprite from a five-row descriptor and runs one blit per row, waiting for the blitter to finish each.
- `store_parameters_and_call`: Stores three input values into global variables and then calls two subroutines.
- `call_blit_routines`: Saves registers, calls two different routines, and then restores the registers before returning.
- `process_list_with_loop`: Loads a word from a pointer in a2, enters a loop that calls another routine five times, and uses a branch to repeat the loop.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `call_two_routines_then_stop`: Calls two subroutines in order.
- `blit_list_entry_and_dispatch`: Stores a pointer argument and an advanced copy of it for later use, then computes blitter width and offsets from the referenced record.
- `blit_sprite_entry_by_mode`: Stores two input words into a sprite record.
- `load_handler_pointer`: Loads a pointer from a structure in a5 into a0 and branches to another routine.
- `calculate_and_jump_to_handler`: Performs several arithmetic operations to calculate values for a structure in a5 and a6, then executes an indirect jump using a pointer from a structure in a5.
- `populate_and_loop_structure`: Populates a local structure in a6 with values from a structure and a list, then enters a loop that updates a6 entries using data from the list until a counter reaches zero.
- `wait_for_bit_clear`: Loops while bit 0 of a 16-bit word at offset 4 of the structure in a6 is set, then compares a 16-bit word at offset 6 of a6 with d0 after shifting it right by 8 bits.
- `rotate_byte_buffer`: Calculates a sum of three bytes from a table using carry propagation, stores the result in the first byte of the table, and then shifts five bytes from the table to the five preceding bytes in memory.
- `step_sixteen_flagged_writes`: Runs sixteen steps, each waiting for two flag bytes to be set by an external handler and calling a subroutine.
- `decay_packed_nibble_fields`: Walks a table of four-byte entries and reduces each packed field of the second word by one unit, leaving fields at zero unchanged.
- `step_nibbles_toward_targets`: Walks a table of four-byte entries and moves each packed nibble field of the second word one step toward the matching field of a second table.
- `clear_five_buffers`: Zeroes five buffers of two thousand long words each, whose addresses are listed in the state block.
- `service_four_audio_channels`: Applies a pending command to each of four audio channels.
- `service_audio_channels_loop`: Services audio channels from the highest index down.
- `service_audio_channels_default`: Writes a default length and sample address for the current channel, then walks the channels downward.
- `set_channel_pending_command`: Sets a channel's pending command to a stop type and stores a word from the first argument.
- `call_with_base_pointer_offset`: Calls a subroutine with an address computed from a base register and a fixed offset, passing the input value through, and returns its result.
- `copy_byte_block`: Copies a block of bytes from the source address to the destination address, counting down a 32-bit length.
- `do_nothing`: this routine is empty and performs no operations before returning.
- `start_program_and_parse_startup`: Allocates a context block and tags it with a four-character marker, then finds the calling task.
- `split_command_line_into_args`: Allocates an argument buffer and copies the command line into it, splitting on spaces and tabs while honouring double-quoted arguments.
- `parse_command_line_arguments`: Copies the command line into a newly allocated buffer, splitting it into arguments on whitespace and keeping quoted text together.
- `append_bounded_string`: Appends a source string to the end of a destination string, copying at most the given number of characters.
- `concatenate_limited_text`: Appends at most a given number of characters from one string to the end of another and terminates the result.
- `load_icon_tooltype_list`: Opens the icon library and reads the tool types of an icon, including the window entry, into an array.
- `append_string_to_end`: Appends a zero-terminated source string to the end of a destination string and returns the destination address.
- `concatenate_strings_unbounded`: Appends a zero-terminated source string to the end of a destination string and returns the destination address.
- `copy_string_with_padding`: Copies at most a given number of characters from the source string to the destination.
- `create_named_message_port`: Allocates a signal bit and a port structure, then fills in its name, type and signal fields and records the owning task.
- `delete_message_port`: Removes a named port from the public list and marks the port structure as freed.
- `format_text_into_buffer`: Formats text from a template into an output buffer, handling flag characters, width and precision fields and length modifiers.
- `measure_string_length`: Counts the characters before the zero terminator of a string and returns the count.
- `initialize_empty_list_head`: Sets up a list header so that it points to itself with an empty list.
- `initialize_each_unit_record`: Walks a table of twenty-two-byte unit records between two offsets from a base register, calling a subroutine on each record.
- `write_byte_to_stream`: Writes one byte to a buffered stream, flushing the buffer when it fills or a newline is written, and returns the byte.
- `close_stream_and_release`: Flushes and closes a buffered stream, releases its buffer if one was allocated, and clears its fields.
- `finish_stream_close_status`: Flushes and closes a buffered stream as the routine above does, but also combines a previously computed status value to decide success.
- `return_success_zero`: Returns zero without doing anything else.
- `flush_all_open_streams`: Flushes every open stream in the stream table when given a null pointer, or flushes only the stream passed in.
- `flush_streams_or_one`: Behaves like the stream flush routine above, flushing all open streams for a null pointer and otherwise only the one stream given.
- `flush_remaining_open_streams`: Walks the stream table from a given position, flushing each open stream in turn.
- `call_single_subroutine`: Calls one subroutine and returns.
- `empty_stub_routine`: The routine consists of six instructions that perform no observable work and end with a return.
- `negate_result_if_negative`: Calls one subroutine and returns its second result, negated when the first input is negative.
- `call_and_pass_through_pair`: Calls one subroutine and returns the second input passed through together with the subroutine's second result.
- `forward_call_and_return_2`: Calls one subroutine and returns its result unchanged.
- `divide_long_by_word`: Divides a 32-bit unsigned value by a divisor, using a bit-by-bit loop when the divisor is wider than sixteen bits and a direct division otherwise.
- `initialize_stream_buffer`: Sets up a stream record with a buffer, allocating a one-kilobyte buffer for ordinary streams.
- `remove_node_and_free`: Searches a linked list of allocated nodes for the given node, unlinks it when found and frees its memory.
- `find_and_free_list_node`: Searches the allocation list starting at the given node for the node passed in a register, then unlinks and frees it.
- `unlink_node_at_position`: Unlinks the node held in an address register from the allocation list and frees it through the memory release call.
- `pop_head_and_free`: Advances the allocation list head to the next node and frees the old head node.
- `free_all_allocated_nodes`: Frees every node in the allocation list, then clears the list head and returns zero.
- `free_node_chain_and_clear`: Frees each node in a chain starting at the given node, then clears the list head and returns zero.
- `allocate_linked_memory_node`: Allocates a block of the requested size with a link word in front, pushes it onto the allocation list, and returns the usable area after the header.
- `copy_zero_terminated_string`: Copies a zero-terminated source string into the destination, including the terminator, and returns the destination address.
- `check_stream_is_interactive`: Checks that a stream index is valid and open, then asks DOS whether that stream is interactive.
- `seek_stream_position`: Checks a stream index and seeks the stream to the given position through DOS. If the seek fails it stores the DOS error code and returns minus one.
- `delete_file_by_name`: Deletes a file through DOS. When the deletion fails it records the DOS error code and returns minus one, otherwise it returns zero.
- `return_zero`: This routine returns a zero value.
- `write_block_to_stream`: Checks a stream index and access mode, optionally repositions the stream before writing, then writes a block of data through DOS. It stores the DOS error code and returns minus one on failure.
- `check_break_signal_bits`: Reads the task signal bits from the system library.
- `write_global_word_to_output_2`: Writes a four-byte global value to the standard output through DOS, then calls one other subroutine with the value one.
- `run_pending_exit_routines`: Runs each pending exit routine from a linked list, unlinking nodes as it goes.
- `shut_down_program_resources`: Closes every open stream, releases the allocated buffers and locks, restores saved values, and closes the libraries it opened.
- `empty_routine`: the routine performs no operations and returns immediately.
- `close_stream_by_index`: Checks a stream index, closes the DOS handle unless it is marked as a standard stream, and clears the slot.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bcdfq

- `start_and_open_libraries`: Program entry point.
- `check_fire_button_pressed`: Returns true when a bit of the cached port A status byte is clear, which means the fire button is pressed, and false otherwise.
- `check_hardware_status_bit`: Returns true when one bit of a hardware status byte at a fixed chip address is clear, and false otherwise.
- `wait_on_second_flag`: Returns at once when a first flag is clear.
- `update_scaled_entry_values`: Repeats 105 times.
- `open_named_file`: Opens a file through DOS using the file name held in the program and stores the handle in the block.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `copy_second_region_to_start`: Copies forty-eight thousand bytes from a region that starts forty-eight thousand bytes into a buffer to the start of that same buffer.
- `clear_bitmap_and_copy_region`: Clears a bitmap through the graphics library, then copies eight thousand long words from a later point in the buffer to its start.
- `read_entries_until_flag`: Calls the DOS read routine once for each entry in a table of values.
- `read_entries_until_flag_2`: Calls the DOS read routine once for each entry in a table of values.
- `read_entries_until_flag_3`: Calls the DOS read routine once for each entry in a table of values.
- `read_block_from_file`: Calls the DOS read routine once through the handle stored in the block.
- `load_palette_over_frames`: Sixteen times, it waits for frame events, calls one subroutine and loads the palette through the graphics library.
- `decay_nibble_table_entries`: Decrements each packed nibble field of a table of words by one unit, keeping zero fields at zero.
- `load_palette_and_set_flag`: Sixteen times, it waits for frame events, calls one subroutine and loads the palette through the graphics library.
- `step_nibble_table_toward_target`: Moves each packed nibble field of a table of words one step toward the matching field of a second table.
- `blit_six_row_passes`: Takes ownership of the blitter, then runs sixteen passes that program the blitter registers for a block of six rows each.
- `select_audio_channel_mask`: Sets a global value to a one-bit mask for the given channel number, but only for channel numbers below four.
- `call_routine_sixteen_times`: Calls one subroutine sixteen times and then clears a global value.
- `start_audio_channel_sample`: Starts playback on an audio channel when its sample table entry is present.
- `store_channel_sample_fields`: Writes a sample address and a length into an audio channel record, using a fixed value when the length argument is zero.
- `set_channel_length_and_pointer`: Stores a length in an audio record, using a default when the length argument is zero, and updates the channel's pointer and count fields.
- `update_channel_pointer_fields`: Stores a length, a pointer offset derived from a count, and a second value into the channel record, updating the channel's count fields.
- `spin_forever_loop`: Loops indefinitely without an exit and never returns to its caller.
- `latch_channel_pointer_if_set`: When the low bit of the input is set and a channel's stored pointer is non-zero, copies that pointer and a related word into the channel record.
- `process_channel_updates`: When the update flag is set, calls four subroutines and then reads a channel entry.
- `advance_pattern_playback_tick`: Advances the playback tick for a pattern sequence: it steps the tick counter, reads each channel's pattern bytes, and applies note, volume and effect commands.
- `decode_pattern_row_for_channels`: Decodes one row of pattern data for each of four channels, updating each channel's sample, volume and effect fields and applying the effect commands.
- `continue_pattern_row_decode`: Handles the remaining per-channel pattern commands, including end-of-pattern and jump commands, and advances the position in the sequence table.
- `continue_pattern_decode_entry`: Handles the remaining per-channel pattern commands after the row setup and updates the sequence position.
- `decode_pattern_effect_commands`: Continues the per-channel pattern decode and applies the remaining effect codes, including the repeat, end-of-row and stop commands.
- `decode_pattern_channel_loop`: Loops over the channels of a pattern row, decoding each channel's note bytes and applying effect commands such as sample pointer, end-of-pattern and stop codes.
- `advance_pattern_channel_counter`: Handles the remaining per-channel pattern codes for one row, advancing a row counter on the zero code and applying volume and effect codes.
- `apply_channel_effect_code`: Applies one effect code from a pattern row to the current channel, setting its volume, speed, offset, sequence jump or slide fields.
- `apply_channel_volume_and_tempo`: Sets a channel's volume, tempo, panning and sequence jump fields from effect codes in a pattern row, scaling volume to a maximum of sixty-four.
- `apply_pattern_effect_fields`: Applies the tempo, panning, sequence jump, slide and volume effects of a pattern row to the channel record.
- `apply_pattern_row_effects`: Applies the panning, sequence jump, slide, end-of-row and volume effects of a pattern row to the channel record.
- `apply_pattern_jump_and_volume`: Applies the sequence jump, tempo, panning, slide and volume effects of a pattern row to the channel record.
- `apply_pattern_slide_effect`: Sets a channel's slide target from a pattern effect code and its offset, and updates the channel's effect fields.
- `apply_channel_slide_target`: Sets a channel's slide target from a period table when the channel index is below four.
- `process_channel_effect_tick`: Runs the per-tick effect step for one channel, applying slide, period offset and volume changes.
- `apply_bit_masks`: Performs several bitwise operations on memory locations pointed to by a0, a2, and a6, and branches to another routine.
- `increment_and_branch`: Adds a word from d4 to a word at offset 18 of a5, moves that word into d5, and branches to another routine.
- `calculate_index_and_offset`: Tests a byte in d4, and if non-zero, divides d3 by 3, swaps the nibbles of the result, and if the result is non-zero, performs a bitwise AND on d4 and adds a byte from a5 to d4 before branching.
- `decrement_and_shift`: Decrements d0 by one, and if it is not zero, branches; otherwise it shifts d4 right by four bits and adds a byte from a5 to d4 before branching.
- `fetch_table_value`: Calculates an index by adding a byte from a structure, a byte from another structure, and a byte from a pointer, then loads a word from a table at that index into d5 and branches.
- `update_byte_with_offset`: Moves a byte from d4 to d1, then moves a byte from offset 2 of a5 to d0; if the lower nibble of d1 is zero, it subtracts d4 from d0 and stores the result in offset 2 of a5, otherwise it stores zero in offset 2 of a5 and d1.
- `clamp_and_store_byte`: Adds the lower nibble of d1 to d0, and if d0 is less than or equal to sixty-four, stores d0 in offset 2 of a5; otherwise it sets d0 to sixty-four and stores it in offset 2 of a5 before branching.
- `subtract_and_branch`: Loads a word from a structure into d5, compares d3 to 3, and if d3 is less than 3, subtracts d4 from d5 before branching.
- `compare_and_increment_values`: Compares a word from offset 30 of a5 to a word from offset 18 of a5, and if the latter is higher, adds a byte from offset 70 of a5 to it and compares the result to the word from offset 30.
- `update_and_clear_value`: Subtracts d4 from d1 and compares the result to d0; if d1 is not higher than d0, it loads a word from a structure into d1, clears that structure location, and then stores d1 into both a structure offset and d5.
- `update_channel_data_2`: Checks if a 16-bit value in register d5 is non-zero, and if so, writes it to offset 6 of a structure pointed to by a value at offset 20 of another structure.
- `update_structure_state`: If d4 is non-zero, it stores d4 into a structure, then performs several arithmetic operations involving a table lookup and bit shifts to update values in a structure and a register before branching.
- `compare_audio_parameters_2`: Compares a byte in d4 to 241, and if it is not equal, compares a byte in d3 to 3, and if that is not equal, branches to another location.
- `compare_audio_parameters`: Compares a byte in d4 to 242, and if equal, compares a byte in d3 to 3; otherwise it branches to another location.
- `trigger_audio_sample`: Compares d4 to 243, and if equal, checks if bits 1 or 2 of d3 are set; if so, it pushes d3 to the stack, loads several bytes from a structure pointed to by a5, and calls start_audio_channel_sample using a pointer from offset 12 of that structure.
- `clear_cia_bit`: Compares d4 to 248 and 249, and if d4 is 249, clears bit 1 of the CIAA Port A Data Register before branching.
- `set_cia_bit`: Sets bit 1 of the CIAA Port A Data Register and branches.
- `update_channel_data`: Checks a word in d5, and if non-zero, writes it to an offset in a structure, writes a byte from a structure to another offset, increments d7, and calls another routine if d7 is below a constant value.
- `compute_period_from_note_3`: Converts an input value into a period in the same way as the routines above, storing the low and high bytes of the result through two pointers.
- `store_buffer_and_return_pointer`: Stores the address of a global buffer in a channel record.
- `add_offset_to_nonzero_long`: Adds the value in the data register to a long word at the address passed in an address register, but only when that long word is non-zero.
- `relocate_section_entries`: For each loaded section flagged as needing relocation, adds its base address to each long word in its relocation table.
- `load_sample_bytes_and_scale`: Scales sixteen per-channel values by a stored volume and writes them into a table.
- `start_module_playback_if_ready`: Returns a fixed failure code when an initial status value is non-zero.
- `call_flagged_then_finish`: When a global flag is set it calls one subroutine, then always calls a second subroutine and returns.
- `clear_flag_bit_if_active`: When a global flag is set, clears the low bit of a byte through a pointer and calls one subroutine, returning its result.
- `open_cia_resource_and_map`: Opens the CIA resource through the system and sets register addresses for the timer and port hardware, depending on which CIA chip responds.
- `clear_flag_and_call_library`: When a flag is set, clears it and calls one routine through the library table.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bcdft

- `run_callback_and_free_block`: Calls a stored routine through a pointer, waits briefly in a delay loop, frees a memory block through the exec library, and clears the pointer slot it used.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `unpack_and_populate_data_table`: Loads a table header from memory to determine offsets and counts, then iterates through a loop to call a routine and store bytes into a destination buffer.
- `execute_conditional_calls`: Sets d4 to 14 and d1 to 5, calls another routine, and if the carry flag is set, sets d1 to 5, calls another routine, adds d0 to d4, and branches.
- `branch_and_initialize_registers`: Calls another routine and, if the carry flag is set, branches to a different location; otherwise, it loads the value one into registers d1 and d4 and branches to another location.
- `branch_on_carry_and_load`: Calls another routine, and if the carry flag is set, branches to another location; otherwise it loads 2 into d1, 3 into d4, and branches to another location.
- `branch_on_carry`: Calls another routine, and if the carry flag is set, branches to another location; otherwise it loads 4 into d1, 7 into d4, and branches to another location.
- `validate_and_branch_on_index`: Sets d4 to 23 and d1 to 8, calls another routine, and adds the result in d0 to d4.
- `call_and_initialize_registers`: Calls another routine, and if the carry flag is not set, sets d1 to five and d2 to zero before branching.
- `copy_table_data`: Sets d1 to 14 and d2 to 544, calls another routine, adds d2 to d0, loads a pointer into a3 from a table, and then enters a loop that moves bytes from the table to a destination while decrementing d4.
- `shift_and_swap_d6`: Decrements d7 and, if not zero, performs a series of operations including shifting d6 right, swapping d6, loading a new word from a pointer in a2 into d6, and swapping d6 again before returning.
- `tiny_lsr_rts`: Small routine of 2 instructions that shifts d6 right by 1, then returns.
- `lookup_table_masked_value`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `process_nested_data_tables`: Iterates through a table in a1 using a count from d5, copying data from a5 to a1 and calling a routine in a4, then enters a second loop that processes a list of offsets from a5 to add values to a table at a1.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d3 to d6, then returns.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
