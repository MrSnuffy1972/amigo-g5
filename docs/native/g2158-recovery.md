# Game 2158 (ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02158). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905232 top-level bytes in 56 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 39 of 39 units identical (bytes and relocations), 23 raw-word lines |
| Functions named and described | pass, 358 of 358 named (0 author symbols), 358 of 358 described (65 by rule, 155 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1001 of 1001 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11265 of 11282 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 71290 |
| code | 36232 |
| data | 502697 |
| padding | 14949 |
| unused | 280064 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Keymap | 1 | 13 | 1 |
| Run | 2 | 117 | 2 |
| Install | 3 | 117 | 2 |
| Else | 2 | 117 | 2 |
| Delete | 2 | 117 | 2 |
| Avail | 11 | 376 | 12 |
| addbuffers | 1 | 101 | 1 |
| Path | 2 | 117 | 2 |
| Resident | 2 | 117 | 2 |
| Assign | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| Rename | 2 | 117 | 2 |
| Dir | 2 | 117 | 2 |
| NewCLI | 2 | 117 | 2 |
| NewShell | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| cls | 2 | 20 | 1 |
| Execute | 2 | 117 | 2 |
| Copy | 2 | 117 | 2 |
| Makedir | 2 | 117 | 2 |
| List | 2 | 117 | 2 |
| CD | 2 | 117 | 2 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| LoadWB | 12 | 537 | 30 |
| Version | 11 | 574 | 18 |
| SetMap | 13 | 674 | 30 |
| More | 19 | 2290 | 76 |
| Format | 30 | 2009 | 76 |
| CLI | 11 | 607 | 25 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_BLIT | 11 | 253 | 7 |
| d | 1 | 13 | 1 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_CPU | 10 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Ambermoon | 8 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Ambermoon_install | 7 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1041 | 32 |

## Cases handled

- runtime segment an address (play-rs-m1500): 543 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_BLIT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_CPU: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Ambermoon: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Ambermoon_install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Keymap

- `empty_routine`: This routine is empty and contains no instructions.

### Run

- `find_max_and_prepare_buffer_3`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: When its pointer argument is not null, copies sixteen long words from a fixed block into it, then calls the routine pointer held in the address register.

### Install

- `find_max_and_prepare_buffer`: Walks a linked list to find the largest value stored at an index field.
- `copy_words_then_call_2`: Copies a fixed number of words from a source table into a destination buffer when the buffer pointer is valid, then invokes another routine.

### Else

- `find_max_and_prepare_buffer_2`: Same logic as the install variant: walks a linked list for the largest value at an index field, fills a buffer and copies a sixteen-long block when a library call succeeds, and returns zero or minus one.
- `copy_sixteen_longs_then_call_2`: Copies sixteen 32-bit values from a fixed offset in a base record into a caller-supplied buffer when that pointer is non-null, then calls through a stored handler pointer.

### Delete

- `allocate_table_by_max_entry`: Walks a linked list to find the largest value stored in each entry's field, then asks a service routine for a buffer.
- `copy_buffer_and_call_2`: Copies a fixed block of words from a source table into a destination buffer when the buffer pointer is valid, then invokes another routine.

### Avail

- `allocate_control_record_and_wait`: Allocates a control record for the current task and takes the current directory lock and the task's message port.
- `release_task_resources_and_reply`: Finds the current task and stores the supplied value in its record.
- `sum_region_sizes_matching_mask`: Walks a linked list of memory regions and adds up the size of each region whose attribute bits match the given mask.
- `print_memory_usage_table`: Prints the memory usage table with available, in-use, maximum and largest figures for chip, fast and total memory, gathered through exec library calls.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `forward_pair_with_stack_pointer`: Passes its two parameters on to another routine together with the address of its own stack slot, then returns.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `format_and_output_text`: Formats text into a 140-byte stack buffer with the system formatter, measures the resulting string, and passes any non-empty text with its length to a write routine.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### addbuffers

- `parse_and_add_volume_buffers`: Finds the device named by the DRIVE argument and adds the number of buffers given by BUFFERS. It prints the resulting buffer count, or reports an invalid device or volume name.

### Path

- `find_max_and_prepare_buffer_13`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block and copies sixteen long words into a buffer before invoking a routine pointer.
- `copy_buffer_block_3`: Copies a fixed block of sixteen words from a source table into a destination buffer when the destination pointer is valid, then invokes another routine.

### Resident

- `find_max_and_prepare_buffer_14`: Walks a linked list of entries to find the maximum value in a field, then stores that maximum along with a small header block into a buffer.
- `copy_words_then_call_4`: Copies a fixed number of words from a source table into a destination buffer when the destination is not null, then calls another routine.

### Assign

- `find_max_and_prepare_buffer_15`: Walks a linked list of entries to find the maximum value in a field, then, when a condition is met, writes a small header block and copies sixteen long words into a target buffer before invoking a routine pointer.
- `copy_buffer_and_call_3`: Copies a fixed block of sixteen words from a source table into a destination buffer when the buffer pointer is valid, then invokes another routine.

### EndCLI

- `find_max_and_prepare_buffer_4`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block and copies sixteen long words into a target buffer before invoking a routine pointer.
- `copy_block_and_call`: Copies a fixed block of words from a source table into a destination buffer when the destination is not null, then invokes another routine.

### Rename

- `find_max_prepare_buffer`: Walks a linked list of entries to find the largest value in a field, then, when a condition is met, writes a small header block and copies sixteen long words into a target buffer before invoking a routine pointer.
- `copy_words_and_call`: Copies a fixed number of consecutive words from one memory location to another when the destination is not null, then invokes another routine.

### Dir

- `find_max_and_prepare_buffer_5`: Walks a linked list of entries to find the maximum value in a field, then writes a small header block containing that value and a sequence of incremented words.
- `copy_words_and_call_2`: Copies a fixed count of words from a source table into a destination buffer when the destination is not null, then invokes another routine.

### NewCLI

- `find_max_and_prepare_buffer_6`: Walks a linked list of entries to find the maximum value in a field, then writes a small header block containing that value and a run of related data.
- `copy_buffer_and_call`: Copies a fixed 15-word block of data from a source table pointed to by a register into a destination buffer passed as an argument, repeating the copy only when that buffer is not null.

### NewShell

- `find_max_and_prepare_buffer_7`: Walks a linked list of entries to find the maximum value stored in a field, then, if a condition is met, writes a small header block and copies sixteen long words into a target buffer before invoking a routine pointer.
- `copy_buffer_block`: Copies a fixed block of sixteen words from a source table into a destination buffer when the buffer pointer is valid, then invokes another routine.

### Echo

- `find_max_and_prepare_buffer_8`: Walks a linked list of entries to find the maximum value in a field, then, when a condition is met, writes a small header block containing that maximum and copies sixteen long words into a caller-provided buffer before invoking another routine.
- `copy_words_and_call_3`: Copies a fixed block of 15 words from a source table into a destination buffer when the destination is valid, then invokes another routine.

### cls

- `write_clear_screen_sequence`: Opens dos.library and writes a fixed screen-clearing sequence to the output handle.

### Execute

- `find_max_prepare_buffer_2`: Walks a linked list of entries to locate the largest value in a field, then builds a small header block and copies a block of sixteen long words before invoking another routine.
- `copy_words_then_call`: Copies a fixed number of words from one table to another when the destination is not null, then invokes another routine.

### Copy

- `find_max_and_prepare_buffer_9`: Walks a linked list of entries to find the largest value in a field, then, when a condition is met, builds a small header block, fills sixteen long words from a fixed source, and stores a routine pointer before invoking it.
- `copy_buffer_block_2`: Copies a fixed block of sixteen words from a source table into a destination buffer when the destination is not null, then invokes another routine.

### Makedir

- `find_max_and_prepare_buffer_10`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block and copies sixteen long words into a caller-provided buffer before invoking another routine.
- `copy_words_from_struct_buffer`: Copies sixteen words from a fixed offset in a global structure into a caller-provided buffer when that buffer is not null, then jumps through a stored pointer.

### List

- `find_max_and_fill_buffer`: Walks a linked list of entries to find the maximum value in a field, then writes that value and a small header into a buffer, fills a region with a repeating pattern, and copies sixteen long words before invoking another routine.
- `copy_words_then_call_3`: Copies a fixed block of words from a source table into a destination buffer when the destination is valid, then invokes another routine.

### CD

- `find_max_and_prepare_buffer_11`: Walks a linked list of entries to find the maximum value in a field, then, when a condition is met, writes that value and a small header into a buffer.
- `copy_words_and_call_4`: Copies a fixed number of words from a source table into a destination buffer when the destination is not null, then invokes another routine.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `find_max_and_prepare_buffer_12`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block and copies sixteen long words into a target buffer before invoking another routine.
- `copy_words_from_table`: Copies a fixed number of words from a source table pointed to by a register into a destination buffer passed as a parameter, stopping after the count reaches zero.

### LoadWB

- `allocate_control_record_and_wait_2`: Allocates a control record for the current task, takes the current directory lock and the task's message port, and opens dos.library.
- `release_task_resources_and_reply_2`: Finds the current task and stores the supplied value in its record.
- `start_workbench_task_from_tag`: Opens the icon and intuition libraries, allocates a record, and starts the Workbench task from its resident tag.
- `compare_null_terminated_strings`: Compares two null-terminated strings byte by byte.
- `build_linked_list_from_table`: Walks a table of pointers, allocating a zeroed node for each entry, copying a looked-up value into it, and linking the nodes into a list.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_function_from_global`: Invokes a function pointer taken from a fixed offset within a global variable structure, then returns.
- `call_function_from_global_offset`: Invokes a function pointer taken from a different fixed offset within a global variable structure, then returns.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `pad_release_string_with_dots`: Checks a release digit in a text buffer and, when the stored prefix matches, writes dotted space padding after it.
- `write_dotted_padding_bytes`: Writes a dot followed by four spaces into a text buffer at a fixed position.
- `compare_stored_prefix_to_buffer`: Compares a stored prefix string against a text buffer, advancing through both while the bytes match and stopping at the end of the prefix or the first mismatch.
- `empty_return_stub`: Contains only a return instruction, so it does no work when called.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `dispatch_task_stub`: Calls the Dispatch entry of the stored exec library base and returns, handing control to the task scheduler.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_other_routine_with_stack`: Calls another routine, passing the two incoming parameters together with a pointer to a local stack slot, then returns.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_output_byte`: Stores the low byte of its input register at the address held in the pointer register and returns.

### Version

- `parse_command_line_tokens`: Parses a command-line string into individual tokens, splitting on spaces and tabs while handling quoted sections and escape sequences, then stores the resulting tokens in a table.
- `store_param_and_invoke_callbacks`: Fetches a structure pointer from a global variable, stores the incoming parameter into a slot at a fixed offset within that structure, and then invokes a series of callbacks from a table based on flag checks before returning the previously stored value.
- `report_library_or_device_version`: Parses the library name, version and revision arguments, opens the named library or device, and prints its version.
- `forward_with_fixed_flag_value`: Passes the first parameter to one routine together with a fixed flag value of 128, then passes the second parameter to another routine.
- `parse_signed_decimal_string`: Parses an optional plus or minus sign followed by decimal digits into an integer.
- `compare_strings_byte_by_byte`: Walks two null-terminated strings in lockstep, comparing each pair of bytes, and returns a negative value when the first string sorts before the second, a positive value when it sorts after, and zero when they match.
- `calculate_string_length`: Returns the length of a null-terminated string by counting bytes up to the terminator.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `invoke_callback_from_structure`: Dereferences a function pointer stored at a fixed offset within a global structure and invokes it, then returns.
- `recurse_with_stack_slot`: Calls another routine passing its two parameters along with the address of a slot on the current stack frame.
- `invoke_system_library_call`: It invokes the routine whose entry point is stored in the system library's function table, passing no arguments.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_indirect_global`: It invokes the function whose address is stored in a global variable and then returns.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_routine_via_table`: It invokes the function whose address is stored at a fixed offset within a global routine table, then returns.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer held in a global variable, then walks a stack buffer computing a rolling checksum until it reaches a null terminator, and forwards the buffer along with the inverted checksum to another routine when that checksum is non-zero.
- `store_byte_from_register`: Copies a single byte held in a data register into the memory location pointed to by an address register, then returns.

### SetMap

- `initialize_map_structure`: Initializes a global structure by calling its setup routines and storing pointers and values into its fields, then returns a status code.
- `restore_patch_state`: Reads a pointer from a global structure, saves the incoming parameter into a slot within the pointed-to structure, conditionally runs several cleanup callbacks from a table while recording specific values before each, and finally returns the saved parameter.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `call_indirect_from_d1`: Invokes the function whose entry point is stored at a fixed offset from a data register, then returns without using the result.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `reschedule_task_stub`: Calls the Reschedule entry of the stored exec library base and returns, letting the scheduler run other tasks.
- `unlock_stored_file_locks`: Walks a table of stored file locks and unlocks each non-zero entry, then releases a few remaining locks and the supplied lock.
- `save_params_and_run_setup`: Saves its two parameters in global variables, calls a setup routine with fixed numeric arguments, then calls a finishing routine with the value twenty.
- `load_keymap_into_console`: Opens the keymap resource, console device and icon library, reads the KEYMAP tool type from the icon, and loads the named keymap file from the keymaps directory.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_indirect_from_d1_2`: Fetches a routine pointer from a fixed offset in a data register and jumps to it, discarding any returned value.
- `call_indirect_from_d1_3`: Loads a function address from a fixed offset in a data register and calls it, returning immediately afterward.
- `call_indirect_from_d1_4`: Dereferences a pointer stored at a fixed offset from a data register and invokes the pointed-to routine.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_indirect_from_d1_5`: Calls the routine whose address is held at a fixed offset in a data register, then returns without using the outcome.
- `call_indirect_from_d1_6`: Fetches a function pointer from a fixed offset in a data register and jumps to it, discarding any returned value.
- `call_indirect_from_d1_7`: Loads a routine address from a fixed offset in a data register and calls it, returning immediately afterward.
- `call_indirect_ptr`: Dereferences a pointer stored in a global variable and jumps to the routine it holds, then returns.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_indirect_d1_offset`: It invokes the routine whose pointer is stored at a global variable located 22 bytes before the address given by register d1, then returns.
- `invoke_indirect_pointer`: Dereferences a pointer stored at an offset from a global variable and invokes the routine it points to.
- `call_global_function_pointer`: Dereferences a function pointer stored in a global structure and jumps to it without passing any argument.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `forward_argument_to_library_call`: Passes its single argument to a library routine together with a stored base pointer and the address of a stack slot, then returns.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer_3`: Loads and runs a function pointer stored in a global variable, then scans a stack buffer byte by byte computing a rolling checksum until it reaches a null terminator.
- `store_byte_from_register_2`: Stores the low byte held in a register into the memory location addressed by a pointer register, then returns.

### More

- `init_and_tokenize_string`: Initializes a small structure by storing a parameter and a stack pointer into it, then branches based on the result of a check.
- `restore_patch_state_2`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `display_file_pages_in_window`: Opens intuition.library and a window, then reads the named file and shows it page by page.
- `apply_helper_to_four_globals`: Calls one routine four times, passing a field of its argument together with each of four global values in turn.
- `check_buffer_for_binary_bytes`: Fills an 80-byte buffer through two routine calls, then scans it for control bytes.
- `read_key_with_break_check`: Waits for input with a timeout and checks for a break signal, returning zero when a break arrives.
- `run_external_editor_command`: Passes a global command string to a routine, then calls a handler pointer taken from a register.
- `run_page_display_loop`: Sets the first two state fields, calls a setup routine, then repeats a page routine while more lines remain to show.
- `process_page_buffer_and_count`: Runs a text-processing routine on the page buffer, stores its result in a state field, and increments a counter in a state record when a flag is clear.
- `find_next_line_break`: Reads a chunk into a buffer, records its size, and scans for a newline or form-feed character to find the end of the next line.
- `run_pager_key_loop`: Chooses how many lines to show from state flags, draws them in a loop, and reads a key after each pass.
- `search_file_for_pattern`: Chooses a search string based on a flag, runs a find routine, and repeats the search while a counter stays positive.
- `run_action_between_setup_calls`: Runs a fixed sequence of setup and reschedule calls around a main action, then returns that action's result.
- `call_action_with_optional_follow`: Calls a routine with two arguments, then calls another routine with the first argument alone when a flag in the state record is clear.
- `draw_then_update_line_count`: Calls a drawing routine with a stored handle and value, then calls a second routine with the line-count field from the state record.
- `draw_then_decrement_line_count`: Calls a drawing routine with a zero argument and a value of one, then calls another routine with the line-count field minus one.
- `draw_text_lines_until_limit`: Iterates over the text buffer, drawing lines through a handler pointer and tracking the line count.
- `call_pair_with_state_field`: Calls one routine with a state record, then calls another with a field of that record, a global buffer and the first argument.
- `reset_position_and_call_routine`: Sets a position counter to one when a flag in the state record is clear, then calls a routine with a field of the record and a global.
- `call_routine_with_selected_offsets`: Calls a routine with a field of the state record and two arguments chosen by a flag.
- `print_formatted_pair_or_count`: Calls a routine that prints a formatted line, using two global string arguments or a counted value when a flag is set.
- `query_and_clamp_result`: Calls a routine once with a fixed argument, then calls it again with an argument of minus one and returns that result, clamped to zero when negative.
- `copy_available_bytes_from_buffer`: Copies up to a requested number of bytes from a buffer region into the output, limited to the bytes remaining.
- `refill_read_buffer`: Flushes buffered bytes when a full block is pending, then reads up to a fixed block size of 4096 bytes into the buffer and records the byte count.
- `scroll_window_to_position`: Works out a target position from an offset and a mode argument, checks it against the window bounds, and moves the window there.
- `read_line_into_buffer`: Reads bytes one at a time through a read routine into a buffer until a newline or a fixed length limit is reached, then terminates the string.
- `copy_null_terminated_string`: Copies bytes from a source string into a destination buffer, including the terminating zero byte.
- `convert_string_to_uppercase`: Converts each lowercase ASCII letter in a null-terminated string to uppercase in place.
- `parse_signed_decimal_2`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `release_handle_if_present`: Passes the argument and a fixed value of minus one to a library routine using the stored handle, when both the argument and the stored handle are non-null.
- `print_text_and_free_record`: Releases the sub-records of a display record, then either outputs the given text or only releases the record.
- `call_with_single_value_argument`: Stores its argument in a one-element array, then calls a library routine with that array, a fixed code and a count of one.
- `read_record_fields_into_struct`: Allocates a small cleared record, passes its pointer to a library routine, copies two of its fields into the caller's structure, and then frees the record.
- `allocate_linked_record_from_template`: Allocates a record, links its list fields to itself and to a template, and copies a given number of values from the source block into it.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `reschedule_task_stub_2`: Calls the Reschedule entry of the stored exec library base and returns.
- `switch_task_stub`: Calls the Switch entry of the stored exec library base and returns, handing the processor to another task.
- `call_indirect_via_d1`: It invokes the routine whose address is stored at the global variable located 60 bytes below the address currently in register d1, then returns.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `allocate_memory_stub`: Calls the AllocMem entry of the stored exec library base and returns its result.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `invoke_stored_function`: This routine calls the function whose address is stored at the beginning of a block of memory, passing no arguments.
- `call_indirect_via_table`: Invokes a function pointer obtained from a table indexed by a value in register d1, then returns.
- `call_indirect_d1`: This routine invokes the function whose address is stored at the location pointed to by the value in register d1, then returns without using the result.
- `call_indirect_global_2`: It invokes the routine whose address is stored in a global variable and then returns.
- `call_indirect_via_d1_2`: It invokes the routine whose address is stored at the offset pointed to by the value in register d1, then returns.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_indirect_global_3`: It invokes the routine whose address is stored in a global variable and then returns.
- `invoke_indirect_handler`: It invokes the routine whose address is stored at the offset located 372 bytes below a global base pointer, then returns.
- `invoke_indirect_caller`: This routine invokes the function whose address is stored at the pointer located 384 bytes before a base value in register d1, then returns.
- `call_indirect_d1_offset_2`: It invokes the routine whose address is stored at the offset located 414 bytes below a global variable pointer, then returns without using any library calls.
- `call_routine_table_entry`: It invokes the function whose address is stored at a fixed offset within a global routine table, then returns without using any return value.
- `call_exec_entry_stub`: Calls one exec library entry through the stored base and returns, with no processing of its own.
- `recurse_with_stack_slot_2`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `read_one_byte_or_fail`: Reads one byte through a read routine into a stack buffer and returns it.
- `invoke_and_checksum_buffer`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack-allocated buffer until a null terminator is found.
- `store_byte_via_a3`: Stores a byte from data register D0 through the pointer held in address register A3.
- `unsigned_divide_with_remainder`: Performs an unsigned 32-bit division by shift and subtract when the divisor is large, or a 16-bit modulo when it fits in 16 bits.
- `multiply_longs_via_partials`: Multiplies two 32-bit values from 16-bit partial products and returns the result together with a byte-swapped copy of the second input.
- `call_then_return_stub`: Calls one routine with no arguments and returns, doing nothing else.
- `call_and_negate_on_flag`: Calls a routine and returns its first result, or the negation of that result when a second register is non-zero.
- `raw_format_string_stub`: Calls the RawDoFmt entry of the stored exec library base and returns, formatting text through the supplied callbacks.
- `store_byte_a3`: Stores the low byte of register D0 into the byte pointed to by the A3 register.
- `call_indirect_global_4`: This routine invokes the function whose address is stored at an offset within a global variable, passing no arguments.
- `allocate_signal_and_message`: Allocates a signal bit and a message structure, then fills in the reply port, the signal number and the message type.
- `release_signal_and_message`: Removes the message from its port list when it is linked, marks its status fields as released, then frees its signal bit and its memory.
- `search_string_for_pattern`: Checks whether the second string appears in the first, stopping at a newline.
- `initialize_empty_list_head`: Points the first field at itself with an offset of four, clears the second field, and points the third field at itself, forming an empty list head.

### Format

- `initialize_task_dispatch`: Initializes a task control block by filling in its fields from the incoming registers, then dispatches based on a command code.
- `restore_stored_parameter`: Fetches a pointer from a global structure and stores the input parameter into a slot within the pointed-to structure.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `allocate_memory_block`: Allocates a block of memory through a system routine and returns no value.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_debug`: Library stub: loads its arguments and the exec library base, calls Debug and returns its result to the caller.
- `switch_active_task`: Switches the active task through a system routine and returns no value.
- `forbid_multitasking_stub`: Calls the Forbid entry of the stored exec library base and returns, which stops task switching until a matching permit call.
- `dispatch_library_message`: Dispatches a message through a system routine and returns no value.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `schedule_task_stub`: Calls the Schedule entry of the stored exec library base and returns.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `allocate_absolute_memory_stub`: Calls the AllocAbs entry of the stored exec library base, which allocates memory at a given address, and returns its result.
- `reschedule_task_queue`: Reschedules the task queue through a system routine and returns no value.
- `run_format_disk_command`: Parses the DRIVE, NAME, FFS, NOICONS and QUICK options, checks that the drive exists, and prompts for a disk before initializing it.
- `free_format_buffers`: Frees the buffers the format command allocated, passing each stored pointer with its size to the free routine, and releases the supplied parameter last.
- `report_disk_io_error`: Retries a disk operation while it reports a recoverable condition, then maps the error code to a message such as bad sector, seek error, disk changed or cannot open device.
- `report_format_drive_status`: Runs the drive preparation calls, then prints the matching message for a write-protected disk, no disk in the drive, or a format error.
- `store_matching_drive_code`: Looks up the given code in an eight-entry table and stores the matching value in a field of a record returned by a routine.
- `find_drive_by_name`: Walks the list of device nodes looking for a volume whose name matches the given drive text.
- `calculate_string_length_2`: Walks through a null-terminated string starting at the input pointer, advancing one byte at a time until reaching the terminator.
- `copy_cstring_to_buffer`: Copies a null-terminated byte string from the source pointer to the destination pointer, advancing both pointers and writing each byte until a zero byte is written.
- `convert_char_to_uppercase`: Converts one lowercase ASCII letter to uppercase and returns other characters unchanged.
- `compare_strings_ignoring_case`: Compares two null-terminated strings one byte at a time after converting each byte to uppercase.
- `truncate_at_colon`: Cuts a string at its first colon by writing a zero byte there, or at its end when no colon is present.
- `check_status_with_error_code`: Passes its first argument in a one-element array to a routine with a fixed code, then returns an error code of one hundred twenty-six when that call fails and zero otherwise.
- `allocate_linked_record_from_template_2`: Allocates a record, links its list fields to itself and to a template, and copies up to seven values from the source block into it.
- `invoke_library_routine`: Invokes a routine from a specific system library and returns no value.
- `call_second_library`: Invokes a routine from another system library and returns no value.
- `call_third_library`: Invokes a routine from a third system library and returns no value.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_table_routine`: Fetches a function pointer from a global table and invokes it, returning whatever that routine produces.
- `invoke_dispatched_call`: Loads a routine address from a global dispatch table and jumps to it, then returns.
- `call_indirect_table`: Reads a code pointer from a global table entry and calls the referenced routine before returning.
- `call_table_entry`: Retrieves a pointer from a global table and branches to the routine it names, then returns.
- `invoke_table_pointer`: Fetches a function pointer from a global table slot and executes the pointed-to routine.
- `call_dispatched_routine`: Loads a routine address from a global dispatch table and calls it, then returns.
- `call_indirect_entry`: Reads a code pointer from a global table and branches to the referenced routine before returning.
- `invoke_table_routine`: Retrieves a pointer from a global table slot and invokes the routine it addresses, then returns.
- `call_table_dispatch`: Fetches a function pointer from a global table and calls the pointed-to routine before returning.
- `call_dispatch_pointer`: Loads a routine address from a global dispatch table and branches to it, then returns.
- `call_indirect_handler`: Fetches a function pointer from a global table and jumps to it, acting as an indirect call trampoline.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `invoke_dispatched_routine`: Loads a routine address from a global dispatch table and transfers control to it via an indirect jump.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_stored_pointer`: Reads a stored code pointer from a global table and calls the referenced routine through it.
- `invoke_stored_function_with_stack`: Takes the address of the current stack location and forwards it along with a parameter to a stored function pointer at the start of a memory block, passing no arguments.
- `invoke_table_routine_2`: Fetches a function pointer from a global table and executes the routine it points to via an indirect jump.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `format_disk_cylinder_by_cylinder`: Runs the format loop cylinder by cylinder through the disk port, reporting progress with the cylinder count to go.
- `configure_disk_port_timing`: Sets timing values on the disk port using two fixed values and an enable value, then passes the parameter through the port routine.
- `invoke_function_and_checksum_stack`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over the stack buffer until a null terminator is found.
- `store_byte_via_pointer`: Stores the low byte of a register into the memory location addressed by a preserved pointer.
- `divide_32bit_16bit`: Divides a 32-bit value by a 16-bit value, using a bit-by-bit restoring loop for large divisors and hardware division otherwise, leaving the quotient and remainder in place.
- `restoring_division_core`: Runs the restoring-division inner loop for sixteen steps on the dividend and divisor registers, leaving the quotient and remainder in place.
- `multiply_32_bit_integers`: Multiplies two 32-bit inputs using 16-bit partial products, returning the low 32 bits of the product.
- `call_routine_from_global_table`: Calls the routine whose address is stored at a fixed offset inside a global data block.
- `store_byte_through_pointer`: Writes the low byte of the first data register to the memory location held in the A3 pointer register.
- `allocate_and_initialize_block`: Allocates a fixed-size memory block with a library call and fills in a stored parameter, a flag byte and a value from a second call.
- `reset_object_and_recurse`: When an object's link field is set, it first processes the object again through the same routine.
- `forward_argument_with_fixed_code`: Passes its argument to another routine together with a fixed numeric code.
- `pass_argument_through`: Passes its single argument straight to another routine and returns nothing.
- `setup_trashcan_drawer_state`: Saves a field of an existing object, fills a block of global variables from that object or from defaults, and calls library routines to build the related functions.
- `read_signed_byte_or_fail`: Reads one byte through a routine that takes a handle, a buffer and a count of one.
- `call_table_handler`: Retrieves a handler address from a global table and jumps to it, serving as an indirect call shim.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `setup_pointer_chain`: Points the first longword of a block to the field four bytes past its start, clears the next longword, and stores the block's own address in the third longword.
- `store_fields_in_new_node`: If the first argument is non-null, allocates a node with a library call and stores a byte value, a word from the second argument and the pointer into fixed fields of it.
- `reset_node_and_call_routine`: If given a non-null node, sets its byte field and two longword fields to all ones, then passes the node and a stored word to another routine and returns that result.
- `call_library_routine_via_global`: Calls a library function through a global library base at a fixed negative offset, passing no arguments.
- `call_library_vector_from_global`: Calls a second library function through the same global library base at a different fixed negative offset, passing no arguments.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.

### CLI

- `parse_command_line_and_run`: Program entry point that allocates a task structure, splits the command-line text into words (handling quoted text and star escapes), and calls the main routine.
- `call_exec_supervisor_function`: Calls the exec Supervisor function through a stored library base, passing no arguments.
- `exit_interrupt_handler`: Ends an interrupt handler by calling the exec ExitIntr function through the stored library base.
- `invoke_task_switch_hook`: Calls the exec Switch function through a stored library base, passing no arguments.
- `invoke_task_dispatch_function`: Calls the exec Dispatch function through a stored library base, passing no arguments.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_exec_make_functions`: Calls the exec MakeFunctions routine through a stored library base, passing no arguments from this code.
- `call_exec_find_resident`: Calls the exec FindResident routine through a stored library base, passing no arguments from this code.
- `enable_interrupts_through_exec`: Calls the exec Enable routine through a stored library base to re-allow interrupts.
- `call_exec_alloc_entry`: Calls the exec AllocEntry routine through a stored library base, passing no arguments from this code.
- `open_shell_window_and_start`: Creates a new command shell by allocating a state block and opening a console window that runs the CLI or Shell startup script.
- `build_linked_node_list`: Builds a singly linked list from a source list, allocating an 8-byte node for each entry and storing a value derived from the entry's second field.
- `free_linked_node_list`: Walks a linked list of nodes, releasing the value stored in each node's second field and then the node itself.
- `parse_signed_decimal`: Reads a null-terminated string of decimal digits with an optional leading plus or minus sign and returns the signed integer value.
- `call_library_vector_no_args`: Calls a library function through a vector in a global library base, passing no arguments.
- `call_library_vector_no_args_2`: Calls another library function through a vector in a global library base, passing no arguments.
- `call_dispatched_pointer`: Loads a routine address from a global dispatch table and transfers control to it through an indirect jump.
- `invoke_indirect_handler_2`: Reads a stored function pointer from a global table and jumps to the referenced routine.
- `call_table_pointer`: Fetches a code pointer from a global table and calls the routine it addresses via an indirect jump.
- `call_indirect_from_data`: Reads a function pointer from a fixed data location and jumps to it, returning immediately after the call.
- `call_library_entry`: It invokes the routine whose address is stored at the start of the library's data block and then returns, performing no work itself.
- `call_library_entry_2`: It invokes the routine whose address is stored at the beginning of the AmigaOS library, performing a simple indirect call.
- `divide_32_bit_with_remainder`: Divides a 32-bit dividend by a divisor and returns the quotient and remainder together.
- `multiply_32_bit_integers_2`: Multiplies two 32-bit inputs using 16-bit partial products, returning the low 32 bits of the product with the second input's halves swapped.
- `negate_result_if_flagged`: Calls another routine and returns its first result value, negated when the second result register is non-zero.

### Disk-Validator

- `validate_disk_block_bitmap`: Walks the disk block bitmap block by block, comparing each bit with its expected allocation state and counting mismatches.

### Ram-Handler

- `copy_memory_via_exec`: Calls the exec CopyMem routine through a stored library base, then makes an indirect call whose target could not be determined.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_BLIT

- `free_blocks_and_reset_slot`: Calls the exec FreeMem routine several times in a loop, then looks up a value in a global structure and, when the slot matches a fixed constant, swaps it with a global and clears that global.
- `decrement_until_minus_one`: Spins a 16-bit counter down from its starting value until it reaches minus one, a busy wait with no other effect.
- `decode_bitstream_then_clear_cache`: Reads a bit-packed stream and processes the decoded values in seven passes.
- `continue_bitstream_decode_loop`: Continues a bit-packed decoding loop that shares its bit buffer and state with the routine before it.
- `decode_bitstream_into_buffer`: Decodes a bit-packed stream by shifting a bit buffer, then adds values backwards through a destination block.
- `copy_decoded_bytes_backwards`: Writes decoded bytes into a destination block by copying groups of four bytes backwards from the source, reading bit-packed counts as it goes.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### d

- `empty_routine_2`: This routine contains no executable instructions; it is empty and performs no operations.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_CPU

- `initialize_game_state`: Runs a sequence of function calls, executes a small loop that invokes a function pointer taken from a global variable, clears a word at a register-derived address, then reads a value from a structure and swaps a table entry with a RAM variable when a specific constant matches.
- `decrement_until_minus_one_2`: Spins a 16-bit counter down from its starting value until it reaches negative one, performing a busy wait with no other effect.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Ambermoon

- `init_system_memory`: Waits out two short busy-wait loops, then runs a small set of setup routines and a four-iteration loop that invokes a handler from a table.
- `decrement_until_minus_one_3`: Repeatedly subtracts one from a signed 16-bit value until it equals negative one, then exits.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Ambermoon_install

- `initialize_game_state_2`: Calls another routine several times, runs a small loop that invokes a function pointer taken from a global variable, clears a word at a register-derived address, then reads a value from a structure and swaps a table entry with a RAM variable when a table entry matches a constant.
- `decrement_to_minus_one`: Repeatedly subtracts one from a local signed value until that value equals negative one, then exits.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_device_io_and_libraries`: Opens required system libraries, verifies their versions, and sets up device I/O requests.
- `examine_locked_object_field_2`: Allocates memory, locks an object through a library, and examines its fields, performing various cleanup or conditional jumps based on the results.
- `examine_locked_object_field`: Locks an object through the second library and examines it.
- `dispatch_table_entry_by_index`: Looks up an index through a library call; if the index is zero, it releases a lock and returns minus one, otherwise it calls a routine from a table and releases the lock.
- `open_then_close_file`: Opens a file through a library, and if successful, stores the handle in a global variable, calls three routines, and then closes the handle.
- `open_show_error_then_close`: Opens a file through a library and, if successful, executes a sequence of routines, including an error-handling routine if a status value is two or three, before closing the file.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `write_while_data_pending`: Calls a library function if a check routine returns non-zero, then enters a loop that repeatedly calls a library function and another routine as long as a condition remains met.
- `write_if_check_passes`: Calls a library function through a pointer in a global variable only if a preceding routine returns a non-zero value.
- `find_word_in_terminated_table`: Scans a table of 16-bit entries ending with a minus-one marker to find an entry matching the value in the data register, leaving the resulting position in the address register.
- `measure_string_length`: Counts the characters of a null-terminated string addressed by the address register and returns the count in the data register.
- `wait_for_hardware_status`: Sets a global variable to zero and enters an infinite loop if a specific bit in a memory location is set, while polling a hardware register.
- `write_interrupt_request`: Writes a value to the interrupt request register.
- `probe_floppy_drive_entry`: When a state cell is zero, it attempts a lock through the second library and records whether a returned status equals a fixed code.
- `parse_unit_number_after_prefix`: Compares the input string against a fixed prefix stored in the program.
- `set_flag_if_field_zero`: Sets a global flag to minus one when a field of the structure it reads is zero, and to zero otherwise.
- `wait_for_port_message`: Waits on a message port, takes the message from it through the system library, and calls the enable routine of the library when a field of that message is nonzero.
- `release_forbid_and_reply`: When a message pointer is set, forbids task switching, replies to the message through the system library and clears the pointer.
- `open_library_and_read_version`: Opens libraries through the system open entry, storing each returned base in a global, and finally reads the 16-bit version field of the last base into the global.
- `close_held_libraries`: Closes the library whose base is held in a global through the system close entry, once for each of three checks that find the global nonzero.
- `call_entry_and_store_flag`: Calls a library entry through its vector and stores minus one in a global when the call returns nonzero, and zero otherwise.
- `step_to_parent_if_flagged`: When a flag in a global is set, calls the parent-directory entry of the second library through its vector, with no arguments set here, and returns nothing.
- `setup_shared_record_state`: When a flag is clear, allocates memory and runs several system library calls, exchanges a field of a shared record with a saved value, and when a lookup succeeds, does the same for a second record.
- `clear_shared_record_entry`: When a flag is set, stores a value in a shared record, looks up and closes an entry through the second library, calls two more library entries and then clears the flag.
- `store_current_task_pointer`: Calls the task lookup entry of the system library and stores the returned task pointer in a global.
- `store_adjusted_char_then_call`: If a global variable is zero, it adds sixty-four to a character from a pointer and stores it in a fixed byte cell before calling another routine; otherwise, it simply calls that routine.
- `copy_null_terminated_string_2`: Copies characters from the source pointer to the destination pointer, one at a time, until a zero byte has been copied.
- `set_flag_if_field_below`: Sets a global flag to minus one when a 16-bit field of a record, reduced by thirty-six, is negative, and to zero otherwise.
- `setup_device_io_request`: Allocates a signal and, if successful, opens a device and performs an I/O operation using a prepared request, with cleanup paths for failures.
- `clear_flag_after_library_call`: When a flag is set, calls a library entry and then clears the flag.
- `try_lock_with_fallback`: When a flag is clear, temporarily sets a field of a shared record to minus one, attempts a lock through the second library and restores the field.
- `undefined_routine`: The routine contains eleven instructions and is called by one unnamed routine, but the specific operations performed are not provided.
