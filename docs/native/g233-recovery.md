# Game 233 (Agony v1.3 0960): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00233). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2604995 top-level bytes in 34 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 24 of 24 units identical (bytes and relocations), 77 raw-word lines |
| Functions named and described | pass, 287 of 287 named (0 author symbols), 287 of 287 described (63 by rule, 198 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3130 of 3289 (95.17%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5637 of 5893 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 23108 |
| code | 21670 |
| data | 2542297 |
| padding | 256 |
| unused | 17664 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Agony.Slave | 1 | 665 | 27 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1934 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 264 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 61 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 207 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 232 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 66 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1046 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 38 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 377 | 79 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 41 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 92 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 204 | 24 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 399 | 62 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 60 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 32 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 132 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- Agony_v1.3_0960/Agony/Disk.1: raw disk image (657408 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Agony_v1.3_0960/Agony/Disk.2: raw disk image (946176 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Agony_v1.3_0960/Agony/Disk.3: raw disk image (940032 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (play-rs-m1500): 948 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 208 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 49 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 61 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 213 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 39 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-mixed): 587 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 22 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 651 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 12 first-run image unit(s) in 2 version group(s); 5132 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- self-modifying code at an address (53 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (53 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (52 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (52 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- 3 packed track region(s), 2525952 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Agony.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `configure_custom_chip_enables`: The pseudo-code could not be produced, so the exact sequence is unconfirmed.
- `patch_jump_instructions_into_code`: When a global matches a fixed constant, writes absolute jump and subroutine call instructions at a series of code locations so they redirect to another routine.
- `write_jump_stub_at_pointer`: Makes a call and, if the result is nonzero, saves it in a global and makes a second call.
- `store_word_field_then_return`: Stores the value 32 into a 16-bit field at offset 156 from A6 and returns the value passed on the stack.
- `search_byte_sequence_in_buffer`: Slides a window across the first buffer up to the address in A1 and compares each position with the second buffer, counting consecutive matching bytes.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `poll_pot_port_button_latch`: Checks bit 6 of a global flag.
- `replace_patterns_with_trap_instructions`: Scans a memory range forward in steps of two bytes.
- `pass_through_d0`: Returns the value already in D0 without changing it and does no other work.
- `frag_move_andi_movea_rte`: Short code fragment that copies d0 to the stack, then masks d0 with an address, then copies d0 to a4, then copies the top of the stack to d0, then returns from the exception.
- `set_fields_on_marker_byte`: If the low byte of D0 equals the underscore character, it stores 20 in a 16-bit field and all ones in a byte field of the record addressed by A5, then returns.
- `set_fields_at_matching_record`: If the low byte of D0 equals the underscore character, it checks A5 against five fixed record addresses and, on a match, stores 20 in a 16-bit field and all ones in a byte field of that record.
- `copy_global_block_low_memory`: Copies 240 bytes one at a time from the global data area to the fixed address 512, then returns.
- `compare_shifted_code_with_global`: Shifts the low word of D0 right by one bit, XORs the low byte with 127 and compares it with a global byte.
- `halt_in_endless_loop`: Enters an endless empty loop and never returns.
- `set_frame_word_all_ones`: Stores all ones into a 16-bit field at offset 68 from A6 and returns.
- `install_fixed_address_jumps`: Writes absolute jump instructions with target addresses into several fixed locations, making one call between the writes.
- `load_or_save_global_block`: When the low byte of D1 is nonzero, copies 48 bytes from the buffer at A0 into the global data area and makes one call.
- `wait_for_flag_then_return`: Calls one unnamed routine and keeps its result.
- `forward_call_to_routine`: Calls one unnamed routine and returns, with no other work.
- `store_index_then_call_pointer`: Stores the character one plus a passed byte into a global, then calls through a function pointer at offset 76 from a stored base record.
- `xor_longs_with_key`: XORs each 32-bit longword of the buffer at A0 with the value in D7, stepping four bytes at a time for the count in D1.
- `decompress_lz_stream_into_buffer`: Decodes a compressed stream into an output buffer.
- `relay_single_call_unconfirmed`: The decompiler timed out, so the logic is unconfirmed.
- `load_scores_block_from_file`: Sets the stored signature word to a fixed value, then calls through a pointer at offset 36 with a count of 48 and the global data area as the buffer.
- `save_scores_block_to_file`: Calls through a pointer at offset 56 with a count of 48 and the global data area as the buffer, then returns the negated result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_registers_and_dispatch`: Restores registers from the stack, sets up two address registers, tests a bit in a custom chip register and branches, increments a global counter, calls a routine, and jumps to another address.
- `restore_and_update_state`: Restores registers from the stack and returns via rte, then branches on a custom chip register and updates several global variables, storing computed values into a table.
- `increment_counter_reset_on_match`: Increments a 16-bit counter in a global variable and returns immediately when it is not equal to one.
- `update_table_index_from_loaded_byte`: Loads a byte from a table indexed by a 16-bit value in a register and compares it with a stored byte in a global variable, branching past the update when the loaded byte is smaller.
- `process_keyboard_input`: Reads a keyboard scancode from the CIA serial register and, if the key is not currently held, stores its decoded value in a global byte then calls a game-input handler.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_structure_list`: Walks a linked list of structures starting at the global table at address 42400, and for each non-empty entry copies its data into a second table at address 42376 while recording its size and offset.
- `call_decoder_triple`: Saves the working and address registers, then calls the decoding routine three times with the values 1, 2 and 3 loaded into d7.
- `jump_to_silence_audio`: Jumps to the routine at offset 186, which zeroes the audio register at offset 170 in a loop that repeats four times before returning.
- `silence_audio_channel`: Writes zero to the audio register at offset 170 in a loop that repeats four times with a 16-bit counter before returning.
- `initialize_structure_table`: Builds and initializes a structure at a table offset computed from d7, copying a 32-bit value, setting several word and byte fields, and clearing additional fields before returning.
- `store_scaled_word`: Multiplies d7 by 64 to compute an offset, indexes a table with it, stores the 16-bit value from d6 at offset 16, and returns.
- `decode_length_prefixed_buffer_2`: Multiplies the word in d7 by 64 and uses the result to index a custom chip structure, then stores the sum of two words at fixed offsets into a DMA register pair and the difference of two words into another register pair.
- `decode_length_prefixed_buffer`: Decodes a length-prefixed byte sequence from a buffer pointer stored at offset 4 of the structure in a0, writing the decoded values into fields at offsets 18, 31, 32, 37, 38, and 56, then calls another routine to continue decoding.
- `compute_index_from_d2`: Loads a word from offset 26 of the structure in a0 into d0, then loads two table base addresses and computes an index from d2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_conditional_byte`: If the value in d0 with bit 0, bit 2 and bit 4 set is less than 96 it stores the sum of the byte at offset 58 and that value into the byte at offset 57 of the structure in a0, otherwise it decrements the value in a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_byte_and_call_cia`: Stores a byte from register d0 at offset 57 of the structure in a0, saves register a1 at offset 4, calls update_structure_cia, then copies the byte at offset 55 to offset 56 of the same structure.
- `update_structure_cia`: Reads a word at offset 26 of the structure in a0, indexes a table with it, and stores a computed value and two words at offsets 20 and 160 of that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_structure_counter`: Initializes a 16-bit counter at offset 6 of the structure pointed to by a0 and a byte at offset 2 to 200, then increments a counter at offset 18 of that structure and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_cia_pra`: Writes the byte in d0 into the CIAA_PRA register and returns.
- `decode_length_sequence`: Decrements the byte at offset 51 of the structure in a0, and when it becomes negative reads a length-delimited sequence from a pointer at offset 4, decoding several byte ranges into offsets 58, 54, 53, and 36 of the structure.
- `update_entity_state`: Updates a per-entity state in the structure in a0: it derives an index from offset 26, looks up a table to set the byte at offset 52 and adjusts the value at offset 32, then decrements a counter at offsets 16 and 18 and, when a threshold is crossed, modifies the value at offset 8 using a global table and the bytes at offsets 49, 50 and 57.
- `decrement_offset_44`: Reads a word at offset 26 of the structure in a0, indexes a table with it, adds one to the fetched byte, subtracts that from the word at offset 44, and stores the result there.
- `scale_and_store`: Multiplies the word at offset 40 of the structure in a0 by the value in d1, shifts the product right by nine bits, and stores the low word at offset 38.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_structure_slots`: Initializes a 12-slot structure in a global array with a pointer, a code, and a marker byte, then repeatedly reads a value from a source and stores it into the next empty slot while calling a system trap to notify the change.
- `process_timer_counter`: Clears the bit selected by a value in d0 in a byte pointed to by a6, then clears the byte at offset 436 in the memory at a0.
- `init_struct_with_counters`: Initializes a structure pointed to by a2 with a base value and several fields copied from a structure pointed to by a1, increments a counter at offset 12 of the structure pointed to by a0, and clears a flag there.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_cia_and_copy_block`: Clears two CIA port registers and configures the serial and timer control registers, then copies a block of longwords from a source table to a RAM region until the source pointer reaches its end.
- `store_and_jump`: Stores the 16-bit value in d6 into a global variable, then jumps to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_audio_interrupt`: It waits until a custom register's bits match a target, clears two DMA-related registers, stores an interrupt handler address, and zeroes a 32-byte table.
- `multiply_nibbles_and_pack`: Calls a helper routine, then repeatedly multiplies each nibble of a source word by a shift factor and packs the results into a destination word, looping over the data and over the factor until the factor reaches seventeen.
- `decrement_and_multiply_nibbles`: Decrements a global counter, calls a helper routine, then repeatedly multiplies each nibble of a source word by the factor and packs the results into a destination word, looping over the data and over the factor until the factor reaches zero.
- `check_zero_global`: Clears a global variable, compares it with zero, and returns if it is not zero; otherwise it falls through to a lone return.
- `compare_zero_register`: Clears a global variable, compares it with a 16-bit value in d0, and returns if they differ.
- `fill_table_with_value`: Fills five consecutive 32-bit slots in a table with the two halves of a 32-bit value from d0, advancing the table by 8 bytes per slot and adding a stride to the value each iteration.
- `init_audio_dma_channels`: It saves registers, increments a global counter and returns immediately when that counter wraps to zero.
- `store_and_jump_2`: Stores the 16-bit value in d6 into a global variable and then jumps to another routine, returning control from there.
- `reset_audio_dma_state`: Locates a byte in a table, computes an offset from its position, clears a block of memory, stores pointers into a second table, and clears several audio volume and DMACON registers before returning.
- `handle_sound_command_byte`: It saves registers on the stack, increments a byte counter at offset 99 of a global structure, compares a byte from a table with a neighboring byte, and returns early when they match.
- `process_four_channel_volume`: For each of four audio channels it resets a shared counter, calls a sound routine to load a new value, then scales that value by subtracting 64 and adding a shared amount, clamps the result to zero when negative, and stores the 16-bit result at offset 8 of the custom audio register block.
- `process_sound_command`: Loads a structure entry from a table using a command byte, stores its fields into the structure, then dispatches to a sound routine based on the command and updates the audio channel lengths.
- `dispatch_structure_byte_handler_2`: Reads the low nibble of the byte at offset 2 of the structure in a6 and dispatches to a handler based on its value, scaling a value from a 74-entry table, adjusting a 16-bit field at offset 16 by a signed nibble at offset 23, decrementing or incrementing that field with clamping, or moving a byte between offsets 3 and 23, otherwise it returns.
- `dispatch_low_nibble_handler`: Reads the low nibble of the byte at offset 2 of the structure in a6 and dispatches to a handler based on its value, scaling a value from a 7.
- `lookup_and_store_value`: Searches a table of 74 words using a nibble from offset 9 of the structure in a6, adjusting the found index when a flag bit is set and the index is nonzero.
- `lookup_and_scale_structure_2`: Adjusts a 16-bit value at offset 16 of the structure in a6 by a signed nibble from offset 23, clamping it to another 16-bit field and clearing the source nibble.
- `lookup_and_scale_structure`: Looks up a value in a table from a structure byte, scales it by another nibble, adjusts a signed result, stores it into a second structure, and increments a field, returning early when a condition is met.
- `dispatch_structure_byte_handler`: Reads a byte from offset 2 of the structure in a6, masks its low nibble, and dispatches to a handler based on that value (9 through 15), updating a global variable and the fire-button bit of CIA port A. For the upper nibble of the byte at offset 3 it sets up a palette index, toggles a sprite flag, or writes a low nibble into a second structure.
- `store_low_nibble`: Extracts the low nibble of the byte in d0 and stores it at offset 18 of the structure in a6, then returns.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `disable_and_call_boot`: Disables DMA and interrupts, clears a global variable, calls two other routines, and jumps to a fixed address.
- `configure_custom_and_divide`: Writes a value to a global variable at offset 66adc, then reconfigures a custom chip register block by XORing a copied base address with a constant and storing the result at offset 66920.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_return_only`: Contains only a return instruction and is entered from outside the static flow in the reference run.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `load_address_and_modify_word`: Loads the address of a global variable into a pointer register, reads a word from offset 30 of the structure in a6 into a data register, and sets bit 5 of that value before falling through to the next routine.
- `call_then_jump`: Calls a routine at the given address and then jumps to another routine at a different address.
- `write_word_and_increment_counter`: Writes the word value 32768 to offset 150 of the structure in a6, increments a word at the global variable offset 6bac in a5, and executes two no-op operations on a data register before falling through.
- `conditional_branch_then_nop`: Branches to a nearby routine when the condition code is greater or equal, then executes three no-op operations on a data register before falling through to the next routine.
- `branch_if_equal_then_nop`: Branches to a nearby target when the condition code is equal, then executes a no-op operation on a data register before falling through to the next routine.
- `tiny_move`: Small routine of 1 instruction that copies d0 to (a0)+.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `shift_interrupt_enable`: Reads the current value of the interrupt enable register, shifts it right by one bit, and stores the resulting 16-bit value into a global variable.
- `conditional_branch_then_nop_return`: Branches to a distant target when the condition code is greater or equal, then executes a no-op operation on a data register before falling through to the next routine.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `branch_if_less_or_equal`: Branches to a distant target when the condition code is less or equal, then falls through to the next routine.
- `decrement_global_word`: Decrements a word at the global variable offset 6c1a in a5 by three, then falls through to the next routine.
- `test_bit_and_clear_d0`: Tests bit 0 of the byte in data register d2 and then clears the low byte of data register d0 with two no-op or-operations.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `test_bit_and_clear_d0_b`: Tests bit 0 of the byte in data register d4 and then clears the low byte of data register d0 with two no-op or-operations.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `tiny_addi`: Small routine of 1 instruction that adds 37 to d1.
- `tiny_addi_2`: Small routine of 1 instruction that adds 32 to d0.
- `tiny_addi_3`: Small routine of 1 instruction that adds 16 to d1.
- `tiny_tst_ori_ori`: Small routine of 3 instructions that tests d6, then merges 0 into d0, then merges 0 into d0.
- `branch_if_equal_then_clear`: Branches to the instruction at offset 20 when the condition code is equal, then clears the low byte of data register d0 with four no-op or-operations.
- `check_zero_flag`: Branches conditionally on the zero flag and clears the zero flag in a register before returning.
- `increment_structure_offset_108`: Adds the constant 8 to the 16-bit value stored at offset 108 of the structure pointed to by address register a5.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `copy_a0_to_a1_a2`: Copies the address in register a0 into registers a1 and a2, then clears the low byte of data register d0 with two no-op or-operations.
- `load_word_and_increment_a0`: Loads a 16-bit word from the address in register a0 into data register d6 while post-incrementing that register, then clears the low byte of data register d0 with five no-op or-operations.
- `set_bit_and_shift_d0`: Loads the constant 0 into data register d3, clears the low byte of data register d0, sets bit 0 of that byte, and shifts the value left by 8 bits.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `store_long_and_set_bit_d0`: Stores the 32-bit value in data register d0 at the address in register a2 while post-incrementing that register, sets bit 3 of the low byte of data register d0, and clears its high byte.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to (a2)+.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `branch_if_not_equal_then_copy`: Branches to the instruction at offset 40 when the condition code is not equal, copies data register d1 into data register d4, then clears the low byte of data register d0 with three no-op or-operations.
- `store_and_call_flagged`: Stores a value into a location pointed to by a2, checks a flag in a global variable, stores a constant at offset 3336 of a structure in a5, calls another routine, and branches.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `clear_d0_and_branch_conditionally`: Clears the d0 register multiple times and branches if the zero flag is not set; otherwise, it clears the d0 register.
- `store_const_call_and_branch`: Stores the constant 50 at offset 3336 of the structure pointed to by register a5, calls the routine at offset 6036, branches to the instruction at offset 818 when the condition code is equal, and clears the low byte of data register d0 with two no-op or-operations.
- `store_status_if_flag_set`: Branches to the label at 884 when the zero flag is set, otherwise continues.
- `tiny_move_3`: Small routine of 1 instruction that copies d1 to (a3)+.
- `clear_global_if_flag_set`: Branches to the label at 1030 when the zero flag is set, otherwise continues.
- `divide_and_store_quotient`: Copies the value of d1 into d6, divides the 16-bit value at offset 6c1c of the structure in a5 by the value of d0, and stores the quotient into d3.
- `tiny_subi`: Small routine of 1 instruction that subtracts 160 from d0.
- `load_next_table_byte`: Loads a byte from the table pointed to by a0 into the data register d0, advances the pointer in a0 by one byte, and returns.
- `store_or_result`: Combines the values of d1 and d0 with a bitwise OR and stores the 16-bit result through the pointer in a0.
- `set_high_bits_of_byte_2`: Clears bits 5, 6 and 7 of the byte at offset 27714 of the structure in a5 by ORing it with 224, branches when the zero flag is set, then sets bits 0, 5 and 6 of register d1.
- `tiny_addi_4`: Small routine of 1 instruction that adds 200 to d4.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `set_high_bits_of_byte`: Sets bits 5, 6 and 7 of the byte pointed to by a0 by ORing it with the constant 224, then returns.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `load_byte_from_array`: Loads the byte at the offset given by d0 within the array pointed to by a0 into d1.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `load_data_if_sign_clear`: Branches to the label at 1498 when the sign flag is clear, otherwise continues.
- `copy_table_until_marker`: Loads a byte from the table pointed to by a0 into the data register, advances the pointer, and stores the value through the pointer in a6, repeating until the loaded byte equals 255, otherwise it returns.
- `test_flag_byte`: Tests the 16-bit value at offset 6c5e of the structure in a5, then returns.
- `tiny_moveq_ori`: Small routine of 2 instructions that loads 0 into d6, then merges 0 into d0.
- `check_vblank_flag`: Reads the vertical blank interrupt flag register and shifts bit 0 of its low byte into the carry flag, then branches past the check when that flag is clear.
- `test_flag_offset_6c66`: Tests the 16-bit value stored at offset 6c66 of the structure in a5 and leaves the condition codes set from that value.
- `test_flag_offset_6c68`: Tests the 16-bit value stored at offset 6c68 of the structure in a5 and leaves the condition codes set from that value.
- `clear_flag_offset_6d12`: Clears the 16-bit value stored at offset 6d12 of the structure in a5 to zero.
- `load_flag_offset_6c64`: Loads the 16-bit value at offset 6c64 of the structure in a5 into d0 and then performs a no-op or on d0.
- `tiny_tst_ori`: Small routine of 2 instructions that tests d0, then merges 0 into d0.
- `store_value_offset_6c6a`: Stores the 16-bit value in d0 into offset 6c6a of the structure in a5.
- `store_value_offset_6c54`: Stores the 16-bit value in d0 into offset 6c54 of the structure in a5, then performs a no-op or on d0.
- `conditionally_or_byte`: Reads a byte from a global variable, ORs it with the constant 109, and if the resulting signed byte is negative it calls another routine before returning.
- `clr_word_then_call`: Clears the word at offset 27752 of the structure in a5, sets bits 2 of register d0, then calls the routine at offset 1722.
- `compute_address_and_increment`: Computes an address by subtracting 17352 from a5 into a0, sets bit 6 of the byte at offset 0 of a3, then adds 15 to d1.
- `tiny_lsl`: Small routine of 1 instruction that shifts d1 left by 8.
- `combine_values_and_store`: Combines d1 into d0 with a bitwise or, sets bit 6 of d0, calls the following routine, then stores d0 into offset 6ce0 of the structure in a5.
- `store_long_a0_global`: Stores the long value in a0 into a global variable at offset 2052 of a5, then sets bits 0, 1, 3, 4, 5 and 6 of the status byte in d0 and returns.
- `tiny_addq_ori`: Small routine of 2 instructions that adds 4 to a1, then merges 0 into d0.
- `load_seven_test_increment`: Loads the constant 7 into d0, then tests and post-increments the word at the address in a3, then clears the low bits of d0 and returns.
- `load_table_base_a2`: Loads the address of a table 14140 bytes below a5 into a2, then clears the low bits of d0 and sets bit 1 of d0 before returning.
- `copy_long_advance_pointers`: Copies the long word from the source in a2 to the destination in a0, advancing both pointers, then loads the long word at offset 11059 of a1 into d0.
- `store_byte_from_pointer`: Reads a byte from the memory location pointed to by a0 and stores it into a byte at offset 1 of the structure in a6, then returns.
- `tiny_move_ori`: Small routine of 2 instructions that copies 1 to d0, then merges 0 into d0.
- `store_word_d1_global`: Stores the word in d1 into a global variable at offset 17494 of a5, then clears the low bits of d0 nine times before returning.
- `store_long_d0_preincrement`: Stores the long word in d0 into the destination in a0 and advances that pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_word_one_global`: Compares the word 1 against a global variable at offset 17514 of a5, and if they differ it branches to an earlier instruction of another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_word_two_global`: Compares the word 2 against a global variable at offset 17514 of a5 and branches away if they differ, clears the low bits of d0, sets bits 1 through 6 of d0, loads the constant 1 into d3, compares the word 3 against the same global variable and branches away if they differ.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_word_and_store`: Compares the word at offset 27754 of the structure in a5 against 4 and branches when unequal, then against 5 and 6, writing the constant 143 to offset 446 of the structure in a6 and clearing the word at offset 27830 when the value is 6.
- `tiny_movea_movea_ori`: Small routine of 3 instructions that copies a0 to a1, then copies a0 to a2, then merges 0 into d0.
- `tiny_move_4`: Small routine of 1 instruction that copies (a0)+ to d6.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `shift_d1_right_d2_left`: Shifts the word in d1 right by one bit, clears the low bits of d0 several times, shifts the word in d2 left by eight bits and clears the low bits of d0 twice more.
- `or_and_add_stack_byte`: Oresets register d0, sets bit 7 of the byte in d4, adds the word pointed to by a5 to d3 while advancing that pointer, pops a byte off the stack into memory at offset -128 of a2, and returns if that byte is nonzero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `conditionally_store_a5_state`: If the byte at offset 6c6b of the structure in a5 equals -1, it stores the 16-bit value from a5 at offset 67b5 of the structure pointed to by a0 and stores a0 at offset 367c of the structure in a5, otherwise it does nothing.
- `branch_if_zero_then_clear`: Branches to a nearby instruction when the zero flag is set, then clears register d0 four times.
- `branch_if_not_zero_then_clear`: Branches to a nearby instruction when the zero flag is clear, then clears register d0 four times.
- `store_and_clear_words`: Stores the value 65535 at offset 28304 of the structure in a5, clears the word at offset 27846 of that structure, and resets register d0 twice before returning.
- `tiny_moveq`: Small routine of 1 instruction that loads 7 into d0.
- `write_table_and_load_word`: Writes a sequence of eight immediate word and longword values into consecutive offsets of the structure in a6, then reads a word from offset 31770 of the structure in a5 into register d3.
- `load_address_and_reset`: Loads the address of the location at offset 4420 of the structure in a5 into register a1, then resets register d0 twice before returning.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `increment_pointer_value`: Increments the longword pointed to by register a1 by two and returns.
- `tiny_cmp`: Small routine of 1 instruction that compares d1 with 128.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `tiny_lsl_move_ori`: Small routine of 3 instructions that shifts d2 left by 8, then copies d2 to d5, then merges 0 into d0.
- `test_bit_and_adjust_stack`: Oresets register d2 with d1, resets register d0 twice, tests bit d1 of the memory location pointed to by a4 while advancing that pointer, and subtracts 256 from the value at the top of the stack.
- `conditionally_clear_d0`: Branches to a later routine when the zero flag is set, otherwise executes an instruction that clears the low byte of register d0.
- `load_address_set_clear_bits`: Loads the address at offset -16126 of the structure in a5 into register a0, resets register d0 eight times, sets bit d0 of the memory location pointed to by the stack pointer, and clears bit d0 of register d7.
- `compare_and_clear_globals`: Skips ahead when the positive flag is set, compares register d2 with d0, tests a global word, and if they differ it clears that global word and the word at offset 27846 of the structure in a5 before returning.
- `store_constants_and_load_address`: Stores the value 3 at offset 27702 of the structure in a5, stores the value 50 at offset 27760 of that structure, loads the address at offset -17524 of that structure into register a0, and resets register d0 twice before returning.
- `test_byte_offset`: Tests the byte at offset 6c6b of the structure in a5 and returns.
- `branch_if_less_equal`: Branches to the target when the value in d0 is less than or equal to zero, otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_word_two_then_clear`: Compares the word at offset 27754 of the structure in a5 against 2 and branches when equal, then clears register d0 four times.
- `shifted_copy_until_zero`: Reads a byte from the address in a0 and stores it at offset 3 of that address, then advances a0 by 4 bytes and repeats until the byte at offset 1 of a0 is zero, otherwise it returns.
- `loop_store_shifted_bytes`: Loops over a block starting at a0, storing a shifted value at offset -3 and a single bit at offset -1 of that pointer, then advances the pointer by 32 and decrements a counter until it reaches 65535.
- `store_word_d0`: Stores the 16-bit value in d0 at the address in a0 and then returns.
- `store_word_and_clear_bytes`: Stores the 16-bit value in d0 to the word pointed to by a0, then writes a zero byte to each of the three consecutive bytes at offsets 0, 1 and 2 of the byte pointed to by d0.
- `tiny_move_ori_2`: Small routine of 2 instructions that copies d0 to (a0), then merges 0 into d0.
- `tiny_move_5`: Small routine of 1 instruction that copies d0 to (a0).
- `store_word_and_clear_d0`: Writes the 16-bit value in d0 to the memory location pointed to by a0, then clears the low byte of d0 three times in a row.
- `store_word_and_nop`: Stores the 16-bit value in d0 to the word pointed to by a0, then writes three zero bytes to d0 via ori.b, which has no effect on its value.
- `store_word_clear_d0`: Stores the 16-bit value in d0 to the memory location pointed to by a0, then clears d0 by OR-ing it with zero three times in a row.
- `store_word_clear_low_byte`: Stores the 16-bit value in d0 to the memory location pointed to by a0, then clears the low byte of d0.
- `store_word_d2_struct`: Stores the 16-bit value in d2 at offset 4dae of the structure in a5 and then returns.
- `restore_regs_and_return`: Restores the caller's registers from the stack and returns via the return address left there.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `clear_struct_fields`: Clears two 16-bit fields of the structure in a5 and sets two 16-bit values at offsets 154 and 156 of the structure in a6, then pops its registers and continues into the next routine.
- `copy_word_and_store_128`: Copies the word at offset 27908 of the structure in a5 into register d0, subtracts 17528 from that pointer, compares register d1 with the byte at offset 27916, branches when d1 is lower, then writes the constant 128 to offset 154 of the structure in a6.
- `copy_struct_word`: Copies a 16-bit value from offset 2006 of the structure in a5 to offset 2004 of the same structure, then returns.
- `push_regs_point_custom`: Stores the contents of registers d0-d7 and a0-a6 onto the stack, points a6 at the custom hardware base, then returns.
- `mask_and_store_d0`: Toggles the low seven bits of d0, stores d0 at offset 17511 of the structure in a5, sets bits 6 and 7 of d0, and masks d0 with the word at offset 4 of a6, then continues into the next routine.
- `compare_offset_6_byte`: Loads a byte at offset 6 of the structure in a6 into d0, builds a shifted value from the same byte in d1, and compares the two values, then continues into the next routine.
- `compare_shifted_d0`: Sets d0 to one, masks it with the word at offset 4 of a6, shifts it left by eight bits, and compares d1 with d0, then continues into the next routine.
- `check_cia_port_below_64`: Clears the lower byte of the CIA port at offset a fixed address, then repeatedly reads that port into d0 and compares d0 against 64, branching while the condition code is not zero.
- `test_d0_and_global_1d8`: Returns immediately if the zero flag is set, then compares d0 against 55 and tests the 16-bit value at the global variable at offset 472, otherwise it returns.
- `unlisted_routine`: A longer routine entered from outside the static flow and called by two other routines; its behavior is not described by the available instruction listing.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_custom_table_from_data`: Saves the registers, points a5 at the custom hardware base and a0 at a table, then loops over entries that start with the 32-bit value an address, copying and adjusting their fields into a second table.
- `call_jump_routine_three_times`: Saves the registers, then calls the jump routine three times with d7 set to 1, 2 and 3 in turn, and finally restores the registers and returns.
- `loop_clear_custom_register`: Points a0 at a data table, points a5 at the custom hardware base, writes zero to the register at offset 170, decrements a counter by sixteen in a loop while it stays below sixteen, and returns.
- `scale_and_index_table`: Points a5 at the custom hardware base, scales d7 by sixty-four, indexes into the table pointed to by a0 with that value, and loads d6 into d0, then continues into the next routine.
- `iterate_table_bytes`: Loads a byte from the table pointed to by a0 into a1, increments a0, and returns when the byte is zero, otherwise it continues with the next entry.
- `clear_d0_load_global`: It clears the low byte of d0 three times in a row, loads the address of a global variable into a0, then clears the low byte of d0 two more times before returning.
- `extend_d0_negate_byte`: It sign-extends the byte in d2 into a word, loads the address of a global table into a1, then sign-extends d2 again and negates its low byte before returning.
- `add_store_multiply_words`: It adds the word at offset 8 of the structure in a0 to d2, stores d2 as a word at offset 166 of the structure in a4, loads the word at offset 36 of a0 into d0, then multiplies d0 by the word at offset 38 of a0.
- `store_word_test_bit`: It stores the word in d0 at offset 168 of the structure in a4, then tests bit 1 of the byte at offset 59 of the structure in a0 before returning.
- `decrement_byte_clear_d0`: It decrements the byte at offset 59 of the structure in a0 by one, then clears the low byte of d0 two times in a row before returning.
- `set_bit15_branch_lsr`: It sets bit 15 of d0, clears the low byte of d0, then branches to the routine that merges a shifted value into d0.
- `index_table_store_long`: It loads the word at offset 20 of the structure in a0 into d0, loads the address of a global table into a1, copies d7 into d2, moves d0 into d2, sign-extends d2, saves a5 into a4, pushes a decremented byte from a0, indexes the table with d0 to load a long into d1, stores that long at offset 160 of the structure in a4, then clears the low byte of d0 two times before returning.
- `tiny_lsr_ori_ori`: Small routine of 3 instructions that shifts d1 right by 1, then merges 0 into d0, then merges 0 into d0.
- `invert_word_shift_result`: It bitwise-inverts the word at offset 48 of the structure in a0, loads that word into d0, calls the routine that sign-extends and arithmetic-shifts d0, stores the result into d2, arithmetic-shifts d0 right by six, then returns.
- `store_byte_write_ffff`: It reads a byte from one address below a0 and stores it into a global variable, writes the value 65535 as a word at offset 33 of the structure in a0, stores the incoming word in d1 at offset 150 of the structure in a5, then calls another routine and returns its byte result.
- `init_structures_and_load_byte`: Stores the value -1 into the word at offset 34 of the structure in a0, copies the low byte of d1 into the byte at offset 150 of the structure in a5, then calls the byte-loading routine and returns.
- `compare_byte_252_2`: Compares the byte in d0 against the value 252 and returns without changing anything when the two are unequal.
- `tiny_ext_ori_ori`: Small routine of 3 instructions that sign-extends d0, then merges 0 into d0, then merges 0 into d0.
- `compare_and_copy_byte`: Branches when the zero flag is set, compares a register against two marker values, copies a byte from a source pointer to a destination pointer, and otherwise returns.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `add_byte_from_struct`: Adds the byte at offset 54 of the structure in a0 to the byte in d0 and returns.
- `store_word_to_struct`: Stores the word in d0 into the word at offset 26 of the structure in a0 and returns.
- `compare_marker_value`: Compares a register against a marker value and branches when they match, otherwise it returns.
- `undecodable_bytes_21`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `tiny_cmpi_andi`: Small routine of 2 instructions that compares d0 with 128, then masks d0 with 63.
- `store_byte_and_compare`: Stores a register into a structure field, compares the register against a marker value, and branches when they match, otherwise it returns.
- `tiny_cmpi_cmpi`: Small routine of 2 instructions that compares d0 with 128, then compares d0 with 96.
- `maybe_add_byte_from_struct`: Branches away when the carry flag is set, clears d0 three times, then adds the byte at offset 58 of the structure in a0 to d0 and returns.
- `store_offsets_and_call_indexed`: Stores d0 into the byte at offset 57 of the structure in a0, decrements a1 by one and stores it into the word at offset 4, then calls the indexed-pointer routine and copies the byte at offset 55 into the byte at offset 56.
- `compute_indexed_pointer`: Loads the word at offset 26 of the structure in a0 into d0, sets a1 and a2 to two table addresses, scales d2 by sixteen after doubling d7, and adds the scaled value to a5 copied into a4.
- `load_and_scale_byte`: Copies the byte at offset 7 indexed by d0 in a1 into d3, then scales d3 by four and returns.
- `copy_byte_via_pointers`: Loads the byte stored at the address held in a0 into the lower byte of d0, then stores that byte through the address held in a1, and returns.
- `clear_word_at_offset`: Clears the word at offset 10 of the structure in a0, clears d0 twice, and returns.
- `store_negative_one`: Stores the value negative one into the word at offset 42 of the structure in a0 and returns.
- `undecodable_bytes_22`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `test_sign_bit`: Tests the sign bit of a value and returns when it is clear, otherwise falling through into the following routine.
- `index_table_and_test_bit`: Branches when the zero flag is set, increments a counter, and tests a bit in a table indexed by the counter, branching when that bit is clear, otherwise it returns.
- `read_joystick_and_port`: Returns if the zero flag is set, arithmetic-shifts d0 right by 8 bits, returns if the result is less than or equal to zero, then decrements the byte at the address in a0 indexed by d0.
- `decrement_byte_at_offset`: Decrements the byte at offset 53 of the structure in a0 by one and returns.
- `load_byte_from_pointer`: Branches away when the carry flag is set, loads the pointer stored at a0 into a1, copies the byte it points to into d0, advances that pointer by one, and returns.
- `mask_and_set_byte_d0`: Clears bit 7 of the byte in d0 and then sets bits 0, 1, 3 and 4 of that same byte, leaving the result there.
- `compare_byte_d0_with_112`: Compares the byte in d0 with 112 and branches to another routine when the value is less than 112, otherwise it returns.
- `undecodable_bytes_23`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `double_byte_d0_conditional`: Branches to another routine when the carry flag is set, otherwise it doubles the value in d0 and then returns.
- `tiny_move_6`: Small routine of 1 instruction that copies a1 to (a0).
- `tiny_move_ori_3`: Small routine of 2 instructions that copies a1 to (a0), then merges 0 into d0.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `undecodable_bytes_24`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `process_string_flags`: Reads a length byte from offset 26 of the structure in a0 and, for each character in the null-terminated string pointed to by a1, sets a flag byte at offset 40 of that structure and a status byte at offset 46, then decrements the 16-bit counter at offset 34 until the string is exhausted.
- `mask_byte_table_lookup`: Clears bit 5 of the byte at offset 48 in the structure pointed to by a0, loads the 16-bit value at offset 26 into d1, indexes a byte table with it, masks the low nibble, stores the result at offset 47, and extends d1 to 16 bits before returning.
- `load_table_address`: Loads the address of a global table into register a2 and returns.
- `load_indexed_word`: Reads a 16-bit value from the table in a2 indexed by d1 into d3, then clears the condition codes before returning.
- `tiny_swap_ori`: Small routine of 2 instructions that swaps the halves of d3, then merges 0 into d0.
- `test_status_byte`: Tests the byte at offset 48 of the structure in a0 and returns.
- `tiny_neg_ori`: Small routine of 2 instructions that negates d3, then merges 0 into d0.
- `store_byte_if_not_252_2`: If the byte in d0 equals 252, it stores that byte through the pointer in a1, otherwise it returns without modifying anything.
- `store_byte_if_not_252`: Compares the byte in d0 with the value 252 and returns without modifying anything when the two differ, otherwise it stores the byte from d0 through the pointer in a0.
- `compare_byte_252`: Compares the byte in d0 against the value 252 and returns without modifying anything when the two differ.
- `decrement_and_scale_words`: Calls the word-decrementing routine, returns if the carry flag is set, calls the word-multiplication routine, and jumps to the routine at offset 1662.
- `decrement_word_by_byte`: Reads a byte from a table indexed by a 16-bit value, increments it by one, subtracts it from the 16-bit value at offset 44, returns if the subtraction borrows, and otherwise stores the result back before returning.
- `scale_word_multiply`: Multiplies the 16-bit value at offset 40 by d1, shifts the product right by nine bits, stores the low 15 bits into the 16-bit value at offset 38, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_node_list`: Initializes a linked list of nodes from a table when the counter is zero, otherwise updates node fields by comparing distances, inserting entries, and advancing pointers before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_or_advance_structure`: Reads a flag word and, when it is zero, initializes a structure at a2 with a base address and a pointer, then increments a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_zero_byte`: Reads a byte from a global variable and, if that byte is zero, stores a zero byte back to the same global variable through the pointer, otherwise it returns.
- `store_value_in_d0`: Stores the 16-bit value passed in d0 into offset 26 of the structure in a0, then calls a helper that loads 255 into d0 and returns.
- `branch_into_routine`: Unconditionally branches forward by 228 bytes into the middle of another routine.
- `store_a1_into_structure`: Stores the contents of a1 into offset 4 of the structure in a0, then falls through into the following routine.
- `load_byte_from_table`: Loads a byte from the table addressed by a0 into a1, increments a0, and returns.
- `access_cia_parallel_port`: Reads and writes the CIA parallel port register that holds the joystick fire buttons and disk status, then returns.
- `index_byte_from_table`: Writes the value 65535 to offset 32 of the structure in a0, then reads a byte from a table indexed by d0 into offset 51 of that structure and falls through into the next routine.
- `clear_d6_and_branch`: Clears the 16-bit register d6, then branches by 712 bytes when d0 is zero and otherwise falls through into the following routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `test_bit_byte`: Tests bit 7 of the byte at offset 61 in the structure pointed to by a0 and returns.
