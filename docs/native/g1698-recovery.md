# Game 1698 (blues brothers, the (1991)(titus)[cr qtx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01698). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (926064 top-level bytes in 66 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 133 of 133 named (0 author symbols), 133 of 133 described (12 by rule, 84 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1202 of 1209 (99.42%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3208 of 3215 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 665388 |
| code | 11806 |
| data | 134161 |
| padding | 16405 |
| unused | 98304 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Blues Brothers, The (1991)(Titus)[cr QTX].adf:blues_brothers | 1 | 197 | 38 |
| Blues Brothers, The (1991)(Titus)[cr QTX].adf:benny | 2 | 85 | 3 |
| Blues Brothers, The (1991)(Titus)[cr QTX].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 580 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 39 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1330 | 49 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 333 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 630 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- runtime segment an address (play-long-joy): 89 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 354 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f100): 260 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 381 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (24 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- blues brothers, the (1991)(titus)[cr qtx]/Blues Brothers, The (1991)(Titus)[cr QTX].adf:blues_brothers: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- blues brothers, the (1991)(titus)[cr qtx]/Blues Brothers, The (1991)(Titus)[cr QTX].adf:benny: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_decrypted_data` (runtime segment an address (play-long-joy)): Calculates a string length, performs a block memory copy of decrypted data, and updates a global variable based on a value from a memory structure. Kind: disk track/data integrity check. Pass path: u6__l0_2A6E (the loop that copies data from the buffer to the destination memory).
- `copy_decrypted_data_block` (runtime segment an address (play-long-joy)): Performs a block memory copy from a source table to a destination buffer after applying a decryption process. Kind: checksum/data integrity trap. Pass path: the branch where the loop completes and the function returns normally (the 'do-while' loop that copies data based on a calculated index/count).

## Functions


### Blues Brothers, The (1991)(Titus)[cr QTX].adf:blues_brothers

- `relocate_code_and_jump`: Program entry point: sets a few control words, copies a block of code into a second location, then jumps to the copied code through a stored pointer.
- `decrunch_packed_block`: Moves a packed data block to the top of a buffer, then decodes it backward, reading bits from the end and writing expanded bytes, as a decompression routine does.
- `copy_block_backward_decode_bits`: Copies a block of bytes backward, then reads a packed bit stream from its end, shifting bits out one at a time to drive the decompression loop.
- `call_stored_jump_target`: Calls the routine held by a stored jump-table pointer and returns.
- `store_byte_then_call_routine`: Stores a byte value at a pointer, then calls a stored routine; the same routine is called on both branches of a counter test.
- `call_stored_routine_either_branch`: Calls the same stored routine whichever way a flag byte compares, so the test has no effect on the call.
- `call_routine_then_jumptable`: Loads the address of a jump table into a register, calls another routine, and then calls the routine pointed to by that table.
- `call_jumptable_entry`: Loads the address of a jump table into a register and then calls the routine pointed to by that table.
- `dispatch_jumptable`: Fetches a target address from a jump table and jumps to it, returning afterward.
- `dispatch_via_jumptable`: Fetches a target address from a jump table and jumps to it, returning afterward.
- `decode_bit_count_value`: Reads a variable-length bit count from a packed bit stream, shifting bits from the end, and uses the count to choose the next step of the decompression loop.
- `dispatch_by_threshold_value`: Calls one stored routine when a value is below six and another otherwise, then returns.
- `dispatch_jumptable_by_threshold`: Dispatches to one of several stored routines by reading a jumptable whose index is derived from a value, using a threshold comparison to select the branch.
- `call_stored_routine_either_branch_2`: Calls the same stored routine whichever way a value compares to one, so the test has no effect on the call.
- `dispatch_jumptable_2`: Dispatches control to one of several routines selected from a jump table, where the target address is computed indirectly rather than being a fixed call.
- `dispatch_byte_jumptable_2`: Loads a jump table address and dispatches to one of its entry points based on a byte value, selecting a different table depending on whether a condition is taken.
- `dispatch_via_jumptable_2_2`: Loads the address of a jump table using a PC-relative reference and then performs an indirect call through that table, dispatching to one of the routines stored there.
- `dispatch_byte_to_jumptable`: Calls a jump-table routine based on a byte value, taking either branch to the same table depending on whether the value is zero.
- `dispatch_jumptable_3`: Fetches a target address from an indirect jump table and jumps to it, then returns.
- `dispatch_by_three_threshold`: Calls one stored routine when a value is below three and another otherwise, then returns.
- `dispatch_threshold_jumptable`: Tests a byte against the value two and calls one of two jump tables depending on whether the test is greater or not.
- `call_stored_routine_either_branch_3`: Calls the same stored routine whichever way a test compares, so the test has no effect on the call.
- `dispatch_byte_jumptable`: Loads a jump table address and dispatches to one of its entry points based on a byte value, selecting a different table depending on whether that value is zero.
- `dispatch_byte_to_jumptable_2`: Calls a jump-table routine based on a byte value, taking either branch to a different table depending on whether the value is zero.
- `dispatch_jumptable_call`: Fetches an entry from a jump table using a PC-relative lookup and then calls the routine whose address is stored there.
- `dispatch_via_jumptable_2`: Loads the address of a jump table via a PC-relative reference and then performs an indirect call through that table, dispatching to one of several routines.
- `load_and_call_jumptable`: Loads the address of a jump table via a PC-relative reference and then performs an indirect call through that table, dispatching to one of its entries.
- `copy_bytes_backward_then_call`: Copies a count of bytes backward from a source end to a destination, then calls a stored routine.
- `dispatch_byte_jumptable_3`: Loads a jump table address and dispatches to one of its entry points based on a byte value, selecting a different table depending on whether a condition holds.
- `dispatch_byte_jumptable_4`: Loads a jump table address and calls one of its entry points, choosing a different table depending on a byte value.
- `dispatch_jumptable_indirect`: Loads a jump table via a PC-relative reference and performs an indirect call through it, dispatching to one of the stored routine addresses.
- `shift_bits_from_packed_stream`: Shifts bits out of a packed stream one at a time, counting down, and calls a stored routine when the count runs out.
- `refill_and_copy_packed_run`: Shifts bits from a packed stream and, when the bit buffer empties, refills it and copies a run of bytes from a back-reference table to the output.
- `read_bits_and_copy_run`: Reads bits from the packed stream while copying a run of previously written bytes into the output, calling a stored routine when the count is exhausted.
- `copy_back_reference_run`: Copies a run of earlier output bytes forward using a back-reference offset, then continues decoding bits from the packed stream.
- `store_control_words_and_call`: Writes a byte into the output, stores two control words into the state block for the DMA and interrupt enable settings, then calls a stored routine.
- `copy_literal_run_backward`: Copies a run of bytes backward from the source end to the destination, stopping at the start boundary.
- `copy_zero_fill_run_backward`: Copies a run of bytes backward from the source, using a zero fill byte for the run, and stops at the boundary.

### Blues Brothers, The (1991)(Titus)[cr QTX].adf:benny

- `run_depacker_decode_loop`: Program entry point of the depacker: initialises the bit reader state and runs the decode loop, which reads bits from the packed stream through helper routines and writes to the background colour register.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Blues Brothers, The (1991)(Titus)[cr QTX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_global_value`: Stores a constant value into a global variable.
- `initialize_hardware_and_tables`: Configures DMA, interrupts, and video positioning while calling routines to interleave records, pack tables, and initialize global values.
- `copy_interleaved_records`: Reads a count from a global variable and copies that many four-byte records from two source locations into a destination, interleaving the source words into consecutive destination words.
- `pack_and_interleave_table`: Reads a byte length from a global table and packs its bytes into two separate sequences of 16-bit words by shifting each byte into the high half of a word.
- `initialize_table_and_pointer`: It writes the value sixteen into two consecutive words of a global table in a loop that runs six times, then stores the address of that table into two consecutive global variables.
- `increment_global_variable`: Adds three to a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_global_variable_2`: Adds three to the value stored in a global variable.
- `copy_value_into_sequence`: Copies a value from a pointer into a sequence of consecutive word locations a fixed number of times, selecting the destination from a global value.
- `store_word_in_indexed_table`: Writes a 16-bit value from a register into a 32-bit slot of a table, using the low half of the input shifted left by two bits as an index.
- `setup_coprocessor_constants`: Clears a global variable, then writes a 16-bit constant and a value of one hundred into two adjacent 32-bit words, and finally stores a 16-bit constant into a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_copper_list_address`: Stores the high and low halves of an address into a copper list pointer and records the value in shared state.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_two_24bit_constants`: Stores two different 24-bit constants into two separate global variables, overwriting the first with the second constant.
- `init_blitter_slots_and_wait`: Initializes five short-value slots to their maximum value and sets two other global values, then waits until a specific bit in a status register clears.
- `toggle_and_reload_audio_tables`: Toggles a global flag between two states and, depending on which state is active, initializes or reloads several sets of four short-value slots in various tables.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `initialize_global_if_empty`: It writes a value through a global variable only when that variable currently holds zero.
- `wait_for_flag_to_clear`: It repeatedly reads a global variable in a loop until that variable becomes zero.
- `init_global_structure_again`: Initializes a global structure by storing fixed default values at several fixed offsets, including a 32-bit value taken from the structure itself.
- `clear_and_copy_blocks`: Clears a block of 16-bit words to zero and then copies that block into several other blocks in a loop.
- `run_infinite_loop`: It is an empty infinite loop that never returns.
- `save_mask_disable_interrupts`: It increments a global counter, saves the current interrupt mask, and disables interrupts via the interrupt enable register.
- `handle_keyboard_interrupt`: The routine manipulates global variables based on flags, writes a constant to a hardware register, and touches CIAA and custom registers.
- `store_value_toggle_flags`: It stores an input value into a global variable, conditionally writes a byte to a hardware register, and toggles bits in a global flag table.
- `calculate_input_difference_2`: Performs arithmetic operations on a global variable based on bitwise checks of a value.
- `calculate_input_difference`: It performs arithmetic operations on a global variable based on bitwise checks of a value.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_memory_write_globals`: Clears a block of memory starting at a fixed address by repeatedly storing zero bytes until a counter reaches its limit.
- `initialize_object_attribute_table`: Calculates spatial bounds and populates a memory block with structured data entries.
- `fill_memory_block_with_pattern`: Fills a block of memory with a repeating pattern, writing several fixed values to specific offsets of each entry.
- `call_and_return`: Calls another routine and returns the value stored in a register.
- `process_decrypted_data`: Calculates a string length, performs a block memory copy of decrypted data, and updates a global variable based on a value from a memory structure.
- `return_immediately`: Returns at once without doing anything.
- `copy_decrypted_data_block`: Performs a block memory copy from a source table to a destination buffer after applying a decryption process.
- `decrypt_and_copy_data`: Performs a bitwise XOR-based decryption loop on a block of memory before copying the resulting data to a destination buffer.
- `load_and_discard_register`: It loads the value at the address held in a register into another register and then returns without using it.
- `modify_byte_and_loop_2`: Modifies specific bits of a byte through a pointer and executes a long decrementing loop.
- `modify_byte_and_loop`: It modifies specific bits of a byte through a pointer and executes a decrementing loop.
- `tiny_ori`: Small routine of 1 instruction that merges 184 into d3.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d6.
- `call_subroutines`: Calls two other routines and returns.
- `configure_hardware_and_wait_for_input`: Writes several values to a memory structure and then enters a loop that waits for a specific value to appear in a memory location or for a bitmask to change.
- `configure_and_wait_hardware`: Writes several values to a memory structure and then enters a loop that waits for a specific value to appear in a memory location.
- `perform_bit_manipulation`: Performs complex bitwise shifts and masking using registers and returns a calculated byte.
- `assemble_tile_bitplanes`: Scans a memory region for a specific marker to locate tile data, then reconstructs bitplane bytes by fetching values from two separate memory buffers and combining them using bitwise OR operations.
- `scan_and_reconstruct_tile_data`: Scans a memory region for a specific marker value and then iterates through a sequence of indices.
- `reconstruct_tile_bitplanes`: Iterates through a sequence of tile indices to fetch individual bitplane bytes from two separate memory buffers and combines them into a single byte.
- `combine_memory_tables`: It searches a table for a specific marker and then populates a destination buffer by combining bytes from two different memory sources.
- `recurse_on_limit`: Recursively calls itself if a global variable equals a specific value.
- `increment_and_compare_global`: Increments a value through a pointer and updates a global variable until a specific limit is reached.
- `nop_placeholder`: This routine is empty and performs no operations.
- `loop_until_bit_clears`: Repeatedly writes values to a memory location through a pointer while checking a bit in a global variable until it is cleared, then sets that variable to zero.
- `decrement_and_recurse`: Writes values to a memory location through a pointer and decrements a global variable if it is non-zero, then calls itself recursively.
- `update_video_pointer`: The routine updates the video hardware pointer register.
- `increment_until_limit`: Writes values to a memory location through a pointer and increments a global variable if it has not reached a specific value, then calls itself recursively.
- `wait_for_value_range`: Loops until a specific byte value within a 16-bit word at a given offset changes from one value to another, then stores a constant into a third offset.
- `nop_padding_sequence`: The routine is empty and performs no operations.
- `update_audio_parameters`: Copies several values from a table into a memory area and modifies audio hardware registers.
- `update_audio_channel_parameters`: Updates audio channel control structures by fetching pitch, period, and volume data from a lookup table based on a nibble-encoded index.
- `compare_channel_target_value`: Copies a 12-bit value from a channel record into a field and sets a flag when it is below a stored target.
- `update_channel_value`: Updates a 12-bit value within a control structure based on a mode flag.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_pointer_and_trap`: Stores the address of an unnamed routine in a global variable and then raises a system trap with no arguments.
- `init_display_and_copper`: Initialises the custom chips: it polls a status bit between steps, writes the copper list pointer and starts the copper, and sets DMA and interrupt control bits.
- `tiny_ori_3`: Small routine of 1 instruction that merges 5 into d1.
- `copy_sixteen_words_to_block`: Copies sixteen 16-bit words from a fixed source table into a block at a set offset from an address register, then returns.
- `empty_return_stub`: Contains only a return and changes no memory or registers; it is an empty routine called once.
- `copy_table_on_button_step`: When the mouse button bit is set, advances a step counter.
- `tiny_ori_4`: Small routine of 1 instruction that merges 2 into d0.
- `build_copper_list_entries`: Fills a copper list with thirty-three wait-and-move pairs, each waiting for a successive scan line and writing a value to a colour register.
- `step_colors_toward_source`: Walks a 32-entry colour source table.
- `fill_bitplane_pointer_copper_list`: Writes eight register-and-data word pairs into a fixed copper list area.
- `interleave_three_source_planes`: Copies three consecutive 2,120-byte source regions into three bitplanes that sit 8 KB apart, one byte at a time, in a single loop.
- `stash_fields_and_reset_limits`: Copies three fields from a state record into globals, saves one more global, and sets three fields of another record to the maximum 16-bit value.
- `copy_block_to_second_buffer`: Copies a fixed block of 179 bytes from one table into a second buffer, one byte at a time, then returns.
- `render_pending_text_line`: On every eighteenth call, reads a zero-terminated text buffer and draws each character as six rows of glyph data from a font table into the bitmap, using a 40-byte row stride.
- `tiny_ori_5`: Small routine of 1 instruction that merges 104 into d4.
- `tiny_ori_6`: Small routine of 1 instruction that merges 2 into d4.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 117 into d0, then returns.
- `clear_sixteen_words_block`: Writes zero to sixteen consecutive 16-bit words in a block at a fixed offset from an address register, then returns.
- `clear_bitmap_region_bytes`: Writes zero to about 40 kilobytes of consecutive bytes, one at a time, starting at a fixed location in bitmap memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `pass_through_register_results`: Calls another routine and concatenates the resulting values from two registers into a single return value.
- `call_and_return_registers`: Calls another routine and returns the concatenated results from two registers.
- `update_channel_pointer`: If a value is non-zero, it calculates an offset to retrieve a value from a table and stores it in two different global variables; otherwise, it sets a flag and a global variable.
- `initialize_memory_table`: If a flag is not set, it sets the flag and performs a series of calculations to populate several global variables using values from a table.
- `update_audio_channels`: If a stop flag is set, it clears a global variable; otherwise, it decrements a timer, reloads it when zero, and updates the four audio channel registers with new pointer, length, period, and volume values.
- `spin_on_volatile_byte`: Repeats an empty loop that compares a byte with itself, so it only ends if the hardware changes that byte between reads.
- `advance_channel_sequence_pointer`: Advances one sound channel's event pointer through its data.
- `interpret_music_channel_commands`: Interprets a music command stream for one channel, where special command bytes select instrument or sample data from a table.
- `initialize_four_sound_channels`: Disables audio interrupts and initializes four channel structures from a table by setting their counters, periods, volumes, and DMA settings.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d4.
- `return_negative_one`: Returns a value of negative one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
