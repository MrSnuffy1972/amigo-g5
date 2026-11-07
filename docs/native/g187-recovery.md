# Game 187 (InternationalKarate+ v1.5 2063): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00187). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (860527 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 19 raw-word lines |
| Functions named and described | pass, 115 of 115 named (0 author symbols), 115 of 115 described (6 by rule, 64 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1075 of 1075 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 362496 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2614 of 2938 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 23481 |
| code | 11514 |
| data | 825506 |
| padding | 26 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 80 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2045 | 83 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 186 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 45 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 206 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 4 |
| IK+.Image (raw code file, load address unknown) | 1 | 372 | 1 |

## Cases handled

- InternationalKarate+_v1.5_2063/InternationalKarate+_v1.5_2063.hdf: hard-disk image (AmigaDOS OFS, 896 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree InternationalKarate+_v1.5_2063_hdf, which is recovered file by file; no block outside the filesystem holds data
- runtime segment an address: 722 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 116 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 44 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 158 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- InternationalKarate+_v1.5_2063/InternationalKarate+_v1.5_2063_hdf/InternationalKarate+/IK+.Image: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 362496 bytes, recorded as UNVERIFIED DATA (under 1% of bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `process_joystick_and_copy_data`: The routine reads a value from a global variable, calls another routine via a pointer in a structure, checks a short result and branches, then toggles a bit in a byte derived from a pointer and a register, and copies a block of words from a table to another table using a loop.
- `conditional_callback_and_swap`: The routine calls a function pointer obtained from a global variable, and if the result is non-zero it calls another function pointer, calls another routine, and sets a RAM byte to one.
- `invoke_callbacks_and_return`: The routine calls another routine, calls a function pointer obtained from a global variable, calls another routine again, and returns a constant zero.
- `swap_buffer_bytes`: The routine calls several other routines, then swaps 50 bytes between a pointer passed in A0 and a fixed byte buffer by exchanging each byte in a loop.
- `swap_two_buffers`: The routine swaps the contents of two byte buffers by exchanging each byte for a fixed count, advancing both pointers each iteration.
- `check_input_and_set_ram`: The routine compares an input byte with a global variable, returns the input byte ORed with a constant if they differ, otherwise writes a constant to a RAM location and stores a computed address in another RAM location before returning the input byte.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_struct_table_entries`: The routine copies selected words from a global structure table to a fixed memory table.
- `clear_state_and_loop`: The routine iterates through a range of offsets, clearing specific memory locations by writing zero bytes.
- `decode_and_clear_table`: The routine builds a byte value from a bitstream stored in memory and then uses it to index a table, clearing a set of bytes in a global structure and writing a byte from the table into the structure.
- `process_input_byte_and_update_state`: The routine reads a byte from a pointer and, based on the value of a global variable and several RAM bytes, updates a set of global variables and RAM bytes, toggles flags, and calls another routine.
- `check_input_flags_and_reset`: The routine tests a byte from a global variable and two bytes from a table and, when the condition holds, writes constants to three bytes in a table, clears a word in RAM, and writes constants to a global variable before returning.
- `store_long_from_pointer`: The routine reads four consecutive bytes from the memory pointed to by a1 and combines them into a 32-bit value, then writes that value to a global variable.
- `call_self_three_times`: The routine calls itself three times in succession and then returns.
- `set_global_and_call_routine`: The routine stores a constant value in a global variable and then invokes another routine four times in succession before returning.
- `update_timer_state`: Checks a global variable against a specific value and updates it using a lookup table or sets it to a default.
- `set_global_and_recurse`: The routine writes the constant 6 to a global variable, then conditionally overwrites that variable with a byte from a small table indexed by bits of the variable, and finally calls another routine twice before returning.
- `call_self_four_times`: The routine calls itself four times in succession and then returns.
- `draw_four_bobs`: Draws four objects in a row by calling the masked bob blitter routine four times with the prepared registers.
- `decrement_table_bytes`: The routine iterates a short index from 1 down to -1, and for each index it checks two bytes from a table at offsets from the index.
- `loop_decrement_and_branch`: The routine repeatedly calls another routine and then enters a loop that decrements a register and checks two bytes from a table; if either byte is zero it continues, otherwise it decrements a byte in a table and conditionally calls another routine or a drawing routine before looping again.
- `call_self_twice`: The routine calls itself twice in succession and then returns.
- `draw_flash_pair_b`: Draws the same two-part object with the second image pair, the other half of the flashing effect.
- `update_global_from_table`: The routine reads a global variable, masks it, uses the result as an index into a table to load a byte into a global variable, and conditionally overwrites that global variable with bytes from two other table entries.
- `initialize_globals_and_loop`: The routine writes constant values to two global variables, then repeatedly calls a library routine and another routine while incrementing one of the globals in a loop whose condition is always false.
- `set_global_and_loop`: The routine stores a byte from a register into a global variable, then repeatedly calls a library routine and another routine while incrementing the global variable in a loop whose condition is always false.
- `clear_memory_and_initialize_globals`: The routine clears a block of memory by writing zero bytes to consecutive locations, then writes constant values to several global variables and to a set of four global pointers.
- `forward_call`: The routine calls another routine and returns its result unchanged.
- `compute_obfuscation_checksum`: The routine increments a global variable by several constants, extracts its high byte, and returns a value derived from the variable combined with bytes read from another routine using the variable as an index.
- `write_sprite_mask_tile`: The routine writes masked bytes from a global variable table into two parallel framebuffer areas for a 7-row tile.
- `update_global_and_call_routine`: The routine reads two bytes from a table indexed by a global variable, updates a counter byte in that table and clamps it, then calls another routine six times while toggling the global variable between the two table bytes based on the counter.
- `update_bcd_counters_and_recurse`: The routine checks a global variable and, if it is zero, reads a pointer from it, then updates two BCD-encoded bytes through indirect pointers derived from that pointer and from a byte at an offset from it, applying BCD adjustment to each.
- `save_and_restore_global_variable`: The routine saves the current value of a global variable, clears it, calls another routine, then restores the original value from the saved copies.
- `decode_global_byte_table`: The routine checks a global variable and, if it is non-zero, it clears it and calls another routine.
- `draw_masked_pair`: Draws a two-part object with two calls of the masked plotting routine.
- `draw_indicator_bob`: Draws one indicator bob at a position taken from the current player's table entry.
- `initialize_sprite_tables`: The routine clears a range of bytes in a table and initializes small byte tables, then computes per-sprite values from input bytes and writes derived limits and offsets into tables.
- `normalize_global_byte`: The routine reads a global variable and applies a small conditional transformation to it.
- `tiny_not_swap`: Small routine of 2 instructions that inverts d7, then swaps the halves of d7.
- `restore_global_variable_and_call`: The routine saves three bytes from a global variable, modifies the variable with a character constant and a conditional table lookup, clears it, computes a new value, calls another routine, restores the saved bytes in reverse order, and calls another routine again.
- `decrement_flag_loop`: The routine sets a global variable to seven and then repeatedly decrements it while calling another routine.
- `draw_panel_into_both_buffers`: Runs the drawing routine that follows into both display buffers in turn and restores the current-buffer pointer.
- `decode_script_bytes`: The routine walks a byte stream using an index stored in a global variable, reads bytes from a table pointed to by another global, and dispatches on the byte value.
- `decode_script_byte_sequence`: Processes a byte value by subtracting an offset and applying masks to a table.
- `process_global_byte_table`: Checks if a global variable is within a valid positive range.
- `return_global_variable`: The routine loads a global variable and returns its value.
- `apply_byte_mask_to_table`: The routine iterates over a block of memory using a pointer and a counter.
- `copy_sprite_attribute_blocks_3`: Copies a fixed number of 40-word attribute blocks from a source table to a destination buffer built from a global variable base.
- `copy_sprite_attribute_blocks`: The routine copies a fixed number of 40-word attribute blocks from a source table to a destination buffer built from a global variable base.
- `draw_fighter_frame`: Draws the current fighter's animation frame into the drawing buffer: picks the frame from one of two frame tables (selected by a fighter flag), positions it from the fighter's coordinates and copies its runs of image words into the bitplanes.
- `apply_sprite_mask_to_tiles`: The routine reads a global variable and uses it to index a table to obtain a pointer to sprite data.
- `store_register_and_call`: The routine stores the value from an address register into a RAM word and then calls another routine before returning.
- `tiny_ori_2`: Small routine of 1 instruction that merges 32 into d6.
- `call_stub_routine`: A one-instruction stub that calls another routine four times in succession.
- `call_stub_routine_2`: A one-instruction stub that calls another routine four times in succession.
- `recursive_self_call`: The routine contains no instructions and performs no operations; it is empty.
- `call_stub_routine_3`: A one-instruction stub that calls another routine four times in succession.
- `four_steps_per_frame`: Calls the following step routine four times in a row, giving four steps in one frame.
- `clear_memory_words`: The routine clears a block of memory by repeatedly zeroing ten consecutive 32-bit words per iteration.
- `conditional_stub_call`: The routine tests a global variable and, if non-zero, calls another routine before returning.
- `duplicate_word_table_entries`: The routine copies a 16-bit word from a source table to a destination table and writes the same value to three additional offsets within each destination entry.
- `initialize_sprite_structure_2`: Writes a byte from a register into two byte fields of a structure pointed to by a register, copies a byte from a global table into the structure and clears several fields of the structure to zero or one.
- `initialize_sprite_structure`: The routine writes a byte from a register into two byte fields of a structure pointed to by a register, copies a byte from a global table indexed by that register into two byte fields of the same structure, and then clears several byte fields of the structure and sets one byte field to one.
- `select_sprite_frame_2`: The routine reads fields from a structure pointed to by a register and uses a global variable as an index into a table to select a sprite frame.
- `select_sprite_frame`: The routine selects a sprite frame index from a global table using fields in a structure pointed to by a register, updates several bytes in that structure with values derived from the table entry and from input registers, and recurses via another routine to resolve the frame.
- `clear_flags_and_recurse`: The routine checks two bytes in a structure pointed to by a register and, if either condition holds, clears two bytes in the same structure and then calls another routine, returning its result.
- `process_sprite_animation_frame`: The routine updates fields in a structure pointed to by a register using values from a global table indexed by a word built from the input register and bytes from the structure, and it calls another routine to update sprite animation state and to process a sprite state table.
- `update_sprite_animation_state`: The routine updates fields in a structure pointed to by a register using values from a global table indexed by a word built from the input registers and bytes in the structure.
- `update_player_animation_state`: The routine updates fields in a structure pointed to by D2 using values from a global table indexed by a word built from D1 and a byte in the structure.
- `increment_counter_conditionally`: The routine extracts a byte from a word argument, reads a flag byte from a structure, and conditionally calls another routine to obtain a return value.
- `process_input_state_flags`: The routine examines bytes in a structure pointed to by D2 and, when certain flag bits are set, updates fields in the same structure using values from a global variable table and another routine.
- `update_input_state_flags`: The routine updates a set of input-related bytes in a structure pointed to by D2, using values from a global variable table and another routine, and returns a value from that routine.
- `toggle_state_flags`: The routine tests bits in a byte at an offset from a pointer and toggles or sets bytes at other offsets in the same structure, stores a small value into a global variable under a condition, calls another routine, and increments a byte field by one.
- `process_sprite_state_table`: The routine uses a global table indexed by bytes from a structure pointed to by a register and input values to select a byte, stores it into a byte field of the structure and updates related byte fields.
- `update_sprite_byte_field`: The routine stores a byte from a global table into a byte field of a structure pointed to by a register, using a byte from the structure and a byte from D1 as an index, then calls another routine.
- `adjust_sprite_position`: Reads bytes from a structure pointed to by a register, indexes a global table with those bytes and updates position bytes in the structure with the computed values, clamping the result to a range.
- `adjust_sprite_byte_field`: The routine updates a byte field in a structure pointed to by a register using a global table and a byte from the structure, then clamps the result.
- `update_sprite_byte`: The routine updates a byte field in a structure pointed to by a register using a global table and a byte from the structure, then clamps the result to a maximum value.
- `compute_and_clamp_byte`: The routine computes a short value from a table indexed by an input and a global variable, subtracts a constant, clamps it to zero, stores it as a byte in a structure, and clamps that byte to a maximum value.
- `conditional_assign_and_clamp`: The routine checks a global variable against a threshold and, depending on a flag byte in a structure, writes either a computed negative offset or a constant into a byte field, then clamps that byte to a maximum value.
- `set_sprite_flag`: Stores a constant byte into a field of a structure pointed to by a register and then clamps that byte to a maximum value.
- `choose_background_colours`: Chooses the two background colour values for the frame: fixed colours in phase 6, otherwise per-player colours from a table when that player is active, with defaults when not.
- `compare_d7_to_table`: The routine loads a short zero into a loop counter and repeatedly compares an input short from D7 with successive entries of a global short table, advancing the counter by two each iteration.
- `reset_game_state`: The routine clears four global variables to zero and then returns.
- `draw_into_both_buffers`: Runs the screen drawing routine once for each of the two display buffers by pointing the current-buffer variable at each in turn, then restores it.
- `copy_sprite_attribute_blocks_2`: The routine copies a block of 40 longwords from a source table to a destination buffer derived from a global variable, repeating the copy for 83 iterations.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_sprite_attribute_table_2`: Calls a routine that writes zeros to ten consecutive 32-bit words and advances the pointer by a fixed stride, repeating the clearing eight times.
- `clear_sprite_attribute_table`: It writes zeros to ten consecutive 32-bit words and then advances the pointer by a fixed stride, repeating the clearing eight times.
- `compute_sprite_offset`: Returns zero when a global variable equals a specific value, otherwise returns the low byte of an input word multiplied by a constant.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `music_frame_all_channels`: Music player frame: runs the channel sequencer for each of the four audio channels.
- `enable_dma_and_copy_structs`: The routine contains 97 instructions, is called by copy_struct_table_entries, copy_and_update_pointers and five unnamed routines, calls copy_and_update_pointers, and writes to the DMACON hardware register with bit 15 set to enable DMA channels.
- `copy_and_update_pointers`: The routine copies words and longs between fields of a structure pointed to by a register, decrements a word field, advances a pointer field, clears a byte in another structure, and stores the address of a global variable into a pointer field.
- `return_or_masked_byte`: The routine returns a byte formed by OR-ing the low byte of D0 with 254.
- `touch_ciaa_pra_register`: The routine contains two instructions and touches the CIAA_PRA hardware register.
- `call_enable_dma_routine`: The routine contains twelve instructions and is entered from outside static flow during a reference run.
- `call_dma_enable_routines`: The routine contains six instructions, is entered from outside the static flow, and calls enable_dma_and_copy_structs and call_enable_dma_routine_3.
- `call_enable_dma_routine_2`: The routine contains two instructions and is entered from outside static flow during a reference run.
- `call_enable_dma_routine_3`: The routine contains nine instructions and is unreached as a gap decode.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_globals_and_table`: The routine writes zero to a series of global variables, calls another routine, then clears a 16-entry table of 32-bit words at a fixed address by storing zero in each entry.
- `title_logo_frame`: One frame of the animated title sequence: updates the logo objects, the second effect on even frames only, and the remaining title elements in a fixed order.
- `initialize_title_arrays`: The routine loops 21 times to copy bytes from a global variable table into a byte array, writes 31 to a byte array, and clears two word arrays, then writes 32 to two global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_sprite_tile_offset`: Computes a memory address by scaling an index from a register and adding it to a base pointer, while validating bounds against a global variable.
- `process_sprite_tiles`: The routine stores a pointer and a counter into a RAM area, then loops to read mask data, rotates words from a source table by a variable amount, combines them with a mask, and writes the results into a destination buffer using bitwise operations.
- `process_sprite_masks`: The routine iterates over a set of mask words, builds four rotated masks from four source words per block, combines them with a control word and writes the resulting mask to an optional output table.
- `conditional_input_dispatch`: The routine checks a masked value from D2 against a threshold, computes a masked and shifted value from D1, performs range tests, and conditionally calls another routine before returning a concatenation of D0 and D1.
- `check_input_range_and_dispatch`: The routine checks a masked value from a register against a threshold and, if the condition holds, masks another register value and tests it against several ranges.
- `apply_rotated_bitmask`: Iterates through a series of data structures, rotating a combined mask value based on a shift amount.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine`: The routine contains no instructions and returns immediately.
- `empty_routine_2`: The routine contains no instructions and returns immediately.
- `empty_routine_3`: The routine contains no instructions and returns immediately.
- `empty_routine_stub`: The routine contains no instructions and returns immediately.

### IK+.Image (raw code file, load address unknown)

- `unnamed_routine_entry`: The routine contains three hundred seventy-two instructions and serves as the entry point for a raw code region.
