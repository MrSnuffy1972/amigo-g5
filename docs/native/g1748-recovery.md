# Game 1748 (zany golf (1988)(electronic arts)[h born to win]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01748). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (919440 top-level bytes in 64 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 15 raw-word lines |
| Functions named and described | pass, 287 of 287 named (26 author symbols), 287 of 287 described (15 by rule, 196 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2988 of 2988 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 106865 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4955 of 5336 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 293093 |
| code | 16714 |
| data | 474806 |
| padding | 13995 |
| unused | 120832 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:type | 2 | 117 | 2 |
| Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:Zany | 15 | 1490 | 98 |
| Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1715 | 135 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1591 | 47 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| QUICKSCORE.UNV (raw code file, load address unknown) | 1 | 0 | 0 |
| SPRITES8.SPR (raw code file, load address unknown) | 1 | 0 | 0 |
| FILE1 (raw code file, load address unknown) | 1 | 397 | 2 |
| HOLE6.REG (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf: file c/sigfile (header 93 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:files/SIGN9.UNV: high-entropy data file (7.64 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:files/HOLE8.UNV: high-entropy data file (7.78 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:files/HOLE6.UNV: high-entropy data file (7.69 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:files/SIGN5.UNV: high-entropy data file (7.65 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:files/HOLE4.UNV: high-entropy data file (7.75 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:files/HOLE2.UNV: high-entropy data file (7.75 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:files/HOLE0.UNV: high-entropy data file (7.81 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:files/HOLE9.UNV: high-entropy data file (7.78 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:files/HOLE7.UNV: high-entropy data file (7.79 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:files/SIGN6.UNV: high-entropy data file (7.61 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:files/HOLE5.UNV: high-entropy data file (7.77 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:files/MARQUEE.UNV: high-entropy data file (7.75 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:files/HOLE3.UNV: high-entropy data file (7.77 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:files/HOLE1.UNV: high-entropy data file (7.77 bits/byte), classified as packed data
- runtime segment an address (play-long-joy): 1434 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1429 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:files/QUICKSCORE.UNV: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:files/SPRITES8.SPR: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:files/FILE1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- zany golf (1988)(electronic arts)[h born to win]/Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:files/HOLE6.REG: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying code at an address (52 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (52 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- 4 never-loaded region(s), 106865 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `handshake_disk_control_bits` (Zany): Clears one control bit and sets another in a shared control byte, then waits for the disk status input read from a CIA to change state. Kind: Amiga disk handshake via CIAB parallel port (printer port) control bits. Pass path: The routine waits on CIAB_PRB (DAT_00bfe001) status bits after toggling the printer-port control lines (DAT_00bfd100) to perform a hardware handshake with the protected disk; the check passes when the drive responds on the parallel port, al.

## Functions


### Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_context_words_then_call`: If the output pointer argument is not null, it copies sixteen 32-bit words from a fixed offset in the context block into that pointer.

### Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:Zany

- `check_original_disk_then_run`: Program entry point that opens a library and checks the disk signature with an unnamed routine, waiting on the disk status input when the original disk must be inserted and showing a message asking for it.
- `divide_signed_32bit`: Divides one signed 32-bit value by another using normalised 16-bit steps and a correction step.
- `negate_division_result`: Calls a division unnamed routine and returns its 64-bit result with both halves negated.
- `divide_unsigned_32bit_quotient`: Computes the unsigned quotient of a 32-bit dividend and divisor using normalised 16-bit steps and a single correction step, returning only the quotient.
- `divide_unsigned_quotient_remainder`: Divides an unsigned 32-bit value by another using normalised steps, returning the 16-bit quotient and the remainder packed into one result.
- `divide_32_by_16_normalised`: Divides a 32-bit dividend by the low 16 bits of the divisor after normalising the divisor, returning a 16-bit quotient and the remainder packed together.
- `multiply_32bit_words`: Multiplies two 32-bit values passed in the first two data registers using 16-bit partial products.
- `forward_string_library_call`: On first use it sets a one-byte flag and makes an initialising library call.
- `install_task_interrupt_handler`: Looks up a task through an exec library call.
- `copy_length_prefixed_string`: Copies a length-prefixed string from the source address register into a fixed global buffer.
- `copy_length_prefixed_file_name`: Performs the same length-prefixed string copy into a fixed global buffer, adds two zero bytes at the end and makes two further calls.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `poll_file_after_delay`: Clears pending interrupt requests, enables interrupts and busy-waits for about one million loop iterations.
- `set_global_flag_to_255`: Stores the value 255 into a single global byte and returns.
- `run_handler_then_enable_interrupts`: Calls a handler stored through a pointer and makes one exec library call.
- `DiskSig`: Small routine of 2 instructions that loads 16 into d0, then returns.
- `allocate_buffers_and_retry`: Allocates a 12-byte control block and a 16000-byte buffer, returning 15 if either allocation fails, and returns 17 early when an initial check fails.
- `retry_loop_after_allocation`: Allocates the same 12-byte control block and 16000-byte buffer, returning 15 if the first allocation fails, without the earlier check.
- `allocate_buffer_then_retry`: Allocates a 16000-byte buffer and frees the earlier control block before returning 15 if that allocation fails.
- `retry_loop_threshold_check`: Runs the three-slot retry loop over an already prepared buffer, giving up with status 18 after more than four errors.
- `resume_retry_loop_check`: Re-enters the same retry loop at a point after the buffers are set up, continuing the counting and error handling.
- `record_value_and_retry`: Sets a global flag to 1 and stores a returned value into the collected array, then keeps retrying with error counting.
- `classify_value_spread_status`: Compares the spread of collected values with thresholds of 50 and 300 to choose status 16 or 18.
- `select_status_from_difference`: Sets status 18 when a stored difference is below 301 and status 16 otherwise.
- `set_status_and_cleanup`: Stores status 18, frees both buffers, makes the cleanup call and returns status 18.
- `release_buffers_return_status_18`: Makes two cleanup calls, frees both buffers and returns status 18 with no other work.
- `scan_entries_until_flagged`: Steps through indexed entries, calling a lookup routine for each one until it reports a flag of 1.
- `read_one_sector_trackdisk`: Allocates an I/O request block and opens the trackdisk device, returning 1 if opening fails.
- `read_sector_into_buffer`: Allocates a 512-byte buffer and reads one sector into it through the disk request, using an unnamed routine to obtain the target address.
- `copy_block_and_set_flags`: Copies a fixed 19-byte block from a global variable into the buffer given as the first argument, then writes a bit pattern derived from the second argument into the first byte of that buffer and stores the third argument eight bytes later.
- `split_request_and_call`: Splits the low and high words of the first argument into two shared control bytes, stores the third argument into a control byte, then calls another routine with the second argument, two zero words, and a fixed device address.
- `handshake_disk_control_bits`: Clears one control bit and sets another in a shared control byte, then waits for the disk status input read from a CIA to change state.
- `poll_flag_bit_then_delay`: Sets bit 1 of the shared flag byte, then calls a routine with the request value repeatedly while bit 4 of that byte stays clear.
- `delay_15000_then_call`: Waits through a 15000-count delay routine and then calls the same routine again with the argument it received.
- `step_position_to_target`: Returns 0 at once if the position stored at offset 6 of the record already equals the requested value.
- `step_frame_position_to_target`: Performs the same side and direction handling and single-step loop as the position stepping routine, but takes its current and target values from the local frame and the record.
- `settle_then_step_frame_position`: Sets bit 2 of the shared control byte and waits for a 200-count delay.
- `step_difference_then_store`: Sets the direction bit in the shared control byte, then steps the difference between two frame values one call at a time.
- `delay_then_store_position`: Waits through a 15000-count delay, stores the requested value as a 16-bit number at offset 6 of the record, and passes the record to a called routine.
- `merge_status_bytes_into_flags`: Clears the bits of the shared flag byte that are set in the first parameter byte.
- `set_drive_control_bits`: Sets bits 3 to 6 of the shared control byte and leaves its other bits unchanged.
- `spin_quarter_count_loop`: Divides the requested count by four and calls a no-argument routine that many times as a timing loop.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `set_bit0_then_delay`: Sets bit 0 of the shared control byte, leaving its other bits unchanged, and then waits through a 3000-count delay routine.
- `start_disk_dma_and_wait`: Points the disk DMA pointer at the buffer named in the record, clears the disk-block interrupt request, and starts a disk DMA transfer with a fixed length word when bit 2 of the shared byte is set.
- `poll_disk_status_timeout`: Repeatedly reads a 16-bit field from the record and checks a status bit in a frame byte.
- `set_one_time_flag`: Checks if a byte at a specific offset within a provided pointer is zero, sets it to one if so, and then performs a tail branch to another routine.
- `set_flag_and_notify`: Reads a pointer eight bytes into a global variable, sets the flag byte just past it, then calls another routine with that pointer and with the stored value, and finally calls another routine with a fixed count.
- `clear_record_flag_and_notify_2`: Checks the flag byte at offset one of the given record and returns without doing anything if it is already clear, otherwise clears it and passes the record to another routine twice.
- `clear_record_flag_and_notify`: Clears the flag byte at offset 1 of the record and passes the record to a called routine twice.
- `FindSector`: Scans the buffer one 16-bit word at a time for the disk sync word and returns null when the buffer length is exhausted.
- `back_up_pointer_three_words`: Returns a pointer held in a frame slot moved back six bytes, which is the start of the sector header three words earlier.
- `scan_forward_for_sync_word`: Advances a scan pointer one word at a time until it finds the disk sync word, and returns null once the end of the buffer is reached.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a6, then returns.
- `encode_and_verify_track`: Checks for the sync word at the expected position in the track buffer, writes the sync preamble in front of it, and encodes the sector header and data.
- `place_sync_and_encode_track`: Writes the sync preamble four or six bytes before the first sector start, depending on whether the sync word follows immediately.
- `encode_track_fixed_sync_offset`: Writes the sync preamble six bytes before the sector start, encodes the header and data, and locates the next sector start.
- `verify_all_eleven_sectors`: Clears the result field of the record, then for each of the eleven sectors it finds the sector buffer, runs the header check, and compares a checksum of the 1024 data bytes with the stored value.
- `verify_remaining_sectors`: Advances the sector index before each check and stops after the eleventh sector.
- `return_stored_result_code`: Returns the value held in the result slot of the frame without changing anything.
- `SecBase`: Walks from the record current sector index toward the requested sector, moving the buffer pointer forward one sector slot at each step.
- `advance_sector_buffer_pointer`: Repeats the sector-slot step until the index counter reaches the requested sector, moving the buffer pointer forward one slot each time.
- `check_sector_header_fields`: Checks the sync word and the header checksum of a sector, then compares its marker byte, track, sector number and sector count with the expected values.
- `check_sector_header_from_frame`: Performs the same sync, checksum and header field checks as the header check routine, reading the sector buffer and the expected header from the caller frame.
- `check_header_fields_only`: Compares the marker byte, track, sector number and sector count of a header with the expected values, without checking the sync word or the checksum.
- `check_track_sector_and_count`: Compares the track, sector number and sector count of a header with the expected values.
- `check_sector_number_and_count`: Checks that the sector number is below eleven and that the sector count is nonzero and below twelve.
- `check_sector_count_range`: Returns 0 when the sector count byte of the header is nonzero and below twelve, and code 8 otherwise.
- `stub_return_zero_status`: Only returns 0 without touching any state.
- `checksum_even_bits_of_longs`: Computes a checksum by XORing together the long words of a buffer, with the byte length given by the caller, and keeps only the even-numbered bits of the result.
- `mask_frame_value_even_bits`: Returns a value from the caller frame with only its even-numbered bits kept and all other bits cleared.
- `decode_interleaved_bit_longs`: Merges two long words that each hold alternate bits of a value.
- `_AllocMem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `_FreeMem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `_FindTask`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `_AllocSignal`: Library stub: loads its arguments and the exec library base, calls AllocSignal and returns its result to the caller.
- `_FreeSignal`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `_AddPort`: Library stub: loads its arguments and the exec library base, calls AddPort and returns its result to the caller.
- `_RemPort`: Library stub: loads its arguments and the exec library base, calls RemPort and returns its result to the caller.
- `_OpenDevice`: Library stub: loads its arguments and the exec library base, calls OpenDevice and returns its result to the caller.
- `_DoIO`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `CreateStdIO`: Creates an Exec message port through the system library call, stores the port pointer in the record, and sets the record size field to 56.
- `DeleteStdIO`: Deletes the Exec message port stored in the record, frees the 56-byte memory block that holds the record, and returns 0.
- `ReadSecs`: Fills in a disk request to read a number of sectors from a given sector offset into a buffer, with byte offset and length derived from the sector numbers, then performs the request and returns.
- `WriteSecs`: Fills in a disk request to write a number of sectors from a buffer at a given sector offset, with byte offset and length derived from the sector numbers, then performs the request and returns.
- `FormatSecs`: Initialises a block of fields within a structure, including a size value and two shifted values, then calls another routine to open a disk request.
- `close_request_from_frame_2`: Invokes another routine and returns its result when that routine reports a nonzero status.
- `close_request_from_frame`: Closes the disk request whose handle is stored in the caller's frame by issuing it through the I/O manager.
- `UpdateTD`: Sends the update command with zero offset and length, which flushes buffered disk writes, then performs the request.
- `MotorOff`: Fills in a disk request with the motor command and zero length and offset, then performs it, which turns the drive motor off.
- `DiskStatus`: Sends a disk request with the given command code and zero offset and length, performs it, and returns the actual-count result stored in the request block.
- `_CreatePort`: Allocates a signal bit and a cleared 34-byte message port structure through the Exec library.
- `_L8`: Continues port allocation after a signal bit has been obtained: it allocates the 34-byte port structure, frees the signal if that fails, and otherwise fills in the port fields.
- `_L7`: Stores the name, priority, type and owning task into an already allocated port structure.
- `_L6`: Initialises the message list of the port structure passed in by calling the empty-list setup routine on its list field.
- `_DeletePort`: Removes the port from the system list when it has a name, marks its type field as invalid, frees its signal bit, and returns its 34-byte memory block.
- `_NewList`: Sets up an empty Exec list header at the address passed in: the head points to the tail slot, the tail is cleared, and the tail predecessor points back to the head.

### Zany Golf (1988)(Electronic Arts)[h Born to Win].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_execute_function`: Copies a value to a global variable and then executes a function pointer located at a specific address.
- `update_function_pointer`: Writes the address of the current routine to a global variable and then executes a function pointer located at a certain offset.
- `execute_indirect_call`: Performs an indirect call to an address constructed from a register and a pointer.
- `execute_indirect_function`: loads a 32-bit address from a pointer-based memory location and jumps to that address.
- `execute_indirect_call_from_table`: Performs an indirect call to an address retrieved from a table.
- `multiply_and_call`: Multiplies a byte at a pointer location by two and then performs an indirect call to a new address.
- `store_and_call`: Stores two bytes from a pointer into a destination or a table and then performs an indirect call.
- `execute_indirect_function_2`: loads a word-sized or long-sized address from a pointer and jumps to that address.
- `execute_indirect_call_2`: constructs a 32-bit address from a register and a memory-resident halfword, then jumps to that address.
- `execute_indirect_function_3`: loads a 32-bit address from a memory location and jumps to that address.
- `multiply_at_offset_and_call`: Multiplies a byte at an offset pointer location by two and then performs an indirect call.
- `execute_indirect_function_4`: constructs a new function pointer from two bytes and calls it.
- `call_indirect_offset_two`: Performs an indirect call to an address or a location offset by two words from a pointer.
- `call_indirect_pointer`: Performs an indirect call using an address stored at a pointer location.
- `shift_left_and_call`: Shifts the byte at a pointer location left by one bit, setting the low bit from the carry flag, and then performs an indirect call.
- `shift_left_with_offset_and_call`: Shifts the byte at a pointer location offset by a signed value left by one bit, setting the low bit from the carry flag, and then performs an unable indirect call.
- `execute_indirect_function_5`: loads a word-sized or long-sized address from a pointer and branches to that address.
- `execute_indirect_call_3`: Constructs a 32-bit address from a register and a memory-resident word, then jumps to that address.
- `execute_indirect_subroutine`: constructs a 32-bit address from two registers and jumps to that address.
- `execute_indirect_function_6`: constructs a 32-bit function pointer from a high-word and a low-word stored in a memory location, then calls that address.
- `shift_right_and_call`: Shifts the byte at a pointer location right by one bit and then performs an indirect call.
- `shift_right_with_offset_and_call`: Shifts the byte at a pointer or offset location right by one bit and then performs an indirect call.
- `update_and_call_subroutine`: Calculates an address using a pointer and an offset, stores two bytes from that address into a destination, and calls the resulting address.
- `load_and_call_function`: Loads a 32-bit value from a pointer-based address, stores the lower two bytes into a memory location, and then executes the resulting address as a function call.
- `execute_conditional_indirect_call`: Checks a byte value and then performs an indirect call to an address retrieved from a table or a pointer-based location.
- `branch_on_non_negative`: Checks if a byte is non-negative and then performs an indirect call using a pointer or an offset from a global variable.
- `branch_on_non_zero`: Checks if a byte is non-zero and then performs an indirect call using a pointer or an offset from a global variable.
- `branch_on_zero`: Checks if a byte is zero and then performs an indirect call using a pointer or an offset from a global variable.
- `branch_on_negative`: Checks if a byte is negative and then performs an indirect call using a pointer or an offset from a global variable.
- `branch_on_positive`: Checks if a byte is greater than negative one and then performs an indirect call using a pointer or an offset from a global variable.
- `execute_indirect_subroutine_2`: Constructs a 32-bit address from two registers and jumps to that address.
- `update_and_call_subroutine_2`: Calculates an address using a pointer, stores two bytes from that address into a destination, and calls the resulting address.
- `call_subroutine_from_table`: Constructs an address from a register and a pointer-based value to perform an indirect call.
- `load_and_call_indirect_function`: Loads a 32-bit value from a memory location, stores two bytes of that value into a destination address, and then executes a function call using the remaining part of that value as a pointer.
- `call_subroutine_from_pointer`: Constructs an address using a register and a value from a pointer to perform an indirect call.
- `call_subroutine_from_pointer_2`: Constructs anaddress using a register and a value from a pointer to perform an indirect call.
- `execute_indirect_function_7`: loads a 32-bit or 16-bit address from a pointer and jumps to that address.
- `execute_indirect_function_8`: loads a function pointer from a memory location and transfers control to that address.
- `execute_indirect_call_4`: Performs an indirect call to an address constructed from a pointer and a high-word value.
- `execute_indirect_function_9`: loads a word-sized or long-sized address from a pointer-based memory location and jumps to that address.
- `execute_indirect_subroutine_3`: loads a 32-bit address from a memory location and jumps to that address.
- `execute_indirect_function_10`: loads a function pointer from a memory location and jumps to it.
- `execute_indirect_call_5`: Loads a word from a memory location into a register and performs an indirect call to a destination address constructed from that value and a previously held register value.
- `execute_indirect_call_6`: Performs an indirect call to anaddress constructed from a pointer and a high-word value.
- `execute_indirect_subroutine_4`: loads a word-sized or long-sized address from the stack and jumps to that address.
- `execute_indirect_call_7`: Performs an indirect call to an address loaded from an offset of a pointer.
- `execute_indirect_function_11`: loads a word-sized pointer from a memory location and executes the routine located at that address using a value or part of a value previously held in a register.
- `execute_swapped_indirect_call`: Performs an indirect call to an address where the target pointer's low-word has been byte-swapped.
- `jump_to_indirect_address`: Performs an indirect jump to an address constructed from a register and a pointer-based offset.
- `shift_and_store_byte`: Shifts a byte right and inserts a carry bit into the high bit before storing it at a pointer location, then jumps to another routine.
- `store_two_bytes_and_jump`: Stores two consecutive bytes from a pointer-based source into a destination, then jumps to another routine.
- `execute_indirect_subroutine_5`: Loads a 32-bit address from a memory location and jumps to that address.
- `execute_indirect_function_12`: Loads a 32-bit function pointer from a memory location and jumps to it.
- `update_and_call_pointer`: Updates a destination buffer with bytes from a source pointer and then performs an indirect call.
- `store_byte_at_calculated_index`: Calculates a destination address using a pointer and an offset, stores a byte there, and then jumps to another routine.
- `store_and_branch_indirect`: stores a byte from a register into a memory location calculated via a pointer and an offset, then performs an indirect call to a destination address located in a table.
- `jump_to_pointer_address`: Performs an indirect jump to an address stored at a pointer location.
- `execute_indirect_function_13`: loads a word-sized or long-sized pointer from a memory location and jumps to that address.
- `execute_indirect_function_14`: loads a word-sized pointer from a memory location and executes the code at that address using a value or part of a value previously held in a register.
- `execute_indirect_call_8`: Performs an indirect call to a location stored in a pointer.
- `execute_indirect_function_15`: loads a function pointer from a memory location and jumps to it, using a value or part of a value from a register to determine the target address.
- `execute_indirect_function_16`: loads a word-sized pointer from a memory location and executes the routine located at that address.
- `store_byte_and_call`: Stores a byte into a memory location and then performs an indirect call.
- `store_and_branch_to_subroutine`: stores a byte from a register into a memory location determined by a pointer and then calls a subroutine located at a pointer-indexed address.
- `store_and_branch_indirect_2`: stores a byte into a memory location determined by a high-word and a pointer, then performs an indirect call to a destination or a value from a table.
- `update_memory_and_call`: Updates multiple bytes in a memory area using a pointer and then performs an indirect call.
- `store_byte_with_offset_and_call`: Stores a byte into a memory location using an offset and then performs an indirect call.
- `update_function_pointer_and_branch`: Calculates a new address using a register and a pointer, stores a byte at that calculated memory location, and then executes the routine located at the new address.
- `update_pointer_and_branch`: calculates a new pointer or index using a register and a high-order byte, stores a byte to that calculated memory location, and then calls a routine located at an offset from a table or pointer.
- `clear_byte_and_call`: Clears a byte at a memory location and then calls another routine using an address from a table.
- `clear_byte_with_offset_and_call`: Clears a byte at a memory location calculated using an offset and then calls another routine using an address from a table.
- `call_from_table`: Loads a routine address from a table and calls it.
- `copy_and_call_table`: Copies two bytes from a source pointer into a destination buffer and then calls a routine from a table.
- `execute_indirect_subroutine_6`: loads a 32-bit address constructed from two registers and jumps to that address.
- `execute_indirect_call_9`: constructs a 32-bit address from a high-order word in one register and a low-order word from a memory location, then jumps to that address.
- `execute_indirect_call_10`: constructs a pointer from two partial register values and executes a call to that address.
- `execute_indirect_call_11`: constructs a 32-bit address from two registers and jumps to that location.
- `execute_indirect_call_12`: Constructs a 32-bit address from two registers and jumps to that address.
- `execute_indirect_call_13`: Constructs a 32-bit address from two registers and jumps to that address.
- `execute_indirect_function_17`: constructs a 32-bit address from two separate memory or register-based components and jumps to that address.
- `store_and_call_indirect`: Stores two bytes from a pointer-derived source into a destination and then calls a routine using a reconstructed address.
- `call_indirect_reconstructed`: Performs an indirect call using a reconstructed address from a register and a pointer.
- `call_indirect_from_table`: Performs an indirect call using an address loaded from a table offset.
- `execute_indirect_function_18`: loads a 32-bit address from a memory location and jumps to it.
- `execute_indirect_subroutine_7`: loads a 32-bit address from a memory location and jumps to that address.
- `execute_indirect_function_19`: constructs a function pointer from a high-word and a low-word stored in a memory location and calls it.
- `execute_indirect_function_20`: constructs a 32-bit address from two registers and jumps to that address to call another routine.
- `store_and_call_with_offset`: Calculates a pointer using an offset, stores two bytes from that pointer to a destination, and calls a routine using a reconstructed address.
- `execute_indirect_function_21`: loads a word-sized pointer from a memory location and executes the code at that address, using a high-order byte or a separate register to form a full 32-bit or 24-bit target address.
- `execute_indirect_function_22`: loads a word-sized pointer from a memory location and executes the code at that address, using a high-order word or a separate register to form a full 32-bit or 24-bit target address.
- `execute_indirect_call_14`: Performs an indirect call to a routine using an address stored in a pointer.
- `execute_indirect_function_23`: loads a word from a pointer and executes the resulting address as a function call.
- `execute_indirect_function_24`: loads a 32-bit address from a pointer-to-pointer and branches to that address.
- `execute_indirect_function_25`: loads a word-sized or long-sized address from a pointer and branches to that location.
- `call_from_offset_pointer`: Performs an indirect call to a routine using an address located two bytes after a pointer.
- `execute_indirect_call_15`: constructs a 32-bit address from two registers and jumps to that address to execute another routine.
- `execute_indirect_call_16`: loads a 32-bit address from a memory location and jumps to that address.
- `decrement_and_call`: Decrements a byte at a pointer location and then performs an indirect call to a routine using an address stored in a pointer.
- `store_bytes_and_call`: Stores two bytes from a pointer location into a destination and then performs an indirect call to a routine using an address located two bytes after a pointer.
- `execute_indirect_call_17`: constructs a 32-bit address from two 16-bit fragments and jumps to that location.
- `jump_via_register_offset`: Performs an indirect jump to an address calculated by combining a high-word register value with a word-sized offset from a pointer.
- `decrement_and_jump`: Decrements a byte at a calculated pointer address and then performs an indirect jump to a new address.
- `jump_via_register_offset_2`: Calculates a jump target using a high-word register value and an offset from a pointer, then jumps.
- `store_bytes_and_jump`: Stores two bytes from a source pointer into a destination buffer and then performs an indirect jump.
- `store_bytes_and_jump_3`: stores two bytes from a source pointer into a destination buffer and then performs an indirect jump.
- `execute_indirect_call_20`: Performs an indirect call using a pointer or an offset from a global variable.
- `execute_indirect_jump`: Performs an indirect jump using a pointer or an offset from a global variable.
- `jump_via_register_offset_3`: Executes an indirect jump using a target address derived from a register and a pointer-based offset.
- `jump_via_register_offset_4`: Performs an indirect jump to a location determined by a register-based high-word and a pointer-based offset.
- `jump_via_register_offset_5`: Calculates a jump destination using a register-derived high-word and an offset from a pointer, then jumps.
- `increment_and_jump`: Increments a byte at a calculated pointer address and then performs an indirect jump to a new address.
- `store_bytes_and_jump_2`: Copies two bytes from a source pointer to a destination buffer and then performs an indirect jump.
- `jump_via_register_offset_6`: Performs an indirect jump to an address constructed from a register-based high-word and a pointer-based offset.
- `execute_indirect_call_18`: Performs an indirect call to a destination address constructed from a register and a pointer.
- `increment_and_call`: Increments a byte at a calculated memory location and then performs an indirect call to another routine.
- `copy_word_and_call`: Copies a word from a source location to a destination location and then performs an indirect call to another routine.
- `copy_bytes_and_call`: Copies a certain number of bytes from a source to a destination and then performs an indirect call to another routine.
- `execute_indirect_call_simple`: Performs an indirect call using a destination address derived from a register and a pointer.
- `copy_block_and_call`: Copies a block of four bytes from a source to a destination and then performs an indirect call to another routine.
- `copy_and_call_word`: Copies a word from a calculated memory location to a destination and then executes a function pointer located at a certain offset in a global structure.
- `execute_indirect_call_19`: Loads a function pointer from a table or memory location using an offset and executes it.
- `reset_hardware_and_globals`: Clears several global memory locations, sets a global variable to a specific value, and calls another routine.
- `xor_memory_block`: Iterates through a block of memory, XORing each byte with a constant, before calling another routine.
- `update_bitmask_from_source_data`: This routine reads a source buffer and, based on the state of specific bits in the source bytes, sets or clears corresponding bits in several destination bitmask buffers.
- `update_sprite_attribute_buffers`: Calculates new memory offsets and copies a block of attribute data into multiple interleaved destination buffers.
- `copy_memory_block`: Uses a value from a global variable to determine a stride and copies a block of data from a source pointer to several destination pointers in a loop.
- `update_table_with_swapped_data`: Calculates a source address based on a base pointer and an offset, then copies bytes from that source to a destination, followed by a secondary calculation involving byte-swapped or rotated words to update a series of bytes in a global structure before calling an indirect function pointer.
- `swap_and_extend_word`: takes a 32-bit value, extracts the upper 16 bits, and returns a new 32-bit value where the lower 16 bits are a byte-swapped version of the original lower 16 bits, and the upper 16 bits are a byte-swapped version of the original upper 16 bits.
- `update_bitmask_from_table`: Iterates through a data table to calculate a pointer and bit-offset, then updates specific bits in a series of memory buffers or bitmasks.
- `swap_word_to_memory`: Swaps the bytes of a word from a global variable and stores the result back into a global variable.
- `update_bitmask_graphics`: Iterates through a command buffer to clear and set specific bits in a bitset or bitmask spread across multiple memory planes.
- `copy_block_with_scaling`: Calculates a destination address using a scaled offset and copies a block of memory from a source range to a destination range.
- `update_mouse_position`: stores a value from the mouse port through a pointer and calls another routine.
- `update_input_state`: updates several global variables or table entries based on button or movement values and calls another routine.
- `process_keyboard_input`: a routine that interacts with the keyboard serial register and certain control registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_input_table`: clears a range of a table or buffer and updates a global variable or a table entry with a value derived from an input byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_input_state_and_table`: Reads a bit-manipulated value from the keyboard serial register and uses it as an index to clear or set bytes in a memory table.
- `clear_table_bits`: clears the high bit of four or more consecutive word-sized entries in a table and calls another routine.
- `swap_table_endianness`: iterates through a table and swaps the byte order of several word or long-sized entries if certain conditions are met.
- `get_swapped_word`: returns a 32-bit value constructed from two 16-bit values that have had their byte order swapped.
- `populate_word_tables`: Iterates through a source buffer, byte-swaps any non-zero-filled words, and writes them to two different destination tables.
- `update_word_buffer`: Iterates through a source buffer, byte-swaps any non-zero-filled words, and writes them to a single destination buffer.
- `sync_table_values`: copies values from a source table to a destination table and sets a hardware register and a global variable.
- `copy_table_to_source`: copies non-zero values from a table to a source location and sets a hardware register.
- `update_mouse_delta_and_counters`: Calculates the difference between the current mouse position and a previous position, updating a running-sum or accumulator.
- `store_loop_result`: Repeatedly calls another routine and stores a returned byte into a global variable until a certain value is returned.
- `execute_and_store_byte`: Calls another routine and stores a returned byte into a global variable, continuing until a certain value is returned.
- `do_nothing`: the routine is empty and performs no operations before returning.
- `initialize_and_loop_buffer`: Clears a global word, calls a routine, and executes a loop that decrements a counter before calling another routine via a pointer.
- `check_and_decrement_loop`: Checks if a global byte is zero, sets it to a certain value, and then executes a loop that decrements a counter before calling a routine via a pointer.
- `update_counters_and_clear_memory`: Updates a set of four counters using a value derived from a pointer, clears a block of memory, and calls another routine.
- `clear_memory_and_call`: Clears a large block of memory, writes a byte to a global variable, and calls another routine.
- `set_variable_and_call`: Writes a byte to a global variable and calls another routine.
- `update_transform_matrix`: Calculates and stores a new transformation matrix or coordinate-related lookup table by performing bitwise-heavy multiplication-like operations on values read from a source table.
- `sync_and_set_global`: Copies a range of bytes from a source pointer to a destination pointer, null-terminates the destination, and updates a global variable with a byte-swapped value derived from a function return.
- `modify_bitmask_buffer`: Iterates through a buffer or a series of buffers, clearing and setting specific bits within a bitmask or a bit-plane-like structure based on a bit-masking or bit-setting pattern.
- `update_scaled_value`: calculates a product of a byte and a byte-swapped word, then stores the result as a 3-byte little-endian value in a memory location, and finally calls a function pointer located at an offset from a base pointer.
- `update_sprite_index_and_position`: Updates a table of sprite or object properties by checking a priority or index-based condition and writing new coordinate or attribute data.
- `update_audio_and_counters`: Toggles a byte in memory and updates a list of counters or values while modifying an audio volume register.
- `swap_and_rotate_registers`: Manipulates high and low register halves and performs a byte-swapping or rotation-like operation on a register.
- `clear_memory_tables`: Zeros out two memory-resident tables and sets a specific memory location to a fixed value.
- `search_and_update_collision_table`: Searches backward through a memory-resident table for an empty or invalid entry, then updates a pointer-based table with a new value and a modified flag.
- `initialize_memory_buffers`: Writes several values to a global structure, sets certain pointers to a specific memory range, and clears a large block of memory before calling a function pointer.
- `update_path_buffer_and_rebuild_list`: Updates a memory-resident buffer with a new value and then rewrites a list or table by copying values from a source table to a destination table, potentially clearing or resetting certain counters or pointers in the process.
- `decrement_conditional_counters`: Checks a value in memory and, if positive, toggles a byte and modifies two different counters by a certain offset.
- `check_and_clear_table_entries`: Compares a value against a table and clears certain memory entries if a match or certain condition is met, then calls another routine.
- `update_sprite_attribute_list`: Iterates backwards through a list of active or previously active objects, updating or clearing certain sprite-related or object-related attributes and clearing a set of associated data fields.
- `update_object_properties_and_dispatch`: Iterates through a list of objects to update a coordinate or property value, checks for overflow, and dispatches a function call based on a value read from a table.
- `copy_if_not_zero`: Checks if a byte in a global variable is non-zero and, if so, copies it to another global variable location.
- `update_minimum_value`: Compares a value derived from a table with a global variable and updates that global variable if the table value is smaller.
- `shift_and_return_byte`: Calls a routine to update a minimum value and returns a byte from a global variable or table-derived value left-shifted by one.
- `execute_conditional_function`: Checks a bit in a global variable and, if certain conditions or certain values in a table are met, executes a function using a value from a table and a modified or or a pointer-derived value.
- `calculate_shifted_value`: Returns a value composed of a bit-shifted version of a byte from a global variable and a provided parameter.
- `update_division_remainder`: Performs a division-based calculation using a table and a global variable, then updates a global variable with the quotient and remainder-based logic.
- `update_table_and_branch`: Copies values from a table to a global, then performs a loop that checks a condition and updates a global pointer or value based on table lookups.
- `check_and_reset_flag`: Checks if a value derived from a global variable matches a byte in another global variable, and if so, resets that byte to zero.
- `update_flag_on_bit_set`: Checks if a specific bit is set in a global variable and, if so, writes an incremented value to a global variable.
- `return_empty_routine`: The routine is empty and performs no operations before returning.
- `return_from_call`: The routine is empty and performs no operations before returning to its caller.
- `return_empty_routine_2`: The routine performs no operations and immediately returns to its caller.
- `populate_data_table_from_buffer`: Calculates a series of destination addresses and copies data or pointers from a source buffer into a global table.
- `increment_global_counters`: Updates a global pointer or value while checking a condition, and increments a series of bytes in a global structure if certain overflow conditions or certain values are met.
- `calculate_and_store_addresses`: Iterates through a loop, calculating and storing byte-swapped or modified addresses or pointers into a global table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_return_constant_16`: Only returns the constant 16 with no other effect.

### FILE1 (raw code file, load address unknown)

- `store_context_then_indirect_call`: Copies a stack argument into a global variable and stores two register values into memory.
- `configure_audio_channels_and_dma`: The decompiled body is only an indirect call through a table entry.
