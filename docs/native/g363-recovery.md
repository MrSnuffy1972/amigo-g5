# Game 363 (Agony v1.3 0960): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00363). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (5765973 top-level bytes in 35 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 24 of 24 units identical (bytes and relocations), 85 raw-word lines |
| Functions named and described | pass, 347 of 347 named (0 author symbols), 347 of 347 described (89 by rule, 232 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3142 of 3157 (99.52%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6231 of 6336 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 20197 |
| code | 23456 |
| data | 5704404 |
| padding | 252 |
| unused | 17664 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 672 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1934 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 267 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 61 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 207 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 223 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 66 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1434 | 38 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 38 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 51 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 375 | 77 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 41 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 91 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 201 | 26 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 399 | 62 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 105 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 132 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- Agony_v1.3_0960/Agony_v1.3_0960_hdf/Agony/Disk.1: raw disk image (657408 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Agony_v1.3_0960/Agony_v1.3_0960_hdf/Agony/Disk.2: raw disk image (946176 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Agony_v1.3_0960/Agony_v1.3_0960_hdf/Agony/Disk.3: raw disk image (940032 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Agony_v1.3_0960/Agony_v1.3_0960.hdf: hard-disk image (AmigaDOS OFS, 6176 blocks): its filesystem parses and all 10 files in it are byte-identical to the extracted tree Agony_v1.3_0960_hdf, which is recovered file by file; no block outside the filesystem holds data
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (play-rs-m1500-unpk02): 933 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 208 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 49 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 61 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 116 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 39 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 586 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 22 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 639 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 12 first-run image unit(s) in 2 version group(s); 5386 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- self-modifying code at an address (53 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (53 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (52 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (52 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- 3 packed track region(s), 2525952 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `write_custom_registers_then_call`: Writes the audio and disk control, DMA control, interrupt enable and request registers and the background colour register, then calls five unnamed routines.
- `patch_code_with_jump_stubs`: Rewrites several code locations in place, storing jump and call instructions that point at unnamed routines when a fixed check value matches, and scans a low-memory range replacing one longword pattern with a call instruction.
- `write_jump_patch_when_set`: Calls an unnamed routine and, when it returns a nonzero value, stores that value in a global cell and calls another routine.
- `store_constant_via_base`: Stores a fixed word to a register at a fixed offset from a base pointer, then returns its input unchanged.
- `find_byte_pattern_in_range`: Scans forward from a start address toward a limit, comparing a byte pattern at each position, and returns as soon as every byte of the pattern matches.
- `poll_right_middle_buttons`: Accesses the pot control and pot data registers to read the right and middle button state.
- `patch_matching_longwords_in_range`: Scans a memory range two bytes at a time, and wherever a longword equals one of two fixed patterns it overwrites the first word with a fixed replacement word.
- `return_input_unchanged`: Returns its input register value unchanged and does nothing else.
- `frag_move_andi_movea_rte`: Short code fragment that copies d0 to the stack, then masks d0 with an address, then copies d0 to a4, then copies the top of the stack to d0, then returns from the exception.
- `set_fields_on_marker_byte`: Compares an input byte with a fixed marker character and, when it matches, stores a fixed word and a fixed byte into two fields of a structure.
- `set_fields_by_base_address`: When the input byte is the marker character, it compares the base address with five fixed addresses and, on a match, stores a fixed word and byte at the matching pair of offsets.
- `copy_block_to_low_ram`: Copies 240 bytes from one fixed area to a fixed low-memory address, one byte at a time.
- `compare_shifted_code_with_global`: Shifts the low word of D0 right by one bit, XORs the low byte with 127 and compares it with a global byte.
- `spin_forever_in_place`: Loops forever doing no work, since the decoded body leaves only a bare infinite loop after removing unreachable blocks.
- `set_word_field_all_ones`: Writes the all-ones word to a field at a fixed offset from a base pointer and returns.
- `write_jump_entries_to_ram`: Writes absolute jump instructions into fixed low-memory locations, each pointing at an unnamed routine, and writes one more far-memory jump the same way.
- `copy_block_either_direction`: When a flag byte in a register is nonzero, it copies a 48-byte block from the caller's buffer into a fixed buffer and calls one unnamed routine.
- `wait_port_bit_when_flagged`: Calls one unnamed routine, then if a RAM count is nonzero it busy-waits while a port status bit stays set.
- `relay_to_unnamed_routine`: Calls one unnamed routine and returns, with no other work.
- `call_library_or_return_zero`: Stores a character derived from a register in a RAM cell, then calls a library entry through a base pointer and returns its result when it is non-negative.
- `xor_longwords_with_key`: XORs each longword in a block with a key held in a register, stepping four bytes at a time until the byte count is used up.
- `stub_return_only`: This routine returns immediately without performing any data manipulation or hardware access.
- `unpack_control_byte_stream`: Decodes a control-byte compressed stream into memory, handling literal copies, back-references of two lengths and repeat-fill runs until the output pointer reaches its end.
- `call_unnamed_routine_undecoded`: The pseudo-code is unavailable, so the only evidence is an eleven-instruction body that makes one call to an unnamed routine.
- `transfer_scores_block_capped`: Sets a fixed check longword, then calls a library entry to transfer up to 48 bytes into a fixed buffer.
- `transfer_scores_block_plain`: Calls a library entry at a fixed offset with a 48-byte buffer and returns the complement of its result.
- `sync_color_to_vposr_on_disk_busy`: Copies the vertical position register to the first color register in a loop that continues as long as bit 6 of the CIAA port A register is set, otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `save_registers_and_update_state`: Pushes registers d0 through d7 and a0 through a6 onto the stack, loads the custom chip base address into a6 and a global variable address into a5, reads a word from offset 30 of the custom chip structure, and branches away if bit 4 of that byte is set.
- `restore_and_update_state`: Restores registers from the stack and returns via rte, then branches on a custom chip register and updates several global variables, storing computed values into a table.
- `increment_counter_reset_on_match`: Increments a 16-bit counter in a global variable at offset 31942 and returns immediately when that counter is not equal to one.
- `update_table_index_from_loaded_byte`: Loads a byte from a table indexed by a 16-bit value in a global variable and compares it with a stored byte at offset 31944, branching past the update when the loaded byte is smaller.
- `process_keyboard_input`: Reads a keyboard scancode from the CIA serial register and, if the key is not currently held, stores its decoded value in a global byte then calls a game-input handler.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_structure_list_2`: Walks a linked list of structures starting at the global table, and for each non-empty entry copies its data into a second table while recording its size and offset.
- `call_decoder_triple_2`: Saves the working and address registers, then calls the decoding routine three times with the values 1, 2 and 3 loaded into d7.
- `jump_to_routine`: This routine is only a jump to another routine, so it does nothing of its own and returns immediately.
- `jump_to_initialize_structure_table`: Jump stub: passes control straight on to initialize_structure_table.
- `jump_to_decode_length_prefixed_buffer_`: Jump stub: passes control straight on to decode_length_prefixed_buffer_2.
- `jump_to_initialize_structure_table_2`: Jump stub: passes control straight on to initialize_structure_table.
- `silence_audio_channel_2`: Writes zero to the audio register at offset 170 in a loop that repeats four times with a 16-bit counter before returning.
- `initialize_structure_table`: Builds and initializes a structure at a table offset computed from d7, copying a 32-bit value, setting several word and byte fields, and clearing additional fields before returning.
- `decode_length_prefixed_buffer_2`: Multiplies the word in d7 by 64 to form a base offset, then, if a status byte is nonzero, stores the sum of two words at a fixed offset into a DMA register pair and the shifted product of two other words into a global variable.
- `decode_length_prefixed_buffer`: Initializes a structure by clearing a byte at offset 62 and setting a 16-bit value at offset 34 to 65535, then reads a length-prefixed byte sequence from a buffer pointer stored at offset 4, decoding the values into fields at offsets 18, 31, 32, 37, 38, and 56, and advancing the buffer pointer, otherwise it returns.
- `prepare_scaled_offset`: Reads a 16-bit value from offset 26 of the structure in a0, computes a scaled offset from d7, copies a5 into a4, adds the scaled offset to it, and clears the 16-bit value in d3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `noop_register_operations`: Performs a series of no-op OR operations on register d0, which change nothing, then returns without altering any data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_byte_and_call_cia`: Stores a byte from register d0 at offset 57 of the structure in a0 and saves register a1 at offset 4, then calls update_structure_cia and copies the byte at offset 55 to offset 56 of the same structure.
- `update_structure_cia`: Reads a word at offset 26 of the structure in a0, indexes a table with it, and stores a computed value at offset 20 of that structure along with two values at offsets 160 and 164 of the structure in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_pointer_sequence`: Reads three pointers from offsets 4, 8, and 12 of the structure in a0, then reads a short from offset 12.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_fire_button`: Writes the byte in d0 to the fire button byte in CIAA_PRA and returns.
- `decode_length_sequence`: Decrements the byte at offset 53 of the structure in a0, and when it was zero reads a length-delimited sequence from a pointer at offset 0, decoding several byte ranges into offsets 58, 54, 53, and 53 of the structure.
- `update_entity_state`: Updates a per-entity state in the structure in a0: it derives an index from offset 26, looks up a table to set the byte at offset 52 and adjusts the value at offset 32, then decrements a counter at offsets 16 and 18 and, when a threshold is crossed, modifies the value at offset 8 using a global table and the bytes at offsets 49, 50 and 57.
- `decrement_offset_2`: Reads a word at offset 26 of the structure in a0, indexes a table with it, adds one to the fetched byte, subtracts that from the word at offset 44, and stores the result there, returning when the subtraction does not carry.
- `scale_and_store_2`: Multiplies the word at offset 40 of the structure in a0 by the value in d1, shifts the product right by nine bits, masks it to fifteen bits, and stores the result at offset 38.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_table_indexed_values`: Loads a 16-bit value from a table indexed by a counter and stores it at offset 4 of a structure, then decrements that counter.
- `increment_and_check_counter`: Increments a 16-bit value at offset 12 of a structure, then checks if it exceeds 1125 to set a 32-bit value to all ones and update specific fields in a global structure.
- `update_state_and_sync_counters`: Updates a structure with values derived from a global table, compares a target value against a calculated threshold, and selects a new state and constant based on the comparison result.
- `update_state_and_increment_counter`: Writes the constant 1330 to an offset of 4 in a structure and the constant 3 to an offset of 6 in another structure, then calls increment_and_check_counter.
- `select_state_by_value`: Compares a 16-bit value at offset 6 of a structure in a3 to 4; if equal, it branches to another routine, otherwise it writes 1250 to offset 4 of a structure in a4, sets the value at offset 6 of the structure in a3 to 1, and branches to another routine.
- `update_state_and_increment_counter_2`: Writes 1370 to an offset 4 of a structure, sets a 16-bit value at offset 6 of another structure to 1, and branches to another routine.
- `update_state_by_comparison`: Compares a 16-bit value at offset zero of a structure in a4 with the value in d1; if they are not equal, it branches to another routine.
- `update_parameter_and_counter`: Writes the value 1370 to an offset of 4 in a structure, writes the value 4 to an offset of 6 in another structure, and branches to another routine.
- `update_state_and_branch`: Compares a 16-bit value at offset 6 of a structure in a3 to 1; if equal, it branches to another routine, otherwise it writes 1290 to offset 4 of a structure in a4, writes 2 to offset 6 of the structure in a3, and branches to another routine.
- `update_state_and_increment_counter_3`: Writes the value 1258 to an offset of 4 in a structure, writes the value 2 to an offset of 6 in another structure, and branches to another location.
- `update_state_and_sync_counters_2`: Calculates two new values by adding constants to values from a structure, stores them in another structure, and then performs a series of conditional updates to a state value and a status flag based on comparisons between the new values and a threshold.
- `update_state_and_increment_counter_4`: Sets a 16-bit value at offset 4 of a structure to 1250 and a 16-bit value at offset 6 of another structure to 2, then calls increment_and_check_counter.
- `update_state_and_reset_counter`: Checks if a 16-bit value at offset 6 of a structure is equal to 1; if so, it branches away, otherwise it writes 1328 to offset 4 of another structure, sets the value at offset 6 of the first structure to 4, and branches to another routine.
- `update_parameter_and_increment_counter`: Writes the value 1370 to an offset of 4 in a structure, writes the value 4 to an offset of 6 in another structure, and then calls increment_and_check_counter.
- `update_state_on_threshold_comparison`: Compares a 16-bit value at offset 2 of a structure in a4 with a value in d2; if the value is not greater, it checks if a 16-bit value at offset 6 of a structure in a3 is equal to 4.
- `update_state_and_increment_counter_5`: Writes 1370 to an offset of 4 in a structure, writes 1 to an offset of 6 in another structure, and then calls increment_and_check_counter.
- `update_state_on_condition`: Compares a 16-bit value at offset 6 of a structure in a3 to 4; if equal, it branches to another routine, otherwise it writes 1290 to offset 4 of a structure in a4, writes 3 to offset 6 of the structure in a3, and branches to another routine.
- `update_counter_and_branch`: Updates a 16-bit value at offset 12 of a structure via increment_and_check_counter, writes 1330 to offset 4 of one structure, writes 3 to offset 6 of another structure, and branches to another routine.
- `init_struct_with_counters`: Initializes a structure pointed to by a2 with a base value and several fields copied from a structure pointed to by a1, increments a counter at offset 12 of the structure pointed to by a0, and clears a flag there.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_cias_and_copy_block`: Clears the CIA peripheral control registers by writing zero and mask values to them, copies a block of words from a source table to a RAM region in a loop, and clears several global variables before branching on a bit in a global to one of two setup sequences that each call a store-and-jump routine.
- `store_and_jump`: Stores a 16-bit value from register d6 into a global variable, then jumps to another routine and never returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_audio_and_interrupts`: It waits until a hardware register holds a specific value, then clears two custom registers and stores an interrupt handler address.
- `multiply_nibbles_and_pack`: Calls a helper routine, then repeatedly multiplies each nibble of a source word by a shift factor and packs the results into a destination word, looping over the data and over the factor until the factor reaches seventeen.
- `decrement_and_multiply_nibbles`: Decrements a global counter, calls a helper routine, then repeatedly multiplies each nibble of a source word by the factor and packs the results into a destination word, looping over the data and over the factor until the factor reaches zero.
- `check_zero_global`: Clears a global variable, compares it with zero, and returns if it is not zero; otherwise it falls through to a lone return.
- `compare_zero_register`: Clears a global variable, compares it with a 16-bit value in d0, and returns if they differ.
- `fill_table_with_value`: Fills five consecutive 32-bit slots in a table with the two halves of a 32-bit value from d0, advancing the table by 8 bytes per slot and adding a stride to the value each iteration.
- `init_audio_dma_channels`: It saves registers, increments a global counter and returns immediately when that counter wraps to zero.
- `init_timer_and_loop_helper`: Initializes a hardware timer by writing a magic value to a global register, divides the input in d0 by 6144 to get quotient and remainder, calls a routine with those values, then loops calling a helper routine eight times while decrementing a counter.
- `handle_raster_scan_table`: Calls a routine three times with the values 3, 4 and 5 in d5 and returns when any of them yields zero, otherwise saving the registers and calling another routine.
- `wait_raster_and_scan_table`: Writes the byte 125 to a byte at offset 12033 in the structure in a3, calls a routine that toggles a CIAB bit and waits for a raster line, then clears a bit selected by d5 and calls a routine that busy-waits for a raster line.
- `call_wait_ciab_bit_four_times`: Calls a routine that waits for a CIAB bit four times in a row, then returns.
- `wait_ciab_bit`: Writes values to the timer and control registers of a CIAB chip, then waits in a loop reading its input register until bit 0 is set, otherwise it returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_raster_and_clear_pointer`: Waits in a loop until bit 4 of a custom chip register is clear, then sets bit 1 of a CIAB register, calls a raster wait routine, and waits until bit 5 of the custom chip register is set.
- `increment_counter_toggle_ciab`: Increments a 32-bit counter stored in the memory location pointed to by a2 and returns.
- `toggle_ciab_and_wait_raster`: Clears bit 0 of a CIAB register, calls a raster wait routine, sets bit 0 again, calls the raster wait routine a second time, then waits in a loop reading another CIAB register until bit 5 is set before returning.
- `wait_ciab_bit_then_raster`: Writes the byte 253 to a byte at offset 12033 in the structure in a3 and calls a routine that waits for a CIAB bit four times.
- `wait_raster_loop`: Waits in a loop by reading the vertical beam position register until bit 4 is set, calling the raster wait routine while a counter is not yet zero, then restores registers and returns.
- `check_flag_and_call_raster`: Sets bit 2 of a byte at offset 12033 in the structure in a3 and clears it if the low bit of the value in a2 is set.
- `init_chips_and_wait_raster_loop`: Initializes several timer and control registers of a chip, calls a raster wait routine, then loops decrementing a counter while calling the raster wait routine until bit 1 of a register is set or the counter reaches zero, finally resetting the register and returning a value.
- `scan_table_and_interleave_bits`: Scans a table of words looking for a matching entry, then for each match calls a helper routine to interleave bits, accumulates the results in a loop, and returns zero when twelve matches are found or negative one otherwise.
- `compare_interleaved_value`: Calls a helper routine to interleave bits from a table, then compares the returned value against four constants depending on a register and returns zero on a match or negative one otherwise.
- `scan_table_and_xor_interleaved`: Scans a table skipping matching entries, then for each remaining block calls a helper routine to interleave bits, XORs the result into a destination buffer, and returns after processing twelve blocks.
- `interleave_bits`: Reads two consecutive 32-bit words from the table pointed to by a1, interleaves their even and odd bits, and returns the combined value.
- `wait_raster_and_setup_block`: Waits in a loop reading the beam position register until it equals a specific value, sets up a block of chip registers and copies a table of words into a destination, then waits again for the beam position and sets another register before returning.
- `mask_and_branch_d0`: Clears the lower bits of register d0 using a bitwise AND operation and then performs a relative branch to another location.
- `reset_audio_dma_state`: Searches a table for a byte greater than a running value, then uses its position to index a second table, clears a block of memory, stores pointers into that table, and clears several audio volume and DMACON registers before returning.
- `check_flag_and_play_sound`: It pushes registers onto the stack, increments a global byte and returns immediately when that byte is nonzero.
- `configure_and_clamp_audio_channels`: For each of four audio channels it clears the channel's volume register, loads a sound definition, and calls the sound setup routine, then clamps the computed volume to zero when it would be negative.
- `configure_and_play_sound`: Loads a sound definition from a table into a structure, sets up its channel data, and then dispatches on a command byte to configure audio channels, play a sound, or update playback state.
- `dispatch_structure_byte_2`: Reads a byte at offset 2 of the structure in a6 and dispatches on its low nibble, handling values 0 through 15 with distinct code paths that modify a 12-bit value at offset 16 of the structure, and returns the modified value with the high nibble cleared.
- `copy_struct_field_and_dispatch`: Copies a 16-bit value from offset 16 of the structure in a6 to offset 6 of the structure in a5 and returns.
- `clamp_and_index_table_2`: Looks up a 16-bit value in a table indexed by a byte at offset 18 of the structure in a6 and a 12-bit value from the structure, clamps the index to the table bounds, then stores the result at offset 12 and compares it with the value at offset 16, setting a flag byte at offset 22 when it is smaller.
- `clamp_and_index_table`: Returns the incoming value if a word at offset 24 of the structure in a6 is zero.
- `compute_scaled_signed_value`: Reads a byte at offset 27 of the structure in a6 and uses its low five bits, after a right shift by two, to index a table, then combines the result with the two low bits of the byte at offset 30 and the sign of the offset-27 byte to compute a signed value that is scaled by the low nibble of the byte at offset 26.
- `dispatch_structure_byte`: Reads a byte from offset 2 of the structure in a6 and dispatches on its low nibble, handling values 9 through 15 with distinct code paths, then reads the byte at offset 3 and dispatches on its high nibble to branch into the remaining cases.
- `store_nibble_and_decrement`: Copies the low nibble of a byte at offset 3 of the structure in a6 into offset 18 and returns.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `disable_interrupts_and_call`: Disables interrupts and clears a global flag, then loads a jump table entry and calls it before jumping to a fixed address.
- `configure_custom_chip_register`: Writes a value from d6 into a global variable at offset a fixed address, then configures a custom chip register by loading its base address into a4, inverting selected bits of that address into d2, storing the result into a global variable at offset a fixed address, and copying the base address into a3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_return_immediately`: A single return instruction with no other effect, so the routine is an empty stub.
- `nop_padding`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_2`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_3`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding_4`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_5`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_6`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding_7`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_8`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_9`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_10`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_11`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding_12`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_13`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_14`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_15`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_16`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_17`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_18`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_19`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_20`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_21`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_22`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_23`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `set_flags_in_structure`: Loads the address a fixed address into a5, reads a 16-bit value from offset 30 of the structure in a6 into d0, and sets bits 1 through 5 of that value into d0.
- `call_then_jump_forward`: Calls the routine at 38850, then jumps to the routine at a fixed address.
- `increment_structure_counter`: Writes the value 32800 into offset 150 of the structure in a6, increments a 16-bit value at offset 27564 of the structure in a5, and then performs three no-op operations on d0.
- `branch_when_sign_clear`: Branches to the routine at 5990 when the sign flag is clear, and otherwise performs three no-op operations on d0.
- `branch_when_zero_set`: Branches to the routine at offset 106 when the zero flag is set, and otherwise performs a no-op operation on d0.
- `tiny_move`: Small routine of 1 instruction that copies d0 to (a0)+.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `read_input_status_register`: Reads the input status register at custom offset a fixed address into d5, copies that value into d2, shifts it right by one bit, and then performs three no-op operations on d0.
- `branch_when_sign_clear_again`: Branches to the routine at offset 212 when the sign flag is clear, and otherwise performs a no-op operation on d0.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `test_bit_in_d2`: Tests bit 0 of the byte in d2, and then performs two no-op operations on d0.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `test_bit_in_d4`: Tests bit 0 of the byte in d4, and then performs two no-op operations on d0.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `tiny_addi`: Small routine of 1 instruction that adds 37 to d1.
- `tiny_addi_2`: Small routine of 1 instruction that adds 32 to d0.
- `tiny_addi_3`: Small routine of 1 instruction that adds 16 to d1.
- `tiny_tst_ori_ori`: Small routine of 3 instructions that tests d6, then merges 0 into d0, then merges 0 into d0.
- `check_zero_flag_d0`: It branches forward when the zero flag is set, then executes five no-op ors on d0 before falling through into the next routine.
- `branch_if_zero_and_mask_d0`: Branches to a nearby routine if the zero flag is set, then clears the upper bits of the d0 register so only its low byte remains.
- `add_eight_to_counter_a5`: It adds eight to the 16-bit value stored at offset 6d08 of the structure in a5 and then falls through into the next routine.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `copy_a0_to_a1_a2`: It copies the address in a0 into a1 and a2, then performs two no-op ors on d0 before falling through into the next routine.
- `load_word_from_a0_d6`: It loads a 16-bit word from the address in a0 into d6 and post-increments a0, then executes five no-op ors on d0 before falling through.
- `set_bit3_shift_d0`: It moves zero into d3, sets bit 3 of d0, shifts d0 left by eight bits, then executes two no-op ors on d0 before falling through.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `store_d0_to_a2_d0`: It stores d0 into the address in a2 and post-increments a2, sets bit 3 of d0, then executes two no-op ors on d0 before falling through.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to (a2)+.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `check_flag_copy_d1_d4`: It branches forward when the zero flag is clear, copies d1 into d4, then executes four no-op ors on d0 before falling through.
- `store_byte_and_set_flag`: Stores a byte from d0 into a memory location, then sets bit 7 of a global variable before returning.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `conditional_branch_nops`: Branches forward when a condition is met, otherwise falls through into a sequence of no-op byte operations.
- `store_fifty_call_routine`: It stores the value fifty into the 16-bit value at offset 6d08 of the structure in a5, calls the routine at 17b4, then branches forward when the zero flag is set before executing two no-op ors on d0.
- `test_flag_check_a5`: It branches forward when the zero flag is set, tests the 16-bit value at offset 6c4c of the structure in a5, sets bit 3 of d0, then executes two no-op ors on d0 before falling through.
- `tiny_move_3`: Small routine of 1 instruction that copies d1 to (a3)+.
- `check_flag_clear_global`: It branches forward when the zero flag is set, sets bit 3 of d0, loads the address of a global variable into a0, clears the 32-bit value at offset 100 of that variable, then executes two no-op ors on d0.
- `divide_and_store_d3`: It moves the value in d1 into d6, then sets bits 0 and bits 3 through 5 of d0, divides d3 by d0 using signed 16-bit division, and finally loads a word from a global variable at offset 174 in a6.
- `tiny_subi`: Small routine of 1 instruction that subtracts 160 from d0.
- `iterate_table_bytes`: Loads a byte from the table pointed to by a0 into a1, increments a0, and returns when the byte is zero, otherwise it continues with the next entry.
- `store_d0_via_a0`: It copies the value in d0 into the memory location pointed to by a0, then clears the low bits of d0 in three separate steps before returning.
- `toggle_bit_and_set_d1`: Toggles bit 0 of a 16-bit value at offset 6c42 of the structure in a5, branches forward when that value becomes zero, and sets bit 7 of d1.
- `tiny_addi_4`: Small routine of 1 instruction that adds 200 to d4.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `set_bits_in_a0`: It sets bit 5 of the byte pointed to by a0 and then returns, so the routine only modifies that single memory location.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `load_byte_offset_a0`: It loads a byte from the address a0 plus the offset in d0 into d1, then clears the low bits of d0 before returning.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `branch_and_load_table`: It branches to a later address when d0 is greater than or equal to zero, then clears the low bits of d0 and finally loads the address of a table at offset 18126 in a5 into a0.
- `copy_table_bytes_indirectly`: Loads a byte from a table indexed by a value in a1, stores it through a pointer in a0, and if the loaded byte is not zero, increments both pointers and repeats the process.
- `test_global_word_a5`: It is a single test instruction on a word in a global variable at offset 174 in a5, so the routine only evaluates that value and returns.
- `tiny_moveq_ori`: Small routine of 2 instructions that loads 0 into d6, then merges 0 into d0.
- `read_custom_register`: It reads a byte from the custom register at offset 205 into d5, shifts it right by one bit into d2, then clears the low bits of d0 in eight steps before returning.
- `test_status_word`: Reads the 16-bit value at offset 174 of the structure in a5 and tests whether it is zero, discarding the result.
- `test_status_word_2`: Tests the 16-bit value at offset 6c68 of the structure in a5 and sets the corresponding condition flags, then returns.
- `clear_status_word`: Clears the 16-bit value at offset 6d12 of the structure in a5 to zero, then returns.
- `load_and_or_register`: Loads the 16-bit value at offset 6c64 of the structure in a5 into d0 and performs a no-op or on that register, then returns.
- `tiny_tst_ori`: Small routine of 2 instructions that tests d0, then merges 0 into d0.
- `store_word_to_struct`: Stores the 16-bit value of d0 at offset 6c6a of the structure in a5, then returns.
- `store_word_and_or`: Stores the 16-bit value of d0 at offset 6c54 of the structure in a5 and performs a no-op on d0, then returns.
- `empty_routine`: This routine is empty and contains no instructions.
- `clear_and_call_routine`: Clears a 16-bit value at offset 6c68 of the structure in a5, sets bit 2 of d0, and calls the routine at offset 6ea.
- `compute_address_and_set_bits`: Computes an address by subtracting 17384 from a5 into a0, sets bits 0 through 6 of the byte at the address in a3, performs several no-ops on d0, and adds 15 to d1.
- `tiny_lsl`: Small routine of 1 instruction that shifts d1 left by 8.
- `combine_registers_and_call`: Performs a no-op on d0, combines d1 into d0 with a bitwise or, sets bit 2 of d0, calls another routine, and stores the pointer in a0 at offset 6ce0 of the structure in a5.
- `store_pointer_and_disable_interrupts`: Stores the pointer in a0 at offset 6ce4 of the structure in a5, sets bits 0 through 6 of d0, disables interrupts via the supervisor register, and performs two no-ops on d0.
- `tiny_addq_ori`: Small routine of 2 instructions that adds 4 to a1, then merges 0 into d0.
- `load_immediate_and_preincrement`: Loads the immediate value 7 into d0, reads and post-increments the 16-bit value at the address in a3, and performs three no-ops on d0.
- `store_offset_into_a2`: Loads the address of a global variable at offset 14140 from a5 into a2, then clears the low byte of d0 twice and finally sets its low eight bits to two.
- `copy_longword_then_load`: Copies one long word from the source pointed to by a2 to the destination pointed to by a0, advancing both pointers, then loads a long word from a global variable at offset 11965 into d0.
- `return_if_d0_byte_not_zero`: Compares the low byte of d0 against zero and returns immediately when it is not equal, otherwise it falls through into the following routine.
- `tiny_move_ori`: Small routine of 2 instructions that copies 1 to d0, then merges 0 into d0.
- `store_word_then_clear_d0`: Stores the low word of d1 into a global variable at offset 27702 from a5, then clears the low byte of d0 eight times before falling through into the next routine.
- `store_word_then_advance`: Stores the low word of d0 into the location pointed to by a0 and advances that pointer, then falls through into the following routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_if_flag_not_one`: Compares the low word of a global variable at offset 27754 from a5 against one and returns when the values are not equal, otherwise it falls through into the following routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_flag_two_then_set_d0`: Compares the low word of a global variable at offset 27754 from a5 against two and returns when the values are not equal, then clears the low byte of d0 twice and sets its low eight bits to 254.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_state_dispatch`: Compares a 16-bit value at offset 27754 of the structure in a5 against 4, 5 and 6, and for each matching case sets bits of d0.
- `tiny_movea_movea_ori`: Small routine of 3 instructions that copies a0 to a1, then copies a0 to a2, then merges 0 into d0.
- `tiny_move_4`: Small routine of 1 instruction that copies (a0)+ to d6.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `shift_d1_right_then_d2_left`: Arithmetically shifts the low word of d1 right by one bit, then clears the low byte of d0 six times before shifting the low word of d2 left by eight bits.
- `or_registers_then_adjust_d4`: Bitwise-ORs the low word of d3 with the low word of d4, then clears the low byte of d0 twice and sets the low eight bits of d4 to 255.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_values_if_flag_matches`: Reads a byte from a global variable at offset 27755 from a5 and returns when it is not equal to 255, otherwise it stores the low word of a5 into a location at offset 26549 from a0 and stores a0 into a global variable at offset 13948 from a5.
- `branch_when_zero_set_2`: Branches to the routine at offset 216 when the zero flag is set, and otherwise clears the low byte of d0 by OR-ing it with zero.
- `branch_when_sign_clear_2`: Branches to the routine at offset 84 when the zero flag is clear, and otherwise clears the low byte of d0 by OR-ing it with zero.
- `store_constants_memory`: Stores the 16-bit value 65535 at offset 70f0 of the structure in a5 and clears the 16-bit value at offset 6cd6 of the same structure, then returns.
- `tiny_moveq`: Small routine of 1 instruction that loads 7 into d0.
- `fill_table_constants`: Fills six consecutive 32-bit and 16-bit fields at offsets 1a6 through 1be of the structure in a6 with the given constants, then reads a 16-bit value from offset 6c1a of the structure in a5 and returns.
- `load_table_address`: Loads the address of the data at offset 1144 of the structure in a5 into a register and returns.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `increment_pointer_value`: Increments the 32-bit value pointed to by a1 by two and returns.
- `tiny_cmp`: Small routine of 1 instruction that compares d1 with 128.
- `extend_word_register`: Sign-extends the 16-bit value in d1 to a 32-bit value and returns.
- `tiny_lsl_move_ori`: Small routine of 3 instructions that shifts d2 left by 8, then copies d2 to d5, then merges 0 into d0.
- `test_bit_stack_adjust`: Combines the values in d1 and d2 with a bitwise OR, tests bit d1 of the byte pointed to by a4 while advancing that pointer, then subtracts 256 from the stack pointer and returns.
- `branch_when_flag_clear`: Branches to the routine at offset 416 when the zero flag is clear, and otherwise clears the low byte of d0 by OR-ing it with zero.
- `load_address_stack_bit`: Loads the address of the data at offset 16126 of the structure in a5 into a register, sets bit d0 of the top of the stack, clears bit d0 of d7, and returns.
- `compare_branch_clear_flags`: Skips the next instruction when the carry flag is clear, compares d0 with d2 and branches away when they differ, otherwise clears the 16-bit values at offsets 1bc and 6cc6 of the structure in a5 and returns.
- `set_flags_load_address`: Stores the value three at offsets 6c36 and 6c70 of the structure in a5, loads the address of the data at offset 4474 of the structure in a5 into a register, and returns.
- `test_status_byte`: Tests the byte at offset 6c6b of the structure in a5 and returns.
- `branch_if_less_equal`: It is a single conditional branch that jumps forward when the condition code set by a previous compare indicates a less-than-or-equal result, otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_word_to_two`: It compares the word at the global variable with the decimal constant 2 and branches forward when the two are equal, otherwise it falls through.
- `branch_if_equal`: It is a single conditional branch that jumps forward when the condition code set by a previous instruction indicates the compared values were equal, otherwise it returns.
- `store_a0_to_global`: It stores the 32-bit value held in register a0 into the global variable, then runs straight into the next routine without returning.
- `load_byte_from_table`: Loads a byte from the table pointed to by a0 into a1, increments a0, and returns.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `store_word_and_align`: It stores the 16-bit value in register d0 to the memory location pointed to by a0, then performs three no-op operations that force the lower byte of d0 to align it to a byte boundary before falling through.
- `store_word_and_clear_d0`: Stores the 16-bit value in d0 to the memory location pointed to by a0, then clears the low byte of d0 twice more, leaving d0 zeroed.
- `tiny_move_ori_2`: Small routine of 2 instructions that copies d0 to (a0), then merges 0 into d0.
- `tiny_move_5`: Small routine of 1 instruction that copies d0 to (a0).
- `store_word_and_clear_bytes`: Stores the 16-bit value in d0 to the word pointed to by a0, then writes a zero byte to each of the three consecutive bytes following that word.
- `store_d0_via_pointer`: Stores the 16-bit value in d0 to the memory location pointed to by a0, then clears the low byte of d0 with two byte-wise OR operations.
- `store_word_and_clear_d0_2`: Writes the 16-bit value in d0 to the memory location pointed to by a0, then clears d0 by OR-ing it with zero three times in a row.
- `store_word_via_pointer`: Stores the 16-bit value in d0 at the address in a0, then clears the upper byte of d0 by OR-ing it with zero, and returns.
- `store_word_from_d2`: Stores the 16-bit value in d2 at offset 29406 of the structure in a5, then clears the upper byte of d0 by OR-ing it with zero four times, and returns.
- `restore_regs_and_rte`: Restores registers d0-d7 and a0-a6 from the stack and returns from an exception via rte.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `clear_counters_and_restore`: Clears two 16-bit counters at offsets 168 and 27914 of the structures in a6 and a5, sets the 16-bit values at offsets 154 and 156 of the structure in a6 to 128, then restores registers d0-d7 and a0-a6 from the stack and returns from an exception via rte.
- `compare_and_store_flag`: Copies the 16-bit value at offset 27908 of the structure in a5 into d0, computes a pointer by subtracting 17528 from the address in a5, and compares a byte at offset 27916 of the structure in a5 against d1.
- `copy_word_within_struct`: Copies the 16-bit value at offset 27912 of the structure in a5 to offset 27910 of the same structure, then clears the upper byte of d0 by OR-ing it with zero, and returns.
- `push_regs_and_load_custom`: Pushes registers d0-d7 and a0-a6 onto the stack and loads the custom register base address into a6, then clears the upper byte of d0 by OR-ing it with zero eleven times, and returns.
- `xor_and_mask_d0`: XORs the upper byte of d0 with 127, stores the result as a byte at offset 17479 of the structure in a5, sets bits 6 and 7 of d0 by OR-ing it with 192, clears the upper byte of d0 by OR-ing it with zero, and masks the low byte of d0 with the byte at offset 4 of the structure in a6, then returns.
- `compare_byte_pairs`: Loads the byte at offset 6 of the structure in a6 into d0, sets d1 to 1, masks it with the byte at offset 4 of the structure in a6, shifts it left by eight bits, stores the byte at offset 6 of the structure in a6 into d1, compares d1 with d0, and returns.
- `shift_and_compare_d0`: Sets d0 to 1, masks it with the byte at offset 4 of the structure in a6, shifts it left by eight bits, clears the upper byte of d0 by OR-ing it with zero, compares d1 with d0, and returns.
- `clear_cia_control_register`: Clears the control register A of the CIA chip, then clears the low byte of d0 twice by OR-ing it with zero.
- `handle_condition_flags`: Branches to the routine at offset 896 when the zero flag is set, and compares d0 against 55.
- `store_table_value_offset`: Loads a byte from a table indexed by a value in a1, adds a constant to it, stores the result through a pointer in a0, increments that pointer, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_structure_list`: Saves the working and address registers, then walks a linked list of structures starting at a global table.
- `call_decoder_triple`: Saves the working and address registers, then calls the decoding routine three times with the values 1, 2 and 3 loaded into d7.
- `silence_audio_channel`: Writes zero to the audio data register at offset 170 of the custom chip four times using a 16-bit counter that starts at 48 and decreases by 16 each pass, then returns.
- `index_custom_table`: Loads the custom chip base address into a5, scales the 16-bit value in d7 by 64, adds that offset to a table base in a0, and places the 16-bit value from d6 into d0.
- `copy_table_until_marker`: Loads a byte from the table pointed to by a0 into the data register d0, advances the pointer in a0, and stores that byte at the address in a1, repeating until the loaded byte equals 255, otherwise it returns.
- `store_zero_bytes_and_load_table`: Writes the zero value held in register d0 to three consecutive bytes in memory, then loads the address of a data table into register a0.
- `extend_and_load_table`: Extends the 8-bit value in d2 to 16 bits, loads a table address from offset 993 into a1, and sign-extends the value back into d2.
- `accumulate_and_multiply`: Adds the 16-bit value at offset 8 of the structure in a0 to d2, stores that sum at offset 166 of the structure in a4, loads the 16-bit value at offset 36 of a0 into d0, and multiplies it by the 16-bit value at offset 38 of a0.
- `store_and_test_bit`: Stores the 16-bit value from d0 at offset 168 of the structure in a4, then tests bit 1 of the byte at offset 59 of the structure in a0.
- `decrement_byte_counter`: Decrements the byte at offset 59 of the structure in a0 by one, then returns.
- `set_flag_and_branch`: Sets bit 15 of the 16-bit value in d0, then branches to the routine at offset 216.
- `index_and_store_table`: Loads the 16-bit value at offset 20 of the structure in a0 into d0, copies the 16-bit value from d7 into d2, narrows d0 to a byte in d2, extends it to 16 bits, saves a5 into a4, sets bit d1 of the byte at the top of the stack, indexes a table with d0 to load a long into d1, stores that long at offset 160 of the structure in a4, then returns.
- `tiny_lsr_ori_ori`: Small routine of 3 instructions that shifts d1 right by 1, then merges 0 into d0, then merges 0 into d0.
- `invert_and_shift_word`: Inverts the 16-bit word at offset 30 in the structure pointed to by a0, loads it into d0, clears the low byte of d0, calls another routine, copies d0 into d2, shifts d0 right by six bits arithmetically, and returns.
- `empty_routine_2`: This routine is empty and does nothing.
- `store_and_call_loader`: Stores 65535 at offset 34 of the structure pointed to by a0, saves the 16-bit value in d1 into offset 150 of the structure in a5, then calls a routine that loads a longword pointer from the address in a0 and copies the byte at that pointer.
- `compare_byte_marker`: Compares the low byte of d0 against the value 252 and returns if they differ.
- `tiny_ext_ori_ori`: Small routine of 3 instructions that sign-extends d0, then merges 0 into d0, then merges 0 into d0.
- `compare_and_copy_byte`: Branches to a nearby routine if the zero flag is set, then compares the low byte of d0 against two marker values.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `add_byte_to_d0`: Adds the byte at offset 36 in the structure pointed to by a0 to d0, then falls through into the next routine.
- `store_d0_to_word`: Stores the value in d0 into the 16-bit word at offset 26 in the structure pointed to by a0, then falls through into the next routine.
- `compare_byte_and_branch`: Compares the low byte of d0 against a marker value and branches to a nearby routine if they match.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `tiny_cmpi_andi`: Small routine of 2 instructions that compares d0 with 128, then masks d0 with 63.
- `store_byte_and_compare`: Stores the low byte of d0 at offset 55 of the structure pointed to by a0, compares that byte against a marker value, and branches to a nearby routine if they match.
- `tiny_cmpi_cmpi`: Small routine of 2 instructions that compares d0 with 128, then compares d0 with 96.
- `maybe_branch_and_add_byte`: Branches to a nearby target if the carry flag is set, clears the low byte of d0 several times, then adds the byte at offset 58 in the structure pointed to by a0 to d0 before falling through into the next routine.
- `store_offsets_and_call_indexed`: Stores the byte in d0 into the byte at offset 57 of the structure in a0, decrements a1 by one, and stores the result into the word at offset 4 of that structure, then calls the indexed-pointer routine and copies the byte at offset 55 into the byte at offset 56.
- `scale_and_adjust_pointer`: Loads the 16-bit word at offset 26 in the structure pointed to by a0 into d0, computes two table addresses, scales d2 by sixteen after copying d7 into it, and adjusts a4 by that scaled amount.
- `load_and_index_byte`: Loads the byte at the indexed offset given by d0 into d3, then multiplies d3 by sixteen, then falls through into the next routine.
- `store_table_byte`: Loads a byte from the table indexed by the 8-bit value in d0 and stores the result into the byte at offset 12 of the structure in a1, then returns.
- `clear_word_at_offset`: Clears the 16-bit word at offset 10 in the structure pointed to by a0, clears the low byte of d0 twice, then falls through into the next routine.
- `store_negative_one_word`: Stores the value negative one into the 16-bit word at offset 42 in the structure pointed to by a0, then falls through into the next routine.
- `undecodable_bytes_21`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `branch_if_positive`: Branches to a nearby routine if the sign bit of d0 is clear, then clears the upper bits of d0.
- `increment_and_check_joystick`: Branches to a nearby routine if d0 is zero, then clears the upper bits of d0 and increments a counter in d1.
- `read_and_adjust_joystick_register`: Returns immediately if the value in d0 is zero, otherwise arithmetic-shifts d0 right by eight bits and returns if the result is less than or equal to zero.
- `decrement_byte_at_a0`: Decrements the byte stored at the address in a0 by one and then returns.
- `load_byte_from_pointer`: If the carry flag is set it returns, otherwise it loads a longword pointer from the address in a0 into a1 and copies the byte at that pointer into d0 while advancing the pointer by one.
- `set_bit7_if_below_128`: Compares the value in d0 with 128, sets bit 7 of that value, and returns if the original value was less than or equal to 128.
- `return_if_below_112`: Compares the value in d0 with 112 and returns if that value was below 112.
- `undecodable_bytes_22`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `double_d0_if_carry_clear`: If the carry flag is set it returns, otherwise it doubles the value in d0 and leaves the result there.
- `tiny_move_6`: Small routine of 1 instruction that copies a1 to (a0).
- `tiny_move_ori_3`: Small routine of 2 instructions that copies a1 to (a0), then merges 0 into d0.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `undecodable_bytes_23`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `process_character_buffer`: Reads a character index from a structure and compares it with a table entry to compute a signed difference, then repeatedly shifts a byte, updates several structure bytes, sets flags in a global variable, and advances a table pointer until the end marker is reached.
- `clear_flag_and_store_nibble`: Clears bit 5 of the byte at offset 48 in the structure pointed to by a0, loads the word at offset 26 into d1, reads the byte at the address formed by that word plus a2, masks the low nibble, stores it at offset 47 in the structure, and extends d1 to a word.
- `load_global_pointer`: Loads the address of a global variable into a2 and then returns by falling through to the next routine.
- `load_word_from_index`: Copies the word at the address formed by the value in d1 plus a2 into d3, then returns by falling through to the next routine.
- `tiny_swap_ori`: Small routine of 2 instructions that swaps the halves of d3, then merges 0 into d0.
- `test_flag_byte`: Tests the byte at offset 48 in the structure pointed to by a0 and then returns by falling through to the next routine.
- `tiny_neg_ori`: Small routine of 2 instructions that negates d3, then merges 0 into d0.
- `clear_upper_bits_d0`: Clears the upper bits of the word in d0 three times in a row after comparing it against zero and branching when it is nonzero, then returns.
- `clear_d0_upper_bits`: Compares the word in d0 against zero and branches when it is nonzero, then clears the upper bits of d0 three times in a row.
- `compare_word_and_branch`: Branches to a nearby routine if d0 is zero, then clears the upper bits of d0 three times.
- `conditionally_call_and_jump`: Calls the routine at offset 680, and if the carry flag is set it returns.
- `decrement_offset`: Reads the word at offset 26 of the structure in a0, indexes a table with it, adds one to the fetched byte, subtracts that from the word at offset 44, and stores the result there.
- `scale_and_store`: Multiplies the word at offset 40 of the structure in a0 by the value in d1, shifts the product right by nine bits, and stores the low word at offset 38.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_or_update_state`: Reads a pointer from offset 4 and a second pointer from offset 8 of the structure in a0, then initializes the second pointer's fields with fixed values and increments a counter at offset 12 when that counter is zero, otherwise it increments a counter at offset 18 and updates several fields.
- `initialize_or_update_state_word`: Reads a pointer from offset 4 and a word from offset 12 of the structure in a0, then initializes the second pointer's fields with fixed values when the counter is zero, otherwise it increments a counter at offset 20 and copies a word from a table indexed by the counter into a field.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_byte_via_pointer`: Reads a byte from a global variable and stores it through a pointer in a6, then returns.
- `store_byte_then_load_255`: Stores the 16-bit value from d0 at offset 26 of the structure in a0, then calls a routine that loads 255 into d0 and returns.
- `branch_into_routine`: Unconditionally branches forward by 228 bytes into the middle of another routine.
- `store_pointer_offset`: Stores the 32-bit value from a1 at offset 4 of the structure in a0, then returns.
- `lookup_and_store_byte`: Loads a byte from a table indexed by a value in a1, stores the result through a pointer in a0, increments that pointer, and returns.
- `read_ciaa_pra`: Reads the fire button and disk status bits from the CIA parallel port A register, then returns.
- `store_indexed_byte`: Stores -1 at offset 32 of the structure in a0, reads a byte from a table indexed by d0 into offset 52 of that structure, and returns.
- `clear_d6`: Clears the 16-bit value in d6, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `test_bit_61`: Tests bit 7 of the byte at offset 61 of the structure in a0 and returns.
