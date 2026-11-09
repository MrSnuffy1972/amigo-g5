# Game 12 (AnotherWorld v2.4 It): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00012). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3294469 top-level bytes in 25 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 389 of 389 named (0 author symbols), 389 of 389 described (199 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2229 of 2229 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 924354 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4205 of 4205 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 477902 |
| code | 15052 |
| data | 2801450 |
| padding | 65 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 836 | 65 |
| another | 1 | 3087 | 106 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 266 | 211 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 6 |
| bank01 (raw code file, load address unknown) | 1 | 0 | 0 |
| bank02 (raw code file, load address unknown) | 1 | 0 | 0 |
| bank03 (raw code file, load address unknown) | 1 | 0 | 0 |
| bank0A (raw code file, load address unknown) | 1 | 0 | 0 |
| bank0D (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- AnotherWorld_v2.4_It/AnotherWorld_v2.4_It.hdf: hard-disk image (AmigaDOS OFS, 3552 blocks): its filesystem parses and all 20 files in it are byte-identical to the extracted tree AnotherWorld_v2.4_It_hdf, which is recovered file by file; no block outside the filesystem holds data
- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 190 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- AnotherWorld_v2.4_It/AnotherWorld_v2.4_It_hdf/AnotherWorldIt/data/bank01: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AnotherWorld_v2.4_It/AnotherWorld_v2.4_It_hdf/AnotherWorldIt/data/bank02: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AnotherWorld_v2.4_It/AnotherWorld_v2.4_It_hdf/AnotherWorldIt/data/bank03: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AnotherWorld_v2.4_It/AnotherWorld_v2.4_It_hdf/AnotherWorldIt/data/bank0A: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AnotherWorld_v2.4_It/AnotherWorld_v2.4_It_hdf/AnotherWorldIt/data/bank0D: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 5 never-loaded region(s), 924354 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `return_minus_one_constant`: Loads the immediate value minus one into d0 and returns immediately to the caller, providing a constant return value.
- `init_library_base_and_jump_table`: Stores the incoming a0 library base in memory, sets a5 to it, calls offsets 96, 52 and 100 of the base, then jumps through the pointer at offset 2.
- `set_two_a5_offsets_and_jump`: Loads the longword at program counter offset 32, adds an address, stores it at a5 offsets 6 and 18, adds an address and branches to a relative routine before jumping to a5.
- `set_a3_to_80000_and_jump_a5`: Loads the immediate value an address into address register a3, then jumps to the address held in a5 without saving a return address.
- `read_and_mask_d0_from_pc_relative`: Loads a longword from a program-counter-relative location into d0, clears bits 4 through 6 and the high byte, and returns with the masked value in d0.
- `test_intreqr_restore_regs_and_rte`: Reads the interrupt request register at the hardware address to acknowledge it, restores d0-d1, a0-a1 and a5-a6 from the stack, then returns from the exception.
- `acknowledge_interrupt_and_return`: Reads the word at the top of the stack and writes it to the interrupt request register at a fixed address, then pops two bytes from the stack and returns.
- `push_return_address_and_set_a3`: Makes room on the stack, moves the saved return address and a3 into the new slots, loads a3 with the address of the code at 3196, sets d0 to zero and d1 to a2, then returns.
- `set_bit1_in_a1_offset_e`: Pops the return address into d1, pushes the address of the code at 3196, pushes d1 back, sets bit 1 of the byte at offset 14 from a1, and returns.
- `load_custom_chip_base_into_a0`: Loads the custom chip register base address into a0 and branches to the routine at the fixed offset, touching the custom hardware register area.
- `store_d0_to_scratch_and_load_ciab_base`: Stores the incoming d0 value into a scratch longword referenced by pc, then loads the CIAB base address into a6 and returns.
- `copy_scratch_to_cop2lch_and_jump_a0`: Copies the scratch longword referenced by pc into the COP2LCH hardware register, then jumps to the address held in a0.
- `copy_a0_offset6_byte_to_a6_800`: Reads the byte at offset 6 from the structure pointed to by a0 and stores it at offset 2048 from a6, then returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_global_equals_21000_return_4_or_1`: Loads the longword at the pc-relative global into d1 and returns 4 in d0 if it equals an address, otherwise returns 1.
- `init_two_minus2_words_and_pointer_fields`: Writes minus two as a longword at a3, advances a3, stores the new a3 into fields of a4 and a2, writes minus two again, clears a word at a4 offset 386, skips the return address and returns.
- `copy_node_offsets_to_a3_fields`: Reads two pointers from offsets four and eight of the structure at a1; if non-null, copies the longword at offset four of each pointed node into offsets 50 and 54 of a3, then pops the return address.
- `return_minus_one_if_d0_nonzero`: Decrements d0, sets the byte to all ones if the result is nonzero, sign-extends it to a longword and returns that value in d0.
- `compare_not_d0_against_pc_bytes`: Clears d4, inverts and rotates d0, compares it with two bytes following the program counter, branches on matches, and otherwise stores the VHPOSR byte into memory at a1.
- `push_minus_one_and_branch`: Pushes the immediate value minus one onto the stack, then branches forward to another routine without returning here.
- `restore_registers_and_disable_interrupts`: Tests d0, and if nonzero unwinds several saved register sets from the stack, writes to INTENA to disable interrupts, then rearranges stack words and returns.
- `compute_scroll_offset_from_table`: Reads three fields from a table pointed to by a0, multiplies and shifts them to compute a combined offset, stores it at offset 198 of a2, then restores d2, a2 and a3 and returns.
- `select_actor_state_and_dispatch`: Reads actor index from a3 offset 67, clears a1 offset 31, sets it to 29 if a3 flag bit 1 is set, then dispatches through a1 offset 40 jump table based on a1 offset 29 and returns the state byte in d0.
- `toggle_flag_and_negate_word`: Negates the word at a1 offset 736b, flips bit 7 of the byte at a3 offset 65, and returns a byte in d0 indicating whether that bit was previously clear.
- `compute_flag_bit_into_offset_32`: Tests bit selected by a3 offset 67 against the byte at pc-relative table, stores the resulting zero-or-one longword into a1 offset 32, then returns.
- `push_pc_relative_address_plus_29e30`: Pushes the longword at the program counter offset onto the stack, adds the immediate value an address to it, and returns that computed address.
- `push_pc_relative_address_plus_296e8`: Pushes the longword at the program counter offset onto the stack, adds the immediate value an address to it, and returns that computed address.
- `clear_bit_and_advance_counter`: Clears a bit indexed by byte at offset 67 of a3 in memory near the program counter, and if it was set, toggles bit 1 at offset 64 of a3 and increments the longword at offset 294.
- `set_d1_to_65537_and_jump`: Loads the immediate value 65537 into d1 and branches unconditionally to the shared routine at the target address.
- `read_byte_at_a0_offset_d2`: Tests d2 for a negative value and returns early if set; otherwise loads the byte at a0 plus d2 into d3, copies d3 to d1 and returns.
- `set_d1_to_280_and_jump`: Loads the immediate value 280 into d1 and branches unconditionally to the shared routine at the target address.
- `clear_a1_offset_c_and_store_a4_pointer`: Clears the longword at offset 12 of a1, loads 12 into d4, then stores a4 into the address loaded from pc-relative location 1150 and returns.
- `zero_filled_data_area`: Contains a single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `compare_a4_to_stored_pointer_and_dispatch`: Compares a4 with the pointer stored at pc-relative location 1150; if equal it branches ahead, otherwise jumps through offset minus 12 of a5, then sets a6 and calls a routine before jumping to a6.
- `set_a3_to_d5e_and_store_self_pointer`: Loads the address of the data at pc-relative offset d5e into a3, writes a3 into memory at a3 minus 110, then loads a c58 address into a3 and moves it to d1 before returning.
- `open_whdload_device_and_init_structures`: Saves all registers, allocates 68 bytes of chip memory, opens expansion.library and whdload.device, builds two stack entries, allocates a 20-byte message with type 16 and pointers, enqueues it, then restores registers and returns.
- `open_dos_library_and_jump_entry`: Loads a pc-relative name into a1, calls the dos.library open function via a6, then loads the routine pointer at offset 22 of the returned structure and jumps to it.
- `open_dos_library_and_wait_for_message`: Opens dos.library via exec, builds a message port, sends a message, waits on the port, then dispatches on the received message type and returns a status in d0.
- `read_three_values_and_test_shifted_result`: Calls three helper routines, moves d7 into d0 and d1, loads a value from offset 28 of a4 into d2, then shifts d0 right by two and branches based on whether it is zero.
- `call_two_helpers_and_return_minus_one`: Calls two helper routines, sets d0 to minus one, and branches to a common exit, returning that value to the caller.
- `copy_stream_chunk_to_buffer`: Clamps the requested byte count to the remaining stream length, advances the stream position, copies that many bytes from the stream buffer to the output buffer, then calls the stream refill and dispatch routines.
- `advance_stream_position_and_refill`: Adds the requested byte count to the stream position, updates the high-water mark if exceeded, calls the stream refill routine with the buffer pointers, then jumps to the main dispatch.
- `set_d2_minus_two_and_branch`: Loads the value minus two into d2 and branches unconditionally to the following routine, transferring control without touching memory.
- `load_level_after_allocating_36_byte_block`: Calls three setup routines with d7, tests the returned longword, and on success stores it at offset 36 of the d7 structure before returning minus one in d0.
- `open_library_and_chain_to_level_loader`: Calls setup routines with d7, opens a library via a2 and a6, stores the base pointer, then branches into the level loader routine.
- `read_offset_20_and_call_ac8`: Loads the longword at offset 20 of a4 into d0, calls the routine at ac8, then sets d0 to minus one and branches to the exit path.
- `adjust_object_position_with_limit`: Loads object pointer from a4 offset 20, adds a delta from offset 24 or 28 depending on sign, compares against the object's limit at offset 148, and either stores the new position or returns failure with d1 set.
- `scale_a4_offset20_by_four`: Reads the longword at a4 offset 20 into d7 and shifts it left by two bits, returning the scaled value in d7.
- `scale_a4_offset24_by_four`: Reads the longword at a4 offset 24 into d7 and shifts it left by two bits, returning the scaled value in d7.
- `load_a4_offset_1c_shifted_left_two`: Loads the longword at offset 28 from a4 into d7, shifts d7 left by two bits, and returns to the caller.
- `shift_string_left_and_store_length`: Copies bytes from a0 forward one position until a zero byte, then writes the byte count into the slot before the shifted string and returns.
- `open_file_and_build_stream_context`: Opens a file via dos library, allocates a structure, fills fields with d2, a5 and a3, and returns the pointer in d0 or zero on failure.
- `close_file_and_return_handle`: Tests d0 for a file handle; if zero returns immediately, otherwise pushes the handle's field at offset four, calls the close routine via a6, pops the handle and jumps to the same close routine with the handle in d0.
- `build_path_and_trim_slashes`: Allocates memory for a combined directory and filename path, copies both parts with a slash separator, then collapses duplicate slashes and strips a trailing slash before returning the buffer or an error code.
- `verify_whdboot_files_present`: Calls a comparison routine twice to check the WHDBoot.exe and WHDBoot.ss strings against expected values, returning zero only if both checks succeed.
- `compare_null_terminated_strings`: Saves a0 on the stack, compares bytes at a0 and a1 until a mismatch or a zero byte, then restores a0 and returns.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `push_pc_relative_pointer_plus_32`: Pushes the longword stored at the pc-relative table entry onto the stack, adds 32 to that pushed value, and returns it as a pointer.
- `open_dos_library_and_run_program`: Opens dos.library, allocates a program segment, calls the entry with the stack argument minus 20, closes the library, and jumps to the exit vector.
- `patch_cmpi_bra_into_jsr_trampolines`: Scans 32512 bytes of instruction space for compare-immediate plus branch patterns, rewrites each match into a jump to a replacement routine, then installs a jump stub at the returned pointer.
- `read_word_and_advance_return_address`: Reads a word from the address passed on the stack, checks CIA fire button and POTGOR bits, optionally writes POTGO, then advances the stored return address by two or by the sign-extended word.
- `call_word_reader_and_skip_argument`: Loads the pointer from the stack argument, reads a word through it, calls the word-reading routine, then discards the argument and returns.
- `vsync_delay_using_vhposr`: Divides d1 by 40, saves the quotient, then waits for the vertical position register to change and counts down d1 with dbra before returning.
- `search_buffer_for_matching_bytes`: Compares d0 bytes at a0 against a2, advancing a0 until a match or a1 is passed, then returns a0 cleared to zero on failure.
- `patch_script_bytes_for_10_1b5ee`: Checks bytes at offsets 5 and 4 of a0 for characters 1 and 0 and d1 equal to an address, then rewrites three byte triples in the a1 buffer.

### another

- `game_boot_init_and_main_loop`: Program entry: calls nine setup routines, reads the ExecBase pointer into a6, sets a display timing word to 300, checks two hardware flag bits, optionally sets the timing word to 2400, clears a word, then calls four more routines and returns.
- `jump_to_video_playback_entry`: Single unconditional jump to the video playback code at a fixed low address, passing the current d0 word through as the requested frame or command selector.
- `clear_buffers_and_run_frame_loop`: Clears a 128-word table and a 32-longword table, then loops copying the 64-byte frame buffer, converting sprite offsets against the base pointer, and calling the frame routine until the stop flag is set.
- `vm_bytecode_interpreter_loop`: Reads opcode bytes from the stream at a0, dispatches on the opcode value to compare, copy, add, jump and draw operations, and loops back to fetch the next instruction.
- `branch_if_zero_to_draw_path`: Tests the zero flag and branches to the draw path when equal, otherwise jumps to the common continuation; it does not call or loop itself.
- `branch_if_nonzero_to_draw_path`: Tests the zero flag and branches to the draw path when not equal, otherwise jumps to the common continuation; it does not call or loop itself.
- `branch_if_gt_to_3d0_else_454`: Tests the condition codes and branches to the target at offset 3d0 when greater-than is true, otherwise branches unconditionally to offset 454.
- `branch_if_ge_to_3d0_else_454`: Tests the condition codes and branches to the target at offset 3d0 when greater-or-equal is true, otherwise branches unconditionally to offset 454.
- `branch_if_lt_to_3d0_else_454`: Tests the condition codes and branches to the target at offset 3d0 when less-than is true, otherwise branches unconditionally to offset 454.
- `bytecode_dispatch_loop`: Reads opcode bytes from the stream at a0, decodes and executes each instruction, updating globals and calling helper routines, then advances a0 and continues until a return opcode.
- `draw_polygon_span_edges`: Reads a polygon command from a6, scales edge deltas by d1, pushes d0/d2-d3, then calls the span filler and the edge routine to draw the shape.
- `restore_span_registers_and_return`: Pops d0/d2-d3 and d4 from the stack, loops back to the span filler while d0 counts down, then returns to the caller.
- `index_20_byte_record_by_d0`: Multiplies the low word of d0 by 20, adds the base offset 24154, and returns the resulting pointer in d0.
- `scan_word_table_and_dispatch_entries`: Saves all registers, walks a word table at a fixed base until a terminator or matching d3, then calls a routine per entry while skipping carriage returns.
- `blit_8_row_glyph_into_four_bitplanes`: Validates d1 and d2 bounds, calls a setup routine, then for 8 rows ORs or ANDs glyph bytes from a table into four bitplane pointers advanced by d7.
- `decompress_bitstream_to_buffer`: Pops a destination pointer and two control words from the stack pointed to by a0, then decodes a variable-length bitstream into the buffer ending at a1, using d0/d5 as the bit source and d1-d4 as counters, and returns after the buffer is filled.
- `refill_bit_buffer_with_xor`: Pops the next longword from the stack at a0 into d0, XORs it with d5, sets the extend flag, and rotates d0 right through extend to supply one new bit, returning the shifted value in d0.
- `extract_bits_into_d2`: Shifts d0 right and refills it from the stack at a0 when empty, rotating each bit into d2 for the count in d1, then returns with the extracted bits in d2.
- `select_and_advance_active_actor`: Scans the twenty-byte actor table for the highest-priority active entry, checks its timers against the frame counters, calls the movement and drawing helpers, then marks the actor active and advances its timer.
- `word_to_four_hex_digit_string`: Converts the low word of d0 into four hexadecimal ASCII characters, rotating d1 four bits per iteration and adding 48 or 55 to each nibble, returning the packed result in d0.
- `decrement_timer_and_mark_type3_entities`: Computes a remaining time from d6 minus a global counter difference, then scans the entity table for type 3 entries matching the d5 flag, marking each with state 4 and subtracting its timer until time runs out.
- `clear_d0_and_restore_registers`: Clears d0, copies a1 into d0, tests it, writes 4 and ORs d5 into the byte at a1, subtracts the long at offset 16 from d6, sets d0 to minus one if negative, calls a subroutine, then pops d1-d7/a0-a6 and returns.
- `scan_script_table_and_apply_deltas`: Saves all registers, walks a table of 20-byte entries until a 255 byte, and for entries whose low nibble is 4 calls a subroutine and subtracts a length from one of two globals depending on bit 7.
- `subtract_length_and_advance_entry`: Subtracts d1 from the long at the second global counter, adds 20 to a0 to move to the next table entry, and branches back into the table scan loop.
- `pop_all_registers_and_return`: Restores d0-d7 and a0-a6 from the stack and returns immediately to the caller.
- `compute_pointer_delta_and_length`: Saves all registers, computes a2 minus a1 into d4, loads a length from memory, optionally subtracts a1, halves it, and branches based on d5.
- `copy_words_with_dbra_and_swap`: Copies words from the address in a1 to the address in a2, decrementing d1 each time, then swaps d1 and branches onward.
- `copy_blocks_and_age_objects`: Copies d1 blocks of 32768 longwords from (a1) to (a2), then walks a 20-byte-stride object table subtracting d4 from each live entry whose timer exceeds d2 and whose flag byte passes the d5 test.
- `restore_all_registers_and_return`: Pops d0-d7 and a0-a6 from the stack with movem and returns, restoring the full register set saved by the caller.
- `select_animation_frames_for_event`: Dispatches on d0 event code to pick frame indices in d1-d4, marks those 20-byte table entries, stores their pointers to globals, records VHPOSR, and jumps to the main loop.
- `clear_invalid_entity_slots_and_copy_links`: Loops over the entity table at the base pointer, clearing the first byte of each 20-byte entry whose second byte is outside the range 3 to 6, then copies two longword pointers into the paired link slots.
- `clear_all_entity_slots_and_copy_links`: Loops over the entity table clearing the first byte of every 20-byte entry, then copies the two longword pointers from the source slots into the destination link slots.
- `count_entity_slots_until_terminator`: Walks the entity table in 20-byte steps reading the first word of each entry until it equals minus one, then stores the entry count minus one into the global count word.
- `blit_polygon_span_to_screen`: Computes a blitter span from d0/d1 coordinates and a0 base, programs custom chip blitter registers, calls the fill routine, then sets or clears a bit in the destination word.
- `clip_and_blit_line_segments`: Clips two line endpoints against screen bounds, stores clipped coordinates in globals, then calls the span blitter for each segment and restores registers.
- `compute_clipped_line_offsets`: Builds a packed offset in d1 from d6/d7 coordinates and the global screen width, swapping halves before returning.
- `midpoint_collision_probe`: Computes the midpoint between two coordinate pairs in d2/d3 and a2/a3, calls the collision test, and updates the stored collision masks and coordinates based on the result.
- `init_polygon_scanline_state`: Reads polygon vertex bytes from a0, scales them by the global scale factor, stores scanline and edge state in globals, then calls the edge setup routine and processes input bits.
- `draw_polygon_scanline_loop`: Walks polygon edges, clips coordinates to the screen, programs the blitter registers and calls the fill and edge routines to render the polygon.
- `init_actor_state_block`: Stores d1, d2 and d5 plus pointers a1, a2 and a4 into fields of the a5 structure, sets two fields to two and two to minus one, then branches to the wait routine.
- `wait_for_copper_blitter_idle`: Loads the custom chip base into a5 and spins while bit fourteen of DMACONR is set, returning once the blitter is idle.
- `read_joystick_into_input_state`: Reads JOY1DAT and CIA fire button, derives direction and button fields in the a1 state block, sets flag bits in its status word and restores registers.
- `copy_four_longs_to_5c14`: Saves a0 and a1, calls the pointer selector, then copies four consecutive longs from (a0) into the four globals at 5c14 through 5c20, restoring a0 and a1 before returning.
- `select_polygon_buffer_by_index`: Returns in a0 a pointer to one of several polygon buffers, offset by sixteen bytes per index for values up to three, or fixed buffers for the special index values.
- `compute_row_offset_and_load_planes`: Computes d5 as d2 times 12 plus d1, loads the four plane pointers from 5c14 through 5c20 into a0-a3, adds d5 to each, sets d7 to 40 and returns.
- `draw_sprite_clipped_to_screen`: Saves all registers, looks up a sprite descriptor by index in d4, clips its rectangle against the screen bounds, then blits masked rows through the routine selected by d0 into the four bitplanes at the plane pointers.
- `and_mask_into_four_planes`: ANDs the word in d5 into the four consecutive words pointed to by a0, a1, a2 and a3, advancing each pointer, then returns to the sprite blitter that calls it.
- `draw_span_between_two_columns`: Computes a clipped horizontal span from the object fields at a6, then calls the span filler with column masks from the two lookup tables to draw the row between the start and end columns.
- `blit_plane_pixel_masked_write`: Uses d0 as a plane bitmask, d1 as pixel data and d5 as a byte offset to set or clear bits in four bitplane pointers, with bounds checks on d2 and d3 and a special path for opcodes 16 and 17.
- `alloc_mem_via_exec_negative_one_on_fail`: Saves registers, loads the exec library base from address 4, calls AllocMem with the caller's d0 size and d1 flags, and returns minus one in d0 if allocation failed.
- `free_mem_via_exec_negative_one_on_fail`: Saves registers, loads the exec library base from address 4, moves d1 into a1 and calls FreeMem, returning minus one in d0 if the free failed.
- `spawn_effect_objects_from_two_lists`: Reads two object lists via the helper, then for each of four entries writes sprite fields at offsets 64-102 of a5 and copies positions from the lists, using a fixed frame value or a scaled offset from the current object's field.
- `emit_four_bitmasked_sprite_entries`: Fetches a sprite descriptor, loops four times over bits of d1, and writes copper sprite entries at offsets 64 to 116 using the same position with a per-bit attach flag.
- `rebuild_copper_list_and_palette`: Copies four longwords into the sprite table, waits for vertical position, optionally loads a palette into COLOR00, builds copper sprite and bitplane lists, and programs COP1LCH, COPJMP1 and DMACON.
- `keyboard_serial_command_dispatch`: Reads the CIAA keyboard serial register, inverts and rotates the byte, then compares it against several key codes and branches to the matching handler; returns if none match.
- `set_three_display_modes_and_jump`: Loads the custom chip base into a5 and writes three alternative sets of display window and bitplane control values, jumping to a continuation routine after each set.
- `apply_display_window_mode_one`: Points a5 at the custom chip registers and writes display start/stop and bitplane control values for one screen mode, then falls through to the next routine.
- `clear_copper_and_playfield_state`: Clears four words at offsets 258, 260, 264 and 266 from a5, clears the word at absolute address 5B9A, then returns.
- `reset_playfield_and_read_keyboard_serial`: Clears words at offsets 258 and 260 from a5, sets offsets 264 and 266 to 40, sets the word at 5B9A to 1, then returns.
- `set_playfield_flag_and_copy_byte`: Sets the word at 5BA2 to all ones, loads a5 from 5CA4, copies the low byte of its word to 5B9E and clears 5BA0, then returns.
- `wait_for_vertical_blank_start`: Saves d0 and a5, points a5 at the custom chip base, then spins reading VPOSR until the vertical position bits equal the start-of-frame value, then restores and returns.
- `play_audio_sample_on_channel`: Saves all registers, checks the audio state, selects a channel from d3, programs the Paula audio registers at the custom base with sample pointer, length, period and volume, waits on a counter loop, and restores registers.
- `dispatch_audio_command_by_mode`: Saves all registers, branches on d0 and d1 to store a channel or sample pointer in audio globals, validates a sample header, calls the audio setup routines, and returns.
- `load_15_resource_ptrs_and_open_ciaa`: Copies a word from the resource base, then fills 15 longwords with pointers from tagged entries, opens ciaa.resource, runs setup routines and returns whether the open failed.
- `close_ciaa_resource_if_open`: If the open flag is set, closes the ciaa.resource via the stored library base, runs the teardown routine, clears the flag and runs teardown again.
- `enable_ciaa_timer_a_interrupt`: Writes the value 129 to the CIAA control register A, enabling the timer A interrupt on the CIA chip.
- `write_cia_timer_a_from_table`: Loads a pointer to a two-byte table, then writes its second byte to CIAA timer A high and its first byte to CIAA timer A low, then returns.
- `clear_object_offsets_in_pointer_table`: Walks fifteen longword pointers from a table, and for each non-null pointer clears the longword at offset eight, then returns.
- `silence_audio_channels_and_clear_state`: Points at the custom chip base, zeroes the volume registers of all four audio channels, clears a word and a longword in low memory, then returns.
- `silence_all_audio_channels`: Points a0 at the custom chip base, clears the four audio volume registers, then writes 15 to DMACON to enable all four audio DMA channels.
- `reset_audio_and_return_zero`: Calls the audio reset routine, then sets d0 to zero as the return value and returns to its caller.
- `count_frames_and_trigger_every_sixth`: Saves all registers, increments the long counter at 21074, and when it reaches six clears the counter and calls the routine at 20364 before restoring registers and returning.
- `init_four_audio_channels`: Loads d7 with 3, points a4 at a parameter table, a6 at a state table and a5 at the audio hardware base, then calls the channel setup routine four times advancing each pointer.
- `update_audio_channel_forward`: Reads a word from a4; if zero returns, if positive adds it to the word at offset 2, clamps that value against the limit at offset 4, and writes the result to a5 offset 6 and a4 offset 2.
- `update_audio_channel_reverse`: Adds d0 to the word at a4 offset 2, clamps the result against the limit at offset 4 using a signed comparison, then branches back into the shared channel update code.
- `dispatch_anim_state_nibble`: Reads the low nibble of byte at offset 2 of a6 and branches to one of three handlers when it equals 1, 7 or 8, otherwise returns immediately.
- `lookup_pair_index_in_table`: Clears and fills words at a4 from a6, scans the word table at the fixed address for the value at a6, and stores the matched table entry offset into a4.
- `fetch_table_word_by_nibble`: Doubles d2, negates it when d4 is zero, reads the word at a0 plus that offset and stores it at offset 4 of a4 before returning.
- `store_d0_to_a4_offset_four`: Writes the low word of d0 into the word at offset four from a4, then returns to the caller.
- `store_byte_d4_then_branch_back`: Stores the low byte of d4 to the address in a4 and branches backward to an earlier instruction, forming a loop.
- `dispatch_indexed_jump_table`: Loads a table base into a0, reads a word counter, decrements it, scales it by four, loads the entry into a0 and jumps to it.
- `scan_word_table_for_match`: Shifts d0 left by one, loads the word at offset 16 of a6 into d1, then walks the word table at a fixed address comparing each entry to d1 until a match, leaving the matched word in d2 and its address in a0.
- `lookup_high_nibble_of_byte3`: Clears d0, loads the byte at offset 3 of a6, shifts it right by four to get the high nibble, then branches into the table scan to find the matching word and store the indexed entry.
- `lookup_low_nibble_of_byte3`: Clears d0, loads the byte at offset 3 of a6, masks it with fifteen to get the low nibble, then branches into the table scan to find the matching word and store the indexed entry.
- `copy_a6_offset_to_a5_field`: Reads the word at offset 16 from a6 and stores it at offset 6 from a5, then returns to the caller.
- `mixer_update_voices_and_dma`: Builds four audio channel pointers from tables, calls the voice setup routine, enables DMA, waits, then copies active voice data to the audio hardware and advances the sample position.
- `decode_voice_event_into_channel`: Reads an event word from the sample stream, looks up its instrument table entry, fills the a6 voice record with sample pointers, lengths and volume, and writes the period to the a5 channel.
- `clamp_vertical_velocity_by_direction`: Stores a global pointer and flag into the object at a6, then reads the direction nibble and adds or subtracts the byte at offset 3 from the word at offset 18, clamping the result between zero and 64 before writing it to a5.
- `add_speed_and_handle_negative_three_state`: Adds the byte at offset 3 of a6 to d3, clamps it to 64, stores it at offset 8 of a5, and when the word at a6 equals minus three copies the word at offset 2 into the display structure and clears it.
- `trigger_sound_dma_and_wait_delay`: If the word at a6 is nonzero, clears a4, copies the word at a6 to offset 16, writes the word at offset 20 to the DMACON register, then busy-waits using a global counter in d7 before handling the minus two state.
- `copy_entity_fields_to_a5`: Copies a longword and two words from offsets of a6 into the a5 structure, then ORs the word at a6 offset 20 into the longword at address 5254 and returns.
- `fill_four_buffers_by_bitmask`: Calls a setup routine, then for each of four bits in d1 fills a pointed-to buffer of 8000 longwords with all-ones or zero depending on the bit, restoring registers and returning.
- `close_dos_lock_and_show_df0_prompt`: Closes an open DOS lock, clears the display, draws a DF0 prompt, polls the keyboard for a key, then locks and examines the disk and closes it, restoring all registers.
- `wait_fire_button_release_then_signal`: Polls CIAA_PRA bit 5 until the fire button is released, then loads the value 3976 into d0 and calls the signal routine before returning.
- `open_seek_read_file_with_blitter_lock`: Saves registers, calls a setup routine, then opens a file, seeks, reads into a buffer, and owns and disowns the blitter around the transfer, returning a status.
- `signal_code_then_return_minus_one`: Loads the value 4088 into d0 and calls the signal routine, then sets d0 to minus one and branches back into the caller's loop.
- `flash_color00_eight_times`: Saves d0-d2, writes d0 to COLOR00 and busy-waits a full inner loop, repeating the outer loop eight times, then restores d0-d2 and returns.
- `flash_color00_then_clear`: Saves d0-d2, writes d0 to COLOR00 while counting down a large delay, then writes zero to COLOR00 for a shorter delay, restores d0-d2 and returns.
- `allocate_game_buffers_and_open_libraries`: Calls memory allocation and library open routines to reserve several buffers, stores the returned pointers in globals, falls back to flashing COLOR00 on failure, and opens dos and graphics libraries.
- `disable_dma_run_audio_then_restore`: Writes a value to d0, calls a subroutine, sets DMACON to disable DMA channels, calls another routine, then restores DMACON and returns.
- `allocate_and_init_interrupt_server`: Allocates 40 bytes with flags, on failure allocates 128 bytes, fills the structure fields with a data pointer, values 100 and 2, a callback, then adds an interrupt server via exec.
- `increment_word_at_a1_return_zero`: Adds one to the 16-bit word pointed to by a1, clears d0 to zero as the return value, and returns to the caller.
- `forbid_and_own_blitter`: Loads the exec library base from address 4 and calls Forbid, then loads the graphics library base from memory and calls OwnBlitter to take exclusive use of the blitter.
- `disown_blitter_and_permit`: Loads the graphics library base from memory and calls DisownBlitter, then loads the exec library base from address 4 and calls Permit to re-enable task switching.
- `set_copper_list_and_enable_dma`: Loads the graphics library base, points the copper list registers at the stored copper pointer, resets the copper jump strobe and writes the DMA control register to enable DMA channels.
- `set_four_globals_via_45da`: Loads four global longs into d1 and passes each with the constants 128, 2048, an address and an address to the routine at 45DA, then clears d0 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_runtime_entry_point`: Transfers execution with a single jump to the runtime routine located at the fixed address c-four-five-zero-e-two and never returns here.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_shared_runtime_entry_one`: Performs an unconditional jump to a shared runtime entry point, transferring execution without touching any registers or memory.
- `jump_to_shared_runtime_entry_two`: Performs an unconditional jump to a second shared runtime entry point, passing control onward without modifying registers or memory.
- `jump_to_shared_runtime_entry_three`: Performs an unconditional jump to a third shared runtime entry point, handing off execution without altering registers or memory.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_runtime_entry_b8`: Performs an unconditional jump to another routine in the runtime segment, transferring execution without touching registers or memory.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_runtime_entry_c4`: Unconditionally jumps to a different routine in the runtime segment, passing control onward without modifying registers or memory.
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
- `jump_to_runtime_entry_190`: Transfers execution with an unconditional jump to another runtime routine, leaving all registers and memory unchanged.
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
- `undecodable_bytes_148`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_149`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_150`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_151`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_152`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_153`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_154`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_155`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_156`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_157`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_158`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_159`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_160`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_161`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_162`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_163`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_164`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_165`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_166`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_167`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_168`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_169`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_170`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_171`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_172`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_173`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_174`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_175`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_176`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_177`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_178`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_179`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_180`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_181`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_182`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_183`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_184`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_185`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_186`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `load_ed_into_d0_then_branch`: Loads the immediate value 237 into data register d0, then branches unconditionally to the code at address 4cee.
- `load_d0_with_2_and_branch`: Loads the immediate value 2 into data register d0, then branches unconditionally to a shared routine elsewhere in the segment.
- `load_d0_with_negative_8_and_branch`: Loads the sign-extended immediate value minus 8 into data register d0, then branches unconditionally to a shared routine elsewhere in the segment.
- `load_d0_with_66_and_branch`: Loads the immediate value 66 into data register d0, then branches unconditionally to a shared routine elsewhere in the segment.
- `load_minus_three_and_branch`: Loads the immediate value minus three into d0 and branches unconditionally to a shared routine elsewhere.
- `load_ninety_three_and_branch`: Loads the immediate value ninety-three into d0 and branches unconditionally to a shared routine elsewhere.
- `load_minus_one_and_branch`: Loads the immediate value minus one into d0 and branches unconditionally to a shared routine elsewhere.
- `nop_then_branch_to_4cea`: Executes a no-operation instruction then unconditionally branches to the code at the target address, transferring control without touching any registers or memory.
- `increment_a6_offset_20_and_return_a6`: Executes a no-operation, branches past the following instructions, then increments the word at offset 32 of a6, moves a6 into d0 and returns.
- `decrement_counter_or_allocate_via_library`: Decrements the word at offset 32 of a6 and returns zero; the following block saves registers, computes a stack-derived pointer, loads a table entry indexed by d0 and calls a library routine.
- `undecodable_bytes_187`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_188`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_189`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_190`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_191`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `push_literal_and_branch_to_shared_tail`: Decrements the stack pointer by eight, pushes the long immediate value at the given literal address, then branches to the shared tail routine.
- `push_literal_and_setup_frame_then_return`: Decrements the stack pointer by eight, pushes a literal long, branches to the shared tail, then stores a5 in the frame, loads a new a5 from pc-relative code, stores it and returns.
- `pop_a5_from_stack_and_return`: Pops a longword from the stack into address register a5 and returns to the caller.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_display_window_and_bitplane_modes`: Branches if zero to a later address, calls a subroutine, clears bit 0 of d0, then writes display window start/stop and bitplane control values to the custom chip registers via a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
