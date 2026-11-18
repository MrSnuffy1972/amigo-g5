# Game 866 (alien storm (1991)(u. s. gold)[cr vf]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00866). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (913344 top-level bytes in 66 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 44 raw-word lines |
| Functions named and described | pass, 315 of 315 named (0 author symbols), 315 of 315 described (94 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 828 of 828 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 131540 bytes, 9 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10895 of 10897 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 341891 |
| code | 34816 |
| data | 434535 |
| padding | 17110 |
| unused | 84992 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Alien Storm (1991)(U. S. Gold)[cr VF].adf:vf1 | 2 | 79 | 3 |
| Alien Storm (1991)(U. S. Gold)[cr VF].adf:BERSERKER | 3 | 18 | 1 |
| Tbar | 2 | 35 | 3 |
| Alien Storm (1991)(U. S. Gold)[cr VF].adf:preferences | 50 | 8896 | 247 |
| Alien Storm (1991)(U. S. Gold)[cr VF].adf:explode.library | 1 | 443 | 31 |
| Alien Storm (1991)(U. S. Gold)[cr VF].adf:VF1991 | 1 | 48 | 2 |
| Alien Storm (1991)(U. S. Gold)[cr VF].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 875 | 18 |
| Alien Storm (1991)(U. S. Gold)[cr VF].adf:vf1#unpacked | 1 | 320 | 5 |
| Alien Storm (1991)(U. S. Gold)[cr VF].adf:a.prg (raw code file, load address unknown) | 1 | 173 | 4 |
| Alien Storm (1991)(U. S. Gold)[cr VF].adf:CHARSX.NAB (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:CHRA.CHR: high-entropy data file (7.81 bits/byte), classified as packed data
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:CHR5.CHR: high-entropy data file (7.62 bits/byte), classified as packed data
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:ELEC3D.CHR: high-entropy data file (7.74 bits/byte), classified as packed data
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:SCOOTER1.PC1: high-entropy data file (7.85 bits/byte), classified as packed data
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:CHRH.CHR: high-entropy data file (7.79 bits/byte), classified as packed data
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:KARLA2.PC1: high-entropy data file (7.75 bits/byte), classified as packed data
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:CHR8.CHR: high-entropy data file (7.78 bits/byte), classified as packed data
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:CHR6.CHR: high-entropy data file (7.64 bits/byte), classified as packed data
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:SCOOTER2.PC1: high-entropy data file (7.81 bits/byte), classified as packed data
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:SHIP3D.CHR: high-entropy data file (7.91 bits/byte), classified as packed data
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:CHRI.CHR: high-entropy data file (7.65 bits/byte), classified as packed data
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:chr7x.chr: high-entropy data file (7.70 bits/byte), classified as packed data
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:KARLA1.PC1: high-entropy data file (7.89 bits/byte), classified as packed data
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:BRAIN3D.CHR: high-entropy data file (7.79 bits/byte), classified as packed data
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:MARK3D.CHR: high-entropy data file (7.69 bits/byte), classified as packed data
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:MISCSPR.PC1: high-entropy data file (7.89 bits/byte), classified as packed data
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:CHRC.CHR: high-entropy data file (7.92 bits/byte), classified as packed data
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 109 64 KB pages each with >= 29491 distinct PCs executed, 1780350 of its 1783279 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 109 64 KB pages each with >= 29491 distinct PCs executed, 1780350 of its 1783279 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 109 64 KB pages each with >= 29491 distinct PCs executed, 1780350 of its 1783279 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 109 64 KB pages each with >= 29491 distinct PCs executed, 1780350 of its 1783279 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-x25k slid through memory (an address: 109 64 KB pages each with >= 29491 distinct PCs executed, 1780350 of its 1783279 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-x35k slid through memory (an address: 109 64 KB pages each with >= 29491 distinct PCs executed, 1780350 of its 1783279 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 000C) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 771 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:vf1: crunched executable unpacked with ancient scan (ByteKiller 2.0 (xfd), 13380 -> 13036 bytes), 5 of 5 windows confirmed in reference RAM, loaded at an address: recovered as u8
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:BERSERKER: Imploder Library (xfd) stream found by ancient scan (3764 -> 6660 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:a.prg: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- alien storm (1991)(u. s. gold)[cr vf]/Alien Storm (1991)(U. S. Gold)[cr VF].adf:CHARSX.NAB: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate
- 2 executed PCs start inside another executed instruction (overlapping-instruction idiom): decoded and counted as instruction starts
- 3 never-loaded region(s), 131540 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Alien Storm (1991)(U. S. Gold)[cr VF].adf:vf1

- `unpack_lzss_stream_to_buffer`: Decodes a bit-packed compressed stream by repeatedly reading flag bits from a bit buffer; on a set flag it copies a back-referenced run of bytes from earlier output, otherwise it reads a literal byte from the stream, writing each byte downward through a destination pointer.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Alien Storm (1991)(U. S. Gold)[cr VF].adf:BERSERKER

- `open_explode_library_retry`: Repeatedly calls the exec library open function for explode.library at version 2 until it returns a non-null library base, looping back to the call while the result is zero.

### Tbar

- `copy_string_into_message_buffer`: Stores a pointer to the input string in a global variable and terminates the buffer at the length in d0, then either calls a function on a looked-up object when the first character is a question mark, or copies up to 72 bytes from the string into a buffer reached through a structure pointer and calls a library function at a fixed negative offset from the exec base.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_open_library`: Wrapper that calls the exec library function OpenLibrary and returns its result to the caller.

### Alien Storm (1991)(U. S. Gold)[cr VF].adf:preferences

- `parse_command_line_arguments`: Looks up the current task, and when its process-related pointer is nonzero it copies the command-line buffer into a global string buffer, trimming trailing whitespace and reading tokens separated by spaces or tabs.
- `restore_saved_handler_and_release_port`: Stores the parameter at the address held in a global variable minus 4, closes a library held in a global variable if it is non-null, and when a message pointer is non-null, calls exec Forbid and then replies to that message, finally returning the value saved at the first stack location.
- `call_two_library_vectors_on_object`: Passes a fixed pointer from a data area as argument to two library functions called at offsets minus 384 and minus 372 of the base held in a register, then returns.
- `loads_dos_library_into_global`: Loads the address of the string dos.library into a1, clears d0, and calls library offset minus 552 through a6.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_delete_file`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_date_stamp`: Library stub: loads its arguments and the dos library base, calls DateStamp and returns its result to the caller.
- `call_delay`: Library stub: loads its arguments and the dos library base, calls Delay and returns its result to the caller.
- `process_preferences_dialog`: Loads a preferences block from a global table and, if present, updates the relevant configuration values and display settings, then runs a modal dialog for the pointer, printer, and serial options.
- `initialize_preferences_window`: Opens the graphics and intuition libraries through a sequence of calls, then allocates a table of entries 44 bytes apart and fills 136 slots with pointers into it.
- `handle_preferences_menu`: Loads a preferences block from a global table and, if present, updates the relevant configuration values and display settings, then runs a message loop that dispatches menu selections to their handlers and replies to messages until the dialog is closed.
- `cleanup_resources_and_reset_interrupts`: Frees a fixed-size memory block based on global counters, traverses and frees a linked list of memory blocks, closes libraries, and resets interrupt vectors.
- `iterate_preference_table_entries`: Calls another routine twice with different first arguments, passing a calculated address derived from a global variable and a constant offset as the second argument and the input parameter as the third.
- `iterate_and_validate_params`: Iterates through a sequence of word-sized values from a memory block, passing each to another routine along with a constant and an index.
- `validate_and_pass_params`: Zeroes a parameter if it exceeds a limit, then decomposes another parameter into its nibbles and passes the parts to a library routine.
- `initialize_preference_entries`: Walks through a table of pointers, copying values from a global variable array into each structure's fields and clearing several fields beforehand.
- `process_preference_selection`: Handles a specific preference menu selection by branching on an input parameter to configure display modes, manage memory allocations, or update configuration tables.
- `dispatch_preferences_menu_choice`: Reads the current menu item number from the preferences structure, bounds-checks it against 37, jumps through a jump table to the matching case, then formats and returns a status value.
- `process_preference_buffers`: Checks whether a short parameter is nonzero and, if so, invokes another routine with a base value, then processes three memory locations by extracting nibbles from a word and clamping a global value into a signed range before storing the results.
- `apply_preference_clamps`: Clamps input parameters to specific signed ranges and adjusts them based on offset values within a data structure if a condition is met.
- `clamp_and_apply_preferences`: Clamps two byte parameters to a signed range and adjusts them based on offset values within a data structure if a condition is met, storing the results back.
- `map_colour_index_to_palette_slot`: Compares the word argument on the stack with 9, returning 3 in d0 when equal and 4 otherwise.
- `select_byte_for_odd_flag`: Tests bit 0 of the byte argument on the stack and returns 37 in d0 when that bit is clear and 36 when it is set.
- `compare_strings_case_insensitive`: Walks two zero-terminated byte strings from their pointer arguments, converting lowercase letters to uppercase with a mask, and returns true only when both strings end at the same position with equal characters.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_add_task`: Library stub: loads its arguments and the exec library base, calls AddTask and returns its result to the caller.
- `call_rem_task`: Library stub: loads its arguments and the exec library base, calls RemTask and returns its result to the caller.
- `call_find_task`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_wait`: Library stub: loads its arguments and the exec library base, calls Wait and returns its result to the caller.
- `call_signal`: Library stub: loads its arguments and the exec library base, calls Signal and returns its result to the caller.
- `call_alloc_signal`: Library stub: loads its arguments and the exec library base, calls AllocSignal and returns its result to the caller.
- `call_free_signal`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `call_add_port`: Library stub: loads its arguments and the exec library base, calls AddPort and returns its result to the caller.
- `call_rem_port`: Library stub: loads its arguments and the exec library base, calls RemPort and returns its result to the caller.
- `call_get_msg`: Library stub: loads its arguments and the exec library base, calls GetMsg and returns its result to the caller.
- `call_reply_msg`: Library stub: loads its arguments and the exec library base, calls ReplyMsg and returns its result to the caller.
- `call_find_port`: Library stub: loads its arguments and the exec library base, calls FindPort and returns its result to the caller.
- `call_close_library_2`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_open_device`: Library stub: loads its arguments and the exec library base, calls OpenDevice and returns its result to the caller.
- `call_close_device`: Library stub: loads its arguments and the exec library base, calls CloseDevice and returns its result to the caller.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `call_send_i_o`: Library stub: loads its arguments and the exec library base, calls SendIO and returns its result to the caller.
- `call_abort_i_o`: Library stub: loads its arguments and the exec library base, calls AbortIO and returns its result to the caller.
- `call_open_library_2`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_init_semaphore`: Library stub: loads its arguments and the exec library base, calls InitSemaphore and returns its result to the caller.
- `call_obtain_semaphore`: Library stub: loads its arguments and the exec library base, calls ObtainSemaphore and returns its result to the caller.
- `call_release_semaphore`: Library stub: loads its arguments and the exec library base, calls ReleaseSemaphore and returns its result to the caller.
- `call_attempt_semaphore`: Library stub: loads its arguments and the exec library base, calls AttemptSemaphore and returns its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `allocate_message_entry`: Attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a value from another routine before dispatching it.
- `delete_port`: Removes the port from a global list if a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal bit, and releases the memory block.
- `format_date_string`: Initializes a memory buffer with date separator characters and a null terminator.
- `parse_date_string`: Iterates through a global string buffer to validate and extract date components.
- `convert_numbers_to_ascii`: Splits each of three integer parameters into tens and units digits and stores the corresponding ASCII characters into consecutive global memory locations.
- `apply_ascii_configurations`: Calls a library function with several constant arguments, copies ASCII values from global memory into other global variables, and computes a table index from digit values before making a final call.
- `calculate_timer_intervals`: Computes timer intervals by parsing ASCII date/time components, performing arithmetic adjustments, and configuring timer device requests.
- `wrap_and_format_date_fields`: Wraps the day, month and year digit buffers back to their minimum values when they exceed their limits, using a switch on the month that accepts days up to 31, 30 or 28 or 29 for February.
- `fill_pattern_bytes_and_branch`: Writes four bytes of a fixed pattern through the address held in a0, using post-increment, then writes two more bytes backwards with pre-decrement, then sets d0 to 31 and branches forward into another routine that wraps the counters and formats the result.
- `load_limit_and_branch_to_wrap`: Loads the constant 30 into d0 and compares it with d3, then branches forward into the middle of another routine that increments the month counter and wraps the counters.
- `wrap_month_and_clamp_day`: Increments the value in d2 and wraps it to 1 if it exceeds 12, clamps the value in d4 to 0 if it is 100 or more, and then calls the ASCII conversion routine with these values and the constant 1.
- `clamp_date_fields_and_format`: Raises undersized digit bytes in the date buffers to their minimum values, then limits the day to 28, 29, 30 or 31 depending on the month, and passes the clamped day, month and year to another routine.
- `normalize_date_fields_and_format`: Reads a two-digit field from the string in a0 and clamps its first digit to '9' when below '0', then forces the first digit to '2' or '3' and caps the second at '3' if it exceeds it.
- `clamp_and_convert_values_alt`: Clamps two integer parameters to specific ranges, ensuring they are within bounds, and then calls a conversion routine to transform them into ASCII characters.
- `initialize_timer_device`: Opens the timer device and allocates an extended I/O request block, then configures the block's fields and waits for a timeout condition before returning a success status.
- `initializes_timer_and_signal_state`: Calls one routine with two arguments to create a signal entry and stores its result in a global variable, then calls another routine to obtain a second value and stores that in a second global variable.
- `opens_timer_device_with_arguments`: Pushes a zero longword, the value at a global pointer and two constants onto the stack, then calls a routine to open a device and tests its result.
- `updates_clock_style_display_timer`: Stores a value into a global variable, then loops, computing a timeout from a global byte and a fixed base and waiting on signal bits, checking message flags and a global counter on each pass, and repeatedly calls library vectors to refresh display state.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `initialize_timer_task`: Opens the timer device, allocates an I/O request, and starts a background task that loops forever handling timer signals and I/O.
- `stop_time_task_and_release_resources`: Checks a signal bit, stops the time task, frees its memory and port, clears the task pointer, clears flag bit 2 and returns.
- `set_request_type_and_arguments`: Reads two long arguments and a pointer from the stack, stores the word value 9 at offset 28 of the structure and the two arguments at offsets 32 and 36, then calls another routine with the structure pointer.
- `check_global_flag_change`: It reads a global variable, calls a delay function, and returns a boolean indicating whether the lower four bits of that global variable have changed.
- `wait_for_condition_timeout`: It conditionally writes constant values to a global structure, sets a bit in a word within that structure, and loops decrementing a counter until a specific bit in the structure is cleared or the counter expires.
- `clear_structure_mask_bits`: Loads a global structure pointer and clears bits 16 and 0 of the long at offset 52 of that structure, leaving the other bits unchanged, then returns.
- `save_system_configuration`: Opens a system configuration file for writing and closes it if opening fails.
- `read_config_block_from_file_2`: Opens the system configuration file, reads 232 bytes into the buffer passed in, and checks that the byte count matches before closing the file.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_init_code`: Library stub: loads its arguments and the exec library base, calls InitCode and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_forbid_2`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `call_set_int_vector`: Library stub: loads its arguments and the exec library base, calls SetIntVector and returns its result to the caller.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_allocate`: Library stub: loads its arguments and the exec library base, calls Allocate and returns its result to the caller.
- `call_alloc_mem_2`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_free_entry`: Library stub: loads its arguments and the exec library base, calls FreeEntry and returns its result to the caller.
- `call_find_task_2`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_signal_2`: Library stub: loads its arguments and the exec library base, calls Signal and returns its result to the caller.
- `call_wait_port`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `call_add_library`: Library stub: loads its arguments and the exec library base, calls AddLibrary and returns its result to the caller.
- `call_old_open_library`: Library stub: loads its arguments and the exec library base, calls OldOpenLibrary and returns its result to the caller.
- `call_global_vector_with_args`: Saves d2, loads the stack argument into d2, pushes a pointer to the argument area and the value in d2 and a global longword, calls through the vector at address zero, then removes the pushed arguments and restores d2.
- `dispatches_indexed_table_entries`: Calls a helper with a global variable address and then walks several ranges of indexed table entries, passing each entry's coordinates and related fields to drawing routines.
- `selects_and_walks_message_list`: Clears the word register d2, loads the first pointer from the address in a2 into a0, and compares the longword at offset 20 of that structure with the constant 16, branching when it differs.
- `drain_message_port_until_done`: Loops over a message list pointed to through a stack-relative pointer, repeatedly fetching messages with the library's message retrieval and replying to them, dispatching each to the palette update routine when its type code is 16 or 64, and clears a flag as it goes.
- `walks_list_and_dispatches_codes`: Iterates over a pointer held in the stack-relative register a2, reloads it from a structure reached through a second pointer, and calls helpers to process each entry.
- `call_routine_return_zero`: Pushes two long arguments onto the stack, calls another routine, removes the arguments, and returns 0 in d0.
- `push_pointer_and_constant_then_jump`: Pushes the longword at the address held in a4 and the constant 252 onto the stack, then branches forward into the middle of another routine.
- `loads_pointer_and_calls_subroutine`: Loads the pointer stored at the address in a3 into a0, pushes the longword at offset 86 of that structure onto the stack, and calls a subroutine at a fixed address.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `copy_five_structure_words`: Copies the words at offsets 168, 170, 172, 174 and 218 from the structure pointed to by the second stack argument into the structure pointed to by the first, then returns.
- `dispatches_state_code_to_actions`: Loads a pointer and a long value from a structure reached through a global pointer, then switches on the 16-bit state code at offset 38 of that structure, updating global flags and bytes per case, and returns 1 or 2 for the codes 64 and 65 or 0 otherwise.
- `call_stub_then_return_one`: Pushes the constant 48 and a pointer to a global variable, calls another routine with them, then returns the value 1 in d0 after removing the pushed arguments from the stack.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `draw_indexed_pen_entry_pair`: Stores 2 into the pen field at offset 24 of the structure referenced by a global pointer, draws two entries using 16-bit offset values from tables indexed by the word argument shifted left by three, then sets that pen field to 255.
- `save_counter_and_branch_onward`: Pushes the longword in d2 onto the stack, loads a word from 10 bytes above the stack pointer into d1, clears the word in d2, and branches forward into the middle of another routine.
- `advance_index_and_compare`: Increments the word in d2, doubles it into an index, and compares a word read from a table at that index against d1.
- `decrement_register_value`: Zero-extends the low word of d2 into d0, subtracts one from it, and returns the result.
- `initialize_preference_buffers`: It clears a global flag, then loops over a table of six entries, calling a helper routine to allocate or retrieve buffers for each entry and storing the result.
- `call_library_with_flag`: Pushes the constant 1 and a pointer to a global variable, calls another routine with them, removes the two pushed arguments from the stack, and returns.
- `push_constants_and_branch_on_flag`: Clears d0 and loads the word from a fixed global address into its low half, keeps only bit 12 of d0, and branches on that result; the two paths push the constants 2, 3 and 73 onto the stack before branching forward into another routine.
- `call_other_with_three_args`: Pushes the constants 2 and 3 and a value computed from a global word plus 70 onto the stack, calls another routine with those three arguments, then cleans the stack and returns.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `load_printer_list`: Scans the DEVS:printers directory to enumerate available printers and stores their names in a sorted global table.
- `compare_string_prefixes`: Compares two null-terminated strings character by character.
- `allocate_and_copy_entry`: Allocates a memory block via the exec library and stores the returned pointer in a global variable.
- `free_linked_list_entries`: Iterates through a linked list of memory blocks pointed to by a global variable, freeing each block by calling another routine, and finally resets the global list pointers to zero.
- `validate_configuration_entries`: Clears a global counter, then repeatedly calls another routine to validate each entry in a table of pointers indexed by that counter.
- `validate_configuration_string`: Builds a temporary buffer by filling it with spaces and then copying characters from a global table into it.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `iterate_table_entries`: Iterates through a range of indices, retrieving a pointer from a global table for each and passing it to another routine along with a global variable.
- `schedule_table_entries`: Loops over a range of indices and calls another routine for each entry in a global table, passing the table entry address and a constant value to it.
- `process_preference_switch`: Reads configuration values from memory structures based on a global switch index.
- `configure_display_modes`: Iterates through a 16x16 grid of bits derived from a global configuration table.
- `measure_zero_terminated_string_length`: Walks forward from the pointer argument until a zero byte is found and returns the byte distance between the end pointer and the start pointer.
- `configure_display_regions`: Acquires a semaphore, then calls a library routine multiple times with coordinates and parameters derived from the input arguments, and releases the semaphore upon completion.
- `draw_rectangle_sequence`: Acquires a semaphore, then calls a library routine repeatedly with coordinates derived from the input parameters to draw shapes, and releases the semaphore after the drawing operations.
- `select_and_pass_constant`: Selects one of two constant values based on an input parameter and passes it to another routine along with fixed coordinates.
- `initialize_library_instance`: Opens two libraries and, if either fails, closes one and returns zero; otherwise it initializes several fields of the first library structure with calculated addresses and flags.
- `wait_for_signal_completion`: Sets a flag and signals a task when a global variable is nonzero, then loops calling a delay routine until that variable becomes zero, and finally signals another task if given a nonzero argument.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `modular_inverse_32bit`: The routine performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_division_arguments`: Saves d2, moves d1 into d2 and d0 into d1, calls the division routine, then restores d2 and returns.
- `return_division_remainder`: Saves d2, shifts d1 into d2 and d0 into d1, calls the division routine, copies d1 into d0 as the return value and restores d2.
- `negate_result_on_flag`: Calls the division routine, then negates the returned value when the address register a0 holds a nonzero word.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `handle_preference_input`: Sends a message to a library, stores several derived values into a structure, and then loops reading messages and dispatching on their status to update preference settings.
- `copy_preference_fields_between_records`: Copies a word at offset 2 and bytes at offsets 183, 182 and 184 from the source record to the destination record, then returns.
- `prefs_handle_arrow_key_switch`: Saves d2/a2-a3, loads the preferences object pointer, reads its key code at offset 38 and dispatches through a jump table for codes 89 to 106, returning a status in d0.
- `execute_resource_setup`: Calls several routines using a pointer from a structure, performs a calculation using a value from a global table, and calls another routine with the result.
- `apply_preferences_screen_settings`: Pushes arguments to five graphics routines using the display structure pointer, then masks and sets the low nibble of a preference byte and returns zero.
- `handle_input_events_loop`: Initializes display configuration and preference data, then enters a loop to wait for and retrieve input messages.
- `copy_and_schedule_tasks`: Copies several fixed fields and a looped block of word-sized entries from one memory location to another.
- `initialize_display_configuration`: Frees previously allocated memory and cleans up resources, then copies configuration values into global variables and configures display colors and modes using Exec library functions.
- `dispatch_prefs_gadget_by_number`: Saves registers, loads the current gadget pointer and its data, reads the gadget number and jumps through a table for numbers between seventy-five and eighty-eight, returning zero otherwise.
- `decode_and_calculate_values`: When a global flag is nonzero, it decodes a 16-bit value into nibbles and passes them to another routine.
- `store_word_recursive`: Writes a word built from three parameters into a global table at an index derived from a second parameter, then calls itself recursively with an incremented index.
- `configure_display_colors`: Reads byte values from global memory and passes them, with small offsets, to a library routine four times, likely configuring display color registers.
- `schedule_preference_tasks`: Invokes another routine three times with a global pointer, a pointer to a global variable, and a constant argument, acting as a wrapper to schedule preference operations.
- `query_and_adjust_state`: Adjusts a parameter, then calls another routine repeatedly with coordinates derived from the input parameters and conditional flags based on a global variable.
- `configure_display_modes_2`: Calls a library routine, then enters a nested loop that iterates through a 16x16 grid, calculates a value from a global configuration table, and calls another routine with that value.
- `modify_word_bits`: Clears specific bits in two global word variables using a mask derived from a shift amount, then sets bits based on another parameter.
- `query_display_state`: Calls library routines to query display state, checks a return value, and conditionally calls another routine with calculated offsets before a final call.
- `initialize_preferences_then_dispatch`: Clears a 30-entry global array, then calls the table validation routine and copies three word values from global variables into fixed global slots before calling a copy routine three times.
- `copy_struct_fields_partial`: Copies specific word-sized and byte-sized fields from a source structure to a destination structure at matching offsets.
- `dispatch_state_by_code`: Reads a 16-bit state code at offset 38 of a structure pointed to by a global variable and branches on it, redrawing pen-style parameters and toggling flag bits in globals per case, with a loop over 30 bytes of a global block in one case.
- `store_global_words_and_call`: Stores the constant 2 into a 16-bit global variable, pushes a 32-bit global variable onto the stack and calls another routine with it, then calls a further routine with no arguments.
- `sets_two_global_words_then_calls`: Calls one routine with no arguments, stores the constant 38 into one 16-bit global variable and the constant 1 into another, calls a second routine, and returns 0 in d0.
- `send_library_messages`: Sends a series of messages to a library base stored in a global variable, passing various integer codes and pointers derived from global data structures.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `update_preference_bounds`: Updates two global variables based on comparison logic between a third global variable and a constant.
- `allocate_preference_entry`: Allocates a memory entry via the exec library and schedules a task to initialize it, then invokes a function pointer from a global table to process the new entry.
- `init_preference_list`: Sets a global initialization flag on first entry, then adjusts a stored value against a global variable before invoking another routine.
- `init_preference_entries`: Allocates and initializes a linked list of preference entries using exec library calls, scheduling the initialization task and managing memory for the entry nodes.
- `initialize_word_and_refresh`: Compares a longword global variable against one and, if it is below one, stores the word 1 into a global variable and calls another routine.
- `init_preference_entry`: Allocates a new entry in the exec list and initializes its fields using values from a global variable and a table offset before linking it into the preference list.
- `process_configuration_table`: Iterates over a range of indices and calls another routine for each entry in a global table, then derives several bit fields from a configuration word and passes computed offsets to another routine.
- `runs_task_loop_and_signal_wait`: Calls a drawing routine with the first longword from a global structure, then draws glyph 107 and updates a byte flag.
- `loops_dispatching_preference_items`: Repeatedly clears a 16-bit result, calls another routine with the longword at offset 20 of the structure referenced from a2 when that value is 64, then calls another routine with the pointer in a2 and tests the result.
- `copy_prefs_block_and_return_zero`: Pushes the address of a global variable and a fixed constant onto the stack, calls another routine to copy the preferences block, removes the arguments, and returns 0 in d0.
- `push_block_address_then_jump`: Pushes the constant 252 and the longword at the address held in a4 onto the stack, then branches forward into the middle of another routine.
- `loads_pointer_calls_then_branches`: Loads the pointer stored at the address in a3 into a0, pushes the longword at offset 86 of that structure onto the stack, and calls a subroutine at a fixed address.
- `function_epilogue_4`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `copy_prefs_tail_fields`: Copies the words at offsets 218, 220 and 222 and the bytes at offsets 224 and 225 from the structure at the first stack argument into the structure at the second, then returns.
- `dispatch_preference_item_state`: Reads a 16-bit item identifier from offset 38 of a structure referenced by a global pointer and switches on it, with ranges 107 to 108 returning 2 and 1 respectively.
- `push_constant_then_return_one`: Pushes the longword constant 48 onto the stack, calls another routine with that argument, removes the argument from the stack, and returns the value 1 in the data register d0.
- `draws_bounded_pen_block`: Saves registers, loads the address constants of two global blocks into a2 and a3, and tests bit 11 of the 16-bit word at offset 218 in the structure in a3.
- `draws_status_glyphs_with_mask`: Draws glyph 110 and then either glyph 111 with a call to a helper or glyph 112, depending on bit 3 of byte 219 of the structure in a3.
- `draws_status_flag_glyphs`: Draws fixed glyph 112 with the drawing vector in a2, then draws glyphs 113 to 115 for bits in byte 224 of the structure in a3 and selects further glyphs from bits 0 to 7 of bytes 219 and 218.
- `tests_flag_and_pushes_args`: Clears d0, loads the 16-bit flag word at offset 218 of the structure in a3 into d0 and masks it with bit 10.
- `dispatch_state_flag_codes`: Reads a 16-bit flag word at offset 218 and two flag bytes at offset 219 of the structure pointed to by a3, and calls the routine in a2 with the constants 123, 3 and 2 and then with one code chosen from the flag bits (126 or 127 from bit 8 of the flag word, and 129 to 132 from bits in the flag byte).
- `conditionally_dispatch_stack_call`: Pushes the constants 2, 3 and 126 as stack arguments and calls the routine whose address is in a2, then removes the arguments and clears d2.
- `tests_flag_and_dispatches`: Tests bit 5 of the byte at offset 219 in the structure in a3 and, when that bit is clear, branches past the call.
- `tests_flag_and_pushes_constants`: Tests bit 6 of the byte at offset 219 in the structure in a3 and, when that bit is clear, branches past the call.
- `conditionally_calls_handler_with`: Tests bit 7 of the byte at offset 219 in the structure in a3 and skips ahead when it is clear.
- `pushes_args_and_dispatches`: Pushes the constants 2, 3 and 128 as stack arguments and calls through the address in a2, then pops the arguments.
- `calls_two_routines_with_d2`: Calls one routine with no arguments, then sign-extends the low word of d2 into a 32-bit value, pushes it as an argument to a second routine, and removes the argument afterwards.
- `draw_prefs_widgets_sequence`: Calls a sequence of five drawing routines on the structure referenced by a global pointer, passing fixed coordinate values, a constant index, and an entry selected from a global array by the low three bits of another global byte.
- `skips_string_spaces_and_branches`: Saves registers d2 to d4 on the stack, loads an argument pointer into a0, reads a byte argument into d3, and clears d4 and d0, then branches into the middle of another routine.
- `skips_spaces_in_string`: Advances the pointer in a0 by one byte, then steps forward past space characters while the byte at the pointer is nonzero, branching out when it reaches the terminator or a non-space character.
- `scan_digits_in_string`: Repeatedly reads bytes from the string at a0, returning as soon as a byte is below the character 0 or a zero terminator, and stopping when a byte is above the character 9, so it advances a0 over a run of decimal digits.
- `multiply_d0_by_ten`: Doubles the longword in d0, copies it into d1, shifts d0 left by two, and adds d1 back, leaving d0 as ten times its original value.
- `parse_decimal_with_fraction`: Reads decimal digit characters from the string in a0 into a running value, stopping at the first non-digit or zero terminator, and when a period follows and a flag bit in a global variable is set, it reads one more digit and shifts the result.
- `select_bytes_from_table`: Stores the values from the arguments on the stack into d2 and d4, compares the value at offset 16 with the value at offset 12, and clears d5 when the comparison is higher, then branches into a later part of the code.
- `copy_global_bytes_into_buffer`: Copies bytes from a global variable table into the buffer addressed by a register, one byte at a time, stepping through indices from zero up to the value held in a 16-bit field of the structure in a6 plus one.
- `format_five_bytes_as_decimal`: Stores five bytes returned by repeated calls into a stack array, then writes decimal digit characters into the output buffer in d2, inserting a leading zero and a decimal point when bits 0 and 1 of a stack byte are set, and finally writes a zero terminator and returns 1.
- `format_number_string`: It iterates through a data structure to build a decimal string representation in a buffer, inserting a decimal point based on flags and converting each digit, then null-terminates the resulting string.
- `initialize_preferences_display`: Extracts a nibble from a global word and then issues a series of library calls with derived coordinates and constant values to configure text formatting and layout parameters.
- `check_flag_and_allocate`: Checks a specific bit in a global variable; if set, it performs several library calls including memory allocation and returns the result.
- `call_two_cleanup_handlers_with_zero`: Pushes zero and two global pointers as arguments and calls a library vector twice, once for each pointer, then discards the stacked arguments and returns.
- `manage_library_semaphore_access`: Acquires a semaphore, then calls a library function several times using a parameter and values read from a global variable array, releasing the semaphore before returning.
- `store_calculated_short_value`: Computes a value from three short parameters and stores it into a global variable array at an index derived from the second parameter, then calls another routine with adjusted arguments.
- `process_three_buffers`: Extracts nibbles from a word in a global structure, then iterates over three buffer pointers, calling routines to process each buffer's data and state.
- `allocate_block_with_size_48`: Saves d2, pushes the argument and the constant 48 and calls the allocation routine through the vector at address zero, then removes the arguments and restores d2.
- `forward_pointer_to_free_routine`: Saves d2, pushes the pointer argument and calls the free routine through a fixed jump vector, then removes the argument and restores d2.
- `create_ext_io`: The routine allocates memory for an I/O request structure when the first argument is non-zero, initializes specific fields with constants and arguments, and returns the pointer to the block or zero.
- `delete_ext_io`: The routine clears specific fields in the I/O request structure and frees the memory block using the size stored within the structure, performing no operation if the pointer is zero.

### Alien Storm (1991)(U. S. Gold)[cr VF].adf:explode.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `allocates_library_entry_and_stores`: Saves a5, copies the incoming a0 pointer into a field of the structure in a5, calls a library function to allocate an entry, stores the result in another field of that structure and returns; otherwise it jumps to a library vector with a constant.
- `patch_library_jump_vector`: Calls a library function at a fixed negative offset from a6, then reads a pointer from a global location and copies a 16-bit word and a 32-bit longword from it into fields of the structure at a5.
- `compares_byte_and_calls_library`: Compares the byte at the address in a0, which is then advanced by one, against the value 112, and branches forward when they match.
- `copy_byte_then_conditionally_dispatch`: Copies one byte from the source pointed to by a0 into the byte at offset 1 of the structure in a1, then compares the 16-bit word at offset 1 of the source against 24576.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `decrements_word_and_branches`: Subtracts 1 from the 16-bit word at offset 32 of the structure in a6, then branches to a later instruction if the result is nonzero.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `verify_library_header`: Library code that verifies a specific signature and function pointer within a structure before executing a callback.
- `set_bit3_at_offset34`: Sets bit 3 of the byte at offset 34 of the structure addressed by a6, leaving the other bits unchanged, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (moveq) and then falls through into the code that follows, with no return of its own.
- `verify_library_header_2`: Library code that checks a memory location for a specific four-byte signature and a string identifying the library.
- `decrement_timer_and_trigger_explosion`: Library code that decrements a short timer in a structure, calls a routine to handle an effect, and calls a function pointer from a global table if a threshold is met.
- `unpack_bitstream_into_table`: Initialises a decompression state structure by copying header bytes and table entries from the input stream, builds a table of 16-bit bit-length counts from the packed size codes, accumulates them into running offsets, and stores the stream pointer.
- `decode_stream_and_copy_longwords`: Decodes an LZ-style bit-packed stream in a loop, using a decoder call to fetch control values and either copying blocks of longwords from the source buffer in a2 to the destination in a3 in groups of eight with a tail copy, or accumulating an offset and adding a value into the destination at that offset.
- `check_pointer_and_decrunch`: Library code that compares two pointers; if they are equal, it calls a routine to decrunch a block of data.
- `copies_longword_buffer_then_frees`: Copies longwords from the source buffer in a2 to the destination buffer in a3 in blocks of eight, with a single-longword tail copy and a bit-counter adjustment, calling a sibling routine to refill the source and read or move the bit-stream state.
- `copies_longwords_between_buffers`: Advances a6 by 32 bytes and adds 8 to the count in d0, then copies longwords from the buffer at a2 to the buffer at a1, decrementing the count in d0 for each copy.
- `forwards_to_backward_bitstream_decoder`: Calls the backward bitstream unpacking routine with the displacement in the instruction, then branches unconditionally to a point in another routine in the explode library.
- `unpack_backward_bitstream_block`: Decodes a block of an LZ-style bit-packed stream read backwards from the pointer in a0, shifting bits out of a bit buffer and copying literal bytes or back-referenced bytes into the output at the destination pointer.
- `decode_bitstream_literal_or_match`: Reads variable-length codes from a bit-packed stream read backwards through the pointer in a0, using a bit buffer kept in d1 and a loop that repeats while a flag byte is non-negative.
- `set_length_and_jump_decoder`: Loads the decimal value 5 into data register d0 and the decimal value 3 into data register d7, then branches to the shared decoding code at a point just past the start of this routine's neighbour.
- `set_length_and_jump_decoder_alt`: Loads the decimal value 4 into data register d0 and the decimal value 2 into data register d7, then branches to the shared decoding code at the same target as the other entry points.
- `set_length_and_jump_decoder_short`: Loads the decimal value 3 into data register d0 and the decimal value 1 into data register d7, then branches to the shared decoding code at the same target as the other entry points.
- `decode_bitstream_literal_match`: Reads variable-length codes from a bit-packed stream read backwards through a0 and decodes each one as either a literal byte or a back-reference length and distance, using length/distance tables at offsets 12 to 36 of the structure in a4.
- `decode_packed_bits_backwards`: Shifts one bit at a time from a byte read backwards through a0 into the extended flag, and when the byte runs out it fetches the next byte, shifting bits into d2 with extend.
- `decode_lz_stream_to_buffer`: Decodes a bit-packed compressed stream read backwards from the source pointer in a0, using a 16-entry length/distance table at offsets 12 onward of the structure in a4 to choose each run.
- `unpack_bitstream_into_buffer`: Decodes a compressed bitstream read backwards from the source pointer in a0 by repeatedly shifting bits out of the bit register d1, expanding the decoded length and offset values into a copy loop that writes bytes to the destination in a3 with wraparound at the ring buffer bounds.
- `store_word_and_two_bytes`: Stores the low byte of d0 at offset 3 and the low byte of d1 at offset 2 from the address in a4, writes the word in d2 at that address, then restores registers d0-d3, d5-d6 and a3 from the stack and returns.
- `execute_callback_loop`: The routine reads a counter from a structure, increments it, and repeatedly calls another routine and a callback taken from a table indexed by a global variable until the counter condition is met, then calls the routine once more and clears a word in memory.
- `decrement_loop_counter`: The routine receives an unsigned value in an input register, converts it to signed, subtracts one, and if the result is positive, repeatedly decrements it until it reaches -1.

### Alien Storm (1991)(U. S. Gold)[cr VF].adf:VF1991

- `load_file_and_init_hardware`: Checks that at least three arguments were supplied, copies the argument string into a global variable, opens the named file through the DOS library after opening the exec library, and allocates a an address-byte buffer to read it into.
- `copy_longwords_then_dispatch`: Copies a block of longwords from the buffer pointed to by a register into a fixed low-memory area using a counted loop, then jumps to another routine at address 260 and returns zero in d0 when it resumes.

### Alien Storm (1991)(U. S. Gold)[cr VF].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_audio_stream`: Increments a counter in a structure and, if a threshold is met, copies several words and bytes from a table into a destination structure, then calls other routines in a loop.
- `update_audio_parameters`: Sets bits in the audio register based on four different word tests in a structure, copies four entries from a list to an audio register structure in a loop, performs a series of data movements in a loop, and then performs a second loop to update audio register values using a calculated offset.
- `copy_audio_data`: Copies data to an audio register structure in a loop and, if a specific bit is set in a structure, performs a second loop to copy data from a different offset within that same structure.
- `interpolate_and_update_parameters`: Calculates a weighted average of values from two source buffers to populate a destination buffer using a loop, then updates several counters and indices within a structure in a0.
- `parse_object_data_stream`: Iterates through a data stream pointed to by a pointer in a0, using command bytes to populate a structure in a0 with values or pointers from a table in a6.
- `saves_dma_state_and_clears_display`: Saves the current DMA and interrupt enable registers into two global variables with bit 15 set, then clears all DMA and interrupt enables and sets the background colour to black.
- `decrements_and_wraps_table`: Loops over 23 consecutive 16-bit values in a global variable, subtracting 6 from each and adding 360 when the result is negative, then stores a byte from a second table offset by the value plus 30 into a parallel 16-bit array before returning.
- `fills_wrapped_sequence_array`: Writes 23 consecutive 16-bit words into a global array, starting at 0 and stepping by 60 with each word, subtracting 360 from any value above 359 before storing it, then returns.
- `update_scroll_sprite_table`: Waits for the blitter to be idle, then fills two sprite-like hardware register blocks from tables and globals: it writes computed positions and fixed sizes, decrements a row table entry by 2 per row, and every eighth call advances a counter that writes a new value into the row table and wraps a global index.
- `fills_strip_of_copper_entries`: Walks a byte table starting at a fixed address, and for each entry writes a block of longword and word values into the structure in a6, using a hardware status bit (offset 2, bit 6) as a busy wait before each write.
- `fill_blitter_tile_grid`: Walks a byte table at a fixed address, and for each byte waits until the blitter-busy flag in the structure pointed to by a6 clears, then writes two 32-bit pointers at offsets 80 and 76 and two more at offsets 72 and 84, sets four 16-bit size and modulo words at offsets 96 to 102, stores a 16-bit mask at offset 64 and 66 derived from a running word, and writes the constant 962 to offset 88.
- `toggles_buffer_and_rebuilds_entries`: Flips a global toggle variable between 0 and 1, then clears one bit per entry in a list of pointer-and-index records at a fixed address, after which it rebuilds the list from a table of 16-bit offsets, setting bits in memory bytes and advancing each offset by 1440 with wraparound.
- `shifts_word_history_buffer`: Resets a global index to zero when it equals 4660, copies the word at that index into two fixed global words, and advances the index by 2.
- `toggle_and_fill_sprite_table`: Toggles a global flag and writes one of two constant pairs into two global words, then fills a table of hardware sprite/copper-style entries with values derived from a global table, looping over the entries and waiting on a status bit at offset 2 of the structure in a6 before each write.
- `fills_projected_point_table`: Loops over a 90-by-180 grid, reading words from a global variable table at stepped offsets, dividing each scaled value by a row-dependent divisor, and writing a pair of words per point to a destination table at a fixed address.
- `builds_offset_pointer_table`: Fills a table of 32-bit pointers starting at a fixed address with a running value that advances by 15840 per entry and wraps at a threshold, adding a second running offset that advances by 84 and wraps at 1424 to each entry.
- `scan_bitmap_bits_and_plot`: Walks a 64-row block of 16-bit words starting at a fixed address, testing each bit from 15 down to 0 in every row, and calls another routine for each bit that is set.
- `plots_bits_into_bitmap_rows`: Repeatedly sets a word at offset 384 of the structure in a6 to 15, computes a bit index from two byte-sized global values and two register-held coordinates, ORs a single bit into a word of the buffer addressed by a1 at that index, then writes 4095 to the same structure word and advances a1 by 2048.

### Alien Storm (1991)(U. S. Gold)[cr VF].adf:vf1#unpacked

- `run_sample_channel_update`: Modifies a word in a structure by subtracting a constant and, when the result is zero, calls one routine, otherwise another; it also increments a byte counter in a structure and, on reaching a limit, rebuilds a set of longword and byte fields from a table row into a destination structure.
- `update_audio_parameters_2`: Sets bits in the audio register based on four different word tests in a structure, copies four entries from a list to an audio register structure in a loop, performs a series of data movements in a loop, and then performs a second loop to update audio register values using a calculated offset.
- `copy_audio_data_2`: Copies data to an audio register structure in a loop and, if a specific bit is set in a structure, performs a second loop to copy data from a different offset within that same structure.
- `interpolate_and_update_parameters_2`: Calculates a weighted average of values from two source buffers to populate a destination buffer using a loop, then updates several counters and indices within a structure in a0.
- `parse_object_command_stream_2`: Reads command bytes from the stream at the pointer in a0 until a zero byte: negative commands store a byte into the structure (offset 14, 10 or 13 and 3), or copy 2-byte or 4-byte values from a table into the structure, while positive commands take a table index and copy a set of values from another table into the structure.

### Alien Storm (1991)(U. S. Gold)[cr VF].adf:a.prg (raw code file, load address unknown)

- `advance_frame_counters_and_copy`: Writes a word from a table into the area addressed by a1, then tests a byte counter at offset 16 and increments it, and when it reaches the limit in the byte at offset 18 it resets the counter and advances a second counter at offset 17.
- `copy_table_then_advance_frame`: Copies a block of 32-bit words from a source address into a destination area in a loop that counts a 16-bit counter down to minus one, then stores a word taken from an indexed table into a pointer-relative location.
- `decode_packed_tile_run_bytes`: Reads a control byte from the stream pointed to by a0 plus 328 and then, for each of its eight bits, copies a sequence of table words into the output buffer in a1, taking alternate forward and mirrored table positions depending on each bit, until the requested count of bytes (taken from the structure at a0 plus 320) is written or a negative table word ends a run.
- `conditionally_writes_masked_word_value`: Reads a byte through a pointer in a6 when the sign bit of a register is set, then tests a combined bit mask against that byte.
