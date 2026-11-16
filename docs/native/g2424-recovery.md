# Game 2424 (caesar deluxe (1993)(impressions)[cr atx][a][fd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02424). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1809648 top-level bytes in 110 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 30 of 30 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 588 of 588 named (1 author symbols), 588 of 588 described (142 by rule, 214 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1563 of 1563 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11400 of 11400 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 526849 |
| code | 29168 |
| data | 295083 |
| padding | 32340 |
| unused | 926208 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:installto | 2 | 117 | 2 |
| ram-handler | 2 | 10 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:assign | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:echo | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:Anthrox | 6 | 250 | 5 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:cd | 2 | 294 | 5 |
| installto | 2 | 117 | 2 |
| assign | 2 | 117 | 2 |
| echo | 2 | 117 | 2 |
| cd | 2 | 294 | 5 |
| install.exe | 3 | 1540 | 115 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:INSTALL.EXE | 3 | 1581 | 115 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:trainer | 3 | 182 | 9 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:play | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:assign | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:echo | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:lab | 1 | 2 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:skip | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:csr.exe | 8 | 228 | 17 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:failat | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:Anthrox | 6 | 250 | 17 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:cd | 2 | 294 | 5 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:endif | 1 | 2 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:caesar.boot | 1 | 31 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:if | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:bootblock | 1 | 10 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1670 | 47 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:Anthrox#unpacked | 2 | 1670 | 109 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:Anthrox#unpacked | 2 | 1670 | 109 |

## Cases handled

- runtime segment an address: 1314 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- caesar deluxe (1993)(impressions)[cr atx][a](disk 1 of 2)[fd]/Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:Anthrox: crunched executable decrunched by XFDMaster (Imploder (xfd), 12208 -> 25784 bytes), 10 of 11 windows confirmed in reference RAM: recovered as a hunk executable
- caesar deluxe (1993)(impressions)[cr atx][a](disk 2 of 2)[fd]/Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:csr.exe: Imploder (xfd) stream found by ancient scan (113624 -> 261180 bytes) but only 0 of 114 sampled windows are in reference RAM: not used
- caesar deluxe (1993)(impressions)[cr atx][a](disk 2 of 2)[fd]/Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:Anthrox: crunched executable decrunched by XFDMaster (Imploder (xfd), 12208 -> 25784 bytes), 10 of 11 windows confirmed in reference RAM: recovered as a hunk executable
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 address
- caesar deluxe (1993)(impressions)[cr atx][a](disk 2 of 2)[fd]/Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:csr.exe: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:trainer

## Copy protection found

- `initialise_custom_chip_state` (install.exe): Writes display window, bitplane modulo, DMA and interrupt control registers to set up the custom chips, saves and restores the previous interrupt and sprite state through library calls, and calls dos.Exit on the exit path. Kind: loader/protection sequence. Pass path: u10__l0_020C.
- `setup_display_and_exit` (INSTALL.EXE): Programs the display window, bitplane modulo, DMA, interrupt and sprite registers after saving the system state, clears shared variables and runs a timed delay. Kind: disk/track check loop. Pass path: the branch where _u11__d0_24F6 == 0xd is taken (breaking the loop to proceed with installation).

## Functions


### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:installto

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If the destination pointer is non-null, copies sixteen 32-bit words from a block at a fixed offset into it.

### ram-handler

- `call_global_then_unrecovered_jump`: Calls a function pointer read from a global variable at offset -624, then jumps through a pointer whose target is not recovered by the decompiler.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:assign

- `prepare_sized_table_block`: Finds the largest value in a linked list, uses it to size a table and fills the table with an increasing sequence of 32-bit words.
- `copy_memory_block_6`: Transfers a fixed number of words from a source offset to a destination address.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:echo

- `find_max_and_update_list`: Traverses a linked list to identify the maximum value in a specific field and then updates a header structure.
- `copy_memory_block_8`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:Anthrox

- `cleanup_cached_block_entry`: Calls an unnamed routine, then makes four indirect library calls in a loop.
- `decrement_word_until_minus_one`: Decrements a 16-bit local counter until it reaches minus one, then returns.
- `clear_cache_after_bit_pass`: Runs two passes that shift bits from an input stream and add the resulting values into a buffer through unnamed routines.
- `decode_backward_bitstream`: Decodes a bit-packed stream by reading bits from a stored byte and copying run-length groups backwards from the input into the output buffer.
- `decode_backward_bitstream_run`: A second variant of the backward bit-stream decoder that uses the same backward copy loop and bit-reader state, with its run lengths read through unnamed routines.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:cd

- `update_linked_list_and_copy_data_6`: Traverses a linked list to find the maximum value in a specific field and then updates a structure with this value.
- `copy_memory_block_and_call_2`: Copies sixteen bytes from a memory location to a destination pointed to by a stack variable using a loop.
- `update_callback_state_and_structure`: Performs a bitwise XOR on a byte at a pointer, initializes several offsets within a structure, and executes a sequence of callback routines that conditionally update values at offset 600 and other locations within the structure based on table lookups.
- `execute_callback_sequence`: Calls a routine through a pointer in a5 multiple times with different parameters in a4 and d0, and updates a longword at offset 600 of a structure based on the results.
- `execute_sequence_of_calls`: Performs a series of calls through a pointer in a5 using various parameters from a2 and a1, updates a longword at offset 600 of a structure in a2, and ends with an indirect jump to the address in a6.

### installto

- `traverse_list_and_update_buffer`: Traverses a linked list to find the maximum value in a specific field and then updates a header and data block.
- `copy_memory_block_and_call_3`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer, then executes a function pointer.

### assign

- `find_max_and_populate_buffer_2`: Traverses a linked list to find the maximum value in a specific field and then writes a header and a sequence of incrementing values into a destination buffer.
- `copy_data_block_and_call`: Copies sixteen bytes of data from a memory location to a destination pointed to by a stack pointer, then calls another routine.

### echo

- `update_max_and_sync_buffer`: Traverses a linked list to find the maximum value in a specific field and writes it to a destination structure.
- `copy_data_block`: Copies sixteen bytes of data from a memory location to a destination pointed to by a stack pointer.

### cd

- `traverse_list_and_update_buffer_2`: Iterates through a linked list to find the maximum value in a specific field, then updates a header and performs a series of memory copies between a source block and a destination buffer based on the calculated maximum.
- `copy_stack_data`: Copies sixteen words from a source pointer to a destination pointer on the stack.
- `update_state_via_callbacks`: Modifies a bit in a byte using a bitmask, updates several pointers and offsets within a structure, and executes a sequence of callback routines through a pointer in a5 based on conditional checks of table values and structure members, finally updating a longword at offset 600 of the structure.
- `execute_parameterized_calls_2`: Calls a routine via a pointer in a5 using a value from a structure at offset 292 of a2, then calls another routine via a pointer at offset 336 of a2.
- `update_game_state_and_dispatch`: Checks a value from a structure via a pointer, updates a field at offset 440 of a structure with a result from a subroutine, and conditionally performs several subroutine calls and data movements based on values in a table before jumping to an indirect address.

### install.exe

- `run_installer_entry_wait_key`: Program entry for the installer: it loads display and DMA control registers, clears a block of state, runs a timed delay, then waits for a keypress.
- `initialise_custom_chip_state`: Writes display window, bitplane modulo, DMA and interrupt control registers to set up the custom chips, saves and restores the previous interrupt and sprite state through library calls, and calls dos.Exit on the exit path.
- `append_fixed_table_record`: Copies a 48-byte record from a fixed data block into the global table at the current index and increments the entry count, but only when a stored value is below one.
- `process_first_flagged_entry`: Walks the table of entries from the start, calls an unnamed routine on each to check its state, and stops at the first one whose check returns one.
- `lock_install_directory`: Calls the DOS routines to lock the install directory and make it the current directory, then runs the same slot transfer step.
- `create_caesar_directory`: Creates the caesar directory through the DOS library and then releases the lock that it returned.
- `handle_text_entry_key_3`: Stores a value in a buffer and increments an index, or clears the previous entry and decrements the index if a specific value is provided.
- `set_default_layout_values`: Stores the values 60, 70, 8 and 4 into four stored fields and sets a value of 200, then calls an unnamed routine.
- `run_seven_routine_sequence`: Calls seven unnamed routines in sequence and returns, with no other operations.
- `run_eight_routine_sequence`: Calls eight unnamed routines in sequence and returns, with no other operations.
- `execute_setup_sequence`: Writes 777 to a hardware register, processes a zero-terminated entry list using values from a table, formats two decimal values via two calls, waits for a flag, and calls another routine six times.
- `initialize_hardware_state`: Clears a hardware register, sets three memory locations, copies a value from one location to another, calls another routine, and returns if a specific register bit is not set.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `run_seven_setup_steps`: Calls seven routines in a fixed order with no logic of its own.
- `execute_sequence_of_routines`: Calls a series of seven routines in a fixed order before returning.
- `wait_for_return_key`: Loops calling an unnamed routine until a key event is flagged and the key code equals 13 (return), then returns.
- `clear_ten_thousand_longs_2`: Writes zero to 10000 consecutive 32-bit words starting at the address held in a register.
- `clear_ten_thousand_longs`: Writes zero to 10000 consecutive 32-bit words starting at the address held in a register, counting down from 9999 to minus one, then returns.
- `forward_single_unnamed_call`: Calls one unnamed routine and returns, with no other operations.
- `copy_ten_thousand_longs`: Copies 10000 consecutive 32-bit words from one buffer to another by storing each source word and advancing both pointers.
- `stub_return_only`: It is a stub, most likely decoded from a gap between routines rather than real logic.
- `reset_cohort_state`: The routine clears a set of global variables and then zeroes a block of memory in RAM by iterating nine times and writing zeros to three consecutive words per iteration.
- `frag_move_cmpi_beq_rts`: Short code fragment that copies 0 to a fixed memory address, then compares a fixed memory address with 0, then branches if the last result was zero, then copies 0 to a fixed memory address, then returns.
- `wait_for_flag_toggle`: Spins until a specific bit in a global variable is cleared and then spins until that bit is set again.
- `frag_move_btst_beq_bsr_dbra_rts`: Short code fragment that copies 8 to d2, then copies 100 to d0, then tests bit 6 of a fixed memory address, then branches if the last result was zero, then calls busy_wait_countdown, then loops back while counting down d0, then loops back while counting down d2, then returns.
- `open_dos_library_and_store`: Calls an open-library function pointer from a global table and, when it returns a nonzero base, stores that base in a global variable.
- `open_and_read_file_value`: Calls the DOS open wrapper and, on success, the read wrapper.
- `write_block_with_status`: On first use sets a flag and runs an initialisation routine.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `invoke_table_callback_store`: Calls a routine taken from a global table at an offset and stores its return value in a global variable.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `lock_path_store_handle`: Calls the DOS lock function from a global table, stores the lock in a global variable, then calls another routine.
- `scan_directory_entries`: Examines a directory through the DOS wrappers, then takes up to 201 entries in turn and calls an unnamed routine on each until none remain.
- `frag_move_bra`: Short code fragment that copies 3896 to d1, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies 3902 to d1, then jumps to another point in the code.
- `frag_move_bra_3`: Short code fragment that copies 3908 to d1, then jumps to another point in the code.
- `append_table_records`: Calls a routine via a pointer in a table, stores the result in a global variable, and then enters a loop that calls a routine and appends a record to a table until a condition is met or a counter reaches 200.
- `frag_move_movea_jsr_rts_movem`: Short code fragment that copies a fixed memory address to d1, then copies a fixed memory address to a6, then calls call_routine_six_times, then calls another routine, then returns, then copies 1 to a fixed memory address, then restores saved registers from the stack, then returns.
- `frag_tst_bne_addq_bsr_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then adds 1 to a fixed memory address, then calls open_dos_library_and_store, then returns.
- `poll_interrupt_request_bit`: Writes 160 to a custom-chip register, then tests bit 32 of the interrupt request register and calls an unnamed routine when it is set.
- `handle_custom_interrupt`: Checks if bit 5 is set in the intersection of two hardware registers, and if so, writes 32 to a hardware register and calls another routine before returning from interrupt.
- `initialize_offset_table_bases`: Stores a base value read from a global variable into a flag and into the first of five consecutive fields in a structure.
- `process_keyboard_input`: Clears interrupt flags, reads a byte from the keyboard serial register, performs a loop 1024 times, and then updates a table entry based on the keyboard byte and a bit test.
- `clear_table_slot_and_flag`: Clears a flag variable, zeroes one byte of a table at an indexed offset, clears bit 64 in a global and sets that global to one, then returns.
- `wait_flag_then_run_sequence`: Busy-waits until a global value is nonzero, then calls an unnamed routine.
- `infinite_loop_stub`: The decompiled body is an infinite loop with no operations, so the routine never returns.
- `infinite_loop_stub_4`: An infinite loop that never returns.
- `set_global_pointers`: Assigns the address of the global block to three global pointer variables and returns.
- `set_global_pointers_3`: Assigns the address of a global variable to three other global pointer variables, then returns.
- `nested_loop_calls`: Runs an outer loop of 15 steps and, inside each, an inner loop of 32 steps that stores the inner counter in a global and calls two routines per step.
- `run_three_routine_sequence`: Calls three unnamed routines in sequence and returns.
- `capture_three_results`: Calls three unnamed routines in sequence and stores the 16-bit value each returns in its own global variable.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `split_value_into_nibbles`: Splits the low 12 bits of a global value into three 4-bit fields, storing the low and middle nibbles in two globals and the high nibble in a third, then returns.
- `store_indexed_table_entry`: Stores a 16-bit value built from two global nibble fields and a routine address into a table at an index taken from a global variable, then returns.
- `load_indexed_table_entry`: Reads a 16-bit value from a table at an index derived from a global variable and stores it into another global variable, then returns.
- `sum_palette_nibble_fields`: Calls an unnamed routine, then for 32 colour words stores into a global table the sum of each word's three 4-bit fields, masked with 3840, 240 and 15.
- `decode_packed_color_entries`: Calls another routine and then iterates through a 32-entry table, summing the three 4-bit components of each word and storing the result back in the table.
- `busy_wait_nested_loops`: Runs a nested counting loop of 2500 outer iterations, each with a short inner countdown, to waste time, then returns without calling anything.
- `call_single_routine_seven_times`: Calls one unnamed routine seven times in succession and returns.
- `call_routine_six_times`: Calls another routine six times in a counted loop, counting down from five to minus one.
- `call_routine_until_zero`: Repeatedly calls another routine and keeps looping while the 16-bit value that call returns is nonzero, then returns.
- `busy_wait_countdown`: Counts a 16-bit counter down from 15000 to minus one as a pure delay loop that touches no hardware or memory, then returns.
- `write_decimal_digits_to_buffer`: Writes the decimal digits of a value held in a register into a buffer pointed to by a pointer, storing each digit as an ASCII character in reverse order.
- `extract_digits_to_buffer`: Writes the decimal digits of a 16-bit value as ASCII characters into a byte buffer at a fixed width, filling backwards from the least significant digit.
- `format_decimal_string`: Writes the decimal digits of a value held in a register into a byte buffer pointed to by a pointer, using a reverse index, then calls another routine twice and restores a global variable from a saved copy.
- `blank_leading_zeros_in_buffer`: Walks a character buffer from its start and replaces each leading zero with a space, stopping at the first character that is not a zero followed by a digit, so the final digit is kept.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `frag_move_cmpi_bne_beq_addi_subi_bra`: Short code fragment that copies 0 to d6, then compares an indexed entry from a1 with 48, then branches if the last result was not zero, then compares an indexed entry from a1 with 0, then branches if the last result was zero, then copies 32 to an indexed entry from a1, then adds 1 to d6, then subtracts 6 from the memory at a1, then jumps to another point in the code.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `frag_move_cmpi_beq_bne_bsr_bra`: Short code fragment that copies 0 to d6, then compares an indexed entry from a1 with 0, then branches if the last result was zero, then compares an indexed entry from a1 with 32, then branches if the last result was not zero, then compares an indexed entry from a1 with 32, then branches if the last result was not zero, then calls another routine, then jumps to another point in the code.
- `frag_addi_bra`: Short code fragment that adds 1 to d6, then jumps to another point in the code.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `frag_move_cmpi_beq_addi_bra`: Short code fragment that copies d6 to the stack, then copies an indexed entry from a1 to an indexed entry from a1, then compares an indexed entry from a1 with 0, then branches if the last result was zero, then adds 1 to d6, then jumps to another point in the code.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `write_letter_encoded_digits`: Writes a 16-bit value into a buffer as decimal digits encoded as capital letters, A for zero through J for nine, filling backwards.
- `write_nibble_digits_from_table`: Writes the low twenty bits of a value into a buffer as five characters, each picked from a 16-character digit table by its four-bit group, filling backwards.
- `write_eight_digit_split`: Writes a 32-bit value as eight zero-padded decimal digits in two groups of four, splitting it at ten thousand, and stores the packed split in a shared variable.
- `format_decimal_digits`: Splits a 32-bit value into two 16-bit parts and writes their decimal digits as characters into a buffer at specific offsets.
- `frag_cmpi_bne_move_bra`: Short code fragment that compares d5 with 1, then branches if the last result was not zero, then copies 97 to d0, then jumps to another point in the code.
- `write_letter_digits_compact`: Fills a buffer backwards with capital-letter digits, A for zero through J for nine, taken from the upper sixteen bits of a value, using a lowercase a as filler.
- `write_padded_digits_then_stored`: Writes the upper sixteen bits of a value as decimal digits padded with zeros, then writes four zero-padded digits of a shared stored value into the buffer.
- `write_upper_digits_second_field`: Writes the upper sixteen bits of a value as zero-padded decimal digits into the second field of a buffer, offset twelve, filling backwards.
- `process_zero_terminated_entry_list`: Copies four words from a parameter block into shared variables, then loops over a zero-terminated byte list, storing each byte in a shared variable, calling two routines for it, and advancing a position by six per entry.
- `compute_row_offset_and_call`: Computes a byte offset from the position word and a row value times forty, stores the low four bits of the position word separately, and returns after the calculation.
- `store_value_if_table_flag`: Checks a flag in a lookup table indexed by a shared code value.
- `apply_shifted_bytes_to_bitplanes`: Takes five source bytes, shifts each into place by a shared bit offset, and clears and then sets bits in five stride-spaced bitplane words per row.
- `apply_mask_to_two_planes`: For five source bytes, shifts each by a shared bit offset and clears and sets bits in two sets of plane words at once, with the planes chosen by a shared mode value.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (bset.b) and then falls through into the code that follows, with no return of its own.
- `iterate_rectangle_cells`: Walks a rectangle of cells with nested row and column loops, setting shared column and row indexes for each cell and calling another routine once per cell.
- `modify_bitplane_data_3`: Calculates a memory address and applies a bitmask to clear or set bits in multiple words based on the state of a global variable.
- `decode_run_length_buffer`: Decodes a run-length encoded byte stream from a source buffer into a destination buffer, repeating fill bytes and copying literal bytes until the header's length is reached.
- `decode_run_encoded_data`: Calculates a target length from a table, then enters a loop that decodes data runs by reading bytes and performing bitwise OR operations on a destination buffer based on specific bit patterns.
- `or_decode_runs_to_limit`: Decodes packed runs from a stream and ORs each run's byte into a destination until a shared counter reaches its limit, then returns.
- `or_decode_runs_with_fill`: Decodes packed runs and literals into a destination with a bitwise OR, using a shared fill value for one record type, and returns when the shared counter reaches its limit.
- `or_decode_packed_runs_nested`: Decodes packed runs and literals into a destination with a bitwise OR, handling one record type in a deeper nested loop, and returns when a shared counter reaches its limit.
- `stub_return_immediately`: A stub that only returns to its caller, with no work and no side effects.
- `decode_packed_runs_skip_zeros`: Decodes packed runs and literals into a destination while writing only nonzero bytes, so zero values leave existing destination data unchanged, and returns when a shared counter reaches its limit.
- `decode_runs_skip_zeros_fill`: Decodes packed runs and literals into a destination, writing only nonzero bytes, with the fill value for one run type taken from a shared variable.
- `decode_runs_skip_zeros_nested`: Decodes packed runs and literals into a destination while writing only nonzero bytes, handling one record type in a deeper nested loop.
- `decode_packed_runs`: Decodes a stream of packed run-length encoded data into a destination buffer, skipping zero values to preserve existing data.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `decompress_data_stream_3`: Calculates a checksum by XORing two longwords, then uses a sequence of bit-reading calls to extract values that determine how many bytes are copied from a table into a destination buffer via a nested loop.
- `frag_moveq_bra`: Short code fragment that loads 8 into d1, then loads 8 into d4, then jumps to another point in the code.
- `branch_on_bit_count`: Loads 2 into d1 and calls a routine, then compares d2 to 2 and 3 to decide whether to call a routine with 8 in d1 or 12 in d1.
- `decode_and_store_bytes`: Calls a routine to read bits, then enters a loop that decrements a pointer in a2 and stores a byte from a table at that pointer, repeating for a count determined by d3, before performing a comparison and a conditional branch.
- `return_constant_one`: Sets register d0 to 1 and returns.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `open_dos_file_handle`: Calls the DOS library's open routine and stores the returned file handle in a shared variable.
- `read_char_if_available`: Checks whether input is waiting on the operating system's file input.
- `read_input_byte_validated`: Reads one byte from the operating system's input into a shared variable and sets a ready flag, clearing the flag again when the byte is zero or 123 or above.
- `call_close_2`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:INSTALL.EXE

- `setup_display_and_key_loop`: Saves the system state, programs display window, bitplane modulo, DMA, interrupt and sprite registers, clears a block of shared variables and runs a timed delay.
- `setup_display_and_exit`: Programs the display window, bitplane modulo, DMA, interrupt and sprite registers after saving the system state, clears shared variables and runs a timed delay.
- `append_record_to_table`: When a shared check value is not positive, copies a fixed block of forty-eight bytes into a shared table at the slot given by a shared counter, then advances that counter.
- `scan_records_then_finish`: Scans shared records one slot at a time, calling a routine for each until it reports a match.
- `lock_directory_and_search`: Uses the DOS library to obtain the current directory and lock a volume path, then calls a routine to look up a record.
- `copy_record_header_words`: Calls a routine, then copies the first three words of a record slot into shared storage and calls four more routines.
- `create_directory_and_unlock`: Creates a directory for the path dh0:caesar through the DOS library, stores the returned lock in a shared variable, then releases a lock through the DOS library.
- `handle_text_entry_key_2`: Handles one key of a text-entry field.
- `set_initial_config_values`: Writes five fixed constants, sixty, seventy, two hundred, eight and four, into shared variables, then calls one routine.
- `call_eight_routines_in_order`: Calls eight other routines in order with no other work.
- `run_eight_setup_steps`: Calls eight routines in a fixed order with no logic of its own.
- `call_seven_routines_in_order`: Calls seven other routines in order with no other work.
- `execute_seven_setup_routines`: Calls seven different routines in a fixed, sequential order before returning.
- `run_eight_helper_sequence`: Calls eight helper routines in order with no logic of its own, serving as a fixed sequence of initialisation calls.
- `wait_for_return_key_2`: Loops until a key-ready flag is set and the key value is the Return key, then returns.
- `clear_table_entries_2`: Writes zero to ten thousand consecutive 32-bit entries starting at a pointer.
- `clear_table_entries`: Writes zero to ten thousand consecutive 32-bit entries starting at a pointer, then returns.
- `call_single_routine`: Calls one other routine and returns.
- `copy_memory_block`: Copies ten thousand consecutive 32-bit words from a source pointer to a destination pointer, then returns.
- `return_immediately`: An empty stub consisting of four instructions that performs no operations before returning to the caller.
- `reset_shared_state_block`: Clears several global variables and then zeroes a block of memory in RAM by iterating nine times and writing zeros to three consecutive words per iteration.
- `frag_move_cmpi_beq_rts_2`: Short code fragment that copies 0 to a fixed memory address, then compares a fixed memory address with 0, then branches if the last result was zero, then copies 0 to a fixed memory address, then returns.
- `wait_for_flag_toggle_2`: Spins until a specific bit in a global variable is cleared and then spins until that bit is set again.
- `frag_move_btst_beq_bsr_dbra_rts_2`: Short code fragment that copies 8 to d2, then copies 100 to d0, then tests bit 6 of a fixed memory address, then branches if the last result was zero, then calls delay_execution, then loops back while counting down d0, then loops back while counting down d2, then returns.
- `open_library_and_store_base`: Calls the operating system's library-opening routine through a shared table entry, for the DOS library, and stores the returned library base in a shared variable when the call succeeds.
- `check_store_result_or_fail`: Calls a routine and, if it succeeds, calls a second one.
- `run_checked_sequence_return_result`: Runs a one-time setup step when a shared flag is clear, then calls routines in sequence.
- `open_file_via_table_store`: Calls the operating system's open routine through a shared table entry and stores the returned handle in a shared variable.
- `read_via_table_entry`: Calls the operating system's read routine through a shared table entry and returns.
- `call_table_entry_store_result`: Calls a DOS library routine selected from the global function table and stores its return value in a global variable.
- `write_via_table_entry`: Calls the operating system's write routine through a shared table entry and returns.
- `close_via_table_entry`: Calls the operating system's close routine through a shared table entry and returns.
- `lock_named_volume_and_store`: Obtains a lock through the DOS library for the volume path ABC:, stores the lock in a shared variable and calls one routine.
- `lock_floppy_path_and_store`: Obtains a lock through the DOS library for the floppy drive path DF0:, stores the lock in a shared variable and calls one routine.
- `step_directory_entries_color_flash`: Examines the first directory entry through the DOS library, then steps through up to 201 entries.
- `nop_padding`: This routine is empty and performs no operations.
- `scan_directory_entries_then_unlock`: Locks the floppy path DF0: through the DOS library, examines up to 201 directory entries and calls a routine for each, then releases the lock and calls one routine.
- `frag_move_bra_4`: Short code fragment that copies 4266 to d1, then jumps to another point in the code.
- `append_records_from_file`: Calls a routine via a pointer in a table, appends a record to a table, and then enters a loop that calls a routine and appends records until a zero value is returned or a counter reaches 200.
- `frag_tst_bne_addq_bsr_rts_2`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then adds 1 to a fixed memory address, then calls open_library_and_store_base, then returns.
- `call_if_vblank_interrupt_enabled`: Reads the interrupt-enable register and, if the vertical blank interrupt bit is set, calls a routine.
- `handle_custom_interrupt_bitplane`: Checks if bit 5 is set in the intersection of two hardware registers, and if so, writes 32 to a hardware register and calls another routine before returning from interrupt.
- `store_five_bitplane_pointers`: Stores a base address in a shared variable and writes five bitplane pointers into a structure, each one 8000 bytes after the previous, then sets a shared flag.
- `update_byte_table_from_input`: Clears interrupt flags, reads the keyboard serial register, performs a loop 1024 times, and then modifies a table entry and a global variable based on the keyboard byte.
- `clear_table_byte_slot`: Clears one byte in a table at a given index, clears a shared flag and sets a shared constant, then returns.
- `wait_for_flag_then_dispatch`: Spins until a shared value is nonzero, then calls one routine, calls one of two routines depending on whether a shared pointer equals the address of a shared variable, and finally calls another routine.
- `infinite_loop_stub_2`: An empty infinite loop that never returns, so it blocks execution.
- `infinite_loop_stub_3`: Contains only an infinite loop that never returns; the routines listed for it are not reached in the decompiled code.
- `set_global_pointers_2`: Sets three shared pointers to the address of the same shared variable, then returns.
- `set_global_pointers_4`: Assigns the address of a global variable to three other global pointer variables, then returns.
- `nested_loop_calls_2`: Runs a fifteen-by-thirty-two nested loop.
- `call_three_routines`: Calls three other routines in order and returns.
- `call_and_store_three_results`: Calls a routine, then stores the low word of the register it leaves into a shared variable, repeated for three calls in turn.
- `return_empty_routine`: The routine performs no operations and immediately returns to its caller.
- `extract_nibble_fields`: Splits a shared word into three four-bit fields, the lowest nibble, the next nibble and a third nibble, storing each in its own shared variable.
- `store_table_entry`: Computes a table index from a shared counter and stores a 16-bit value, built from two shared values and a routine's address, into that table slot.
- `load_table_entry`: Loads a 16-bit value from a table at the index given by a shared counter into a shared variable, then returns.
- `convert_color_words_to_table`: Calls one routine, then fills a shared table of thirty-two entries from a source pointer, each entry being the sum of three masked four-bit fields of its source word.
- `decode_color_table`: Calls one routine, then rewrites a shared table of thirty-two words in place, replacing each word with the sum of its three masked four-bit fields.
- `delay_loop`: Busy-waits through 2500 outer iterations, each with a short two-step inner countdown, then returns.
- `call_routine_seven_times`: Calls one routine seven times in a counted loop, then returns.
- `call_routine_six_times_2`: Calls another routine six times in a counted loop, counting down from five to minus one, then returns.
- `execute_delay_loop`: Repeatedly calls another routine in a loop until it returns zero.
- `delay_execution`: Runs a decrementing loop for fifteen thousand iterations.
- `write_decimal_digits_to_buffer_2`: Writes the decimal digits of a value held in a register into a buffer pointed to by a pointer, storing each digit as an ASCII character in reverse order.
- `write_decimal_digits_to_buffer_3`: Converts a value to decimal digits and writes them as ASCII characters into a buffer, last digit first.
- `format_decimal_string_2`: Writes the decimal digits of a value held in a register into a byte buffer pointed to by a pointer, using a reverse index, then calls another routine twice and restores a global variable from a saved copy.
- `replace_numeric_characters`: Iterates through a string and replaces specific characters with spaces if they match a certain numeric range.
- `return_from_procedure`: The routine is empty and performs no operations before returning to the caller.
- `process_decimal_character`: Checks if a byte at offset 8 of a structure in a1 is the character 48, and if the byte at offset 9 is 0, it changes the byte at offset 8 to 32 and modifies a value at the address in a1 before looping.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `validate_buffer_content`: Clears d6 and checks if the byte at offset 8 of a1 is 0, or if it is 32 and the byte at offset 9 is also 32, then calls another routine or returns.
- `frag_addi_bra_2`: Short code fragment that adds 1 to d6, then jumps to another point in the code.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `frag_move_cmpi_beq_addi_bra_2`: Short code fragment that copies d6 to the stack, then copies an indexed entry from a1 to an indexed entry from a1, then compares an indexed entry from a1 with 0, then branches if the last result was zero, then adds 1 to d6, then jumps to another point in the code.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `convert_to_alphanumeric_text`: Converts a numeric value into a sequence of alphanumeric characters and stores them in a buffer.
- `write_nibbles_as_hex_text`: Writes the five low nibbles of a value into a buffer as characters from a sixteen-character digit table, then calls two routines.
- `format_split_numeric_string_2`: Splits a value into two parts and writes the decimal digits of each part into two separate locations within a buffer.
- `format_split_numeric_string`: Splits a value into two parts and writes the decimal digits of each part into two separate locations within a buffer.
- `frag_cmpi_bne_move_bra_2`: Short code fragment that compares d5 with 1, then branches if the last result was not zero, then copies 97 to d0, then jumps to another point in the code.
- `format_alphanumeric_string`: Populates a memory buffer with a sequence of characters derived from a numeric value.
- `format_numeric_string_with_padding`: Converts a numeric value into a string of digits and stores the result in a buffer with specific padding.
- `format_numeric_string`: Converts a numeric value into a string of digits and stores it in a buffer at a specific offset.
- `walk_text_with_parameters`: Loads four 16-bit parameters from the block at the A1 register into globals, then walks a zero-terminated byte string that follows them.
- `compute_row_offset_and_nibble_2`: Computes a byte offset from a position word and a row value, stores the low four bits of the position word separately, and returns.
- `dispatch_by_mode_flag`: Checks a flag in a table indexed by a value and, when it is set, stores a pointer and dispatches to one of two routines depending on a mode value.
- `apply_bit_mask_pattern`: Reads five source bytes and shifts each into a bit mask, then clears and sets bits in five groups of longwords in a destination table chosen by a mode value.
- `apply_mask_to_two_planes_2`: Processes five source bytes to build bit masks that clear and set bits in two separate destination tables.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (bset.b) and then falls through into the code that follows, with no return of its own.
- `iterate_cell_grid`: Walks a rectangular range of cells row by row and column by column, setting the cell position globals and calling a routine for each cell.
- `mask_memory_blocks`: Calculates a pointer to a table and performs bitwise AND operations on five specific memory locations using a mask derived from a global variable.
- `unpack_packed_picture_stream_alt`: Decodes a packed picture stream into an output buffer using a control-byte scheme where specific patterns repeat bytes or copy literal bytes.
- `decompress_data_stream`: Decompresses data by reading a length and control bits from a source buffer, then performs bitwise OR operations to expand data into a destination buffer using different branching logic based on the control bits.
- `decompress_data_stream_2`: Processes a compressed data stream by reading control codes to perform repeated byte writes into a destination buffer.
- `decompress_data_stream_variant_a`: Iterates through a data stream to expand compressed values into a destination buffer using various bit-flagged control codes.
- `decompress_data_stream_variant_b`: Decodes a stream of bytes by interpreting specific bit patterns to determine how many bytes to write to a destination buffer.
- `return_stack_value`: This routine returns a value located on the stack.
- `decompress_non_zero_stream`: Processes a compressed data stream and writes only non-zero bytes from the source to a destination buffer based on control codes.
- `decompress_data_stream_variant_c`: Expands a compressed data stream into a destination buffer, skipping zero values during the write process.
- `decompress_data_stream_variant_d`: Reads a compressed stream and writes non-zero bytes from the source to a destination buffer based on control codes.
- `decompress_data_stream_variant_e`: Decodes a data stream by using control bits to decide whether to repeat a single byte or write a sequence of bytes.
- `return_uninitialized_stack_value`: This routine is empty and returns an uninitialized value from the stack.
- `decompress_data_stream_4`: Performs a checksum via XORing two longwords and enters a loop structure that reads bits to control the copying of bytes from a table into a destination pointed to by a2.
- `frag_moveq_bra_2`: Short code fragment that loads 8 into d1, then loads 8 into d4, then jumps to another point in the code.
- `branch_on_bit_count_alt`: Loads 2 into d1 and calls a routine, then compares d2 to 2 and 3 to decide whether to call a routine with 8 in d1 or 12 in d1.
- `decode_and_copy_byte`: Calculates a count based on a value in d2, calls another routine, and then enters a loop that copies a byte from an offset of a2 into a2, decrementing a2 and the loop counter each iteration.
- `return_constant_one_2`: Loads the value one into a data register and returns immediately.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `open_raw_window_handle`: Opens the raw window named by a fixed specification string through the DOS library and stores the returned handle in a shared variable.
- `check_input_byte`: Calls a routine to read a byte and, if successful, stores the value in a global variable and sets a flag if the value meets specific criteria.
- `read_input_byte_flagged`: Sets a flag, reads a byte from an input handle using a library call, and clears the flag if the byte is zero or greater than one hundred twenty-two.
- `call_dos_close`: Calls a library routine to close a file handle.
- `drain_pending_input`: Repeatedly checks through the DOS library whether input is waiting, and while it is, reads a byte and sets a ready flag, until no input remains.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:trainer

- `init_graphics_and_keyboard_loop`: Opens the graphics library, saves the system's copper and interrupt state, programs the custom chips' display and copper registers, then loops reading keyboard input until a key status is clear.
- `copy_glyph_rows_to_bitmap`: Draws a string into a bitmap by copying each character's eight-byte glyph from a font table into eight rows spaced forty bytes apart.
- `step_value_up_with_delay`: Repeatedly adds 273 to a shared value, pausing with a countdown after each step, until the value reaches 4095, then returns.
- `step_value_down_with_delay`: Repeatedly subtracts 273 from a shared value, pausing with a countdown after each step, until the value reaches zero, then returns.
- `clear_shared_byte_block`: Zeroes a block of 10240 consecutive bytes in a shared memory region, then returns.
- `init_text_rows_poll_keyboard`: Fills a table of text rows with spaced characters and zero bytes, then toggles a shared byte and polls keyboard input in a loop.
- `frag_move_adda_dbra_bra`: Short code fragment that copies 70 to the memory at an offset from a2 (2 times in a row), then copies 1 to the memory at a4 (advancing a4), then adds 37 to a2, then loops back while counting down d0, then jumps to another point in the code.
- `wait_for_status_bit_transition`: Spins until a status bit in a hardware byte becomes set, then spins until it is clear again, then returns.
- `patch_nop_instructions_once`: If a shared flag is clear, stores seven address offsets and two no-operation instruction words into shared memory, then returns its first argument unchanged.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:play

- `find_max_build_record`: Walks a linked list to find the largest value in a field, then builds a new record with a counting pattern and exchanges sixteen words through pointer calls.
- `copy_sixteen_words_then_call`: If a destination pointer is set, copies sixteen 32-bit words from a fixed shared table into it, then calls one routine.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:assign

- `update_linked_list_and_copy_data`: Traverses a linked list to find the maximum value in a specific field and then updates a header structure.
- `copy_memory_block_2`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:echo

- `find_max_and_populate_buffer`: Traverses a linked list to determine the maximum value in a specific field and then writes a header and a sequence of incrementing values into a destination buffer.
- `copy_memory_block_3`: Copies sixteen bytes from a memory location offset from a register to a destination address.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:skip

- `update_linked_list_and_copy_data_2`: Traverses a linked list to find the maximum value in a specific field, then updates a header and performs a series of memory copies between a source block and a destination buffer based on the calculated maximum.
- `copy_memory_block_4`: Copies sixteen bytes of data from a memory location to a destination pointed to by a stack value.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:csr.exe

- `run_timed_display_sequence`: Runs a timed display sequence: saves the system state, sets up the display, DMA and copper registers and a sprite table, then loops over frame values with delays.
- `decrement_until_minus_one`: Counts a 16-bit register down until it reaches minus one, then returns.
- `loop_table_updates`: Performs three calls to a routine via a pointer in a4 while incrementing d6, then calls a routine via a pointer in a4 with d6 set to d5, and finally enters a loop that increments d5 until it reaches 4.
- `update_table_values`: Performs a series of checks and calls on d0, then calls a routine via a pointer in a4, and finally enters a loop that adds a value from d3 to a table at an offset determined by d0 and d2.
- `frag_beq_movea_bsr_bra`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `frag_cmpa_beq_move_subq_bne_rts`: Short code fragment that compares a2 with a6, then branches if the last result was zero, then copies the memory just below a2 (moving a2 down) to the memory at a1 (advancing a1) (4 times in a row), then subtracts 2 from d0, then branches if the last result was not zero, then copies the memory just below a1 (moving a1 down) to d0, then returns.
- `decode_variable_bitstream_2`: Reads a control byte from a table to determine a bit count and a number of repetitions, then decodes a variable-length bitstream from a source buffer in a0 into a destination buffer in a3 using bit-shifting and carry-propagation logic.
- `frag_moveq_add_bne_move_addx_dbra_addq`: Short code fragment that loads 2 into d7, then adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d0 and the extend flag to d0, then loops back while counting down d7, then adds 6 to d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_3`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_4`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_5`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `decode_variable_bitstream_expansion`: Decodes a variable-length bitstream from a buffer in a0 by reading control bytes from a table to determine bit counts and using bit-shifting and carry-propagation to reconstruct values.
- `frag_add_bne_move_addx_bra`: Short code fragment that adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d2 and the extend flag to d2, then jumps to another point in the code.
- `decode_variable_bitstream`: Reads a control byte from a table to determine a bit-count, then decodes a variable-length bitstream from a source buffer in a0 into a destination buffer in a3.
- `decode_and_copy_bitstream`: Reads a control byte from a table to determine a bit-count, then decodes a variable-length bitstream from a source buffer into a destination buffer using bit-shifting and carry operations.
- `frag_move_movem_bra`: Short code fragment that copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:failat

- `update_linked_list_and_copy_data_3`: Traverses a linked list to find the maximum value in a specific field and then updates a data structure with this value.
- `copy_memory_block_5`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:Anthrox

- `run_sequence_restore_saved_word`: Calls routines in sequence, looping a few times through one of them, then clears a word.
- `decrement_until_minus_one_2`: Decrements a value stored in a register until it reaches negative one and then returns.
- `update_array_via_function_calls`: Calls a function pointer via a4 multiple times, using the returned values to increment elements in an array pointed to by a1.
- `update_table_via_function_call`: Calls a function pointer in a4, then uses the returned value in a1 to add a value to an array located at a3, repeating this process in a loop based on the count in d1.
- `frag_beq_movea_bsr_bra_2`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry_4`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_from_source`: Compares two pointers and, if they are not equal, copies four bytes from the source pointed to by a register to the destination pointed to by another register before decrementing a counter and looping if the counter is not zero; otherwise, it loads a value from the destination into a register and returns.
- `decode_variable_bitstream_and_copy_data_3`: Reads a control byte from a table to determine a bit count and a value, then performs a bit-shifting loop to decode a variable-length bitstream into a destination buffer.
- `update_bitstream_counters`: Increments a byte in a register and a word in another register using the carry flag, then updates a byte in a register by adding 6 before branching to another routine.
- `frag_moveq_bra_6`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_7`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_8`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `decode_variable_bitstream_and_copy_data_2`: Decodes a variable-length bitstream from a memory buffer pointed to by a0 using a control table to determine bit counts and values, then performs a block move of data to a destination address.
- `frag_add_bne_move_addx_bra_2`: Short code fragment that adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d2 and the extend flag to d2, then jumps to another point in the code.
- `decode_variable_bitstream_and_copy_data`: Decodes a variable-length bitstream using a control table to determine bit counts and values, then uses the resulting values to perform a block memory copy from a source buffer to a destination buffer.
- `decode_and_copy_bitstream_2`: Reads a control byte from a table to determine a bit-count and a value, then performs a bit-shifting loop to decode a variable-length bitstream from a source buffer into a destination buffer.
- `frag_move_movem_bra_2`: Short code fragment that copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:cd

- `update_linked_list_and_copy_data_4`: Traverses a linked list to find the maximum value in a specific field, then updates a header and performs a series of memory copies between a source block and a destination buffer.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer, then calls another routine.
- `update_state_via_callback_sequence`: Performs a bitwise XOR on a byte in a structure, updates several pointers and offsets within that structure, and executes a sequence of calls through a function pointer while conditionally updating a longword at offset 600 based on values retrieved from a table.
- `execute_parameterized_calls`: Performs a series of calls through a pointer in a5 using parameters from a1 and a2, and updates a longword at offset 608 of a structure in a2.
- `execute_system_calls`: Performs a sequence of calls to a routine in a5 using various parameters from a structure in a2, and eventually jumps to an address in a6.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:endif

- `stub_return_zero`: A stub that returns zero immediately, with no other work.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:caesar.boot

- `ProgStart`: Opens the graphics and intuition libraries, sets a flag in the intuition display structure if it is clear, and in that case frees a graphics raster and remakes the display.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:if

- `update_linked_list_and_copy_data_5`: Traverses a linked list to find the maximum value in a specific field, then updates a header and performs a series of memory copies between a source block and a destination buffer.
- `copy_memory_block_7`: Copies a block of sixteen bytes from a source address to a destination address.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:bootblock

- `open_library_and_set_flag`: Gets a library base through the system's library-opening routine and sets a bit in that library's flags when one is returned.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_graphics_copper_sequence`: Opens the graphics library, saves interrupt and DMA state, clears sprite control registers, and fills a table of sprite values.
- `swap_pair_with_offset_store`: Calls routines, then stores a shared value shifted by a fixed offset into a shared word and swaps two shared 32-bit values.
- `lookup_table_bytes_by_counter`: If a shared counter is nonzero, either sets a shared value and calls a routine when the counter is below ten, or, when it is above sixteen, loads one byte and three further bytes from a table at an offset derived from the counter into shared variables.
- `swap_buffer_pair_back_offset`: Calls a routine twice, stores a value two bytes before a second saved address into a shared pointer, and swaps two saved buffer addresses.
- `pass_through_helper_call`: Calls a helper routine and returns.
- `store_shifted_parameter_values`: Calls a routine, stores a parameter shifted right by three into a shared word, calls another routine, then stores the parameter shifted right by six into a second shared word.
- `advance_and_mask_position`: Adds a stored velocity pair to a two-word position and masks each coordinate to an even value below 4096.
- `start_blit_with_fixed_size`: Calls one routine, then programs the blitter's first and last word masks, control registers and a destination pointer, and starts a blit with a fixed size word.
- `setup_blitter_registers`: Waits for the blitter to become idle and then writes values to several blitter hardware registers.
- `vertical_blank_state_machine`: On a vertical blank request, reads a value through a routine, then advances a frame state machine: an idle state, a countdown, and a running phase that counts 110 frames before returning to idle.
- `start_destination_only_blit`: Calls a routine first, then loads the blitter first-word mask, a control word that enables only the destination channel with no shift, a zero destination modulo, the destination pointer and a small blit size, which starts the operation.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_for_blitter_idle`: Polls a status byte at a fixed offset from an address register and loops while its busy bit is set, returning once the blitter is idle.
- `reset_text_cursor_state`: Points the character stream pointer at a fixed start address, clears the column counter, sets the line counter to one and clears a flag byte, preparing the decoder for a new run of text.
- `advance_text_stream_cursor`: Reads the next byte of a character stream.
- `step_text_stream_column`: Advances the column counter and starts a new line at 41 columns.
- `start_new_text_line`: Resets the column counter and advances the line counter, then reads the byte at the stream pointer.
- `configure_blit_bit_alignment`: Calls a routine, then sets the blitter first-word mask and control word from the low bits of two position values.
- `copy_paired_words_from_records`: Writes two fixed longword headers at the start of two tables, then copies the first two words of each of 64 six-word source records into the first table and the second table respectively.
- `copy_paired_words_second_bank`: Calls a routine, then writes two fixed longword headers at the start of a second pair of tables and copies the first two words of 64 six-word source records into them.
- `copy_paired_words_to_pointers`: Writes two fixed longword headers at the start of a destination and at a second destination 64 longwords further on, then copies the first two words of 64 six-word source records into the two tables, one word each.
- `store_table_addresses_in_slots`: Stores the low and high words of four table addresses, each offset from a base address by a fixed amount, into two sets of memory slots, then stores one fixed value into another slot.
- `pack_position_into_table_entries`: Writes a position and the position 16 units further on into two tables, storing the halved values in the low bytes with the low bit of each original beside them, then merges two bit fields from a second parameter into the entries.
- `set_table_cursor_and_end`: Stores the input address as the current cursor, then stores an end address computed from it: the cursor plus the first word scaled by four, plus two.
- `toggle_global_buffer_flag`: Calls a routine and then flips the low bit of a global variable, so the value alternates between 0 and 1 on each call.
- `store_state_then_step_thrice`: Saves an input address into two globals, stores two input word values into two others, then calls a routine three times in a row.
- `wait_for_bit_set`: Loops until a specific bit is set in a byte located at an offset from a pointer.
- `start_rectangle_fill_blit`: Computes the width and height of a rectangle from four bounds values in a shared record, then loads the destination pointer returned by a called routine with full first-word masks, a width-based modulo and the blit size, enabling only the destination channel.
- `start_rectangle_copy_blit`: Computes the width and height of a rectangle from four bounds values in a shared record, then loads a pointer returned by a called routine as both source and destination, sets the modulo from the width and starts a blit using the source and destination channels.
- `project_points_to_screen_coords`: Calls a routine, then loops over a list of point pairs, multiplies each x and y by a fixed-point factor masked to ten bits and shifted right by ten, offsets them by 160 and 128, and stores the results, calling a routine for each point.
- `process_indexed_entry_pairs`: For each index pair in a count-driven list, reads a longword pair from a global table at the first index and passes it, with the entry at the second index, to a called routine; it returns the last longword pair read.
- `reset_bounding_box_to_centre`: Sets the four extents of a bounding box, both x values to 160 and both y values to 128, the centre of a 320 by 256 screen, as the starting point before the box is grown to include points.
- `expand_bounding_box_with_point`: Extends a four-value bounding box so it includes a point: the minimum and maximum x are lowered or raised to the point's x, and the minimum and maximum y likewise to its y, leaving each extent unchanged when the point lies inside it.
- `setup_blitter_line_draw`: Programs the blitter for a line between two points: it computes the start address from a row pitch and the column, the major and minor axis steps and the octant code from the signs and the dominant axis, then starts the blit in line mode.
- `build_indexed_record_lists`: Scans a byte table for the first entry above a running value.
- `tick_and_update_audio_channels`: Advances a tick counter; when it reaches its limit it runs a block of routines, pauses briefly and copies stored location and length values into all four audio channel register pairs.
- `reset_global_flag_word`: Writes zero to a global variable several times and then sets it to a fixed mask.
- `apply_channel_effect_tick`: After calling four routines, reads the effect nibble of a channel word and applies one tick of it: a pitch slide up or down, vibrato, a stepped three-tick arpeggio, a volume or panning value, or an extended command.
- `play_tracker_channel_row`: Advances one channel of a music tracker.
- `apply_pattern_effect_tick`: Runs the per-tick effect for one tracker channel from a record: depending on the effect nibble it steps the period up or down, slides toward a target, cycles a three-step arpeggio, applies a vibrato offset from a table or applies a volume slide.
- `copy_record_word_field`: Copies one word from a fixed field of a record into another word field of a second record.
- `find_target_period_entry`: Searches a row of a 37-word table for the first entry that is not above the record's current word, stepping back one entry when a flag is set.
- `slide_period_toward_target`: Moves a channel's current period one slide step toward its target, clamping at the target and clearing the slide once it is reached, then looks up the matching table entry and stores it as the output value.
- `compute_vibrato_period_offset`: Computes a vibrato offset from a phase value and a depth nibble: it reads a value from a waveform table, a ramp or a fixed level depending on the waveform setting, scales it by the depth, applies its sign and adds it to the base period, then advances the phase by the speed nibble.
- `dispatch_tracker_row_effect`: Interprets a row's effect command on the first tick.
- `store_low_nibble_field`: Copies the low nibble of one byte in a record into another byte field of the same record; the decompiled body is this single store, and the wider activity in the listed facts is not shown.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:Anthrox#unpacked

- `initialize_graphics_and_copper_lists`: Checks the attention flags to decide whether to enter supervisor mode, opens a library, and disables interrupts and DMA. It then initializes several copper lists and sprite control registers, populates a table with encoded position words, and enters a loop that updates a value in a global variable based on a counter and the status of a joystick button.
- `swap_buffer_pair_store_offset`: Calls four helpers, stores a value 350 bytes past the second saved buffer address into a shared pointer, then swaps the two saved buffer addresses.
- `fetch_table_values`: Fetches a word from a PC-relative location and, depending on its value, either calls another routine or fetches four bytes from a table and stores them as words in a destination buffer.
- `swap_buffer_pair_back_offset_2`: Calls two helpers, stores a value two bytes before the second saved buffer address into a shared pointer, then swaps the two saved buffer addresses.
- `set_table_pointers`: Loads a word value from a program counter relative location, masks it with hexadecimal F, uses that as an index to fetch a 32-bit value from a table, adds another program counter relative address to that fetched value, and sets a register to point to the resulting address.
- `store_scaled_position_pair`: Calls a helper, stores the incoming position shifted right by three bits into one shared variable, calls another helper, then stores the position shifted right by six bits into a second shared variable.
- `combine_and_mask_coordinate_pair`: Adds each of two word pairs to its partner and masks both results to clear the low bit and the top four bits, keeping the values even and within a limited range.
- `frag_movea_suba_bra`: Short code fragment that copies a location in the code to a0, then subtracts 4096 from a0, then jumps to another point in the code.
- `configure_blitter_registers`: Writes values to several blitter control and size registers and waits for the blitter to become idle.
- `handle_interrupt_request`: Checks bit 5 of the interrupt request register, calls another routine, and updates various PC-relative memory locations based on the result.
- `frag_bsr_movem_move_rte`: Short code fragment that calls another routine, then restores saved registers from the stack, then copies 32 to a fixed memory address, then returns from the exception.
- `update_counter_and_call`: Calculates offsets based on a PC-relative word, calls another routine twice with different calculated addresses, and increments a PC-relative byte until it reaches 110.
- `frag_lea_bsr_move_rts`: Short code fragment that loads the address of a fixed memory address into a6, then calls another routine, then copies an address to the memory at an offset from a6, then copies an address to the memory at an offset from a6, then copies 0 to the memory at an offset from a6, then copies a0 to the memory at an offset from a6, then copies 151 to the memory at an offset from a6, then returns.
- `frag_subq_bne_bra`: Short code fragment that subtracts 1 from the memory at a0, then branches if the last result was not zero, then jumps to another point in the code.
- `wait_for_raster_line_2`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_blitter_idle`: Spins while the blitter busy flag in the DMA control status register is set, then returns.
- `reset_text_stream_state`: Resets a text stream: points the read position at a fixed start address, zeroes the column counter, sets the line counter to one and clears a byte in shared memory.
- `calculate_table_offset`: Reads a byte from a table, performs arithmetic involving division and multiplication to calculate an address, and calls another routine.
- `frag_lea_move_addq_cmpi_beq_bra`: Short code fragment that loads the address of a location in the code into a1, then copies a0 to the memory at a1, then loads the address of a location in the code into a5, then adds 1 to the memory at a5, then compares the memory at a5 with 41, then branches if the last result was zero, then jumps to another point in the code.
- `frag_lea_clr_addq_bra`: Short code fragment that loads the address of a location in the code into a5, then clears the memory at a5, then adds 1 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_lea_addq_tst_beq_move_rts`: Short code fragment that loads the address of a location in the code into a1, then adds 1 to a0, then tests the memory at a0, then branches if the last result was zero, then copies a0 to the memory at a1, then loads the address of a location in the code into a0, then copies 450 to the memory at a0, then returns.
- `frag_lea_adda_move_rts`: Short code fragment that loads the address of a location in the code into a0, then adds 5916 to a0, then loads the address of a location in the code into a1, then copies a0 to the memory at a1, then loads the address of a location in the code into a0, then copies 450 to the memory at a0, then returns.
- `frag_lea_move_addq_rts`: Short code fragment that loads the address of a location in the code into a5, then copies 0 to the memory at a5, then adds 1 to the memory at an offset from a5, then loads the address of a location in the code into a1, then copies a0 to the memory at a1, then returns.
- `setup_blitter_transfer`: Sets up blitter registers including source, destination, and control words using values from a1 and a3, then conditionally modifies the control register and destination address before returning.
- `frag_move_btst_bne_rts`: Short code fragment that copies an address to the memory at an offset from a6, then tests bit 0 of d1, then branches if the last result was not zero, then copies 32768 to the memory at an offset from a6, then copies 1994 to the memory at an offset from a6, then copies 2050 to the memory at an offset from a6, then returns.
- `frag_lea_movea_adda_bra`: Short code fragment that loads the address of a location in the code into a0, then copies a location in the code to a1, then adds 1280 to a1, then jumps to another point in the code.
- `frag_lea_movea_adda_bsr`: Short code fragment that loads the address of a location in the code into a0, then copies a location in the code to a1, then adds 1280 to a1, then calls another routine, then loads the address of a location in the code into a0, then copies a location in the code to a1, then adds 1792 to a1.
- `frag_move_addq_moveq_adda_dbra_rts`: Short code fragment that copies an address to the memory at a1, then copies an address to the memory at an offset from a1, then adds 4 to a1, then loads 63 into d7, then copies the memory at a0 to the memory at a1, then copies the memory at an offset from a0 to the memory at an offset from a1, then adds 2 to a1, then adds 12 to a0, then loops back while counting down d7, then returns.
- `set_copper_list_pointers`: Stores the high and low words of four list addresses, each a shared base address plus a fixed offset, into shared pointer variables, and sets a control constant.
- `encode_sprite_position_words`: Encodes a stored position into the words of two sprite control entries: the start position is halved, a fixed offset is added to the stop position, and a bit field derived from a second stored value is merged in.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `store_arguments_and_run_helpers`: Saves the incoming address and two register values into shared variables, then calls three helpers in order.
- `poll_memory_bit`: Polls a specific bit within a byte at an offset from a pointer until that bit is cleared.
- `setup_blitter_rect_copy`: Reads two corner coordinate pairs from shared storage, computes the blit width in words and the height in rows from their differences, sets a full-mask destination-only blit with a modulo from the width, and starts it using an address supplied by a helper.
- `compute_blit_dimensions`: Calculates blit width and height from coordinate pairs, sets up blitter registers with modulo values and control words, and then starts the blit operation.
- `scale_and_store_coordinates`: Iterates through a list of values, scales each value by a factor derived from a table using bit shifts, and stores the results into a global structure.
- `iterate_distance_calculations`: Loops through a table of indices, fetches two 16-bit values from a global structure using each index multiplied by 4, and calls another routine to perform calculations with those values and a value from the table.
- `reset_default_clip_rect`: Stores four default bounds values, 160, 160, 128 and 128, into a shared rectangle record.
- `update_boundary_values`: Compares d0 and d1 against four consecutive words in a table and updates the table entries if the register values are outside the current bounds.
- `calculate_distance_metrics`: Performs arithmetic calculations using two input values to update several global variables and a value in a memory table based on the difference between the inputs.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.
- `update_audio_parameters`: Calls another routine, copies a pointer to a global variable, loops to copy data from a table into a destination buffer, sets several audio volume registers to zero, and sets a bit in a hardware register.
- `check_and_initialize_audio`: Clears a 16-bit word in a global variable, increments a byte in a table, and if the byte is not less than a comparison value, clears another byte in a table and calls another routine if a byte at a specific location is non-zero.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `initialize_audio_channel_periods`: Sets the period registers for all four Amiga audio channels by calling a subroutine with specific data structures passed in a6.
- `initialize_audio_channels_3`: Calculates a value by fetching a byte from a table using an index, shifts it, and adds a constant, then clears a 16-bit word in a global variable and calls initialize_audio_channels four times with different hardware register addresses.
- `update_entity_attributes_and_dispatch`: Loads a pointer from a table into a6, extracts a type index from the bytes at offsets 2 and 0, and uses this index to fetch new attribute data from a table pointed to by a3.
- `update_structure_and_dispatch_command`: Calculates a new address by adding a value to a pointer in a structure, stores it at offsets 10 and 36, and updates a field from a table.
- `frag_bsr_bra_2`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_3`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_hardware_and_dispatch_command`: Searches a table to find a specific entry, updates a structure with new values, modifies the DMA controller configuration, and then branches to a command dispatcher based on a masked byte from the structure.
- `initialize_audio_channels`: Sets up audio channel registers by writing values from a table to DMA control and audio registers, then performs a delay loop before returning.
- `dispatch_structure_operation_by_nibble`: Reads a nibble from offset 2 of a structure in a6 and branches to various subroutines based on its value, while also copying a word from offset 16 of a6 to offset 6 of a structure in a5.
- `frag_move_rts`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then returns.
- `process_table_index`: Divides a byte from a table by 3, swaps the result, and branches based on whether the value is 0 or 2, otherwise it performs a bit shift on a byte from a structure.
- `frag_moveq_move_andi_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_move_bra_5`: Short code fragment that copies the memory at an offset from a6 to d2, then jumps to another point in the code.
- `fetch_table_entry`: Calculates an offset into a table using a byte from a structure, fetches a word from that table, and calls another routine to store it at an offset from a5.
- `frag_move_rts_2`: Short code fragment that copies d2 to the memory at an offset from a5, then returns.
- `update_structure_value_subtraction`: If a global variable is zero, it sets it to 255, subtracts a masked byte from offset 3 of a structure from a word at offset 16, and potentially clamps the result to a minimum value.
- `update_structure_value_addition`: If a global variable is zero, it sets it to 255, adds a masked byte from offset 3 of a structure to a word at offset 16, and potentially clamps the result to a maximum value.
- `search_table_value`: Searches a table for a word that matches a value from a structure, then stores that word at offset 24 of the structure, clears a byte at offset 22, and optionally sets a byte at offset 22 to 1 if the value is less than a comparison value at offset 16.
- `frag_clr_rts`: Short code fragment that clears the memory at an offset from a6, then returns.
- `frag_move_beq_clr`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then copies d0 to the memory at an offset from a6, then clears the memory at an offset from a6.
- `update_accumulator_value`: If a word at offset 24 of a structure is not zero and a byte at offset 22 is zero, it adds a byte from offset 23 to the word at offset 16, then clamps the value at offset 16 to the value at offset 24 and clears offset 24 if the limit is reached.
- `decrement_channel_period_and_lookup`: Subtracts the input value from the channel period and clamps it to the target when reached.
- `merge_nibbles_into_byte`: Merges the low nibble of a byte from offset 3 of a structure into the low nibble of a byte at offset 26 of a structure, and merges the high nibble of the same source byte into the high nibble of the destination byte.
- `calculate_scaled_offset`: Calculates a value by shifting and masking bytes from a structure, then stores the result at an offset from a5.
- `frag_tst_bpl_move_sub_bra`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_6`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `frag_move_andi_mulu_lsr_tst_bmi_add_bra`: Short code fragment that copies an indexed entry from a4 to d2, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then multiplies d2 by d0 (unsigned), then shifts d2 right by 7, then copies the memory at an offset from a6 to d0, then tests the memory at an offset from a6, then branches if the last result was negative, then adds d2 to d0, then jumps to another point in the code.
- `subtract_and_advance_phase`: Subtracts d2 from d0 and stores the result in offset 6 of a structure in a5, then advances a phase byte in the structure by an amount taken from the high nibble of a neighbouring byte.
- `frag_bsr_bra_4`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_5`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_scaled_byte_value`: Merges nibbles from a byte at offset 3 into a byte at offset 28, then calculates a scaled value using a table and a byte from offset 29, and stores the result at offset 8 of a structure.
- `frag_tst_bpl_move_sub_bra_2`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_7`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `multiply_and_shift_byte`: Multiplies a byte from a structure by a masked byte from another location, shifts the result, and adds it to a byte from a structure if a bit is set.
- `clamp_and_store_value`: Clamps the signed difference between d2 and d0 to a range between zero and sixty-four, stores the result as a word at offset 8 of a structure in a5, and updates a byte at offset 29 of a structure in a6 by adding the high nibble of the byte at offset 28 shifted right by two.
- `update_accumulated_value`: Checks a byte from a structure, and if non-zero, updates a byte at offset 20 and performs arithmetic involving a bit-shifted value and a byte at offset 8.
- `frag_move_rts_3`: Short code fragment that copies 1 to the memory at an offset from a6, then returns.
- `increment_clamped_byte`: Extracts the high nibble of a byte from a structure, adds it to a byte at offset 19 with a maximum limit of 64, and stores the result in a word at offset 8 of a5.
- `decrement_and_clamp_byte`: Subtracts the low nibble of the byte at offset 3 of the structure in a6 from the byte at offset 19, clears that byte to zero when the result is negative, and stores the resulting byte as a word at offset 8 of the structure in a5.
- `frag_move_subq_lea_clr_st_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then subtracts 1 from d0, then loads the address of a location in the code into a4, then copies d0 to the memory at a4, then loads the address of a location in the code into a4, then clears the memory at a4, then loads the address of a location in the code into a4, then sets the memory at a4 to all ones, then returns.
- `clamp_byte_and_store`: Reads a byte from offset 3 of the structure in a6, clamps it to a maximum of sixty-four, stores the clamped value as a byte at offset 19 of that structure, and stores it as a word at offset 8 of a structure in a5.
- `calculate_index_and_store`: Calculates a value based on the high and low nibbles of a byte from a structure and stores the result in a global variable if it is less than 63.
- `frag_move_beq_lea_clr_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then loads the address of a location in the code into a4, then clears the memory at a4, then loads the address of a location in the code into a4, then copies d0 to the memory at a4, then returns.
- `dispatch_command_byte`: Reads a byte from offset 2 of a structure in a6, masks the lower nibble, and branches to different subroutines based on the resulting value.
- `frag_moveq_move_lea_movea_rts`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then copies a6 to the stack, then loads the address of a location in the code into a6, then copies d0 to the memory at a6, then copies the top of the stack to a6, then returns.
- `dispatch_structure_operation_by_nibble_3`: Reads a byte from an offset of 3 in a structure pointed to by a6, extracts the high nibble, and branches to a specific handler routine based on that value.
- `frag_bset_rts`: Short code fragment that sets bit 1 of a fixed memory address, then returns.
- `mask_and_merge_byte`: Masks the low nibble of a byte at offset 31 from a pointer in a6 and then merges the low nibble of a byte at offset 3 from the same pointer into it.
- `clear_low_nibbles_of_byte`: Clears the low nibble of a byte at offset 30 from an address in a6 and then ORs in the low nibble of another byte from the same base.
- `store_masked_byte`: The routine reads a byte from an offset in a structure pointed to by a register, masks it with a constant, and stores the result into a different offset in the same structure.
- `update_structure_byte`: Checks a global byte and if it is non-zero, decrements a byte at offset 34 of the structure in a6 and stores a byte from offset 33 of the structure into another global variable; otherwise, it stores a bit-shifted version of a byte from offset 3 of the structure into a global variable.
- `frag_move_bra_8`: Short code fragment that copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_lsr_rts`: Short code fragment that copies a location in the code to d0, then shifts d0 right by 4, then copies d0 to the memory at an offset from a6, then returns.
- `mask_and_merge_nibble`: Masks the low nibble of a byte at offset 30 from a pointer in a6 and merges the high nibble from a byte at offset 3 of the same pointer, storing the result back at offset 30.
- `decode_and_copy_title_data`: Reads a nibble from a table entry and, if a global variable and the entry satisfy a modulo condition, copies two 32-bit words and one 16-bit word from the table through a pointer, performs two delay loops, writes a modified value to a global variable, and then copies two more 32-bit words from the table.
- `frag_lea_tst_bne_moveq_move_andi_bra`: Short code fragment that loads the address of a location in the code into a4, then tests the memory at a4, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_lea_tst_bne_moveq_move_andi_bra_2`: Short code fragment that loads the address of a location in the code into a4, then tests the memory at a4, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_moveq_move_andi_cmp_bne_clr_rts`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a location in the code, then branches if the last result was not zero, then clears the memory at an offset from a6, then copies 0 to the memory at an offset from a5, then returns.
- `frag_moveq_move_andi_cmp_bne_beq_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a location in the code, then branches if the last result was not zero, then copies the memory at a6 to d0, then branches if the last result was zero, then copies d1 to the stack, then jumps to another point in the code.
- `increment_and_store_byte`: Checks a global byte and if it is non-zero, takes a byte from offset 3 of the structure in a6, masks it, increments it, and stores the result into a global variable; otherwise, it returns.
- `frag_lea_tst_bne_move_andi_lsl_or_beq`: Short code fragment that loads the address of a location in the code into a4, then tests the memory at a4, then branches if the last result was not zero, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then shifts d0 left by 4, then masks the memory at an offset from a6 with 15, then merges d0 into the memory at an offset from a6, then tests d0, then branches if the last result was zero.
- `update_structure_index_and_toggle_byte`: Reads a nibble from offset 31 of a structure, uses it as an index into a table to add a value to offset 35 of the structure, and if the high bit of that byte is set, clears the byte and updates a pointer at offset 36 based on a comparison between a calculated address and a pointer at offset 10, finally toggling the sign of the byte pointed to by the new pointer.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:Anthrox#unpacked

- `initialize_graphics_and_hardware`: Checks the attention flags to potentially enter supervisor mode, opens a library, and disables interrupts and DMA. It then configures the copper, DMA, and sprite control registers, initializes a table of values with specific offsets, and enters a series of loops that manipulate graphics parameters and check the status of a CIA register.
- `swap_buffer_pair_store_offset_2`: Calls four helpers, stores a value 350 bytes past the second saved buffer address into a shared pointer, then swaps the two saved buffer addresses.
- `update_table_pointers`: Updates a global word value or fetches four bytes from a table to store as four longwords in another location based on a value from a program counter relative location.
- `swap_buffer_pair_back_offset_3`: Calls two helpers, stores a value two bytes before the second saved buffer address into a shared pointer, then swaps the two saved buffer addresses.
- `set_table_pointers_2`: Loads a word value from a program counter relative location, masks it with hexadecimal F, uses that as an index to fetch a 32-bit value from a table, adds another program counter relative address to that fetched value, and sets a register to point to the resulting address.
- `store_scaled_position_pair_2`: Calls a helper, stores the incoming position shifted right by three bits into one shared variable, calls another helper, then stores the position shifted right by six bits into a second shared variable.
- `combine_and_mask_coordinate_pair_2`: Adds each of two word pairs to its partner and masks both results to clear the low bit and the top four bits, keeping the values even and within a limited range.
- `frag_movea_suba_bra_2`: Short code fragment that copies a location in the code to a0, then subtracts 4096 from a0, then jumps to another point in the code.
- `configure_blitter_registers_2`: Writes values to several blitter control and size registers and waits for the blitter to become idle.
- `process_interrupt_status`: Checks bit 5 of the interrupt request register, calls another routine, and then updates several global word values based on the value of a word fetched from a table, including setting specific values to zero or negative constants.
- `frag_bsr_movem_move_rte_2`: Short code fragment that calls another routine, then restores saved registers from the stack, then copies 32 to a fixed memory address, then returns from the exception.
- `increment_and_check_counter`: Increments a 16-bit value in a global variable, compares it to 110, and if equal, writes 65534 to another global variable; otherwise, it performs two blitter calls using calculated offsets and returns.
- `configure_blitter_registers_3`: Calls another routine to wait for the blitter, then writes 32-bit values to the blitter control and address registers, writes zero to the blitter mode register, stores an address in the blitter partition register, and writes 151 to the blitter size register.
- `frag_subq_bne_bra_2`: Short code fragment that subtracts 1 from the memory at a0, then branches if the last result was not zero, then jumps to another point in the code.
- `wait_for_raster_line_3`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_blitter_idle_2`: Spins while the blitter busy flag in the DMA control status register is set, then returns.
- `reset_text_stream_state_2`: Resets a text stream: points the read position at a fixed start address, zeroes the column counter, sets the line counter to one and clears a byte in shared memory.
- `increment_table_index`: Fetches a byte from a table, and if it is not 0, 1, or 13, calculates an offset using division and multiplication to update a pointer in a3 and calls another routine; it then increments a byte at a specific offset and checks if it reaches 39.
- `frag_lea_move_addq_cmpi_beq_bra_2`: Short code fragment that loads the address of a location in the code into a1, then copies a0 to the memory at a1, then loads the address of a location in the code into a5, then adds 1 to the memory at a5, then compares the memory at a5 with 41, then branches if the last result was zero, then jumps to another point in the code.
- `frag_lea_clr_addq_bra_2`: Short code fragment that loads the address of a location in the code into a5, then clears the memory at a5, then adds 1 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_lea_addq_tst_beq_move_rts_2`: Short code fragment that loads the address of a location in the code into a1, then adds 1 to a0, then tests the memory at a0, then branches if the last result was zero, then copies a0 to the memory at a1, then loads the address of a location in the code into a0, then copies 450 to the memory at a0, then returns.
- `frag_lea_adda_move_rts_2`: Short code fragment that loads the address of a location in the code into a0, then adds 5916 to a0, then loads the address of a location in the code into a1, then copies a0 to the memory at a1, then loads the address of a location in the code into a0, then copies 450 to the memory at a0, then returns.
- `frag_lea_move_addq_rts_2`: Short code fragment that loads the address of a location in the code into a5, then copies 0 to the memory at a5, then adds 1 to the memory at an offset from a5, then loads the address of a location in the code into a1, then copies a0 to the memory at a1, then returns.
- `setup_blitter_transfer_2`: Calls another routine, sets several blitter registers to 65535 or 0, and then conditionally sets the blitter mode, address, and size registers based on the bits of two input values, potentially updating address pointers.
- `frag_move_btst_bne_rts_2`: Short code fragment that copies an address to the memory at an offset from a6, then tests bit 0 of d1, then branches if the last result was not zero, then copies 32768 to the memory at an offset from a6, then copies 1994 to the memory at an offset from a6, then copies 2050 to the memory at an offset from a6, then returns.
- `frag_lea_movea_adda_bra_2`: Short code fragment that loads the address of a location in the code into a0, then copies a location in the code to a1, then adds 1280 to a1, then jumps to another point in the code.
- `frag_lea_movea_adda_bsr_2`: Short code fragment that loads the address of a location in the code into a0, then copies a location in the code to a1, then adds 1280 to a1, then calls another routine, then loads the address of a location in the code into a0, then copies a location in the code to a1, then adds 1792 to a1.
- `copy_data_blocks`: Writes two 32-bit values to a destination table, then enters a loop that copies 37 words from a source table to the destination table, incrementing pointers accordingly.
- `set_copper_list_pointers_2`: Stores the high and low words of four list addresses, each a shared base address plus a fixed offset, into shared pointer variables, and sets a control constant.
- `encode_sprite_position_words_2`: Encodes a stored position into the words of two sprite control entries: the start position is halved, a fixed offset is added to the stop position, and a bit field derived from a second stored value is merged in.
- `frag_bra_6`: Short code fragment that jumps to another point in the code.
- `frag_bra_7`: Short code fragment that jumps to another point in the code.
- `store_arguments_and_run_helpers_2`: Saves the incoming address and two register values into shared variables, then calls three helpers in order.
- `poll_memory_bit_2`: Polls a specific bit within a byte at an offset from a pointer until that bit is cleared.
- `setup_blitter_rect_copy_2`: Reads two corner coordinate pairs from shared storage, computes the blit width in words and the height in rows from their differences, sets a full-mask destination-only blit with a modulo from the width, and starts it using an address supplied by a helper.
- `compute_blit_dimensions_2`: Calculates blit width and height from coordinate pairs, sets up blitter registers with modulo values and control words, then starts the blit operation.
- `scale_and_store_coordinates_2`: Iterates through a list of values, scales each value by a factor derived from a global variable using a bit shift, and stores the results into a global structure.
- `iterate_distance_calculations_2`: Loops through a table of indices, fetches two 16-bit values from a global structure using each index multiplied by 4, and calls calculate_distance_metrics to process these values along with a third value from the same structure.
- `reset_default_clip_rect_2`: Stores four default bounds values, 160, 160, 128 and 128, into a shared rectangle record.
- `update_boundary_values_2`: Compares four 16-bit values from a global structure against two input values and updates the structure entries if the inputs are outside the current bounds.
- `calculate_distance_metrics_2`: Performs arithmetic calculations using two input values to update several global variables and a value in a memory table based on the difference between the inputs.
- `frag_bra_8`: Short code fragment that jumps to another point in the code.
- `frag_bra_9`: Short code fragment that jumps to another point in the code.
- `frag_bra_10`: Short code fragment that jumps to another point in the code.
- `update_audio_parameters_2`: Calls another routine, stores a pointer in a global variable, uses a loop to copy bytes from a table to a calculated destination, clears several audio volume registers, and sets a bit in a hardware register.
- `check_and_initialize_audio_2`: Sets a 16-bit word in a global variable to zero, increments a byte in a table, and if the byte is not less than a comparison value, clears a byte in a table and calls another routine if a byte at a specific location is non-zero.
- `frag_bsr_bra_6`: Short code fragment that calls another routine, then jumps to another point in the code.
- `initialize_audio_channel_periods_2`: Sets the period registers for four audio channels by loading the address of the hardware registers into a5 and calling a routine that processes a structure in a6 containing the new period values.
- `initialize_audio_channels_4`: Fetches a byte from a table using an index, performs bit shifts and adds a constant, clears a 16-bit word in a global variable, and then calls initialize_audio_channels four times with different hardware register addresses.
- `update_entity_attributes_and_dispatch_2`: Loads a pointer from a table into a6, extracts a type index from the first two bytes, and uses it to fetch new attribute data from a table pointed to by a3.
- `update_structure_and_dispatch_command_2`: Calculates a new value by adding a constant to a field in a structure, stores it in two different offsets, and updates a hardware register with a value from a table.
- `frag_bsr_bra_7`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_8`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_hardware_and_dispatch_command_2`: Searches a table to find a specific entry, updates a DMA control register and several fields in a structure, and then branches to a subroutine based on the lower nibble of a byte at offset 2 of that structure.
- `initialize_audio_channels_2`: Sets up the four audio channel registers by loading values from a table, enables DMA by writing to the DMA control register, and performs two delay loops before executing complex logic to update a global 16-bit value based on a byte and a table lookup.
- `dispatch_structure_operation_by_nibble_2`: Reads a nibble from offset 2 of a structure in a6 and branches to different subroutines based on its value, while also copying a word from offset 16 of a6 to offset 6 of a structure in a5.
- `frag_move_rts_4`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then returns.
- `calculate_table_index`: Divides a byte fetched from a PC-relative address by 3, and if the result is 0 or 2, branches; otherwise, it masks a byte from offset 3 of the structure in a6 and branches.
- `frag_moveq_move_andi_bra_2`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_move_bra_9`: Short code fragment that copies the memory at an offset from a6 to d2, then jumps to another point in the code.
- `search_table_for_value`: Calculates a starting address in a table based on a byte from a structure, then loops up to 37 times to find a word that is less than or equal to a threshold, storing the result in a destination offset.
- `frag_move_rts_5`: Short code fragment that copies d2 to the memory at an offset from a5, then returns.
- `decrement_structure_value`: Checks if a global variable is zero, then subtracts a masked byte from offset 3 of a structure in a6 from a word at offset 16 of that structure, potentially clamping the result to a specific value, and finally stores a masked version of the result into offset 6 of a structure in a5.
- `increment_structure_value`: Checks if a global variable is zero, then adds a masked byte from offset 3 of a structure in a6 to a word at offset 16 of that structure, potentially clamping the result to a specific value, and finally stores a masked version of the result into offset 6 of a structure in a5.
- `search_table_for_value_2`: Iterates through a table using a value from offset 18 of a structure in a6 as an index, compares table entries to a value from offset 0 of the structure, and stores the found or calculated value into offset 24 of the structure while updating offset 22 based on specific conditions.
- `frag_clr_rts_2`: Short code fragment that clears the memory at an offset from a6, then returns.
- `frag_move_beq_clr_2`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then copies d0 to the memory at an offset from a6, then clears the memory at an offset from a6.
- `update_clamped_value_2`: Checks if offset 24 of a structure in a6 is non-zero, then either adds or subtracts a byte from offset 23 to the word at offset 16 of the structure, clamps the result using offset 24, and performs a table lookup to update offset 6 of a structure in a5.
- `decrement_channel_period_and_lookup_2`: Subtracts the input value from the channel period and clamps it to the target when reached.
- `update_masked_byte`: If a byte at offset 3 of the structure in a6 is non-zero, it masks and combines bits from that byte with a byte at offset 26 of the structure and stores the result back to offset 26.
- `calculate_scaled_offset_2`: Reads values from a structure to calculate a scaled index, then either performs a table lookup or bitwise shifts to determine a value, which is finally added to a base value and stored in a destination.
- `frag_tst_bpl_move_sub_bra_3`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_10`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `calculate_scaled_offset_3`: Loads a byte from a table using an index in d0, multiplies d2 by the lower nibble of d0, shifts the result right by seven, and then adds d0 to a word at offset 16 of a structure in a6, otherwise it returns.
- `subtract_and_advance_phase_2`: Subtracts d2 from d0 and stores the result in offset 6 of a structure in a5, then advances a phase byte in the structure by an amount taken from the high nibble of a neighbouring byte.
- `frag_bsr_bra_9`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_10`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_clamped_value`: Updates a byte at offset 28 of the structure in a6 by masking and combining bits, then calculates a new value by multiplying a table entry by a masked byte, shifts it, and clamps the result between 0 and 64 before storing it at an offset from a5.
- `frag_tst_bpl_move_sub_bra_4`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_11`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `clamp_and_store_value_3`: Loads a byte from a table using an index in d0, multiplies d2 by the lower nibble of d0, shifts the result right by six, and then adds d0 to a byte at offset 19 of a structure in a6, otherwise it returns.
- `clamp_and_store_value_2`: Clamps the signed difference between d2 and d0 to a range between zero and sixty-four, stores the result as a word at offset 8 of a structure in a5, and updates a byte at offset 29 of a structure in a6 by adding the high nibble of the byte at offset 28 shifted right by two.
- `process_offset_and_pointer`: If a byte at offset 3 of a structure in a6 is not zero, it stores that byte at offset 32 of the structure, shifts the value left by seven, and if it is less than a word at offset 8 of the structure, subtracts it from that word and adds a shifted version of the value to a longword at offset 4 of the structure.
- `frag_move_rts_6`: Short code fragment that copies 1 to the memory at an offset from a6, then returns.
- `decrement_and_clamp_byte_3`: Extracts the high nibble from a byte at offset 3 of a structure in a6, and if it is not zero, adds it to a byte at offset 19 of the structure, clamping the result to a maximum of sixty-four, then stores the result in a word at offset 8 of a structure in a5.
- `decrement_and_clamp_byte_2`: Subtracts the low nibble of the byte at offset 3 of the structure in a6 from the byte at offset 19, clears that byte to zero when the result is negative, and stores the resulting byte as a word at offset 8 of the structure in a5.
- `decrement_and_store_byte`: Decrements a byte value from a structure and stores it in a global variable, then clears two subsequent bytes in the same global variable area.
- `clamp_byte_and_store_2`: Reads a byte from offset 3 of the structure in a6, clamps it to a maximum of sixty-four, stores the clamped value as a byte at offset 19 of that structure, and stores it as a word at offset 8 of a structure in a5.
- `calculate_and_store_index`: Calculates a value by adding the lower nibble of a byte from a structure in a6 to the result of multiplying the high nibble of that same byte by ten, and if the result is less than sixty-four, stores it in a global variable and sets a subsequent byte to two hundred fifty-five.
- `frag_move_beq_lea_clr_rts_2`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then loads the address of a location in the code into a4, then clears the memory at a4, then loads the address of a location in the code into a4, then copies d0 to the memory at a4, then returns.
- `dispatch_structure_command_byte`: Reads a byte from offset 2 of a structure in a6, masks the lower nibble, and branches to different routines based on the value.
- `frag_moveq_move_lea_movea_rts_2`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then copies a6 to the stack, then loads the address of a location in the code into a6, then copies d0 to the memory at a6, then copies the top of the stack to a6, then returns.
- `dispatch_high_nibble_command`: Extracts the high nibble from a byte at offset 3 of a structure in a6 and performs a series of branches to different routines based on the value of that nibble.
- `frag_bset_rts_2`: Short code fragment that sets bit 1 of a fixed memory address, then returns.
- `mask_and_merge_byte_2`: Masks the low nibble of a byte at offset 31 from a pointer in a6 and then merges the low nibble of a byte at offset 3 from the same pointer into it.
- `clear_low_nibbles_of_byte_2`: Clears the low nibble of a byte at offset 30 from an address in a6 and then ORs in the low nibble of another byte from the same base.
- `store_masked_byte_2`: The routine reads a byte from an offset in a structure pointed to by a register, masks it with a constant, and stores the result into a different offset in the same structure.
- `decrement_counter_and_store`: Checks if a global variable is zero and if a masked byte from offset 3 of a structure in a6 is non-zero, then decrements a byte at offset 34 of the structure and, if it reaches zero, stores a value from offset 33 into a global variable.
- `frag_move_bra_12`: Short code fragment that copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_lsr_rts_2`: Short code fragment that copies a location in the code to d0, then shifts d0 right by 4, then copies d0 to the memory at an offset from a6, then returns.
- `mask_and_merge_nibble_2`: Masks the low nibble of a byte at offset 30 from a pointer in a6 and merges the high nibble from a byte at offset 3 of the same pointer, storing the result back at offset 30.
- `decode_and_copy_title_data_2`: Reads a nibble from a table entry and, if a global variable and the entry satisfy a modulo condition, copies two 32-bit words and one 16-bit word from the table through a pointer, performs two delay loops, writes a modified value to a global variable, and then copies two more 32-bit words from the table.
- `frag_lea_tst_bne_moveq_move_andi_bra_3`: Short code fragment that loads the address of a location in the code into a4, then tests the memory at a4, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_lea_tst_bne_moveq_move_andi_bra_4`: Short code fragment that loads the address of a location in the code into a4, then tests the memory at a4, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_moveq_move_andi_cmp_bne_clr_rts_2`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a location in the code, then branches if the last result was not zero, then clears the memory at an offset from a6, then copies 0 to the memory at an offset from a5, then returns.
- `branch_on_masked_nibble`: Masks a byte from offset 3 of a structure in a6, compares it to a global variable, and if it matches and the word at offset 0 of the structure is non-zero, pushes d1 to the stack and branches to another routine.
- `increment_masked_byte`: Masks a byte from offset 3 of a structure in a6, checks a global variable, and if valid, increments the masked value and stores it into a global variable.
- `set_masked_bits`: Masks a byte from offset 3 of a structure in a6, shifts it left by 4, and performs a bitwise OR with a byte at offset 31 of the structure.
- `clear_sprite_entry_2`: Reads a nibble from a byte in a structure and uses it as an index into a global table and adds the result to another byte in the same structure.
