# Game 158 (Amazed): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00158). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 57 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 144 of 144 named (0 author symbols), 144 of 144 described (30 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 110 of 110 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 198296 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5311 of 5313 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 348638 |
| code | 20518 |
| data | 274794 |
| padding | 17042 |
| unused | 240128 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| run | 2 | 458 | 12 |
| IffLoaderV2.0 | 3 | 218 | 5 |
| execute | 2 | 117 | 2 |
| Music | 2 | 498 | 34 |
| cd | 2 | 282 | 5 |
| Wait | 2 | 117 | 2 |
| Amazed.adf:nomem | 2 | 33 | 1 |
| Amazed.adf:copy | 2 | 319 | 24 |
| Amazed.adf:cd | 2 | 282 | 5 |
| Amazed.adf:LOADER | 2 | 465 | 43 |
| Amazed.adf:bootblock | 1 | 218 | 10 |
| Amazed.adf:sound1 (raw code file, load address unknown) | 1 | 0 | 0 |
| Amazed.adf:CHAR (raw code file, load address unknown) | 1 | 0 | 0 |
| Amazed.adf:SPACE (raw code file, load address unknown) | 1 | 2306 | 1 |

## Cases handled

- Amazed/Amazed.adf:Trashcan/LOADER: truncated hunk executable (hunk 0 runs past the end of the file), not loadable: data
- Amazed/Amazed.adf:sound1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Amazed/Amazed.adf:CHAR: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Amazed/Amazed.adf:SPACE: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 branch displacement
- 2 never-loaded region(s), 163840 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `disable_interrupts_then_checksum_copy` (bootblock): Checks a flag at offset 44 of the structure in A1 and, if it is clear, may read the CIA port register B and port register A to test two fire-button bits, then disables interrupts through INTENA. Kind: checksum trap. Pass path: The routine computes a checksum of the copied data and stores it back at offset 4 in the buffer. If the computed checksum matches what's expected (i.e., the check passes), execution continues normally via `_u10__d0_03D6()`. The check is per.

## Functions


### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copies_d1_into_d0`: Copies the 32-bit value in register d1 into register d0, then branches to a point inside another routine.
- `initialise_sprite_structure_fields`: Writes a series of values from registers and a source table into the structure pointed to by a1, storing words at offsets 0, 4, 8, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64 and 68, and a byte-derived value at offset 48 and table-derived values at 36 and 68.
- `builds_entry_table_and_links`: Stores a value from register d1 at offset 128 of the structure in a1 and copies the longword at offset 0 into offset 132, then calls two routines.
- `fills_table_and_updates_state`: Copies a block of bytes from a source buffer into a per-entry table and writes a terminator byte after it, using a stored count and offset from the structure.
- `updates_sprite_state_and_loops`: Writes constant and copied values into a per-entry table indexed by a stored count, then reads a status longword at offset 40 of the structure in a1 and, when it is non-zero, advances the entry at index 15 in the same table.
- `adds_offset_and_branches_forward`: Loads 1 into d1, adds the 32-bit value at offset 84 of the structure in a1 to d1, and branches unconditionally to a fixed address elsewhere in the program.
- `test_field_and_branch_forward`: Tests the longword at offset 40 of the structure in a1 for zero, branches to a later instruction in another routine when it is non-zero, and otherwise branches unconditionally to a different later instruction.
- `tests_flag_bit_and_branches`: Tests bit 7 of the longword at offset 48 of the structure in a1, sets all upper 24 bits of that longword when the bit is set, then compares the longword against 133 and branches to one of two targets depending on whether it is less.
- `update_entry_slots_and_dispatch`: Stores a longword from the structure in a1 at offset 108 into a table in a0, clears several indexed slots of that table, initializes a block of fields in the structure from other fields of the same structure, and then calls another routine.
- `reset_entry_fields_and_dispatch`: Writes a longword from offset 28 of the structure in a1 into an indexed slot of the table in a0 and clears several indexed slots of that table, then initializes a block of fields in the structure in a1 at offsets 124 through 136 from its own values and the value at offset 28, and conditionally decrements a counter in the table.
- `updates_linked_entry_fields`: Stores the value in register d1 at offset 128 of the buffer in a1 and copies the longword at offset 0 of the structure in a1 into offset 132.

### IffLoaderV2.0

- `load_iff_file_and_open_display`: Opens a file named by the string in a0 with dos.library, reads its chunk headers and compares their four-byte IDs against FORM and IL BM markers, then allocates memory for a chunk and conditionally adjusts flag bits in a global variable.
- `close_libraries_and_screens`: Waits in a loop while bit 6 of the CIAA_PRA byte is set, then closes the window and screen whose global pointers are nonzero and closes two library bases stored in global variables.
- `frees_stored_memory_block`: If the global pointer at address 104 is nonzero, it calls the exec library FreeMem routine with that pointer and the size stored at address 136, then clears the pointer to zero.
- `sets_intuition_pointer_shape`: Loads the window pointer from address 44 and a 1-by-15 pointer image address, with zero hotspot offsets, then calls the intuition SetPointer routine through the library base at address 124 and returns.
- `load_handler_and_compare`: Reads the exec library base from a fixed low-memory address into a register and loads a0 into another register, then calls a library routine through the base in a6 and compares its result with the value in d3.

### execute

- `scan_list_and_build_frame`: Walks a linked list of entries to find the largest value in a field, then, when a result buffer is allocated, writes a header and a run of sequential words into it, sets up a stack frame, and copies sixteen words between a temporary area and a global buffer.
- `copy_d1_and_branch`: Copies the value in d1 into d0 and then branches unconditionally to a location in another routine.

### Music

- `wait_for_port_message`: Finds the current task, returns zero immediately if a field at a fixed offset in that task is non-zero, otherwise opens a library into a global variable, waits on the task's message port, stores the received message in a global variable, calls a routine through a function pointer read from the library base, then forbids multitasking and replies to the stored message.
- `poll_audio_library_loop`: Opens the graphics, intuition and iff libraries into global variables, then loops indefinitely reading the CIA-A port A byte and the custom potentiometer register, calling a library function through a stored pointer on each pass.
- `initialize_audio_and_timers`: Stores a fixed value into a global variable, then calls another routine and adds its result together with a constant to fill fourteen consecutive longword slots in a global table with a running sum.
- `convert_word_to_hex_digits`: Takes the word in d0 and writes four ASCII hex digit characters to consecutive bytes of a global variable buffer, starting from the most significant nibble, rotating the value left by four bits after each digit.
- `find_max_byte_in_buffer`: Scans a fixed run of bytes in a global variable buffer, keeping the largest byte value in a word register, and then returns that maximum plus one multiplied by 1024 in the upper word of d0.
- `configure_cia_timer_a_and_interrupts`: Writes 129 to the CIA-A interrupt control and timer A control registers, then loads the timer A low and high bytes, taking the low byte from a table entry and the high byte from a global variable.
- `calls_routine_returns_zero`: Calls another routine via a branch to a fixed location and then sets the result register to zero before returning.
- `clear_fourteen_pointed_longs`: Walks fourteen consecutive longword slots starting at a global variable, treating each slot as a pointer and writing a zero longword to the address it holds.
- `zero_audio_volumes_and_clear_state`: Writes 65535 to a global variable, zeroes the four audio channel volume registers and a custom chip word at offset 0 from the custom base, then clears three longword and word globals used as state.
- `zero_audio_volumes_and_set_dma_mask`: Clears one global word, zeroes the four audio channel volume registers and a custom chip word at offset 0 from the custom base, writes 15 to the DMA control register, and returns.
- `increments_counter_and_calls_routine`: Saves all registers, adds 1 to a global word and compares it with 6, clearing it to zero when equal and otherwise branching past that step, then calls another routine via a relative call, restores all registers and returns.
- `loops_over_four_audio_channels`: Loops four times with a counter starting at 3 and decrementing to zero, each pass calling a routine with the counter and the current pointer into the audio channel location registers, which advance by 4 bytes per pass.
- `advance_and_clamp_word_pair`: Adds d0 to the word at offset 2 of the structure in a4 when the word at offset 0 is nonzero and not negative, then reads the limit at offset 4 and stores the smaller of the sum and that limit into d0, the word at offset 2 and the word at offset 6 of the structure in a5, and returns.
- `add_offset_then_clamp_word`: Adds the word in d0 to the word at offset 2 of the structure in a4, then loads that sum into d0 and compares it with the word at offset 4 of the same structure, keeping the smaller value in d0.
- `dispatch_or_store_table_entry`: Reads the low nibble of the byte at offset 1 of the structure in a6 and branches on values 1 to 4, 7 and 8: value 1 jumps through a global table indexed by a decremented word, values 2 adjusts a word by the high or low nibble of the byte at offset 3, value 3 sets bit 1 of the CIA-A port A register, value 4 clears it, and values 7 and 8 fill a 3-word structure in a4 from the table and nibbles.
- `set_bit_one_of_cia_port_a`: Sets bit 1 of the CIA-A port A register by reading its current value and writing it back with bit 1 set, then returns.
- `clear_cia_port_bit`: Reads the byte at the CIA-A port A register, clears bit 1 and writes it back, then returns.
- `decode_table_entry_into_struct`: Clears the word at the start of the structure in a4 and copies the word from a6 into offset 2, then stores the low nibble of the byte at offset 3 of a6 as the first word.
- `fetch_table_word_by_nibble`: Doubles the word in d2 and negates the offset when d4 is zero, then reads the word from the table at a0 at that offset and stores it at offset 4 of the structure in a4, then returns.
- `store_d0_at_offset_four`: Stores the word in d0 at offset 4 of the structure in a4, then returns.
- `stores_low_byte_then_branches`: Stores the byte in d4 through the address in a4 and then branches forward into the middle of another routine, so it has no return of its own.
- `dispatch_through_indexed_pointer`: Loads a fixed global table base into a0, takes the word from a global variable, decrements it, scales it by four to index the table, and jumps to the code address fetched from that entry.
- `lookup_word_from_nibble_table`: Doubles the index in d0 and reads a word from a fixed table at the offset given by that index, comparing it with the word at offset 16 of the structure in a6 and stepping through the table until a match is found.
- `looks_up_nibble_table_word`: Clears the full d0 register, places the high nibble of the byte at offset 3 of the structure in a6 into d0, and then branches to the shared lookup routine, which searches a global variable table for the word at offset 16 of the structure in a6 and stores the matching table entry at offset 6 of the structure in a5.
- `looks_up_low_nibble_table_word`: Clears the full d0 register, places the low nibble of the byte at offset 3 of the structure in a6 into d0, and then branches to the shared lookup routine, which searches a global variable table for the word at offset 16 of the structure in a6 and stores the matching table entry at offset 6 of the structure in a5.
- `copy_word_offset_16_to_six`: Copies the word at offset 16 of the structure in a6 into offset 6 of the structure in a5, then returns.
- `add_high_nibble_to_word`: Extracts the high four bits of the byte at offset 3 of the structure in a6, and if they are non-zero adds them to the word at the start of that structure.
- `subtract_low_nibble_and_store`: Takes the low four bits of the byte at offset 3 of the structure in a6, and if they are non-zero subtracts them from the word at a6 and stores the result at offset 6 of the structure in a5; otherwise it returns.
- `sets_audio_dma_and_copies_channel`: Clears a global word, calls another routine once for each of four channels, then writes the accumulated value OR'd with bit 15 to the DMA control register.
- `unpack_channel_state_per_frame`: Clears two words of the structure in a6, copies a long word from the table indexed by d1 when a flag is set, and when the first word is not -3 it unpacks the high nibble of the byte at offset 2 into a table-selected channel record, updating words at offsets 4 through 18 and the volume word through a per-mode adjustment clamped to 0 to 64.
- `update_channel_dma_state`: Adds a word from a table to the structure in a6 and updates the volume word at offset 4 with a per-channel adjustment clamped to 0-64 when the low nibble of the flag word is 5 or 6.
- `updates_channel_state_and_dma`: Adds the byte at offset 3 of the structure in a6 to the word in d3, clamps the sum to a maximum of 64 and stores it at offset 2 of the structure in a5.
- `start_dma_and_copy_channel_fields`: If the word at the start of the structure in a6 is non-zero, it clears the word at the structure in a4, copies a word into offset 16 and writes another word to the DMA control register, then waits a fixed delay loop.
- `copy_fields_and_or_flag_word`: Copies the longword at offset 4 and the words at offsets 8 and 0 of the structure in a6 into the structure in a5, then ORs the word at offset 20 of a6 into a global variable and returns.

### cd

- `find_max_and_build_block`: Walks a linked list of entries to find the largest value in a field, then, if the subsequent allocation call succeeds, writes that maximum and a run of sequential words into a newly obtained block, copies sixteen words from a temporary area into a global buffer and installs a function pointer.
- `branch_into_maximum_search`: Copies d1 into d0 and branches into the middle of another routine, transferring control there.
- `toggle_bit_and_dispatch`: Toggles one bit of the byte at the address in a3, selected by the low three bits of d0, then stores a0 and a1 values into the structure in a1 and the offset-indexed longwords at offsets 600 and 604 of the structure in a2, after several calls through the pointer in a5.
- `dispatch_slot_and_store_value`: Calls a routine through the pointer in a5 several times, and when the longword indexed from the table in a0 is nonzero it stores the value in d1 at offset 600 of the structure in a2 and at offset 440 of the structure in a1 when d1 is zero or not.
- `process_entries_and_dispatch`: Calls through a0-indexed pointer in a5 several times, conditionally stores a longword from d1 at offset 440 of a1, checks a signed longword in a table indexed from a2 and calls through a5 again, stores a longword from offset 608 to offset 608 of a2 and finally jumps through an unrecovered table entry.

### Wait

- `build_search_result_block`: Scans a linked list of entries for the largest value in a field, then if a pointer obtained from a call is non-null it writes that maximum as a header word followed by sequential words into a buffer, sets up a frame on the stack, and copies sixteen longwords between a temporary area and a global buffer.
- `branch_into_maximum_search_2`: Copies d1 into d0 and branches into the middle of another routine, transferring control there.

### Amazed.adf:nomem

- `idle_wait_and_reply`: Finds the current task, waits on its message port if the flag at offset 2 of its structure is zero, takes a message, then repeatedly queries free memory and allocates blocks of that size until allocation fails.

### Amazed.adf:copy

- `build_search_frame_and_call`: Finds the largest value in a linked list field and then, if the callee succeeds, writes a header, fills a block of sequential words, sets up a stack frame, and copies sixteen words between a temporary area and a global buffer before installing a function pointer.
- `copy_d1_and_branch_2`: Copies the value in d1 into d0 and then branches to a fixed address inside another routine.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `set_counter_and_store_limit`: Sets the longword at offset 4 of the structure in a1 to 1, copies a longword from a fixed negative offset of a4 into offset 8 of that structure, and then branches forward into another routine.
- `compare_indexed_longword_then_jump`: Loads a global variable, adds the longword at offset 4 of the structure in a1, shifts the sum left by 2, and compares the longword at a1 with the longword at a0 plus that offset.
- `increment_then_store_longword`: Increments the longword at offset 4 of the structure in a1, compares it with the longword at offset 8 of that structure, and branches to another routine when it is not greater.
- `clear_bits_and_branch_on_counter`: Toggles bit d3 of the byte at offset 8288 from a2, pre-decrements a0 to read a longword into d0, and compares a global variable against a second global variable.
- `dispatch_indexed_byte_jump`: Increments a counter in a global variable, reads a byte from the buffer in a0 at an index computed from a second global variable multiplied by 4 plus that counter, stores it in a third global variable, and then jumps indirectly through a6.
- `toggle_bit_and_call_twice`: Toggles one bit in a byte of a table selected by the low three bits of d3, then if the word at offset -176 of a4 equals 39 it calls through a5, and in either case calls through a5 again before an indirect jump through a table entry.
- `branch_into_dispatch_sequence`: Tests a bit in a byte of the buffer in a0 indexed by d7, then decrements a0 and copies two longwords from a global-variable area of a4 into the structure in a1.
- `set_minus_one_and_jump`: Stores the value -1 as a longword at offset -200 of the structure in a4, then performs an indirect jump through a table entry.
- `call_twice_store_longs`: Loads 32 into d0, adjusts a4 by zero, calls through a5, reads a longword from a1 into d2, loads 20 into d0, adjusts a4 by 288, calls through a5 again, reads a longword from a1 into d1, and then jumps indirectly through a table entry.
- `store_longword_conditional_reset`: Calls through a5, stores the longword in d1 through a1, and if the word at offset -64 of a4 is not 41 stores -1 at offset -52 of a4.
- `dispatch_on_longword_code`: Reads the longword at offset 4 of the structure in a1 and compares it with the codes 35, 40, 41 and 124.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.
- `toggle_bit_and_process_table`: Toggles one bit in a byte addressed relative to a5 using the low three bits of d3, clears the longword at offset 4 of the structure in a1, then repeatedly calls a routine through a pointer and copies values between table entries and a byte array indexed by a global variable, stopping when the table entry equals one of three fixed codes, and finally jumps through an unrecovered pointer.
- `store_byte_at_indexed_offset`: Computes an index from a global variable scaled by 4 plus the longword at offset 0 of the structure in a1, then stores the low byte of a global variable into the buffer in a0 at that index.
- `call_with_fixed_register_setup`: Loads a longword from offset -356 of a4 into d2 and a longword from offset 8 of a1 into d1, sets d0 to 24, adjusts a4 by 140, calls through a5, and then branches forward to a later point.
- `indirect_jump_stub_2`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `swap_byte_list_entries`: Loops over a linked list in a1: for each node it exchanges the byte at the position given by the node's first longword plus a global multiplier with the byte at offset 7 of the node, stores the old byte in the node's third longword, and continues while the first longword is nonzero.
- `branch_to_scan_loop`: Stores d1 into the longword at offset 8 of the structure in a1, tests d1, and branches to a later point in the code when it is nonzero.
- `branch_to_scan_routine`: Performs an unconditional branch to a later point in another routine that scans zero-terminated bytes.
- `scan_zero_terminated_bytes`: Repeatedly reads a longword from a1 as an index into the buffer in a0 scaled by a global multiplier, advancing until it reads a zero byte, then writes the byte at offset 7 of the node into that position and jumps indirectly through a table entry.

### Amazed.adf:cd

- `build_list_maximum_table`: Walks a linked list of entries to find the largest value in a field, then if a buffer allocation succeeds it writes that maximum and a sequence of ascending words into a table, copies sixteen longwords between a stack area and a global buffer, stores a function pointer, and returns 0 on success or -1 otherwise.
- `copy_d1_to_d0`: Copies the 32-bit value in register d1 into register d0, then branches unconditionally to a later point in the code, so control does not fall through to the routine's end.
- `toggle_bit_then_init_structure`: Toggles one bit of the byte at the pointer in a3, using the low three bits of d0 as the bit number, and then writes zero and a pointer value into fields of the structure in a1 and a2.
- `dispatch_indexed_entries_conditionally`: Calls through a function pointer in a5 several times, and when the longword at the indexed entry in the table at a0, selected by the value in a1, is nonzero it stores d1 at offset 608 of a2, sets d1 or a second value into offset 440 of a1 depending on d1 and a second value, and checks the signed longword at offset 120 plus a scaled index from a2 before finishing with a final call and a jump through a pointer.
- `dispatches_indirect_jump_table`: Calls through a function pointer several times, stores a 32-bit value from a register into a structure at offset 440, and conditionally calls the pointer again when a structure field indexed by a global variable is negative.

### Amazed.adf:LOADER

- `start_loader_and_fold_bitplanes`: Starts from the program entry, reads the exec base from absolute address 4, opens the DOS library by name, and stores the returned library base in a global variable, then enters a supervisor-state wrapper, and runs a block fold that ORs five rows of 16-bit words from a fixed source area, inverts each result and writes it out, using the exec memory allocation, availability, task-lookup and message-port calls.
- `closes_library_when_flagged`: Tests a global variable and, when it is nonzero, calls one library routine through a6 and then a second one with a pointer loaded from a global variable.
- `combines_bitplane_rows_into_buffer`: Loops over a block of 16-bit words starting at a fixed source address, ORs five source rows together and inverts the result into one destination row, and copies the source rows to five further destination rows at fixed offsets.
- `call_super_state`: Wrapper that calls the exec library function SuperState and returns its result to the caller.
- `call_user_state`: Wrapper that calls the exec library function UserState and returns its result to the caller.
- `open_dos_and_list_directory`: Loads the exec base from absolute address 4, opens the DOS library by name with version zero, stores the returned library base in a global variable, and returns immediately when that base is zero, otherwise it calls a directory-listing routine and then returns.
- `lock_examine_and_close_directory`: Calls another routine, locks the path string in a global variable in shared mode, and if the lock succeeds stores the lock handle in a global variable and examines it into a file information block, then reads the error code, calls two routines, closes the stored handle and returns.
- `lists_directory_entries_until_done`: Repeatedly calls the DOS directory-scan call on a directory handle, and for each entry found calls two other routines; when the scan returns zero it reads the DOS error code, then closes the handle.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `opens_dos_file_by_name`: Moves a name pointer from a1 and a mode value from d0 into the registers used by the DOS open call, invokes that DOS library function through the library base from a fixed global address, and returns its result in d0.
- `writes_null_terminated_string`: Counts the bytes of the null-terminated string at the address in d0 by scanning forward one byte at a time, then passes the string and that count to the DOS write call on the standard output handle.
- `writes_byte_ten_then_thirteen`: Loads the value 10 into d0 and calls the routine that writes the low byte of d0 to the output handle, then loads the value 13 into d0 and falls through into the following code without a return.
- `writes_byte_to_output_handle`: Saves all registers on the stack, stores the low byte of d0 into a global variable, and writes that single byte to the standard output handle via the DOS write call.
- `waits_for_input_character`: Passes the standard input handle from a global variable and a timeout of 500 to the DOS wait-for-character call through the library base, then tests the returned value and returns.
- `reads_line_with_backspace_editing`: Reads characters one at a time through the reading routine into a global buffer, removing the previous character and erasing it on screen when a backspace or delete code arrives, and stops at a carriage return.
- `clears_byte_and_adjusts_counter`: Clears the byte at the address in a2 and post-increments a2, then subtracts 1036 from that address and stores the low word into a global variable.
- `writes_backspace_space_and_clears_byte`: Compares a2 against a fixed address, and if they are not equal writes the bytes 8, 32 and 8 through the output routine, clears the byte at a2, decrements a2 by one and branches back into the following code; if they are equal it branches directly to that code.
- `reads_one_byte_from_input`: Sets up a read of one byte from the standard input handle into a global variable through the DOS read call, then returns that byte zero-extended in d0.
- `set_load_parameters_and_write`: Loads the constant 1006 into d2, calls a subroutine, branches to a return on zero, otherwise loads the constants an address and an address into d2 and d3, calls two subroutines, and returns.
- `fold_bitplane_rows_inverted`: Loops over a block of 16-bit words read from a fixed source area, ORs five rows of the bitplane together for each word, inverts the result and stores it into a destination buffer.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_2`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_3`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_4`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_5`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_6`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_7`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_8`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_9`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_10`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_11`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_12`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_13`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_14`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_15`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_16`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_17`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_18`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_19`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_20`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.

### Amazed.adf:bootblock

- `disables_interrupts_copies_block`: Writes the interrupt-enable register with bit 15 clear to disable interrupts, then increments a nesting byte in the system library base and, if the top byte of an interrupt vector entry is set, copies a global variable's bytes into a memory list header area and calls a library routine through the base in a6, returning its result.
- `clears_counters_and_toggles_interrupts`: Clears two global counters, calls two other routines, then calls a library routine through the base in a6 and, on success, decrements a byte global variable at offset 294 of the structure in a6; when that byte is zero or less it writes the interrupt enable register with bit 14 set, and returns a 32-bit result of 0 on success or -1 on failure.
- `enable_interrupts_then_disable`: Writes the INTENA register with bit 14 set, increments a byte global variable at offset 294 of the structure in a6, calls two routines through pointers stored in that structure and stores the results into 32-bit global variables, then decrements the byte, and re-enables interrupts through INTENA if the decremented value is not positive.
- `initialize_node_and_call_library`: Writes a small self-referencing node into the data area, storing a marker word and pointers that point back to the node's own location, then stores the stack-pointer-independent global address into an offset of the structure in a6.
- `disable_interrupts_then_checksum_copy`: Checks a flag at offset 44 of the structure in A1 and, if it is clear, may read the CIA port register B and port register A to test two fire-button bits, then disables interrupts through INTENA.
- `dispatch_through_pointer_with_counter`: Checks two global variables for the value minus one and, if both match, increments a 32-bit global counter and calls another routine when the counter exceeds 20999.
- `receive_keyboard_serial_sequence`: Disables interrupts and enables the DMA channel, then polls the CIA interrupt register for a serial keyboard byte, toggles the handshake bit in CIAA_CRA around a delay loop, and compares the decoded byte against a global byte table.
- `decrement_counter_and_mask_interrupts`: Writes 33024 to the DMA control register, then decrements a byte counter at offset 294 of the structure in a6.
- `increment_counter_and_disable_interrupts`: Loads the word at a fixed location into d0 and, if it is below 2, adds 1 to that same word; the routine then loads the exec base from absolute address 4 into a6, calls a routine that writes the interrupt enable register, increments a byte global variable at offset 294 of the structure in a6 and calls two other routines, and then returns.
- `store_minus_one_and_branch`: Writes the 16-bit value 65535 through the pointer in a0 and then branches unconditionally to a point inside another routine.

### Amazed.adf:SPACE (raw code file, load address unknown)

- `unavailable_raw_region_entry`: This is a 2306-instruction raw code region starting at the file beginning; it writes DMA control, interrupt enable and interrupt request registers, but its decompiled body could not be produced, so only its register accesses are known.
