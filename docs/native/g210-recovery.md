# Game 210 (Test Drive): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00210). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901776 top-level bytes in 92 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 547 of 547 named (0 author symbols), 547 of 547 described (134 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1320 of 1320 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 110144 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8821 of 8824 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 236886 |
| code | 28584 |
| data | 578907 |
| padding | 23095 |
| unused | 34304 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| AddBuffers | 2 | 117 | 2 |
| KickRAMTest | 3 | 457 | 30 |
| AddMem | 13 | 320 | 18 |
| EndIF | 1 | 2 | 1 |
| IF | 2 | 117 | 2 |
| Test Drive.adf:TD | 3 | 7781 | 473 |
| Test Drive.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 18 |
| Road.Shp (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- Test Drive/Test Drive.adf:Cars/Vette.ST.Shp: high-entropy data file (7.72 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/VetteDash.Shp: high-entropy data file (7.83 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/VetteLogo: high-entropy data file (7.75 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/P911TLogo: high-entropy data file (7.67 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/LotusGas: high-entropy data file (7.91 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/P911TGas: high-entropy data file (7.92 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/Countach.ST.Shp: high-entropy data file (7.91 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/Lotus.SB: high-entropy data file (7.70 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/P911T.SB: high-entropy data file (7.70 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/P911TDash.Shp: high-entropy data file (7.86 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/RossaLogo: high-entropy data file (7.61 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/RossaGas: high-entropy data file (7.91 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/VetteGas: high-entropy data file (7.91 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/CountachGas: high-entropy data file (7.91 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/Rossa.ST.Shp: high-entropy data file (7.71 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/CountachDash.Shp: high-entropy data file (7.86 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/LotusLogo: high-entropy data file (7.64 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/Lotus.ST.Shp: high-entropy data file (7.80 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/RossaDash.Shp: high-entropy data file (7.84 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/Rossa.SB: high-entropy data file (7.70 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/P911T.ST.Shp: high-entropy data file (7.78 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/Countach.SB: high-entropy data file (7.70 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/Vette.SB: high-entropy data file (7.70 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/CountachLogo: high-entropy data file (7.62 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Cars/LotusDash.Shp: high-entropy data file (7.83 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Pics/Gas.Shp: high-entropy data file (7.69 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Pics/EndGame: high-entropy data file (7.61 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Pics/Endgame.Shp: high-entropy data file (7.75 bits/byte), classified as packed data
- Test Drive/Test Drive.adf:Pics/TitleCar.Shp: high-entropy data file (7.66 bits/byte), classified as packed data
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Test Drive/Test Drive.adf:Pics/Road.Shp: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 110144 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `stop_disk_dma_and_enable_ints_then_call` (TD): Points a1 at the custom chip base, writes 4098 to DSKLEN and 16384 to INTENA, loads a6 and a1 from a5/a4, calls the library vector at -24, then branches away. Kind: track/disk check. Pass path: the branch or return following the routine, likely continuing normal execution if the check passes.
- `select_disk_drive_and_set_dma_length` (TD): Writes 127 then the byte at offset 25 of a5 to CIAB port B to pick the drive, then writes 16384 to the disk DMA length register and returns. Kind: track/length check. Pass path: an address.

## Functions


### AddBuffers

- `allocate_and_init_buffer_list`: Calls library routines through a5 to find the largest buffer size, allocates a buffer, fills it with incrementing longword tags, sets up a stack frame and copies sixteen longwords into the buffer structure, returning zero or minus one.
- `return_d1_in_d0_and_branch`: Moves d1 into d0 as the return value and branches forward into another routine's code.

### KickRAMTest

- `entry_jump_to_library_vector`: Program entry that jumps to the library vector at absolute address 104, transferring control without returning.
- `test_byte_write_readback`: Writes byte 90 to the address passed on the stack, reads it back into d3, and returns 1 in d0 if equal or 0 otherwise.
- `check_ram_at_two_addresses`: Tests writability of the two RAM addresses passed to the byte test routine, then calls the error routine with 0 on success or 5 on failure.
- `clear_stack_frame_and_install_trap`: Derives a base from the caller return address, clears 40 longwords there, saves the stack pointer and trap vector, then calls the install routine.
- `call_ram_test_hook_and_cleanup`: Tests the longword at offset minus 32766 from a4; if nonzero, loads it into a6 and calls it, then pushes the word at offset 8 of the frame and calls a routine before unlinking and returning.
- `open_dos_and_mathffp_libraries`: Opens dos.library and mathffp.library by name, stores their bases in a4-relative globals, and on failure exits via the saved stack pointer; otherwise parses a command line into tokens and sets up library vectors.
- `move_d0_to_d0_noop`: Copies d0 onto itself, an empty one-instruction routine that changes no register and falls through to the following code.
- `kickram_test_cleanup_and_exit`: Runs ten initialization calls, invokes optional callbacks and frees two pointers, then either allocates buffers and calls a routine with the argument or frees a pointer, returning the argument in d0 and restoring a7 from a saved value.
- `load_sevenfff_into_d0_then_branch`: Loads the immediate value 32767 into d0 and branches forward into the middle of another routine, with no loop or call of its own.
- `bounded_string_copy_return_dest`: Scans the destination for its terminator, copies up to the given count of bytes from source to destination, writes a final zero byte, and returns the destination pointer in d0.
- `copy_string_to_destination`: Loads destination pointer from stack into a0 and source pointer into a1, copies bytes from source to destination until a zero byte is copied, returns destination pointer in d0.
- `measure_string_length`: Loads a string pointer from the stack into a0, scans bytes until a zero terminator, then returns the length in d0 as the difference between the end and start pointers.
- `copy_string_with_padding`: Copies bytes from source a1 to destination a0 up to a count in d1 or until a zero byte, then pads the remaining count with zero bytes and returns the destination pointer in d0.
- `release_memory_node_by_index`: Looks up a six-byte node in a table indexed by the word argument, checks bounds and presence, optionally frees the pointed block via a call, clears the node and returns zero or minus one with an error flag set.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### AddMem

- `addmem_parse_args_and_allocate`: Saves registers, opens the current directory, splits the command line into whitespace-separated arguments, calls allocation routines, stores the resulting memory pointers in globals and returns zero.
- `addmem_exit_cleanup_and_reply`: Restores the saved stack pointer, replies the startup message to the caller, forbids and frees the allocated memory block, then returns the original argument value.
- `open_and_close_addmem_library`: Loads the AddMem library by name from offset 92 of a4, calls its open vector, then calls its close vector on the same name pointer and returns.
- `open_dos_library_store_base`: Loads the dos.library name pointer into a1, calls the exec open-library vector, stores the returned base at absolute address 8 and returns if nonzero.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `print_usage_and_exit`: Saves the stack pointer, pushes a null and a usage string, calls the print routine, then calls the exit routine with status 10 and restores the stack.
- `init_memory_node_and_register`: Saves registers, fills a memory node at the first argument with size fields, links it to the second argument, then calls three subroutines to register it.
- `parse_hex_string_to_integer`: Reads characters from the string in a2, accumulating hex digits into d3, and returns the parsed value in d0 when a null byte is reached.
- `hex_digit_accumulate_and_report`: Accumulates a hex digit into d3 shifted left four bits, and on a non-hex character prints a message and returns the value in d0.
- `add_memory_report_available_bytes`: Saves d2-d4/a2, reads a type argument and a parameter block, formats the block's start and end addresses, computes the byte difference and prints it with the bytes-added message.
- `call_printf_with_format_and_arg`: Pushes d2, loads the format string and one argument from the stack, calls the printf-style routine at the absolute vector, then cleans the stack and restores d2.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_add_head`: Library stub: loads its arguments and the exec library base, calls AddHead and returns its result to the caller.
- `format_string_via_rawdofmt_then_output`: Calls exec RawDoFmt with a format template and arguments into a 140-byte stack buffer, measures the resulting string length and passes it to the output routine.
- `store_d0_byte_to_a3_postincrement`: Writes the low byte of d0 to the memory pointed to by a3, then increments a3 by one and returns to the caller.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.

### EndIF

- `return_zero_in_d0`: Clears the entire d0 register to zero and returns immediately, providing a zero return value to the caller.

### IF

- `if_command_allocate_and_install_handler`: Scans a linked list for the largest value, allocates memory via library calls, fills a structure with command words, copies a sixteen-longword block, installs a handler pointer and returns zero or minus one.
- `copy_d1_to_d0_and_branch`: Moves the longword in d1 into d0 and then branches to another routine, so d0 returns the value passed in d1.

### Test Drive.adf:TD

- `program_entry_branch_to_startup`: The program entry point: an unconditional branch that transfers control to the main startup code at a fixed offset.
- `zero_filled_data_area`: A single zero word that decodes as an ori.b instruction; the bytes are data referenced by address, not executable code.
- `zero_word_data_slot`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by address, not executable code.
- `two_zero_word_data_slots`: Two consecutive zero words decoding as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `main_game_loop_setup_and_input_dispatch`: Initializes game state and hardware, then loops reading CIA keyboard input, dispatching keys, updating timers and DMA, until exit conditions end the loop.
- `call_car_data_update_via_object`: Opens a stack frame, loads the object pointer from the global at offset -31822 in a4, pushes the longword at offset 52 of that object, calls two library routines, then returns.
- `load_cars_text_into_pointer_array`: Opens a text file named Cars.Txt, reads its line count, stores each line pointer into a global array, null-terminates each line and sets an error flag on failure.
- `advance_string_pointer_past_line`: Takes a pointer to a string pointer in a2, advances it past characters until a null or newline, skips the newline, and returns the updated pointer in d0.
- `update_global_state`: Stores a bitmask from a global variable into a memory location and updates that global variable with a new value.
- `check_memory_value`: Checks if a value at a memory location is zero and, if so, updates a global variable.
- `keyboard_command_dispatch`: Polls the keyboard, reads a character, and branches on it to toggle music, sound, detail and other game flags, waiting on a key release and restoring the saved flag.
- `call_56f8_and_store_a5_offset`: Calls the routine at the pc-relative address, then copies the word at offset minus eight from a5 into the word at offset minus five-b-five-zero from a4, and branches away.
- `check_byte_4d_and_set_flag`: Compares the byte at offset minus five from a5 with the value seventy-seven; if equal and the word at offset minus seven-b-three-two from a4 is zero, sets that word to one, then branches.
- `clear_flag_and_call_if_zero`: Clears the word at offset minus seven-b-three-two from a4, tests it, and if still zero calls the routine at offset minus seven-e-d-e from a4 before branching away.
- `check_key_53_and_set_flag`: Compares byte at offset minus five from a5 with 83, and if equal and the word at a4 minus 31930 is zero, sets that word to one.
- `clear_flag_and_call_if_zero_2`: Clears the word at a4 minus 31930, tests it, and if still zero calls the routine at pc offset 9514 before branching onward.
- `check_key_44_and_set_flag`: Compares byte at offset minus five from a5 with 68, and if equal and two words at a4 offsets are zero, sets the second word to one.
- `reset_sound_volume_and_pan`: Clears the word at offset -29898 from a4, reads it back into d0, multiplies by -1, then stores the low byte to two adjacent bytes at -29901 and -29902 from a4 before branching onward.
- `check_key_4f_and_set_flag`: Compares byte at -5 from a5 with 79, and if equal and the word at -22506 from a4 is zero, sets the word at -23374 from a4 to 1 unless already nonzero, then branches.
- `clear_flag_word_and_branch`: Clears the word at offset -23374 from a4 to zero and then branches to the shared continuation point, undoing the flag set by the preceding check.
- `set_flag_if_a5_byte_equals_18`: Compares the byte at offset minus five from a5 with decimal 18 and, when equal, writes the word value 1 to the a4-relative address minus 31928, then releases the a5 frame and returns.
- `check_test_drive_title_and_store_params`: Calls several library routines to read the title text and two parameters, compares the title result and parameter difference against limits, and on success stores the two words into a4-relative memory and sets a flag word.
- `adjust_counters_and_return`: Increments d5 by two, decrements the word pointed to by a4, copies d4 into a0, increments a1, adds three to d5, moves d0 into d5, then releases the a5 frame and returns.
- `scan_iff_chunks_for_ilbm`: Calls a library function with the second argument, then checks whether the first longword at the given pointer is FORM and the third is ILBM, and if so passes the chunk pointer to the chunk parser.
- `advance_chunk_pointer_past_size`: Loads the chunk base pointer, adds the caller-supplied offset, reads the chunk size at offset four, adds the current offset and nine, clears bit zero, and stores the aligned next-chunk pointer back through the pointer argument.
- `walk_iff_chunks_find_bmhd_bodys`: Sets up a stack buffer and chunk offset, compares the offset against the FORM size plus eight, loads each chunk identifier, and dispatches BMHD, BODY, CMAP and CMP2 chunks to the matching parsers while advancing the offset.
- `call_chunk_parser_with_offset_and_frame`: Pushes the address of a local frame buffer, then calls the chunk parser with the base pointer plus the current offset, cleans the stack and branches back to the dispatcher.
- `call_chunk_parser_with_base_offset`: Loads the base pointer plus the current offset into a0, pushes it, calls the chunk parser, adds four to the stack pointer and branches back to the dispatcher.
- `set_next_chunk_pointer_past_header`: Computes the base pointer plus the current offset plus eight and stores it as the next chunk pointer in the local frame, then branches back to the dispatcher.
- `set_second_chunk_pointer_after_bmhd`: Adds the base pointer at offset 8 of a5 to the offset at -8 of a5 plus 8, storing the result at -54 of a5, then branches to the shared chunk dispatcher.
- `branch_to_chunk_dispatcher`: Branches unconditionally to the shared chunk dispatcher at the same target used by the other chunk entry stubs, without touching any registers or memory.
- `dispatch_chunk_id_by_subtraction`: Subtracts successive constants from d0 to identify the chunk type, branching to the matching handler for BMHD, BODY, CMAP or CMP2, otherwise falling through to the default branch.
- `scan_iff_chunks_for_bmhd_body_pal`: Repeatedly calls the chunk reader on the file at offset 8, stores the chunk id, and records pointers to BMHD, BODY and CMAP chunks before calling the image decoder.
- `fill_palette_and_decode_planar_pixels`: Copies 32 words of default palette from a4-relative table into the destination, then reads up to 32 pixels from the source and packs three bytes into 12-bit color words.
- `unpack_three_byte_pixels_to_palette`: Reads three bytes from a2 per iteration, combines their nibbles into a 12-bit color word, and stores it at the destination indexed by d5 until d5 reaches d4.
- `read_byte_count_and_store_palette_entry`: Calls a library routine to read a byte, clamps it to 32, stores a word from the parameter block into a global, then copies up to 32 three-byte colour entries into a global palette buffer.
- `convert_three_bytes_to_palette_word`: Reads three consecutive bytes from a2, packs their nibbles into a 12-bit colour word, writes it to the palette buffer indexed by d5, and repeats until d5 reaches d4.
- `init_stack_frame_and_jump_to_loop`: Creates a stack frame reserving 32 bytes, clears the local word at offset minus 8, then branches forward into another routine's loop body.
- `copy_tile_ptrs_and_start_blit`: Copies four-byte tile pointers from the graphics structure into a local array indexed by a counter, then computes a row byte width and starts the blit loop.
- `reset_column_index_and_branch`: Clears the local column counter at offset minus four from a5 and branches forward to the inner loop test.
- `blit_tile_row_and_finish`: Calls the tile blit routine for each column, advances the row counter, and when rows are exhausted calls the cleanup and blit-finish routines before returning.
- `copy_sprite_ptrs_from_two_objects`: Reads a byte count from the first object's sprite structure, copies that many longword pointers from two objects into local stack arrays, then branches into the shared processing loop.
- `gather_object_sprite_pointer_arrays`: Loops over d6 up to d5, copying longword sprite pointers from the primary object and optional secondary object into two stack arrays, then sets d4 to seven and selects a return value.
- `repeat_sprite_blit_seven_times`: Calls the blit routine with the two stack pointer arrays, the count in d5 and index d4, decrements d4, repeats while non-negative, then restores d4-d6 and returns.
- `blit_plane_mask_combine_loop`: For 200 rows and 9 columns, clears masked bits in destination bitplanes and ORs in source bits from the second plane set when the flag word is zero, advancing the destination offset by 280 each row.
- `parse_bytecode_operand_stream`: Walks a longword operand stream, using values 0 to 5 as mode selectors and calling the operand fetch routine for other values, storing results into the output array until a zero selector ends it.
- `fetch_operand_and_store_result`: Pushes the current operand and the caller's argument, calls the operand fetch routine, then stores the returned pointer into the output slot when the mode word equals one.
- `store_second_arg_into_next_slot`: When d4 equals 2, calls the routine at 20(a5) with a2, stores the result through the pointer at 16(a5) and advances that pointer by four bytes, returning the result in d0.
- `clear_local_and_restore_registers`: Clears the word at -6(a5), tests d4, and if d4 is zero pops d4 and a2 from the stack, releases the a5 frame and returns.
- `allocate_eight_byte_node_for_arg`: Calls the library routine at -32376(a4) with the argument at 8(a5), and if the returned pointer is nonzero stores the argument and the value 8 in its first two longwords before returning it in d0.
- `check_pointer_delta_against_limit`: Clears a local long, tests the pointer argument at offset 8 of the frame, and if non-null loads it into a0 and a1, computes the difference between the first two longs and compares it with the limit at offset 12, branching accordingly.
- `advance_pointer_and_clear_flag`: Adds the pointer argument to its own first long into a local, adds the value at offset 12 to the pointer's second long, clears a word at a4 minus 32286, and returns the local long.
- `compute_first_minus_second_long`: Saves a2, loads the pointer argument from offset 8 of the frame into a2, returns the difference of its first long minus its second long, then restores a2 and returns.
- `compute_bitmap_size_from_header`: Reads width and height words from the structure pointed to by the first argument and returns their product plus sixteen as a long in d0.
- `test_pointer_and_clear_local`: Tests the long at offset twelve of the frame; if zero, clears the local long at offset minus eight, then branches onward to shared exit code.
- `copy_and_or_rows_into_buffer`: Computes a size, allocates a buffer, copies four longs from the source header, then ORs every other word of each row into the destination and returns the buffer pointer.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_stack_frame_return`: Sets up and immediately tears down an a5 stack frame, saving and restoring d4, d5 and a2 without touching any other memory, then returns.
- `init_blitter_control_registers`: Loads the custom chip base into a6, calls a helper, then clears blitter modulo registers and sets control registers to fixed values before returning.
- `set_dma_control_register`: Writes the value 1024 into the custom chip register at offset 150 from a6, then returns to the caller.
- `count_vblank_and_copper_events`: Pushes d0 onto the stack, increments a counter at offset -31858 from a4, clears d0, tests bit 6 of byte at offset 2 from a6 and conditionally increments a second counter at offset -31850 from a4 before branching.
- `increment_d0_until_ten_then_restore`: Increments the low word of d0, compares it with ten, branches back if less, then pops the saved d0 from the stack and returns to the caller.
- `read_dmaconr_register`: Loads the custom chip base address into a0, reads the DMACONR register at offset 2 into d0 and returns it to the caller.
- `setup_sprite_blit_params_100`: Computes a0 destination from d0/d1 grid coordinates, stores -1 in a6 offsets 68 and 70, saves a0 at 84, sets 64 to 256, and stores d1 and d3-derived values at 102 and 88.
- `setup_sprite_blit_params_1ff`: Like the 256 variant but sets a6 offset 64 to 511 and negates d2 before calling the blit helper, then stores the same a6 fields and returns.
- `read_sprite_position_pair`: Loads the word at offset 8 of a0 into d0 and the word at offset 10 into d1, then returns to the caller.
- `subtract_sprite_offsets_from_d0_d1`: Subtracts the word at offset 4 of a0 from d0 and the word at offset 6 of a0 from d1, then returns to the caller.
- `copy_five_longs_to_a4_buffer`: Copies five longwords from the pointer passed on the stack into the buffer at a4 minus 29714, advancing both pointers, then returns.
- `call_d44_with_a6_saved`: Saves a6 on the stack, loads the custom chip base into a6, calls the routine at d44 relative to pc, restores a6 and returns.
- `set_playfield_window_bounds`: Saves d0-d3, loads d0=0, d1=200, d2=0, d3=320, calls the window setup routine, then restores d0-d3 and returns.
- `store_window_rects_masked`: Writes d0, d1, d2 and d3 into four consecutive words in the a4-relative data area, then clears the low four bits of the d2 and d3 words.
- `open_two_library_bases`: Calls the open-library routine twice with two different name strings, storing each returned base pointer into a4-relative globals, then returns.
- `call_two_cleanup_functions`: Pushes two global pointers from the a4 base onto the stack and calls the same subroutine for each, then returns.
- `allocate_and_init_double_buffers`: Calls two setup routines, allocates two 32-byte buffers and ten 8000-byte buffers, initializes two display structures with 320 by 200 dimensions, and links them.
- `free_buffers_and_disable_dma`: Frees the main buffer, releases ten 8000-byte buffers, frees two 32-byte buffers, tears down two display structures, and writes to DMACON to disable DMA.
- `allocate_buffer_and_store_word_arg`: Allocates a zero-initialised buffer via a library call, saves the pointer in a local, then passes the sign-extended word argument and that pointer to a second routine before returning.
- `install_vblank_interrupt_handler`: Stores the a4 base pointer globally, writes interrupt vector bytes and handler pointers into a4-relative memory, then requests interrupt level 5 with the handler structure address.
- `request_interrupt_level_five`: Pushes the a4-relative handler structure address and interrupt level 5 onto the stack, calls the interrupt setup routine, cleans the stack and returns.
- `read_ciaa_fire_button_bit`: Reads the CIAA port A byte, shifts it right by seven bits, masks to one bit and inverts it, returning the fire button state in d0.
- `read_joystick_direction_index`: Reads JOY1DAT, combines the low two bits with bits six and seven shifted down, then uses that index to fetch a direction byte from a table relative to a4 into d0.
- `zero_filled_data_area_2`: Contains a single zero word that decodes as an ori.b instruction; this is zero-filled data referenced by address, not executable code.
- `poll_input_and_trigger_sound`: Increments the longword at a1, calls two subroutines, and when the second returns nonzero and a mode word equals one, stores minus one to a state word, compares two coordinate pairs and plays a sound byte indexed by d0 through library calls.
- `init_game_resources_and_install`: Saves registers, calls setup routines storing results at offsets 12, 4 and 16 of a5, and on success installs an interrupt server with vector 32773 and a negative offset before returning zero.
- `load_a5_base_and_a4_a6_globals`: Loads a5 with the address of a nearby data block, then loads a4 from offset four of a5 and a6 from absolute address four, returning immediately.
- `clamp_negated_1e00_to_5000`: Loads the longword at offset 1e00 from pc, negates it into d0, and if the result is negative or at least 5000 replaces it with 5000 before returning.
- `return_address_of_1954_table`: Loads the effective address of the pc-relative location 1954 into a0, copies that address into d0 as the return value, and returns.
- `save_regs_call_six_subroutines`: Pushes d3 through d7 and a2 through a6 onto the stack, then calls six subroutines in sequence and branches onward without restoring the saved registers.
- `compute_and_store_relative_offset`: Saves d0/a0-a1, computes the byte distance between two program-counter-relative addresses, halves it, stores the result at the first address, then restores registers and returns.
- `run_race_loop_with_input_and_render`: Reads two arguments, saves registers, calls setup and input routines, toggles a flag, writes 127 to a5 offset 25, then repeatedly polls input, updates display and loops until a condition ends.
- `check_library_call_result_bit31`: Loads a program-counter-relative address into a0, calls a library function through a6 offset, tests bit 31 of the returned d0 and returns.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `track_loader_data_table`: Bytes are data decoded as code: a table of ori.b immediates encoding track and sector values, not executable instructions.
- `open_dos_library_and_read_file`: Opens dos.library, opens a file for reading, reads its contents into a buffer, closes the file and library, then returns the status word from the buffer.
- `start_trackdisk_read_request`: Saves d0, calls a setup routine, fills the trackdisk request block with the buffer pointer, length and offset, then issues the read command.
- `open_graphics_and_allocate_screen_buffer`: Calls the Amiga graphics library open function with -1, stores the returned handle in d7, opens the intuition library, saves its result to (a5), and copies both handles into the structure at offset 40 of a4.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `close_library_and_release_screen`: Loads the library base from offset 24 of a4 into a1, calls the close library function, then branches to a nearby routine before returning.
- `copy_field_and_branch_back`: Calls the routine at a nearby offset, copies the longword at offset 32 of a1 to offset 48 of a1, then branches to another nearby routine.
- `set_d4_8002_and_branch`: Loads the immediate value 32770 into the low word of d4, then branches to a shared continuation elsewhere.
- `set_d4_8003_and_branch`: Loads the immediate value 32771 into the low word of d4, then branches to a shared continuation elsewhere.
- `zero_d2_d1_set_d4_800a_branch`: Clears d2 with moveq, copies d2 into d1, loads 32778 into the low word of d4, then branches to a shared continuation.
- `load_d4_six_then_branch`: Loads the immediate value 6 into data register d4 and branches unconditionally to a shared continuation elsewhere.
- `load_d4_thirteen_then_branch`: Loads the immediate value 13 into data register d4 and branches unconditionally to a shared continuation elsewhere.
- `load_d3_one_then_branch`: Loads the immediate value 1 into data register d3 and branches unconditionally to a shared continuation elsewhere.
- `queue_sound_sample_playback`: Computes a sample offset from d0, d1 and d2, fills an audio channel structure at 24(a4) with volume 5, period 32777, pointers from a5 and the computed offsets, then calls the audio device and returns whether the channel status byte is nonzero.
- `open_disk_resource_and_init_request`: Opens the disk.resource library, stores the base at offset 20 from a5, fills a request block from a4 fields with command 5, length 86 and a callback pointer, then returns zero in d0.
- `poll_disk_io_and_enable_audio_dma`: Loops calling the disk device check and read routines until completion, then writes DMACON and ADKCON to enable audio DMA channels and jumps back to the main loop.
- `set_audio_filter_bit_via_library`: Reads byte at offset 24 from a5, sets that bit in d0, calls the library vector at offset -318 from a6, and returns.
- `stop_disk_dma_and_enable_ints_then_call`: Points a1 at the custom chip base, writes 4098 to DSKLEN and 16384 to INTENA, loads a6 and a1 from a5/a4, calls the library vector at -24, then branches away.
- `branch_to_shared_cleanup`: Executes a single unconditional branch to the shared code at the target address, transferring control without touching any registers.
- `toggle_drive_select_bits`: Reads byte at offset 25 from a5, clears bit (d0+3) and sets bit 2 when d1 is zero, then writes the result back to offset 25 from a5.
- `select_disk_drive_and_set_dma_length`: Writes 127 then the byte at offset 25 of a5 to CIAB port B to pick the drive, then writes 16384 to the disk DMA length register and returns.
- `start_disk_dma_read_with_sync`: Calls the disk select setup, delays, enables disk and interrupt registers, stores a0 as disk pointer, sets sync word, computes length from d0 and calls two more routines.
- `set_audio_control_high_bit`: ORs the incoming d0 word with the high bit and writes the result twice to offset 36 of the structure pointed to by a1, then returns.
- `configure_channel_and_call_library`: Writes fixed control words to offsets 36 and 156 of a0, builds a bit mask from a1 byte 24, loads a1 from its first longword and calls a library vector through a6.
- `scan_track_sector_for_sync_marker`: Combines masked longwords from a0 into a sync value, compares its high byte against a d6/d7-derived byte, then scans up to 1000 words for the sync marker and returns the count found.
- `crc_block_with_seed_and_poly`: Sets up an a5 frame, saves d3-d5, loads source pointer, length, seed and polynomial from the caller's argument block, calls the checksum loop, then restores registers and returns.
- `bitwise_crc_loop_over_words`: Processes d4 words from a0, shifting each bit through d0 and conditionally xoring the polynomial in d3, then masks d0 to sixteen bits and returns.
- `crc_wrapper_from_stack_args`: Creates an a5 frame, loads two longword arguments from the stack into d0 and d1, calls the checksum routine at a pc-relative address, then releases the frame and returns.
- `signed_32bit_multiply_abs`: Multiplies d0 by d1 as signed 32-bit values using 16-bit partial products, masks the result to 31 bits, negates it if the sign flags differ, and returns it in d0 while preserving d1-d4.
- `unsigned_32bit_multiply_high`: Multiplies d0 by d1 using 16-bit partial products, adjusts the high word for negative operands, and returns the combined product in d0 while preserving d1-d5.
- `signed_divide_setup_negate_dividend`: Saves d1-d3, sign-extends d0, records the sign in d2, negates d0 if negative, and branches onward with d3 set to the maximum positive value when d1 is zero.
- `signed_divide_32_by_16_with_sign_flags`: Divides d0 by the signed word in d1, builds a 32-bit quotient in d3 from the signed and unsigned division results, negates it if d2 is nonzero, restores d1-d3 and returns the quotient in d0.
- `integer_square_root_by_bisection`: Takes the absolute value of d0, then narrows a candidate in d1 by repeated division and averaging until it is at most the quotient, returning the integer square root in d1 and restoring d1-d2.
- `return_word_from_d1_in_d0`: Clears d0, copies the low word of d1 into d0 as the return value, then pops d1 and d2 from the stack and returns to the caller.
- `abs_word_d0_return`: Tests the low word of d0; if negative, negates that word, then returns to the caller.
- `abs_long_d0_return`: Tests the full 32-bit d0; if negative, negates the long value, then returns to the caller.
- `format_long_d0_as_decimal_string`: Converts the long in d0 to a signed decimal ASCII string in a4-relative buffers using repeated BCD doubling, strips leading zeros, prepends a minus sign when negative, and returns with d0 cleared.
- `interpolate_sine_table_value`: Saves d1-d2/a0, normalizes the byte in d0 into a table index, reads two adjacent signed words from the table at pc-relative address, interpolates between them using the fractional part of d0, and returns the result in d0.
- `dispatch_angle_to_sine_lookup`: Pushes a0, tests the signed word in d0, adds 90 to negative angles, then branches to the table lookup routine with the adjusted angle.
- `lookup_sine_table_word`: Subtracts 90 from d0, negates and masks it to 127, doubles it, loads the signed word from the pc-relative table at that offset into d0, restores a0 and returns.
- `signed_sine_table_lookup`: Pushes a0, loads the sine table address, negates d0 if negative, masks and scales the index, reads a longword from the table into d0, negates it, then branches to the shared return.
- `sine_table_lookup_and_pop_a0`: Masks d0 to seven bits, doubles it as a byte index, loads the longword from the table at a0 into d0, pops a0 from the stack and returns.
- `init_sfx_vbl_interrupt_system`: If the sound flag is clear, allocates a sound channel, stores a4 in globals, clears sound state longs, programs ADKCON, DMACON and INTENA, installs a level-3 interrupt vector and a sound structure, then marks the system initialized.
- `disable_dma_restore_copper_and_clear_flag`: If the enable flag at a4 offset is set, calls two subroutines, writes 1920 to DMACON and 15 to INTENA, restores a saved pointer to the copper list at address 112, then clears the flag.
- `set_interrupt_enable_mask_from_arg`: Reads the word argument, inverts it, masks to four bits, shifts left seven, stores it, and if the enable flag is set writes the mask with bit 15 to INTENA and then its complement to INTENA.
- `dispatch_sound_command_with_scaled_timer`: If the pointer argument is non-null, reads its word field, scales values below 100 by 1000, divides by a constant, and pushes the result plus pointer fields onto the stack before calling a sound routine.
- `init_sound_channel_entry`: Validates the pointer and channel index, calls the channel check and stop routines, then fills the 30-byte channel slot with the sample pointer, length, period, volume and reset markers.
- `stop_all_four_sound_channels`: Loops a counter from zero to three, calling the channel stop routine for each index, then restores d4 and returns.
- `stop_sound_channel_and_clear_dma`: Computes the channel slot and audio register base from the index, clears the active flag, disables that channel's DMA bit in DMACON, resets the slot pointers and clears the audio control register.
- `check_voice_slot_in_use`: Reads the word argument from the stack, multiplies it by 30, tests the longword at that offset in the voice table based at a4, and returns zero or minus one depending on whether it is nonzero.
- `set_voice_period_and_volume`: Indexes the voice table by the first stack word and the audio hardware registers by the second, writing optional period and volume values into the voice structure and the AUD registers.
- `store_voice_sample_pointer_and_length`: Multiplies the stack word by 30 to index the voice table at a4, stores the longword sample pointer at offset 26 and the masked length longword at offset 22.
- `zero_filled_data_area_4`: A single zero word that decodes as an ori.b immediate instruction; the bytes are data, not executable code, and are referenced by address.
- `audio_interrupt_service_routine`: Saves registers, reads interrupt enable and request registers, acknowledges audio interrupts, and for each active channel updates its period counter and restarts playback before returning with rte.
- `per_frame_audio_channel_update`: Copies active channel state into the audio hardware registers, advances each channel's period toward its target, and writes the resulting DMA and interrupt masks.
- `init_song_vblank_and_audio_channels`: If the song-active flag is clear, stores the base pointer, clears song state, sets two channel bytes and two pointers, then opens the audio device and starts playback.
- `stop_song_and_close_audio`: If the song-active flag is set, stops playback, calls the audio stop routine, closes the audio device and clears the song-active flag.
- `load_song_files_into_slots`: For four slots, either copies an existing song pointer or formats a filename, opens the file and stores the handle, then records the slot's track index.
- `flush_pending_sound_slots`: If the pending flag at offset -31536 from a4 is set, loops over four slots, calls the function pointer at -32370 for each non-zero slot entry, then clears the flag.
- `store_sound_pointer_and_shifted_word`: Saves the longword argument at offset 10 from a5 into -6106 from a4 after shifting the word argument at offset 8 left by 16 bits, and stores the raw pointer at -610e.
- `read_sound_state_word_if_enabled`: Returns zero in d0 unless the flag at -78cc from a4 is set, in which case it returns the word at -610a from a4.
- `conditional_setup_display_tables`: Checks two global flags at offsets from a4; if both are set, saves all registers, calls the setup routine, loads two table pointers and a count of six, then calls the table routine and restores registers.
- `reset_counter_and_call_check`: Saves all registers, tests a global counter at a4 offset, sets it to three if nonzero, calls a subroutine at pc-relative address, and repeats while its result is nonzero before restoring registers.
- `store_sprite_geometry_and_scaled_offsets`: If a global flag is set, copies a0 and four longs from a1 plus four words from a2 into globals, then stores d0 and its shifted and scaled multiples into a4-relative tables and sets a flag.
- `check_sound_state_return_flag`: Reads the word at offset minus thirty thousand eight hundred and ninety-two from a4; returns zero if it is zero or three, otherwise returns minus one in d0.
- `load_and_start_song_file`: Builds a song path from the given name, opens it, stores the file handle in a4-relative memory, sets up playback parameters and starts the music, or reports an error.
- `stop_song_and_release_handle`: If a song handle is stored in a4-relative memory, stops playback, waits for completion, closes the file, clears the handle and resets the audio channel.
- `clear_and_decrement_counter`: Clears a 16-bit word at an offset of 122 from the pointer in a5 and decrements a 16-bit value in a global variable.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `open_song_iff_with_stack_args`: Builds a stack frame, pushes the song filename and three pointers plus the value 32, calls a routine, then discards 16 bytes of arguments and returns.
- `clear_word_and_decrement_absolute`: Clears the word at offset 122 from a5 indexed by d7, then subtracts one from the word at the absolute long address a fixed address.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `play_test2_iff_song`: Pushes the Test2.Iff.Sng string pointer and two global pointers with values 22 and 3 onto the stack, calls the song player, then cleans the stack and returns.
- `play_testgas_iff_song`: Pushes the TestGas.Iff.Sng string pointer and two global pointers with values 22 and 3 onto the stack, calls the song player, then cleans the stack and returns.
- `play_endsuccess_iff_song`: Pushes the EndSuccess.Iff.Sng string pointer and two global pointers with values 22 and 3 onto the stack, calls the song player, then cleans the stack and returns.
- `load_highscores_and_update_table`: Opens the HighScores file, reads and validates entries, updates the score table, and returns a status word in d0 while restoring d4 and the a5 frame.
- `copy_printable_string_advance_source`: Copies bytes from the source pointer a3 to destination a2 while each byte is at least space, stops at a control or null byte, writes a null terminator, and returns the advanced source pointer in d0.
- `verify_and_clear_highscore_entries`: Reads the HighScores file into buffers, compares a computed checksum against the stored value, and either keeps the entries or clears eight name, score and text slots, returning a flag in d0.
- `format_highscore_table_entries`: Allocates a 500-byte buffer, formats eight high-score rows using the score and name arrays, computes the total byte length, writes the header and length text, then frees the buffer.
- `open_topaz_font_and_window`: Allocates a text attribute and a 100-byte rastport, opens the topaz font, sets the rastport font, creates a window, frees the attribute, calls the setup routine and opens the display.
- `close_window_and_free_rastport`: Calls the window close routine with the stored window pointer, frees the stored rastport memory, clears that global pointer and returns.
- `convert_and_append_string_to_buffer`: Calls a conversion routine on the string argument, sign-extends the result, then passes the global buffer pointer, the string and the length to an append routine and returns.
- `append_two_words_to_buffer`: Sign-extends two word arguments from the stack, pushes them with the global buffer pointer and calls a routine that appends both values to that buffer.
- `append_word_to_buffer`: Sign-extends the single word argument from the stack, pushes it with the global buffer pointer and calls a routine that appends the value to that buffer.
- `draw_rect_outline_restore_pen`: Saves the current pen color, sets pen to 1 and draws four lines connecting the four corner coordinates passed on the stack, then restores the original pen color.
- `draw_filled_rect_restore_colors`: Saves pen and fill colors plus two coordinates, sets pen to zero, fills the rectangle between the passed corners, then restores pen, fill and position.
- `draw_thick_rect_restore_colors`: Saves pen and fill colors plus two coordinates, sets pen to 31 and fill to 2, draws a rectangle expanded by eight pixels, then restores colors and position.
- `read_name_input_into_buffer`: Reads keyboard characters into a caller-supplied buffer, handling backspace by erasing the previous character on screen and enter by terminating the string, then returns after a timeout or abort.
- `insert_high_score_name_entry`: Displays the logo and name prompt, reads a name via the input routine, shifts the high-score table entries down and inserts the new score and name, then waits for a key.
- `show_credits_and_wait_for_key`: Clears two screens, draws each credits text line centered on screen, then waits up to a fixed time for a key press and returns a flag.
- `show_top_scores_and_wait_for_input`: Opens a screen, prints the Test Drive top scores and your score, waits for a key or mouse press, then closes the screen and returns a status word.
- `load_and_display_logo_shape`: Builds a logo.Shp filename, opens the file, loads it into memory, sets the logo palette, blits the logo bitmap and frees the file buffer.
- `load_logo_palette_or_fade_colors`: Builds a Logo.Pal filename, opens and reads palette entries, then writes each color to the custom chip color registers, optionally fading from a stored palette.
- `crc16_over_buffer_with_seed`: Builds a stack frame, loads buffer pointer, length and two word parameters from the caller's stack, then calls the checksum routine with those values and returns.
- `crc16_bitwise_block_checksum`: Saves registers, processes each word of the buffer through sixteen shift-and-xor bit steps using the seed and polynomial in d0 and d2, masks d0 to a word and restores registers.
- `init_crc_accumulator_and_jump`: Creates a stack frame with two bytes of locals, clears the word local at offset minus two from a5, then branches into the checksum routine's entry point.
- `poll_until_abort_or_count_reached`: Calls two library routines, increments the word counter at offset -2 from a5, compares it with the limit at 8(a5), and loops calling two more library routines until the limit is reached or a nonzero result ends the loop.
- `check_flag_and_set_global_word`: Calls a library routine, and if it returns nonzero stores the value 1 into the global word at -22492 from a4, then returns that word in d0.
- `draw_eight_row_text_block`: Formats a string into a buffer, loops eight times drawing text rows via library calls and a helper, then scales a global counter by forty and calls another helper before returning.
- `select_dashboard_gauge_graphics_by_speed`: Calls a random generator, divides the result by three to pick a gauge variant, stores it in the frame, sets two flags, and loads two graphics pointers from tables indexed by that variant.
- `pick_gauge_graphics_for_speed_over_105`: Compares the speed value at offset eight of the frame with 105 and, if greater, loads two graphics pointers from tables indexed by the stored gauge variant.
- `pick_gauge_graphics_for_speed_over_95`: Compares the speed value at offset eight of the frame with 95 and, if greater, loads two graphics pointers from tables indexed by the stored gauge variant.
- `select_car_sprite_ptrs_by_index`: Compares the long at offset 8 of a5 with 65; if below, loads two sprite pointers from the first table pair into a5 offsets -46 and -50, otherwise loads them from the second table pair, then branches away.
- `build_results_screen_with_speed_text`: Loads two sprite pointers by index, opens a 320x24 display, allocates 960 bytes, formats the average speed and time text into the buffer, and prints minutes and seconds when the time reaches 120.
- `format_minutes_seconds_when_over_59`: Checks the word at offset 12 of a5; if at least 60, divides it by 60 to get minutes and seconds, pushes both plus the format string, calls the text formatter and branches on.
- `print_time_and_random_tip`: Formats and prints two numbers and a text string via the library, then unless a flag is set picks a random entry from a table and prints it, finally frees the buffer and returns.
- `show_gas_picture_and_wait_input`: Loads the gas shape, opens a screen, draws the picture, waits in a loop for a key or button, then closes and frees the resources before returning.
- `close_gas_screen_and_restore_buffer`: Closes the gas screen, frees its bitmap and window, restores the previous screen pointer, and clears the input buffer words when the flag is set.
- `clear_32_word_array_if_flag`: Copies one byte between two a4-relative bytes, then if the word at a4-relative offset is nonzero clears 32 words of the a4-relative array using a5-relative loop counter.
- `setup_display_and_draw_screen`: Calls several a4 library vectors to initialise graphics, passes a6 structure offset to a setup call, then pushes screen dimensions and pointer to a draw call before finalising.
- `invoke_49fe_with_flag_10001`: Pushes the long immediate 65537 and the caller's first argument onto the stack, calls the routine at 49fe, then discards both arguments and returns.
- `load_file_with_mode_10003`: Sets up an a5 stack frame, pushes the constant 10003 and the caller's first argument, calls the file-loading routine, then cleans the stack and returns.
- `load_and_validate_pckd_resource`: Opens a file, allocates a buffer, reads a header, checks for the PCKD tag, loads the resource data into memory and stores the resulting pointers in globals.
- `load_non_pckd_resource_block`: Allocates a buffer, copies four header longs, reads the remaining resource bytes into it, frees the file handle and stores the buffer and size in globals.
- `free_five_a5_pointers_and_clear_globals`: Tests five pointer slots at negative offsets from a5, frees each non-null one via library or local free calls, then clears two a4 globals and returns the first cleared value in d0.
- `load_long_from_object_offset_eight`: Sets up an a5 frame, loads the pointer argument from the stack, reads the long at offset 8 of that object into d0, then releases the frame and returns.
- `init_stream_state_from_descriptor`: Clears two a4 globals, copies fields from the descriptor argument into a4 state slots, stores the second argument, and branches to the stream dispatcher with the descriptor type word.
- `poll_input_until_ffff_then_loop`: Calls the input routine, saves the word result in d4, compares it with minus one and exits if equal, otherwise pushes d4 and calls the second routine before looping back.
- `branch_to_shared_exit_4d5e`: Unconditionally branches to the shared exit code at a later address, transferring control without touching any registers or memory.
- `data_pointer_branch_to_shared_exit`: Reached through a data pointer, it unconditionally branches to the shared exit code at a later address without modifying registers or memory.
- `dispatch_word_offset_by_count`: Subtracts 2 from d0, compares it with 7, branches away if higher, doubles d0 and jumps through a word table indexed by the result.
- `restore_d4_unlink_and_return`: Pops d4 from the stack, releases the a5 stack frame and returns to the caller.
- `read_count_and_fill_pair_table`: Stores 99 in a4-relative memory, calls a routine to get a count, rejects values outside 0 to 256, then fills a table of word pairs from further calls.
- `init_two_word_buffers_and_clear_d4`: Writes the immediate value -257 to two words at negative offsets from a4, sets d4 to zero, then branches to another routine.
- `fill_two_word_arrays_from_random_source`: Loops while d4 is below a count at a4, calling a generator twice per iteration and storing each result into two parallel word arrays indexed by d4 times four.
- `combine_two_generator_calls_into_word`: Saves d4, calls a generator twice, keeps the first result in d4, shifts the second left eight bits, ORs them into one word, restores d4 and returns.
- `get_next_byte_from_buffer`: Tests the byte counter at offset -24640 from a4; if zero returns zero, otherwise decrements the counter, loads the buffer pointer, increments it, and returns the next byte in d0.
- `return_minus_one`: Loads the value minus one into d0 and returns immediately to the caller.
- `store_byte_if_below_limit`: Increments the counter at offset -24646 from a4, compares it with the limit at -24638, and if below stores the byte from the stack argument into the buffer pointed to by -24638's pointer, incrementing that pointer.
- `load_resource_10001_via_loader`: Pushes the constant a fixed address and the caller's first argument onto the stack, calls the shared loader routine, cleans up eight bytes and returns.
- `push_param_and_constant_call`: Builds a stack frame, pushes the long constant 65539 and the caller's first argument, calls a subroutine, then cleans the stack and returns.
- `load_and_attach_sound_resource`: Allocates a resource, opens a file, checks it, reads a size field, loads a second resource, calls a routine with the size and second argument, stores results in globals and frees the file handle.
- `push_global_and_call_75ce`: Pushes the longword at offset minus eighteen from a5 onto the stack, calls the routine at displacement 75ce relative to pc, then discards the pushed argument and branches onward.
- `call_two_globals_then_clear_counters`: Pushes and calls two routines using values at a5 offsets minus six and minus fourteen, then clears two longwords in a4-relative memory and returns the second cleared value in d0.
- `update_obfuscated_value`: If a certain 16-bit value in a structure pointed to by a4 is zero, it resets that value to a new value from the display pointer and sets a secondary offset to a maximum value.
- `rle_decode_into_output_buffer`: Reads run-length encoded bytes from the source pointer, expands them into the destination pointer up to the given count, and updates both pointers.
- `copy_literal_run_bytes`: Copies literal bytes from a2 to a3 while incrementing both pointers and d4, looping until d4 exceeds d5, then branches to the shared exit.
- `setup_repeat_run_count`: Skips the end marker, reads the repeat byte into d6, negates d5 to get the run length, increments it, clears d4 and branches to the copy loop.
- `copy_d6_bytes_to_a3_advance`: Increments the word at offset minus two from a5, copies byte d6 to the address in a3, advances a3 and increments d4, then loops while d4 is less than d5 before branching away.
- `store_a2_a3_into_frame_pointers`: Loads the pointer at offset eight from a5 and stores a2 there, then loads the pointer at offset twelve and stores a3, restores d4-d6 and a2-a3, releases the frame and returns.
- `clear_cia_interrupt_bit_seven`: Reads the CIA control register at the given long address, clears bit seven with a mask, writes the result back, then releases the a5 frame and returns.
- `store_result_to_57a6`: Calls the routine at offset 5402, then stores the returned word from d0 into the a4-relative memory location minus 57a6 and returns.
- `store_result_to_5aec`: Calls the routine at offset 5416, then stores the returned word from d0 into the a4-relative memory location minus 5aec and returns.
- `store_both_results_to_globals`: Calls the routine at offset 5402 and stores d0 to a4 minus 57a6, then calls the routine at offset 5416 and stores d0 to a4 minus 5aec before returning.
- `read_fire_button_state`: Reads CIAA_PRA, shifts bit 7 down, masks it to one bit and inverts it, returning the fire button state in d0.
- `decode_joystick_direction`: Reads JOY1DAT, combines the low two bits with bits 2 and 3 shifted right, indexes a direction table at pc and returns the byte in d0.
- `check_and_set_flag_word`: Clears a word in a4-relative memory, calls a routine if another word is zero, sets that word to all ones when needed and returns it in d0.
- `lookup_entry_via_table`: Builds a stack frame, loads a table pointer and a key from the caller's arguments, calls the table search routine, and returns the found pointer in d0.
- `search_sorted_table_by_key`: Scans a counted table of longs at a0 for the value in d0, then computes and returns the offset of the matching entry's data, or zero if absent.
- `return_zero_pointer_restore_regs`: Sets d0 and a0 to zero, restores d1 and a1 from the stack, and returns, signalling a failed table lookup.
- `open_console_and_input_devices`: Opens console.device and input.device, stores their handles in a4-relative globals, sets up a console window structure and message port, and saves the input device's reply port pointer.
- `close_console_and_input_devices`: Closes the console and input device handles stored in a4-relative globals, resets the console window fields, and releases the associated message ports.
- `get_value_and_dispatch_to_caller`: Calls a subroutine to obtain a value in d0, pushes it on the stack, then calls another subroutine with that value as its argument before returning.
- `read_byte_via_ioctl_into_word`: Builds a small parameter block, calls the file I/O routine with a one-byte buffer, and returns the byte as a word if the call reports success, else zero.
- `zero_filled_data_area_3`: A single zero word that decodes as ori.b #0,d0; this range is zero-filled data referenced by address, not executable code.
- `queue_event_if_type_one`: For a type-one event record, checks flag bit seven and a mask, then appends the event byte and word to two parallel arrays and clears the record type.
- `check_flag_return_ff_if_set`: Clears d0 to zero, tests the word flag at offset -23538 from a4, and sets d0 to -1 when that flag is nonzero before returning.
- `shift_byte_and_word_arrays_down`: Calls the flag check, invokes two library vectors, packs a word and byte into a long, decrements the count at -23538(a4), then shifts byte and word arrays down by one element and returns the packed value.
- `clear_stack_array_and_walk_list`: Zeroes a 32-word stack array, walks a linked list from the passed pointer, computes a size from the node's word fields, and calls a library routine with the array, node and shifted bit mask.
- `iterate_node_children_and_advance`: Walks the current node's child list, calling the display routine for each child index, then advances to the next node until the list ends, restoring d4 and a5.
- `run_two_pass_node_processing`: Pushes the two longword arguments from the stack frame and calls the first pass routine, then calls the second pass routine with the same arguments before returning.
- `walk_two_lists_calling_display`: Loads two list heads from the arguments, then repeatedly calls the display routine with each pair of node entries until either list is exhausted.
- `restore_d4_d5_a2_a3_and_return`: Pops d4, d5, a2 and a3 from the stack, releases the a5 frame pointer and returns to the caller.
- `blit_linked_sprite_pairs_loop`: Walks two linked lists of objects, reads each object's sprite descriptor, and repeatedly calls the blitter setup routine with width, height and blit parameters until either list ends.
- `restore_a2_a3_and_return`: Pops a2 and a3 from the stack, releases the a5 frame pointer and returns to the caller.
- `allocate_memory_block_type_10001`: Sets up a stack frame, pushes the constant 65537 and the caller's argument, calls the shared allocator, then discards the arguments and returns.
- `allocate_memory_block_type_10003`: Sets up a stack frame, pushes the constant 65539 and the caller's argument, calls the shared allocator, then discards the arguments and returns.
- `allocate_and_link_memory_block`: Aligns the requested size, calls the allocator with flag 65536, then links the new block into a circular list, writes begin and end markers, and returns the block payload.
- `check_stack_arg_and_branch`: Creates a small a5 stack frame, tests the longword at offset 8 from a5, and branches onward if it is nonzero, otherwise jumps to another routine.
- `compare_minus_one_and_probe_counter`: Compares the longword at offset 8 from a5 with minus one, then tests a counter in a4-relative memory and may call the stack-argument check routine with a pointer.
- `branch_back_into_compare_block`: Unconditionally branches backward into the middle of the preceding comparison routine, transferring control without touching registers or memory.
- `test_memlist_flag_and_branch`: Tests the word at offset minus 25630 from a4 and branches to the shared exit, leaving the flags set by that test.
- `unlink_memory_block_from_list`: Walks back through a linked list of memory blocks, validates their tags, unlinks the current block, clears its links and calls the free routine, returning zero on success.
- `return_minus_one_failure`: Loads minus one into d0 as a failure return value and branches into the shared exit path.
- `call_raw_put_char`: Library stub: loads its arguments and the exec library base, calls RawPutChar and returns its result to the caller.
- `scan_string_until_zero_and_dispatch`: Loads the return address from the stack into a0, reads one byte from (a0) and increments a0, returns if the byte is zero, otherwise calls the routine at the branch target and jumps back to continue.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `call_parser_with_stack_args_and_callback`: Saves a2, loads a1 with the stack argument area and a0 with the first stack argument, sets a2 to the local callback address, calls the parser, restores a2 and returns.
- `call_raw_do_fmt`: Library stub: loads its arguments and the exec library base, calls RawDoFmt and returns its result to the caller.
- `call_parser_with_four_stack_args`: Saves a2 and a3, loads a0 through a3 from the four stack argument slots, calls the parser routine, restores a2 and a3, then returns to the caller.
- `to_upper_ascii_char`: Reads the byte argument from the stack, and if it is a lowercase ASCII letter between 97 and 122, subtracts 32 to make it uppercase, returning the result in d0.
- `to_lower_ascii_char`: Reads the byte argument from the stack, and if it is an uppercase ASCII letter between 65 and 90, adds 32 to make it lowercase, returning the result in d0.
- `store_callback_and_clear_flag`: Saves the first argument into a global slot, clears a global byte flag, then pushes the second argument and a PC-relative pointer and calls a routine before restoring the stack.
- `read_next_char_or_flag_eof`: If the argument word is zero, reads the byte at the current pointer, advances the pointer by one, and returns it zero-extended in d0; otherwise returns minus one.
- `set_end_of_input_flag`: Stores byte value one into the end-of-input flag at offset minus 23402 from a4 and branches to the shared return path.
- `step_back_one_char`: If the end-of-input flag is clear, decrements the current pointer by one, loads the byte it now points to, zero-extends it into d0, and branches to the shared return path.
- `set_d0_minus_one_and_jump`: Loads minus one into d0 and branches to a shared continuation routine, providing a failure return value.
- `parse_percent_format_specifier`: Parses a percent-prefixed format specifier from the string in a2, handling star width, decimal digits, l modifier and conversion character, storing flags in the a5 frame and advancing a2.
- `resume_format_parse_at_percent`: Loads the percent character into d4 and branches to the shared format parser continuation, re-entering parsing at a percent specifier.
- `clear_flag_byte_at_a5_minus_6`: Clears the byte at offset minus six from a5 to zero, then branches forward to another routine.
- `set_flag_and_init_d4_d6_alt`: Writes one to the byte at offset minus six from a5, sets d4 to twelve and d6 to ten, then branches forward.
- `set_flag_and_init_d4_d6_zero`: Writes one to the byte at offset minus six from a5, sets d4 to zero and d6 to sixteen, then branches forward.
- `scan_format_string_into_output`: Walks the format string at a2, writing literal characters and parsed arguments into the buffer pointed to by a3, handling percent escapes, width digits, and length flags, and returns a status in d0.
- `store_word_arg_and_advance`: Loads the next argument pointer from a3, advances a3 by four, writes the word at -2(a5) through that pointer, increments d5, then branches back into the format loop.
- `detect_caret_or_tilde_flag`: Clears the flag byte at -6(a5), checks whether a2 points at a caret or tilde character, optionally advances a2 and sets the flag, then loads the buffer address into a0 and stores it at -12(a5).
- `copy_until_bracket_into_buffer`: Copies bytes from the a2 string into the buffer pointed to by the pointer at -12(a5), advancing both, until a closing bracket byte is read, then writes a zero terminator and branches onward.
- `setup_whitespace_table_and_read_record`: Stores a flag and a space, tab, newline, zero whitespace table at -142(a5), calls the input routine, then reads a longword from a3 into -12(a5) and fetches a character via the indirect call at -23394(a4).
- `return_zero_and_branch_to_shared_exit`: Sets d0 to zero with moveq and branches to the shared exit code, returning zero to its caller.
- `check_byte_at_a5_minus_8e`: Pushes d4, passes the address a5 minus 142 to a called routine, cleans the stack, and returns 1 in d0 if the call returned zero.
- `return_zero_and_call_cleanup`: Sets d0 to zero, then pushes a word, calls the routine pointed to by the a4-relative pointer, pops the stack word and branches onward.
- `store_d4_byte_if_flag_clear`: Tests the byte at a5 minus 5; if clear, stores d4 at the address in a5 minus 12, increments that pointer, sets the byte at a5 minus 7 to 1, then branches away.
- `check_flags_and_advance_counter`: Tests byte at offset minus seven from a5, branches if zero, tests byte at minus five, compares d7 with decimal 99, clears the byte pointed to by the longword at minus twelve from a5, and increments d5.
- `set_state_and_jump_back`: Tests byte at minus seven from a5, and if zero stores word one at minus 23396 from a4, clears byte at minus 142 from a5, sets byte at minus six from a5 to one, then branches away.
- `dispatch_command_by_subtraction_chain`: Subtracts successive constants from d0 and branches to different handlers when the result is zero, ending with a branch to a default handler.
- `scan_input_bit_table_for_next_set_bit`: Copies d4 to d0, increments d0, loads a table based at a4 minus 25382 into a0, tests bit 4 of the byte at a0 indexed by d0, and branches to a call or exit depending on the result.
- `compare_input_code_with_two_library_calls`: Pushes zero on the stack, calls the routine pointed to by a4-23394, cleans the stack, compares d0 with d4, and if unequal pushes one and calls the same routine again before branching.
- `jump_to_shared_exit_path`: Branches unconditionally to a shared exit routine at a fixed offset, transferring control without touching registers or memory.
- `check_d5_then_call_neg5b62`: Tests d5 and, if zero, pushes a zero word, calls the routine pointed to by the longword at offset -23394 of a4, cleans the stack, and returns -1 in d0 when the call result is -1, restoring d4-d7/a2-a3 and a5.
- `call_neg5b62_with_one_and_return_d5`: Pushes the word 1, calls the routine pointed to by the longword at offset -23394 of a4, removes the argument, moves d5 into d0 and branches back into the preceding routine.
- `call_neg5b62_and_test_bit4_of_table`: Sets up an a5 frame, pushes a zero word, calls the routine pointed to by the longword at offset -23394 of a4, removes the argument, increments d0, and tests bit 4 of the byte at offset d0 in the table at -25510 of a4, branching accordingly.
- `check_joystick_port_zero_button`: Pushes a one word argument, calls the routine pointed to by the a4-relative pointer, cleans the stack, and returns minus one in d0 if the result equals minus one, otherwise leaves d0 unchanged.
- `return_zero_and_branch_back`: Sets d0 to zero and branches backward into the preceding routine, so it returns a zero result through that shared exit path.
- `test_counter_and_return_zero`: Builds a stack frame, saves d4-d5 and a2, tests the a4-relative word counter, and if it is not positive sets d0 to zero, restores registers and returns.
- `parse_signed_number_from_input`: Clears local sign and value words, reads characters via the input routine, handles a leading minus or plus, accumulates digits into the value, and stores the signed result through the caller's pointer.
- `skip_plus_sign_increment_count`: Compares d4 with the plus character, and if equal increments d5 and branches forward past the digit loop entry.
- `read_char_after_sign_and_jump`: Pushes one word, calls the input routine through the a4 table pointer, pops the argument, then branches forward to the shared exit path.
- `read_key_and_accumulate_steering`: Calls the keyboard read vector, saves the key in d4, calls the position lookup with the object pointer, and if no match checks the key against 120 and 88 before returning.
- `apply_steering_delta_and_store_result`: Scales the steering value by the object field, adds the table byte offset by the pointer difference, increments d5, loops while below the limit, then stores the negated or plain result.
- `store_steering_result_and_jump_back`: Loads the pointer at offset 18 of a5, writes the accumulated value from -4 of a5 into it, moves d5 to d0 and branches back to the earlier routine.
- `allocate_buffer_and_call_worker`: Calls a subroutine to allocate a buffer, stores the returned pointer in a2, and if allocation succeeded passes two stack arguments to a worker routine; returns zero on failure.
- `find_entry_and_store_result_byte`: Scans a table of six-byte entries for a match, calls a lookup with the entry value, and on success writes the result byte and a flag into the output structure.
- `store_pointer_and_invoke_callback`: Saves the first argument into a global variable, then calls a routine with the second argument and a pointer to the caller's stack area.
- `call_6fa8_with_a4_offset_word`: Builds a stack frame, pushes the longword at offset -23454 from a4 and the word at offset 8 from a5, calls the routine at 6fa8, then discards six bytes and returns.
- `patch_hunk_relocations_in_memory`: Scans a loaded hunk block for relocation entries, frees each referenced memory chunk via exec FreeMem, and writes jump instructions with adjusted offsets into the target area.
- `find_first_matching_byte_in_string`: Loads a pointer and a byte value from the stack, scans bytes until a match or zero terminator, and returns the address of the matching byte or zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `init_alert_and_open_dos_library`: Clears a block of 1284 longs at a4-relative memory, saves the stack and exec base, checks exec flags, opens dos.library and alerts on failure.
- `set_a4_to_absolute_7ffe`: Loads the absolute address 32766 into a4 and returns immediately to the caller.
- `allocate_and_init_manx_structure`: Allocates a size-scaled block, fills MANX header fields, opens a library, sets flags and calls several subroutines before returning.
- `copy_d0_to_d5`: Copies the longword in d0 into d5, then falls through into the following routine.
- `tokenize_text_into_argument_list`: Allocates a buffer, copies a source string, splits it into whitespace-separated tokens honoring double quotes, counts them, allocates a pointer array and stores each token start address, terminating with a null pointer.
- `copy_d0_to_d0`: Copies the longword in d0 onto itself, a no-op that falls through into the next routine.
- `bounded_strncpy_pad_zero`: Copies bytes from source a1 to destination a0 until a null byte or the word count in d1 is exhausted, then pads the remainder with zero bytes and returns the destination pointer in d0.
- `open_file_with_mode_wrapper`: Builds a stack frame, pushes the mode word and filename pointer from the caller's arguments, calls the file-open routine, cleans the stack and returns.
- `allocate_file_slot_and_open`: Scans the file table for a free slot, opens the named file with the requested access mode, stores the handle and flags in the slot, and returns the slot index or an error code.
- `data_table_not_code`: The bytes decode as meaningless immediate operations and a jump to an invalid address, so this is data, not executable code.
- `patch_track_vectors_into_jump_table`: Saves registers, reads a track index from the stack, walks a table of entries and writes jump instructions with computed targets into the destination buffer.
- `restore_registers_and_return`: Pops d0 through d2 and a0 through a4 from the stack and returns to the caller, restoring the saved register set.
- `open_dos_append_node_to_list`: Opens dos.library via exec, seeks to end of file, reads a longword, then walks the list at offset 8 of a4 and appends the value, returning the library base in d2.
- `store_d0_into_a0_and_return`: Writes the longword in d0 to the address in a0, restores d2, d3 and a6 from the stack, and returns to the caller.
- `call_alert`: Wrapper that calls the exec library function Alert and returns its result to the caller.
- `bounded_strcpy_return_destination`: Finds the end of the destination string in a0, copies up to the word count from a1 into it, null-terminates, and returns the original destination pointer in d0.
- `compare_strings_return_order`: Compares two null-terminated strings passed on the stack byte by byte, returning zero if equal, one if the first is greater, or minus one if less.
- `copy_string_return_destination`: Copies bytes from the source string at stack offset eight to the destination at offset four until a null byte, returning the destination pointer in d0.
- `multiply_16bit_halves_to_32bit`: Multiplies the low and high 16-bit halves of d0 and d1, combining partial products into a 32-bit result in d0 while preserving d1 through d3.
- `load_window_icon_and_font_resources`: Opens the WINDOW library, fetches the window's icon list, loads an icon and a font, stores their pointers and counts into the window structure, then frees the temporary resources.
- `copy_d0_to_d5_for_icon_size`: Copies the value in d0 into d5 so the caller can keep the icon size while d0 is reused.
- `store_context_call_callback_clear_flag`: Saves d4, stores the first argument in an a4-relative slot, calls a routine with two arguments, clears a byte through the stored pointer and returns the result in d0.
- `append_byte_to_output_pointer`: Loads the output pointer from a4-relative storage, increments it, writes the byte argument from the stack into that location, and returns the byte zero-extended in d0.
- `invoke_formatter_with_stack_buffer`: Pushes a stack buffer address and the caller's argument, calls the formatting routine, then discards the arguments and returns.
- `format_number_into_buffer_with_sign`: Converts a value from the argument block into digits using the digit table, prepends a minus sign when negative, and returns the buffer pointer in d0.
- `format_string_output_parser`: Parses a printf-style format string, reading width, precision and type specifiers, fetching arguments from the stack pointer and calling the output callback for each character, returning the count or minus one on error.
- `signed_divide_with_sign_fixup`: Takes a dividend in d0 and divisor in d1, negates each if negative while tracking signs in d4, calls the unsigned division routine, then negates the quotient if the signs differ and returns it in d0.
- `signed_divide_alternate_entry`: Saves d1 and d4, clears d4, negates d0 and d1 when negative while toggling the sign flag, calls the division routine, then moves the quotient from d1 into d0 and branches back into the shared return path.
- `divide_d1_by_d0_return_quotient`: Pushes d1, calls the shared division routine, moves the quotient from d1 into d0 as the return value, restores d1 and tests d0.
- `divide_d1_by_d0_keep_d1`: Pushes d1, calls the shared division routine, restores d1 from the stack and tests the returned d0 without moving it.
- `unsigned_32bit_divide_d0_by_d1`: Saves d2-d3, then divides the 32-bit value in d0 by d1 using a 16-bit divu path or a 16-step shift-subtract loop, returning quotient in d0 and remainder in d1.
- `strlen_return_length_in_d0`: Loads the caller's first stack argument into a0, scans bytes until a zero terminator, then returns the string length in d0.
- `call_6fa8_with_global_and_arg`: Opens an a5 frame, pushes the address of a global offset by a4 and the word argument at offset 8, calls the routine at 6fa8, cleans the stack and returns.
- `dispatch_by_word_arg_and_flag`: Saves d4, calls 6fea with the word argument and pointer, and if the argument equals ten and the pointed byte has bit seven set, calls 70d4 with minus one and the pointer.
- `branch_to_shared_putc_tail`: Branches unconditionally to the shared code at the earlier address, transferring control without touching any registers or memory itself.
- `flush_buffer_if_full_else_put_byte`: Sets up a stack frame, loads the buffer pointer from the argument, compares its current position against its end, and if not full calls the byte-output routine with the low byte of the second argument.
- `store_byte_and_advance_buffer_pointer`: Loads the buffer write pointer from a2, increments the pointer stored at a2, writes the byte from the stack argument through a0, then branches back to the shared output tail.
- `iterate_stream_structs_call_processor`: Walks a table of 22-byte stream structures from a fixed base to an end address, calling the stream processor on each entry, then restores a2 and returns.
- `flush_and_free_stream_struct`: Validates the stream pointer, flushes pending output, processes the byte at offset 13, frees buffers and clears the structure fields, returning a status word.
- `write_byte_to_stream_buffer`: Checks stream flags, verifies buffer position, refills or allocates the buffer, stores the given byte at the write pointer and returns the byte value.
- `find_free_22_byte_slot`: Scans a table of 22-byte entries starting at a fixed offset in a4, returns the first entry whose byte at offset 12 is zero after clearing its first three longs, or returns zero if none found.
- `allocate_buffer_for_stream_entry`: Requests a 1024-byte block, stores the pointer at offset 8 of the a2 entry, sets its size field, marks flags, and tests a byte value via a helper to set the high flag bit.
- `load_a2_from_global_and_branch`: Opens an a5 frame, saves a2 and a3 on the stack, loads a2 from a fixed global in a4, then branches forward into another routine.
- `free_list_nodes_and_clear_head`: Walks the linked list at a2, calling a free routine on each node's payload plus eight, then clears the list head pointer in a4-relative memory and returns zero.
- `allocate_and_prepend_list_node`: Stores a callback pointer in a4-relative memory, allocates a node of the given size plus eight, links it at the head of the a4 list, and returns the payload pointer or zero.
- `allocate_node_from_word_size`: Reads a word size argument from the stack, zero-extends it into d0, passes it to the node allocation routine, then releases the argument and returns.
- `unlink_node_and_free_block`: Searches the global list at offset -23382 from a4 for the node whose address is eight bytes before the argument, unlinks it, then calls the free routine with the node and its size plus eight, returning zero or minus one.
- `validate_index_and_test_entry`: Checks the word argument against zero and the count at -24940 from a4, indexes the six-byte table at -22554 from a4, sets error word two at -22568 if invalid or empty, otherwise calls the test routine on the entry and returns one or zero.
- `check_object_then_store_error`: Calls the check routine with the argument; if it returns zero, calls the error routine, stores its word result at -22568 from a4 and returns minus one, otherwise returns zero.
- `open_file_by_index_and_call`: Validates a file index against the table base and count, checks the entry is open and its flags, then calls the open routine with the entry pointer and two caller arguments, returning the result or minus one on error.
- `allocate_memory_and_check_flag`: Calls the memory allocator for 4096 bytes, stores the pointer in a local, tests bit 12 of the result and returns it unless a global flag is set, in which case it calls the error routine and returns zero.
- `report_error_with_message`: Pushes a length and a message pointer, calls the string output routine, then calls the error display routine with the returned value and finally invokes the exit routine with argument one.
- `call_hook_then_forward_word_arg`: Tests the longword hook pointer at offset -22530 from a4; if non-zero calls it through a0, then pushes the word argument from the stack frame and calls the routine at 7492 before returning.
- `shutdown_audio_and_free_buffers`: Saves d4, loops calling a per-voice routine for each of the voice count, frees the voice block, invokes optional callbacks, checks the exec library for audio flags, frees sample buffers and returns the saved argument in d0.
- `restore_d4_and_unwind_frame`: Pops d4 from the stack, releases the a5 stack frame and returns to the caller; reached only as a gap that decodes as code.
- `free_sound_slot_by_index`: Looks up a six-byte sound slot by index, validates the index and slot pointer, calls the slot release routine on the stored pointer, clears the pointer and returns zero, or sets an error flag and returns minus one.
- `jump_to_sound_release_vector`: Jumps through a vector in the sound library base at offset minus thirty-six, transferring control to the release routine without touching any registers.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `jump_to_sound_stop_vector`: Jumps through a vector in the sound library base at offset minus one hundred two, transferring control to the stop routine without touching any registers.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_input_2`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `jump_via_a4_offset_minus_84`: Loads a pointer from the a4-relative table at offset minus 22558, then jumps indirectly through the entry at minus 84 bytes from that pointer.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `jump_via_a4_offset_minus_30`: Loads a pointer from the a4-relative table at offset minus 22558, then jumps indirectly through the entry at minus 30 bytes from that pointer.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output_2`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `jump_via_a4_offset_minus_42`: Loads a pointer from the a4-relative table at offset minus 22558, then jumps indirectly through the entry at minus 42 bytes from that pointer.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `jump_via_a4_offset_table_entry`: Jumps indirectly through a pointer fetched from a table relative to register a4, transferring control to the target routine.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_write_2`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `jump_via_a4_second_table_entry`: Jumps indirectly through a different pointer fetched from a table relative to register a4, transferring control to another target routine.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_630`: Library stub: loads a library base from a global variable and calls the library function at offset minus 630 of that base, returning its result to the caller.
- `allocate_and_initialize_message_node`: Allocates a message node, fills its fields from the two arguments, stores a handle, and returns the node pointer or zero on failure.
- `reset_object_state_and_notify`: If the object's field at offset 10 is nonzero, calls the cleanup routine with the object pointer; then sets byte 8 to 255, long at 20 to -1, passes byte 15 to another routine, and calls a routine with the object and constant 34.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `forward_object_with_constant_48`: Pushes the constant 48 and the object pointer from the stack argument, calls the shared routine at the given offset, then cleans the stack and returns.
- `forward_object_pointer_to_routine`: Pushes the object pointer taken from the stack argument, calls the routine at the given offset, adjusts the stack by four bytes and returns to the caller.
- `allocate_node_and_init_fields`: Checks the first argument, allocates a 65537-byte block via a library call, then stores type 5, a word from the second argument and the first argument into the new block, returning its pointer or zero.
- `reset_node_and_dispatch_release`: If the node pointer is non-null, sets its type byte to 255, fills two long fields with -1, then calls the release routine with the node and its word field.
- `jump_to_release_node_entry`: An empty trampoline that immediately jumps to the release routine at the following address, forwarding the caller's registers unchanged.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `jump_via_a4_offset_table_entry_2`: Loads a pointer from the table at a4 minus 22562, adds a negative offset, and jumps indirectly to the resulting code address.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `jump_via_a4_handler_slot`: Reads a handler pointer from the data table at a4 minus 22562 plus a negative offset and jumps indirectly to that address.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `jump_via_a4_dispatch_slot`: Fetches a code pointer from the table at a4 minus 22562 with a negative offset and performs an indirect jump to it.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `init_self_referential_struct`: Loads the return address from the stack into a0, stores a0 at (a0), adds four to that value, clears the longword at offset four, stores a0 at offset eight, and returns.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `jump_through_a4_offset_table`: Jumps indirectly through a pointer table reached via the a4 base register plus a negative offset, transferring control to the target routine without returning here.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_300_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_372_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_576`: Library stub: loads a library base from a global variable and calls the library function at offset minus 576 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_540`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_360_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_456_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_306_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_54_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_228_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_78_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.

### Test Drive.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_17`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_18`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
