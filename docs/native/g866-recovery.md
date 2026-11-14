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
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 107 64 KB pages each with >= 29491 distinct PCs executed, 1753088 of its 1783279 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 107 64 KB pages each with >= 29491 distinct PCs executed, 1753088 of its 1783279 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 107 64 KB pages each with >= 29491 distinct PCs executed, 1753088 of its 1783279 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 107 64 KB pages each with >= 29491 distinct PCs executed, 1753088 of its 1783279 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-x25k slid through memory (an address: 107 64 KB pages each with >= 29491 distinct PCs executed, 1753088 of its 1783279 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-x35k slid through memory (an address: 107 64 KB pages each with >= 29491 distinct PCs executed, 1753088 of its 1783279 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
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
- `apply_preferences_from_dialog_state`: Checks the first parameter: when it is zero it reads a preferences table through a pointer, probes several library and dialog entries to choose a value stored in a global variable, and calls the restore routine and the library-close routine to release resources.
- `initialize_preferences_window`: Opens the graphics and intuition libraries through a sequence of calls, then allocates a table of entries 44 bytes apart and fills 136 slots with pointers into it.
- `run_preference_dialog_event_loop`: Calls a routine on a global variable to check a pending state, and if that yields a nonzero result it clears bit 4 of a status global, calls a routine to fetch an entry from a table indexed by a word global, then loops over a message queue, dispatching each queued message to a handler and repeatedly calling a wait routine until a flag clears.
- `frees_global_resources_and_closes`: Calls another routine with the address of a global variable, then calls another routine with three global values, two more routines with no arguments, and one with a global value and the constant 5984.
- `calls_slot_writer_twice`: Passes the input parameter along with a constant mode of 0 and then 1 to another routine, each time supplying the same address computed from a global pointer plus 44 as the second argument.
- `writes_slot_values_by_mode_flag`: If the first argument is zero it walks three 16-bit words starting at offset 102 of the structure at the third argument and passes each to another routine with slot indices 17 to 19.
- `split_colour_word_into_nibbles`: Clears the first argument if the second argument exceeds 4, then splits the third argument into its high, middle and low 4-bit fields and passes them with the first two arguments to the library function at offset minus 288.
- `rebuild_preference_record_table`: Loops 136 times over a global pointer table, shifting each entry down by one slot and zeroing the last, then copies a 16-bit word from a global table into seven fixed offsets of each record.
- `applies_selected_preference_mode`: Stores the mode index passed as a word argument into a global variable and reads the matching entry from a global table.
- `dispatch_preference_mode_actions`: Reads a 16-bit mode value from a structure pointed to by a global pointer and runs a large switch over it, where each case stores constants or fields into global variables and calls one or more of the listed routines, including drawing, palette, date-field and config-block helpers.
- `conditionally_draws_then_redraws_fields`: Calls another routine with the first argument when the word argument is nonzero, then makes a sequence of calls with the results of other routines and global values as arguments, and returns zero.
- `compute_slider_positions_draw`: Reads several fields from the structure in the first argument and a pair of values from a structure in the library-base globals, then calls a drawing routine three times with computed coordinates, clearing the four values before calling another routine and returning zero.
- `clamp_and_apply_colour_offsets`: Clamps two signed byte arguments to fixed limits and, when the fourth argument is nonzero, stores them into a structure after limiting them against values in that structure.
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
- `allocate_and_fill_signal_entry`: Allocates a signal bit and, if one is available, allocates a 34-byte zeroed block, stores the first argument at offset 10, the second byte at offset 9 and the constant 4 at offset 8, then stores the signal number at offset 15 and the task pointer at offset 16 before registering the block through another routine; returns the block pointer or zero.
- `release_signal_entry`: Calls another routine to remove the block from a list when the long at offset 10 is non-zero, sets the byte at offset 8 to 255 and the long at offset 20 to -1, then calls two other routines with the signal byte at offset 15 and the block pointer.
- `split_seconds_into_clock_digits`: Writes the slash and colon characters into fixed display-buffer addresses and a zero terminator, divides a time value by 60 and then by 10 to produce ASCII digit characters, stores them into fixed display-buffer addresses, and returns 0.
- `convert_day_count_to_date_fields`: Walks a day counter through the month-length table with a leap-year adjustment, computes a month and day from the remaining count, then calls another routine with the adjusted year, month and day.
- `format_three_numbers_as_digits`: Takes three longword arguments, divides each by ten, and writes their tens and units digits as ASCII characters into fixed global buffer positions, then returns zero.
- `render_digit_selected_label`: Calls the same helper routine several times with constant coordinates and sizes, then copies two bytes from a global variable pair, uses the digit characters stored in a byte global to index a table of four-byte pointers, and passes the selected entry with a size of 3 to that helper.
- `compute_date_offset_and_draw`: Parses two digit fields from global byte buffers into numbers, adjusts the year and month values, and then accumulates day counts for each month by repeated conditional additions.
- `wrap_and_format_date_fields`: Wraps the day, month and year digit buffers back to their minimum values when they exceed their limits, using a switch on the month that accepts days up to 31, 30 or 28 or 29 for February.
- `fill_pattern_bytes_and_branch`: Writes four bytes of a fixed pattern through the address held in a0, using post-increment, then writes two more bytes backwards with pre-decrement, then sets d0 to 31 and branches forward into another routine that wraps the counters and formats the result.
- `load_limit_and_branch_to_wrap`: Loads the constant 30 into d0 and compares it with d3, then branches forward into the middle of another routine that increments the month counter and wraps the counters.
- `wrap_counters_and_format`: Increments the value in d2 and resets it to 1 once it exceeds 12, resets d4 to 0 when it exceeds 99, then calls the three-number formatter with d4, the adjusted d2 and the constant 1.
- `clamp_date_fields_and_format`: Raises undersized digit bytes in the date buffers to their minimum values, then limits the day to 28, 29, 30 or 31 depending on the month, and passes the clamped day, month and year to another routine.
- `normalize_date_fields_and_format`: Reads a two-digit field from the string in a0 and clamps its first digit to '9' when below '0', then forces the first digit to '2' or '3' and caps the second at '3' if it exceeds it.
- `clamp_values_and_format`: Sets d3 to 12 when it is below 1 and d4 to 99 when it is negative, then calls the three-number formatter with d4 and d3.
- `allocate_and_init_timer_structure`: Calls the allocation and signal routines to set up a 60-unit block, then loops subtracting a table of period values from a counter to pick an index, and when a flag is set it writes twelve longs into a hardware-address block and returns 1 on success, or cleans up and returns 0 on failure.
- `initializes_timer_and_signal_state`: Calls one routine with two arguments to create a signal entry and stores its result in a global variable, then calls another routine to obtain a second value and stores that in a second global variable.
- `opens_timer_device_with_arguments`: Pushes a zero longword, the value at a global pointer and two constants onto the stack, then calls a routine to open a device and tests its result.
- `updates_clock_style_display_timer`: Stores a value into a global variable, then loops, computing a timeout from a global byte and a fixed base and waiting on signal bits, checking message flags and a global counter on each pass, and repeatedly calls library vectors to refresh display state.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `creates_buffers_and_installs_task`: Calls another routine with the address of a global variable and tests the returned pointer, storing zero in the result when it is null.
- `tests_bit_then_frees_block`: Tests bit 0 of a bit mask against the bit number given by the low six bits of the byte at offset 15 of a global structure and, when that bit is clear, calls another routine on a global pointer.
- `set_request_type_and_arguments`: Reads two long arguments and a pointer from the stack, stores the word value 9 at offset 28 of the structure and the two arguments at offsets 32 and 36, then calls another routine with the structure pointer.
- `poll_counter_low_nibble_change`: Saves the low nibble of a global counter, calls another routine with the constant 100, then returns true when the low nibble of the counter has changed, otherwise false.
- `write_audio_periods_then_poll_flag`: Writes three word-pair period constants to offset 60 of a global structure unless bit 2 of byte 63 is set, ORs a constant mask into the long at offset 52, then loops up to 200 times reading bit 1 of byte 55 and returns as soon as that bit is clear.
- `clear_structure_mask_bits`: Loads a global structure pointer and clears bits 16 and 0 of the long at offset 52 of that structure, leaving the other bits unchanged, then returns.
- `read_config_block_from_file`: Opens the system configuration file, falling back to a second file record when the first lookup fails, then reads 232 bytes into the caller's buffer.
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
- `allocate_and_copy_six_buffers`: Clears a global counter, then in a loop of six iterations calls another routine to allocate a buffer sized from a table, copies that many words from a source block into it, and stores its pointer in a global array; if an allocation returns zero it returns 1, otherwise it stores six preset values into fixed global locations and returns 0.
- `call_library_with_flag`: Pushes the constant 1 and a pointer to a global variable, calls another routine with them, removes the two pushed arguments from the stack, and returns.
- `push_constants_and_branch_on_flag`: Clears d0 and loads the word from a fixed global address into its low half, keeps only bit 12 of d0, and branches on that result; the two paths push the constants 2, 3 and 73 onto the stack before branching forward into another routine.
- `call_other_with_three_args`: Pushes the constants 2 and 3 and a value computed from a global word plus 70 onto the stack, calls another routine with those three arguments, then cleans the stack and returns.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `build_sorted_name_table`: Calls the directory-scanning routines to read entries from a directory, copies each entry name into a fixed 29-byte space-padded line in a stack buffer and passes it to another routine, counting the lines produced.
- `compare_string_prefix_match`: Walks two null-terminated strings in parallel, comparing bytes until a mismatch or the end of either string is reached.
- `allocate_node_and_copy_data`: Allocates a 34-byte block through a library call, stores the previous head of a global list in its first longword and makes it the new head.
- `free_global_list_entries`: Frees a global block through a library call, clears that global, then repeatedly walks a second global list, freeing each entry with a 34-byte size until the list is empty.
- `validate_all_table_entries`: Clears a global counter, then calls another routine with each pointer from a table in turn, stopping at the first failure.
- `verify_padded_name_buffer`: Fills a 32-byte stack buffer with spaces, then copies characters from a global table into positions 1 to 28 until a zero byte is found.
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
- `call_draw_for_index_range`: Loops from the first word argument up to the second, and for each index calls another routine with a global pointer and the 32-bit entry at that index of a global table.
- `draw_sprite_range_with_mask`: Loops from the first word argument up to the second, and for each index calls another routine with a global pointer, the 32-bit entry at that index of a global table and a mask of minus one.
- `dispatch_palette_update_by_state`: Reads a state byte and, depending on its value from 0 to 8, reads colour components from a sprite or colour structure through a pointer, shifts them down to 4-bit values and passes them to another routine, or adds 3 to a cycle counter and wraps it to 0 at 3 before drawing a selection overlay.
- `iterate_indexed_table_entries`: Calls a helper with a global variable address, then walks three ranges of indexed table entries in turn, passing each entry's coordinates, text or pointer values plus fixed offsets to a drawing routine.
- `measure_zero_terminated_string_length`: Walks forward from the pointer argument until a zero byte is found and returns the byte distance between the end pointer and the start pointer.
- `draw_framed_rectangle_grid_panel`: Calls a graphics helper on a global variable, computes several coordinate pairs from the row argument (multiples of 34 with offsets), and issues a sequence of rectangle and colour-setting calls on the given rastport, then calls the helper again on the global variable.
- `draw_rectangle_outline_with_fill`: Obtains a rastport from a structure, sets a colour, and issues a series of line and rectangle drawing calls derived from the four 16-bit arguments (x, y, width, height), finishing by releasing the same rastport.
- `selects_colour_and_draws_rectangle`: Tests the word argument on the stack, setting a value of 38 when it is zero and 153 otherwise.
- `allocate_two_prefs_buffers_and_init`: Allocates a 92-byte block and a 16000-byte block through library calls, initialises byte and longword fields in the first block, and returns it, or frees the memory and returns zero if either allocation failed.
- `flush_pending_prefs_sound_buffer`: If a global pointer is set, sets bit 0 of a status byte, calls a library vector with a shifted mask, loops until the pointer clears, then frees the passed block if it is nonzero.
- `format_and_write_checked_buffer`: Calls the formatting library routine into a 140-byte stack buffer, computes the length of the resulting string by scanning for its null terminator, and passes the buffer and length to another routine when the length is non-zero.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compute_quotient_and_remainder`: Divides the 32-bit value in one register by the 32-bit divisor in another; when the divisor exceeds 65535 it runs a 16-iteration shift-and-subtract loop, otherwise it performs two 16-bit divisions and returns the quotient and remainder packed into a 64-bit result.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_division_arguments`: Saves d2, moves d1 into d2 and d0 into d1, calls the division routine, then restores d2 and returns.
- `return_division_remainder`: Saves d2, shifts d1 into d2 and d0 into d1, calls the division routine, copies d1 into d0 as the return value and restores d2.
- `divide_and_negate_by_flag`: Calls the division routine, then negates the returned value when the address register a0 holds a nonzero word.
- `divide_and_conditionally_negate`: Calls the division routine and returns the value in d1, negated when address register a0 is nonzero.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `runs_preferences_event_loop`: Draws several fields from global records, calls a library vector to set a bit mask, then repeatedly fetches messages from a port, copies a status word and selection flag, and loops until the selection value is 1 or 2, when it calls a routine and returns zero.
- `copy_preference_fields_between_records`: Copies a word at offset 2 and bytes at offsets 183, 182 and 184 from the source record to the destination record, then returns.
- `cycle_selection_index_and_redraw`: Reads a state code from the structure pointed to by a global variable and, depending on the code, increments or decrements one of two global index counters with wraparound, or updates nibbles of two global byte variables, then passes the resulting value through a routine call with arguments 3 and 2.
- `configure_library_vector_settings`: Makes five library vector calls using a base pointer from a global variable, with constants and a value looked up by a table index, stores the table index into a global variable, and returns zero.
- `update_library_vector_flags`: Makes five library vector calls with a base pointer from a global variable and different constants, then ORs a global variable with its own bits masked by 240 and returns zero.
- `loops_over_message_queue`: Repeatedly fetches a message from a port referenced through a global pointer, and for messages of type 16 walks a linked list of such messages while calling a helper on each.
- `copies_preference_block_and_refreshes`: Copies five fields from the source block to the destination at offsets 100 to 106, then copies 36 words from offset 28 onward.
- `setup_preferences_requester`: Copies three word-sized global tables into a buffer and, if the file-open routine succeeds, clears a global and builds a requester window with gadget values read from the palette and the tables.
- `dispatch_state_code_actions`: Reads a state code from a structure pointed to by a global variable and dispatches on it through a switch: codes 75 and 79 return 1 and 2 respectively, codes 76 to 78 and 83 set a global variable to 4, 5, 6 or 7 before calling another routine, and codes 80 to 82 call another routine on a global value and conditionally on the value 3 before calling it again.
- `set_three_gadget_colors_from_palette`: Returns zero when a mode byte equals 3; otherwise it looks up a 16-bit palette word indexed by the argument, extracts its three 4-bit fields into globals, and passes each to a pair of color routines on three gadget structures, returning the last result.
- `store_packed_palette_word_and_forward`: Writes a 16-bit word built from three nibble arguments combined with a fifth argument into the global palette table at an index formed from the second argument plus four, then calls another routine with the same five arguments adjusted by the index.
- `outline_selected_tile_from_bounds_table`: Reads four byte values per tile from a table indexed by the tile argument and calls a library rectangle routine four times with offsets of plus or minus one or two around those values to outline the tile.
- `clear_three_preference_slots`: Calls another routine three times with the same first argument and the value minus one, once for each of three global variables.
- `draws_robot_slot_selection_overlay`: Adjusts the third argument by adding 3 and resetting it to 0 when it equals 3, then issues a series of drawing calls at coordinates scaled from the first two arguments by 8.
- `builds_bitplane_grid_overlay`: Calls a library function through a stub using a base from a global variable, then loops over a 16 by 16 grid, deriving each cell value by combining the matching bit from two 16-bit words of a global table into a 2-bit value and passing it to another routine with the row and column indices.
- `set_bitplane_pixel_bits`: Clears one bit and then sets it again in two 16-bit words of a global table, selecting the word pair by the second argument and the bit position by the first argument, using bits 0 and 1 of the third argument as the new values.
- `draw_prefs_robot_icon_and_highlight`: Calls the drawing library function at several offsets, using a pointer loaded from a global structure, and branches on the returned value of one call to draw a second element before drawing a fixed rectangle at coordinates derived from two global variables.
- `initialize_preferences_then_dispatch`: Clears a 30-entry global array, then calls the table validation routine and copies three word values from global variables into fixed global slots before calling a copy routine three times.
- `copy_preference_fields_between_structs`: Copies a set of word fields at fixed offsets, a byte at offset 1, and a 30-byte name area starting at offset 128 from one structure to another.
- `dispatch_state_by_code`: Reads a 16-bit state code at offset 38 of a structure pointed to by a global variable and branches on it, redrawing pen-style parameters and toggling flag bits in globals per case, with a loop over 30 bytes of a global block in one case.
- `store_global_words_and_call`: Stores the constant 2 into a 16-bit global variable, pushes a 32-bit global variable onto the stack and calls another routine with it, then calls a further routine with no arguments.
- `sets_two_global_words_then_calls`: Calls one routine with no arguments, stores the constant 38 into one 16-bit global variable and the constant 1 into another, calls a second routine, and returns 0 in d0.
- `send_sequence_of_library_messages`: Calls one library function with a series of small constant codes, then with a few pointers taken from global arrays indexed by a global index, through a library base held in a global.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `store_clamped_global_byte`: Sets a global byte to one if it is zero, forces a second global byte to one when the value it compares against is below one, then copies the first byte into the second when it is within range.
- `store_value_and_refresh_first_entry`: Calls a helper with a fixed pointer, stores a byte from a global table into a global variable, and passes it to a formatting helper.
- `clamp_second_global_to_first`: Sets a global byte to one when it is zero, forces another global byte to one when the compared longword is below one, and copies the first byte into the other when the longword is smaller.
- `store_value_and_refresh_second_entry`: Calls a helper with a second fixed pointer, stores a different byte from a global table into a global variable, and passes it to a formatting helper.
- `initialize_word_and_refresh`: Compares a longword global variable against one and, if it is below one, stores the word 1 into a global variable and calls another routine.
- `copy_word_and_refresh_display_entry`: Calls a helper with a third fixed pointer, copies a word from a global table into a longword global variable, and passes it to a formatting helper.
- `draws_indexed_preference_option_markers`: Calls a sequence of routines with constants, a pointer taken from a global array indexed by a global index, and the bits of several global words shifted into small offsets, then calls one routine with the value 1 and another with a global variable and returns.
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
- `format_digits_into_decimal_buffer`: Writes decimal digit characters from a stack-based byte array into a buffer, inserting a leading zero and a decimal point when a flag bit in the structure at a6 is set, null-terminates the buffer, and returns 1.
- `draws_widgets_from_nibble_setting`: Stores the high nibble of a global bit field in a global byte, then issues a series of library vector calls with fixed coordinates and colour constants, one with a value indexed by the word argument, and finally one with a global pointer.
- `conditionally_close_windows_on_flag`: Reads a 16-bit field at offset 12 of a structure pointed to by a global table entry and tests bit 8.
- `call_two_cleanup_handlers_with_zero`: Pushes zero and two global pointers as arguments and calls a library vector twice, once for each pointer, then discards the stacked arguments and returns.
- `draw_pen_with_style_and_bounds`: Stores two byte arguments and a table entry in a global block, then calls library vectors with the pen values, a computed rectangle from a table index, and a release call before returning.
- `store_packed_colour_and_set_pen`: Packs three colour components into a word stored in a global table at the index given by the second argument minus 13, then calls a library vector with the same values.
- `apply_palette_nibbles_to_three_objects`: Reads a word from a global table indexed by the argument minus 13, splits it into three 4-bit values stored in globals, then passes fields from three pointed-to records to a helper and a library call in sequence.
- `allocate_block_with_size_48`: Saves d2, pushes the argument and the constant 48 and calls the allocation routine through the vector at address zero, then removes the arguments and restores d2.
- `forward_pointer_to_free_routine`: Saves d2, pushes the pointer argument and calls the free routine through a fixed jump vector, then removes the argument and restores d2.
- `allocate_io_request_block`: When the first argument is non-zero, allocates a zeroed block through another routine, stores the constant 5 at offset 8, the second argument as a word at offset 18 and the first argument at offset 14, then returns the block pointer or zero when the first argument is zero or the allocation fails.
- `clear_fields_and_free_block`: If the pointer argument is non-zero, it writes 255 to the byte at offset 8, writes a fixed address to the longwords at offsets 20 and 24, then calls another routine with the argument and the word at offset 18 before returning.

### Alien Storm (1991)(U. S. Gold)[cr VF].adf:explode.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `allocates_library_entry_and_stores`: Saves a5, copies the incoming a0 pointer into a field of the structure in a5, calls a library function to allocate an entry, stores the result in another field of that structure and returns; otherwise it jumps to a library vector with a constant.
- `patch_library_jump_vector`: Calls a library function at a fixed negative offset from a6, then reads a pointer from a global location and copies a 16-bit word and a 32-bit longword from it into fields of the structure at a5.
- `compares_byte_and_calls_library`: Compares the byte at the address in a0, which is then advanced by one, against the value 112, and branches forward when they match.
- `copy_byte_then_conditionally_dispatch`: Copies one byte from the source pointed to by a0 into the byte at offset 1 of the structure in a1, then compares the 16-bit word at offset 1 of the source against 24576.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `decrements_word_and_branches`: Subtracts 1 from the 16-bit word at offset 32 of the structure in a6, then branches to a later instruction if the result is nonzero.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `removes_library_node_and_closes`: Reads a value from a global structure, checks that a word there equals 20217 and that the stored pointer matches one of two handler addresses.
- `set_bit3_at_offset34`: Sets bit 3 of the byte at offset 34 of the structure addressed by a6, leaving the other bits unchanged, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (moveq) and then falls through into the code that follows, with no return of its own.
- `checks_explode_library_signature`: Calls a routine to get an index, multiplies it by 4, and compares longwords at fixed offsets from that address against the constants a fixed address, a fixed address, a fixed address, a fixed address and a fixed address.
- `decrements_timer_and_clears_cache`: Subtracts 1 from the 16-bit word at offset 32 of the structure in a6, then calls two routines.
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

- `advance_sample_playback_state`: Increments a byte counter in a global variable and, when it reaches a limit, resets it and advances a second counter; at that point it copies four longwords and four bytes from a table row into a structure whose pointer is held in a global variable.
- `update_audio_channel_registers`: Builds the DMA control word from flags in a structure pointed to by a global variable and writes it to the DMA control register, then loops over four channel records to copy volume, period and pointer fields into the audio registers.
- `copy_audio_channel_fields`: Loops over four channel records in a structure pointed to by a global variable, copying one byte and one word from each record into the audio registers for that channel.
- `update_channel_sample_state`: Reads a byte from a sample table through the structure in a0 and computes a scaled volume value into offset 21 when the active flag is set.
- `parse_object_command_stream`: Reads command bytes from a stream whose pointer is stored at the start of the structure in a0 and loop until a zero byte.
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
- `updates_audio_channel_registers`: Computes a DMA control word from flags in a structure, writes it to the custom DMACON register, then copies per-channel sample pointer, length and volume values from a structure into the four audio channel register blocks starting at AUD0LCH.
- `copies_audio_channel_state`: Loops over four audio channel register blocks starting at AUD0LCH, writing a byte from a structure into the channel's upper register and a word into its volume register.
- `interpolate_channel_state_buffer`: Computes a volume-scaled sample from one table and a blended value from another table, stores it in the structure in a0, and writes a 32-entry buffer by linearly blending two source buffers (or copying one with a cross-fade from its tail) when bit 0 of the flags byte at offset 20 is set.
- `parse_object_command_stream_2`: Reads command bytes from the stream at the pointer in a0 until a zero byte: negative commands store a byte into the structure (offset 14, 10 or 13 and 3), or copy 2-byte or 4-byte values from a table into the structure, while positive commands take a table index and copy a set of values from another table into the structure.

### Alien Storm (1991)(U. S. Gold)[cr VF].adf:a.prg (raw code file, load address unknown)

- `advance_frame_counters_and_copy`: Writes a word from a table into the area addressed by a1, then tests a byte counter at offset 16 and increments it, and when it reaches the limit in the byte at offset 18 it resets the counter and advances a second counter at offset 17.
- `copy_table_then_advance_frame`: Copies a block of 32-bit words from a source address into a destination area in a loop that counts a 16-bit counter down to minus one, then stores a word taken from an indexed table into a pointer-relative location.
- `decode_packed_tile_run_bytes`: Reads a control byte from the stream pointed to by a0 plus 328 and then, for each of its eight bits, copies a sequence of table words into the output buffer in a1, taking alternate forward and mirrored table positions depending on each bit, until the requested count of bytes (taken from the structure at a0 plus 320) is written or a negative table word ends a run.
- `conditionally_writes_masked_word_value`: Reads a byte through a pointer in a6 when the sign bit of a register is set, then tests a combined bit mask against that byte.
