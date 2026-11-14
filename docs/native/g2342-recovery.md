# Game 2342 (Blades of Steel): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02342). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916432 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 35 raw-word lines |
| Functions named and described | pass, 99 of 99 named (0 author symbols), 99 of 99 described (3 by rule, 12 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 321 of 321 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2173 of 2173 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 22966 |
| code | 14306 |
| data | 846392 |
| unused | 32768 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Blades of Steel.adf:bootblock | 1 | 31 | 1 |
| logged disk load from Blades of Steel.adf boot-block load | 1 | 85 | 2 |
| logged disk load from Blades of Steel.adf boot-block load | 1 | 0 | 0 |
| logged disk load from Blades of Steel.adf boot-block load | 1 | 424 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1633 | 75 |

## Cases handled

- Blades of Steel/Blades of Steel.adf: custom-format (non-DOS) disk, 1677 non-empty blocks: matched against track-loaded segments
- Blades of Steel/Blades of Steel.adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- Blades of Steel/Blades of Steel.adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- Blades of Steel/Blades of Steel.adf: the boot block reads 4096 bytes from disk offset 2048 to an address (found statically); recovered as a unit
- runtime segment an address: 321 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Blades of Steel/Blades of Steel.adf: 3 logged loads cover 16384 bytes of the image
- Blades of Steel/Blades of Steel.adf: 29 track-loaded blocks found in recovered runtime segments and classified through them
- Blades of Steel/Blades of Steel.adf: blocks 22-111 (bytes 11264-57344) outside the filesystem hold data, not code (opcode density 0.047, entropy 6.48)
- Blades of Steel/Blades of Steel.adf: blocks 141-158 (bytes 72192-81408) outside the filesystem hold data, not code (opcode density 0.036, entropy 5.38)
- Blades of Steel/Blades of Steel.adf: blocks 160-180 (bytes 81920-92672) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.54)
- Blades of Steel/Blades of Steel.adf: blocks 182-219 (bytes 93184-112640) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.26)
- Blades of Steel/Blades of Steel.adf: blocks 221-235 (bytes 113152-120832) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.49)
- Blades of Steel/Blades of Steel.adf: blocks 237-266 (bytes 121344-136704) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.81)
- Blades of Steel/Blades of Steel.adf: blocks 268-282 (bytes 137216-144896) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.58)
- Blades of Steel/Blades of Steel.adf: blocks 284-297 (bytes 145408-152576) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.84)
- Blades of Steel/Blades of Steel.adf: blocks 299-369 (bytes 153088-189440) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.32)
- Blades of Steel/Blades of Steel.adf: blocks 372-383 (bytes 190464-196608) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.71)
- Blades of Steel/Blades of Steel.adf: blocks 387-482 (bytes 198144-247296) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.32)
- Blades of Steel/Blades of Steel.adf: blocks 485-919 (bytes 248320-471040) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.74)
- Blades of Steel/Blades of Steel.adf: blocks 922-934 (bytes 472064-478720) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.58)
- Blades of Steel/Blades of Steel.adf: blocks 937-948 (bytes 479744-485888) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.40)
- Blades of Steel/Blades of Steel.adf: blocks 952-964 (bytes 487424-494080) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.05)
- Blades of Steel/Blades of Steel.adf: blocks 967-979 (bytes 495104-501760) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.14)
- Blades of Steel/Blades of Steel.adf: blocks 982-1103 (bytes 502784-565248) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.30)
- Blades of Steel/Blades of Steel.adf: blocks 1105-1119 (bytes 565760-573440) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.27)
- Blades of Steel/Blades of Steel.adf: blocks 1121-1139 (bytes 573952-583680) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.46)
- Blades of Steel/Blades of Steel.adf: blocks 1149-1168 (bytes 588288-598528) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.04)
- Blades of Steel/Blades of Steel.adf: blocks 1178-1183 (bytes 603136-606208) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.06)
- Blades of Steel/Blades of Steel.adf: blocks 1193-1737 (bytes 610816-889856) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.00)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `step_disk_head_to_track` (Blades of Steel.adf boot-block load +an address to an address (4096 bytes)): Moves a current track counter toward a target one step at a time. Kind: Disk hardware timing / head-position check (CIAB PRB). Pass path: Loop exits when in_D0w == _DAT_000006ee (i.e., the expected number of head-movement steps has completed); the routine then returns normally..
- `pulse_drive_select_bit` (Blades of Steel.adf boot-block load +an address to an address (4096 bytes)): Sets one bit of a CIA B port chosen from the low two bits of its input, holds it through a fixed delay loop, clears it, then calls a routine and clears a state word. Kind: CIAB PRB port strobe / drive-select timing check. Pass path: The routine toggles a CIAB port-B line (DAT_00bfd100 = CIAB_PRB) as a strobe, then calls u3__sub_03D0 (which reads the drive-select response on the same port). A genuine 68000 disk controller latches back onto the bus during the strobe, so .
- `manipulate_cia_registers` (Blades of Steel.adf boot-block load +an address to an address (4096 bytes)): Performs bitwise operations on CIAB_PRB based on a value derived from D0, loops 256 times to clear a bit, and conditionally modifies CIAA_PRA and CIAB_PRB before calling another routine. Kind: disk hardware/timing check. Pass path: bmi.b an address.

## Functions


### Blades of Steel.adf:bootblock

- `read_disk_blocks_copy_loader`: Fills in disk I/O request blocks and calls the I/O routine to read blocks from disk into memory.

### logged disk load from Blades of Steel.adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Blades of Steel.adf boot-block load

- `select_drive_and_read_track`: Selects the disk drive through a CIA port, sets the disk DMA pointer and length, and calls positioning routines.
- `copy_words_from_offset`: Copies a run of 16-bit words from a source address offset by a global value into a destination pointer, for a count given in a data register.
- `decode_mfm_sector_bits`: Decodes MFM-encoded data over ten passes.
- `skip_mfm_sync_words`: Searches forward for the MFM sync word and then skips any consecutive sync words, leaving the pointer at the first non-sync data.
- `step_disk_head_to_track`: Moves a current track counter toward a target one step at a time.
- `wait_for_interrupt_request_bit`: Busy-waits until one bit of a status word is set, then stores a fixed acknowledge value.
- `wait_for_cia_status_clear`: Runs a fixed delay loop, then busy-waits until one status bit of the fire-button port clears.
- `repeat_until_status_bit_clears`: Repeats a routine while a status bit remains set, setting the low control bits before each call, then clears the status word.
- `wait_for_cia_interrupt_flag`: Busy-waits until the CIA B interrupt control register reports its flag bit as set.
- `pulse_drive_select_bit`: Sets one bit of a CIA B port chosen from the low two bits of its input, holds it through a fixed delay loop, clears it, then calls a routine and clears a state word.
- `pulse_drive_control_bit`: Sets bit seven and one bit selected by the input's low two bits in a control word, clears the selected bit after a delay loop, and sets the word to all ones.
- `manipulate_cia_registers`: Performs bitwise operations on CIAB_PRB based on a value derived from D0, loops 256 times to clear a bit, and conditionally modifies CIAA_PRA and CIAB_PRB before calling another routine.
- `wait_for_vposr`: Loops until the value in the video beam position register equals 160, decrementing a counter for a total of 400 iterations before returning.
- `manage_disk_operations`: Checks a bit in CIAA_PRA and enters a loop that calls several routines to pulse drive bits, wait for status, select a drive, read a track, step the head, and pulse a control bit.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `search_table_for_value`: Calls another routine, loads a value from a buffer, and enters a loop to compare this value against entries in a table at an offset from a pointer in a1, potentially calling another routine if a match is found.
- `increment_and_check_global`: Loads a longword from a table using an offset, increments a register, and if a global variable is negative, calls another routine before returning the incremented value.
- `read_and_update_global`: Calculates a value based on a pointer at offset 1532, calls a routine to read a track, and updates a global variable if the new value is larger than the current one.
- `read_tracks_in_loop`: Calls another routine to select a drive and read a track, then enters a loop that calls another routine to copy words from an offset until a counter reaches zero.
- `pulse_drive_control`: Clears register d0 and calls another routine before returning.
- `load_long_word`: Loads a long word from a global variable into register d0 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `program_blitter_from_block`: Waits until the blitter is idle, then copies a block of parameters into the blitter registers for source, destination, modulo, control and size.
- `setup_blt_registers`: Copies various values from a structure pointed to by a2 into the hardware bitplane and block transfer registers.
- `poll_until_high_word_clears`: Writes a fixed value to a control field, then repeatedly calls a routine and stores its low word until the high word of the returned value is zero.
- `forward_call_to_routine`: Calls one routine and returns, adding no logic of its own.
- `copy_table_entry_double_buffers`: Looks up a three-word entry from a table indexed by a shifted value, optionally replaces the last word with all ones, and writes the three words into two parallel buffers at an offset taken from a stored position.
- `advance_frame_counter_and_call`: Calls two routines, increments a 16-bit counter in the object, then calls a third routine.
- `write_longword_pairs_to_buffers`: Writes a pair of 32-bit values, as high and low words, into two parallel buffers, then writes a second pair eight and twelve bytes further on.
- `init_sprite_position_from_table`: Looks up a base value in a table indexed by a short entry, adds a second value from the same entry and stores the sum.
- `setup_paired_buffer_objects`: Initializes an object's fields with a size and an offset, computes two buffer addresses at fixed distances, and calls a routine twice to program them.
- `clamp_offset_adjusted_value`: Subtracts a stored offset from each of two values when the matching bit in a flag word is set.
- `write_sprite_control_bytes`: Computes a start value from an input and writes it and derived bytes into two control buffers.
- `setup_graphics_operation_fields`: Initializes a block of fields for a graphics operation, calls routines to compute an address and a size, and combines a shifted value with a constant into a size field before setting a mode field.
- `queue_optional_graphics_operation`: When one status bit is set, copies a stored address into the object, sets a size field, optionally offsets the address by a fixed amount and calls a routine to finish the operation.
- `conditional_finish_graphics_operation`: If one bit of a status byte is set, copies the stored address into the object, calls a routine and clears the mode field.
- `setup_and_run_blitter`: Calculates a destination address using values from a4, populates a structure at offset -26982 of a4 with blitter parameters, calls a routine to perform the transfer, and waits for a DMA bit to clear.
- `setup_and_run_blitter_alt`: Calculates a destination address using values from a4, populates a structure at offset -26982 of a4 with different blitter parameters, calls a routine to perform the transfer, and waits for a DMA bit to clear.
- `initialize_graphics_buffers`: If a value at a specific offset in a4 is non-negative, it sets a field in a structure, calls another routine to setup graphics fields, calls another routine to initialize sprite positions, and calls another routine to setup paired buffer objects.
- `write_longword_into_double_buffers`: Reads a 32-bit value through a pointer table and writes its high and low words into two parallel buffers at an offset computed from a stored position.
- `configure_cia_timer_and_interrupts`: Writes a sequence of constants to the CIA A timer and interrupt control registers and then to the interrupt enable register, with one call.
- `advance_timer_and_acknowledge_interrupt`: When a status bit is set, advances a counter, and when a timer reaches its limit it resets state fields and calls routines to restart the sequence.
- `return_stack_value`: It returns the 64-bit value that was passed to it on the stack without modifying anything.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `swap_buffer_pointer_sets`: Swaps several pointer and longword fields between two buffer sets, clears a counter and copies a selected pointer into a global value.
- `compute_position_deltas_and_copy`: Computes the differences between current and previous coordinate fields, copies several fields into their companion fields, and clears a counter.
- `fill_column_cells_with_ff`: Computes a table address from a pair of 16-bit inputs, calls one routine, then writes all ones to eight bytes spaced 44 bytes apart in the table.
- `fill_cells_in_nested_loop`: Clears two values in a4, then executes nested loops that call another routine to fill column cells, using values from a stack-based structure to control the iteration counts.
- `build_blitter_block_list`: Constructs a series of Blitter block transfer commands by populates a list with source and destination addresses, widths, and heights, then updates the copper list and enables DMA channels.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `update_cell_state_and_position_2`: Initializes several fields in a structure pointed to by a4, updates a bitmask derived from d2, and modifies a status word based on bitwise operations.
- `update_cell_bit_two`: Tests bit 2 of register d2; if set, it writes the word in d0 to an offset of minus 20762 from the pointer in a4 and branches to another routine, otherwise it branches to another routine.
- `update_cell_status`: Tests a word at offset 20508 in a structure pointed to by a4; if non-zero, it branches to another routine, otherwise it writes d0 to offset 20762 of the structure, modifies d2 by masking bit 0 and 1 and setting bit 1, writes 0 and 1 to offsets 0 and 2 of a structure in a1, decrements a 16-bit value at offset 20748 in the structure in a4, and branches to another routine based on the result.
- `update_cell_mask`: Tests bit 2 of register d4; if set, it performs a bitwise AND on register d2 with 3, writes the result to an offset of minus 20758 from the pointer in a4, and branches to another routine, otherwise it branches to another routine.
- `update_cell_bit_three`: Tests bit 3 of register d2; if set, it writes the word in d0 to an offset of minus 20762 from the pointer in a4 and branches to another routine, otherwise it branches to another routine.
- `update_cell_state_and_position`: Checks a status word in a structure pointed to by a4; if zero, it initializes several fields including a bitmask derived from d2, writes values to a table via a1, and calls reset_cell_loop_then_mark.
- `modify_status_word`: Performs bitwise operations on d2, moves a word from one offset of a4 to another, and updates several other offsets in a4 based on the result.
- `update_cell_data`: Checks bits in a value at an offset from a4, clears a word at an offset, and if a specific bit is set, writes two words to a table at a1 and modifies a value at an offset from a4.
- `update_cell_values`: If bit 3 of d0 is set, writes four words to a table at a1 and calls another routine; otherwise, it modifies a value in d2 based on bits in a global variable and updates a global variable.
- `update_offset_values`: Checks a bit in a global variable and, if set, modifies a value in d2 by adding or subtracting 16, then stores the result in a global variable.
- `update_position_with_bounds`: Adjusts an x and a y position by direction flags, clamps each to its range, stores a derived offset from the x value, and calls a routine to finish.
- `reset_cell_loop_then_mark`: Clears a pending flag if set, computes a table index from position values, and calls two routines repeatedly until one returns zero.
- `program_blitter_three_rows`: Loops three times, waiting for the blitter to be idle each time, and writes the blitter control, mask, modulo, pointer and size registers.
- `program_blitter_three_rows_swapped`: Loops three times, waiting for the blitter to be idle each time, and writes the blitter control, mask, modulo, pointer and size registers.
- `copy_32_byte_cell_planes`: Copies a 32-byte entry from a table selected by a shifted index into a destination block, writing each byte at offsets that step by a fixed row spacing and by a second plane offset.
- `copy_32_bytes_strided_destination`: Copies 32 bytes from an input pointer into a destination block, writing each byte at offsets that step by a fixed row spacing and by a second plane offset.
- `write_four_positions_to_lists`: Writes four 32-bit positions into each of two output lists, each position being a base offset plus a stored value adjusted by a small amount.
- `wait_for_flag_then_clear`: Busy-waits until a state flag becomes nonzero, then clears it.
- `program_blitter_from_table_entry`: Loads a value from a table selected by its input, waits until the blitter is idle, then writes a set of blitter registers with constants and offsets.
- `copy_and_process_data`: Copies bytes from a source table to a destination table using a loop, then performs a second loop that copies words from a table to a destination while calculating a value based on division and multiplication.
- `copy_32_words_three_destinations`: Copies 32 words from an input block into three destinations: two state structures and a fixed table in the object.
- `clear_word_array`: Fills an array of words with zero using a loop that runs 29384 times.
- `load_and_copy_structure`: Decrements a global counter and, if it reaches zero, calls another routine, then calculates an offset to copy three words from a table into a local structure and calls copy_32_words_three_destinations.
- `write_calculated_positions`: Copies several values from the stack to global variables, calculates a new value by shifting and masking a global word, and calls write_four_positions_to_lists.
- `set_word_to_all_ones`: Writes the word 65535 to an offset of minus 20962 from the pointer in a4 and returns.
- `clear_flag_word`: Sets one 16-bit state word to zero and returns.
- `reset_state_fields_to_defaults`: Sets four longwords to all ones, clears several 16-bit fields in two tables and a state flag, and sets a counter to one.
- `read_timer_values`: Reads bytes from CIAA and CIAB timer registers to construct two 32-bit values, which are then stored in global variables.
- `process_keyboard_input`: Reads a byte from the keyboard serial register, performs bitwise operations to transform it, stores the result in a global variable, and toggles a bit in a CIAA control register.
- `test_key_and_set_flag`: Compares a word at an offset from a4 to a constant and checks a byte at an offset from a4, and if a specific byte in d0 matches a constant, stores a byte at an offset from a4.
- `check_key_and_reset_flag`: Compares a byte in d0 to 107; if it does not match, it branches to another routine, otherwise it writes a byte of all ones to an offset of minus 20573 from the pointer in a4 and branches to another routine.
- `clear_byte_and_branch`: Writes a zero byte to an offset of minus 20637 from the pointer in a4 and branches to another routine.
- `fetch_and_compare_byte`: Checks the sign of d0, and if positive, loads a byte from a table at an offset from a4 using d0 as an index, then compares it to a constant and branches.
- `toggle_key_flags`: Compares d0 against three specific byte values and, if a match is found, inverts the corresponding byte in a global variable.
- `save_registers_and_branch`: Pushes registers d0 through d7 and a0 through a6 onto the stack, loads a pointer from an offset of 64 from the stack pointer into a0, and branches to another routine.
- `copy_word_table`: Calls another routine, and if a byte at an offset from a4 matches itself, enters a loop that copies 32 words from one location to another using a4 as a base.
- `save_registers_and_branch_short`: Pushes registers d0 through d7 and a0 through a6 onto the stack and branches to another routine.
- `reset_and_copy_table`: Checks if a word at an offset from a4 is non-negative, and if so, sets it to all ones, calls another routine, and enters a loop that runs 8 times.
- `calculate_and_copy_words`: Runs a loop 31 times to calculate three scaled values from a word in a0 using another routine, stores them in a table at offset -20568 of a4, and then calls a routine to copy 32 words from that table.
- `calculate_scaled_value`: Multiplies d1 by d7, divides the result by 15, and then combines this with d2 shifted left by 4 bits.
- `copy_words_loop`: Copies 31 words from a source pointed to by a pointer at offset 64 of the stack to a destination at offset -20680 of a4 using a loop.
- `call_setup_sequence_routines`: Calls a routine with an argument computed from its input, then calls two more routines.
- `fill_column_cells`: Iterates through a table at a2, and if a byte is neither zero nor 32, calls another routine with parameters pushed from the stack.
- `initialize_setup_sequences`: Calls call_setup_sequence_routines and build_rows_from_two_lists after performing data initialization.
- `copy_16_words_then_call`: Copies sixteen 16-bit words from an input block into a fixed table in the object, then calls one routine on that table.
- `build_rows_from_two_lists`: When a first counter is nonzero, initializes a set of position fields and calls a routine in a loop, advancing a stored address by a fixed stride each pass until the routine returns zero.
- `place_flagged_cells_into_lists`: Loops over an input list of entries and, for each entry whose flag is negative, computes a position from table values and clamps a vertical value to a window.
