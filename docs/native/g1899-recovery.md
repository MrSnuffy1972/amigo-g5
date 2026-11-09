# Game 1899 (historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01899). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (930320 top-level bytes in 75 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 585 of 585 named (0 author symbols), 585 of 585 described (27 by rule, 36 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3378 of 3378 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 34704 bytes, 186 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16799 of 17538 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 59911 |
| code | 35260 |
| data | 679230 |
| padding | 13071 |
| unused | 142848 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:install | 4 | 1616 | 63 |
| Run | 2 | 117 | 2 |
| Assign | 2 | 117 | 2 |
| endcli | 2 | 117 | 2 |
| BBStart | 5 | 546 | 24 |
| Execute | 2 | 117 | 2 |
| Copy | 2 | 117 | 2 |
| BBCopy | 4 | 463 | 27 |
| IconX | 12 | 806 | 37 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5872 | 234 |
| Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:intro.dwn#unpacked (raw code file, load address unknown) | 1 | 7228 | 186 |

## Cases handled

- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:CHAR6.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (426 -> 1108 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/HLINTRO2.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (786 -> 4756 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/BANGPAL.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (430 -> 1664 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/AUTO.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (51 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/SCROLL.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (17979 -> 40724 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/MORD.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (90639 -> 116752 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/BUCH1B.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (77284 -> 114794 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/BUCH1B.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/BANG.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (53853 -> 106202 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/gun.fx: TPWM: Turbo Packer-packed, unpacked with ancient (4066 -> 4550 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/BUCH1A.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (130281 -> 247037 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/CHR14.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (2030 -> 5834 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/HLINTRO3.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (567 -> 2708 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/BANG.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (154 -> 420 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/HLINTRO1.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (1051 -> 6804 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/AUTO.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (99101 -> 153520 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/MORD.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (51 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/FRONT.FX: TPWM: Turbo Packer-packed, unpacked with ancient (44639 -> 59376 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/BUCH1A.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:INTRO/hlintro4.sng: TPWM: Turbo Packer-packed, unpacked with ancient (1367 -> 8852 bytes)
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:intro.dwn: TPWM: Turbo Packer-packed, unpacked with ancient (21050 -> 34704 bytes)
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3050 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- historyline 1914-1918 (1992)(blue byte)(de)(disk 1 of 7)[a]/Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:intro.dwn#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)

## Cracktros (kept, not protection)

- Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:intro.dwn#unpacked (raw code file, load address unknown)

## Copy protection found

- `disk_error_prompt_and_retry_loop` (runtime segment an address): Opens a requester, draws a bordered box, prints disk error messages, waits for a key, then retries the disk operation until it succeeds or the user aborts. Kind: disk/track check. Pass path: The loop continues or returns via the branch where iVar2 != 0 (specifically the path leading to u13__sub_3EF2 and the return of iVar2, implying the error condition was cleared or the check passed)..

## Functions


### Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:install

- `run_program_entry_and_cleanup`: Program entry point: runs a message loop that opens and manages the program's window through library calls, then closes the windows and libraries it opened and returns zero.
- `report_dos_open_failure`: Stores the dos.library failure message pointer and calls the library routine at vector offset minus 348 to display it.
- `report_graphics_open_failure`: Stores the graphics.library failure message pointer and calls the library routine at vector offset minus 348 to display it.
- `report_window_open_failure`: Stores the window failure message pointer and calls the library routine at vector offset minus 348 to display it.
- `show_requester_with_handler`: Saves the caller's pointer in a handler slot and calls the library routine at vector offset minus 348, which shows a requester through the intuition library.
- `show_68020_install_prompt`: Stores the install prompt message pointer and calls the library routine at vector offset minus 348 to ask whether the 68020 version should be installed.
- `find_and_check_task`: Uses a global pointer to locate a task and checks a certain offset in the task structure, potentially calling other functions based on that check.
- `open_window_after_setup`: Calls a setup helper and, on success, opens a window through the library routine at vector offset minus 204, stores the window and its related pointer and returns zero.
- `call_library_pair_72_66`: Stores the result of the library routine at vector offset minus 72 and then calls the routine at vector offset minus 66 on the same library base.
- `call_three_intuition_routines`: Calls three library routines in sequence on the same base, at vector offsets minus 438, minus 432 and minus 462, and returns nothing.
- `call_two_intuition_routines`: Calls two library routines in sequence on the same base, at vector offsets minus 438 and minus 432, and returns nothing.
- `call_intuition_routine_444`: Calls the library routine at vector offset minus 444 and returns nothing.
- `call_intuition_function`: Performs an indirect call to a function located at an offset from a global pointer.
- `draw_border_twice`: Calls the library routine at vector offset minus 108 twice on the same base, which draws a border through the intuition library, and returns nothing.
- `open_workbench_and_layout`: Opens the Workbench screen through the library routine at vector offset minus 210 and, when that succeeds, stores its pointers, calls a routine at offset minus 474 and computes layout values from a stored count.
- `layout_gadget_rectangles`: Computes position and size fields for a set of gadget rectangles from window metrics, using one layout for state value 1 and another for state value 4, and returns minus one for any other state.
- `layout_wide_gadget_rectangles`: Computes gadget rectangle fields from window metrics with a wider row step than its neighbour, using one layout for state value 1 and another for state value 4, and returns minus one for any other state.
- `layout_gadget_from_argument`: Fills the gadget rectangle fields of the structure passed in the address register from window metrics, using one layout for state value 1 and another for state value 4, and returns minus one for any other state.
- `redraw_window_gadgets`: Calls library routines at vector offsets minus 342, minus 348 and minus 354 with the window pointer taken from the argument, then calls the routine at offset minus 306 with no arguments.
- `redraw_stored_window_gadgets`: Calls the library routines at vector offsets minus 342, minus 348 and minus 354 with a stored window pointer, then calls the routine at offset minus 306 with no arguments.
- `run_installer_event_loop`: Runs the installer's message loop: it waits on the window port, replies to each message and dispatches gadget selections.
- `check_and_trim_path_name`: Checks a name with a helper, trims the character at the position found in the name, checks it again and restores it.
- `normalize_directory_path_name`: Appends a colon or slash separator to a device or directory name when one is missing, creates the directory through helpers and returns a pointer to the resulting name, or to an empty string when creation fails.
- `write_install_batch_file`: Writes the install batch file line by line through helpers, comparing each returned value with the expected count and finishing with a closing step.
- `print_intuition_text_message`: Stores the text structure passed in the address register and calls the intuition library's text printing routine at vector offset minus 216 to draw it.
- `copy_string_address_registers`: Copies a zero-terminated string from the source address register to the destination address register, including the terminator.
- `count_string_characters`: Counts the characters of a zero-terminated string in the address register, excluding the terminator, and returns the count.
- `call_library2_vector_minus_54`: Calls the library function at vector offset minus 54 of the second library base stored in the state block, returning nothing.
- `find_char_in_string`: Scans a zero-terminated string for a fixed character and returns its index, or minus one when the character is absent.
- `copy_files_with_info_icons`: Walks the names of a directory listing, appends each name to a path and runs a copy step through helpers.
- `run_install_steps`: Runs a sequence of installation steps through helper calls, repeating the sequence while the helpers report success.
- `increment_progress_counter_then_call`: Increments a counter in the state block and calls a helper, returning the data register pair unchanged.
- `update_progress_value_and_call`: Increments a second counter when a flag byte is set, stores the stored total divided by 2048, with a minimum of 1, as a progress value, then calls a helper.
- `call_helper_pass_through`: Calls a helper and returns the data register pair unchanged.
- `execute_subroutine_and_return_result`: Calls another routine and returns its combined 64-bit result.
- `execute_subroutine_and_return_result_2`: An unreached routine that calls another routine and returns its 64-bit result.
- `calculate_progress_ratio`: calculates a ratio or clamped value representing the progress bar fill level.
- `draw_progress_bar_fill`: Computes the filled width of a progress bar as a fraction of its range, clamped to the bar's extent, draws the filled rectangle through the library routine at vector offset minus 342 and then refreshes the window with the routines at offsets minus 348, minus 354 and minus 306.
- `check_abort_request_messages`: Drains pending window messages, and when a close-window message arrives or gadget item 6 is selected, asks the user to confirm breaking the installation and returns a non-null value when the answer is yes.
- `clear_and_print_status_text`: Fills a 32-byte status buffer with spaces, then calls a helper and displays the buffer through the intuition text printing routine at vector offset minus 216.
- `allocate_work_buffer_or_fail`: Allocates a memory buffer through the exec library and returns zero on success, or a pointer to a not-enough-memory message when allocation fails.
- `free_work_buffer`: Releases the work buffer through the exec library when one is held, clears its pointer and returns zero.
- `free_second_work_buffer`: Releases the second work buffer through the exec library when one is held, clears its pointer and related counters, and returns zero.
- `reset_install_counters`: Sets three state variables to zero and returns nothing.
- `call_function_if_not_zero`: Checks if a global variable is non-zero and calls a function at an offset from a global pointer if so.
- `call_function_and_reset_variable`: Checks if a global variable is non-zero, calls a function at an offset from a global pointer, and clears that global variable.
- `execute_conditional_function`: Checks a global flag and, if non-zero, executes a function pointer retrieved from a table before resetting that flag to zero.
- `verify_and_clear_global_variable`: Checks if a global variable is non-zero, calls a function, and if the return value does not match a register, clears the global variable.
- `validate_range_and_set_global`: Checks a global variable, calls a function, and validates a range or value before clearing a global variable and returning a result.
- `set_flag_if_tag_matches`: Stores minus one or zero into a flag depending on whether a longword at byte offset 40 of a structure equals the four-character tag TPWM.
- `check_install_status`: Checks a global variable and calls a function, potentially clearing a global variable and setting another if a certain condition or return value is met.
- `update_global_pointer`: Updates a global variable with a return value from a function call and conditionally resets a global variable to a constant.
- `read_file_line_to_buffer`: Reads a sequence of bytes from a memory buffer into a global buffer until a newline or end of range is reached.
- `count_newline_characters`: Walks a text buffer of a given length and adds up its newline characters into a global counter.
- `evaluate_library_return`: Evaluates a return value from a function pointer or a global offset, and conditionally calls another function or returns a constant value.
- `decode_lz_stream_block`: Decodes a bit-flag compressed stream by alternately copying literal bytes and back-references into a 2048-byte window, with copy lengths from two to seventeen bytes.
- `decode_lz_stream_inner_loop`: Inner decoder loop that reads flag bits, writing literal bytes or copying two to seventeen bytes from earlier output into the destination buffer, stopping at the input limit.
- `decode_lz_refill_block`: Second variant of the bit-flag decoder that refills the flag byte after every eight bits, emitting literals or window copies and tracking the output count.
- `decode_lz_literal_runs_block`: Decoder loop variant that emits a literal byte, then reads flag bits to choose between further literals and window back-copies, returning the output count when the input is exhausted.
- `decode_lz_copy_then_literals`: Decoder loop that starts with a back-copy from the window and then alternates literal bytes with copies, stopping at the input or output limit.
- `decode_lz_offset_copy_loop`: Decoder loop that takes a copy length from the low nibble of a byte and combines it with the next byte to form a back-reference offset into the window.
- `decode_lz_run_length_block`: Decoder loop that counts down a repeat run and copies from the window, keeping the output count and returning an error marker if the limits are hit.
- `save_cpu_registers_to_context`: Copies nine incoming register values into consecutive longwords of a context structure and clears a global flag.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_and_invoke_callback`: Copies sixteen longwords from a fixed offset of a context block into the caller's buffer when one is supplied, then calls a stored entry point.

### Assign

- `build_assign_table_entry`: Scans a linked list for the largest stored value, then builds a new table of entries filled with an incrementing marker and a handler pointer, copying a sixteen-longword block; returns minus one when the lookups fail.
- `copy_stack_and_execute_function`: Copies a fixed number of dwords from a source pointer to the stack and then calls another routine.

### endcli

- `initialize_program_context`: Iterates through a linked list or table to find a maximum value, then initializes a context block by copying a sequence of incrementing values and copying a block of data from a source pointer to a destination pointer, ending with a pointer assignment to itself.
- `copy_stack_words`: Copies a fixed number of words from a memory location to the stack and then calls another routine.

### BBStart

- `bootstrap_and_launch_program`: Opens the DOS library, fetches the startup message, and launches a named script or program with its arguments through a library call, then restores saved task fields and closes the library.
- `receive_startup_message_args`: Finds the current task, waits for its startup message, copies the argument string or the default program name MAIN.PRG into a buffer, and returns zero or minus one.
- `apply_relocation_table_entries`: Checks a header marker, then walks a byte-encoded table of offsets, adding the load base to each longword it points at and skipping forward for the escape code.
- `read_file_until_complete`: Opens a named file, then calls a subroutine in a loop until it reports completion, returning the final status.
- `free_buffer_and_clear_pointer`: Frees the buffer stored in a context slot through a library call when the slot is non-zero, then clears that slot.
- `read_8k_block_into_buffer`: When the active flag is set, reads one 8192-byte block from the file handle into the buffer after a 40-byte header, reduces the remaining byte count, and returns an error if the read is short.
- `update_tpwm_tag_match_flag`: When a context flag is set, stores minus one or zero depending on whether the TPWM tag is found at byte offset 40 of the header structure.
- `clear_context_status_byte`: Clears a single status byte in the context block and returns zero.
- `initialise_decoder_state_flags`: Stores the result of an allocation call in the context; when that succeeds it sets three decoder state flags and returns zero, otherwise it returns minus one.
- `decode_lz_stream_8k_window`: Decodes flag-bit compressed input into the context output buffer with an 8192-byte window limit, writing literal bytes and back-references of two to seventeen bytes while tracking the remaining input count.
- `decode_lz_stream_counted`: Decodes flag-bit compressed input into a buffer, keeping the remaining input count in the context and stopping when it runs out; back-references copy two to seventeen earlier bytes.
- `decode_lz_refill_stream`: Decodes compressed input in which each flag byte is refilled after eight bits, emitting literals or window copies and keeping the input count in the context.
- `decode_lz_literal_copy_stream`: Decodes compressed input that emits a literal and then reads flag bits to choose literals or window copies, keeping its counters in the context.
- `decode_lz_nibble_copy_stream`: Decodes compressed input where a copy length comes from the low nibble of a byte and a window offset from the next byte, writing the copied bytes to the output buffer.
- `save_registers_and_clear_flag`: Clears a flag in the context and copies nine incoming register values into consecutive longwords of the context structure.
- `get_file_size_via_examine`: Allocates a file-info block, locks the named path, examines it, unlocks it and returns the size field from that block, freeing the block afterwards; returns minus one if any step fails.
- `read_from_dos_handle`: Calls the DOS Read routine on the stored file handle, passing the destination buffer and byte count.
- `close_dos_handle`: Calls the DOS Close routine on the handle passed in the stack frame to release a file.
- `open_dos_file_handle`: Calls the DOS Open routine on the file name in the stack frame and returns the resulting handle.
- `store_parameter_to_table`: Increments a global counter and stores a provided parameter and a value from a library call into a table.
- `store_parameter_to_table_2`: Stores a parameter and a value from a function call into a table at an offset determined by an incrementing counter.
- `clear_and_iterate_table`: Resets a global counter and iterates backwards to call a library routine using pointers to a table.
- `decrement_and_call_routine`: Decrements a global counter and calls a library routine if the counter is not zero.
- `copy_memory_block_longwords`: Copies a byte count from source to destination, moving sixteen-longword chunks where alignment allows, then the remaining longwords and single bytes.

### Execute

- `build_shell_command_table_entry`: Scans a linked list for the largest stored value, builds a new table of entries with an incrementing marker and a handler pointer, copies a sixteen-longword block, and returns minus one when the lookups fail.
- `copy_and_execute_block`: copies a block of sixteen-byte data from a memory location to the stack before branching to another routine.

### Copy

- `update_context_data_and_copy_block`: Calculates the maximum value in a linked list or table, updates a destination buffer with an incrementing pattern, and performs a block copy of a fixed number of longwords between a source and a destination buffer or context block.
- `copy_stack_words_2`: Copies a fixed number of words from a source pointer to a destination pointer in the stack.

### BBCopy

- `run_file_copy_session`: Opens dos.library, runs the file copy steps, closes the library and returns zero; returns the code 300 when the library cannot be opened.
- `write_null_terminated_string`: Measures a null-terminated string and writes it to the stored output handle through DOS Write.
- `copy_file_entries_loop`: Loops over the file list, reading each entry and copying its blocks until the list ends or an error occurs, then releases its buffers and handles.
- `check_free_memory_threshold`: Compares the free memory reported by exec AvailMem, less a recorded starting amount in kilobyte units, against a stored limit; returns zero when enough memory remains and one otherwise.
- `allocate_copy_buffer`: Allocates the copy buffer through exec AllocMem, returning null on success or a pointer to the error flag, which maps to the 'Not enough memory for buffer' message.
- `free_copy_buffer`: Frees the copy buffer through exec FreeMem when it is set, then clears the stored pointer.
- `free_secondary_buffer`: Frees the secondary buffer through exec FreeMem when it is set, then clears the stored pointer.
- `close_input_file_handle`: Closes the input file handle through DOS Close when it is set, then clears it.
- `close_output_file_handle`: Closes the output file handle through DOS Close when it is set, then clears it.
- `close_auxiliary_file_handle`: Closes the auxiliary file handle through DOS Close when it is set, then clears it.
- `write_8k_chunk_to_file`: Writes the next chunk of at most 8192 bytes to the output handle through DOS Write, reduces the remaining byte count, and returns an error pointer when fewer bytes were written than requested.
- `read_8k_chunk_from_file`: Reads up to 8192 bytes from the input handle through DOS Read, reduces the remaining byte count and returns an error when a short read leaves nothing to copy.
- `split_device_name_at_colon`: Scans the argument for a colon separating a device name from its path, terminates the name at the colon and returns zero, or minus one when no colon is present.
- `open_copy_destination_file`: Opens the destination file through DOS Open (offset -30) and returns a null pointer on success, or a pointer to the error flag with the 'Cannot open destination file' message.
- `open_copy_source_file`: Looks up the source file index, then opens the source file through DOS Open, returning an error flag when the file is missing or cannot be opened.
- `read_next_copy_filelist_line`: Loads the file list into allocated memory on first use via exec AllocMem and DOS Open and Read, then copies the next newline-terminated entry into an output buffer, failing at end of list.
- `count_filelist_newlines`: Counts the newline characters in the loaded file list and stores the total as the entry count.
- `find_character_index`: Returns the index of the first occurrence of a given character in a null-terminated string, or minus one when that character is absent.
- `create_directory_path_tree`: Creates each directory level of a path with DOS CreateDir, unlocking the lock returned by each call, and accepts the 'already exists' error codes as success; any other failure returns minus one.
- `check_path_exists_via_examine`: Locks a path, examines it and unlocks it, returning the examine result, or minus one when any of the three DOS calls fails.
- `parse_copy_command_arguments`: Checks the argument count and rejects a question-mark request, then reads four argument fields and stores their values, returning a flag pointer when the arguments are invalid; the usage text is copy FROM device TO device MEMORY.
- `parse_decimal_number_arg`: Parses decimal digits from the start of an argument string up to a newline, accumulating the value and returning it, or the input value unchanged when no digits are present.
- `split_command_argument_token`: Copies one space-delimited argument from the command line into a buffer, treating double-quoted segments as single tokens and stopping at the terminator.
- `match_keyword_ignoring_case`: Compares a command argument with a keyword such as FROM, converting lowercase letters to uppercase before comparing, and stops at the first difference.
- `compare_strings_ignore_case`: Compares two strings byte by byte while ignoring case-specific bit differences.
- `copy_bytes_until_null`: Copies bytes from a source pointer to a destination pointer until a zero byte is reached in the source, advancing both pointers.
- `store_standard_io_handles`: Calls DOS Input and Output to fetch the standard input and output handles and stores each one in its own global slot.

### IconX

- `run_icon_command_from_args`: Allocates a message record, opens dos.library and checks whether the program was started from Workbench, parses the command arguments into a list of strings with quote and escape handling, runs the icon command through a subroutine and releases the record; returns 20 when memory is short and 100 when the library cannot be opened.
- `restore_saved_context_and_cleanup`: Stores the given value in a saved slot, runs the conditional close and reply cleanup callbacks, frees the message record and returns the saved value.
- `setup_icon_console_window`: Opens a temporary console window with the icon's window settings, allocates a 1536-byte work buffer, configures the icon from the WINDOW or DELAY options and releases everything afterwards; prints an out-of-memory message on allocation failure.
- `process_icon_action_request`: Builds an argument buffer, looks up a handler, and when that handler returns a single dot response it runs a second handler call; afterwards it optionally waits or starts endcli depending on a flag.
- `initialize_icon_library_functions`: Recursively processes a pointer-based structure, calls a routine to create function entries, and then enables those functions.
- `build_icon_path_string`: Walks a linked list of path nodes to find the matching entry, builds its path in a 260-byte temporary buffer, replaces the first slash with a colon and frees the temporary memory.
- `join_three_strings_into_buffer`: Copies one string into a buffer, appends a second string, then appends a third string after the combined length.
- `count_string_length`: Walks a zero-terminated byte string and returns the number of bytes before the terminator.
- `copy_null_terminated_string`: Copies bytes from a source string to a destination string, including the terminator, stopping after the null byte.
- `skip_string_then_call`: Advances a pointer to the terminating zero of a string and passes it, with the unchanged second argument, on to another routine.
- `parse_signed_decimal`: Parses an optional sign and decimal digits from a string, accumulating the value and negating it when a minus sign was present.
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
- `forward_with_stack_slot`: Passes its two arguments on to another routine together with the address of a stack slot.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `format_text_and_pass_length`: Formats text into a 140-byte stack buffer through exec RawDoFmt, measures its length up to the terminator and passes the buffer and length on to another routine when the text is not empty.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `invoke_raw_format_entry`: Calls the RawDoFmt routine through a function pointer stored at a fixed offset in the global data structure.
- `store_byte_through_pointer`: Writes the byte in the data register to the address held in the output pointer register and returns.

### Disk-Validator

- `validate_disk_bitmap_blocks`: Walks the disk allocation bitmap in 32-bit words, compares each bit against the expected block state for the disk, counts mismatches and reports the result with a formatted numeric code.

### Ram-Handler

- `dispatch_ram_handler_copy`: Calls exec CopyMem through the library base, then transfers control through an indirect jump whose target the decompiler could not recover, so it acts as a dispatch stub.

### Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_three_library_vectors_and_spin`: Loads three library base addresses into d1 with d2 and d3 zeroed, calls the library vector at offset -222 through a6 three times, then branches into an infinite loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `save_stack_pointer_and_run_startup_calls`: Stores the stack pointer a7 into a pc-relative longword, calls two subroutines, tests d0 and branches if nonzero, then calls two more subroutines and loads a0 with an immediate address.
- `call_pc_relative_then_bsr_and_load_a0`: Computes a pc-relative address into a0 using its own value, jumps to that address, calls a nearby subroutine, then loads a0 with an immediate address.
- `setup_a4_a3_offsets_and_call`: Loads a code pointer relative to a0 into a0 and jumps to it, sets a4 and a3 from the current pc, adds 31352 to a4 and 33744 to a3, calls a nearby routine, clears a6, pushes a pc-relative pointer and branches away.
- `sign_extend_d0_push_and_call`: Sign-extends the low word of d0 into a long, pushes it on the stack, calls a nearby routine, then loads the absolute address 16170 into a0 and falls through.
- `jump_relative_to_a0_then_load_a0`: Loads a code pointer relative to a0 into a0 and jumps to it, then loads the absolute address 25406 into a0 and falls through to the next routine.
- `jump_via_pc_relative_a0_offset`: Adds the program counter to a0, loads the address seventy-eight bytes past that into a0, then jumps to it and loads a fixed immediate into a0.
- `jump_then_call_and_restore_stack`: Jumps through a0 after adding the program counter, calls a nearby routine, pops a longword into d0, reloads a7 from a fixed address and returns.
- `load_fixed_address_into_a2`: Loads a fixed immediate address into address register a2 and then falls through into the following routine.
- `set_a2_pc_relative_and_a3_constant`: Loads a2 with the address of the code 158 bytes past the current program counter plus a2, then loads a3 with the immediate value 30322.
- `open_libraries_from_name_list_2`: Walks a null-terminated list of library name strings at a2, calling exec OpenLibrary for each and storing returned bases through a3, returning zero on success or minus one on failure.
- `load_a2_with_constant_address`: Loads a2 with the immediate value 30266, a fixed pointer used by the following code.
- `close_library_list_from_a2`: Walks a null-terminated list of library pointers at a2, calling exec CloseLibrary for each non-negative entry, then returns zero in d0.
- `load_a0_with_constant_pointer`: Loads the immediate value 7632 into address register a0 and then falls through into the following routine.
- `load_a0_pc_relative_and_a1_constant`: Computes a program-counter-relative address into a0 and loads the immediate value 762c into a1, then continues into the next routine.
- `store_7fc00_to_two_pointers`: Writes the immediate value a fixed address to the longword at a0 and to the longword at a1 plus 270, then returns.
- `push_100_and_load_6f2c_into_a0`: Pushes the immediate value 256 onto the stack and loads the address 28460 into a0, then falls through to the next routine.
- `call_via_a0_then_store_d0_at_7d4`: Calls the routine whose address is a0 plus 298, discards four stack bytes, stores the returned d0 at the absolute address 2004, and returns.
- `load_a0_with_705e_pointer`: Loads the immediate value 705e into address register a0, then falls through into the following routine.
- `store_pc_relative_ptr_at_a4_offset_0`: Loads the address of the following code into a0, stores it at offset 0 of a4, then loads the immediate value 29398 into a0.
- `store_pc_relative_ptr_at_a4_offset_4`: Loads the address of the following code into a0, stores it at offset 4 of a4, then loads the immediate value 29408 into a0.
- `install_first_pcrel_vector_at_a4_offset_8`: Loads the address of the following code into a0 relative to its own position, stores it at offset 8 of a4, then loads the immediate value into a0.
- `store_pc_relative_ptr_at_a4_offset_12`: Loads the address of the following code into a0, stores it at offset 12 of a4, then loads the immediate value 29396 into a0.
- `store_pc_relative_ptr_at_a4_offset_16`: Loads the address of the following code into a0, stores it at offset 16 of a4, then loads the immediate value 29392 into a0.
- `store_pc_relative_ptr_at_a4_offset_20`: Loads the address twenty bytes past the current program counter into a0, stores it at offset 20 of a4, then loads the immediate value 29398 into a0.
- `store_pc_relative_ptr_at_a4_offset_24`: Loads the address twenty-four bytes past the current program counter into a0, stores it at offset 24 of a4, then loads the immediate value 29392 into a0.
- `store_pc_relative_ptr_at_a4_offset_28`: Loads the address twenty-eight bytes past the current program counter into a0, stores it at offset 28 of a4, then loads the immediate value 29386 into a0.
- `store_first_pcrel_handler_in_a4_slot`: Loads the address of the following code into a0, stores it at offset 32 of the a4 base, then loads the immediate value 29382 into a0.
- `store_second_pcrel_handler_in_a4_slot`: Loads the address of the following code into a0, stores it at offset 36 of the a4 base, then loads the immediate value 29388 into a0.
- `store_third_pcrel_handler_in_a4_slot`: Loads the address of the following code into a0, stores it at offset 40 of the a4 base, then loads the immediate value 29380 into a0.
- `install_callback_at_a4_offset_2c`: Loads the address of the following code into a0 relative to the program counter, stores it at offset 44 in the a4 structure, then loads the immediate value 29392 into a0.
- `install_callback_at_a4_offset_30`: Computes a program-counter-relative address into a0, writes it to offset 48 of the a4 structure, then sets a0 to the immediate value 29508.
- `install_callback_at_a4_offset_34`: Forms a program-counter-relative address in a0, saves it at offset 52 of the a4 structure, then loads the immediate value 29734 into a0.
- `store_204_handler_pointer_in_a4`: Loads the address of the code at offset 204 relative to a0 into a0, stores it at offset 56 of a4, then loads the immediate value 30426 into a0.
- `store_212_handler_pointer_in_a4`: Loads the address of the code at offset 212 relative to a0 into a0, stores it at offset 60 of a4, then loads the immediate value 29982 into a0.
- `store_220_handler_pointer_in_a4`: Loads the address of the code at offset 220 relative to a0 into a0, stores it at offset 64 of a4, then loads the immediate value 29724 into a0.
- `store_pc_relative_ptr_at_a4_offset_68`: Loads the address of the following code into a0, stores it at offset 68 of a4, then loads the immediate value 29774 into a0.
- `store_pc_relative_ptr_at_a4_offset_72`: Loads the address of the following code into a0, stores it at offset 72 of a4, then loads the immediate value 29904 into a0.
- `set_a0_to_pc_relative_24a_and_store`: Loads a0 with the address 24a bytes past the program counter plus a0, stores a0 at offset 4c of a4, then loads a0 with the immediate value 74c6.
- `install_first_pcrel_callback`: Computes the address of the following code relative to a0, stores it at offset 80 in the a4 structure, then loads the immediate value 29884 into a0.
- `install_second_pcrel_callback`: Computes the address of the following code relative to a0, stores it at offset 84 in the a4 structure, then loads the immediate value 29874 into a0.
- `install_third_pcrel_callback_and_return`: Computes the address of the following code relative to a0, stores it at offset 88 in the a4 structure, and returns to the caller.
- `load_custom_base_and_audio_ptr`: Loads the custom chip base address into a4 and the immediate value 29878 into a0, then falls through to the next routine.
- `compute_pc_relative_audio_table_pointer`: Computes a program-counter-relative address into a0 and loads the immediate value 29870 into a1, then continues into the following code.
- `install_audio_and_interrupt_servers`: Sets up custom chip interrupt bits, fills six interrupt server structures with handlers and data pointers, and calls AddIntServer and SetIntVector for audio and vertical blank interrupts.
- `disable_interrupts_and_dma_set_vectors`: Writes to the custom chip registers to disable all interrupts and DMA, then installs four interrupt vectors and two more vectors via exec library calls before returning.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `test_a1_and_enter_interrupt_handler`: Tests the word at a1 and branches away if zero; otherwise saves all registers on the stack and loads the exec library base into a6.
- `increment_counter_and_call_two_subs`: Sets a6 to a program-counter-relative address, increments the word at offset 230 of a6, calls two subroutines, then loads a0 with the immediate value 72dc.
- `check_byte_at_pc_offset_462`: Loads the address of the byte at offset 462 from the program counter into a0, tests that byte, and if it is zero loads the immediate value 12292 into a0.
- `call_pc_relative_470_then_set_a6`: Loads the address at offset 470 from the program counter into a0, calls the code there, then loads the immediate value 27938 into a6.
- `call_a6_offset_26a_if_present`: Sets a6 to the address at offset 1148 from the program counter, loads the longword at offset 618 from a6 into d0, and if nonzero calls it, then restores registers and returns zero.
- `load_a0_with_6fac_pointer`: Loads the immediate value 6fac into address register a0 and then falls through into the following routine.
- `set_a0_pc_relative_and_a1_6fb4`: Computes a program-counter-relative address into a0 using its current value and loads the immediate 6fb4 into a1.
- `set_a1_pc_relative_and_a2_6fa0`: Computes a program-counter-relative address into a1 using its current value and loads the immediate 6fa0 into a2.
- `set_a2_pcrel_and_a5_6f86`: Loads a2 with the address four hundred ninety-two bytes past the current program counter plus a2, then sets a5 to the immediate value 28550.
- `set_a5_pcrel_and_a3_6fd8`: Loads a5 with the address four hundred eighty-six bytes past the current program counter plus a5, then sets a3 to the immediate value 28632.
- `read_fire_button_and_joy1dat`: Loads a3 relative to the program counter, tests bit seven of the CIA port register, sets d7 to one if the button is released, reads joystick port one movement into d0, then branches ahead.
- `load_a0_with_6f96_pointer`: Loads the immediate value 6f96 into address register a0 and then falls through into the following code.
- `set_a0_pc_relative_and_a1_6f9e`: Computes a program-counter-relative address into a0 using its current value and loads the immediate 6f9e into a1.
- `set_a1_pc_relative_and_a2_6f8a`: Computes a program-counter-relative address into a1 using its current value and loads the immediate 6f8a into a2.
- `set_a2_to_pc_relative_table_and_a5_base`: Loads a2 with the address of a nearby table relative to the program counter plus a2, then sets a5 to the immediate value 28528.
- `read_joystick_and_update_input_state`: Reads CIA fire buttons and JOY0DAT mouse movement, merges them with stored input flags, updates direction and button state bytes in the a0, a1 and a2 structures, and advances five entries.
- `test_a1_word_then_setup_custom_regs`: Tests the word at a1 and branches away if zero, otherwise saves all registers, loads a4 with the custom chip base and a3 with the immediate value 6e6e.
- `poll_ciaa_keyboard_serial`: Reads the CIAA serial data register, inverts and rotates the byte, compares it with the previous key code, shifts history bytes, sets a change flag and calls the decode and dispatch routines.
- `set_a2_and_or_d1_into_flags`: Loads a2 relative to its own address, ORs d1 into the word at offset six of that block, then loads a2 with a fixed pointer.
- `set_a2_or_d2_into_flags_and_branch`: Loads a2 relative to its own address, ORs d2 into the word at offset six of that block, then branches to the shared continuation.
- `mask_d3_call_6b6_invert_d1_d2`: Clears the top bit of d3, calls the routine at offset 6b6, inverts all bits of d1 and d2, then loads the address 6da0 into a2.
- `mask_d1_into_a2_offset6_and_reload_a2`: Loads a2 from the program counter plus offset, ANDs d1 into the word at offset 6 of a2, then loads a2 with an immediate address.
- `mask_d2_into_a2_offset6_then_branch`: Loads a2 from the program counter plus offset, ANDs d2 into the word at offset 6 of a2, then branches to another routine.
- `decode_joystick_bits_to_d1_d2`: Compares byte in d3 against several joystick code values and sets individual bits in d1 and d2 accordingly, returning both masks.
- `restore_all_registers_and_return_zero`: Pops d0-d7 and a0-a6 from the stack, sets d0 to zero as the return value, and returns to the caller.
- `range_check_d0_between_0_and_69`: Compares d0 against 0 and 69, branching away if outside, else zeroes d1, copies d0 into d1, sign-extends it, and loads a0 with an immediate address.
- `store_table_byte_to_a3_offset_two`: Adds the signed word in d1 to the base address in a0, loads a byte from that table and stores it at offset two from a3, then returns.
- `write_ff_to_a3_offset_two`: Writes the immediate byte value 255 to offset two from the address in a3 and returns to the caller.
- `test_a1_then_setup_audio_registers`: Tests the word at a1 and returns if zero; otherwise saves registers, loads the custom chip base into a4, a5 and a3 and branches onward.
- `build_joystick_bitmask_from_port_bits`: Reads joystick port bits from offset 30 of a4, stores them at offsets 156 and 154, then builds a bitmask in d2 by testing four direction bits and updating counters at a3.
- `allocate_and_initialize_game_resources`: Allocates memory blocks, opens files, sets up display and sound hardware registers, and initializes game state structures through many library calls, returning minus one on failure.
- `call_two_library_functions_and_return`: Loads two library function addresses into a0 and calls each via jsr, then returns to the caller.
- `scan_until_nonzero_result`: Clears the word at offset 560 of the structure pointed to by a4, then repeatedly calls a helper while the counter is below the passed limit and the helper returns zero, returning the first nonzero result or zero.
- `consume_flag_and_check_escape`: Reads the structure at offset 40 of a4, returns zero if its word at offset 4 is already clear, otherwise clears that word and returns minus one when the byte at offset 2 equals the escape code.
- `parse_two_optional_fields`: Calls a parser on the buffer and length arguments, stores the first field pointer and size in the structure, and if a flag byte is set parses a second field into the following slots, returning the total consumed length or minus one.
- `play_sound_sequence_with_retries`: Pushes arguments, calls four helper routines via pc-relative jumps to play a sound sequence, decrements a counter and returns zero on success or minus one on failure.
- `swap_two_byte_buffers`: Swaps the first d7 bytes between the buffer at a5 and the buffer at a2, using d5 as a temporary byte and d6 as the index.
- `restore_registers_and_return`: Pops d5 through d7 and a2/a5 from the stack, releases the a6 frame and returns to the caller.
- `parse_and_link_sprite_frame_offsets`: Calls a size parser on the frame data, aligns the size, stores the frame base and per-frame offsets into the output structure, and returns zero or minus one.
- `render_text_with_font_glyph_lookup`: Walks a null-terminated string, converts digits, letters and special glyph codes, looks up each glyph in the font table and calls the glyph drawing routine with the current pen position.
- `center_and_draw_text_block`: Measures the text width via a helper, computes a centered horizontal position from the screen width, then calls the text drawing routine with the adjusted coordinates.
- `parse_encoded_string_to_score`: Walks the null-terminated string at a5, converting digits, letters and table-matched symbols into values, adding each entry's byte at offset 21 plus d7 to d3, and returns the total in d0.
- `draw_tile_grid_rows`: Loops over rows and columns, reads tile indices from the array at a1, looks up two bytes per tile in the table at a0, and calls the draw routine with a5, a2 and the tile bytes.
- `increment_column_counter_and_loop`: Adds one to the column counter d5 and branches back into the enclosing tile-drawing loop to continue with the next column.
- `restore_registers_unlink_and_return`: Pops registers d4 through d7 and a2 and a5 from the stack, releases the a6 frame pointer with unlk, then returns to the caller.
- `reset_four_entry_table_and_call`: Loops over four eight-byte entries in the table at offset 56 of a4, setting first word to minus one and next two words to zero, then calls a routine via a0.
- `save_registers_and_load_a4_base`: Pushes registers d2 through d7 and a2 through a6 onto the stack, then loads a4 with the absolute base address for the following code.
- `load_bmhd_cmap_body_chunks`: Stores three stack arguments into a4-relative fields, calls a loader with them, saves the returned handle, and on success runs two further setup routines before returning minus one.
- `find_and_parse_bmhd_cmap_body`: Looks up the BMHD, CMAP and BODY chunk identifiers via a search routine, stores the found pointers in a4 fields, then reads width, height and bit depth from the BMHD header.
- `scan_chunk_list_for_id`: Walks the chunk pointer list at a4 offset 26 for a count from a4 offset 34, comparing each longword against d0 and leaving a0 past the match or at the end.
- `nested_block_copy_loop`: Loops over rows and columns, loading a5 from the table at a6 offset 562 plus d3, calling the copy routine, advancing d5 by four and d3 by the word at 536(a6).
- `check_count_and_flag_setup`: Reads the word at 6(a4) into d4, returns if it is zero or negative, else sets d0 to zero or to d4 minus one depending on the byte at 8(a4).
- `run_length_byte_copy`: Reads a byte from (a0) into d0, if non-negative copies the next byte to (a5), decrements d4 and loops d0 times, otherwise branches back to the caller's loop.
- `copy_bytes_backwards_to_a5`: Negates d0 to count down, copies bytes from (a0) to (a5) incrementing both pointers, decrements d4, loops via dbra on d0, then branches back to the caller's earlier code.
- `unpack_three_nibbles_to_words`: Reads a count from the pointer at offset 14 of a4, divides by three, then loops packing three high nibbles from consecutive bytes into one word stored at a2.
- `program_four_audio_channels`: Loops over four audio channel entries, programs AUD0LCH and related custom registers from sample tables, sets DMACON and INTENA/INTREQ bits, and calls a helper twice.
- `set_d7_scanline_retry_count`: Loads the immediate value 20 into data register d7, setting the retry count used by the following vertical position polling code.
- `wait_for_stable_vposr_reading`: Saves d4-d7/a5, reads the VPOSR register at custom chip base twice masked with a fixed address, loops via dbra d7 until two consecutive reads match, then restores registers and returns.
- `write_audio_period_from_stack_args`: Reads two words from the caller's stack, scales the first by 16, adds it to the audio base at a fixed address, and stores the second word into the period register at offset 8.
- `load_vhdr_sound_bank_entries`: Parses FORM/VHDR/BODY chunks from a sound bank, fills sixteen-byte entries with sample pointers, rates and lengths, and returns the total bytes consumed or zero on failure.
- `shift_body_data_forward_by_width`: Moves the body byte block forward by twice the stored width using a backward byte copy, updates a0 and a3, and restores a2 and a4.
- `decode_body_nibbles_with_checksum_table`: Reads nibble pairs from the body pointer, accumulates table values into a checksum, then writes decoded bytes backward while updating the stored length and pointer.
- `scan_word_array_for_d0`: Sets d7 to 32767, then compares each word at (a0) with d0 while advancing a0 by two bytes, stopping on a match or after the count expires, then skips one more word and returns.
- `init_audio_and_clear_channel_state`: Saves registers, clears the word at the audio state pointer, calls the sound setup routine, sets the CIAA port A bit 1, marks a flag byte, then clears the word at offset six of four eight-byte channel records.
- `shutdown_audio_and_clear_flag`: Saves registers, clears the byte at the sound flag pointer, clears CIAA port A bit 1, calls the sound teardown routine, clears the word at the audio state pointer and restores registers.
- `load_fourteen_raw_sample_files`: Loads fourteen RAW sample files into memory using a base name plus per-entry suffixes, then sets the CIAA port A bit to enable sound output and returns zero or minus one.
- `store_sample_pointer_and_start_audio`: Saves the caller's argument into the global sample pointer at the table base, calls the audio start routine, then sets bit 1 of CIAA_PRA before restoring registers and returning.
- `set_audio_and_custom_chip_pointers`: Loads the address of the sample table into a0, the custom chip base into a1, the audio register block into a2 and a fixed value into a3, then falls through to the next routine.
- `tick_four_channel_sound_slots`: Decrements four channel countdowns, programs finished channels with a sample pointer and volume, clears their bits in the active mask, advances a counter and calls the sound update.
- `copy_selected_unit_state_to_globals`: Loads a pointer from a global, copies byte at offset 470 into a global byte, copies word at offset 4 into a global word, then calls two routines and returns zero in d0.
- `clear_fifteen_global_pointers`: Loads a table of fifteen longword pointers and clears the longword pointed to by each entry, then returns.
- `reset_audio_volumes_and_state`: Writes all ones to a state word, clears the four audio volume registers at offsets 168, 184, 200 and 216 from the custom chip base, and zeroes three state words before returning.
- `silence_audio_and_enable_dma`: Clears a state word and the four audio volume registers, then writes fifteen to DMACON to enable the audio DMA channels, and returns.
- `advance_audio_sample_pointer`: Reads a sample index and offset, programs the four audio channels' pointers and periods, updates DMACON, then advances the sample offset and index with wraparound before returning.
- `apply_sprite_frame_and_dma`: Reads a four-byte sprite record from a0 indexed by d1, looks up frame data in a table, updates the sprite structure at a6, writes DMACON to enable DMA, and calls the blitter setup routine.
- `load_and_play_animation_file`: Parses a colon-separated filename, opens the file, allocates buffers, runs three setup checks, then loops calling the frame routine until playback finishes and returns a status.
- `free_animation_buffer_and_clear`: If the pointer at offset minus eight from a6 is nonzero, pushes it and calls the free routine, then clears that pointer and returns zero in d0.
- `read_chunk_verify_remaining`: Checks flag at offset -28 of a6, reads 4096 bytes from a6-relative buffer into a0+40, subtracts 4096 from counter at -12(a6), compares read count and returns 0 or -1 in d0 with d1 set.
- `check_tpwm_magic_flag`: If byte at -30(a6) is set, compares longword at offset 40 of the a6-relative structure with the TPWM signature and stores the equality result as a byte at -31(a6).
- `clear_flag_and_return_zero`: Clears the byte at offset -29 of a6 and returns zero in d0 to the caller.
- `open_stream_and_set_flags`: Pushes the stream pointer at offset 12 of a6 and a zero word, calls the open routine, stores the returned handle at -8(a6), and sets flags at -30, -31 and -28 when it is non-zero, returning zero on success or minus one on failure.
- `decode_bitstream_into_buffer`: Checks flags at -31 and -30 of a6, then either jumps through a saved vector or decodes a packed bitstream from the buffer at 8(a6) into the destination at -4(a6), updating counters at -24 and -16 and setting the flag at -28.
- `bitstream_byte_fetch_step`: Reads one byte from (a0) into d0, increments d6, sets d5 to seven, doubles d0 and branches on carry, decrementing the counter at -16(a6) and comparing d6 with d3.
- `copy_bytes_advance_and_check_limit`: Copies bytes from the address in a0 to a1, increments d6 per byte, loops via dbra d5, then branches back and compares d6 with d3, branching if greater or equal.
- `read_byte_scale_by_sixteen_check_limit`: Reads a byte from (a0) into d1, increments d6, zeroes d2, copies d1 into d2, shifts d2 left four bits, then compares d6 with d3 and branches if greater or equal.
- `rle_decode_dispatch_next_decoder`: Decodes run-length data from a0 into a1 using d1/d2 counts and a6 stack counter, then calls a routine and stores a decoder address into a3 before branching.
- `save_registers_to_context_block`: Clears a flag byte at offset -30 from a6, loads the context pointer from -4(a6), stores d0-d2, d5-d7, a0-a2 into offsets 4 through 36 of that block, then returns.
- `swap_d0_d1_if_nonzero_result`: Pops d0, d1 and a0 from the stack, pushes them back as arguments to a subroutine, discards the arguments, and if the returned d0 is nonzero exchanges d0 with d1 before returning.
- `open_and_configure_two_file_handles`: Sets up a stack frame, calls several subroutines to open two file handles with modes 1 and 0, invokes a library routine at offset -198 with delays of 100 and 200, then cleans up and returns zero.
- `return_minus_one_after_bsr`: Loads minus one into d0, calls an unnamed routine, then restores d2-d7/a2-a5, releases the a6 frame and returns.
- `open_library_and_query_interface`: Opens a library with version 260, obtains an interface, queries and releases it, reads offset 124 from the base, closes the library and returns that value or minus one.
- `call_library_offset_minus_42`: Saves registers, loads three longwords from the caller's argument block into d1-d3, calls library offset minus 42, restores registers and returns.
- `compare_library_result_to_third_arg`: Calls the library vector at offset minus forty-eight with the three arguments from the stack frame, compares the returned value in d0 against the third argument, and returns zero on match or minus one otherwise.
- `return_minus_one_restore_registers`: Sets d0 to minus one, restores d2 through d7 and a2 through a5 from the stack, releases the a6 frame and returns to the caller.
- `call_library_vector_minus_thirty_six`: Loads the first stack argument into d1, calls the library vector at offset minus thirty-six through a6, then restores registers and returns.
- `call_library_offset_minus_30_with_arg`: Saves registers, adds 1005 to the word argument at offset 8 of the frame, loads the long at offset 10 into d1, then calls the library vector at offset minus 30 with a6 pushed and returns.
- `call_library_offset_minus_132_no_args`: Saves registers, pushes a6, loads the library base into a6 and calls the function at offset -132, then restores a6 and all saved registers before returning.
- `shutdown_restore_interrupts_and_library`: Saves all registers, calls a subroutine, then invokes library functions at offsets -228, -456, -132 and -228 again, writes 32767 to INTENA, sets d0 to -1 and loads a0 with a fixed address.
- `store_d0_to_pc_relative_word_then_set_a0`: Loads a pc-relative address into a0, writes the low word of d0 there, then loads the fixed value minus 14066 into a0.
- `store_d0_to_next_word_then_load_hw_base`: Computes a pc-relative address into a0, writes the low word of d0 to that memory, then sets a0 to a fixed hardware register address.
- `store_d0_to_pc_relative_word`: Stores the low word of d0 into the word at the pc-relative address, then loads a fixed hardware address into a0.
- `store_d0_to_pc_relative_slot_and_set_a0`: Writes the low word of d0 into the word at the pc-relative address, then loads a0 with the fixed hardware address near the top of memory.
- `store_d0_to_second_pc_relative_slot`: Writes the low word of d0 into a second pc-relative word slot, then loads a0 with a different fixed hardware address near the top of memory.
- `store_d0_and_configure_cia_keyboard`: Writes d0 to a pc-relative slot, copies a word into offset 154 of a4, writes 33792 to offset 150, clears the CIA keyboard serial register, restores all registers and returns.
- `stop_custom_chips_and_call_os`: Saves all registers, silences the custom chip audio and interrupt registers, clears the keyboard serial register, waits for the copper/blitter busy bit, then calls two OS library vectors through a6.
- `disk_error_prompt_and_retry_loop`: Opens a requester, draws a bordered box, prints disk error messages, waits for a key, then retries the disk operation until it succeeds or the user aborts.
- `abort_disk_retry_and_return_minus_one`: Closes the requester, clears a status byte, optionally redraws and frees the message, then restores registers and returns minus one in d0.
- `swap_palette_entries_with_shadow`: Copies 32 words from the shadow palette at a0 into the hardware palette entries at offset 2 of each 4-byte slot, while saving the old hardware values into the buffer at a1.
- `restore_palette_from_saved_buffer`: Writes 32 saved words from the buffer at a0 back into the hardware palette entries at offset 2 of each 4-byte slot, restoring the previously stored colors.
- `find_trackdisk_device_node_2`: Saves all registers, calls exec FindName for the trackdisk.device node, tests the returned pointer in d0, then restores registers and returns.
- `check_trackdisk_tasks_and_signal`: Opens trackdisk.device via exec.FindName, scans six task entries for a non-null task whose byte at offset 65 has bit 7 clear, and signals the found task through library call -198.
- `init_audio_and_clear_globals`: Calls two setup routines then branches to another, clears several word globals to -1, writes the interrupt enable register and zeroes two global longs.
- `copy_string_until_null`: Copies bytes from the address in a0 to the address in a1, incrementing both pointers, stopping when a zero byte is read and returning.
- `rle_decompress_to_buffer`: Expands a run-length encoded byte stream from a0 into the buffer at a1 for d0 bytes, copying literals or back-referenced runs, then restores d1-d3/a2-a3 and returns.
- `reset_flag_and_clear_pointer`: If the flag byte at a5 is set, clears the longword at offset 618 of a6, clears the flag, calls a subroutine, then restores a5-a6 and returns.
- `setup_display_window_from_state`: When the word at offset 638 of a6 is nonzero, calls a subroutine, then writes zeroed words, a 256 value, a longword and a combined bitfield word into a4 offsets.
- `setup_sprite_from_tile_coords`: Saves a4/a6, checks a global flag, calls a helper, then writes zero and 256 to a4 offsets 66 and 64, computes a tile index from two globals into d0, stores doubled half-width at offset 102, and sets a4 offset 84 to a base pointer plus zero and offset 88 to d0.
- `copy_tileset_block_to_buffer`: Saves registers, checks a global flag, calls a helper, sets a4 fields including 100, 66, 68, 64, 80, 84 and 88, then copies five blocks of twelve longwords from source to destination pointers advanced by 240 bytes each, looping five times.
- `setup_fullscreen_tile_blit`: Saves a4-a6, calls a helper twice, computes a tile index from two globals into d0, stores doubled half-width at a4 offsets 100 and 102, sets a4 offsets 80, 84, 66, 68, 64, 150 and 88, and toggles offset 150 between 1024 and 33792.
- `blit_clear_and_copy_playfield_buffers`: Waits for the blitter to finish, programs blitter control, modulo and size registers, then copies 14 blocks of 48 bytes from one buffer to another and optionally calls a cleanup routine.
- `store_word_to_object_field`: Loads the global base pointer, fetches the object pointer at offset 510, and stores the word argument from the stack into that object's field at offset 86.
- `flush_palette_and_clear_flag`: If the flag at offset 640 is set, calls a helper, clears the flag, then writes 8 color entries from the stored long value into the copper list at offset 216.
- `decode_and_apply_script_commands`: Walks a byte script, copying runs of bytes into a destination computed from two word arguments, and handles control bytes for jumps, calls, returns and nested script execution.
- `store_script_pointer_global`: Loads the address of a global pointer slot into a0 and stores the caller's first stack argument there, then returns.
- `set_bitmap_plane_offsets_and_mask`: Stores two word arguments into globals, computes a plane offset from the second argument and a nibble mask from the first, then calls two setup routines.
- `queue_two_words_into_ring_buffer`: Pushes two longword arguments from the stack into the next free slot of a four-entry ring buffer at offset 1324 from the base, incrementing the write index, unless the buffer is disabled or full.
- `flush_ring_buffer_to_blitter_list`: Walks the queued ring-buffer entries, converts each pair of coordinates into blitter control words with modulo and flag bits, writes them into the copper/blitter list at offset 216 of the structure, then clears the queue count.
- `xor_rect_bits_across_planes`: Computes a bit offset from the rectangle coordinates, then sets, clears or toggles one bit per row across the plane pointers, stepping by the row stride for each of the configured planes.
- `draw_line_into_blitter_planes`: Computes a Bresenham line between two points, looks up pixel masks, and programs the blitter for each of several bitplanes, storing blitter pointers and control words into the a4 structure.
- `compare_two_points_and_redraw`: Saves registers, loads four words from the global table at a6, and if the two point pairs match it calls the line redraw routine before restoring registers and returning.
- `blit_line_across_bitplanes`: Computes line deltas and octant from d0-d3, builds blitter control words, then loops over the plane count programming blitter pointers, modulos, data and size registers at the custom chip base.
- `init_sprite_channel_from_script`: Reads a pointer from (a0)+, offsets it by d4, calls the copper wait helper, then fills fields at offsets 64-116 of the a4 structure with sprite pointers, dimensions, control words and saved registers, looping d7 times.
- `wait_for_copper_blitter_idle`: Loads the custom chip base into a4 and spins while bit 14 of DMACONR at offset 2 is set, returning once the blitter/copper busy flag clears.
- `pack_bitplane_bits_into_word`: Uses the table at a6 to locate bitplane pointers and a bit offset, then loops d3 times shifting each fetched byte by d2 and accumulating carry bits into d7, storing the packed word at offset 0 of a6.
- `toggle_planar_pixels_by_bitmask`: Reads a bit index and row count from a table, then sets, clears or toggles one bit per plane pointer across d7 rows using d0 and d1.
- `check_point_within_bounds`: Compares d0 and d1 against four boundary words at offsets 18, 22, 20 and 24 of a6, setting d2 to minus one and returning early when outside the rectangle.
- `clamp_point_to_bounds`: Clamps d0 and d1 into the rectangle stored at a6 offsets 18 and 22, sets d2 to minus one if d2 falls outside offsets 20 and 24, and returns.
- `draw_tile_span_into_bitmap`: Reads tile coordinates from the map structure, computes bitmap offsets and masks, then writes tile pointers, colors and span sizes into the display structure for each of several rows.
- `normalize_map_bounds_and_redraw`: Loads map bounds from the map structure, swaps them if reversed, adjusts the visible row range, then calls the redraw routine twice to refresh the map.
- `initialize_map_view_from_parameters`: Stores caller-supplied map parameters into the map structure, computes rotation and row counts, loads optional bitmap pointers, then calls the setup and draw routines and returns zero.
- `init_velocity_tables_and_clear_state`: Stores four word parameters into the a5 state block, rotates a word into offset 546, clears the long at 642, calls three subroutines, then fills two arrays at offsets 562 and 646 with incrementing long values from the step at 538.
- `copy_rom_table_to_ram_buffer`: Loads the source address and a count of 348 words, calls the copy routine, then stores the returned destination pointer into the a5 block at offset 510 and returns.
- `copy_words_to_high_memory_or_allocate`: If a0 is below the an address boundary it returns a0 in d0; otherwise it calls the allocator, copies d7 words from a0 to the new block, halves and decrements the count, and jumps into the copy loop.
- `set_copper_list_and_dma_control`: Loads a base pointer, writes a copper list address to COP1LCH, enables DMA channels with value 33728 in DMACON, stores a0 into COPJMP1 to start the copper, then returns.
- `call_sub_and_jump_to_other`: Calls the routine at offset 27990 with bsr, then unconditionally branches to the code at 25458, transferring control without returning here.
- `build_incrementing_word_table`: Reads a count and step from the parameter block, then fills consecutive words starting at a fixed table with values increasing by the step, decrementing the count until done, and returns.
- `compute_bitmap_row_geometry`: Loads the graphics context base, reads width and height words, stores width shifted right by four, its double, and two multiplied byte sizes into context fields, then stores height minus one.
- `allocate_and_fill_bitmap_row_offsets`: Allocates a buffer sized from the context, fills row offset tables with increasing strides, and writes sprite dimension words into the display structure.
- `set_copper_list_and_enable_dma`: Loads the copper list pointer from the graphics context into COP1LCH, clears COPJMP1, and writes the DMA control register to enable DMA channels.
- `check_vposr_beam_and_set_flag`: Clears a byte and word in a data area, compares the low word of VPOSR masked with a constant against a value, and sets the byte flag if they match.
- `call_decompress_with_two_args`: Saves registers, loads two arguments from the stack frame into a5 and a6, calls the decompression routine, restores a6, returns zero in d0 and restores registers.
- `decompress_bitstream_to_buffer`: Reads a control byte and length from a6, expands bits into a5 output buffer, copying literals or back-references until the end address a4 is reached.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `rle_decompress_to_a5_buffer`: Reads run-length encoded bytes from (a6), copying runs into the buffer at a5 until a4 is reached, using d1 as count and d2 as back-offset, then restores registers and returns.
- `clip_and_setup_sprite_blit`: Clips a sprite rectangle against screen bounds in a6, computes source and destination offsets, sets blitter registers in a3 and returns with d0 as a flag.
- `queue_sprite_blit_variants`: Selects one of several sprite descriptor tables based on a stack argument, fills blitter register blocks at a4 from a5 and a6 fields, and loops over d6 entries.
- `enqueue_sprite_block_from_rect`: Clamps d0-d3 to the a6 rectangle limits, computes tile offsets and size, then writes a sprite block into the a4 structure or the a6 free list and returns.
- `drain_free_list_callback_loop`: Sets up a6 from a pc-relative base, then repeatedly pops entries from the a6 free list and calls a routine on each until the list head reaches the tail.
- `reset_free_list_head_to_tail`: Copies the a6 tail pointer into the a6 free list head, restores a6 from the stack and returns to the caller.
- `setup_sprite_descriptor_loop`: Saves registers, loads a data base pointer, writes -1 and zero and a sprite control value into the a4 structure, then loops over a descriptor copying pointer, offset and count fields into a4.
- `render_text_string_with_control_codes`: Saves registers, stores a mode word and optional pointer, then walks the a3 string byte by byte, handling space, pipe and carriage return codes and calling a glyph routine while accumulating d0.
- `advance_text_cursor_two_pixels`: Adds two to the d0 text cursor value and branches back into the string rendering loop.
- `sum_stack_args_into_d0_d1`: Loads the word at offset 48 from the stack into d0 and adds the word at offset 54 to d1, then branches to another routine.
- `store_a2_to_a6_offset_and_restore`: Stores a2 into the longword at offset 518 from a6, restores d2-d7 and a2-a6 from the stack, and returns.
- `accumulate_string_char_values`: Sets up a stack frame, saves registers, then while a global flag is set reads bytes from a3 and adds each computed value plus one to d0 until a zero byte.
- `blit_tile_row_with_mask_and_modulo`: Reads a tile offset from the tile table indexed by d7, sets up blitter pointers, modulo, size and control from a6 fields, then blits each row using d0 bit to pick the mask and returns the shifted size in d5.
- `copy_32_words_to_copper_and_shadow`: Copies 32 words from the caller's buffer into every fourth word of the copper list at offset 88 and also into the shadow buffer, then restores registers and returns.
- `copy_32_words_to_copper_list_only`: Copies 32 words from the caller's buffer into every fourth word of the copper list at offset 88, without updating the shadow buffer, then restores registers and returns.
- `decrement_palette_components_toward_zero`: Copies 32 words from the source table to the working palette, decrementing each red, green and blue nibble toward zero, then calls the palette update and repeats until no component changed.
- `copy_32_words_to_palette_buffer`: Copies 32 words from the caller-supplied pointer into the working palette buffer at a fixed offset, then calls the palette refresh routine and restores registers.
- `increment_color_components_until_match`: Clears 32 words, then bumps each color's red, green and blue nibbles toward a target table, redrawing and repeating until no change.
- `blit_span_interpolate_rows`: Saves registers, reads two source pointers and four words from the stack, then for each row calls the nibble interpolation routine three times and stores the packed result into the copper list at offset 88 plus four times the start index.
- `interpolate_nibble_pair`: Extracts two nibbles from d0 and d1 at bit position d5, scales their difference by d3 divided by d2, adds the low nibble back, shifts into place and ORs the result into d4.
- `blit_fixed_span_interpolate`: Saves registers, takes a source pointer and a word from the stack, then for 33 rows calls the nibble interpolation routine three times and writes the packed result into the copper list at offset 88.
- `blend_shifted_masked_word_into_d4`: Shifts d0 right by d5, masks with d3, multiplies by d2, shifts right eight, shifts left by d5, then ORs the result into d4 and returns.
- `store_word_into_indexed_table_entry`: Saves registers, loads a table base via pc-relative lea, reads two words from the caller stack, and stores the second word at an offset indexed by the first shifted left twice.
- `init_state_and_dispatch_updates`: Saves registers, loads a base pointer, and if the word at offset 530 equals minus one calls three subroutines, then calls a fourth when the word at offset 640 equals one.
- `enable_flag_and_run_two_updates`: Saves registers, points a6 at the base structure, and if the byte at offset 531 is zero sets it to all ones then calls two update routines before restoring registers.
- `disable_flag_and_run_two_updates`: Saves registers, points a6 at the base structure, and if the byte at offset 531 is nonzero clears it then calls two update routines before restoring registers.
- `check_vposr_against_beam_position`: Computes a target beam value from words at offsets 550 and 524, shifts it left eight bits, masks the VPOSR register with that range and compares, returning after restoring a6.
- `wait_for_vposr_line_match`: Saves a6, loads the base pointer, reads a target line value from offset 550, shifts it left eight bits, then spins comparing the VPOSR hardware register masked to the line bits until it matches, then restores a6 and returns.
- `copy_vertex_list_with_offset`: Loads the base pointer, reads a count and destination pointer, then for each entry reads a long from the source table, adds the offset at 642, and writes the high and low words to the destination with an eight-byte stride; optionally copies one extra entry.
- `swap_vertex_buffers_and_coords`: Swaps each long between the source table and the destination buffer for the counted entries plus an optional extra, then if a flag byte is clear exchanges the coordinate pairs at offsets 594 through 606 and returns.
- `init_display_and_audio_state`: Saves a4/a6, loads the global base into a6, calls a setup routine, computes a word from fields at offsets 524, 526 and 536, stores zero and 256 into a4 offsets 64, 66 and 102, copies a long from a6 offset 562 to a4 offset 84, stores the computed word at a4 offset 88, calls setup again and conditionally calls another routine when a6 offset 638 is nonzero.
- `open_library_and_queue_request`: Opens a library via jsr through a6 with d1 set to 65539, and if the result is nonzero increments a counter, computes an eight-byte slot and stores the caller's argument and the library base there.
- `tail_call_library_open_with_65537`: Sets up an a6 frame, saves d2-d7/a2-a5, loads the caller's argument into d0 and 65537 into d1, then branches into another routine to continue the library open.
- `flush_pending_cleanup_list`: Reads a count from a global table, clears it, then for each entry calls the library function at offset -210 with the entry pointer and a saved frame pointer.
- `block_move_with_alignment`: Copies a byte count from source to destination, aligning pointers and using longword and register-block moves for large sizes, returning nothing.

### Historyline 1914-1918 (1992)(Blue Byte)(De)(Disk 1 of 7)[a].adf:intro.dwn#unpacked (raw code file, load address unknown)

- `load_intro_program_data`: Allocates memory for the intro loader data and track buffers, reads disk blocks through device calls, configures drive and display settings, and frees the buffers on each error path, returning minus one on failure.
- `load_intro_program_entry`: Second entry into the intro loader that calls through a computed address, then runs the same allocation, disk read and cleanup sequence and returns minus one on failure.
- `load_intro_sequence_buffers`: Calls a computed-address routine, allocates the intro work buffers and reads the sequence data through device calls, then releases the buffers on each failure path and returns minus one on error.
- `cleanup_and_return_status_word`: Calls two subroutines and a final cleanup routine, then returns the low word of the input value unchanged.
- `run_stage_and_return_argument`: Calls a computed-address routine, runs two subroutine calls and returns the first stacked argument unchanged.
- `invoke_stage_and_return_argument`: Calls a computed-address routine, runs a cleanup call and returns the first stacked argument unchanged.
- `call_stage_entry_and_return`: Calls a computed-address routine and a subroutine, then returns the stacked argument.
- `populate_data_array`: Reads a sequence of bytes from a memory buffer and stores a value returned by another routine into a global array, continuing until a null terminator or a specific sentinel value is encountered.
- `scan_string_for_terminator`: Scans a string or memory area for a certain byte, calling a function and updating a global array if the byte is not found.
- `open_libraries_from_name_list`: Walks a name list, opens each library through the OpenLibrary call (offset minus 552) into consecutive slots, and returns minus one if any open fails.
- `iterate_and_call_function`: Iterates through a global array and calls a function for every non-zero element found, stopping if a negative value is encountered.
- `close_relative_library_handles`: Walks a table of stored library handles located relative to the code, calling the CloseLibrary routine (offset minus 414) for each non-zero entry until a negative marker.
- `set_two_default_words`: Stores the same fixed constant into two fixed global locations.
- `store_constant_two_locations`: Stores a fixed constant into a location given by a relative offset and into one fixed global location.
- `store_constant_at_two_pointers`: Writes a fixed constant through a pointer argument and into a location relative to the code.
- `store_result_of_256_call`: Calls a subroutine with the value 256 and stores its result in a global.
- `call_computed_entry_and_store`: Calls a routine at a computed address and stores its return value in a global.
- `fill_intro_handler_table`: Fills a 23-entry table at the destination pointer with addresses of intro routines and unused global slots.
- `fill_intro_handler_table_relative`: Fills a 23-entry table at the destination pointer, with the first slot computed relative to the code and the others set to intro routine addresses and unused global slots.
- `fill_intro_handler_table_offsets`: Fills a table of 22 longword slots with routine addresses and unused global slots, storing the first slot relative to the code.
- `fill_intro_table_slot_two`: Fills the intro handler table from its second slot onward with routine addresses and unused global slots, with the first written slot computed relative to the code.
- `fill_intro_table_slot_three`: Fills the intro handler table from its third slot onward with routine addresses and unused global slots, with the first written slot computed relative to the code.
- `fill_intro_table_slot_four`: Fills the intro handler table from its fourth slot onward with routine addresses and unused global slots, with the first written slot computed relative to the code.
- `fill_intro_table_slot_five`: Fills the intro handler table from its fifth slot onward with routine addresses and unused global slots, with the first written slot computed relative to the code.
- `fill_intro_table_slot_six`: Stores a routine address at slot six of the intro handler table, computed relative to the code, and fills the following slots with intro routine addresses and unused global slots.
- `fill_intro_table_slot_seven`: Stores a routine address at slot seven of the intro handler table, computed relative to the code, and fills the following slots with intro routine addresses and unused global slots.
- `fill_intro_table_slot_eight`: Stores a routine address at slot eight of the intro handler table, computed relative to the code, and fills the following slots with intro routine addresses and unused global slots.
- `fill_intro_table_slot_nine`: Stores a routine address at slot nine of the intro handler table, computed relative to the code, and fills the following slots with intro routine addresses and unused global slots.
- `fill_intro_table_slot_ten`: Stores a routine address at slot ten of the intro handler table, computed relative to the code, and fills the following slots with intro routine addresses and unused global slots.
- `fill_intro_table_slot_eleven`: Stores a routine address at slot eleven of the intro handler table, computed relative to the code, and fills the following slots with intro routine addresses and unused global slots.
- `fill_intro_table_slot_twelve`: Stores a routine address at slot twelve of the intro handler table, computed relative to the code, and fills the following slots with intro routine addresses and unused global slots.
- `fill_intro_table_slot_thirteen`: Stores a routine address at slot thirteen of the intro handler table, computed relative to the code, and fills the following slots with intro routine addresses and unused global slots.
- `fill_intro_table_slot_fourteen`: Stores a routine address at slot fourteen of the intro handler table, computed relative to the code, and fills the following slots with intro routine addresses and unused global slots.
- `fill_intro_table_slot_fifteen`: Stores a routine address at slot fifteen of the intro handler table, computed relative to the code, and fills the following slots with intro routine addresses and unused global slots.
- `fill_intro_table_slot_sixteen`: Stores a routine address at slot sixteen of the intro handler table, computed relative to the code, and fills the following slots with intro routine addresses and unused global slots.
- `fill_intro_table_slot_seventeen`: Stores a routine address at slot seventeen of the intro handler table, computed relative to the code, and fills the following slots with intro routine addresses and unused global slots.
- `fill_intro_table_slot_eighteen`: Stores a routine address at slot eighteen of the intro handler table, computed relative to the code, and fills the following slots with unused global slots and the table's final constant.
- `fill_intro_table_slot_nineteen`: Stores a routine address at slot nineteen of the intro handler table, computed relative to the code, and fills the following slots with unused global slots and the table's final constant.
- `fill_intro_table_slot_twenty`: Stores a routine address at slot twenty of the intro handler table, computed relative to the code, and fills the following slots with unused global slots and the table's final constant.
- `fill_intro_table_slot_twentyone`: Stores a routine address at slot twenty-one of the intro handler table, computed relative to the code, and fills the last slot with the table's final constant.
- `store_routine_in_last_slot`: Stores a routine address, computed relative to the code, into the last slot of the intro handler table.
- `initialize_hardware_and_functions`: Sets multiple custom hardware registers and certain memory locations, and calls several function pointers or copies or pointers to memory.
- `install_interrupts_from_offset`: Sets the enable bit in the interrupt control register value taken from the structure, then registers the same audio and vertical blank interrupt servers as the setup routine.
- `register_audio_interrupt_servers`: Stores the interrupt enable value from the structure, sets its enable bit, and registers the audio and vertical blank interrupt servers and vectors through exec calls.
- `shutdown_interrupts_and_audio_dma`: Clears all interrupt enable bits and pending requests, turns off the four audio DMA channels, and calls the exec removal routines for the interrupt servers.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `run_pending_callbacks_if_set`: When the value at the input pointer is non-zero, increments a counter, runs two subroutines, then calls a flag-guarded routine and a stored callback pointer.
- `bump_counter_and_run_callbacks`: Increments a counter in the structure, runs two subroutines, calls a flag-guarded routine and a stored callback pointer, then returns the input value.
- `run_flagged_callback_and_return`: Calls a flag-guarded routine when its flag byte is set, then calls a stored callback pointer, and returns the input value.
- `call_stored_routine_then_callback`: Calls a routine at a computed address, then calls the stored callback pointer when it is set, and returns the input value.
- `call_pointer_if_set`: Calls the function pointer at a fixed offset of the structure when it is non-null, and returns the input value.
- `update_joystick_debounce_counters`: Shifts the previous input state, derives new press flags from the control word, and updates four per-input counters that wrap at 25 and set a hold flag after 50 frames.
- `save_and_update_input_state`: Saves the previous input state into memory, then derives new press flags and updates the per-input debounce counters with a 25-step wrap and a 50-frame hold threshold.
- `update_input_state_in_record`: Stores the previous input state into a record, then updates the press flags and debounce counters with a 25-step wrap and a 50-frame hold threshold.
- `update_input_state_array`: Stores the previous input state into an array, then updates the press flags and debounce counters with a 25-step wrap and a 50-frame hold threshold.
- `update_input_state_with_slot`: Stores the previous input state in a slot, clears the pending value, then updates the press flags and debounce counters with the same rules.
- `poll_joystick_port_input`: Reads the fire-button register and joystick port movement register as input state, updating the press flags, hold flags and debounce counters.
- `update_second_input_debounce`: Updates the debounce counters and hold flags for a second input channel, using a different control bit and a 50-frame hold threshold.
- `update_second_input_record`: Stores the previous second-channel input state into a record, then updates its debounce counters and hold flags.
- `update_fire_input_record`: Stores the previous fire-channel state into a record, then updates its debounce counters and hold flags using the control bit for that channel.
- `update_fire_input_array`: Stores the previous fire-channel state into an array, then updates its debounce counters and hold flags with the same 25-step wrap and 50-frame hold threshold.
- `poll_mouse_port_input`: Reads the fire-button register and the mouse port movement register as input state, updating the press flags, hold flags and debounce counters.
- `decode_keyboard_code_to_input`: Decodes a keyboard code byte by inverting and rotating it, records any change, and sets or clears the matching input bits in the state words.
- `decode_keyboard_serial_byte`: Reads the keyboard serial register value through its stored byte, decodes it by inverting and rotating, records the change and sets or clears the matching input bits.
- `set_input_state_bits`: Ors a 16-bit mask into two input state words, one in the structure and one in the global input state.
- `set_secondary_input_word_bits`: Ors a 16-bit mask into a secondary input state word in the structure, then into the global input state.
- `clear_input_state_bits`: Ands a 16-bit mask into the global input state word, clearing the bits that are not in the mask.
- `clear_secondary_input_bits`: Ands a 16-bit mask into a secondary input state word in the structure, clearing the bits that are not in the mask.
- `do_nothing_routine`: the routine is empty and performs no operations before returning.
- `map_key_code_to_entry`: Maps a key code in the range zero to sixty-nine through a byte table to the stored entry, storing minus one for codes outside that range.
- `store_indexed_handler_pointer`: Stores an entry from a code-pointer table, indexed by a 16-bit value, into the structure.
- `update_audio_channel_state`: When the flag is set, reads the audio control register and, for each of the four channels, either counts down its length and enables its DMA bit or clears the bit and loads the sample pointer and full volume into the channel registers.
- `update_audio_state_in_record`: Performs the same four-channel audio update as its sibling routine, but keeps the DMA mask and counters in the structure passed to it.
- `load_intro_resources_from_disk`: Allocates two fixed-size memory areas, reads intro data records into them through device calls, sets up display and drive state, and frees the buffers on each error path, returning minus one on failure.
- `call_subroutine_with_saved_args`: Saves three stacked arguments into the structure, calls a subroutine with them, stores its error result and returns minus one when the subroutine failed.
- `copy_image_header_fields`: Stores three pointers from the stacked arguments and copies the byte and word header fields into the output structure, deriving a byte width by shifting the width value.
- `search_word_table_for_value`: Scans a table of 16-bit entries for the value passed in, stopping when it is found or when the stored entry count runs out.
- `fill_grid_cells_with_subroutine`: Walks a grid of rows and columns whose sizes are stored in the structure, calling a subroutine for each cell.
- `decode_run_length_bytes`: Decodes a run-length byte stream: a positive count copies that many literal bytes, a negative count repeats the next byte, until the requested length is filled.
- `unpack_twelve_bit_samples`: Unpacks packed 12-bit values from three-byte groups into consecutive 16-bit words, with the number of groups taken from the first longword divided by three.
- `call_setup_with_structure_values`: Passes three stored values from the structure to a subroutine.
- `initialise_bitmap_row_table`: Computes an aligned row width and stride from its arguments, stores them in the structure, builds a table of row offsets, clears the table area and copies a default data block into it.
- `compute_text_alignment_offsets`: Computes the horizontal start and end offsets for a text line from its alignment letter, R for right and L for left, centring the text for any other letter, and stores the results in the structure.
- `execute_logic_with_flags`: Executes logic that includes a call to a function that returns a 64-bit result, returning the low 32 bits if a flag is set.
- `call_store_result_or_fail`: Calls a subroutine with three stacked arguments; when its high result is zero it returns minus one, otherwise it stores the low result in the structure and returns the high result.
- `parse_script_directive_line`: Parses directives from a text script: skips blank characters, handles the F, A and N commands, counts entries and computes timing values until the caret terminator.
- `fill_frame_table_entries`: Computes a row count from the width and height, then walks a linked chain of frames, filling the structure with each frame's offset, position and size fields.
- `parse_up_to_four_digits`: Reads up to four decimal digits from a text pointer and combines them into a number, returning the value.
- `measure_text_width_in_font`: Measures the pixel width of a text word by summing per-character widths from the font table, stopping at whitespace and returning minus one at the caret terminator.
- `draw_text_glyphs_until_break`: Walks a text string, mapping each character through a glyph table and passing each glyph to a drawing subroutine, skipping whitespace and stopping at the caret terminator.
- `stub_infinite_loop_block`: Contains only an infinite loop with no other work, so it never returns.
- `copy_bytes_backwards`: Copies a block of bytes from its end toward its start, moving each byte to an offset derived from the stored width.
- `decode_nibble_delta_stream`: Decodes a nibble-packed delta stream by summing lookup values for the two nibbles of each byte, then writes the reconstructed bytes backwards into the buffer.
- `search_longword_table_for_value`: Scans a table of longwords for the value passed in, returning when it is found or when the scan limit of 32767 entries runs out.
- `build_entry_file_names`: Builds a file name for each of fourteen entries by copying a base name and appending the RAW suffix, storing the resulting pointers in a table; returns minus one if a lookup fails.
- `build_raw_file_name_list`: Copies a base name and appends the RAW extension for each entry, loads the named file and records its size in a table; returns minus one when a load fails.
- `load_raw_file_sequence`: Appends the RAW extension to a base name, loads each file in a sequence, records the sizes in a table and returns minus one when a load fails.
- `return_failure_status`: Returns minus one without doing any other work.
- `copy_fixed_fields_and_run`: Copies one byte and one word from fixed offsets into global variables, runs two subroutine calls and returns zero.
- `clear_fourteen_pointed_longwords`: Zeroes the longword that each of fourteen pointer slots points to, starting from a global table.
- `silence_all_audio_channels`: Sets the volume registers of all four audio channels to zero and clears the related state words.
- `silence_audio_and_disable_dma`: Sets the four audio channel volumes to zero, clears the related state words and turns off DMA for the audio channels.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `open_device_path_for_load`: Splits a device name at a colon, tries to open the path, and on success either loads the file or returns a status; it returns a failure code when the path cannot be opened.
- `resolve_colon_prefixed_filename`: Splits a name at a device-style colon prefix, such as RAM:, then tries the full name, the remainder after the prefix and a combined lookup through library calls.
- `release_stored_handle`: Releases the resource held in a frame slot if that slot is non-zero, then clears the slot and returns zero.
- `read_block_and_advance`: When an active flag is set, reads one 4096-byte block at the current position and decrements the remaining byte count.
- `set_flag_if_tag_matches_2`: When a mode flag is set, stores a true value in a status byte only if the tag field at offset 40 of the referenced structure reads TPWM, and a false value otherwise.
- `clear_pending_flag_byte`: Clears a one-byte state flag in the routine's frame and returns zero to the caller.
- `initialise_stream_state`: Obtains a handle through a library call and keeps it in the frame.
- `decode_lz_stream_step`: In decode mode, reads control bits from the source and copies either literal bytes or runs of earlier output within a 4096-byte window until the input or output limit is reached.
- `unpack_lz_literals_and_matches`: Reads control bits from the source: a clear bit copies one literal byte, and a set bit reads a two-byte token and copies a run of earlier output whose length is the token's low nibble plus two.
- `copy_literal_then_bit_runs`: Writes a literal byte and then reads control bits, where a set bit copies a run of earlier output using a length from the token.
- `copy_run_then_resume_bits`: Copies a back-reference run from earlier output using the token's low-nibble length, then resumes reading control bits.
- `copy_run_from_token_offset`: Copies a run of earlier output whose source offset comes from the token and whose length comes from its low nibble plus two, then resumes the control-bit loop.
- `save_register_snapshot`: Stores the data and address register values into a fixed save area of a context structure and clears a state flag byte.
- `return_low_word_if_flagged`: Calls a three-argument worker that returns a 64-bit result and returns its low 32 bits when the high 32 bits are non-zero, otherwise zero.
- `allocate_query_and_free_block`: Allocates a memory block through the system library, runs a lock and unlock pair on a resource with two further library calls, and reads the value at offset 124 of the block.
- `call_library_with_frame_record`: Passes the address of a frame record to one library routine and returns.
- `check_library_result_matches`: Calls one library routine and returns zero when its result equals the third argument, otherwise minus one.
- `call_library_with_frame_slot`: Passes the address of a frame slot to one library routine and returns.
- `call_library_with_frame_base`: Passes the address of the frame base to one library routine and returns.
- `call_library_function`: Calls a function pointer retrieved from a global offset and passes a stack-based or pointer-based argument.
- `reset_chip_interrupts_and_dma`: Calls two library routines, sets six shared variables to all-ones, writes a saved value to the interrupt enable register and sets the DMA control register to a fixed value.
- `write_five_slots_and_dma`: Stores its word argument into five shared variables and into a table slot, then loads the interrupt enable register and sets the DMA control register to fixed values.
- `write_four_slots_and_dma`: Stores its word argument into four shared variables and a table slot, then sets the interrupt enable and DMA control registers to fixed values and clears a global flag.
- `write_three_slots_and_dma`: Stores its word argument into three shared variables and a table slot, then sets the interrupt enable and DMA control registers to fixed values and clears a global flag.
- `write_two_slots_and_dma`: Stores its word argument into two shared variables and a table slot, then sets the interrupt enable and DMA control registers to fixed values and clears a global flag.
- `write_one_slot_and_dma`: Stores its word argument into one shared variable and a table slot, then sets the interrupt enable and DMA control registers to fixed values and clears a global flag.
- `reset_dma_after_word_store`: Stores its word argument into a table slot and sets the interrupt enable and DMA control registers to fixed values, then clears a global flag.
- `clear_interrupts_wait_blitter`: Clears pending interrupt requests and writes fixed values to the DMA and interrupt enable registers.
- `show_disk_error_requester`: Shows one of several disk prompts in a text requester, choosing between write-protected, fatal disk error, file not found and insert-disk messages, and calls a library routine.
- `swap_buffer_with_structure_region`: Swaps 32 words between a shared buffer and a region of a pointed-to structure, advancing four bytes through the structure for each word.
- `copy_shared_buffer_into_structure`: Copies 32 words from a shared buffer into a region of a pointed-to structure, advancing four bytes through the structure for each word.
- `find_trackdisk_device_node`: Locates the trackdisk.device entry by calling a name-search routine held in a shared pointer, and returns the result to the caller.
- `scan_trackdisk_unit_table`: Looks up the trackdisk.device entry, and if a size field in the caller's record exceeds a limit it calls a library routine.
- `copy_zero_terminated_string`: Copies bytes from a source buffer to a destination buffer until it copies a zero byte, which it also writes to the destination.
- `initialise_record_from_shared_state`: When a shared enable flag is set, clears two fields of a record and sets another to 256, then copies a shared long and a computed word into the record.
- `program_blitter_region_copy`: When a shared flag is set, initialises a record and then programs the blitter's masks, modulo values, pointers and size word to copy a rectangular region.
- `build_bitmap_header_and_rows`: Clears the header fields of a bitmap record, stores its width and height from the size argument, and computes the words per row as width plus fifteen divided by sixteen.
- `plot_row_bits_from_table`: Sets one bit in a flag byte of a record, then copies words from a source table into successive output rows.
- `update_entry_coordinate_table`: When a shared flag is set and a countdown is non-zero, steps through a table of entries and computes adjusted coordinate pairs from shared offsets, shifting the results before writing them back.
- `setup_clipped_blitter_copy`: Computes a blit size from a record's height and width, sets clip edges at 319 and 199, waits for the blitter to go idle and programs its masks, modulo values and pointers.
- `compute_fixed_point_clip_extents`: Converts two fixed-point coordinate pairs into absolute magnitudes with sign correction and rounding, then derives the clipped extents and loop counts for a copy.
- `prepare_blit_parameters_from_record`: Reads width and height fields from a record and a shared descriptor, computes the row count and byte width of the image, and stores them in shared blit variables before the blit routine runs.
- `draw_horizontal_bitplane_span`: Works out the span between two shared x positions and the bit offset within a byte, then writes the span into each plane's row buffer.
- `blit_rectangle_with_masks`: Compares two shared edge values and, when they differ, programs the blitter's source and destination data, masks, modulo values and size word to fill the span.
- `wait_for_blitter_idle`: Polls the blitter-busy status bit until it clears and then returns.
- `compute_span_mask_for_plane`: Derives a byte offset from a shared row position and a bit offset from the shared x value, adjusting x through a helper when a flag is set.
- `check_point_against_clip_bounds`: Compares an x and y coordinate against the four bounds of a clip record and returns in every case, so the routine has no visible effect.
- `check_row_column_clip_limits`: Compares an x and y value against two of the four bounds of a clip record and returns in every case, so the routine has no visible effect.
- `compute_span_bounds_and_offsets`: Adds one to a shared end coordinate, orders it against the start coordinate and orders the two shared y values, then turns the results into a byte offset and bit values for the span, calling one routine to finish.
- `order_and_shrink_shared_edges`: When two shared edge values differ, stores the smaller as the start edge, moves the end edge inward by one, and calls routines to update the shared x position and table.
- `store_allocated_buffer_pointer`: Calls one routine and stores the pointer it returns in a shared buffer slot.
- `copy_block_when_source_valid`: When the source pointer is above a fixed threshold, obtains a destination from a routine and copies a counted block of words into it.
- `start_copper_list`: Loads the copper list pointer from a shared value, triggers the copper jump and enables the DMA channels for display, bitplanes, copper and blitter.
- `copy_shared_values_into_record`: Calls one routine, then copies several shared values into fixed fields of a record and sets one field to a byte pattern repeated across both nibbles.
- `build_row_offset_table`: Fills a table with running multiples of a shared stride, one entry per row, starting from zero.
- `compute_bitplane_size_values`: Derives shared row stride, plane stride and total byte count from shared width, height and depth values.
- `allocate_and_fill_plane_pointers`: Allocates a bitplane buffer on first use, doubling its size when a shared flag is set, then stores a sequence of plane pointers spaced by the plane stride.
- `call_routine_return_zero`: Passes the frame pointer to one routine and returns zero to the caller.
- `unpack_lz_run_block`: Reads control bits from a compressed block: a clear bit copies one literal byte and a set bit copies a run of earlier output, with the run length taken from a token's low nibble plus two.
- `clip_and_blit_sprite_rectangle`: Offsets a sprite position by its origin, tests it against shared clip limits and, when visible, computes the clipped width and bit shift.
- `build_masked_blit_control_list`: Selects a drawing mode from a stack argument, then steps through a mask word bit by bit, storing a blitter control value chosen for each bit and advancing the destination pointers across a list of entries.
- `update_sprite_clipping_and_list`: Calculates a new sprite attribute or position based on a bounding box, then updates a linked list of sprite records with a new or modified entry.
- `initialise_sprite_record_from_params`: Sets the status fields of a sprite record to fixed values, then copies a position and two attributes from the parameter structure.
- `measure_text_width`: When a flag is set, walks a zero-terminated text and adds up the width of each character by calling a measuring routine, then returns the total.
- `program_blitter_from_table_entry`: Looks up an offset in a table, sets the data pointer and size registers, computes a source offset from two coordinates and programs the blitter's modulo, pointer and size registers.
- `copy_words_to_shared_structure`: Copies 32 words from the argument buffer into both a shared buffer and a region of a pointed-to structure, advancing four bytes through the structure for each word.
- `copy_words_into_structure_region`: Copies 32 words from the argument buffer into a region of a pointed-to structure, advancing four bytes through the structure for each word.
- `compare_shared_entry_nibbles`: Clears 32 shared words, then compares bits 8 to 11 of successive entries in two shared buffers and records whether any entry differs.
- `stub_returns_immediately`: The decompiled body is only a return, so the routine has no visible behaviour.
- `stub_returns_without_action`: The decompiled body is only a return, so the routine has no visible behaviour, even though its call list suggests otherwise.
- `set_global_flag_to_negative`: Checks if a global variable is zero and, if so, sets it to negative before recursively calling itself twice.
- `reset_global_flag_to_zero`: Checks if a global variable is non-zero and, if so, sets it to zero before recursively calling itself twice.
- `wait_for_raster_row`: Polls the video position register until the beam reaches a row computed from two shared values, then returns.
- `wait_until_shared_raster_row`: Polls the video position register until the beam reaches the row held in a shared variable, then returns.
- `fill_plane_pointer_table`: Writes one pointer per plane into a table, each computed as a shared base address plus that plane's offset, and writes one more entry when a flag is set.
- `swap_plane_pointer_lists`: Exchanges entries between the current and alternate plane pointer lists, swapping one extra entry when a flag is set, then updates a shared pointer.
- `prepare_blit_record_fields`: Calls one routine for a value, stores it with a width and a fixed control word into a blit record, and returns an offset-adjusted sum of two shared values.
- `prepare_blit_record_alternate`: Stores a returned value, a width and a fixed control word into a record, then copies a shared value into a second field and loops over the entries.
- `wait_for_blitter_and_fail`: Polls the blitter-busy bit until it clears, then returns minus one.
- `wait_blitter_read_pair`: Polls the blitter-busy bit until it clears, then returns two table entries packed into one result.
- `allocate_and_record_entry`: Calls a library allocation routine and, if it succeeds, records the argument and the returned pointer as a pair in the next slot of a table and increments the count.
- `record_allocation_pair`: Calls a library allocation routine and, when it succeeds, records the argument and the returned pointer in the next table slot and increments the count.
- `copy_memory_block_aligned`: Copies a block of bytes between two buffers.
