# Game 1735 (vroom (1991)(lankhor)[cr vf - company][t +21 atx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01735). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911002 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 21 raw-word lines |
| Functions named and described | pass, 81 of 81 named (0 author symbols), 81 of 81 described (4 by rule, 43 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1693 of 1696 (99.82%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2624 of 2628 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 42206 |
| code | 10464 |
| data | 597714 |
| padding | 10 |
| unused | 260608 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Vroom (1991)(Lankhor)[cr VF - Company][t +21 ATX].adf:bootblock | 1 | 23 | 1 |
| logged disk load from Vroom (1991)(Lankhor)[cr VF - Company][t +21 ATX].adf boot-block load | 1 | 214 | 5 |
| logged disk load from Vroom (1991)(Lankhor)[cr VF - Company][t +21 ATX].adf boot-block load | 1 | 232 | 21 |
| logged disk load from Vroom (1991)(Lankhor)[cr VF - Company][t +21 ATX].adf boot-block load | 1 | 163 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1293 | 26 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 505 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 364 | 7 |

## Cases handled

- vroom (1991)(lankhor)[cr vf - company][t +21 atx]/Vroom (1991)(Lankhor)[cr VF - Company][t +21 ATX].adf: custom-format (non-DOS) disk, 1248 non-empty blocks: matched against track-loaded segments
- vroom (1991)(lankhor)[cr vf - company][t +21 atx]/Vroom (1991)(Lankhor)[cr VF - Company][t +21 ATX].adf: the boot block reads 20480 bytes from disk offset 584192 to an address (found statically); recovered as a unit
- vroom (1991)(lankhor)[cr vf - company][t +21 atx]/Vroom (1991)(Lankhor)[cr VF - Company][t +21 ATX].adf: the boot block reads 7680 bytes from disk offset 576512 to an address (found statically); recovered as a unit
- vroom (1991)(lankhor)[cr vf - company][t +21 atx]/Vroom (1991)(Lankhor)[cr VF - Company][t +21 ATX].adf: the boot block reads 16384 bytes from disk offset 856064 to an address (found statically); recovered as a unit
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 676 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- RAM snapshot an address: 3 executed PCs whose RAM was overwritten before any snapshot match its bytes 252784-252848: recovered as a unit run at an address
- RAM snapshot an address: 3 executed PCs whose RAM was overwritten before any snapshot match its bytes 254864-254944: recovered as a unit run at an address
- 818 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 2 first-run image unit(s) in 1 version group(s); 594 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- vroom (1991)(lankhor)[cr vf - company][t +21 atx]/Vroom (1991)(Lankhor)[cr VF - Company][t +21 ATX].adf: 3 logged loads cover 44544 bytes of the image
- vroom (1991)(lankhor)[cr vf - company][t +21 atx]/Vroom (1991)(Lankhor)[cr VF - Company][t +21 ATX].adf: blocks 1584-1671 (bytes 811008-856064) outside the filesystem hold data, not code (opcode density 0.015, entropy 7.09)
- 2 packed track region(s), 549888 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `copy_data_and_call_subroutine` (Vroom (1991)(Lankhor)[cr VF - Company][t +21 ATX].adf boot-block load +an address to an address (20480 bytes)): Writes two words to locations relative to the custom registers, sets a byte at a specific memory address, and then copies 100 longwords from a source table to a destination starting at offset 256 before calling another routine through a pointer in the copied data. Kind: loader/decryption routine. Pass path: the loop completes and the indirect call at the end of the loop (the jump to the loaded code) is executed.
- `calculate_and_configure_hardware` (Vroom (1991)(Lankhor)[cr VF - Company][t +21 ATX].adf boot-block load +an address to an address (7680 bytes)): Performs arithmetic on d0, stores the result in a global variable, calls another routine, performs further arithmetic and bit manipulation on d0 and d1, and writes several 16-bit values to hardware registers including the DSKLEN register. Kind: disk hardware/track check. Pass path: the branch that continues normal execution/loading after the hardware configuration/verification.

## Functions


### Vroom (1991)(Lankhor)[cr VF - Company][t +21 ATX].adf:bootblock

- `load_boot_blocks_and_halt`: Issues disk read requests through the system I/O library into fixed memory locations, copies a block of data into low memory, then enters an infinite loop.

### logged disk load from Vroom (1991)(Lankhor)[cr VF - Company][t +21 ATX].adf boot-block load

- `copy_data_and_call_subroutine`: Writes two words to locations relative to the custom registers, sets a byte at a specific memory address, and then copies 100 longwords from a source table to a destination starting at offset 256 before calling another routine through a pointer in the copied data.
- `call_library_jump_entry`: Calls a routine through a fixed negative offset from a base register, which the decompiler could not resolve into a jump table, and then returns.
- `unpack_bitstream_backward_copy`: Decodes a compressed bitstream into memory, first moving the packed source when it overlaps the destination.
- `decode_bitstream_literals_and_matches`: Decodes a bit-coded stream of literal bytes and back-references into a forward output buffer, reading each length from a table and each distance from bit fields.
- `decode_bitstream_continuation`: Continues the bit-coded decoder by reading a length from a table and a distance from bit fields before copying from earlier output.

### logged disk load from Vroom (1991)(Lankhor)[cr VF - Company][t +21 ATX].adf boot-block load

- `update_interrupt_settings`: Writes 9 to a global variable, sets a hardware register to 2, and then enters a loop that decrements a global variable and writes 2 to a hardware register until the variable reaches zero.
- `poll_and_call`: Clears two words at specific locations, calculates an address in a register using bitwise operations, tests bit 2 of that address, and if not set, branches to another routine, otherwise it clears and sets bit 1 of that address while calling another routine twice, then tests bit 2 again before returning.
- `wait_for_button_press`: Polls bit 5 of a hardware register in a loop and returns with 0 in d0 when the bit is no longer set.
- `mask_hardware_register`: Sets bits 7 through 4 and clears bits 2 through 0 of a hardware register via a pointer.
- `compare_long_value`: Saves registers, checks the sign flag, loads several values from a table, adds them, and compares the result to a constant, branching to another routine or subtracting a value and branching to another routine.
- `calculate_and_configure_hardware`: Performs arithmetic on d0, stores the result in a global variable, calls another routine, performs further arithmetic and bit manipulation on d0 and d1, and writes several 16-bit values to hardware registers including the DSKLEN register.
- `copy_filtered_data_table`: Initializes hardware parameters and calls another routine, then iterates through a data table to copy bytes into a destination pointed to by a0, skipping bytes that match a specific value and decrementing a counter in d7 until a loop condition is met.
- `store_byte_and_branch`: Stores a byte from a global variable and branches to another routine.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `compare_and_branch`: Saves registers, loads a constant into d7, and compares a value from a table with a constant, branching to another routine if they are not equal or adding 2 to the pointer and branching if they are.
- `process_data_block`: Calls another routine twice, performs bitwise operations on values from a table, executes a loop 128 times to write modified values back to the table, and then conditionally updates a pointer in a table based on a comparison.
- `combine_masked_values`: Reads two 32-bit values from a table, masks them with d7, shifts the first value left by 1, and returns the result of the bitwise OR of the two values in d0.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `clear_cia_register`: Saves registers, loads the address of a CIA register into a1, loads the address of another CIA register into a2, clears bit 7 of the register in a1, and branches to another routine.
- `branch_to_another_routine`: Does nothing and then branches to another routine.
- `toggle_bits_and_branch`: Sets and then clears bit 3 of a byte at a location pointed to by a1, checks a global variable, and if it is positive, checks bit 4 of a byte at a location pointed to by a2, otherwise it calls another routine and branches to another routine.
- `clear_and_call`: Clears a long value at a global variable, performs bitwise operations on d0, and branches to another routine based on comparisons, otherwise it calls another routine, increments a global variable, and branches to another routine.
- `decrement_and_branch`: Calls another routine, decrements a long value at a global variable, and branches to another routine.
- `set_conditional_bit`: Sets bit 2 of a byte at a location pointed to by a1, and if bit 0 of d6 is set, clears bit 2 of that same byte before returning.
- `set_bit_and_branch`: Sets bit 1 of a byte at a location pointed to by a1 and then branches to another routine.
- `clear_and_set_bits`: Clears bits 1 and 0 of a byte at a location pointed to by a1, sets bit 0, and then executes a loop 384 times to perform a task before returning.

### logged disk load from Vroom (1991)(Lankhor)[cr VF - Company][t +21 ATX].adf boot-block load

- `decompress_bitstream_to_memory`: Copies a block of data from a source to a destination and then enters a loop that reads a bitstream to expand compressed data into a target buffer.
- `shift_bit_reader_word`: Writes the low byte of a stored value into the output word, then returns the source long shifted right by one bit with its top bit set.
- `shift_bit_buffer_by_count`: Shifts a bit buffer right by a count taken from a register, refilling it through a helper call whenever it empties.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `modify_byte_and_register`: Sets a byte at the address in a4 to 3, performs a bitwise OR on a hardware register with 0, tests bit 0 of d1 against d0, and sets bit 0 of d0 to 1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `prepare_display_and_copy_table`: Forbids multitasking and disables audio DMA and interrupts, then waits on a loop for a button.
- `build_copper_and_copy_tiles`: Writes bitplane pointer pairs into two copper lists using a fixed row stride, then sets DMA and sprite pointer words.
- `copy_tile_rows_from_string`: For each character of a string, maps it through a lookup to a tile index and copies eight bytes of that tile into a destination at a forty-byte row stride.
- `copy_block_to_low_memory`: Copies 800 bytes from a fixed source table into low memory starting at a fixed low address, one byte at a time.
- `tick_countdown_and_scale_value`: Decrements a countdown byte while it is nonzero, calling a routine when it reaches zero.
- `blit_text_tiles_with_markers`: Sets the blitter registers for a fixed copy, then for each character of a stored string copies an eight-row tile from a table into a destination row.
- `read_mouse_movement_position`: Reads the mouse port movement counters and updates a stored horizontal position by the difference from the previous reading.
- `divide_offset_byte_by_eight`: Adds 126 to a byte, wraps the result to eight bits, divides it by eight and stores it back in the same location.
- `update_two_digit_counter_display`: Checks a button port and increments or decrements a digit counter held in a text buffer, with modular digit wrap.
- `copy_strided_bytes_to_bitmap`: Copies eight bytes spaced forty bytes apart from a source to a destination computed from row and column values.
- `wait_with_countdown_loop`: Runs a short countdown loop from a difference of two globals, returning when it reaches minus one.
- `wait_blitter_flag_clear`: Repeatedly masks a status word in the record down to its busy bit until that bit is clear, a blitter-style wait loop.
- `handle_flag_bit_event`: Clears all but bit four of a flag word in the record and, when that bit was set, stores sixteen into a timer field and calls several helper routines.
- `clear_pointer_table_by_threshold`: Searches a byte table for the first entry above a threshold, then clears a table of thirty pointer entries and stores the matching pointers into a second table.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `restart_audio_channel_pointers`: Advances a frame counter and, when it runs out, waits in fixed delay loops around a DMA bit change.
- `process_audio_channel_effect`: Decodes one effect command from a stored word and applies it to a channel value.
- `sequence_audio_channel_step`: Reads the next packed sequence byte and, when its instrument nibble is nonzero, loads that instrument's pointer, length, volume and period values into the channel record.
- `apply_channel_effect_to_record`: Applies the effect nibble of a channel record.
- `copy_channel_word_field`: Copies one 16-bit field from the channel record at offset sixteen into the output structure at offset six and returns.
- `find_target_period_from_table`: Scans a row of a word table, selected by an index byte, for the first entry greater than the current value.
- `slide_value_toward_target`: Moves a stored value toward a target by a fixed step in one of two directions, clamping it when it arrives.
- `compute_oscillating_offset_value`: Computes a periodic offset from a waveform selected by two bits: a table lookup, a ramp, or a constant.
- `dispatch_effect_command`: Dispatches on the low nibble of a command byte.
- `store_low_nibble_field`: Stores the low four bits of one byte in the record into a second byte field of the same record.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutines_and_modify_memory`: Sets a global variable to 0, calls another routine twice, and performs a bitwise OR on a 16-bit value at an offset from a pointer in a0 with 48.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_fire_button_bit`: Extracts the lowest bit of d0, shifts it left by one, and performs a bitwise OR to update bit 1 of the CIAA Port A Data Register.
- `update_low_nibble`: Extracts the lower 4 bits from a byte at offset 3 of a structure in a6 and writes them into offset 31 of the same structure.
- `update_high_nibble`: Extracts the lower 4 bits from a byte at offset 3 of a structure in a6 and writes them into offset 30 of the same structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_global_and_trap`: Stores a value into a global variable and executes a software interrupt.
- `initialize_system_state`: Sets several global variables to specific values, calls another routine, and enters a loop that checks a bit in a global variable before calling another routine and performing memory operations.
- `configure_interrupt_masks`: Writes two 16-bit values from a table into the INTENA and INTENB registers and returns from an interrupt.
- `update_table_and_increment_counter`: Loads a byte from a table via a pointer in a0, uses that value doubled as an index into another table to write a word to a destination pointed to by a1, writes 336 to offset 2 of that destination, increments a long integer at a global address, and loops back to the start if the byte loaded from the first table is non-zero.
- `configure_blitter_parameters`: Sets several hardware registers, calculates values from a table to update a destination pointer, and enters a loop that calls another routine and updates global pointers.
- `process_hunk_relocation_table`: Iterates through a table of data structures, performing fixed-point multiplication to calculate offset values and updating a structure in a6 with the results.
- `copy_data_loop`: Copies 50 words from a source table to a destination table and another destination table using a decrementing loop, then increments a global variable and returns a value from memory if a comparison with a constant passes.
- `configure_blitter_hardware`: Stores several values from memory into specific locations, including a swapped word, and if a specific bit in a hardware register is not set, configures the blitter with a list pointer, a mask, a color, and a mode.
- `update_blitter_config`: Loads values from a table into several global memory locations, including swapped words, and if bit 14 of a value at offset 2 of a6 is not set, configures a blitter list pointer, a mask, a color, and a mode within a structure pointed to by a6.
- `decrement_global_counter`: Checks a 16-bit value in a global variable; if it is negative, it increments a word at an offset of 8260 in a structure in a5 and decrements the global variable before returning.
- `initialize_graphics_and_audio_tables`: Populates several memory tables with calculated values, including a sequence of scaled values, a series of incrementing offsets, and a set of division-based constants.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_string_data`: Finds the maximum byte value in a sequence of 128 bytes, uses it to calculate an offset into a data table, and copies 30 longwords into a destination buffer.
- `mt_end_2`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `modify_global_and_branch`: Saves registers to the stack, loads an address into a0, increments a byte in a global variable, compares a byte from a relative offset to another byte, and either branches to a previous instruction or clears a byte in a global variable before reloading the address into a0.
- `initialize_audio_and_memory`: Sets a global variable to zero, calls another routine four times, and if a specific memory location is non-zero, updates the global variable by calling another routine and performing a bitwise OR with 32768.
- `update_audio_channels`: Compares a byte from a hardware register to itself and, if they are not equal, enters a loop that calls another routine four times using different addresses from a table as arguments, then branches back to the start of the routine.
- `update_dma_and_joystick_state`: Enables DMA channels by setting bit 15 of the DMA controller and calls process_joystick_input_data to update a data structure based on joystick or mouse button states.
- `process_joystick_input_data`: Reads the CIAA port A peripheral register to check for joystick or mouse button states and processes a data structure based on the resulting nibble.
