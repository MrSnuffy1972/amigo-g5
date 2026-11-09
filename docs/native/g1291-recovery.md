# Game 1291 (targhan (1989)(silmarils)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01291). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 67 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 316 of 316 named (0 author symbols), 316 of 316 described (24 by rule, 292 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 68 of 68 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 58182 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5782 of 6697 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 626894 |
| code | 27580 |
| data | 167911 |
| padding | 16271 |
| unused | 62464 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Targhan (1989)(Silmarils)[a2].adf:T.X | 6 | 6618 | 314 |
| Targhan (1989)(Silmarils)[a2].adf:bootblock | 1 | 79 | 2 |
| Targhan (1989)(Silmarils)[a2].adf:PARCH.CO (raw code file, load address unknown) | 1 | 0 | 0 |
| Targhan (1989)(Silmarils)[a2].adf:TARGHAN.CO (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- targhan (1989)(silmarils)[a2]/Targhan (1989)(Silmarils)[a2].adf:PARCH.CO: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- targhan (1989)(silmarils)[a2]/Targhan (1989)(Silmarils)[a2].adf:TARGHAN.CO: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- targhan (1989)(silmarils)[a2]/Targhan (1989)(Silmarils)[a2].adf: blocks 1738-1759 (bytes 889856-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.35)
- 2 never-loaded region(s), 58182 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Targhan (1989)(Silmarils)[a2].adf:T.X

- `branch_into_routine`: Contains a single unconditional branch into the middle of another routine, so it transfers control and never returns on its own.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_libraries_and_memory`: Initializes the keyboard, graphics, and intuition libraries, then opens the dos library and verifies it; if any library fails to open it stores a nonzero flag and returns without doing anything else.
- `init_keyboard_device`: Saves the result of a function pointer call into a global variable, calls a second function pointer, then stores two pointers and two constants into fields of a structure and calls a third function pointer, returning if that call is non-zero.
- `release_library_resources`: Clears registers d0 through d3, then calls four routines by loading a base pointer from a global variable and invoking a function pointer at negative offsets from it, returning zero at the end.
- `call_close_device`: Library stub: loads its arguments and the exec library base, calls CloseDevice and returns its result to the caller.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `allocate_and_copy_pointers`: It calls two routines to allocate memory, stores the returned pointers, calls a third routine, and when a flag is set calls a fourth routine and copies four long values from a structure.
- `copy_buffer_and_call_routines`: It copies ten 16-bit values from a table into a global buffer, stores the end pointer, then clears two DMA registers and calls two routines.
- `close_screen_and_window`: Loads a pointer from a global variable, calls the function pointer at offset -72 of the base in another global, then loads a second pointer and calls the function pointer at offset -66, and returns.
- `call_indirect_function`: Saves two registers, loads a pointer from a global variable, clears a register, calls the function pointer at offset -234 of another global variable, restores the registers, and returns the saved register value.
- `wait_for_flag_bit`: Polls a byte at a fixed address and loops until bit 4 of that byte becomes set, then returns.
- `store_dma_channels`: Saves all registers, copies a pointer and its pointed value into global variables, disables interrupts, then copies ten 16-bit values from one pointer into consecutive 4-byte slots of another buffer, re-enables interrupts, restores the registers, and returns.
- `copy_four_plane_rows`: Saves all registers, loads four destination pointers and a source pointer, then runs a nested loop that copies one word from each of four source offsets into four destination buffers for each of 200 outer iterations, restores the registers, and returns.
- `write_bytes_then_switch_state`: Copies a longword from a global variable to two others, adds 2 to it, then writes the bytes 80 and 176 through the pointer it saved.
- `execute_sequence`: Calls two routines, sets bit 15 of the DMA enable register and writes a constant to an interrupt register, then switches to supervisor state, stores a routine address in a vector, and returns to user state.
- `reset_and_recurse_pointer`: Clears three bytes in a global variable, calls a routine pointer retrieved from a table offset by a global variable, stores the result in a global variable, switches to supervisor mode, installs a routine address into a vector table, restores the processor status, and calls another routine using a pointer from a table.
- `set_system_state`: Saves a base register, calls a function pointer and stores its result, disables interrupts, writes a global pointer into a hardware register, restores the status register, restores the base register, calls a second function pointer, and returns.
- `increment_and_loop`: Increments a global variable at offset 588, executes a loop three times by calling another routine, then calls a fourth routine before returning a concatenated pair of registers.
- `test_flag_then_branch`: It tests a flag byte at offset 348, and when that byte is zero it branches to another routine, otherwise it returns.
- `update_channel_countdown_unchecked`: Saves registers, computes an index from a register, checks two flag bytes and decrements one of them, and when it reaches zero sets interrupt and DMA mask bits for the channel, then optionally sets a third bit if a flag is negative and a related word is non-zero, restores the registers, and returns.
- `test_byte_and_branch`: Tests a byte in a hardware register table and returns when that byte is zero, otherwise it branches into another routine.
- `shift_and_test_register`: Copies a register value into d1, shifts it left by four bits, and tests a byte in a hardware register table, branching away when that byte is negative.
- `shift_byte_and_increment`: Tests bit 1 of the byte in d0 and, when it is set, branches away.
- `write_dma_registers_indexed`: It reads two index values from a structure, looks up a byte in a table, and when a stored byte does not match it writes paired DMA register values and sets a bit in the DMA control register.
- `write_paired_registers_indexed`: Writes two 16-bit register values into indexed hardware DMA registers, then sets a bit mask in the DMA control register derived from a bit position register, restores the registers, and returns.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `load_address_return`: Loads the address of a global variable into a register and returns immediately.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `decode_structure_string`: Checks a global flag and, if set, walks a list of structures until it finds one whose type byte is 1 or 2 and whose flag at offset 6 is 1.
- `expand_run_length_planes`: Walks a list of 6-byte entries pointed to by a1, and for each entry whose type is 1, 3 or 4 and whose flags at offset 1 do not set bits 4 or 5, expands a run-length encoded block into four 16-bit planes at offsets 32768, 24576 and 16384 of the buffer in the global pointer, plus a fourth plane at offset 0, using bit operations for the OR-combined cases; otherwise it skips the entry.
- `return_constant`: Returns the constant 3000 in d0 to the caller without any other effect.
- `poll_until_out_of_range`: It repeatedly calls a subroutine, incrementing a counter, and continues looping while the 16-bit result plus one stays below 32, then restores the registers and returns.
- `store_index_and_call`: Adds the value in d0 to a pointer from a table at offset 00BC four times, stores the 16-bit value in d1 at the resulting address, then calls a routine from a table at offset 00D4 passing the low byte of d1 and returns.
- `call_indirect_function_2`: It calls another routine, then invokes a function pointer from a table and returns the concatenated contents of two registers.
- `swap_registers_if_greater`: Swaps the d0 and d2 registers if d2 is greater, then swaps d1 and d3 if d3 is greater, and otherwise returns without changing any values.
- `store_byte_and_call`: Stores the low byte of d0 into a global variable at offset 0252, then calls a routine from a table at offset 00D4 and returns.
- `store_byte_and_call_pen`: Stores the low byte of d0 into a global variable at offset 0253, then calls a routine from a table at offset 00D4 and returns.
- `adjust_value_and_call`: Increments d0 to 4 when it equals 3, stores the low byte into a global variable at offset 0254, then calls a routine from a table at offset 00D4 and returns.
- `loop_until_nonzero_return`: Calls a routine in a loop, keeping its result, and continues while that result is zero.
- `store_byte_and_copy_structure`: It calls three routines, then based on the resulting byte either increments a column, resets a row, or stores the byte and copies four long values from a structure.
- `traverse_buffer_nonnull`: Reads a byte from the buffer pointed to by a0 and advances the pointer.
- `clamp_and_scale_values`: Clamps d0 to 39 and d1 to 24 when they exceed those limits, multiplies d0 by 9 and d1 by 8, and stores both products into global variables at offsets 024E and 0250.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `compare_flags_and_update`: It compares two sets of twelve flag bytes, and when they differ it finds the first differing bit, then either loads a value from a table or clears the second set and copies the first set.
- `read_flags_and_set_status`: Calls a routine from a table at offset 00D4, then reads six flags at offset 12 of a structure at offset 01CE and sets the corresponding bits into a byte stored in a global variable at offset 01CD.
- `read_joystick_and_set_bits_2`: Saves two registers, reads a joystick data register into a register, sets bits 3, 2, and 1 of a value depending on the button bits, and branches to a later routine.
- `read_joystick_and_set_bits`: Reads the joystick data register into d1, clears d0, and sets bits 3 and 1 of d0 depending on whether bits 1 and 0 of d1 are set.
- `check_d1_bits_and_set_d0`: Tests bit 0 of d1 and branches away when it is set.
- `test_flags_and_restore`: Tests bit 4 of register d1 and returns immediately when that bit is clear, then tests bit 2 of d2 and jumps away when it is set.
- `open_file_or_minus_one`: Calls the dos library Open vector with a name pointer from d1, and returns minus one when the handle is zero, otherwise returning the handle.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `lock_file_then_unlock`: Calls the dos library Lock vector, and when the lock succeeds calls the UnLock vector to release it, then clears the return register and returns.
- `close_and_check_sound_file`: Calls the dos library Close vector, and when the handle is zero reads a flag at offset 024A, opens a file named BLANC.SND, closes it, and clears the flag when its size equals 400.
- `jump_to_open_file`: Jumps to another routine that opens a file using a name pointer in d1 and returns minus one when the handle is zero, otherwise returning the handle.
- `call_dos_open`: Loads the DOS library base, calls the Open vector at offset 46, and returns the result, storing minus one when the result is zero.
- `call_delete_file`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `call_offset_function`: Reorders the incoming arguments and calls the function pointer at offset 42 from a value in a global variable, then returns.
- `write_then_set_flag`: Saves several registers, clears a value, calls a routine through a function pointer loaded from a global table, then stores the constant one into a global byte and returns.
- `dispatch_by_value`: Saves all registers, clears a value, and calls a routine in a loop until two global values are nonzero, then dispatches based on a third value or returns early when it is a specific constant.
- `restore_and_return`: Restores the saved registers and returns immediately to its caller.
- `initialize_screen_and_wait`: Initializes the screen by calling setup routines and copying table data, then repeatedly reads a byte from a buffer and calls a routine until that byte equals the carriage return character.
- `copy_and_clear_tables`: Copies 359 words from five source tables into a destination table while clearing each source word, repeating until the counter reaches minus one.
- `copy_source_tables`: Copies 359 words from a source table into five destination tables, repeating until the counter reaches minus one.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `dispatch_and_call_routines`: Dispatches based on the value in d0 by loading a return address, and when d0 is neither one nor two it calls a routine through a pointer and stores the result.
- `load_minus_one_and_jump`: Loads the value minus one into register d0 and then jumps into the middle of another routine, taking over its execution.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `check_range_increment`: Calls another routine and, when its return value is outside the range between 65456 and 65487, adds an address to a global variable and increments another by two.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `store_register_then_call_routines`: Stores a register into a custom chip register and calls five routines in sequence, then returns.
- `save_and_reset_registers`: Saves four custom register values into global variables, writes constants to several custom registers, and resets a CIA register.
- `clear_bits_and_wait`: Clears bits 7 and 3 of a hardware register, busy-loops a 100-iteration counter while repeatedly clearing bit 3 of that same register, and finally calls two other routines.
- `decrement_counter`: Decrements a counter from 3000 until it reaches minus one and then returns.
- `wait_bit_clear`: Waits in a loop while bit 5 of the CIA register is set and returns as soon as it becomes clear.
- `poll_fire_button_loop`: While bit 4 of a CIAA port is set, it clears and sets bits 0 and 1 of a CIAB port, calls another routine, and then returns.
- `poll_cia_and_delay_loop`: Polls bit 4 of the CIAA port and, while it is set, calls a routine in a loop.
- `pulse_serial_port_until_ready`: Clears a bit in a global variable, writes a control word and a longword to a custom chip block, then polls a status register for a bit a fixed count of times or until that bit changes.
- `clear_and_set_offsets`: Clears a word at offset 36 of the structure in a6 and stores the value 16 at offset 150 of the same structure, then returns.
- `wait_for_status_bit_set`: Reads the interrupt control status byte of CIA B and loops until its bit four is set, then returns.
- `latch_status_into_four_words`: Clears bit 3 of a global status byte and sets bit 7, then writes each of four global words ORed with the top bit into four 16-bit slots of the structure in a6.
- `search_bit_pattern`: Walks a table of 32-bit words looking for the pattern 17545 in the high word, shifting each word bit by bit and counting matches, then stores a flag when found and returns.
- `search_table_for_pattern`: Walks a table of 32-bit words looking for the pattern 17545 in the high word, shifting each word bit by bit and counting matches, then stores the count either into a global variable or the caller's pointer and sets a global flag accordingly.
- `jump_and_invoke_function_pointers`: Jumps into the middle of another routine, then stores the stack pointer into a global variable, loads four function pointers from a global table, and invokes them through a base pointer before looping over a table of words until it calls a final routine.
- `dispatch_node_update`: Skips its first instruction by jumping into another routine, then loads a node from a table and clears two flags in a structure.
- `dispatch_via_indexed_jump`: Pushes a return address onto the stack, reads a byte from a pointer, scales it by two, and jumps through a word table indexed by that scaled value in a2.
- `nop_then_branch`: Performs a no operation and then unconditionally branches into another routine, transferring control without returning.
- `swap_pointer_and_dispatch`: Swaps a global pointer in d6 with a saved slot in a local variable, then reads a byte from a pointer, scales it by two, and jumps through a word table loaded from a local address.
- `setup_and_call_dispatch`: Copies a word from a pointer into an indexed table, sets a bit in another indexed location, and ORs several constants into registers d0 and d1.
- `call_and_restore_dispatch`: Jumps to another routine, saves registers d1 and d2 on the stack, and copies register d0 into d7.
- `store_byte_via_offset`: Reads a signed word from a source pointer, stores a byte from the following location into a global buffer at offset one plus that word, then restores the registers and returns.
- `clear_global_byte_89`: Clears the byte at offset 89 of a global variable and then falls through into the following routine.
- `scan_byte_and_update`: Calls a routine, reads a byte from a structure into a register and returns when it is positive, compares it against 254 and jumps when equal, then loads a second byte into a register, advances the pointer, and jumps to a shared section before returning.
- `scan_list_and_update`: Saves several words, then in a loop reads values from a table, adds them to stored words, toggles a flag byte, and calls a routine, decrementing a counter until it reaches one, after which it restores the saved words and returns.
- `test_flags_and_call_routine`: Tests a global byte and branches when it is nonzero, then tests another global byte and branches when it is zero, after which it calls a routine and branches when the result is zero.
- `read_indexed_byte_and_loop`: Reads a byte from a table indexed by a register into a register, returns when it is zero, calls a routine, and loops back while the value is zero.
- `init_structure_and_scan_list`: Initializes a structure pointed to by a1 by copying several bytes and words from a global structure in a6 into it, then calls a routine and, while a flag byte at offset 0 of the structure is nonzero, reads a word from a list indexed by d1 and stores it, returning early when a global variable is zero.
- `scan_list_until_flag_clear`: Calls a routine and, while the result is nonzero, reads a flag byte from a list indexed by d1.
- `load_word_and_check_zero`: Copies register d1 into d2, loads a word from an offset indexed by d1 into d1, and returns immediately when that value is zero.
- `compare_table_word`: Reads a word from a table indexed by d1 at offset 2 of a structure in a1 and compares it with a word at offset -16 of the frame in a6, then returns.
- `check_stack_pointer_zero`: Compares the stack pointer against zero and returns.
- `load_word_and_check_zero_2`: Clears d2, loads a word from offset minus 18 of the structure in a6 into d1, and returns when that word is zero.
- `loop_table_comparisons`: Copies a word from d1 into d2, loads a word from a table indexed by d2 at offset 4 of a1 into d1, and returns if that value is zero.
- `compare_pointer_value`: Compares the 16-bit value pointed to by a7 with zero and returns.
- `load_word_and_branch`: Clears d2, loads a word from offset minus 18 of the structure in a6 into d1, and returns when that word is zero.
- `conditional_table_check`: Copies a word from d1 into d2, loads a word from a table indexed by d2 at offset 4 of a1 into d1, and returns if that value is zero.
- `compare_a7_pointer_zero`: Compares the 16-bit value at the address in a7 with zero and then returns.
- `store_d0_and_check_d2`: Loads a word from a global variable into d0 and returns when it is zero.
- `link_node_after_head`: Stores the two data registers and the off-register value into a structure at offsets 4 and 6, copies a byte from a global variable to offset 1 and sets the flag byte at offset 0 to 255, then relinks the node by pointing offset 6 at the following node and pointing that following node's offset 6 back at this node.
- `call_pair_then_store_value`: Loads the constant ten into d0, calls a routine via an indexed jump, calls another subroutine, stores the value in d7 into a frame word at offset -42 of a6, and returns.
- `store_two_subroutine_results`: Calls a subroutine and stores the value in d7 into a frame byte at offset -38 of a6, then calls the subroutine again and stores the value into the next frame byte at offset -39, returning afterward.
- `compute_scaled_vector_offset`: Loads a reference vector from a table, subtracts it from a source vector, then applies a bit-shifted scale factor divided by a z-distance to produce scaled x and y deltas.
- `accumulate_signed_bytes`: Checks the sign of a byte at offset 1 of a structure in a0 and returns early if it is non-negative.
- `test_flag_bit_branch`: Tests bit 7 of the byte at offset 139 of the global structure and, if that bit is set, branches to the routine at 12912, otherwise it returns.
- `lookup_indexed_table_entry`: Reads a base pointer from the stack frame, adds an offset from a global table, multiplies the index register by four and adds it to a second table entry, then returns the doubleword found at the resulting address.
- `call_routine_then_return`: Saves the frame pointer a6 on the stack, loads a global pointer into a6, calls another subroutine, restores the previous a6 from the stack, and returns.
- `call_and_store_d7`: Calls the routine at 13020, stores the value in d7 through the byte at offset 130 of the global structure, loads a1 from a global pointer, and calls the routine at 12942, returning when the word at offset 18 of the structure in a6 is zero.
- `call_then_branch`: Calls the routine at 13020 and then unconditionally branches to the routine at 12942.
- `call_until_zero_result`: Clears the word in d2, loads a frame word from offset -24 of the structure in a6, and returns immediately when that word is zero.
- `relink_node_by_flag_byte`: Reads a flag byte at a list position indexed by d1 and, if it is negative, saves the node's data word and links the node around d2.
- `update_linked_list_node`: Compares the value in a data register with the link word of the first node and, if it matches, replaces that node's data word; otherwise it walks the list following link words until it finds a matching data word and replaces it.
- `store_three_subroutine_results`: Calls a subroutine and stores the value in d7 into a word at offset 0 of a frame in a6, calls it again and stores the value at offset 2, calls it a third time and stores the value at offset 4, then returns.
- `save_stack_frame_words`: Copies the two words held at the start of the stack frame into two global variables and returns without any other work.
- `shift_node_list_positions`: Subtracts three global values from the three words in the stack frame and, if any difference is nonzero, walks a linked list marking each node whose flag byte is zero and adding the three deltas to its position words.
- `offset_record_and_mark_list`: Adds three words from the register frame to the position words of a record, walks a linked list marking each node whose flag byte is zero, and clears bit 7 of the record's flag byte.
- `dispatch_table_event_handler`: Polls a status byte at offset 181 and, if a pending event is set, saves the registers and clears it.
- `process_table_entry`: Checks a flag bit in a structure and, if set, copies coordinates into global variables and computes a width, then processes a linked list of records by copying fields and rewiring links based on a per-record type byte.
- `process_table_entries`: Copies a 16-bit value from one table slot to another and stores a global word into a structure, then repeatedly reads entries from a table and, based on a marker byte, either updates a structure's fields or searches for a matching entry by comparing a global value.
- `copy_values_to_table`: Calls a routine, copies a word from one table offset to another, copies two longwords and a word from global variables into a structure, stores a global byte, then calls two more routines before returning.
- `copy_table_entry`: Calls a routine, copies a word from one table offset to another, stores a global word into a structure, writes a held value back into that global word, then returns.
- `copy_and_update_table_entry`: Calls a routine, copies a word from one table offset to another, copies two longwords and a word from global variables into a structure, stores a global byte, and calls two more routines before returning.
- `initialize_global_bounds`: Reads a word from a structure and stores the high signed value into two global variables and the low signed value into two other global variables.
- `update_bounds_from_structure`: Reads a pointer and two words from a structure, then updates four global bounds to the smaller minimum and larger maximum values derived from those fields, and returns.
- `update_linked_list`: Walks a linked list comparing several fields of each node against values from the caller and, when a node matches, rewires the surrounding links, clears the caller's flag byte, and updates a caller index if needed.
- `process_clipping_list`: Returns early when a flag bit is set, otherwise clears a byte, calls a routine, and iterates over a list of entries calling routines based on their flags and values.
- `initialize_buffer_constants`: Loads a global pointer into a0, copies four word values from global offsets into consecutive words starting at that pointer, stores the pointer back, and returns if the pointer does not exceed a threshold.
- `copy_table_block_loop`: Loads a pointer from a global variable, then loops five times copying four consecutive 16-bit values from the table into four global variables at offsets 90, 98, 92 and 96, calling a routine after each copy, until the pointer reaches the end marker at offset 184.
- `clamp_rect_bounds`: Clears a flag byte, then returns early if any of the four rectangle values fall outside the global bounds; otherwise it clamps each value to the nearest bound and stores the results into four global variables.
- `expand_rect_bounds`: Resolves a pointer from a structure, checks that several fields are within range, and if so expands four global bounds to include the area by storing the clamped values and setting a flag; otherwise it clears the flag.
- `load_clipping_region`: Reads a pointer from a structure at offset 8 using an index, adds the pointed value to it, and copies four adjacent words from that structure into a global table, then returns.
- `resolve_indexed_pointer`: Adds two offsets from a table to a base pointer and then adds an indexed offset to the result before returning.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `clear_table_entry_conditional`: Calls a subroutine to load a signed index into d7, and if that index is non-negative it writes zero to the byte four before the word located at that index within a table pointed to by a global variable, otherwise it returns.
- `copy_table_blocks`: Reads a byte from a structure, decrements it, and copies words from a source table into a global table in a loop, using a second table when the byte equals thirty-one, then sets a global flag.
- `set_global_flag`: Sets a global byte to two hundred and fifty-five and returns.
- `store_word_conditional`: Calls the routine at 24242 and then the routine at 24228.
- `store_word_at_index_unchecked`: Stores the 16-bit value in d6 into the table slot selected by the index in d7 without any range check, then calls a subroutine that sets a global byte to two hundred and fifty-five.
- `mask_and_add_nibbles`: Masks two registers to their low nibbles, adds them together, and returns unless the sum exceeds fifteen, in which case it branches away.
- `tiny_move_rts`: Small routine of 2 instructions that copies 15 to d1, then returns.
- `tiny_clr_rts_2`: Small routine of 2 instructions that clears d1, then returns.
- `subtract_nibbles_check_negative`: Masks the low nibbles of d0 and d1, subtracts d0 from d1, and if the result is negative it returns without doing anything else.
- `load_fifteen`: Loads fifteen into a register and returns.
- `clear_register`: Clears a register and returns.
- `copy_words_conditional`: Clears a byte at offset 139 of the global structure, calls two routines, and returns early when that byte equals one of the values one, three, five, or seven.
- `copy_and_shift_words_conditional`: Reads a byte from the location before a pointer, decrements it, and if negative replaces it with fifteen, then copies that many 16-bit words from the pointer into a global table.
- `set_status_from_nibble`: Sets a global byte to fifteen, derives a value from the high nibble of a register, stores it in two globals, sets a final flag, and returns.
- `compare_and_fix_nibbles`: Iterates over a table of 16-bit values, comparing each nibble of a source table with the matching value in a destination table and, when they differ, storing a corrected nibble value returned by another routine.
- `split_and_store_nibbles_2`: Reads a 16-bit value from a table and splits it into its two nibbles, then stores the nibbles into a slot in another table as a 16-bit value.
- `split_and_store_nibbles`: Reads a 16-bit value from a table and splits it into its two nibbles, then stores the nibbles into a slot in another table as a 16-bit value with the high nibble in the high byte and the low nibble in the low byte.
- `accumulate_and_call`: Calls the routine at 20698, shifts the word in d1 left by 8 bits, combines it with d3, and stores the result into the slot pointed to by a1 plus d7.
- `load_constant_branch`: Calls the routine at 24228, then checks the global byte against the values 1, 5, 3, and 7, branching to 20696 when any matches.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `call_add_nibbles_with_one`: Sets d0 to one and calls a routine that masks two values to their low nibbles and adds them, then returns.
- `load_one_call_subtract`: Loads the constant 1 into d0, calls the routine at 20198, and returns.
- `adjust_flags_via_offset`: Calls a subroutine five times pushing each result on the stack, then pops them into registers, and if a signed word at offset 22 of the structure in a6 is non-negative it adjusts two bits in the byte following a pointer derived from that word and calls a routine when d0 is nonzero.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `noop_register`: Performs a no-op bitwise OR on a register and returns.
- `write_zero_byte`: It contains a single instruction that writes a zero byte to the memory location pointed to by a0.
- `store_name_and_store_registers`: Loads a word from the stack frame at offset 14 into a global word, then repeatedly calls a routine that copies a name with an extension and stores a byte from register d7 into three global bytes before returning when a global word becomes zero.
- `copy_name_with_extension`: Copies the byte string from the input pointer into a global buffer, inserting a chosen extension letter after the final dot, then stores the argument register into a status word and calls a channel-selection routine.
- `store_name_and_check_flag`: Loads a word from the stack frame at offset 14 into a global word, then repeatedly calls a routine that copies a name with an extension and stores a byte from register d7 into three global bytes before returning when a global word becomes nonzero.
- `compute_signed_quotient`: Computes the signed quotient of a global byte shifted left eight bits divided by another global word, stores the result, forces it to one when the quotient is zero, negates it, sets a status byte to three, and calls a channel-selection routine.
- `select_channel_by_index`: Compares two bytes from a global variable against four sets of reference values and, when one set matches, stores a constant byte, the incoming byte, and several structure fields at fixed offsets of a pointer in a6.
- `init_struct_from_register`: Initializes a 20-byte structure pointed to by a6, writing a constant byte at offset 0, the incoming byte from d0 at offset 1, several values from global variables at offsets 2, 4, 6, 8, 10, 12 and 14, clears the byte at offset 7, and finally overwrites offset 0 with the value 2 before restoring the registers and returning.
- `divide_byte_by_globals`: Calls a subroutine to load a byte into two global variables, then divides the resulting value by two different global divisors, skipping each division when its divisor is zero and storing the two quotients in separate globals.
- `test_flag_bit_branch_2`: Tests bit 7 of the byte at offset 139 of the global structure and, if that bit is set, branches to the routine at 21836, otherwise it returns.
- `lookup_record_pointer_entry`: Loads a base pointer from a structure in a6, adds two stored word offsets to it, then indexes into a table using the value in d7 scaled by four and returns the sum of the pointed-to value and the pointer itself.
- `switch_a6_then_call`: Pushes a6 onto the stack, replaces it with a global pointer, calls a subroutine that indexes a table using the value in d7, then restores a6 and returns.
- `set_flag_then_branch`: Stores the value one into a byte at offset 139 of a global structure, then unconditionally branches into another routine.
- `call_routine_and_check_result`: Calls a nested routine three times, then if its result is negative clears two registers and branches, otherwise it branches to another routine when the returned byte is neither 1 nor 2.
- `store_table_entry_if_match`: Calls a nested routine and, when a byte it points to is 1 or 2, stores its result plus sixteen and a companion value into a table selected by the low five bits of a register argument.
- `process_returned_byte_and_store`: Clears a byte in a global variable, then calls a routine that returns a byte and a pointer.
- `call_and_check_byte`: Calls two routines in succession, then loads a byte from the address in a0 into d1 and returns immediately if that byte is zero.
- `increment_and_store_values`: Adds 6 to a passed value and stores it at offset 58, then repeatedly increments a counter in d7 while storing it at offsets 18, 20, 30, and 34, computing two quotients via unsigned division and storing them at offsets 36 and 38, and finally branches to another routine based on a flag at offset -6 of the structure in a1 before returning.
- `call_and_set_global`: Calls another routine, then stores the value 1 into a global variable and returns.
- `call_routine_then_return_2`: Calls a subroutine five times in succession and then returns.
- `copy_register_data`: Pushes the value in d7 onto the stack, calls another routine, copies d7 into d6, restores d7 from the stack, and returns.
- `tiny_move`: Small routine of 1 instruction that copies d7 to d6.
- `jump_via_indexed_table`: Clears d0, reads the next byte from a byte array pointed to by a3, uses it to index a word table pointed to by a5, and jumps to the address stored at that table entry.
- `swap_globals_and_index_table`: Swaps the two words stored at the global variables at offsets 202 and 210, then indexes a word table at offsets 210 through 214 using the next byte from the byte array in a3 and jumps to the address found there.
- `save_registers_and_set_flag`: Saves all registers, calls a routine that loads a global address and returns, restores the registers, then clears a global word and calls a routine that sets a global byte to 255 before returning.
- `initialize_window_buffer`: Sets up a windowed buffer, stores several fixed values at fixed offsets, then loops adding 36 to a counter until it reaches zero or a boundary, writing the running offset and a zero longword into consecutive entries and finally clearing a trailing word.
- `open_file_by_name`: Copies a null-terminated name from a register into a global buffer, appends an extension letter after the last dot depending on a global flag, then opens a file by that name and initializes a block of global fields from the resulting handle.
- `process_window_in_loops`: Sets a global word to one or eight depending on a bit in another global byte, then repeatedly calls two helper routines in nested loops while decrementing a counter register.
- `compare_and_retry_call`: Compares a3 with a4 and, if a4 is greater, returns the byte pointed to by a3.
- `write_byte_at_offset_guarded`: Compares a window pointer against a limit and, when it is within bounds, writes the low byte of d0 through the pointer and advances the pointer; otherwise it recomputes the pointer from a stored base, decrements a counter, and returns without writing when the counter is not positive.
- `shift_table_pointers_by_gap`: Finds the smallest pointer in a sorted table that is greater than a given pointer, then shifts the block of pointers it occupies down by the gap and updates every pointer in a second table that falls inside that block.
- `remove_table_entry_2`: Removes one entry from a fixed-size table of 48 slots by shifting the trailing entries down to close the gap, then decrements a slot counter.
- `jump_back_into_routine_2`: Jumps back into the middle of another routine via a single unconditional branch and does nothing else.
- `jump_back_into_routine`: It contains a single unconditional branch to an earlier address, so it jumps back into the middle of another routine and does nothing else.
- `shift_record_offsets`: Walks a linked list of records in a global table, and for each record whose two 24-bit values exceed a value in d3, subtracts a value from d1 from each and stores the difference back as a 24-bit value at offsets 10 and 20.
- `pop_register_and_compare_pointers`: Pops a register from the stack into d0, reads a word from a table indexed by that value through a0, and returns if that word is zero.
- `forward_call_preserving_registers`: Pushes two data registers and two address registers, calls a nested routine, pops the registers back unchanged, and returns the original register values concatenated.
- `binary_search_word_table`: Binary-searches a table of pointers whose entries begin with a 16-bit key, comparing the key in d2 against each midpoint and narrowing the range until a match is found or the range is exhausted, returning in either case.
- `initialize_list_structure`: Loops adding 6 to a counter while writing the running offset and a zero longword into consecutive entries of a list pointed to by a0 until the pointer reaches a boundary, then clears a word at offset 4 and stores the values 1 and 6 into two global variables.
- `insert_pointer_into_sorted_list`: Inserts a pointer into an ascending sorted list of pointers stored in a global table, keeping the list ordered, then increments the list count and calls two other routines.
- `build_and_store_block`: Builds a 46-byte block in a global table, filling it with pointer fields, small constants, and zeroed words, then returns; if the block cannot be placed it returns without writing.
- `remove_table_entry`: Removes one entry from a fixed-size table of 48 slots by shifting the trailing entries down to close the gap, then decrements a slot counter and updates a linked list of active slots keyed by the passed index.
- `open_file_by_bit`: Tests bits of the incoming byte and, for the matching case, repeatedly opens a file by a name in a register until the handle is valid, storing the result in a global variable.
- `pass_through_register_value`: Saves data registers on the stack, extends the sign of the value in d0, calls a routine, restores the registers, and returns the original d0 value unchanged.
- `retry_call_until_success`: Loads a value from a global variable into d0 and calls a routine, looping back to retry the call whenever the returned value is negative and otherwise returning.
- `retry_open_file_loop`: Loads a value from a global variable, extends it to a long, stores it at offset 40, then repeatedly opens a file by a name in d1 until the handle is non-zero, returning that handle.
- `retry_open_file`: Loads a file from disk using the name pointer in d1, then repeatedly retries the open until it succeeds, storing the resulting handle in the global variable at offset 40 and returning it.
- `retry_delete_file_loop`: Calls a routine that deletes a file, then returns if the result is non-negative, otherwise it branches back to repeat the call while the result is negative.
- `loop_call_until_success`: Loads the constant 9 into d0, calls a routine, and branches back to the start to repeat the call while the returned value is negative.
- `retry_call_with_all_registers`: Saves all data and address registers, loads a value from a global variable into d0, and calls a routine, looping back to retry when the returned value is negative and otherwise restoring the registers and returning.
- `call_and_return_if_negative`: Saves a data register, loads a global longword into it, calls a routine through a function pointer, restores the register, and returns when the returned value is negative.
- `loop_call_until_success_small`: Loads the constant 7 into d0, calls a routine, and branches back to the start to repeat the call while the returned value is negative.
- `decrement_and_call_routines`: Increments a byte at offset 175 and wraps it if it overflows, then decrements a counter at offset 179 and calls a routine when it reaches zero, decrements another counter at offset 177 and clears a flag at offset 176 when it reaches zero, and finally branches to a routine based on the sign and zero state of a shared variable.
- `clear_flags_and_call_conditionals`: Calls a routine and clears a shared variable, then calls routines conditionally on the sign and zero state of that variable and of a second byte.
- `decrement_and_clamp_values`: Decrements a word at offset 4 of the structure in a3 and returns when it reaches zero.
- `clear_record_and_call`: Clears a word at offset 6 and a word at offset 10 of the structure in a3, then clears the bytes at offsets 0 and 1 of that structure and calls a routine before returning.
- `clear_four_record_blocks`: Sets a3 to the address at offset 8d0, calls a routine, then repeats this for the addresses at offsets 8e0, 8f0, and 900, and finally returns.
- `clear_record_state_fields`: Clears the bytes at offsets 0, 1 and 2 and the words at offsets 6 and 10 of the record pointed to by a3, then returns.
- `update_counters_and_call`: Tests three global words and branches when the first two are nonzero or the third is zero or negative, otherwise subtracts a global word from another and, when the result is positive and nonzero, decrements a counter, stores the result, and jumps to a shared section that calls two routines before returning.
- `clamp_and_decrement`: Adds a global value to another global word, clamps the sum against a reference and a maximum constant, sets a flag when the reference is exceeded, decrements a counter, and calls two routines in sequence.
- `decrement_global_and_call`: Decrements a global variable at offset 20 and then calls two different routines in sequence.
- `call_interrupt_and_clear`: Calls the interrupt handler via a trap, clears a global byte, and returns.
- `return_to_caller`: Divides the constant 5 by zero, which overflows, and then returns without touching any data.
- `describe_data_table`: Consists only of instructions that change nothing, so it clears nothing, calls nothing and does not alter any data.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `compare_table_pointer_and_store`: Compares the value in a3 against four different table addresses and, if none match, stores the zero value of d1 at the following word of that table, then returns.
- `empty_register_filler`: Contains only filler instructions that write zero to data registers, performing no operation and returning immediately.
- `noop_or_d0_table`: Performs a series of no-op OR operations on data register d0, which change nothing, then returns.
- `write_four_zero_bytes`: Writes the byte value zero to four consecutive bytes of memory starting at the address in a0.
- `store_four_zero_bytes`: It writes four zero bytes in a row to a global variable through a pointer, leaving the data cleared.
- `store_four_zero_bytes_2`: It writes four consecutive bytes of zero to a global variable through a pointer, one byte at a time.
- `store_zero_bytes`: It contains four consecutive zero bytes that other code references by address, and it does nothing else.
- `store_zero_data_table`: It contains only a single ori.b #0 to a data register, which changes nothing, so this range is most likely a zero-filled data table that other code refers to by address rather than working code.
- `no_op_register_or`: Performs 64 sequential byte-wise OR operations of the value zero into data registers, each of which leaves the register unchanged, then returns.
- `compare_register_and_store_zero`: Compares the value in register a3 against four fixed addresses and, if it matches any of them, stores a zero byte at the location one past that address, otherwise it calls another routine and returns.
- `setup_blitter_channels`: Sets up two blitter channels, waits for the vertical blank, then programs five blitter source pointers and loops until the vertical blank ends.
- `conditionally_copy_table_rows`: Checks whether a value read from a global variable is outside the 0-320 range, and if so copies a block of 40-word rows from one table into another using five source pointers and blitter-wait loops, otherwise it returns.
- `configure_blitter_channels_2`: Reads three coordinates from a global variable, converts them to 4-by-4 tile units, and programs the blitter channels with source, destination and size registers while repeatedly waiting for the vertical blank until the channel is free.
- `configure_blitter_channels`: If two flag bits are clear, configures the blitter channels and waits for the vertical blank, otherwise it stores a computed pointer.
- `reverse_copy_byte_block`: Computes block dimensions from the source and destination sizes, then copies a rectangular block of bytes from one memory area to another, reversing the order of rows and reversing the order of bytes within each row.
- `empty_data_area`: It is a zero-filled data area that other code refers to by address, containing a single ori.b #0, d0 instruction that is decoded from a zero word.
- `clamp_and_copy_rect`: Clamps a rectangle defined by coordinates in a structure to a fixed range, stores the result into four global variables, records a mode byte, then programs the blitter channels and copies a byte block.
- `select_routine_by_index`: Checks a byte in a global variable and returns early when it is set, otherwise selects a routine address by comparing a3 against four known entry points and storing a matching index in d3 before jumping to the chosen routine.
- `configure_audio_channel`: Writes a byte from a global variable to offset 1 of the structure in a3, loads a pointer and two values from global variables, sets d4 to a byte from a global variable, clears d2, and calls a routine before restoring the registers and returning.
- `start_audio_channel_playback`: Calls a system library routine through a library base, then programs an audio channel's pointer, length, period and volume from its arguments, capping the volume at 100 and computing a per-channel mask.
- `clear_channel_flag_if_negative`: Tests a per-channel flag byte and, if its sign bit is set, clears that flag byte, clears a byte through a pointer, and computes a channel bit mask shifted left by seven into a global variable.
- `process_interrupt_channels`: Saves some registers, combines interrupt request and enable flags, and calls a routine for each of bits 7 through 10 that is set, setting d3 to a channel index before each call, then writes a constant to an interrupt register and returns.
- `decrement_channel_counter`: Decrements a per-channel word counter through a pointer and, when it reaches zero, clears a byte through a pointer, clears the per-channel flag byte, and computes a channel bit mask shifted left by seven into a global variable.
- `process_four_channels`: Saves d3 and a3, then for each channel index from 0 to 3 sets d3, loads the corresponding table address into a3, and calls a routine before restoring the registers and returning.
- `pass_through_registers`: Performs arithmetic on d1 and d0, then calls a routine through a pointer at a fixed address and returns the modified register pair.
- `conditionally_call_interrupt_update`: Sets a register to one, calls a routine that updates interrupt state, then restores the register and returns its original value.
- `forward_clamped_value_to_globals`: Masks a signed word in d0 to its lower seven bits, shifts it right by one, and forwards the result to another routine that clamps it and stores it through several pointers.
- `compute_scaled_seven_bit_value`: Masks the value in d0 to seven bits, multiplies it by eight, subtracts seven, stores the result in a global variable, and returns.
- `call_and_restore_registers`: Saves three registers, calls another routine, restores the registers, and returns.
- `clamp_and_write_audio_values`: Clamps the value in d0 to a maximum of 100 and stores it through four pointers, then returns.
- `store_word_at_offset_216`: Stores a 16-bit value from register d0 into a word at offset 216 of the structure in a0, then returns.
- `setup_audio_interrupt_vector`: Saves registers into custom chip registers, derives a value from a table using an index, and returns when a global byte is negative.
- `write_value_then_jump`: Writes the value 1024 at offset 154 of the structure in a0, then jumps to another routine.
- `setup_audio_interrupt_state`: Saves four register values into global variables, calls a routine through a function pointer held in a global table, and clears a block of global bytes.
- `check_global_word_then_branch`: Tests a word in a global variable and returns early when it is non-zero, otherwise it branches to another routine to handle the zero case.
- `configure_registers_by_d0`: If d0 is negative it sets two global variables and returns, otherwise if d0 is zero it copies one global variable into another, clears it, masks two custom registers, and writes two constants to hardware registers before returning.
- `set_registers_and_reload_value`: Writes constant values to the audio period and interrupt-enable registers, clears a byte in a custom register, and reloads a global variable from another one when the cleared byte is zero.
- `clear_flags_and_reload_value`: Clears two global variables, masks one, clears a bit in a byte through a pointer, writes constant values to the audio period and interrupt-enable registers, and conditionally reloads a global variable when a flag is zero.
- `conditionally_set_control_bits`: If a global byte is non-zero, it stores a handler pointer into a global variable and writes two constant words to global variables.
- `test_bit_and_dispatch`: Saves several registers, loads the custom chip base address, copies a stack word into a register, tests bit 10 of a masked value and returns when it is clear, then tests bit 8 of another masked value and jumps to an address loaded from a table after pushing a routine address.
- `dispatch_by_bit_test`: Tests bit ten of a register and returns when it is clear, otherwise loads two pointers from a table, pushes a routine address onto the stack, and jumps to the first pointer.
- `dispatch_handler_by_register`: Tests bit 7 of a register and returns when it is clear, otherwise loads pointers from a table, pushes a routine address onto the stack, and jumps to a handler looked up from a table.
- `dispatch_handler_by_bit`: Tests bit 9 of d1 and returns when it is clear, otherwise pushes a routine address onto the stack and jumps to a handler looked up from a table.
- `mask_and_check_d1`: Loads 1920 into d1, masks it with two words from the structure in a0, and returns if the result is zero.
- `noop_byte_write`: Performs a no-op byte write of zero through a data pointer and returns, doing nothing to the referenced data.
- `empty_zero_word`: Contains a single zero word that decodes as ori.b #0, d0; other code refers to this range by address.
- `writes_zero_byte`: It writes a byte value of zero through a pointer in a0, then returns.
- `write_zero_byte_2`: It is a single instruction that writes a zero byte to the memory location pointed to by a0, then returns.
- `store_zero_byte`: Writes a zero byte to the byte pointed to by the register in a0, then returns.
- `empty_zero_words`: Contains two consecutive zero words that decode as ori.b #0, d0; other code refers to this range by address.
- `write_zero_byte_3`: It writes a zero byte to the memory location addressed by the register pair a0 and d0, then returns.
- `initialize_timer_and_pointers`: Writes two constants to custom registers and sets a bit in another custom register, then stores two routine pointers into a structure, clears both globals, and stores a value into a structure retrieved from a global variable.
- `reset_ciab_registers`: Writes zero to the control register of CIAB, sets the timer high register to 2 and sets bits 0 and 3 of the other CIAB register, then returns.
- `adjust_timer_and_store`: Subtracts a value from a global word and returns early when the result is carry or zero, otherwise adds four to a global word, loads a pointer, and stores a constant at a fixed offset.
- `dispatch_byte_via_jump_tables`: Reads a byte from a global table and branches to a routine from a jump table based on its value, otherwise reads the next byte and dispatches to another jump table.
- `decrement_pointer_then_load`: Decrements a0 by one, loads a byte from a global variable into d0, then jumps to another routine.
- `clear_byte_and_branch`: Clears a byte in a global variable, loads a pointer into a register, and branches to another routine.
- `adjust_d0_and_save_pointers`: Subtracts 128 from d0, stores the result into a global variable, saves a0 into another global variable, then jumps to another routine.
- `store_table_via_scaled_byte`: Clears a register, reads a byte from a pointer and increments the pointer, scales the byte, then stores a word and a long from a table into a structure and branches to another routine.
- `update_amiga_audio_channel_0`: Computes a length from a timer's stored start and end values, then writes the halved length and a pointer to the start into the audio channel 0 registers of the Amiga sound hardware, updating the channel bookkeeping.
- `push_jump_table_entry`: Pushes a word from a jump table indexed by a register onto the stack and branches to another routine.
- `dispatch_audio_channel_update`: Reads a byte from a global table and branches to a routine based on its value, then updates the audio channel pointer and volume fields of a structure in a5 using values from other tables.
- `store_pointer_from_table`: Loads a table address into a6, stores the word at the offset of d0 from that table into offset 4 of the structure in a5, then jumps to another routine.
- `advance_pointer_and_branch`: Increments the pointer in a0 by 2 and then branches to another routine.
- `set_bit0_pointer_and_width`: If bit 0 of a byte at offset 3 of the structure in a0 is set, computes a width from two fields before the pointer in a1 and stores it at offset 164, stores the pointer plus a field at offset 160, and sets a word at offset 154 to 128.
- `set_bit0_handler_and_mask`: If bit 0 of the byte at offset 3 of the object is set it stores a routine pointer into a global variable, then it stores the value 128 at offset 156 of the object.
- `set_mode1_and_masks`: Stores the value 1 at offset 150 and the value 128 at offsets 154 and 156 of the object.
- `set_bit1_pointer_and_width`: If bit 1 of a byte at offset 3 of the object is set, computes a width from two fields before the pointer and stores it at offset 180, stores the pointer plus a field at offset 176, and sets a word at offset 154 to 256.
- `set_bit1_handler_and_mask`: If bit 1 of the byte at offset 3 of the object is set it stores a routine pointer into a global variable, then it stores the value 256 at offset 156 of the object.
- `set_mode2_and_masks`: Stores the value 2 at offset 150 and the value 256 at offsets 154 and 156 of the structure in a0, then returns.
- `set_bit2_pointer_and_width`: If bit 2 of a byte at offset 3 of the object is set, computes a width from two fields before the pointer and stores it at offset 196, stores the pointer plus a field at offset 192, and sets a word at offset 154 to 512.
- `set_bit2_handler_and_mask`: If bit 2 of the byte at offset 3 of the object in a0 is set, stores the address of this routine into the pointer at offset 1620, then always stores the value 512 at offsets 154 and 156 and the value 4 at offset 150 of the structure in a0.
- `set_mode4_and_masks`: Stores the value 4 at offset 150 and the value 512 at offsets 154 and 156 of the structure in a0, then returns.
- `decrement_counter_set_bit3_mode`: If bit 3 of the byte at offset 3 of the object in a0 is set, decrements a global counter and, when it becomes negative, stores the value 8 at offset 150 and 1024 at offset 154.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `is_no_op`: Contains a single no-op instruction and does nothing.
- `is_no_ops`: Contains two consecutive no-op instructions and does nothing.
- `noop_byte_or_d0`: Performs a no-op byte OR of register d0 with the immediate value 0, leaving the register unchanged, then returns.
- `noop_d0_register`: Executes a single ori.b #0, d0 instruction, which is a no-op that leaves the 32-bit data register d0 unchanged, then falls through into the following routine.
- `store_byte_via_pointers`: Reads a byte from the memory location pointed to by a0 and stores it at the address in a1.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.

### Targhan (1989)(Silmarils)[a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `decrement_and_call_routine`: It checks bit 5 of a global variable and, when set and the value exceeds 48, decrements that variable, then calls a routine from a table and stores its result into a global variable.
