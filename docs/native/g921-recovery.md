# Game 921 (AbandonedPlaces2 v2.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00921). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (7486916 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 509 of 509 named (0 author symbols), 509 of 509 described (202 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2701 of 2701 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 3280390 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3920 of 3927 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12574 |
| code | 13580 |
| data | 7460749 |
| padding | 13 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 1005 | 72 |
| AP2 | 3 | 2245 | 182 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 268 | 211 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 395 | 35 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 6 |
| ap2.000 (raw code file, load address unknown) | 1 | 0 | 0 |
| ap2.001 (raw code file, load address unknown) | 1 | 0 | 0 |
| ap2.002 (raw code file, load address unknown) | 1 | 0 | 0 |
| ap2.003 (raw code file, load address unknown) | 1 | 0 | 0 |
| ap2.dir (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- AbandonedPlaces2_v2.0/AbandonedPlaces2_v2.0.hdf: hard-disk image (AmigaDOS OFS, 8032 blocks): its filesystem parses and all 14 files in it are byte-identical to the extracted tree AbandonedPlaces2_v2.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- 18 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 202 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 316 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- AbandonedPlaces2_v2.0/AbandonedPlaces2_v2.0_hdf/AbandonedPlaces2/data/ap2.000: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AbandonedPlaces2_v2.0/AbandonedPlaces2_v2.0_hdf/AbandonedPlaces2/data/ap2.001: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AbandonedPlaces2_v2.0/AbandonedPlaces2_v2.0_hdf/AbandonedPlaces2/data/ap2.002: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AbandonedPlaces2_v2.0/AbandonedPlaces2_v2.0_hdf/AbandonedPlaces2/data/ap2.003: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AbandonedPlaces2_v2.0/AbandonedPlaces2_v2.0_hdf/AbandonedPlaces2/data/ap2.dir: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 5 never-loaded region(s), 3280390 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `warm_reset_restore_and_rte` (AP2): Restores a7 from saved value, installs a new trap handler, masks interrupts, reinitializes hardware via helper calls, re-enables DMA and interrupts from saved globals, sets copper pointers and returns from exception with rte. Kind: checksum trap / integrity check. Pass path: The routine appears to be a state-restoration wrapper used during a sensitive execution phase. The pseudo-code shows a pattern of saving a value (likely a stack pointer or a specific register state `iVar1`) and restoring it after several su.
- `vblank_handler_swap_audio_state` (AP2): Restores the saved stack pointer, saves registers, reads ADKCONR, DMACONR and INTENAR, calls three setup routines, sets the flag byte, then writes masked interrupt and DMA enable values to the custom chip registers and returns with rts. Kind: disk hardware/sync check. Pass path: the routine continues execution normally after the sub-calls, implying the hardware/sync checks passed.
- `assert_ciab_disk_select_lines` (AP2): Writes 255, then 135, then 255 to CIAB port B at the disk control register, with two nops between each write, then returns. Kind: disk hardware/interface validation. Pass path: the instructions immediately following the call (the routine likely sets a status flag or error code in d0/d1/a5).

## Functions


### game.slave

- `return_minus_one_at_entry`: Loads the immediate value minus one into d0 and returns immediately, providing a constant return value to the program entry caller.
- `store_base_and_init_library_calls`: Saves the incoming a0 base pointer to a global, sets a5 to it, calls library offsets 96 and 52 with constants, stores a callback pointer, then jumps through the base pointer.
- `set_two_offsets_and_jump_base`: Loads the global base pointer, adds an address to form a4, stores a4 at offsets 6 and 18 of a5, adds an address, calls a routine, then jumps to a5.
- `set_a3_to_custom_chip_base_and_jump`: Loads the custom chip register base address into address register a3, then jumps to the address held in a5.
- `read_and_mask_status_word_in_d0`: Reads a longword from program-counter-relative memory into d0 and clears bits 4 through 6 and the high byte, returning the masked value.
- `vblank_interrupt_ack_and_rte`: Tests the interrupt request register at the custom chip base, restores d0-d1, a0-a1, a5-a6 from the stack and returns from the interrupt with rte.
- `acknowledge_interrupt_from_stack`: Reads a word from the top of the stack, writes it to the INTREQ hardware register to acknowledge an interrupt, advances the stack pointer by two and returns.
- `push_return_and_set_a3_vector`: Grows the stack by eight bytes, copies the return address and a3 into the new frame, loads a3 with a nearby code address, sets d0 to zero and d1 to a2, then returns.
- `pop_d1_push_vector_and_set_a1_flag`: Pops d1 from the stack, pushes a nearby code address, pushes d1 back, sets bit one of the byte at offset fourteen in a1 and returns.
- `set_custom_chip_base_and_jump`: Loads the custom chip register base address into a0 and branches to another routine, passing that base pointer along.
- `clone_node_list_with_allocated_buffers`: Allocates a block sized from the source node count, copies the count and each entry, allocating memory for nonzero sizes, and frees partial allocations on failure.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `find_node_by_name_string`: Walks a linked list from the node in a0, comparing each node's name string at offset ten against the string in a1, returning the matching node or zero.
- `store_d0_to_pc_relative_buffer`: Loads the address of a nearby buffer into a6, writes the longword in d0 there, then loads the CIAB base address into a6 and returns.
- `copy_buffer_to_cop2lch_and_jump`: Reads the longword stored in the pc-relative buffer and writes it to the COP2LCH hardware register, then jumps to the address held in a0.
- `copy_byte_from_a0_offset_to_a6_800`: Reads the byte at offset six from the address in a0 into d0, stores that byte at offset two thousand forty-eight from a6, and returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_loaded_segment_id`: Loads the longword at the program counter relative offset and returns 4 in d0 if it equals an address, otherwise returns 1.
- `init_two_sentinel_lists_and_clear_flag`: Writes the value minus two to two consecutive longwords at a3, stores the advanced a3 pointer into a4 and a2 fields, clears the word at a4 offset 386, and skips fourteen bytes of return address.
- `copy_child_list_heads_to_a3_fields`: Reads two pointers from a1 offsets 4 and 8, and if non-null copies the longword at each target offset 4 into a3 offsets 50 and 54, then skips fourteen bytes of return address.
- `notify_children_on_pointer_activation`: Checks the activation flag in the window structure against the library base, then walks the child list and calls the notify function for each child whose flags include both the active and pointer bits.
- `pop_four_bytes_and_return`: Adds eight to the stack pointer, discarding two longwords, then returns to the caller; it is a small stack-cleanup trampoline.
- `copy_word_from_a0_offset_to_a1_offset`: If both a0 and a1 are non-null, copies the word at offset twenty of a0 into offset fifty-eight of a1, then returns; otherwise returns without copying.
- `merge_joypad_bits_into_port`: Calls the low-level library routine at offset -84, combines its returned byte with d0 and d1 masks and the byte at offset 56 of a1, stores the result there and returns it ORed with the byte at offset 22 of the pointer at 52(a1).
- `return_true_if_d0_was_nonzero`: Subtracts one from d0, sets the byte to all ones if the result is nonzero, sign-extends it to word and long, and returns that value in d0.
- `compare_rotated_byte_to_table`: Inverts and rotates d0, compares it against two bytes near the program counter, and on a match loads a1 from a nearby address and stores the vertical position register byte there before returning.
- `push_ffff_then_branch_into_other_code`: Pushes the immediate value -1 onto the stack, then branches unconditionally to another routine's code; it does not return here.
- `restore_registers_and_disable_interrupts`: Tests d0, and if nonzero unwinds several stack frames restoring d2-d4, a6, a2, a0-a1 and a5, writes the saved interrupt mask to INTENA, then rearranges stack words and returns.
- `compute_position_from_clock_and_divisor`: Reads three longwords from the structure pointed to by a0, multiplies the first by 1440, adds the second, scales by 60, adds the third divided by 50, and stores the result at offset 198 of a2.
- `select_disk_menu_action`: Reads the disk index from offset 67 of a3, clears the status byte at offset 31 of a1, and depending on flag bits and the mode byte at offset 29 of a1 either sets a status code or calls a jump-table routine with the a1 fields.
- `disk_index_bit_table_data`: The bytes are a data table of bit masks indexed by the disk number, not executable code; the decoded instructions are meaningless immediates and a stray rts.
- `disk_index_mask_data`: The bytes are data, not executable code: they compute a mask from the disk index byte and store it, but the surrounding instructions are decoded table bytes.
- `push_pc_relative_address_plus_offset_a`: Pushes the longword at the program counter plus thirty-two onto the stack, adds a fixed offset to it, and returns, yielding a relocated address.
- `push_pc_relative_address_plus_offset_b`: Pushes the longword at the program counter plus thirty-two onto the stack, adds a different fixed offset to it, and returns, yielding another relocated address.
- `clear_bit_and_toggle_flag_in_object`: Reads a bit index from offset 67 of a3, clears that bit in a byte table, and if it was set toggles bit one of the flags at offset 64 and bumps the counter at offset 294.
- `set_d1_10001_and_jump`: Loads the immediate value 65537 into d1 and branches unconditionally to a shared routine at a fixed offset.
- `read_byte_at_a0_offset_d2`: Tests d2 for negative, and if non-negative loads the byte at a0 plus d2 into d3, copies d3 to d1 and returns.
- `set_d1_280_and_jump`: Loads the immediate value 280 into d1 and branches unconditionally to a shared routine at a fixed offset.
- `clear_a1_offset_c_and_store_a4_pointer`: Clears the longword at offset 12 of a1, loads 12 into d4, then stores a4 into the absolute address formed from the pc-relative lea and returns.
- `zero_filled_data_area`: A single zero word that decodes as ori.b with immediate zero to d0; the bytes are data, not executable code, referenced by address.
- `compare_a4_to_stored_pointer_and_dispatch`: Compares a4 with the longword stored at the pc-relative address, branches if equal, otherwise jumps through the pointer at offset minus 12 of a5; the equal path sets a6 and calls a routine before jumping through a6.
- `store_pc_relative_pointer_in_a3_data`: Loads a program-counter-relative address into a3, writes that same address into memory at offset minus 322 from a3, then loads another pc-relative address into a3 and copies it to d1 before returning.
- `open_whdload_device_and_init_structures`: Saves all registers, opens expansion.library and whdload.device via exec, allocates memory blocks, sets flags and pointers in the returned structures, enqueues a message, and restores registers.
- `open_dos_library_and_jump_offset`: Loads a pc-relative string into a1, calls the dos.library open function through a6, then loads a pointer from offset 22 of the result and jumps to it.
- `open_dos_library_and_wait_port`: Opens dos.library via exec, builds a message port, finds the task, waits on the port, reads the message and dispatches on its command word through a jump table.
- `poll_input_and_branch_on_result`: Calls three helper routines passing d7 and the value at offset 28 of a4, shifts the result right by two and branches to one of two exits depending on whether it is zero.
- `call_two_subs_then_return_ff`: Calls the routine at offset d42, copies d7 into d0, calls the routine at offset df8, sets d0 to minus one, then branches to offset 828.
- `copy_stream_buffer_to_output`: Advances the stream position by the requested count, copies bytes from the stream buffer into the output buffer, and falls through to the common exit path.
- `append_bytes_to_stream_buffer`: Adds the requested byte count to the stream position, grows the high-water mark, and copies bytes into the stream buffer, allocating it via the OS when needed.
- `flush_stream_tail_and_exit`: Loads the remaining count and position into d0 and d1, calls the stream write vector with the buffer and output pointer, then returns to the common exit.
- `set_d2_minus_two_and_branch`: Loads the immediate value minus two into data register d2, then branches unconditionally to the following routine entry point.
- `open_file_and_store_handle`: Calls several helper routines with d7 as argument, tests the returned longword, and on success stores the handle at offset 36 of the structure pointed to by a0, returning minus one in d0.
- `open_file_via_dos_and_chain`: Calls helpers to obtain a filename from d7, opens it through the a2 jump table and the DOS library at offset minus 210 of a6, then branches into the file-handle storing routine.
- `flush_audio_channel_and_restart`: Loads the channel structure from offset 20 of a4, checks its status at offset 256, and if active calls the stop routine at offset 56 of a2 with the sample pointer, then reloads the structure and calls the restart routine.
- `advance_sample_position_with_bounds_check`: Adds a signed delta from offset 28 of a4 to the position at offset 24, choosing the loop length or a fixed increment, compares against the limit at offset 148 of a0, and stores the new position or returns an error code.
- `scale_channel_offset_by_four`: Loads the value at offset 20 of a4 into d7 and shifts it left by two bits, returning the scaled offset in d7.
- `load_a4_offset_24_shifted_left_two`: Loads the longword at offset 24 from a4 into d7, shifts d7 left by two bits, and returns.
- `load_a4_offset_28_shifted_left_two`: Loads the longword at offset 28 from a4 into d7, shifts d7 left by two bits, and returns.
- `pack_string_with_length_prefix`: Copies bytes from a0 forward, overwriting each with the previous byte, until a zero byte, then stores the count before the start.
- `allocate_and_init_108_byte_struct`: Calls a subroutine, allocates 264 bytes via a6 library offset minus 198, fills fields at offsets 28 and 148, stores d4, d2, a5 and a3 into the block, and returns the block pointer in d0.
- `free_two_blocks_via_a6_offset`: If d0 is nonzero, frees the block at offset 260 and then the block at offset 4 using a6 library offset minus 210, then jumps through that same offset.
- `build_merged_path_alloc`: Combines the d0 path string with the d1 name, strips directory and colon parts, allocates a buffer sized for the joined path, copies the merged text and returns it in d0.
- `check_two_whdboot_strings`: Compares two pairs of strings against the WHDBoot names using the compare routine, returning zero if both match or the compare result otherwise.
- `compare_null_terminated_strings`: Saves a0, compares bytes at a0 and a1 until a mismatch or a null byte, restores a0 and returns with the comparison flags set.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `push_pc_relative_address_plus_32`: Pushes the address of the following instruction plus 32 bytes onto the stack and returns, yielding a relocated pointer value.
- `open_dos_library_and_start_game`: Opens dos.library via exec OldOpenLibrary, sets up a5 and a0/a1 pointers, calls the setup routine, pushes -1 and jumps through the saved return vector.
- `check_rts_marker_and_jump_table`: Computes an address from d7 times four plus offset, checks whether the word there equals the rts opcode, and if so loads a pointer and jumps through a2.
- `write_byte_and_call_delay`: Writes byte 65 to offset 7681 of a0, saves d0 on the stack, sets d0 to 2, calls a nearby routine, restores d0 and returns.
- `wait_two_vblank_lines`: Saves d0, polls the VHPOSR register until two consecutive reads match, then restores d0 and delays dbra times before returning.
- `bit_toggle_data_table`: Bytes are data, not executable code: the decoded instructions toggle bits in memory at a0 and load sr, so this is a small data table.
- `open_library_and_call_vector`: Saves all registers, opens a library via a6 offset, optionally calls a5, invokes the library vector with adjusted stack argument, then closes the library and returns.

### AP2

- `boot_ap2_setup_and_jump`: Loads the address of the boot ap2 string into a0 with length 12 in d0, then branches to the startup code that stores these and jumps onward.
- `init_sequence_clear_color_and_trap_loop`: Calls eleven setup routines in order, clears the COLOR00 hardware register, loads a data pointer into a0, then loops trapping to the system while calling another routine.
- `open_dir_and_load_file_loop`: Stores a return pointer, writes a filename into memory, copies a string, opens a directory via trap, reads a file into memory and calls a loader before setting a flag.
- `data_table_ori_bytes_not_code`: Nine ori.b instructions with zero and 32 immediates on d0 are data bytes decoded as code, not executable logic.
- `count_entries_until_double_zero`: Stores minus one in a longword global, walks a table of records skipping nonzero words, increments the counter per record and stops at two consecutive zero words.
- `parse_digit_after_call_and_store`: Calls a subroutine, tests d0 for zero or negative, clears d0, loads a byte from a0, subtracts 48 and stores the result in a longword global.
- `copy_filename_to_ec0_buffer`: Calls the filename parser, then copies the returned d0 bytes from (a0) into the buffer at pc-relative ec0, null-terminates it and returns; negative or zero counts skip the copy.
- `copy_filename_to_buffer_and_append_dot`: Tests the nonzero word flag, calls the input routine, then copies d0 bytes from a0 into the buffer at the pc-relative address, appends a dot, stores the end pointer and length plus four, and clears the byte three past the end.
- `read_up_to_four_entries_into_tables`: Loops up to four times calling the input routine, storing each returned length in the word table and each buffer pointer in the long table, copying the bytes, and saving the final count to the global word.
- `allocate_400_bytes_and_switch_stack`: Calls two subroutines, allocates 1024 bytes via d0, stores the returned a0 pointer at absolute address 1038, then swaps the stack pointer to a0 plus d0 and returns.
- `init_clear_3400_bytes_and_setup_struct`: Clears a global, allocates 3392 bytes and zero-fills it, calls two subroutines to get values, stores them in globals, then fills fields of the returned structure with pointers and a word.
- `index_table_by_d0_times_106`: Tests d0, rejects negative or values of 32 and above by branching away, else multiplies d0 by 106 and adds the base pointer from memory into a0, returning d0 unchanged.
- `test_bit_in_mask_word`: Checks whether bit d0 (0 to 31) is set in the longword at the pc-relative mask; returns d0 as -1 if the bit is set, otherwise leaves d0 unchanged, preserving d1.
- `find_and_set_free_bit`: Scans bits 0 to 31 of the pc-relative mask for the first clear bit, sets it in memory, and returns that bit index in d0, preserving d1.
- `clear_bit_in_mask_word`: Validates d0 is between 0 and 31, clears that bit in the longword at the pc-relative mask, stores the mask back, and preserves d1.
- `clear_global_at_16ca`: Writes zero to the longword at absolute address 16CA and returns to the caller.
- `enable_dma_channels_via_dmacon`: Writes the value 8200 to the DMACON hardware register, setting bit 15 to enable DMA channels, then returns.
- `call_four_setup_subroutines`: Calls four unnamed subroutines in sequence and then returns to its caller.
- `play_sixteen_sound_effects_from_table`: Loads a sound table address into a0, calls a setup routine, then loops sixteen times loading successive pointers from a1 and calling a play routine, incrementing d0 until it reaches sixteen.
- `enable_interrupts_and_restore_status`: Calls a subroutine, writes the value with bit 15 set to the interrupt enable register to enable interrupts, then restores the status register from the stack and returns.
- `set_d0_and_jump_to_sound_loop`: Sets d0 to fifteen, loads a table address into a0 and a low memory pointer into a1, then branches to another routine to continue processing.
- `copy_eight_nonzero_longs_to_custom_chip`: Copies up to eight longwords from the table after the current PC into the custom chip register area starting at address 96, skipping any zero entries and advancing the destination pointer each time.
- `push_eb4_offset_46_longword_and_return`: Loads the pointer stored after the current PC into a0, reads the longword at offset 70 from it, and if nonzero pushes that value onto the stack before returning.
- `read_ciaa_icr_call_1cf4_push_offset_3e`: Reads the CIA-A interrupt control register into d0, calls the routine at 1cf4, then loads the pointer after the current PC and pushes the longword at offset 62 from it if nonzero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `push_vector_32_from_base`: Loads the base pointer from the pc-relative longword, reads the longword at offset 50 into d0, and if nonzero pushes it onto the stack before returning.
- `push_vector_36_from_base`: Loads the base pointer from the pc-relative longword, reads the longword at offset 54 into d0, and if nonzero pushes it onto the stack before returning.
- `push_vector_3a_from_base`: Loads the base pointer from the pc-relative longword, reads the longword at offset 58 into d0, and if nonzero pushes it onto the stack before returning.
- `jump_via_struct_offset_66`: Loads the pointer at the pc-relative global into a0, reads the longword at offset 66 into d0, and if nonzero pushes it on the stack and returns to it.
- `jump_via_struct_offset_74`: Loads the pointer at the pc-relative global into a0, reads the longword at offset 74 into d0, and if nonzero pushes it on the stack and returns to it.
- `jump_via_struct_offset_78`: Loads the pointer at the pc-relative global into a0, reads the longword at offset 78 into d0, and if nonzero pushes it on the stack and returns to it.
- `push_vector_52_from_base`: Loads the base pointer from a nearby constant, reads the longword at offset 82 into d0, and if nonzero pushes it on the stack before returning.
- `push_vector_56_from_base`: Loads the base pointer from a nearby constant, reads the longword at offset 86 into d0, and if nonzero pushes it on the stack before returning.
- `push_vector_5a_from_base`: Loads the base pointer from a nearby constant, reads the longword at offset 90 into d0, and if nonzero pushes it on the stack before returning.
- `push_global_offset_5e_vector`: Loads the global pointer from pc-relative memory into a0, reads the longword at offset 94 into d0, and if nonzero pushes it onto the stack before returning.
- `push_global_offset_62_vector`: Loads the global pointer from pc-relative memory into a0, reads the longword at offset 98 into d0, and if nonzero pushes it onto the stack before returning.
- `push_global_offset_66_vector`: Loads the global pointer from pc-relative memory into a0, reads the longword at offset 102 into d0, and if nonzero pushes it onto the stack before returning.
- `enable_audio_interrupt_and_call_setup`: Saves d0 and a0, calls two subroutines, loads d0 with 3, then writes 32776 to INTENA to enable the audio interrupt before restoring registers and returning.
- `disable_audio_interrupt_and_ack_cia`: Saves d0, disables the audio interrupt by writing to the custom chip interrupt enable register, calls the sound routine with d0 set to three, clears the CIA interrupt control register bit, then restores d0 and returns.
- `vertical_blank_interrupt_handler`: Interrupt handler that saves d0, selects a handler based on flags, calls setup or teardown routines, and on error waits for the vertical blank signal before restoring display and audio registers and returning from the exception.
- `match_name_against_pattern_list`: Saves a0-a2, swaps a0 and a1, then loops over a null-terminated pattern string, calling a matcher for each entry and advancing a1 until a match or end of string.
- `restore_a0_a2_and_return`: Pops a0, a1 and a2 from the stack and returns to the caller, restoring the registers saved by the matching name-pattern routine.
- `resolve_and_apply_name_pattern`: Saves d5-d7/a0-a3, sets a flag, calls a lookup, then loops matching a name against patterns, invoking several subroutines and restoring registers before returning.
- `check_flag_and_copy_d5_d7_to_a0`: Saves d0/a0-a1, tests a word at offset 28 of a3, and if set loads a3 offset 20 into d0, calls a routine with a1 at a0 offset 12, and on nonzero result copies d5-d7 to a0 before restoring and returning.
- `set_volume_digit_and_refresh_display`: Compares d0 with a cached word; if changed stores it, calls a routine, writes three digit bytes into the buffer at the pointer from offset 4104, then calls two more routines with d0 and a1 before restoring registers.
- `dispatch_interrupt_by_d2_index`: Saves d2/a6, returns early if d2 exceeds 6, calls a routine, indexes a jump table by d2 times four, jumps through a6, calls another routine, restores registers and returns from interrupt.
- `store_d0_word_to_global_1012`: Writes the low word of d0 into the longword global at address 1012 and returns to the caller.
- `install_85c_handler_and_dispatch`: Stores the longword 85c into global 16c2, tests word at 1882, and if zero loads 16c2 into a6 and jumps to it; otherwise clears 16ca and calls the routine at 1466.
- `install_82e_handler_and_dispatch`: Stores the longword 82e into global 16c2, tests word at 1882, and if zero loads 16c2 into a6 and jumps to it, then branches onward.
- `set_300_and_call_1466`: Stores the immediate value 300 into the longword at absolute address 5834, then calls the routine at 5222 and returns.
- `dispatch_or_fetch_node_field`: Checks the word at 3756; if zero it installs a handler pointer, optionally calls it or clears 5834 and calls 5222; otherwise it calls 3446 with a0/a1 and returns the longword at offset 24 of a0.
- `save_a5_switch_frame_check_flag`: Saves a5 into 5826, points a5 at 5834, tests the word at 6274 and either calls the saved handler through a6 or branches past it.
- `clear_global_and_call_init`: Clears the longword at absolute address 16CA, then calls the routine at 1466 before returning to its caller.
- `scan_string_length_and_call_8d0`: Saves d0-d1/a0, scans the a1 string for its terminating zero, sets d1 to zero, calls the routine at 8D0 with a0 pointing at the string, loads a1 from a nearby data table and restores registers.
- `scan_two_strings_and_call_246e`: Saves d1-d2/a0-a2, scans the a1 string for its zero terminator, calls 8D0 with the string, then calls 246E with a0 from a2, d0 from d2 and a1 from a data table, restoring registers before returning.
- `append_path_with_retry_scan`: Scans the a1 string for its terminator, builds a path from a0 and a1 via two helper calls, retrying with an incremented index until the helper returns non-negative or the limit at the table is reached.
- `append_path_single_attempt`: Scans the a1 string for its terminator, then builds a path from a1 using two helper calls once, returning after the first non-negative result or when the index limit is reached.
- `build_path_from_index_and_suffix`: Copies the indexed path entry into the buffer at a1, appends a slash unless it ends in slash or colon, then appends d0 bytes from a0 and a terminating zero.
- `parse_next_token_skip_spaces`: Scans the argument buffer at the global pointer, skipping spaces and an optional dash or quote, then measures the token length and advances the stored pointer and remaining count, returning the length or minus one.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `snapshot_dma_int_aud_control_registers`: Calls a helper, then copies the ADKCONR, DMACONR and INTENAR hardware register values into three words of the structure pointed to by the global pointer.
- `apply_audio_filter_and_period_settings`: Writes the audio filter bit, copies two longwords into the copper list pointers, waits for the vertical blank bit, then reads the current audio and interrupt registers and programs the three audio channel periods and volumes.
- `load_exe_hunk_and_enter_rte`: Saves DMA and interrupt registers, copies 106 bytes of a hunk, parses EXEC header, allocates hunk memory, then restores registers and returns via rte.
- `count_a5_longwords_times_four`: Walks the longword list at a5 counting entries until a zero terminator, then returns the count plus one shifted left by two in d0.
- `load_hunk_file_and_dispatch_type`: Opens a file via dos trap, checks the first longword for the hunk magic, then jumps through a table indexed by the hunk type bits.
- `call_128e_then_jump_c64`: Calls the routine at offset 128e, then branches unconditionally to the shared continuation at offset c64.
- `call_12a0_then_jump_c64`: Calls the routine at offset 12a0, then branches unconditionally to the shared continuation at offset c64.
- `scan_tokens_count_and_dispatch`: Calls a setup routine, then scans a token list skipping entries whose masked value is not 3f2, increments the counter at offset 12 of a3 while it stays within the limit at offset 8, calls a display routine, loads a0 from offset 16 of a3, restores registers and returns.
- `zero_filled_data_area_2`: Contains a single zero word that decodes as ori.b #0,d0; this range is zero-filled data referenced by address, not executable code.
- `walk_linked_list_via_offset_four`: Saves d0 and a0, loads the longword at offset four of a0, calls a routine, shifts the result left by two, and branches based on the outcome while adjusting a0.
- `restore_d0_a0_and_return`: Pops d0 and a0 from the stack with movem and returns immediately to the caller.
- `dispatch_irq_vectors_and_chain_lists`: Saves registers, reads the interrupt vector base, stores the new vector and handler pointer, then walks two pointer lists at offsets 32 and 36 calling the free routine on each entry.
- `free_second_pointer_list_entries`: Loads the pointer list at offset 36 of a2, frees it, then walks the list at offset 36 freeing each non-null entry until a null terminator is reached.
- `restore_stack_and_return_from_interrupt`: Frees the list at offset 36, rebuilds the saved stack frame from a2 fields, restores registers, then calls two routines and returns from the interrupt with rte.
- `zero_filled_data_area_3`: A single zero word that decodes as an ori.b instruction; this is zero-filled data referenced by address, not executable code.
- `find_entry_by_index_or_name`: Walks a table of 32-byte records using the pointer at pc-relative offset, matching either a case-insensitive name in a0 or a record index in d0, and returns the record address in a0 or -1 in d0.
- `interrupt_return_stub`: Executes an rte instruction to return from an interrupt, restoring the saved status register and program counter from the supervisor stack.
- `interrupt_dispatch_and_color_cycle`: Scales d0 by four to index a jump table, jumps to the handler and returns from exception; the handlers push 32 or 16, save all registers, poll CIAA_PRA fire bit and write an incrementing colour to COLOR00.
- `init_linked_list_headers`: Walks a table of list pointers, and for each entry clears the first longword, stores the byte distance to the next node, and writes the computed next-node address into the third longword.
- `allocate_aligned_block_from_list`: Rounds d0 up to a four-byte boundary plus twelve, scans the list table for a free node via a helper, and returns the block pointer in d0 after saving d1-d5 and a1-a3.
- `insert_size_into_free_block`: Walks a free-list block chain from a1, checks each block's size field, and if the requested size in d0 fits, splits the block, stores d0 and the remainder, and returns d1 set.
- `find_and_clear_free_block`: Saves registers, scans a table of memory blocks for one containing the address twelve bytes below a0, verifies its size field, clears its first longword, recomputes its end pointer and calls a routine with a2 and a4.
- `coalesce_adjacent_free_blocks`: Walks blocks from a0 to a1, recomputes each block's end pointer, remembers the first free block and merges it with following free blocks via the merge routine.
- `link_relocatable_block_offsets`: If d0 is nonzero, stores a0 minus d0 at offset 4 of the block, then writes the sum of the first word, that offset and the block address to offset 8, and returns zero in d0.
- `load_table_count_and_clamp_index`: Saves d1-d4 and a0-a3, clears d0, loads a word count from the table after the current pc into d3, decrements it, and clamps d1 against it before branching.
- `iterate_table_entries_call_worker`: For each index in d3, scales it by four, loads the table entry at a0 into a1, calls the worker, and repeats with dbra until d3 is exhausted, then restores registers and returns.
- `sum_or_max_block_lengths`: Reads a count and block descriptors from a1, validates each block header, and accumulates block lengths into d0 or keeps the maximum depending on d2, returning after all blocks.
- `continue_block_scan_keep_max`: Compares d0 with d4 keeping the larger, advances a2 to the next block, loops over blocks and dbra count, then pushes an error code and branches to the error path.
- `call_106c_with_ffff_d1`: Saves d1 on the stack, sets d1 to minus one, calls the routine at 106c, tests d0 and returns after restoring d1 if the result is nonzero.
- `check_d1_flag_after_bsr`: Pushes d1, clears d1 to zero, calls the routine at offset 106c, tests the returned d0 and returns with d1 restored only if d0 is nonzero.
- `open_graphics_library_and_install_vblank`: Calls the check routine with d1 set to one, then saves all registers, opens graphics.library, stores its base and jump table entries, and installs a vertical blank interrupt vector.
- `vblank_blitter_swap_interrupt`: Saves registers, waits on VHPOSR until the beam passes line 144, then disowns and owns the blitter, disables and enables interrupts, and chains to the old vertical blank vector.
- `cold_boot_hardware_init`: Saves a7, masks interrupts, points a5 at custom chip base, calls seven init routines, snapshots ADKCONR, DMACONR and INTENAR into globals, writes them back to ADKCON, DMACON and INTENA, sets a flag and jumps to the main entry.
- `warm_reset_restore_and_rte`: Restores a7 from saved value, installs a new trap handler, masks interrupts, reinitializes hardware via helper calls, re-enables DMA and interrupts from saved globals, sets copper pointers and returns from exception with rte.
- `close_library_and_restore_regs`: Pops a return address into d0, stores a value at absolute address 128, loads a library base into a1, calls CloseLibrary through the exec base in a6, restores registers, clears d0 and returns.
- `restore_vblank_vectors_and_audio`: Saves a7, restores the saved stack pointer, installs the level-2 interrupt vector, reads ADKCONR, DMACONR and INTENAR into saved words, calls three setup routines, then writes masked interrupt and DMA enable values to the custom chip registers and clears the flag word before returning with rte.
- `vblank_handler_swap_audio_state`: Restores the saved stack pointer, saves registers, reads ADKCONR, DMACONR and INTENAR, calls three setup routines, sets the flag byte, then writes masked interrupt and DMA enable values to the custom chip registers and returns with rts.
- `set_copper_list_and_wait_blitter`: Writes the value 16384 to the custom register at offset 154 from a5, then polls bit 5 of the register at offset 31 from a5 until it is set before returning.
- `read_ciaa_ciab_ports_to_tables`: Saves d0 and a0-a1, then reads CIAA and CIAB port A into two destination tables via a shared copy routine, and restores the registers.
- `copy_cia_port_and_pulse_handshake`: Copies port A and port B bytes from the CIA base in a0 into offsets 14 and 15 of the table in a1, then writes and reads back the handshake register.
- `read_ciaa_ciab_ports_second_variant`: Saves d0 and a0-a1, reads CIAA and CIAB port A into two tables using the alternate copy routine, then restores the registers and returns.
- `write_two_bytes_and_read_status_byte`: Copies byte at offset 14 of a1 to offset 3584 of a0 and byte at offset 15 to offset 3840, writes 127 to offset 3328, then reads that byte back into d0 and returns.
- `install_temporary_level_two_vector`: Saves the current level two autovector from address 32 into a longword, installs the address 5682 there, reads the status register into d0, then restores the saved vector and returns.
- `interrupt_return_with_status_word`: Copies the status register into d0, clears the supervisor bit in d0, and returns from the exception with rte, leaving the masked status in d0.
- `assert_ciab_disk_select_lines`: Writes 255, then 135, then 255 to CIAB port B at the disk control register, with two nops between each write, then returns.
- `retry_call_until_nonzero_result`: Saves d0 on the stack, calls a subroutine, compares the returned word in d0 with zero and repeats the call while it is zero, then restores d0 and returns.
- `zero_filled_data_area_4`: The single word is zero, decoded as ori.b with an immediate zero; this is zero-filled data referenced by address, not executable code.
- `zero_filled_data_word_at_16c6`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by address, not executable code.
- `zero_filled_data_word_at_16ca`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data pointed to by absolute and pc-relative references, not code.
- `zero_filled_data_word_at_16ce`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data whose address is taken by callers, not executable code.
- `zero_filled_data_area_5`: Contains a single zero word that decodes as an ori.b immediate instruction; the bytes are data, not executable code.
- `push_one_and_branch_to_shared_code`: Pushes the immediate value 1 onto the stack and branches unconditionally to shared code at a later address.
- `push_two_and_branch_to_shared_code`: Pushes the immediate value 2 onto the stack and branches unconditionally to the same shared code target used by the previous entry.
- `push_three_and_branch_to_common`: Pushes the immediate value 3 onto the stack and branches to the shared routine at the later address, which consumes it.
- `push_four_and_branch_to_common`: Pushes the immediate value 4 onto the stack and branches to the shared routine at the later address, which consumes it.
- `push_five_and_branch_to_common`: Pushes the immediate value 5 onto the stack and branches to the shared routine at the later address, which consumes it.
- `interrupt_dispatch_and_vector_redirect`: Reads INTREQR, masks active bits, calls each set handler from a table, acknowledges INTREQ, and redirects the return address through a vector table before returning.
- `copy_vector_table_to_ram`: Copies 64 longwords from address zero into the RAM vector table at the relocated base, preserving d0 and a0-a1.
- `restore_vector_table_from_ram`: Copies 64 longwords from the RAM vector table back to address zero, preserving d0 and a0-a1.
- `store_a0_to_global_1994`: Copies the address in a0 into the longword global at address 1994, then returns to the caller.
- `return_constant_64_in_d0`: Loads the immediate value 64 into d0 as the return value and returns immediately; no memory or other registers are touched.
- `store_a0_into_indexed_global_table`: Saves d0 and a1, calls a routine returning an index in d0, shifts it left twice, stores a0 into the global table at 1994 indexed by that offset, restores registers, calls another routine and returns.
- `clear_bit_in_global_flag_word`: Saves d1, reads the global flag word at the pc-relative location, clears the bit indexed by d0 (or zeroes the word if d0 is negative), writes it back and restores d1.
- `set_bit_in_global_flag_word`: Saves d1, reads the global flag word at the pc-relative location, sets the bit indexed by d0 (or fills the word with all ones if d0 is negative), writes it back and restores d1.
- `zero_filled_data_area_6`: A single zero word that decodes as ori.b with immediate zero; this is zero-filled data referenced by address, not executable code.
- `swap_and_merge_bitfields_d0_d1`: Saves d2, computes d1 AND NOT d0 and d0 AND NOT d1 into swapped halves, clears bit 15 of d0, sets bit 15 of d1, restores d2 and returns.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `read_vposr_and_vhposr_with_intena`: Saves d1, reads INTENAR masking bit 14 into INTENA, reads VPOSR bit 0 and VHPOSR into d0, re-enables the interrupt, restores d1 and returns.
- `scan_a0_list_for_magic_and_call`: Saves d0, a0 and a3, loads a table address into a3, reads a longword from a0, masks it, compares against a magic value and branches or calls accordingly.
- `increment_counter_and_compare_limit`: Loads the long at offset 12 of a3, adds one, stores it back, compares it with the long at offset 8 of a3 and branches if not greater, then pops d0/a0/a3 and returns.
- `hunk_type_data_table`: Twenty-four ori.b zero words that change nothing; the bytes are a data table referenced by address, not executable code, and clear or call nothing.
- `hunk_relocation_dispatch`: Compares d0 against AmigaDOS hunk type codes and branches to relocation, symbol, copy or clear handlers that walk a0 and a3 tables, adjust a1/a2 offsets and return.
- `release_bitmap_and_set_blit_size`: Reads a word from (a0)+ into d2, computes a bitmap slot from a3 offsets, clears the low bits of its first longword, zeroes the second, and stores a shifted size into the previous structure's field.
- `jump_to_init_sequence`: Branches unconditionally to the initialization code at a fixed offset, transferring control without touching registers or memory itself.
- `branch_to_alternate_entry`: Branches unconditionally to another code location, transferring control without modifying registers or memory.
- `branch_to_1d56`: Branches unconditionally to the code at offset 1d56, transferring execution there without touching any registers or memory.
- `branch_to_1e2c`: Branches unconditionally to the code at offset 1e2c, transferring execution there without touching any registers or memory.
- `branch_to_1e72`: Branches unconditionally to the code at offset 1e72, transferring execution there without touching any registers or memory.
- `init_ciab_and_keymap_buffers`: Clears key buffers, sets repeat counters, stores keymap pointers, zeroes 128 bytes at the keymap area, then programs CIAB port bits and returns the read port value in d0.
- `poll_ciab_keyboard_and_queue_events`: Reads CIAB keyboard port, decodes key bits into the keymap and event queue, advances the queue, and returns after storing a key event or exiting early.
- `pop_key_event_from_queue`: Pops the first word from the key event queue into d7, shifts remaining entries down, decrements the count, and clears buffers when the queue empties.
- `clear_d7_if_bitmap_bit_clear`: Saves d5, d6 and a0, loads a bitmap base from a pc-relative pointer, tests the bit indexed by d7 in the row selected by d6, and clears d7 if that bit is clear before restoring registers and returning.
- `zero_byte_into_d5_data_word`: A single instruction that ORs an immediate zero byte into the low byte of d5, leaving d5 unchanged; it is a zero-filled data word rather than executable code.
- `zero_filled_data_area_7`: Nine consecutive zero words that decode as OR-immediate-zero instructions on d0; this is a zero-filled data area referenced by address, not executable code.
- `branch_to_215c_handler`: Branches unconditionally to the code at offset 215c, transferring control there without touching registers or memory.
- `branch_to_2196_entry`: Branches unconditionally to the code at offset 2196, transferring control there without touching registers or memory.
- `branch_to_2260_entry`: Branches unconditionally to the code at offset 2260, transferring control there without touching registers or memory.
- `clear_three_word_globals_then_branch`: Branches forward past three clear-word instructions that zero three word globals, then returns; the clears are skipped by the branch.
- `read_joystick_and_update_pointer`: Reads JOY0DAT and the fire buttons via CIAA_PRA and POTGOR, updates stored mouse position and button state, and calls the pointer movement routine.
- `scale_byte_delta_into_pointer_offset`: Takes byte values in d0 and d1, computes their signed difference, scales it by the stored speed and adds the result to the word at a0.
- `clamp_word_to_range`: Reads a word from a0 and a low/high pair from a1, clamps the a0 word into that range, writes the result back to (a0) and returns.
- `zero_filled_data_area_8`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `data_word_ori_3f`: A single word that decodes as ori.b #63,d0; it is data referenced by pointer, not executable code.
- `ori_b_immediate_c7_into_d0`: Performs a bitwise OR of the immediate byte value 199 into the low byte of data register d0, then falls through to the following code.
- `zero_filled_data_area_word`: The single word here is zero, decoded as an OR of zero into d0; this range is zero-filled data referenced by address, not executable code.
- `ori_b_immediate_14_into_d0`: Performs a bitwise OR of the immediate byte value 20 into the low byte of data register d0, then falls through to the following code.
- `set_low_bits_of_d1_d2`: Sets the low bit of both d1 and d2 by ORing each with the immediate value one, then falls through to the following code.
- `decompress_block_into_a1_buffer`: Saves registers, reads a compressed block header from a0, then decodes bytes into the a1 buffer using a jump table indexed by d3, returning zeroed stack words when done.
- `adjust_a0_d0_and_jump_to_decode_loop`: Decrements a0 by one, masks d0 to its low five bits, increments d0, clears d1, then branches to the shared decode loop.
- `build_tile_index_from_d0_d4`: Decrements a0 by two, masks d0 to five bits, shifts it left eight, inserts d4 into the low byte, adds thirty-three, clears d1 and branches to shared code.
- `build_masked_pair_index_from_d0_d1`: Decrements a0 by two, masks d1 to three bits and adds three, masks d0 to two bits and shifts it left eight, copies d0 to d2, inserts d4 into d2's low byte, clears d0 and branches.
- `build_two_byte_index_from_d0`: Decrements a0 by one, masks d0 to four bits, copies it to d2, sets d1 to two, clears d0 and branches to the shared continuation.
- `adjust_a0_and_build_d1_d2_from_d4_d5`: Subtracts 3 from a0, masks d0 to 3 bits, copies d0 to d1, builds d2 from d4 with bit 7 cleared, conditionally adds 8 and 4 to d1, shifts d2 left 8 bits, loads d5 into d2's low byte, clears d0 and branches away.
- `adjust_a0_and_build_d1_d2_from_d4_d5_d6`: Subtracts 4 from a0, masks d0 to 3 bits, shifts d0 left 8 bits, loads d4 into d0's low byte, copies d0 to d1, adds 5 to d1, builds d2 from d5 shifted left 8 bits with d6 in the low byte, clears d0 and branches away.
- `adjust_a0_and_split_d0_d1_into_counts`: Subtracts 2 from a0, copies d0 to d2, moves d1 into d0, masks d1 to 2 bits and adds 3, shifts d0 right twice and masks to 2 bits plus 1, masks d2 to 2 bits, shifts it left 8 bits, loads d4 into d2's low byte and branches away.
- `unpack_bitplane_span_to_buffer`: Reads a run-length encoded span from memory at a0, decodes control bytes in d4/d5 into d1/d2, and copies bytes backwards into the buffer at a1 using a jump table indexed by d1.
- `decrement_d1_loop_and_branch_back`: Decrements the word counter in d1 and branches back to the earlier loop body while d1 is non-negative, otherwise branches to the exit point.
- `jump_to_shared_span_decoder`: Unconditionally branches to the shared span decoding routine, acting as a thin entry stub for callers that reach it through a data pointer.
- `branch_to_shared_cleanup`: Branches unconditionally to the shared cleanup code at a later address, transferring control without touching registers or memory.
- `open_resource_3ee_and_play`: Saves d1, moves d0 into d1, opens resource number 1006, and if successful plays it through the sound routines before restoring d1 and returning.
- `open_resource_3ed_and_play`: Saves d1, calls a routine to obtain a handle, opens resource number 1005, and if successful plays it via the sound routines before restoring d1 and returning.
- `open_dos_file_and_seek_start`: Saves all registers, opens the dos.library base, seeks the file handle to offset 1005, then opens the file twice with mode flags and stores the resulting handle in global memory before closing the library.
- `open_dos_library_once`: Saves all registers, checks the global library base pointer, and if zero opens dos.library via exec.OldOpenLibrary using the name string, storing the base in global memory.
- `close_dos_library_if_open`: Saves all registers, loads the global dos.library base, and if nonzero calls exec.CloseLibrary on it then clears the global base pointer to zero.
- `open_file_store_handle`: Saves all registers, loads the dos library base, calls dos.Open with the name in a1 and mode in d0, stores the returned file handle in global memory and returns nonzero on failure.
- `close_file_if_open`: Saves all registers, loads the dos library base, and if the stored file handle is nonzero calls dos.Close on it, then clears the stored handle to zero.
- `write_to_open_file`: Saves all registers, loads the dos library base, and if the stored file handle is nonzero calls dos.Write with the buffer in a0 and length in d0.
- `dos_read_via_a6_vector`: Saves all registers, loads the dos library base from pc-relative memory into a6, and if the flag at the next longword is nonzero calls dos.Read with a0 and d0 as arguments.
- `dos_seek_via_a6_vector`: Saves all registers, loads the dos library base into a6, and if the flag longword is nonzero calls dos.Seek with d0 as offset and d3 set to minus one.
- `zero_filled_data_area_9`: A single zero word that decodes as ori.b with zero immediate; this is zero-filled data referenced by address, not executable code.
- `zero_fill_data_word_at_2628`: Single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by address, not executable code.
- `zero_fill_data_word_at_262c`: One zero word decoding as ori.b #0,d0, a data byte pair used as a handler pointer target rather than real code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_runtime_entry_one`: Performs an unconditional jump to another routine in the runtime segment, transferring execution without saving any registers or return address.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_runtime_entry_two`: Transfers control with a single unconditional jump to a different runtime routine, leaving all registers and the stack untouched.
- `jump_to_runtime_entry_three`: Executes an unconditional jump to a separate routine in the runtime segment, passing control onward without altering registers or memory.
- `jump_to_shared_exit_stub`: Jumps directly to a shared exit routine elsewhere in the segment, transferring control without touching any registers or the stack.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_shared_runtime_entry`: Performs an unconditional jump to the shared runtime entry point, transferring execution without saving registers or returning.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_alternate_runtime_entry`: Performs an unconditional jump to a second runtime entry point, transferring control without touching registers or the stack.
- `undecodable_bytes_21`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_22`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_23`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_24`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_25`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_26`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_27`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_28`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_29`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_30`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_31`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_32`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_33`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_34`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_35`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_36`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_37`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_38`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_39`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_40`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_41`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_42`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_43`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_44`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_45`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_46`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_47`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_48`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_49`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_50`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_third_runtime_entry`: Performs an unconditional jump to a third runtime entry point, handing off execution with no register or stack changes.
- `undecodable_bytes_51`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_52`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_53`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_54`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_55`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_56`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_57`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_58`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_59`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_60`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_61`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_62`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_63`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_64`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_65`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_66`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_67`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_68`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_69`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_70`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_71`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_72`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_73`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_74`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_75`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_76`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_77`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_78`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_79`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_80`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_81`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_82`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_83`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_84`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_85`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_86`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_87`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_88`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_89`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_90`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_91`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_92`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_93`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_94`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_95`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_96`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_97`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_98`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_99`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_100`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_101`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_102`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_103`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_104`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_105`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_106`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_107`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_108`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_109`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_110`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_111`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_112`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_113`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_114`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_115`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_116`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_117`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_118`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_119`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_120`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_121`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_122`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_123`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_124`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_125`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_126`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_127`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_128`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_129`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_130`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_131`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_132`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_133`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_134`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_135`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_136`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_137`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_138`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_139`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_140`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_141`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_142`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_143`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_144`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_145`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_146`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_147`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
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
- `jump_vector_entry_19`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_20`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_21`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_22`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_23`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_24`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_25`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_26`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_27`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_28`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_29`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_30`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_31`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_32`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_33`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_34`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_35`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_36`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_37`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_38`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_39`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `load_ed_into_d0_and_branch`: Loads the immediate value 237 into data register d0 and then branches unconditionally to a shared routine elsewhere.
- `load_two_into_d0_and_branch`: Loads the immediate value 2 into data register d0 and then branches unconditionally to a shared routine elsewhere.
- `load_f8_into_d0_and_branch`: Loads the immediate value 248 into data register d0 and then branches unconditionally to a shared routine elsewhere.
- `load_d0_with_66_and_branch`: Loads the immediate value 66 into data register d0, then branches unconditionally to a shared routine elsewhere.
- `load_d0_with_negative_three_and_branch`: Loads the sign-extended immediate value negative three into data register d0, then branches unconditionally to a shared routine elsewhere.
- `load_d0_with_93_and_branch`: Loads the immediate value 93 into data register d0, then branches unconditionally to a shared routine elsewhere.
- `set_d0_minus_one_then_branch`: Loads the immediate value minus one into data register d0 and then branches unconditionally to another part of the code.
- `nop_then_branch_to_shared_code`: Executes a no-operation instruction and then branches unconditionally to a shared code location, changing no registers or memory.
- `nop_then_branch_to_alternate_code`: Performs a no-operation instruction followed by an unconditional branch to a different shared code location, leaving all registers untouched.
- `get_or_decrement_counter_at_a6_offset_32`: Reads the word at offset 32 of a6, decrements it and stores it back, returning zero in d0; the earlier entry increments that same word and returns a6 in d0.
- `jump_vector_entry_40`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_41`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_42`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_43`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `push_five_string_pointers_and_branch`: Pushes five different string pointers onto the stack one at a time, each preceded by reserving eight bytes, then branches to a common continuation.
- `push_one_string_pointer_and_branch`: Reserves eight bytes on the stack, pushes a single string pointer, then branches to the same common continuation used by the other pointer-pushing stubs.
- `push_return_and_jump_to_5100`: Decrements the stack pointer by eight, pushes a longword address, then branches to another routine while setting up a5 frame pointers.
- `pop_a5_and_return`: Pops a longword from the stack into address register a5 and returns immediately to the caller.
- `jump_vector_entry_44`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_byte_flag_at_c4be1c`: Sets the byte at the absolute longword address to all ones and returns immediately to the caller.
- `restore_sr_from_a0_offset_18`: Loads the status register from the word at offset 18 of a0, then loads d0 from a program-counter-relative word and returns.
- `restore_sr_from_a0_offset_18_alt`: Copies the word at offset 18 of a0 into the status register, sets d0 from a nearby constant word, and returns to its caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_subsystems_and_dispatch_by_d7`: Calls several setup routines, stores a large value and one into memory, invokes a library vector, traps, saves d7, then jumps through a table indexed by d7.
- `show_menu_text_via_trap`: Loads the address of the menu string into a0, reads a word value into d1, and issues a system trap to display it, then returns.
- `restore_display_and_advance_counter`: Saves registers, disables DMA via DMACON if a pending flag is set, calls three library vectors through a6, stores words to a buffer, clears COLOR00, increments a counter and restores registers.
- `compare_abs_difference_to_d0`: Saves d0-d2, computes the absolute difference between two nearby words into d1, compares it with d0, and returns with the flags set by that comparison.
- `compare_d0_to_abs_d1_minus_constant`: Saves d0-d2, computes d1 minus the word constant at pc offset 304, takes its absolute value, compares it with d0 and restores d0-d2 before returning.
- `store_args_and_capture_trap6_vectors`: Stores d0 and a0 to fixed memory, then calls trap 6 with d0 set to 0, 1 and 2, saving each returned longword to consecutive fixed memory locations.
- `install_callback_pointer_at_offset_70`: Loads the pointer from pc offset 552 into a0 and writes the address of a fixed routine into offset 70 of that structure, then returns.
- `copy_two_vectors_to_globals`: Loads a pointer from a program-relative longword into a0, then copies the longwords at offsets 32 and 36 from that structure to two fixed global addresses before returning.
- `load_two_pointers_into_a0`: Loads two program-relative longword pointers into a0 in succession, leaving the second pointer in a0, then returns to the caller.
- `init_sequence_then_trap_two`: Calls four subroutines in order, clears the word at a fixed global address, loads a program-relative word into d7 and executes trap number 2.
- `poll_keyboard_and_adjust_menu_index`: Reads a key via the a6 library call, maps letters to a menu index, compares the index with the previous value and stores the clamped result in the menu state word.
- `draw_selected_menu_entry`: Calls the menu redraw, reads the current menu index word, scales it by four, loads the matching entry pointer from the table and draws it into the screen buffer.
- `redraw_all_menu_entries`: Loops three times over the menu entry pointer table, drawing each entry into the screen buffer at the corresponding position before returning.
- `copy_13_words_stride_16`: Copies thirteen words from the address in a0 into a1, advancing a0 by two bytes and a1 by fourteen bytes per word, then returns.
- `data_table_three_ori_bytes`: Three ori.b instructions with immediate bytes zero, one hundred four and two hundred eight; the bytes are data decoded as code, not executable logic.
- `data_table_two_ori_words`: Two ori.w instructions with immediate words one hundred three and sixty-five thousand five hundred thirty-five; the bytes are data, not executable code.
- `stop_audio_dma_and_advance_track`: Saves registers, disables audio DMA via DMACON if a pending value is set, clears that value, calls three audio library vectors through a6, stores d0-d2 to a buffer and increments a counter.
- `compare_d0_against_absolute_counter`: Saves d0-d2, loads the word counter at 4352 into d2, computes its absolute value into d1, compares d0 with it and returns with the saved registers restored.
- `install_copper_list_and_start_audio`: Calls a setup routine, points the copper list registers and COP1LCH at a fixed list, clears COPJMP1, writes a value to a custom register, then calls the compare routine with d0 set to one.
- `copy_two_blocks_and_third_table`: Copies a 8000-byte block and a 7-entry block via a shared copy routine, then copies 536 bytes from a pc-relative table into the first destination and returns.
- `copy_block_with_swapped_longwords`: For each of d0+1 entries, stores a0 as two swapped words at a1, advances a1 by 10 and a0 by d1, then returns.
- `save_d0_a0_and_read_two_vectors`: Stores d0 and a0 to fixed memory, reads two system vectors via trap 6 with d0 set to 0 then 1, stores both results, and returns.
- `install_vector_pointer_in_structure`: Loads a pointer from a program-relative location into a0, stores a fixed code address into offset 70 of that structure, and returns.
- `zero_filled_data_word`: A single zero word that decodes as ori.b with zero immediate on d0; it is zero-filled data referenced by address, not executable code.
- `zero_filled_data_word_second`: Another single zero word decoding as ori.b with zero immediate on d0; zero-filled data reached by address, not executable code.
- `decompress_backward_byte_stream`: Saves registers, reads a big-endian length and control byte backwards from a0, then loops decoding bytes into a1 until a1 reaches the saved end pointer, returning with d7 tested.
- `decode_one_byte_control_case`: Decrements a0 by one, masks d0 to five bits, increments d0, clears d1 and branches back to the shared decode loop.
- `decode_two_byte_control_case`: Decrements a0 by two, masks d0 to five bits, shifts d0 left eight bits, inserts d4, adds thirty-three, clears d1 and branches to the shared decode loop.
- `adjust_a0_and_build_d2_from_d4`: Subtracts 2 from a0, masks d1 to 3 bits and adds 3, masks d0 to 2 bits and shifts it left 8, copies d0 into d2 then overwrites its low byte with d4, clears d0 and branches away.
- `decrement_a0_and_set_d2_low_nibble`: Subtracts 1 from a0, masks d0 to 4 bits and copies it into d2, sets d1 to 2, clears d0 and branches to another routine.
- `build_d2_from_d4_d5_with_bit_clear`: Subtracts 3 from a0, masks d0 to 3 bits into d1, copies d4 into d2 and clears its bit 7, conditionally adds 8 and 4 to d1, shifts d2 left 8 and stores d5 in its low byte, clears d0 and branches.
- `build_byte_from_d4_d5_d6_and_clear_d0`: Subtracts four from a0, masks d0 to three bits and shifts it left eight, inserts d4 into the low byte, copies d0 to d1 and adds five, builds a byte from d5 and d6 into d2, then clears d0 and branches away.
- `split_d0_d1_into_two_byte_counts`: Subtracts two from a0, copies d0 to d2, moves d1 into d0, derives two small counts from d1 and d0 with masks and shifts, inserts d4 into d2's low byte, then branches away.
- `unpack_bitplane_run_into_a1`: Adjusts a0 and d0, reads four bytes from a3 into d3 through d6, computes a destination offset from d4 and d5, then copies bytes backwards into a1 in runs selected by a jump through a5.
- `decrement_d1_loop_to_earlier_block`: Decrements d1 and branches back to an earlier address while it is non-negative, otherwise branches to another earlier address; no registers other than d1 are touched.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
