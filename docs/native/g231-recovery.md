# Game 231 (AfterTheWar v1.1 1084): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00231). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (981561 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 35 raw-word lines |
| Functions named and described | pass, 215 of 215 named (0 author symbols), 215 of 215 described (23 by rule, 139 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4777 of 4789 (99.75%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 17186 of 17186 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 24603 |
| code | 67504 |
| data | 888929 |
| padding | 13 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| AfterTheWar.Slave | 1 | 102 | 13 |
| Disk.1:bootblock | 1 | 21 | 1 |
| logged disk load from Disk.1 | 1 | 455 | 34 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16308 | 151 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 298 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |

## Cases handled

- AfterTheWar_v1.1_1084/AfterTheWar/Disk.1: custom-format (non-DOS) disk, 1757 non-empty blocks: matched against track-loaded segments
- 2 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address (play-rs-m1500): 4450 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 270 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- AfterTheWar_v1.1_1084/AfterTheWar/Disk.1: 2 logged loads cover 14336 bytes of the image
- AfterTheWar_v1.1_1084/AfterTheWar/Disk.1: blocks 1737-1759 (bytes 889344-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.52)
- 1 packed track region(s), 873472 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `seek_track_and_copy_sectors` (Disk.1 +an address loaded at an address (12288 bytes)): Divides a value by 22 and by 11 to get a track and sector position, steps the disk drive head through the CIA-B port the required number of times, and sets the disk length register. Kind: track/sector validation and manual seek loop. Pass path: The routine performs complex arithmetic on input values (likely sector/track indices) to calculate required motor/head movements. It uses a loop to copy data from a specific memory offset (an address) and validates the sector count. A successful.
- `copy_disk_sectors_into_buffer_2` (Disk.1 +an address loaded at an address (12288 bytes)): Sets a control bit, steps through a run of sectors copying 128 longs for each one, and toggles a side flag at each track boundary. Kind: disk/track check and data loading loop. Pass path: The loop continues to load sectors (puVar5/puVar7) until sVar3 (sector count) reaches 0, with a conditional branch (param_1) that handles error/verification states via u2__sub_02FC and u2__sub_037E..
- `copy_disk_sectors_into_buffer_3` (Disk.1 +an address loaded at an address (12288 bytes)): Steps through a run of sectors starting from a track position, copying 128 longs from the buffer for each, and toggles a side flag at each track boundary. Kind: track/sector loading loop with conditional hardware state check. Pass path: The loop continues to load sectors (sVar3 decrementing) until the sector count is exhausted; if param_1 is non-zero, it performs specific hardware/status register updates (DAT_00bfd100) and calls subroutines (u2__sub_02FC) which likely vali.
- `copy_sectors_with_retry_loop` (Disk.1 +an address loaded at an address (12288 bytes)): Sets a control bit and copies 128 longs per sector until the sector count runs out, calling a routine between attempts and clearing the control bit once on a retry. Kind: retry loop with data integrity check. Pass path: the inner loop completes the copy/verification and the function returns via the sVar3 == 0 branch.
- `spin_in_endless_loop` (Disk.1 +an address loaded at an address (12288 bytes)): The visible body is only an endless loop with no exit, although the facts list disk length, pointer, sync and ADKCON registers as touched. Kind: infinite loop trap/sabotage path. Pass path: not determined.
- `set_port_bits_delay_wait` (Disk.1 +an address loaded at an address (12288 bytes)): Sets bits zero and two of the low port bits, clears bit one, runs a fixed delay, then waits while bit five stays set. Kind: disk hardware/timing check. Pass path: the code continues execution after the loop finishes (the loop is a busy-wait for a hardware flag/bit change).
- `set_port_bits_delay_wait_2` (Disk.1 +an address loaded at an address (12288 bytes)): Sets bits zero and one of the low port bits, runs a fixed delay, then waits while bit five stays set. Kind: hardware timing/delay loop check. Pass path: the loop completes and execution continues to the next instruction.
- `write_port_value_delay_wait` (Disk.1 +an address loaded at an address (12288 bytes)): Writes 117 to the CIA-B port register, runs a fixed delay, then waits while bit five stays set before returning. Kind: delay/timing loop for hardware synchronization. Pass path: bset.b #an address, an address.l.
- `setup_display_and_copper_registers` (Disk.1 +an address loaded at an address (12288 bytes)): Clears the audio and loop state words, then writes display window, fetch, bitplane modulo and control register values together with a copper list pointer, and calls one other routine. Kind: hardware register initialization/display setup. Pass path: the execution continues to the next instruction after the call (the routine is a setup/initialization routine for the display/Copper, not a conditional check).
- `infinite_loop_halt` (runtime segment an address (play-rs-m1500)): The routine enters an infinite loop without performing any operations. Kind: sabotage path. Pass path: not determined.
- `mask_and_delay_loop` (runtime segment an address (play-rs-m1500)): Sets certain bits in a global variable, executes a delay loop, and then waits for a bit in a global variable to clear. Kind: disk/track check (sync/bit-pattern check). Pass path: DAT_00bfe001 bit 5 cleared (loop terminates).
- `set_global_constant_2` (runtime segment an address (play-rs-m1500)): Checks a bit in a global variable and sets that global variable to a constant value. Kind: disk hardware/track check. Pass path: the branch where (DAT_00dff01f & 2) == 0, setting _DAT_00dff09c to 5.
- `select_disk_control_by_flag` (runtime segment an address): Tests bit one of a status word: when it is set it writes control values 16384 and then 2 and sets a flag word to one, otherwise it writes 5. Kind: disk hardware/media type check. Pass path: the branch where DAT_00dff09c is set to 5.

## Functions


### AfterTheWar.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `patch_jump_vectors_and_call`: Stores an argument, calls one routine, then writes a 68000 absolute jump opcode and several routine addresses into fixed memory words, and adds 512 to a stored word.
- `call_three_routines_only`: The decompiler timed out, so only the listed facts are known: nine instructions, an address taken through a pc-relative push, one caller, and three calls with no listed register writes.
- `call_four_routines_only`: The decompiler timed out, so only the listed facts are known: twelve instructions, an address taken through a pc-relative push, one caller and four calls with no listed register writes.
- `patch_move_instruction_to_call`: Scans a memory range for a move-word-immediate instruction that targets one particular global, and rewrites it in place into a no-op followed by a subroutine call to another routine.
- `start_blit_and_wait`: Writes a blitter size word to a global and then polls a status bit until the blitter reports it is idle before returning.
- `call_indirect_handler_slot`: Calls through a function pointer stored at a fixed offset in a structure, then returns the register values unchanged.
- `dispatch_indirect_handler_pair`: Calls one routine, then calls through the structure pointer at offset 36, and when that returns non-zero it calls one routine and the pointer at offset 8.
- `dispatch_indirect_handler_pair_2`: Calls one routine, then calls through the structure pointer at offset 36, and when that returns non-zero it calls one routine and the pointer at offset 8.
- `stub_return_only`: The routine is only a return, so it performs no work.
- `stub_return_only_2`: The routine is only a return, so it performs no work.
- `chain_calls_via_handler_slot`: Calls one other routine and keeps its result, then calls one more routine, then calls through the structure pointer at offset 12 and returns the kept result.
- `chain_calls_via_handler_slot_2`: Calls one other routine and keeps its result, then calls one more routine, then calls through the structure pointer at offset 12 and returns the kept result.

### Disk.1:bootblock

- `read_disk_block_into_memory`: Allocates memory through the exec library, points an I/O request at it with a length of 12288 bytes and an offset of 1024, and runs a read command.

### logged disk load from Disk.1

- `forward_single_call_entry`: The decompiler timed out, so only the listed facts are known: four instructions and one call to another routine, reached by execution from the boot entry.
- `forward_single_call_entry_2`: The decompiler timed out, so only the listed facts are known: seven instructions, one call to another routine, and one caller that refers to it as a return target.
- `patch_jump_vectors`: Stores an argument, calls another routine, and writes an absolute jump opcode and several addresses into fixed memory locations.
- `call_four_routines_only_2`: The decompiler timed out, so only the listed facts are known: twelve instructions, four calls, and one caller that references it.
- `forward_single_call_entry_3`: The decompiler timed out, so only the listed facts are known: four instructions and one call to another routine.
- `copy_block_rows_into_planes`: Copies sixteen rows from a source table into a destination area, reading four bytes per row from every eighth source byte and writing each to a different plane at offsets 0, 8000, 16000 and 24000.
- `set_interrupt_masks_around_calls`: Writes one interrupt-enable mask with bit 15 set, calls two routines, then writes a second mask with bit 15 set.
- `seek_track_and_copy_sectors`: Divides a value by 22 and by 11 to get a track and sector position, steps the disk drive head through the CIA-B port the required number of times, and sets the disk length register.
- `copy_disk_sectors_into_buffer`: Steps through sectors from a starting position, copying 128 longs from the buffer for each sector and toggling a side flag at each track boundary.
- `copy_disk_sectors_into_buffer_2`: Sets a control bit, steps through a run of sectors copying 128 longs for each one, and toggles a side flag at each track boundary.
- `copy_disk_sectors_into_buffer_3`: Steps through a run of sectors starting from a track position, copying 128 longs from the buffer for each, and toggles a side flag at each track boundary.
- `copy_sectors_with_retry_loop`: Sets a control bit and copies 128 longs per sector until the sector count runs out, calling a routine between attempts and clearing the control bit once on a retry.
- `clear_drive_control_then_delay`: Writes 247 to the CIA-B port register and then runs a long fixed-count delay loop before returning.
- `spin_in_endless_loop`: The visible body is only an endless loop with no exit, although the facts list disk length, pointer, sync and ADKCON registers as touched.
- `set_port_bits_delay_wait`: Sets bits zero and two of the low port bits, clears bit one, runs a fixed delay, then waits while bit five stays set.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `wait_for_port_status_bits`: Calls a routine repeatedly while bit four of a port register stays set, then waits while bit five of the same port remains set before returning.
- `set_port_bits_delay_wait_2`: Sets bits zero and one of the low port bits, runs a fixed delay, then waits while bit five stays set.
- `delay_then_wait_port_bit`: Runs a fixed delay and then waits while bit five of the CIA port stays set, before returning.
- `wait_while_port_bit_set`: Waits while bit five of the CIA-A port register stays set, then returns.
- `write_port_value_delay_wait`: Writes 117 to the CIA-B port register, runs a fixed delay, then waits while bit five stays set before returning.
- `decode_raw_track_sectors`: Clears a 32-long area, checks for the sync word at the start of a raw track buffer, and decodes the odd and even bits of each sector's header and body into a destination area.
- `find_table_entry_by_byte`: Scans a table of eight-byte entries until one has a byte at offset two equal to a register value, then returns with the pointer on that entry.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `spin_in_endless_loop_2`: Loops forever with no exit and does no other work.
- `unpack_bit_stream_with_lookup`: Unpacks a bit stream into bytes until the output pointer reaches its limit, reading bits from a 32-bit buffer.
- `unpack_bit_stream_with_lookup_2`: Decodes the same kind of bit stream as its neighbour, reading bits from a 32-bit buffer and writing a literal byte or a table byte at each step.
- `unpack_bit_stream_with_repeats`: Decodes a bit stream into output bytes, choosing between a literal byte from a lookup table, a repeat of a base byte, and a short table entry according to the next bits.
- `unpack_bit_stream_with_repeats_2`: Decodes the bit stream in the same style as its neighbours, starting with a table byte and then choosing literal, repeat or table output from the next bits.
- `unpack_bit_stream_with_repeats_3`: Decodes a bit stream into output bytes using literal, repeat and table outputs chosen from the next bits, writing until the output pointer reaches its limit.
- `unpack_bit_stream_with_repeats_4`: Decodes a bit stream into output bytes, starting with a table byte and choosing literal, repeat or table output for each step.
- `setup_display_and_copper_registers`: Clears the audio and loop state words, then writes display window, fetch, bitplane modulo and control register values together with a copper list pointer, and calls one other routine.
- `fill_pointer_table_with_stride`: Fills eight-byte records with addresses that start at a fixed base and step by 8000 each time, storing each address as two words, for a count passed in a register.
- `copy_thirty_two_words_block`: Copies thirty-two words from a source pointer into a fixed destination block, one word per iteration.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_infinite_loop_sequence`: This routine executes an infinite loop that performs no operations.
- `initialize_hardware_state`: Clears a range of memory and writes a sequence of constant values to a global variable, while updating hardware registers such as DMA and interrupt or color settings.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `update_color_and_interrupts`: Sets a global variable to a specific value and updates the interrupt enable bit in the certain hardware register.
- `tiny_ori_2`: Small routine of 1 instruction that merges 253 into d0.
- `modify_global_state`: Checks specific bits in a global variable and resets it to a constant or increments it before calling another routine.
- `modify_global_bits`: Performs bitwise checks and conditional bitwise or operations on a global variable.
- `clear_memory_range`: Iterates through a memory range and sets each byte to zero.
- `store_split_values`: Iterates through a memory range, storing a value and a bit-shifted version of a value into a table.
- `decrement_table_values`: Iterates through a table and decrements certain bit-fields or values in each entry, followed by a delay loop.
- `copy_and_adjust_table`: Copies data from a source pointer to a table, clears several global variables, and then adjusts bit-fields in a table based on a source.
- `update_table_with_offset`: Modifies a pointer, updates a hardware register, and copies a block of data to a global table.
- `copy_data_to_table`: Copies a fixed number of words from a source pointer to a global table.
- `copy_block_of_words`: copies a fixed number of words from a source pointer to a destination pointer using a decrementing loop counter.
- `do_nothing`: the routine is empty and performs no operations before returning.
- `process_keyboard_input`: Performs complex bitwise manipulation and masking on a global variable and updates a table based on a calculated index.
- `do_nothing_2`: the routine is empty and performs no operations.
- `update_global_bit_mask`: Checks a bit in a global variable and updates that variable with a new bitmask or a cleared value based on certain bit conditions.
- `set_hardware_registers`: Calculates a bitmask from a register, updates a global variable, and writes a value to a custom hardware register or a pointer-based address.
- `clear_global_variable`: Performs a series of loops checking a bit in a global variable and then sets that global variable to a specific constant value.
- `set_global_constant`: Performs several loops checking a bit in a global variable and then sets that global variable to a specific constant value.
- `configure_blt_dma_registers`: Sets a global variable to a specific value and updates multiple bitplane or DMA-related hardware registers.
- `execute_single_instruction`: This routine executes a single instruction.
- `infinite_loop_halt`: The routine enters an infinite loop without performing any operations.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `mask_and_delay_loop`: Sets certain bits in a global variable, executes a delay loop, and then waits for a bit in a global variable to clear.
- `set_global_constant_2`: Checks a bit in a global variable and sets that global variable to a constant value.
- `set_global_value`: Sets a global variable to a constant value.
- `decrement_and_check_global`: Decrements a local variable until it reaches a certain value and then loops while a bit in a global variable is set.
- `check_global_bit`: Loops while a specific bit in a global variable is set.
- `initialize_bitmask_tables`: Clears a block of memory and then populates a series of tables with bit-interleaved or masked bit-pattern data.
- `find_end_of_list`: Iterates through a list of bytes by incrementing a pointer until a certain byte or a byte at an offset is reached.
- `enter_infinite_loop`: The routine enters an infinite loop that performs no operations.
- `initialize_memory_tables`: If a global variable equals a certain value, it clears a range of memory and then proceeds to fill other memory ranges with specific patterns or zeros using a pointer.
- `reset_table_and_globals`: If a global variable equals two, it iterates through a table to clear certain entries based on specific byte values, then resets a global variable and clears a block of memory.
- `fill_memory_with_constant`: Iterates through a memory range, writing a constant byte value to a destination pointer if a source pointer matches a specific word value.
- `extract_bits_to_byte`: Extracts specific bits from a 16-bit register and packs them into a single byte stored in memory.
- `increment_global_with_bounds`: Increments a global variable and checks if it reaches a certain value or if a certain range check fails.
- `validate_global_range`: Checks if a global variable falls within a specific range and returns a constant or a masked value based on the result of a comparison.
- `search_data_pattern`: Scans a memory range for a complex sequence of specific byte patterns and updates a global variable with a pointer if a match is found.
- `xor_memory_with_global`: Iterates through a block of memory, performing a bitwise XOR between the contents of each memory location and a previously loaded global variable.
- `tiny_ori_3`: Small routine of 1 instruction that merges 255 into d0.
- `tiny_ori_4`: Small routine of 1 instruction that merges 67 into d4.
- `tiny_ori_5`: Small routine of 1 instruction that merges 2 into d3.
- `tiny_ori_6`: Small routine of 1 instruction that merges 0 into d0.
- `tiny_ori_7`: Small routine of 1 instruction that merges 0 into d1.
- `clear_data_register`: clears the lower byte of a data register.
- `clear_data_register_2`: sets the lower byte of a data register to zero.
- `clear_data_register_3`: performs a logical or operation to zero a data register byte.
- `update_counter_and_check_threshold`: increments a global counter and resets a global variable if a certain condition or character value is met, while checking if the counter has exceeded a certain threshold or certain value.
- `check_and_reset_global_state`: Checks if a global variable is zero or matches a specific character, then resets that variable to zero and increments a counter.
- `tiny_ori_8`: Small routine of 1 instruction that merges 249 into d0.
- `clamp_and_sync_global_variable`: Clamps a global variable within a specific range and synchronizes its value with or resets it based on other global or memory-resident flags.
- `execute_repeated_calls`: Executes a specific number of calls to another routine in two separate descending loops.
- `apply_bitmask_to_memory`: Iterates through a memory range, applying a bitmask or clearing certain memory locations to zero.
- `traverse_linked_list`: Recursively traverses a linked list by checking a global pointer and a secondary pointer, updating a global pointer or setting it to a maximum value if a certain condition is met.
- `process_memory_and_tables`: Performs a series of memory clearing, bitmasking, and table-updating operations, including a delay loop that checks a value from a function call.
- `process_memory_buffer`: Iterates through a memory block, clearing or modifying memory contents based on certain byte values and bitmasking operations.
- `copy_memory_block`: Copies a fixed number of longwords from one memory location to another, offset by a constant amount.
- `copy_memory_with_offset`: Copies a block of memory from a source offset to a destination offset using a loop.
- `shift_bit_stream_left`: Performs a left bit-shift on a large block of memory, propagating the sign or overflow bit from one word to the next in a cascading manner.
- `shift_bit_stream_left_2`: Iteratively shifts a series of 16-bit or 32-bit values left, propagating the sign or overflow bit into the least significant bit of the next value in a chain.
- `copy_struct_elements`: Copies a fixed number of bytes from a source table or structure to a destination buffer using a pointer-based loop.
- `copy_memory_block_2`: Copies a large block of data from a global variable or buffer to a destination offset using a loop that copies multiple words or longs per iteration.
- `rotate_bitstream_right`: Performs a right bit-shift on a large block of memory, where any bit shifted out of a word is moved into the most significant bit position of the next word in a circular fashion.
- `shift_bit_stream_left_3`: Performs a left bit-shift on a series of memory-resident words, propagating the sign bit or overflow bit into the least significant bit of the next word in a chain.
- `increment_nibble_values`: Copies several global words and then iterates through a table, incrementing the lower nibble or certain bits of each word if they do not match a specific pattern.
- `copy_and_adjust_table_2`: Copies a block of longwords from a source to a destination, clears several global variables, and then updates a table by adjusting bit-fields or incrementing values.
- `copy_and_adjust_table_3`: Copies a block of data from a source pointer to a destination table and clears a series of adjacent global variables.
- `copy_and_adjust_table_alt2`: Copies a block of longwords from a source to a destination, clears several global or local variables, and then updates a table by adjusting bit-fields or incrementing values.
- `update_table_and_clear_globals`: Copies data from a source pointer to a table, clears several global variables, and adjusts bit-fields in a table based on a source.
- `update_table_with_decremented_values`: Copies a block of words from a source to a destination, then iterates through a table to decrement certain bits of the destination words if they do not match the source words.
- `reinitialize_table_and_clear_buffer`: Iterates through a table to copy or overwrite values, resets a global pointer, and clears a block of memory or a buffer with zeros.
- `process_memory_with_delays`: Clears a memory block, applies a bitmask to a buffer, and executes a series of delay loops or dummy calls.
- `apply_bitmask_to_buffer_3`: Applies a bitwise mask to a destination buffer by performing a bitwise AND-like operation using a calculated bitmask derived from a source buffer.
- `apply_bitmask_to_buffer`: Performs a bitwise AND-like masking or blending operation between a source buffer and a destination buffer using a calculated bitmask.
- `wait_and_clear_variable`: Waits for a global variable to reach zero and then sets that global variable to zero.
- `fetch_and_mask_pointer`: Loads a pointer from a table, updates a global variable with a masked result, and iterates through a list until a null pointer is found.
- `execute_and_mask`: Calls another routine and returns the result of that routine masked to ensure the least significant bit is zero.
- `copy_memory_blocks`: Performs large block memory copies or updates a pointer using a masked or offset address based on a global variable value.
- `increment_and_mask_return_value`: Calls another routine, increments its return value, and masks the least significant bit to ensure the result is even.
- `set_global_constants`: Sets a global variable to zero and then sets it to several different constant values in sequence.
- `update_table_with_offsets`: Iterates through a list of structures, storing a pointer to the list head in a table and adding an offset or value to the existing pointer stored at that table index.
- `tiny_ori_9`: Small routine of 1 instruction that merges 32 into d5.
- `tiny_ori_10`: Small routine of 1 instruction that merges 122 into d6.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 234 into d1, then merges 250 into d1, then merges 10 into d1.
- `toggle_interrupt_enable_state`: Updates a global variable and modifies the interrupt enable bit in the hardware or certain register, then calls another routine.
- `expand_bitstream_to_buffer`: Decompresses a bitstream or variable-length-encoded bitstream into a destination buffer.
- `decrement_and_update_counters`: Decrements a global variable and, if it reaches zero, resets it to a constant and proceeds to modify a memory buffer or table based on certain bit checks.
- `update_pointer_table_and_global`: Checks a pointer-based condition and, if met, updates a global variable and a table with new pointer or value-based data.
- `copy_struct_data`: Iterates through a table to find a zeroed entry, then copies several multi-byte or single-byte values from a source pointer to a destination pointer and increments a global variable.
- `copy_struct_from_offset`: Iterates through a table of pointers, and if a zero-value is encountered, copies a series of multi-byte or single-byte values from a source pointer offset to a destination pointer offset.
- `copy_block_fields`: Copies several multi-byte or single-byte values from a source pointer offset to a destination pointer offset.
- `update_global_pointer`: Calculates a new value based on a multiplier and a value from a source pointer, then adds this to a global variable.
- `update_sprite_attribute_table`: Updates a table of sprite or object attributes by calculating or loading new attribute values, such as priority, or certain coordinate or pattern offsets, based on a conditional check of a status byte.
- `update_entity_attribute_table`: Iterates through a table of pointers and updates or initializes specific memory-mapped or global structure members based on a conditional check of a flag in a separate data structure.
- `search_and_update_entity_position`: Iterates through a table of entity or object descriptors to find a match based on a calculated type or ID, then updates a global coordinate or state if certain boundary or type conditions are met.
- `update_entity_position_and_size`: Iterates through a linked list of entity or object descriptors to check for collisions or proximity, and if a certain condition is met, updates a size or coordinate-related value in a table using a calculated offset.
- `update_entity_position_and_state`: Iterates through a table of active entities to check for collisions or proximity-based triggers, updating an entity's state or position if a certain condition or boundary-check is met.
- `iterate_and_call_functions`: Iterates a fixed number of times, using a value from a table to call a routine at an offset from a global address.
- `update_object_type_and_priority`: Iterates through an object list to decrement certain counters and reassign object type or priority based on the value of a global variable.
- `update_table_elements`: Iterates through a table and clears a value if certain conditions involving a second table and a byte check are met.
- `evaluate_complex_conditions`: Performs a series of complex range and value checks on a table and clears an entry if any condition or a nested branch is satisfied.
- `decrement_table_counters`: Iterates through a table to decrement a byte or a short value and potentially clear an entry or reset a value based on a byte check.
- `update_game_state_table`: Iterates through a table of values to decrement certain counters and update a global variable or a secondary global score based on specific flag or type checks.
- `execute_object_function_table`: Iterates through a table of pointers and executes the routine located at the address stored in each entry.
- `update_object_state_and_counters`: Iterates through a table of object descriptors to check for specific bit-flags and certain termination or type-based conditions, updating a global variable and decrementing counters in a separate table.
- `validate_amigadp_hunk_types`: Iterates through a memory range to check for specific AmigaDOS hunk type codes and associated size or address constraints.
- `update_entity_attribute_table_2`: Iterates through a table of pointers to update or initialize specific members of a global structure based on conditional checks.
- `update_entity_lifecycle_and_attributes`: Iterates through an entity table to decrement a lifetime counter and update attribute or pointer values.
- `update_object_state_and_attributes`: Iterates through an object list to check for specific bit-flags and visibility or collision conditions, updating object attribute tables and global state or pointers if certain criteria are met.
- `process_entity_state_transitions`: Iterates through a table of entity or object pointers to check type and state flags, updating global state, attribute tables, and pointer-based properties or sprite-like attributes based on conditional checks.
- `update_sprite_attributes_and_vram`: Updates a global buffer or video memory or attribute-related memory based on certain conditions or object types.
- `update_sprite_attributes_and_list`: Iterates through a linked list of object descriptors to update a global buffer and modify certain video memory or attribute-related memory.
- `update_object_list_and_vram`: Iterates through a linked list of object descriptors to update a global buffer and modify certain video memory or attribute-related memory locations based on specific bitmask or type flags.
- `update_sprite_attribute_list`: Iterates through a table of sprite or tile metadata to update or recalculate certain attribute values or pointers.
- `test_bit_conditional`: Checks if a specific bit is set within a bitmask or buffer, returning or continuing execution based on the bit's state and the high bit of an input register.
- `test_bit_in_buffer`: Checks if a specific bit is set within a memory buffer at an offset derived from a register value.
- `permute_data_using_lookup_table`: This routine performs a bitwise XOR on a single bit of a target byte and then uses a lookup table to permute or remap a block of memory.
- `scan_and_update_object_list`: Iterates through a linked list of object descriptors to check for certain bit-flags and range-based visibility or collision conditions.
- `decrement_and_loop_variable`: Decrements a global variable and executes a loop until the variable reaches a certain value.
- `decrement_global_variable`: Decrements a global variable multiple times if it is not zero.
- `execute_on_bit_set`: Checks if specific bits in a global variable are set and calls another routine if they are.
- `update_object_type_and_state`: Checks the type or flag of an object and updates its type or associated global state variables.
- `update_object_position_and_state`: Iterates through a table of object descriptors to check certain type flags and visibility ranges, then updates an object's position or state based on a global counter and a secondary lookup table.
- `update_global_state_from_register`: Checks a global variable against a value stored in a high-order register bit and, if certain conditions are met, updates a global variable with a value from a register and a secondary value derived from the register's upper bits.
- `update_table_index_and_pointer`: Updates a global index or pointer by traversing a table-based structure, or decrements/increments a counter based on conditional checks against table-stored values.
- `traverse_and_update_object_table`: Iterates through a table of object descriptors, updating or decrementing a counter or index based on certain bit-flags and pointer-based lookups.
- `copy_memory_block_3`: copies a block of memory from a source address offset by a global variable to a destination address at a global variable.
- `traverse_and_update_table`: Iterates through a table or structure, checking for a specific value and updating a global pointer or pointer-based offset.
- `clear_struct_members`: Iterates a fixed number of times to clear specific offsets within a structure or array using a pointer.
- `clear_array_elements`: Iterates a fixed number of times to clear specific memory offsets or array elements using a pointer.
- `iterate_tiled_buffer_offsets`: Calculates a starting offset from a global variable and iterates through a sequence of memory-tiled buffer copies or checks.
- `zero_memory_block`: Clears a large block of memory by setting a large number of consecutive words to zero.
- `zero_global_buffer`: Clears a large block of memory starting from a global pointer by setting a large number of consecutive words to zero.
- `apply_bitmask_to_buffer_2`: Performs a bitwise AND or OR operation between a source bitmask and a destination buffer using bit-shifting and masking.
- `blit_sprite_tile_to_vram`: Copies and expands bitmask or pattern data from a source table into a destination buffer in video memory.
- `copy_tiled_memory_block`: Copies a block of memory from a source table or buffer to a destination address using a calculated offset.
- `decrement_table_values_2`: Decrements certain bit-fields or values within a table and executes a delay loop.
- `update_global_value`: Checks a global variable and uses its value to fetch a byte from a table or offset, then updates another global variable with the result or a masked version of it.
- `traverse_buffer_and_copy`: Iterates through a buffer, checking specific bits in each entry, and either calls another routine or copies a tiled memory block.
- `update_global_multiplier`: Updates a global variable using a multiplication and subtraction routine that handles overflow or underflow.
- `tiny_ori_11`: Small routine of 1 instruction that merges 148 into d2.
- `empty_routine`: The routine contains no executable instructions.
- `tiny_ori_12`: Small routine of 1 instruction that merges 156 into d2.
- `tiny_ori_13`: Small routine of 1 instruction that merges 164 into d2.
- `tiny_ori_14`: Small routine of 1 instruction that merges 172 into d3.
- `tiny_ori_15`: Small routine of 1 instruction that merges 0 into d3.
- `tiny_ori_16`: Small routine of 1 instruction that merges 0 into d7.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `empty_routine_2`: the routine contains no executable instructions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_multiple_subroutines`: Calls four different routines including one that copies rows into planes and another that sets interrupt masks.
- `copy_block_rows_into_planes_2`: Copies sixteen rows from a source table into a destination area, reading four bytes per row and writing each to a different plane at offsets 0, 8000, 16000 and 24000.
- `set_interrupt_masks_around_calls_2`: Writes one interrupt-enable mask with bit 15 set, calls two routines, then writes a second mask with bit 15 set.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `select_disk_control_by_flag`: Tests bit one of a status word: when it is set it writes control values 16384 and then 2 and sets a flag word to one, otherwise it writes 5.
- `write_control_value_five`: Writes five to a global control word and returns.
- `unpack_bit_stream_with_lookup_3`: Unpacks a bit stream into bytes until the output length is reached, reading bits from a 32-bit buffer and writing literal bytes or lookup-table bytes at each step.
- `setup_display_and_copper_registers_2`: Clears the state words, then writes display window, fetch, bitplane modulo and control values, a copper list pointer and an interrupt-enable mask, and calls one routine.
- `tiny_ori`: Small routine of 1 instruction that merges 68 into d0.
- `clear_interrupt_requests_and_count`: Checks two interrupt-status bits and, when one is set while the other is clear, increments a pair of counters and calls one routine.
- `write_incrementing_values`: Writes a series of incrementing values into a table using word-sized increments.
- `load_colour_registers_from_buffer`: Copies thirty-two words from a source buffer into the colour registers starting at COLOR00.
- `record_keyboard_key_state`: Handles a keyboard serial byte by converting its code into a key index, looking up the key value in a table, and setting the matching bit in a byte array on every other call.
- `toggle_bit_target_routine`: The decompiler timed out, so only the listed facts are known: twelve instructions, an address taken by an absolute bit-change instruction, and one caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_return_routine`: The routine performs no operations and returns immediately.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning.
