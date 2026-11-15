# Game 14 (ArkanoidRevengeOfDoh v1.0 0765): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00014). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (935688 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 209 of 209 named (0 author symbols), 209 of 209 described (61 by rule, 42 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2199 of 2199 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 256 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5271 of 5271 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 360943 |
| code | 24532 |
| data | 549094 |
| padding | 93 |
| unused | 1026 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 163 | 11 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 4554 | 117 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 513 | 59 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 41 | 22 |
| Disk.1 bytes 0-256 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- ArkanoidRevengeOfDoh_v1.0_0765/ArkanoidRevengeOfDoh_v1.0_0765_hdf/ArkanoidRevengeOfDoh/Disk.1: raw disk image (403968 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- ArkanoidRevengeOfDoh_v1.0_0765/ArkanoidRevengeOfDoh_v1.0_0765.hdf: hard-disk image (AmigaDOS OFS, 1008 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree ArkanoidRevengeOfDoh_v1.0_0765_hdf, which is recovered file by file; no block outside the filesystem holds data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 2 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ArkanoidRevengeOfDoh_v1.0_0765/ArkanoidRevengeOfDoh_v1.0_0765_hdf/ArkanoidRevengeOfDoh/Disk.1: bytes 0-256 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 jump target
- ArkanoidRevengeOfDoh_v1.0_0765/ArkanoidRevengeOfDoh_v1.0_0765_hdf/ArkanoidRevengeOfDoh/Disk.1: 3 logged loads cover 392958 bytes of the image
- ArkanoidRevengeOfDoh_v1.0_0765/ArkanoidRevengeOfDoh_v1.0_0765_hdf/ArkanoidRevengeOfDoh/Disk.1: chunks 18-43 (bytes 4608-11264) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.97)
- 1 never-loaded region(s), 256 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 3328 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `patch_jump_vectors_and_init`: Loads colour registers from a block of values and calls two routines through a pointer table.
- `start_blit_and_wait`: Writes a size value to the blitter size register and then polls the blitter status bit until the operation completes.
- `store_byte_thirty_two`: Stores the value thirty-two into a global variable.
- `set_interrupt_request_mask`: stores a value into a global variable and modifies the interrupt request register.
- `xor_fill_block_with_countdown`: Calls a routine through a vector and, when it returns nonzero, XORs a 160-byte area with a countdown of byte values before returning 160.
- `execute_conditional_sequence`: Checks a byte in a register and, if it matches a specific value, enters a loop that calls another routine until it returns zero, then calls a routine via a pointer and another routine.
- `xor_fill_block_with_countdown_2`: XORs a 160-byte area with a countdown of byte values and returns 160.
- `wait_for_flag_toggle`: Spins until the low bit of a global is clear, then spins until that bit is set again, and returns.
- `install_interrupt_handler_and_enable`: Stores a routine pointer in a global slot, writes a control value to one CIA register and clears one bit in another CIA control byte.
- `poll_rotated_state_value`: Returns a fixed state when a control bit is clear; otherwise it sets a control bit, rotates a byte and compares it with a global, looping forever on inequality in the recovered code.

### logged disk load from Disk.1

- `initialize_display_and_copy_data`: Waits for a state, sets up the copper and colour registers, copies a block of longwords into globals and clears a longword region in memory, then runs nested loops over twenty rows and calls the copper start routine.
- `reverse_velocity_on_overlap`: Scans a list of object records and, when one lies inside a box range of a reference position, negates two velocity globals and stops; otherwise it steps through the remaining object slots, updates counters and writes a per-object state byte.
- `negate_velocity_and_scan_objects`: Scans object records for a bounding-box hit, then negates two velocity globals and continues into a per-object update loop.
- `update_velocity_components`: Loads two signed components from a per-object table at an offset taken from a global and negates each according to the sign of an input value and an equality test.
- `move_bitplane_rows_and_clear`: Walks 100 rows of 14 words, copying each word of four plane regions into a second set of regions and zeroing the source words.
- `fill_tables_with_minus_one`: Fills a 64-entry set of word and longword tables with all-ones values.
- `advance_position_with_countdown`: Counts a timer down and flips the sign of a step value when it expires, adding the step to a position each cycle.
- `dispatch_input_code_event`: Masks an input code and branches on value ranges.
- `claim_first_free_slot`: Scans eight table entries for one with a negative value and fills the first match with a value derived from the input byte and a fixed offset.
- `repeat_routine_until_low_value`: Calls one routine repeatedly while an address register value minus two is above 95, and returns once it falls to that threshold.
- `copy_table_value_with_sign`: Reads a table entry through an index, writes it into a slot with its sign set to match the slot's previous sign, and then applies the same sign rule to the slot again.
- `copy_table_value_to_slot`: Reads a table value at an index derived from the slot and a fixed stride, then stores it into the slot.
- `load_velocity_from_table_entry`: Loads a table value into one velocity global, negating it when a flag global is nonzero.
- `execute_subroutine_six`: An executed routine containing fifteen instructions.
- `configure_object_by_kind`: Chooses settings from a kind byte by comparing it with several small values.
- `reset_object_defaults`: Sets several counters and globals to fixed default values, including 7, 11, 16 and the spacing value 2080.
- `initialize_position_defaults`: Writes six fixed values to position and speed globals and sets a flag global to one.
- `run_timed_routine_loops`: Calls two routines, then runs a counted loop of routine calls followed by a second counted loop that stops after about thirty-four steps.
- `run_counted_routine_loops`: Runs a counted loop of routine calls followed by a second counted loop, and with a mode value of three it runs one more counted loop of routine calls.
- `double_word_value`: Returns its 16-bit argument multiplied by two.
- `load_object_record_fields`: Copies eight words from a record at fixed 128-byte steps into eight state globals.
- `store_object_record_fields`: Writes eight state globals back into a record at the same fixed 128-byte steps.
- `swap_block_with_itself`: Loops 192 times swapping each longword with the one at the same address, so memory is unchanged.
- `pass_through_after_routines`: Calls two routines, one of which is empty, then returns its two data-register inputs unchanged.
- `save_call_restore_position_state`: Saves three position and speed globals, sets them from the input byte and fixed values, calls three routines, then restores the saved values and returns the input unchanged.
- `call_single_routine`: Calls one routine and returns, with no other logic in the recovered body.
- `call_subroutine`: calls another routine and then returns.
- `call_subroutine_2`: executes a subroutine before returning.
- `execute_subroutine_seven`: An executed routine containing thirteen instructions.
- `return_from_input_dispatch`: This routine is an empty function that performs no operations before returning to its caller.
- `copy_rows_to_two_targets`: Copies a rectangular block of words row by row from a source region into two destination regions.
- `copy_block_rows_from_pointers`: Copies a rectangular block of words row by row between pointers passed in registers, writing each word to a destination and to a second target.
- `set_position_and_call_routine`: Sets two position globals to fixed values and a flag global to one, then calls a routine.
- `set_position_then_decrement_flag`: Sets a position global to a fixed value, copies another global into a second position global and sets a flag to one.
- `copy_eight_thousand_longwords`: Copies 8000 longwords from the source pointer to the destination pointer, one at a time.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d0.
- `setup_four_plane_blit`: Looks up an entry and, when its value is nonnegative, sets the blitter masks, control words, modulo and size from it.
- `tiny_clr`: Small routine of 1 instruction that clears d0.
- `setup_blit_from_table_entry`: Looks up a source entry from a byte in the input.
- `configure_blitter_fixed_block`: Writes fixed blitter masks, control words and modulo values, takes pointers from two registers, and sets the size register for a block with a fixed height and a width of two words.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `return_register_d0`: Returns the current value held in register d0.
- `build_plane_pointer_table`: Sets two base pointer globals, then writes four pairs of high and low words into a pointer table, each pointer 8000 bytes beyond the previous one.
- `call_routine_for_each_char`: Walks a zero-terminated byte string and calls a routine for each character before returning.
- `return_stack_argument`: Returns the value taken from the stack unchanged.
- `place_text_character`: Maps a character to a glyph offset: digits and letters fall into separate glyph ranges, and spaces advance the cursor by a fixed step.
- `process_text_character`: Processes a character by checking if it is a space, a carriage return, or a letter/digit, and updates a global pointer and a counter accordingly.
- `map_letter_glyph_and_draw`: Maps a letter-range character to a glyph offset, wrapping the offset past a threshold, stores it as a pointer, calls a routine and advances the cursor by a fixed step.
- `reset_text_stream_state`: Calls one routine, then sets a group of counter globals to zero or to fixed values and stores three pointer arguments into state slots.
- `interpret_command_stream`: Advances a timer and, when it expires, reads command bytes from two byte streams.
- `enable_interrupts_and_reset_flags`: Sets two flag globals to one, clears two counters, then writes a value with the top bits set to the interrupt-enable register.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `setup_audio_channel_from_record`: Reads a channel index and four record words, writes a pointer and three values into that channel's audio registers, and ORs the channel's enable bit into a shared mask.
- `set_global_flag_one`: Sets one global byte to one.
- `advance_multiplicative_seed`: Keeps a seed global, initialising it from the input when it is zero.
- `set_keyboard_serial_state`: Writes a constant to a global and rewrites another global to itself.
- `translate_key_code_to_char`: Looks up a keyboard code in a character table whose referenced text is the key row from the tilde to the backslash, and returns the character.
- `extract_low_nibble`: Returns the low four bits of a 16-bit register value.
- `setup_display_window_registers`: Writes a sequence of display window, data fetch and bitplane control values, sets the bitplane modulo values to zero, then calls a copper-list start routine and two more routines.
- `swap_two_global_longwords`: Exchanges two global longwords through a temporary value.
- `build_plane_pointer_table_2`: Writes three high and low word pairs into a pointer table, each pointer 8000 bytes beyond the previous one, starting from a global base.
- `poll_routine_until_zero`: Calls a routine repeatedly until it returns zero, then returns.
- `halt_in_infinite_loop`: Enters an unconditional infinite loop with no operations, so execution never returns from this point.
- `wait_and_reset_flag`: Waits until a counter global reaches two or more, then clears it and returns.
- `setup_copper_and_interrupts_2`: Configures copper registers and interrupt enable bits, then stores a routine pointer and a value into a global variable.
- `setup_copper_and_interrupts`: Writes DMA and interrupt enable values, stores a routine pointer and copper list setup, starts the copper through a routine, then writes a final DMA control value.
- `acknowledge_vblank_interrupt`: Increments a counter global, calls routines, then writes the vertical-blank request bit to the interrupt request register to acknowledge that interrupt.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `set_copper_list_pointer`: Sets the copper list pointer to the address of a global block, writes two constants to globals, and then starts the copper list through the copper start routine.
- `set_copper_list_pointer_2`: Sets the copper list pointer to a global block address, records the same address in a second pointer global, writes two constants and starts the copper list through the copper start routine.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `write_dma_control_value`: Writes a single constant to a global that the facts associate with the DMA control register.
- `configure_interrupt_dma`: When a pending mask is nonzero, it merges the mask into a shadow word and writes the mask in both set form and clear form to the DMA and interrupt registers, then clears the pending mask.
- `mask_interrupt_flags`: Writes a constant with the audio channel bits set, derives a four-bit field from it and clears matching bits in a shadow word, writing the field back when none were set.
- `stub_bad_instruction_data`: The recovered code is bad instruction data, so control flow is truncated at once and no behaviour can be recovered.
- `accumulate_global_delta`: Reads a global, writes it back unchanged and adds the difference between the old and new values to a counter.
- `toggle_global_top_bit`: Sets a global to one when bit six is clear and to zero otherwise, then sets or clears bit seven of it so the top bit toggles.
- `clear_eight_thousand_longwords`: Writes zero to 8000 consecutive longwords starting at a pointer held in a global.
- `clear_strided_longword_rows`: Clears seven longwords out of every ten, across 800 rows, starting from a fixed address.
- `call_routine_count_times`: Calls one routine once for each step of a count, running from the input value down to zero inclusive.
- `tiny_move`: Small routine of 1 instruction that copies 1 to d0.
- `add_byte_with_carry_chain`: Adds a small value to a byte.
- `nop_operation`: This routine is empty and performs no operations before returning.
- `handle_space_or_s_key`: On a space input it skips following spaces, waits for a flag global and sets a counter to two.
- `copy_table_triple_into_record`: Looks up three longwords in a table indexed by a global value times four and stores them into a record, with the third value offset by the first.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `call_routines_if_flags_clear`: Calls five routines, then calls one more when a record byte at one offset is zero, and calls a final routine when a second record byte at another offset is zero.
- `nop_placeholder_routine`: This routine is empty and performs no operations.
- `perform_blit_operations`: calls a routine to set up blitter plane pointers and control values, then performs several other routine calls and returns a concatenated value from two registers.
- `setup_blit_four_plane_pointers`: Writes fixed blitter control and mask values, then writes four plane pointers, modulo and size values, stepping the two source pointers by 8000 bytes per plane.
- `call_routines_with_saved_argument`: Calls routines with a saved register argument, and calls two of them only when the matching record bytes are zero.
- `call_routine_with_temp_pointer`: Saves a pointer global, sets it to a fixed address, calls a routine, then restores the saved value.
- `call_routines_then_swap_longs`: Calls two routines, then exchanges two global longwords.
- `call_routine_with_reset_pointer`: Points a global at a fixed address, calls a routine, then sets the same global back to that address.
- `call_routine_for_each_char_2`: Walks a null-terminated string through an address register and calls a routine for each character before returning.
- `draw_text_glyph_with_newline`: Maps a character to a glyph pointer, handling space, line feed, a control code that clears seventeen rows and carriage return separately.
- `call_four_routines_per_row`: Runs 95 rows, calling four routines in sequence on each row.
- `setup_blit_row_pointers`: Writes fixed blitter control and mask values, then loops over four plane pointers stepped by 8000 bytes and writes a size value derived from an input word.
- `configure_blit_from_pointer`: Writes fixed blitter masks, control and modulo values, a source pointer taken from a global and a fixed destination, then sets the size register to a fixed value.
- `write_dma_control`: Stores a constant value into a global variable.
- `load_color_registers_from_block`: Calls a routine, then copies sixteen words from a global block into the colour registers and writes a value to a global.
- `load_color_registers_from_block_2`: Copies sixteen words from a global block into the colour registers and writes a value to a global.
- `copy_color_registers`: Copies sixteen words from a global table to the hardware color registers and updates a global variable.
- `update_color_registers`: Copies sixteen words from a data block to the hardware color registers and updates a global status flag.
- `update_color_registers_2`: Copies sixteen words from a memory block to the hardware color registers and updates a global status value.
- `load_color_registers_from_block_3`: Copies sixteen words from a global block into the colour registers and writes a value to a global.
- `call_routine_sixteen_times`: Calls one routine sixteen times, counting down from fifteen.
- `xor_pixel_bit_into_planes`: Waits for two routine-returned values to pass their thresholds, then XORs one bit, chosen from a low-nibble value, into a word of each of three bitplanes spaced 8000 bytes apart.
- `copy_longword_block_to_buffer`: Copies a pointer global to a second pointer global, then copies 8001 longwords from one buffer to the other.
- `copy_longword_block_to_buffer_2`: Copies 8001 longwords from one buffer pointer to another.
- `clear_four_plane_word_rows`: Clears one word in each of four plane regions spaced 8000 bytes apart, across 104 rows of 14 words starting from a fixed address.
- `copy_and_transform_data`: Copies 8192 words from one memory area to another, then processes 8 blocks of 17 words each by transforming pixel data using bitwise operations and shifts, followed by copying bytes from a large buffer to specific offsets in another buffer.
- `frag_movea_move_subi_addq_dbra_rts`: Short code fragment that copies an address to a0, then copies an address to a1, then copies 3811 to d0, then copies the memory at a0 (advancing a0) to d1, then subtracts 128 from d1, then copies d1 to the memory at a1 (advancing a1), then adds 1 to a0, then loops back while counting down d0, then returns.
- `unpack_lz_backreference_stream`: Decodes a packed stream by reading flag bits through the bit-buffer refill routine, then either copies a literal byte or copies a run backwards from output already written.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bit_buffer_by_count`: Shifts a longword bit buffer right repeatedly by a word-sized count, pulling in the next lower longword from memory whenever the buffer empties and feeding its low bit into the top.
- `nop_padding_sequence`: This routine consists of two no-operation instructions and serves as a gap in the executable code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `iterate_until_zero_or_negative`: Repeatedly calls another routine and resets a global variable to zero as long as the returned value is greater than or equal to one.
- `setup_dma_parameters`: Initializes DMA transfer parameters for graphics rendering by calculating and setting up register values based on input data, then configures multiple hardware registers including BLTCON0, BLTCON1, and various BLT* registers before initiating a loop of DMA operations.
- `frag_move_movem_rts`: Short code fragment that copies an address to the memory at a4, then restores saved registers from the stack, then returns.
- `call_external_function`: Executes another routine and returns.
- `wait_for_blitter_2`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `return_register_d0_2`: Returns the current value held in register d0.
- `frag_move_bra`: Short code fragment that copies an address to a fixed memory address, then copies an address to a fixed memory address, then jumps to another point in the code.
- `frag_movem_moveq_move_beq_bsr_bra`: Short code fragment that saves registers on the stack, then loads 0 into d0, then copies the memory at a4 (advancing a4) to d0, then branches if the last result was zero, then calls another routine, then jumps to another point in the code.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `process_input_character`: Processes input characters by comparing against specific values, performing arithmetic operations to calculate offsets, storing calculated memory addresses in global variables, and calling another routine to handle further processing.
- `frag_cmpi_bpl_beq_bne_addi_clr_rts`: Short code fragment that compares d0 with 48, then branches if the last result was not negative, then compares d0 with 32, then branches if the last result was zero, then compares d0 with 13, then branches if the last result was not zero, then adds 18 to a fixed memory address, then clears a fixed memory address, then returns.
- `frag_addi_bra`: Short code fragment that adds 43 to d0, then jumps to another point in the code.
- `initialize_game_flags`: Sets up game state flags and variables by writing specific values to memory locations, including clearing several byte flags, setting others to predefined constants, and storing pointer values in global variables.
- `update_game_state_3`: Updates a game state variable by incrementing a counter, copying data from a table based on input, and managing control flow through conditional branches.
- `frag_movea_subq_beq_move_bra`: Short code fragment that copies a fixed memory address to a0, then subtracts 1 from the memory at an offset from a0, then branches if the last result was zero, then copies the memory at a0 to a fixed memory address, then jumps to another point in the code.
- `frag_subq_bra`: Short code fragment that subtracts 4 from a fixed memory address, then jumps to another point in the code.
- `frag_addq_clr_move_movea_bra`: Short code fragment that adds 4 to a fixed memory address, then clears d0, then copies the memory at a0 to d0, then copies a fixed memory address to a0, then copies d0 to the memory at an offset from a0, then copies a fixed memory address to the memory at a0, then jumps to another point in the code.
- `update_game_state`: Updates a game state variable by incrementing a counter, copying data from a table based on input, and managing control flow through conditional branches and subroutine calls.
- `frag_clr_subq_beq_tst_cmpi_bcs_bra`: Short code fragment that clears a fixed memory address, then subtracts 1 from a fixed memory address, then branches if the last result was zero, then tests a fixed memory address, then branches if the last result was zero, then compares a fixed memory address with 2, then branches if the last result was zero, then branches if the carry flag is set, then jumps to another point in the code.
- `update_game_state_5`: Reads a byte from a memory location, performs conditional operations based on its value including comparisons against specific constants, updates a global variable, and branches to different code paths depending on the byte's value.
- `frag_movea_subq_beq_move_bra_2`: Short code fragment that copies a fixed memory address to a0, then subtracts 1 from the memory at an offset from a0, then branches if the last result was zero, then copies the memory at a0 to a fixed memory address, then jumps to another point in the code.
- `frag_subq_bra_2`: Short code fragment that subtracts 4 from a fixed memory address, then jumps to another point in the code.
- `frag_addq_clr_move_movea_bra_2`: Short code fragment that adds 4 to a fixed memory address, then clears d0, then copies the memory at a0 to d0, then copies a fixed memory address to a0, then copies d0 to the memory at an offset from a0, then copies a fixed memory address to the memory at a0, then jumps to another point in the code.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `update_game_state_4`: Updates a global variable based on input data from a pointer, performs conditional operations including table lookups and counter decrements, and handles state transitions through branching logic.
- `frag_move_clr_bra`: Short code fragment that copies 16 to a fixed memory address, then clears a fixed memory address, then jumps to another point in the code.
- `frag_move_clr_bra_2`: Short code fragment that copies 15 to a fixed memory address, then clears a fixed memory address, then jumps to another point in the code.
- `update_state_flags`: Clears a byte at a fixed memory address, decrements a counter at another fixed address, and branches based on the counter's value or comparison results.
- `process_game_data`: Reads a byte from a memory location, performs conditional operations based on the byte value, updates pointers and counters, and calls another routine before returning.
- `frag_movea_subq_beq_move_bra_3`: Short code fragment that copies a fixed memory address to a0, then subtracts 1 from the memory at an offset from a0, then branches if the last result was zero, then copies the memory at a0 to a fixed memory address, then jumps to another point in the code.
- `frag_subq_bra_3`: Short code fragment that subtracts 4 from a fixed memory address, then jumps to another point in the code.
- `frag_addq_clr_move_movea_bra_3`: Short code fragment that adds 4 to a fixed memory address, then clears d0, then copies the memory at a0 to d0, then copies a fixed memory address to a0, then copies d0 to the memory at an offset from a0, then copies a fixed memory address to the memory at a0, then jumps to another point in the code.
- `frag_move_bpl_eori_bra`: Short code fragment that copies the memory at a0 to d7, then branches if the last result was not negative, then copies a fixed memory address to d7, then exclusive-ors 12 into d7, then copies d7 to a fixed memory address, then jumps to another point in the code.
- `update_game_state_2`: Updates a global variable with data from a register, performs arithmetic operations on the variable using values from memory, and stores a word value at a calculated address.
- `frag_move_clr_bra_3`: Short code fragment that copies 15 to a fixed memory address, then clears a fixed memory address, then jumps to another point in the code.
- `frag_clr_movem_rts`: Short code fragment that clears a fixed memory address, then restores saved registers from the stack, then returns.
- `enable_interrupts_and_reset_flags_2`: Sets two flag globals to one, clears two counters, then writes a value with the top bits set to the interrupt-enable register.
- `configure_audio_dma`: Configures audio DMA channels by calculating register addresses based on input data, writing waveform parameters to audio registers, enabling interrupts and DMA, then updating a global interrupt flag with the calculated value.
- `frag_move_rts`: Short code fragment that copies 1 to a fixed memory address, then returns.
- `advance_multiplicative_seed_2`: multiplies the low 16 bits of a global variable by 58925 and stores the result back into that variable.
- `update_blt_parameters`: Updates the source and destination or length-related parameters for the bitblt (Blitter) hardware.
- `lookup_and_branch`: Performs a lookup operation using a byte value as an index into a table, loads a word from that location, and branches to a specific address based on the result of bitwise operations performed on the loaded value.
- `frag_clr_bra`: Short code fragment that clears d0, then jumps to another point in the code.
- `extract_low_nibble_2`: returns the low four bits of a 16-bit register value.
- `configure_video_registers`: Writes fixed 16-bit values to multiple hardware register addresses related to display and bitplane configuration, clears several registers, and calls three other routines while setting a specific bit in a DMA control register.
- `frag_movea_move_rts`: Short code fragment that copies a fixed memory address to a0, then copies a fixed memory address to a fixed memory address, then copies a0 to a fixed memory address, then returns.
- `update_audio_parameters`: Updates audio channel parameters by loading values from global memory locations, writing them to an audio register table with specific offsets, and incrementing a pointer through a loop of three iterations.
- `frag_bsr_dbra_rts`: Short code fragment that calls another routine, then loops back while counting down d0, then returns.
- `halt_in_infinite_loop_2`: enters an unconditional infinite loop so that execution never returns.
- `wait_and_reset_flag_2`: waits until a global variable reaches two or more, then clears it to zero and returns.
- `start_copper_list_2`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `frag_move_rts_2`: Short code fragment that copies 128 to a fixed memory address, then returns.
- `configure_interrupt_dma_2`: when a global variable is non-zero, it performs bitwise operations to calculate a mask and writes it to hardware registers before clearing the global variable.
- `handle_interrupt_request`: Processes interrupt requests by reading the current interrupt status, masking specific bits, setting up DMA and interrupt registers based on the masked value, and updating a global flag with an XOR operation of the masked bits.
- `frag_move_dbra_rts`: Short code fragment that copies 0 to the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `frag_lea_move_movea_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 799 to d1, then copies a0 to a1, then copies 6 to d0, then copies 0 to the memory at a1 (advancing a1), then loops back while counting down d0, then loads the address of the memory at an offset from a0 into a0, then loops back while counting down d1, then returns.
- `frag_move_bsr_dbra_rts`: Short code fragment that copies d0 to the stack, then calls another routine, then copies the top of the stack to d0, then loops back while counting down d0, then returns.
- `frag_move_add_cmpi_bcs_subi_subq_bra`: Short code fragment that copies 1 to d0, then adds the memory at a0 to d0, then copies d0 to the memory at a0, then compares d0 with 104, then branches if the carry flag is set, then subtracts 10 from d0, then copies d0 to the memory at a0, then subtracts 1 from a0, then jumps to another point in the code.
- `frag_move_rts_jsr_lea_dbra`: Short code fragment that copies 2 to a fixed memory address, then returns, then copies a2 to a fixed memory address, then copies a1 to a fixed memory address, then calls another routine, then loads the address of the memory at an offset from a2 into a2, then loads the address of the memory at an offset from a1 into a1, then loops back while counting down d0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_empty_operation`: This routine is empty and performs no actions before returning.
- `nop_padding`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_2`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_operation_2`: The routine is empty and performs no operations.
- `nop_padding_3`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_4`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_5`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_6`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `perform_no_operation`: This routine contains a single instruction that performs no action and returns immediately.
- `return_no_op`: The routine consists of two instructions that perform no work and immediately return to the caller.
- `nop_padding_7`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_8`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_9`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_10`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_11`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_12`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_13`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_14`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_15`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_16`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_17`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `frag_nop_rts_bra`: Short code fragment that pauses for one no-operation step, then returns, then jumps to another point in the code.
