# Game 834 (walker (playable preview) (1992)(psygnosis)[h scx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00834). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 203 of 203 named (0 author symbols), 203 of 203 described (17 by rule, 183 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 607 of 607 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 16892 bytes, 89 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5313 of 5324 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 34403 |
| code | 18710 |
| data | 449150 |
| padding | 3593 |
| unused | 395264 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Walker (playable preview) (1992)(Psygnosis)[h SCX].adf:WALKTRO | 4 | 2483 | 51 |
| startup | 1 | 882 | 62 |
| Walker (playable preview) (1992)(Psygnosis)[h SCX].adf:bootblock | 1 | 10 | 1 |
| code (raw code file, load address unknown) | 1 | 1949 | 89 |

## Cases handled

- walker (playable preview) (1992)(psygnosis)[h scx]/Walker (playable preview) (1992)(Psygnosis)[h SCX].adf:game/logodd.ppc: high-entropy data file (7.98 bits/byte), classified as packed data
- walker (playable preview) (1992)(psygnosis)[h scx]/Walker (playable preview) (1992)(Psygnosis)[h SCX].adf:game/nwalkedd.ppc: high-entropy data file (7.99 bits/byte), classified as packed data
- walker (playable preview) (1992)(psygnosis)[h scx]/Walker (playable preview) (1992)(Psygnosis)[h SCX].adf:game/nwalkedd.ppd: high-entropy data file (7.99 bits/byte), classified as packed data
- walker (playable preview) (1992)(psygnosis)[h scx]/Walker (playable preview) (1992)(Psygnosis)[h SCX].adf:game/gmanagdd.ppp: high-entropy data file (7.89 bits/byte), classified as packed data
- walker (playable preview) (1992)(psygnosis)[h scx]/Walker (playable preview) (1992)(Psygnosis)[h SCX].adf:game/nwalkedd.ppp: high-entropy data file (7.96 bits/byte), classified as packed data
- walker (playable preview) (1992)(psygnosis)[h scx]/Walker (playable preview) (1992)(Psygnosis)[h SCX].adf:game/nwlevdd.ppc: high-entropy data file (7.98 bits/byte), classified as packed data
- walker (playable preview) (1992)(psygnosis)[h scx]/Walker (playable preview) (1992)(Psygnosis)[h SCX].adf:game/nwlevdd.ppp: high-entropy data file (7.96 bits/byte), classified as packed data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- walker (playable preview) (1992)(psygnosis)[h scx]/Walker (playable preview) (1992)(Psygnosis)[h SCX].adf:s/code: raw code file no run loaded verbatim: recovered position-independently (load address unknown)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Walker (playable preview) (1992)(Psygnosis)[h SCX].adf:WALKTRO

- `initialize_hardware_at_entry`: Waits for the blitter to become idle, then writes the copper, DMA and interrupt control registers and stores the current DMA status into a global variable.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into (a6).
- `service_fire_button_interrupt`: Saves the processor status register and all data and address registers, then checks three global counters in turn, calling a helper for each one that is zero.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `advance_ring_indices_and_accumulate`: Advances two ring-buffer offsets by fixed amounts and masks them, then stores two table values shifted right by nine bits into a structure.
- `tiny_subi`: Small routine of 1 instruction that subtracts 0 from d0.
- `empty_noop_routine`: This routine is empty, containing only a single instruction that does nothing and returns immediately.
- `reset_tables_with_stride`: Fills a 31-entry table with a stride of 680, makes helper calls to initialise nearby tables, then clears a 98-entry word table by writing zero and calling a helper for each entry before returning.
- `rotate_three_buffer_pointers`: Rotates three stored buffer pointers, updates a pointer pair, then waits while a status bit in one structure is set.
- `advance_position_and_fill_pointers`: When a counter reaches seven it sets a flag, otherwise it updates a stored position and rate.
- `write_zero_byte`: It writes a byte value of zero through a pointer in a0, then returns.
- `test_bit_in_d0`: Tests bit 2 of the longword pointed to by d0 and leaves the result in d2, then returns.
- `tiny_ori_3`: Small routine of 1 instruction that merges 128 into (a4).
- `refresh_entries_without_status`: Refreshes the same eight entries from the lookup table without setting the status bit.
- `refresh_entries_single_lookup`: Refreshes eight entries from the lookup table, storing address halves and copying three lookup words into each entry's trailing fields.
- `wait_then_program_blitter`: Waits until the blitter-busy bit in the DMA status register is clear, then writes control and mask words and two address fields into the chip register block.
- `store_table_address_values`: Loads the address of a table from a global variable, stores a byte value of 255 into a 16-bit field at offset 68 of the structure in a6, stores the table address into two consecutive 32-bit fields at offsets 80 and 84, stores the value an address into a 32-bit field at offset 100, and stores the value 38421 into a 16-bit field at offset 88.
- `store_registers_to_table_unrolled`: Saves the eight data and six address registers onto the stack, then stores the same sixteen register values into sixteen consecutive longwords of a table at descending offsets, repeating the store for a long run of values.
- `transform_triplets_to_points`: Advances three ring-buffer indices, reads table entries at those indices and builds several fixed-point coefficients from pairs of them.
- `compute_face_shade_entries`: Waits until the blitter-busy bit is clear, then sets fields in the chip register block.
- `store_clamped_face_values`: Waits until the blitter-busy bit clears, sets the same chip register fields as its neighbouring routine, then walks the record list.
- `tiny_ori_4`: Small routine of 1 instruction that merges 31 into d0.
- `increment_register_d0`: Adds the decimal constant 31 to the 8-bit value in register d0, then returns.
- `clip_line_segment`: Clips a line segment to a rectangular region by computing intersection coordinates with linear interpolation, then orders the endpoints and calls the unclipped line setup routine with the adjusted coordinates.
- `setup_unclipped_line_draw`: Orders two endpoints without clipping, computes the octant and step terms of the line, and writes them to blitter-style chip registers.
- `noop_register_d0`: Performs a bitwise OR of register d0 with the decimal constant 0, leaving its value unchanged.
- `store_helper_words_in_loop`: Loops a count from a data register, calling a helper on each pass and storing its word result at the address held in an address register, advancing that register each iteration.
- `copy_word_then_fill_loop`: Copies one word from the source address register to the destination, then loops a count, calling a helper on each pass and storing its word result at the destination.
- `fill_word_pairs_from_helper`: Loops a count, calling a helper on each pass and writing its word result into two consecutive words at the address register.
- `process_packed_nibble_triples`: Reads a packed 12-bit value from the table pointed to by a0, splits it into three 4-bit fields, calls a helper to step each field one move toward its target, then stores the resulting word into two consecutive words at the address pointed to by a1 before looping until the counter in d7 is exhausted.
- `step_three_nibbles_toward_target`: Splits the 12-bit value in d0 into three four-bit fields and moves each field one step toward its corresponding target value held in registers d3, d4 and d5.
- `init_table_and_setup_screen`: Initializes a table by writing alternating a fixed address and a fixed address words, then repeatedly swaps three global pointers while filling a bitmap table by OR-ing a lookup byte into a computed position, and finally programs several screen-related registers.
- `advance_offset_and_reload_stream`: Advances a stored offset by 368 and, when it passes its limit, resets it and reads a fresh byte stream.
- `store_byte_via_pointer`: Writes the low byte of the 32-bit value in d0 to the memory location pointed to by a0, then returns.
- `recompute_scaled_pair_differences`: Stores the shifted-right-by-four differences of paired table values into a destination table, taking the loop count from register d7.
- `convert_text_and_encode_bitmap`: Reads a character from a pointer that advances as it goes, converts lowercase letters to uppercase, and stores a 60-byte block into a 46-byte-per-row table.
- `clear_byte_at_offset_1792`: Stores the constant zero into the byte at offset 1792 of a global variable, then returns.
- `load_next_text_glyph_columns`: Every sixteenth call it reads the next byte from a text buffer, converts letters above 96 to uppercase, and copies 14 two-byte entries from a character table into three separate buffers.
- `index_table_and_fill_array`: Scans a byte table from a base pointer, advancing until it finds a value greater than the running byte, then indexes a second table by that byte to obtain a pointer.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `increment_and_dispatch_action`: Saves several registers on the stack, increments a global byte at offset 4351 and compares it against the byte at offset 4350, returning early otherwise.
- `init_audio_channels_and_decrement_counter`: Initializes the four audio channels by loading their volume and duty-cycle registers, then reads a global counter and decrements it by a value from a table indexed by a game variable, clamping the result between 113 and 952.
- `process_scripted_action`: Loads a scripted action from a table into a structure in a6 based on a nibble index, computing derived position and direction fields, then branches on the action type to either update a second structure, search a lookup table, or drive a timer that eventually writes to the audio and DMA control registers.
- `dispatch_nibble_action_2`: Reads a byte at offset 2 of the structure in a6 and branches on its value, then reads the high nibble of the byte at offset 34 and either stores a clamped value from a table into offset 6 of the structure in a5, adjusts a 16-bit counter at offset 16 by a table-derived amount with clamping, or decrements a counter while advancing through a table.
- `dispatch_nibble_action_3`: Copies a word from the structure in a6 to a field in the structure in a5 and returns, otherwise branches on a divided byte to select a nibble and calls a helper that searches a table and stores a computed value.
- `clamp_and_store_table_value`: Looks up a value in a table indexed by a structure byte, clamps it toward another structure field, stores the result, and records a direction flag depending on the comparison.
- `clamp_value_and_index_table`: Clamps a 16-bit value at offset 16 of the structure in a6 toward a byte at offset 23 by adding or subtracting it, then caps the result at the value at offset 24.
- `adjust_value_from_table_and_store`: Reads a nibble from the byte at offset 26 and a two-bit value from offset 30 of the structure in a6, indexes a table to build a signed magnitude value, multiplies it by a nibble from offset 26, shifts it, and stores the adjusted 16-bit result at offset 6 of the structure in a5.
- `dispatch_nibble_action`: Reads a nibble from the byte at offset 2 of the structure in a6 and branches on its value, then reads the high nibble of the byte at offset 3 to dispatch further.
- `store_nibble_and_manage_counter`: Stores the low nibble of the byte at offset 3 of the structure in a6 into offset 18 and returns.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.

### startup

- `run_startup_checks`: Calls a routine and returns its result when the value equals 31, otherwise calls a second routine and returns, then calls a third routine and returns when that result is nonzero.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `contains_zeroed_data`: Contains a single instruction that ORs a register with zero, and is almost certainly a zero-filled data area referenced by address rather than executed.
- `load_custom_base`: Loads the base address of the Amiga's custom chip registers into a0, places the value 0 into d0, and returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `clear_state_and_run_setup`: Clears 91 bytes starting at offset 40 of the structure in a5, then runs a chain of helper routines that stop at the first nonzero result.
- `open_workbench_when_flagged`: Calls three helper routines in sequence, then calls the intuition library's OpenWorkBench through its vector when a flag bit at offset 36 of the structure in a5 is set.
- `probe_dos_file_handle`: Opens a file through the DOS library and, when the handle is valid, passes it to a second DOS call before closing it.
- `compare_hunk_type_codes`: Loads a 32-bit value from the address pointed to by a0, compares it against the hunk type codes 1001, 1004 and 1010, and branches away when one matches.
- `dispatch_hunk_type_codes`: Calls a helper routine, then compares a 32-bit word loaded from the area pointed to by a0 against the hunk type codes 1001, 1004 and 1010, branching to the matching handler for each, and otherwise falling through to a default path.
- `return_empty_gap_routine`: Contains only a return with no operations.
- `copy_words_in_loop`: Calls a helper routine, then copies 32-bit words from the area pointed to by a0 into the area pointed to by a1, decrementing a counter until it reaches zero, and finally jumps to another routine.
- `scale_and_jump_via_pointer`: Loads a 32-bit value from the area pointed to by a0 into d7, advances a0 by eight bytes, scales d7 by four, adds it to a0, and jumps to another routine.
- `accumulate_through_pointers`: Reads a marker value and, when it matches a specific constant, adds a base register to a pointed-to value and stores the sum back through the same pointer.
- `restore_registers_and_return_zero`: Loads zero into a register, restores the caller's registers from the stack, and returns the restored value.
- `restore_registers_and_return_34`: Loads the value 34 into a register, restores the caller's registers from the stack, and returns the restored value.
- `run_three_checks_then_cleanup`: Calls a helper and, when it returns zero, calls a second helper and then a third.
- `own_blitter_and_forbid`: Calls the graphics library's OwnBlitter through a vector in the caller's structure, then calls exec's Forbid through the exec base held in a global slot, and returns the register values unchanged.
- `permit_and_disown_blitter`: Calls exec's Permit through the exec base held in a global slot, then calls the graphics library's DisownBlitter through a vector in the caller's structure, and returns the register values unchanged.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `setup_stack_and_exec_vector`: Calls two routines by absolute address, restores a1 from the stack, sets a6 to the copper register base, moves the stack pointer to a fixed address, loads the exec library base, stores a vector into it, and raises multitasking with a trap.
- `init_stack_and_clear_struct`: Sets the stack pointer to a fixed address, clears the CIA interrupt registers and disables interrupts, zeroes several fields of the structure in a6, clears a 32-bit global variable, then zeroes thirty-two consecutive 16-bit words starting at offset 384 of that structure before jumping through the address in a1.
- `call_intuition_display_beep`: Pushes the working registers, loads the intuition library base from a vector in the structure held in a5, calls the DisplayBeep routine at offset -96 of that base, restores the registers, and returns the register values unchanged.
- `save_registers_and_loop`: Saves registers d1 through d7 and a0 through a6 on the stack, loads a0 from d2, clears d3, tests the byte at a0 and returns when it is zero, otherwise increments d3 and loops back to reload a0.
- `test_flag_and_jump`: Tests bit 15 of the word in d3 and branches away when it is set.
- `call_library_and_test_result`: Loads a pointer from offset 48 of the structure in a5, pushes the structure base, and calls a library routine through that pointer.
- `call_library_and_return_result`: Loads a value from offset 44 of the structure in a5 and a pointer from offset 48, calls a library routine through that pointer, and returns either the call result or the constant 24 when the result is negative.
- `restore_registers_and_return`: Restores the caller's registers d1-d7 and a0-a6 from the stack and returns, passing the caller's return value back to its caller.
- `store_request_and_autorequest`: Saves the caller's registers, stores the library base from a5 and the caller's return address into global slots, sets up arguments, calls the intuition library's AutoRequest through a vector in that structure, restores the registers, and returns the saved register value.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d1, then merges 13 into d6, then merges 0 into d0.
- `call_intuition_auto_request`: Saves the registers, copies the two incoming arguments into d2 and d3, zeroes d0 and d1, loads the intuition library base from a5, calls the AutoRequest routine at offset -348, copies the result into d1, restores the registers, and returns that result.
- `conditionally_call_via_structure`: Tests the value in d0 and, when it is nonzero, loads the routine at offset -90 of the intuition library base in a5 and calls it, otherwise it returns.
- `validate_size_arguments`: Returns the value 21 when the second argument is zero or negative, returns zero when the first argument is below 42, and returns 22 otherwise.
- `data_area_zero_fill`: Contains nine consecutive ori.b #0, d0 instructions, which are zero words most likely forming a data area referenced by address rather than executed code.
- `zero_ten_bytes`: Fills ten consecutive bytes of a data area with zero values, writing one byte at a time through a pointer.
- `poll_task_message_slot`: Saves a5, loads the task list base from a global variable, and calls FindTask; when the returned task has a zero field at offset 172 it calls FindTask again and then GetMsg, storing the resulting message pointer at offset 72 of the caller's structure, and always returns zero.
- `open_three_libraries`: Opens three libraries through exec's OpenLibrary, storing each base at offsets 40, 44 and 48 of the caller's structure and returning error codes 1, 2 or 3 for the first that fails; when the flag at offset 72 is clear it also calls the DOS Output function, storing the handle at offset 68 and returning 4 when it is null, otherwise zero.
- `close_stored_libraries`: Closes each of up to three stored library bases through exec's CloseLibrary when its pointer is nonzero, clearing each stored pointer after closing.
- `open_misc_resource_and_setup`: Calls a helper routine, stores its result at offset 60 of the structure in a5, and returns error 4 when that result is zero.
- `allocate_misc_resources`: Claims the first misc resource twice through its claim entry, recording each claim in a flag byte at offset 121 and offset 120.
- `claim_misc_resource_pair`: Claims two misc resources through the first entry of the misc resource vector, recording each claim in a flag byte.
- `allocate_potgo_resource`: Opens potgo.resource through exec's OpenResource and returns error 7 when it is missing.
- `call_three_helper_routines`: Calls three helper routines in sequence and then returns.
- `free_misc_resources`: For each misc resource whose claim flag byte is set, calls the resource's release entry at offset minus 12 and clears the flag.
- `free_misc_resource_pair`: Releases two misc resources through the release entry at offset minus 12 for each claim flag byte that is set, clearing each flag as it goes.
- `free_potgo_bits`: When the potgo claim value is nonzero, clears it and calls the potgo resource's release entry at offset minus 12.
- `remove_named_device_if_present`: Disables multitasking, looks up a named node with exec's FindName, removes it with RemDevice when it is found, and then re-enables multitasking.
- `install_interrupt_servers`: Copies three stored values from the caller's structure and removes an existing interrupt server through exec when one is set.
- `remove_interrupt_servers_and_release`: Removes an interrupt server through exec's RemIntServer when its flag is set, then calls exec's entry at offset minus 12 for two further flagged blocks.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `clears_byte_at_pointer`: Writes the value zero into the byte at the address in a0, effectively clearing that byte in a data area.
- `allocate_memory_then_continue`: Allocates a memory block through exec's AllocMem when the size field at offset 20 of the structure in a5 is nonzero, storing the returned pointer at offset 108.
- `allocate_blocks_with_checks`: Calls a helper to allocate memory for a block whose size field at offset 4 is nonzero, storing the returned pointer at offset 112, then does the same for the block whose size field at offset 16 is nonzero, storing its pointer at offset 116.
- `find_region_with_free_size`: Walks a linked list of memory blocks from a global root and returns the free size of the first block whose attribute flags match and that is large enough for the requested size.
- `allocate_memory_blocks`: Allocates four memory blocks through exec's AllocMem using distinct flags and stores each returned pointer at offsets 76, 80, 84 and 88 of the structure in a5.
- `allocate_memory_from_pointer`: Reads a pointer at offset 8 of the structure in a5 and allocates a memory block through exec's AllocMem when it is nonzero, storing the returned pointer at offset 96.
- `alloc_mem_from_base`: Reads a pointer at offset 16 of a base structure and returns immediately when it is zero, otherwise pushes the base, loads an exec library base, and calls the memory allocation routine.
- `call_alloc_mem`: Reads a pointer at offset 16 of the exec base in a5 and, when it is non-zero, pushes a5 on the stack, loads the exec library base into a6 and calls the AllocMem routine at offset -198 of that library.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `free_five_memory_blocks`: Frees up to five stored memory blocks through exec's FreeMem, clearing each stored pointer after it is released.

### Walker (playable preview) (1992)(Psygnosis)[h SCX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### code (raw code file, load address unknown)

- `jump_to_boot_entry`: Computes the address of the next routine and stores it at offset 132 of a global structure, then jumps to it through trap number 1.
- `setup_and_jump_boot`: Disables interrupts and DMA channels, clears the interrupt request register, and stores two memory region pointers into a global table based on comparisons against fixed addresses.
- `unpack_game_blocks`: Initializes the custom chip registers and clears memory, then repeatedly loads a code pointer and a data pointer from a table and unpacks a compressed block, advancing the pointer by the returned size until each block is handled.
- `init_custom_and_clear_memory`: Writes several control values to the custom chip registers, stores a code pointer and a data pointer into the structure in a6, then clears a memory block and fills two tables with computed values before calling two helper routines.
- `clear_bitplane_rows`: Reads a count from a table, divides it by six and, for each full group of six pointers, clears a byte at three fixed offsets that look like bitplane spacing.
- `setup_video_and_indirect_jump`: Sets up pointers to several video memory areas and loads a jump-table length from a global table.
- `write_plane_pixel_and_advance`: ORs a mask into a plane byte and ANDs a second mask into the same position on two other planes spaced at fixed offsets, then stores the pointer and either records a column counter or dispatches through a table.
- `update_wave_column_heights`: Toggles a parity flag, selects one of two 64-entry tables, and for each entry computes a height from a table value multiplied by a delta and offset.
- `update_scroll_velocity_with_phase`: Advances a phase counter by two within a zero to two hundred fifty-five range and either adds a signed step to a velocity value clamped between a negative limit and a positive limit, or adds an accumulating step to the velocity value.
- `store_pointer_in_global`: Stores the pointer in a1 into a global variable at offset ten and returns.
- `incrementally_align_table_entries`: Loads a pointer to a source table and uses the byte at offset 16 of the structure at offset 4056 to select either that table or the same table advanced by 192 words, paired with the buffer at an address or the one advanced by 192 words.
- `set_flags_and_wait_counter`: Sets a byte at offset 17 of the structure in a0 to 255 and a 16-bit value at offset 14 to 1, then calls another routine.
- `check_interrupt_requests`: Reads the interrupt request and enable registers, and when bit 5 is set it calls three subroutines and stores the value 32 in a global variable.
- `probe_disk_units`: Writes a digit character into a name template at offset 8 for each of a small number of attempts, calling a helper that probes a device and returns success in d0.
- `read_joystick_axes_and_button`: Reads a 16-bit word from a structure in a6 and checks bit 10 of it.
- `shift_feedback_register_32_steps`: Initializes a 32-bit global register and runs a loop of 32 steps.
- `decode_message_header`: Saves the registers and copies the incoming pointers into a4, a5 and a6, then calls a helper to obtain a value in d6.
- `copy_segmented_data_blocks`: Reads a segment count from offset 324 of the structure in a0 and, for each segment, calls a helper that returns nonzero when the segment is invalid, otherwise copies that many words followed by bytes from the source table at offset 24 into the destination buffer in a5.
- `decrement_counter_and_call`: Checks a counter at offset 8 of the structure in a6 and a flag at offset 504.
- `fill_index_table_with_sequences`: Fills a 128-entry table with a repeating sequence of 12 index values, copying a variable-length byte string into each entry and advancing a running counter, then returns.
- `store_indexed_value`: Loads two values from a program-relative table, calls a helper routine, and branches away when it returns nonzero.
- `sum_path_with_helpers`: Runs two helper routines and branches to a shared exit when either reports a nonzero result.
- `set_flag_in_bitfield`: Combines the high and low halves of a 32-bit value in d0 into a single 64-bit index, divides it by 32 to compute a table offset, selects the 32-bit word at that offset from a table pointed to by a6, and sets one bit within it corresponding to the low 5 bits of the index.
- `decrement_and_loop_until_match`: Loads a short from a register, decrements it, and repeatedly calls another routine with the updated value until the value equals a specific constant, then returns.
- `wait_for_specific_value`: Calls a helper routine once, then enters a loop that repeatedly calls another helper while a short register does not equal 1759, incrementing two counters each iteration.
- `combine_registers_into_return_value`: Receives two 32-bit values in d0 and d1 and returns them concatenated as a 64-bit value, with the first value as the high part and the second as the low part, without performing any other operations.
- `clear_bit_from_table`: Combines the high and low halves of a 32-bit value in d0 into a 64-bit index, divides it by 32 to compute a table offset, selects the 32-bit word at that offset from a table pointed to by a6, and clears one bit within it corresponding to the low 5 bits of the index.
- `sum_array_via_helpers`: Loads a pointer from a global table at offset 452 of the structure in a6 and calls a helper to sum an array of words.
- `copy_name_bytes`: Copies bytes from a source pointer to a destination buffer at an offset from a base pointer, advancing both pointers each iteration until a null byte or after 30 copies.
- `copy_bytes_from_table`: Iterates over a table of pointers at offset 24 of the structure in a6, stepping by 4 until the counter reaches 288.
- `clear_zero_table`: Clears a block of memory by writing zeros to consecutive 32-bit words.
- `initialize_disk_transfer_block`: Initializes a memory block by writing the constants 2, 72, -1 and 881 to offsets 0, 12, 312 and 316 of the structure in a0, then stores 1 at offset 508.
- `process_path_segments`: Saves a4 on the stack, loads a0 from a6, and calls a helper that returns early when it is nonzero, then calls a second helper and returns early when it is zero, otherwise indexes a table and calls a third helper.
- `parse_drive_path`: Reads characters from another routine to detect a drive path pattern, checking for the letters D and F followed by a digit and a colon.
- `tiny_move`: Small routine of 1 instruction that copies (a0)+ to d0.
- `normalize_lowercase_letter`: Normalizes the byte in d0 to uppercase if it is a lowercase letter, otherwise it returns.
- `copy_path_segment_to_buffer`: Scans a null-terminated string from a base pointer, advancing until it reaches a null terminator or a slash character.
- `check_extraout_flag`: Calls a helper routine and, when it returns zero, tests a flag and two fields of the pointer passed in a0.
- `return_array_sum_or_default`: Calls a helper to sum the words of an array and returns its result when the helper sets the zero flag, otherwise returns the constant 405.
- `sum_array_or_return_default`: Calls a helper to sum the words of an array and returns its result when the helper sets the zero flag, otherwise returns the constant 405.
- `sum_negate_array_words`: Adds 128 consecutive 16-bit words pointed to by a pointer in a0, accumulating the sum in a register.
- `store_zero_byte`: It writes a byte of value zero through the pointer in a0, then returns.
- `no_op_return`: This routine is a lone instruction that does nothing to the machine state and returns immediately.
- `store_zero_byte_2`: It writes a byte value of zero through the pointer in a0, then returns.
- `pad_zero_data_area`: Performs eight sequential no-op byte operations on register d0, which is used as a zero-filled data area referenced by address from other routines.
- `sum_and_store_offset`: Clears the 32-bit value at offset 20 of the structure pointed to by a0, calls a routine that accumulates the sum of 128 consecutive 16-bit words starting at that pointer, then stores the resulting sum back to offset 20 of the same structure.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d2.
- `save_and_call_helper`: Pushes the value in d3 onto the stack, loads the constant 1 into d3, and calls another routine, then restores the original d3 value from the stack and returns.
- `set_register_to_one`: Loads the value one into the 32-bit register d2 and then returns.
- `tiny_moveq_2`: Small routine of 1 instruction that loads 0 into d3.
- `compute_packed_table_index`: Splits a 16-bit input into quotient and remainder using division by 11, combines them into a packed value, and calls a setup routine, then loops up to ten times decrementing a counter and adjusting a pointer and accumulator based on a joystick/mouse status byte and a structure flag, and finally recomputes a combined quotient/remainder result before returning.
- `update_walker_state_loop`: Clears three 16-bit fields in the structure in a6, then loops ten times checking a joystick/mouse status byte and a structure flag, writing the magic values a fixed address and 17545 to a table entry and calling helper routines to update state and audio DMA control.
- `wait_cia_timer_flag`: Sets a flag in the structure in a6 and, when a byte field equals 2, calls a setup routine.
- `render_park_view`: Writes a constant to a memory location derived from a pointer, then repeatedly calls other routines to update a flag, toggle a flag, and compute a checksum using a decrementing counter.
- `loop_check_flag_decrement`: Loops ten times calling a flag-checking helper and a second helper, returning the helper's value early when the flag is clear and returning 24 when the counter expires.
- `check_flags_and_update_state`: Loads a base pointer from a6, scales a structure field by 1088, and calls a helper that returns early when a flag is set, then compares fields and calls helpers that set flags and mix words in a loop that increments a counter.
- `multiply_offset_by_512`: Reads a 16-bit value from the structure in a6 at offset -16, multiplies it by 512, and stores the 32-bit result into the register pair d1/d0.
- `set_flag_and_increment_counter`: It sets a bit in a 16-bit word located four bytes before the pointer in a6, using a bit index derived from d3.
- `clear_table_entries`: It clears a table by writing zero to successive entries.
- `reassemble_low_byte_pair`: Calls a routine to read two consecutive 32-bit values from the pointer in a0 and reassembles the result so the low 16 bits come from the returned value and the high 8 bits come from the original high byte, then stores the four extracted bytes into registers d0 through d3.
- `interleave_word_bits`: It reads two consecutive 32-bit values from the pointer in a0 and returns a value formed from their low bits.
- `compare_interleave_with_constant`: Loads the address of a global variable at offset 8 from the base pointer in a5 and the constant 40 into a register, then calls a routine to interleave word bits and compares the produced value with the saved constant, returning the result.
- `load_pointer_and_constant`: Loads the address of a global variable located at offset 8 from the base pointer in a5 into a0 and loads the decimal constant 40 into d1.
- `compute_xor_checksum`: Iterates over a number of words pointed to by a pointer, XORing each into an accumulator, then returns the result masked to retain alternating bits.
- `fill_memory_pattern`: Fills a block of memory with a repeated 32-bit pattern, choosing the block end from a flag in a structure and writing the pattern backwards from that end.
- `interleave_word_bits_2`: Processes 128 pairs of words from two input pointers, masking and shifting each pair to interleave their bits into a single word written to a third pointer.
- `loop_clear_and_mix_words`: Computes a loop count from the first argument and iterates that many times, calling routines to clear a flag bit and to mix and store a word on each pass, then calls those same routines once more and returns the two original arguments.
- `shift_and_call_twice`: Pushes a register onto the stack, shifts a word right, then calls another routine twice, restoring the register and returning.
- `clear_flag_byte`: Reads a byte and a flag from the preceding byte, then sets or clears a bit in the target byte depending on the flag before writing the modified byte back.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `set_audio_dma_control`: Writes several constant values to custom chip registers and a global variable, storing an incoming pointer in a register and a shifted word with its high bit set.
- `write_ciab_timer_registers`: Reads bit 1 of the CIAB register at offset 31 and, when it is clear, writes the value 2 to the register at offset 156 and the value 16384 to the register at offset 58, then returns.
- `write_audio_register_check_byte`: Writes the constant 1024 to the audio hardware register, then tests a byte in a structure and, if that byte is negative, loads the constant 255 into a register; it contains no loop and calls nothing.
- `set_bit_in_global_flag`: Reads a 16-bit value from a structure, adds a constant, and sets the corresponding bit in a hardware register by OR-ing a derived mask into it.
- `check_ciab_bit_decrement_2`: Tests the least-significant bit of a CIAB register and, if it is set, returns; otherwise it decrements a register and returns when that register reaches zero.
- `update_table_entry`: Saves d2 and d3, loads a0 from a table indexed by a structure field, and calls a helper that returns early when the value is negative, then computes a difference and calls a helper that decrements a counter before writing the result back.
- `wait_for_input_flag_clear`: Polls a global variable flag in a loop, decrementing a counter until the flag is cleared or the counter expires.
- `read_and_mask_ciab_port_b`: Reads a byte from the CIAB port B register, clears bit 1 when a global variable bit is set, then clears bit 0 and sets bit 0 again on the same register while looping until a timer flag routine reports a value other than one.
- `store_masked_input_byte`: Calls a routine to mask an input byte and stores the resulting byte into the joystick port register, then returns.
- `mask_input_byte`: Builds a mask from a global variable and a structure byte, clears one bit based on the structure, optionally clears another bit from a table flag, then returns a combined value.
- `wait_for_timer_and_flag`: Calls a routine to set a CIAB timer and, in a loop, keeps calling it while a global variable bit is set and the returned value is not one, spinning on the CIAB_CRA register between attempts, then returns zero.
- `check_ciab_bit_decrement`: Tests the least-significant bit of a CIAB register and, if it is set, returns; otherwise it decrements a register and returns when that register reaches zero, containing no loop and calling nothing.
- `set_ciab_timer`: Writes three constant values in sequence to the CIAB_CRA, CIAB_TAHI, and CIAB_TALO hardware registers, then returns.
- `unpack_rnc_compressed_block`: Verifies a four-byte header against the signature RNC followed by version byte one, and when it matches reads the packed and unpacked sizes, decodes the stream through helper calls for each bit-level step, and writes the output in 32-byte groups while updating a checksum word; otherwise it returns zero.
- `extract_flag_bits_from_table`: Scans a table of word pairs until a global mask no longer matches the following word, then reads a byte from the entry and uses it as an index into the mask to extract a bit, calling another routine if a borrow is detected, and repeats the process for a second byte.
- `mask_and_conditional_call`: Masks a word in a register with a word from another register and, if a byte in one register is less than a byte in another, calls a routine and uses its returned word as the result, otherwise returning the masked word.
- `return_empty_routine`: Empty; it contains no instructions that modify memory or registers and immediately returns.
- `accumulate_32bit_from_bytes`: Reads four consecutive bytes from the pointer in a0 and assembles them into a 32-bit value in d0, with the first byte placed in the most significant byte and the last byte in the least significant byte.
- `decode_and_fill_table`: Fills a stack buffer with a computed number of bytes from a helper, then iterates over that buffer and a table, writing bit masks and derived values into table entries when the bytes match.
