# Game 664 (GemX v1.1 1181): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00664). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2012249 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 174 of 174 named (0 author symbols), 174 of 174 described (12 by rule, 162 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1657 of 1658 (99.94%) |
| Recovered without load address (counted in proof 1, flagged) | 14080 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3421 of 3422 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 283584 |
| code | 15248 |
| data | 1711104 |
| padding | 9 |
| unused | 2304 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 297 | 17 |
| logged disk load from disk.2 | 1 | 1817 | 61 |
| logged disk load from disk.3 | 1 | 0 | 0 |
| logged disk load from disk.4 | 1 | 1207 | 83 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 80 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| disk.1 bytes 0-13824 (code never loaded by a reference run) | 1 | 0 | 0 |
| disk.1 bytes 15104-15360 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.1: raw disk image (362496 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.2: raw disk image (55296 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.3: raw disk image (6144 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.4: raw disk image (264192 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.5: raw disk image (184320 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- GemX_v1.1_1181/GemX_v1.1_1181.hdf: hard-disk image (AmigaDOS OFS, 2176 blocks): its filesystem parses and all 9 files in it are byte-identical to the extracted tree GemX_v1.1_1181_hdf, which is recovered file by file; no block outside the filesystem holds data
- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 3 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address (play-long-joy): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 51 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.1: bytes 0-13824 look like code no run loaded: recovered position-independently (load address unknown)
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.1: bytes 15104-15360 look like code no run loaded: recovered position-independently (load address unknown)
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.1: 2 logged loads cover 14080 bytes of the image
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.1: 6 track-loaded chunks found in recovered runtime segments and classified through them
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.1: chunks 61-89 (bytes 15616-23040) outside the filesystem hold data, not code (opcode density 0.037, entropy 4.97)
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.1: chunks 92-166 (bytes 23552-42752) outside the filesystem hold data, not code (opcode density 0.031, entropy 5.28)
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.1: chunks 171-433 (bytes 43776-111104) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.26)
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.1: chunks 435-448 (bytes 111360-114944) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.12)
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.1: chunks 450-453 (bytes 115200-116224) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.33)
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.1: chunks 455-1415 (bytes 116480-362496) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.76)
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.2: 1 logged loads cover 18432 bytes of the image
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.2: chunks 0-143 (bytes 0-36864) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.75)
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.3: 1 logged loads cover 6144 bytes of the image
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.4: 1 logged loads cover 264192 bytes of the image
- GemX_v1.1_1181/GemX_v1.1_1181_hdf/GemX/disk.5: chunks 0-719 (bytes 0-184320) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.23)
- 2 never-loaded region(s), 14080 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `reset_playback_state_variables` (disk.4 +an address loaded at an address (264192 bytes)): Clears a set of globals and memory bytes, loads several state words from indexed tables, and fills a block of memory locations with a pointer, a 16-bit value and a zero longword in a descending loop. Kind: not judged: Expecting value: line 1 column 1 (char 0). Pass path: not determined.
- `nop_global_variable` (runtime segment an address (play-long-joy)): performs a no-operation on a global variable. Kind: checksum/integrity trap. Pass path: the instruction immediately following the call (the 'after' list).
- `update_global_state` (runtime segment an address (play-long-joy)): Performs multiple bitwise and assignment operations on a global variable and calls a routine that waits for a specific bit to be set. Kind: checksum/state trap. Pass path: execution continues to u6__sub_009E().

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `initialize_system_and_loop`: Writes initial constants to global variables and fixed memory locations, sets interrupt and DMA control registers, and then runs an endless loop that repeatedly reinitializes memory, writes code pointers and constants, and calls helper routines.
- `initialize_and_dispatch_handlers`: Runs an endless loop that zeroes global variables, writes a set of constants and code pointers into fixed memory locations, invokes several other routines and tables, and conditionally rewrites a few words before repeating.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `call_single_routine`: Calls another routine once and returns, performing no other work.
- `forward_call_to_helper`: Takes no arguments and simply forwards control to another routine, calling it once before returning.
- `call_indirect_table_entry`: Calls a routine whose address is read from a fixed offset within a table referenced by a global variable.
- `dispatch_table_slot_by_value`: Computes a value from a global variable, then compares it against two thresholds and calls the routine stored at a fixed offset in a table referenced by a global variable.
- `call_indirect_table_slot`: Calls a routine whose address is read from a different fixed offset within the same table referenced by a global variable.
- `invoke_indirect_table_slot`: Calls the same table-slot routine as another routine, reading its address through a global variable's table at a fixed offset.
- `dispatch_segment_copy_request`: Tail-calls another routine that derives a segment index and count from a stored value and copies the corresponding data.
- `compute_segment_and_copy`: Derives a segment index and a count from a value stored in a structure and a fixed multiplier, repeatedly subtracts a per-segment amount from a counter, and calls a table-slot routine each iteration until the counter reaches zero.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `conditionally_store_constant_byte`: Tests a masked bit pattern of a global variable and, if the test fails, stores the constant 8 into that global variable before returning a stack value unchanged.
- `poll_global_flag_then_set`: Tests a global variable against a mask of 88 hex and loops forever if both bits are set; otherwise it stores the constant 8 into that global variable and returns.
- `read_cia_timer_low`: Reads the low byte of the CIA-A time-of-day counter, which is the only hardware access evident from the recovered code.
- `wait_for_vertical_position`: Reads a global variable and repeatedly polls it until a bit field changes relative to a captured value, then returns a combined value from the global variable.

### logged disk load from disk.2

- `run_frame_timed_state_machine`: Initializes a set of global variables and hardware sprite and copper-related registers, then runs a main loop that waits for a fixed raster-position window, invokes a function pointer selected from a table indexed by a global counter, and cycles that counter through four states with optional extra helper calls.
- `poll_raster_position_loop`: Repeatedly waits until the raster position value falls within a fixed range, then calls another routine and decrements a global counter.
- `compare_values_within_range`: Computes the difference between two 16-bit register values and returns without changing any data, as the range check has no effect on memory or hardware.
- `fill_global_descending_words`: Writes a descending sequence of 16-bit values, starting at minus one, into a global variable until the value wraps back to minus one, which ends the loop.
- `run_timed_table_transform_loop`: Repeatedly waits on a raster-position value, calls two helper routines, copies a table word into two other locations, and counts a global counter down from a fixed start value until it goes negative.
- `write_fixed_text_message`: Writes a fixed sequence of byte values into a global variable, choosing between two alternative character sets depending on a register test.
- `scan_byte_command_stream`: Scans a byte stream: skips over repeated 255 bytes four bytes at a time, returns on a 254 terminator, and for each other byte calls a helper before advancing one byte.
- `store_constant_at_offset`: Stores the constant 112 as a 16-bit word at a fixed offset from the A5 base register.
- `store_seven_at_offset`: Calls another routine, then stores the constant 7 as a 16-bit word at a fixed offset from the A5 base register.
- `set_pointer_and_reset_fields`: Stores the address of a global variable into a structure field, calls another routine, then writes a series of 16-bit values into fixed structure offsets and several global variables.
- `initialize_blitter_and_wait`: Stores a pointer into a structure, calls another routine, sets several 32-bit and 16-bit fields through a base register, and polls a status bit before continuing.
- `setup_multi_row_transfer`: Clears two global variables, stores a pointer into a structure, calls another routine, and writes fixed values to hardware-facing offsets while polling a status bit.
- `copy_rows_with_busy_wait`: Runs four iterations that store an address and a 16-bit value through the A5 base register, each time waiting on a status bit at a fixed offset before advancing the address by a register-held stride.
- `update_sprite_position`: Updates the horizontal and vertical coordinates of a sprite by reading values from a table and writing them to the hardware registers.
- `reset_global_state`: Sets a global variable to the address of itself and resets another global variable to zero.
- `populate_data_table_2`: Iterates through a loop to update a structure with values derived from a table, or calls other routines to unpack data if a counter reaches a specific value.
- `initialize_graphics_data`: Sets two specific values in a structure, calls a routine to copy rows twice with different parameters, and updates global state variables.
- `initialize_graphics_loop`: Decrements a global counter and, if it reaches zero, initializes a series of memory values and executes a loop that performs repeated memory writes.
- `update_graphics_state_and_buffer`: Manages a state machine that updates hardware registers and memory buffers through a series of loops and conditional checks.
- `populate_data_table`: Iterates four times to store a value and a constant byte into a memory structure pointed to by a register.
- `advance_counter_and_update_state`: Writes a value from one global variable into two others, decrements a 16-bit counter, and when it goes negative resets it, advances a second counter by a fixed step, and conditionally calls other routines and updates a BCD-adjusted global variable.
- `initialise_registers_with_wait`: Stores a set of 32-bit and 16-bit constants at fixed offsets from the A5 base register, then performs two busy-wait loops that each poll a status bit before writing a further 16-bit value.
- `call_routine_sequence`: Calls a sequence of other routines in order, with one of them invoked in a counted loop of six iterations.
- `call_routine_sequence_with_loop`: Calls a fixed sequence of other routines in order, invoking one of them in a counted loop of six iterations.
- `copy_table_bytes_to_buffer`: Unless a register equals a specific value and another register is zero, reads eight bytes from a table indexed by a register and stores them into a buffer at fixed spacing of 40 bytes.
- `invoke_two_routines_sequentially`: Calls two other routines one after the other and then returns, performing no other work on data itself.
- `advance_scrolling_state_tables`: Updates several global variables and words, computes packed byte values from them, copies a block of longwords from a table selected by an index into global storage, and advances index and counter variables with wrap-around.
- `decrement_and_step_toward_target`: Decrements one 16-bit global variable, then moves a second global variable one step toward a third value, incrementing it if below the target and decrementing it if above.
- `decrement_global_word`: Subtracts one from a 16-bit global variable and returns.
- `update_entries_and_write_hardware_blits`: Decrements a countdown global and, when it expires, resets it and walks a table of four-longword entries, updating hardware-related registers and issuing a polled sequence of four writes per active entry before decrementing one field of each entry.
- `return_without_action`: This routine is empty: it consists only of a return instruction and performs no data reads, writes, or hardware access.
- `copy_and_split_table_entries`: Walks backward through 32 words of a global variable block from a source area, copying each word and then writing nibble-derived byte values from it into two separate tables.
- `toggle_flag_and_transform_nibble_tables`: Toggles a global flag and, when it becomes zero, processes 32 words of a global table by doubling bytes, adding values from two tables into a third, and recombining nibbles into a word, then optionally calls another routine.
- `copy_words_backward_block`: Copies 32 consecutive 16-bit words from one position in a global variable block to another, advancing forward through the block one word at a time.
- `unpack_nibbles_into_tables`: Iterates over 32 words, storing each word's low nibble, high nibble and middle nibble into three separate tables while also writing the original word back in place.
- `set_cia_timer_registers`: Stores a sequence of constant values into global variables, apparently setting up timer and control values, then returns a fixed byte.
- `read_keyboard_serial_key_code`: Reads a global byte, inverts and rotates it into a new code, then uses that code to search a table of 14-byte records and stores matching values into a global variable.
- `shift_records_forward_bcd_counter`: Sets a global variable to one and then zero, loops incrementing a BCD-adjusted global counter while scanning a table of 12-byte records, and when a record's stored value exceeds a register value it shifts a block of bytes forward by 12 bytes.
- `copy_table_words_alternating`: Copies 100 longwords between two fixed memory blocks, alternating the source and destination direction depending on a global flag and setting that flag to one on its first use.
- `initialize_transfer_state_once`: On first entry, sets a flag, writes a fixed value to a hardware-facing register offset, clears two global variables, stores a table pointer and small constants into global variables, calls another routine, and then calls an indirect routine through a stored pointer.
- `copy_table_until_negative_marker`: Walks a global byte table, calling another routine for each non-negative entry, and returns the first negative entry it finds.
- `update_timed_state_and_input_sequence`: Waits for a timing window on a global value, then repeatedly decrements a countdown and, when it expires, processes a byte from a global buffer as a command: it either edits a small list of entries, advances or retreats an index, clears the buffer, or stores the byte into the buffer, and it also toggles a global flag on a periodic countdown.
- `clear_indexed_byte_and_reset_counter`: Clears one byte in a table at an offset derived from a global variable and sets a 16-bit global counter to zero.
- `poll_input_and_advance_cursor`: Reads a port-related global byte and a status register, updates a fire-state flag and a sequence counter, stores a byte from a table into a global array slot, and adjusts and wraps two byte-sized global counters and values depending on a bit test.
- `return_without_action_2`: This routine is empty: it consists of a single return instruction and performs no work on any data.
- `sequence_hardware_init_steps`: Calls a series of helper routines in order, sets one global variable to 1, then stores two fixed word constants into global variables and calls further helpers.
- `write_block_words_to_hardware`: Writes a fixed set of words to hardware-facing register offsets and then loops, toggling a control word, waiting for a status bit, and writing a fixed constant, while decreasing a global count by a block size and advancing a source pointer until the count reaches zero or below.
- `fill_track_buffer_with_pattern`: Fills a block of memory with an alternating 170 pattern, writes two sync-style words into a fixed buffer, and then runs a loop calling a helper twice per iteration.
- `mask_and_merge_bit_pattern`: Transforms a 32-bit value in a data register by XOR-ing it with an alternating-bit mask, rotating and ANDing adjacent bits, optionally clearing the top bit depending on a flag byte read before the target address, then stores the result merged with the original alternating bits through a pointer.
- `decode_mfm_sector_to_buffer`: Waits for a sync word from the disk controller hardware, then reads a block of raw MFM-encoded words, decodes the odd and even bit streams into a buffer while accumulating a checksum, and compares that checksum with a stored value.
- `wait_until_counter_matches`: Repeatedly calls another routine while a short value held in a register differs from a global variable, looping until the two are equal.
- `set_flag_and_wait_for_clear`: Sets bit 1 of a global variable, calls two helper routines, then repeatedly calls two helpers while bit 4 of that variable stays set.
- `copy_global_to_itself`: Assigns a global variable to itself, leaving its value unchanged, and then returns.
- `set_bit_and_decrement_counter`: Sets bit 1 of a global variable, calls one helper routine, decrements a counter global variable by one, and then writes the flag variable back to itself.
- `clear_bit_and_increment_counter`: Clears bit 1 of a global variable, calls one helper routine, increments a counter global variable by one, and then writes the flag variable back to itself.
- `set_bit_and_poll_until_set`: Sets the lowest bit of a global variable, then repeatedly reads that variable until the bit is observed as set.
- `clear_bit_and_wait_clear`: Clears the top bit of a global variable, writes it back, then repeatedly reads it until a particular bit is clear.
- `set_global_bit_seven`: Sets the high bit of a global variable and then writes that same value back to it.
- `clear_shifted_bit_mask`: Computes a single-bit mask from the input word in the data register plus three, shifted left, then inverts it and ANDs it with a constant before storing the result into a global variable.
- `toggle_flag_and_poll_bit`: Sets or clears bit two of a global variable depending on whether it is zero, then busy-waits until bit zero is set, sets bit zero, busy-waits again, and finally stores two constants into a global variable.
- `configure_timer_registers_and_wait`: Updates several global variables with masked and literal byte values, then calls a helper that waits on a flag.

### logged disk load from disk.4

- `conditionally_run_pending_updates`: Saves a word-sized value, then when a flag byte is nonzero it calls routines conditionally on another signed byte and a pending word.
- `clear_scattered_global_variables`: Writes zero or fixed small constants into a set of scattered global variables and memory locations.
- `store_masked_value_and_clear`: Stores the low nine bits of a data register into a global word, clears two global bytes, and then calls another routine.
- `decode_command_byte_into_channel_state`: Takes a command byte from a data register, splits it into a channel index and a command value, and updates per-channel state bytes and words in several tables, with a special path that stores the command value and a state flag.
- `initialize_table_pointers_and_clear`: Sets a global longword to a fixed constant, clears several global variables, stores pointers derived from a data register offset into global longwords, and stores a constant value into a series of table entries in a loop.
- `reset_scattered_state_variables`: Writes zero to a set of scattered global variables and memory locations, along with a few constant values such as 1920 and 15.
- `jump_to_reset_playback_state_variables`: Jump stub: passes control straight on to reset_playback_state_variables.
- `reset_playback_state_variables_2`: This routine is empty and performs no operations before returning to its caller.
- `jump_to_dispatch_command_byte_to_state`: Jump stub: passes control straight on to dispatch_command_byte_to_state.
- `update_conditional_state`: Checks values in a memory structure and updates several fields in a table based on comparison logic and bitwise masking.
- `configure_dma_and_reset`: writes to hardware registers and a memory location before calling another routine to reset playback state variables.
- `decrement_timer_and_dispatch`: Decrements a counter word in a record and, when it goes negative, reloads it from a table value and repeatedly clears a flag byte and calls another routine until that flag is set.
- `tiny_addq`: Small routine of 1 instruction that adds 4 to a0.
- `advance_channel_script_state`: Reads a command byte from a per-channel record and, for values below 240, applies a signed offset and mask to the channel position before recursing into another routine.
- `set_byte_and_advance_counter`: Writes the byte value 255 into a structure field at a fixed offset from one address register, then updates a word counter in a second structure by resetting it to a stored value when it reaches a limit or otherwise incrementing it.
- `update_animation_frame_state`: Advances a per-object state record through a small state machine: it decrements a counter field with wraparound, loads a table entry indexed by a state byte into a pointer field, and on certain codes copies or zeroes neighbouring fields, then recurses into another routine for sub-items.
- `advance_script_position_from_table`: Reads a byte from a record at a fixed offset and uses it as an index into a pointer table to compute a new position value, which is stored in the record.
- `copy_byte_and_increment_counter`: Copies a byte from a record at a fixed offset into another record's field, then increments a 16-bit counter stored in that second record.
- `store_ff_at_offset`: Writes the byte value 255 into a structure field at a fixed offset from the address register passed in.
- `jump_to_clear_word_and_set_byte`: Jump stub: passes control straight on to clear_word_and_set_byte.
- `process_script_command_stream`: Iterates through a data stream to execute commands by reading bytes and updating channel state.
- `process_script_byte_stream`: Reads a byte from a record at a fixed offset and uses it as an index into a pointer table to compute a new position value, which is stored into the record along with a copied counter.
- `clear_word_and_set_byte`: Writes a zero word through a frame-pointer-relative address at a fixed offset and writes a byte value of 255 through the address register at another fixed offset.
- `parse_script_command_stream`: Reads a byte from a script buffer and branches based on its value to update state pointers or execute specific command logic.
- `load_channel_records_from_stream`: Reads a 16-byte record from a pointer in a table, stores its first eight 16-bit words into fields of a channel structure, and for each word that is non-negative computes a pointer through a lookup table, sets a counter field to zero and a flag byte to 255.
- `clear_byte_at_offset_2`: Writes zero to a single byte located at a fixed offset from the address held in an address register, then returns.
- `advance_and_update_channel_state`: Increments a 16-bit counter in a structure, then on its first pass decodes two bytes from a source record into state fields and sets a direction flag.
- `clear_byte_at_offset`: Calls another routine, then writes zero to a byte located at a fixed offset from the address held in a register.
- `mix_sample_stream_channels`: Advances a set of per-channel accumulators and envelope counters, then fills a buffer of sample bytes by looking up each sample value in a table and applying channel volume and envelope state, with a separate path that writes a fixed value when the channel is inactive.
- `tiny_addq_2`: Small routine of 1 instruction that adds 4 to a5.
- `render_sample_stream_with_channel_state`: Runs a loop that computes a byte value per sample from an accumulator and a table lookup, applies per-channel volume and envelope state with optional sign flipping, and writes each result through a pointer into a buffer, advancing the buffer index and updating several per-channel state variables.
- `accumulate_and_fill_sample_buffer`: Runs an accumulator-driven loop that writes computed byte values into a buffer through a pointer, updating an index and several per-channel state words and bytes as it goes.
- `reset_state_and_dispatch_table_entry`: Clears several state bytes and words, increments a 16-bit index counter, and loads a longword from a table at the indexed slot into a record.
- `load_table_entry_and_dispatch`: Copies a word from a global block into a variable, conditionally ORs a word into a record field or loads a value, increments a 16-bit index counter, and loads a longword from an indexed table slot.
- `sum_fields_store_and_dispatch`: Clears a byte, adds two longword fields from records, stores the sum in a record and through a pointer, increments a 16-bit index counter, and loads a table entry.
- `dispatch_next_table_handler`: Copies a word from one block field into two other locations, advances a 16-bit index, loads a longword from a table at that index into a record, then clears the record's low byte and jumps through a dispatch table indexed by the byte's value.
- `advance_index_and_dispatch`: Copies a word from one block field into another location, advances a 16-bit index, and returns early when a flag byte is negative; otherwise it loads a longword from a table at that index, clears its low byte, and jumps through a dispatch table indexed by that byte.
- `step_countdown_and_dispatch`: Decrements a countdown byte in a structure, or resets it to 255 and advances a 16-bit index when it reaches zero, then loads a longword from a table indexed by that value.
- `jump_to_process_state_transition`: Jump stub: passes control straight on to process_state_transition.
- `clear_global_byte_target`: Stores a zero byte through a pointer held in a register and returns.
- `scale_table_word_and_dispatch`: Reads a word from a global table indexed by a masked sum of two byte fields, scales it by an adjusted offset value, and stores the result in a record, also copying it to a second location when a flag word is zero.
- `scale_table_word_and_dispatch_2`: Reads a word from a global table indexed by a masked byte field, scales it by an adjusted offset value, and stores the result in a record, also copying it to a second location when a flag word is zero.
- `dispatch_event_handler`: Clears several memory locations, increments a counter, and executes a routine from a dispatch table using a value fetched from a table.
- `copy_flag_gated_word_and_advance`: Copies a byte-derived pair of values into block fields, sets a fixed byte, copies a word into a second location only when a counter word is zero, increments a 16-bit index, then loads and dispatches the next table entry through the dispatch table.
- `compute_scaled_word_and_store`: When a byte field equals minus two, reads a word from a global table through an index derived from two byte fields, scales it by a value derived from a 16-bit field, and stores the result in two record fields.
- `scale_audio_sample`: Fetches a value from a table, scales it using arithmetic, and stores the result in a memory location before calling a routine from a dispatch table.
- `jump_to_store_word_fields_advance_coun`: Jump stub: passes control straight on to store_word_fields_advance_counter.
- `update_state_counter`: Modifies a counter or a byte value based on conditional checks and then calls a routine from a dispatch table.
- `jump_to_clear_fields_and_dispatch`: Jump stub: passes control straight on to clear_fields_and_dispatch.
- `update_sample_playback_state`: Copies pointer and index values from a source structure to a destination structure and calculates a new memory address based on an offset.
- `accumulate_sample_value`: Adds a value to a destination, updates a counter, and fetches the next sample from a table to dispatch via a table.
- `copy_longword_and_advance_index`: Clears a byte, copies a longword from one block field into two other locations, sets two words to one, increments a 16-bit index, then loads a longword from a table at that index and dispatches through the dispatch table.
- `clear_flag_load_word_and_dispatch_2`: Clears a byte in a record, copies a word from that record into a global variable, and increments a 16-bit counter.
- `calculate_sample_sum`: Calculates a sum of two values, stores it in multiple locations, increments a counter, and fetches the next sample to dispatch via a table.
- `process_sample_split`: Calculates a halved value, updates several fields in a structure, increments a counter, and fetches the next sample to dispatch via a table.
- `shift_sample_value`: Shifts a value left by eight bits, stores it in a structure, increments a counter, and fetches the next sample to dispatch via a table.
- `copy_sample_data`: Copies a value into multiple locations within a structure, increments a counter, and fetches the next sample to dispatch via a table.
- `store_sample_longword`: Stores a longword into a structure, increments a counter, and fetches the next sample to dispatch via a table.
- `update_sample_state_2`: Copies values from one structure to another, increments a counter, fetches a new sample from a table, and dispatches it via a jump table.
- `process_sample_increment_2`: Stores a byte and a shifted value into a structure, increments a counter, and fetches a new sample from a table to dispatch via a jump table.
- `reset_and_fetch_sample`: Increments a counter, clears several fields in a structure if a condition is met, and fetches a new sample from a table to dispatch via a jump table.
- `dispatch_indexed_handler_call`: Increments a 16-bit counter in a global block, and if a flag byte is non-negative, loads a longword from a table at the counter-indexed slot, clears a byte in another block, and calls a handler selected by that byte from a jump table.
- `dispatch_table_handler_step`: Copies a byte from a block field into a global variable and another field, updates an accumulator from a word field, then advances a 16-bit counter and loads a longword from a table at the counter-indexed slot.
- `clear_flag_load_word_and_dispatch`: Clears a byte, copies a word from a global block into a global variable, increments a 16-bit index counter, and returns early if a flag byte is negative.
- `select_table_entry_and_store`: Selects one of several fixed table addresses based on bit tests of a global variable, copies a word from that table at a fixed offset into a global variable twice, writes 255 to the first byte of the selected table, and returns the input data register value unchanged.
- `conditional_counter_table_dispatch`: Sets or advances a 16-bit counter depending on a byte comparison, then loads a longword from a table at the counter-indexed slot.
- `decrement_or_reload_countdown_byte`: Sets a byte to 255 and increments a 16-bit index when the byte is zero; when it is 255 it reloads the byte from a block field minus one, otherwise it decrements the byte, then loads the next table entry and dispatches through the dispatch table.
- `advance_counter_dispatch_with_flag`: Stores a fixed 255 byte into a global block field, increments a 16-bit counter, and loads a longword from a table at the counter-indexed slot.
- `store_decremented_byte_and_index`: Stores one less than a byte from a block field into a global block field, copies a word from another field into the 16-bit counter, and loads a longword from a table at the counter-indexed slot.
- `process_state_transition`: Updates a counter or a byte value based on several conditional checks and then calls a routine from a dispatch table.
- `process_sample_and_dispatch`: performs arithmetic on a value in a table, increments an index, fetches a longword from a table, and calls a routine from a dispatch table based on a byte value.
- `calculate_scaled_sample`: Performs arithmetic to scale a value from a table and stores the result in a memory location before dispatching to another routine.
- `update_sample_index_and_dispatch`: copies a byte to a table, increments an index, fetches a longword from a table, and calls a routine from a dispatch table based on a byte value.
- `store_word_fields_advance_counter`: Copies a word from a block field into a global block field, conditionally copies it into a second location when a flag word is zero, and increments a 16-bit counter.
- `clear_fields_and_dispatch`: clears several memory locations, increments an index, fetches a longword from a table, and calls a routine from a dispatch table based on a byte value.
- `update_sample_state`: Updates a state structure with new pointer and index values, decrements a counter, or resets fields before fetching the next sample and dispatching via a table.
- `process_sample_increment`: Copies data from a source to a destination, increments a counter, and fetches the next sample from a table to dispatch via a table.
- `dispatch_command_byte_to_state`: Takes a command byte from a data register and, depending on its value, updates several per-channel state bytes and words in a table indexed by the low two bits of the high byte of the input, or stores a value in a global variable; for certain command ranges it also writes a lookup-table entry and sets a flag.
- `clear_audio_and_state_variables`: Writes zero to a set of memory locations and loads two global variables with 1920, then clears several other globals and one memory byte.
- `reset_playback_state_variables`: Clears a set of globals and memory bytes, loads several state words from indexed tables, and fills a block of memory locations with a pointer, a 16-bit value and a zero longword in a descending loop.
- `set_fields_from_table_lookup`: Sets a flag byte to one, stores a low byte from a data register into a global, and writes a global to a value formed by adding a constant to a longword read from an indexed table.
- `initialize_hardware_fields`: writes specific values to several memory locations and hardware registers.
- `set_masked_word_and_clear_byte`: Stores the low five bits of a data register, OR'd with a constant, into a global word, clears a global byte, and then calls another routine that resets playback state.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_hardware_and_call_subroutines`: Sets a global variable to zero, modifies hardware color registers, executes a sequence of four subroutines, updates the global variable with a series of values, and returns the result of a final subroutine call.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_hardware_event`: Sets a bit in a global variable and enters a loop that waits for a specific bit to be cleared before resetting the variable.
- `nop_global_variable`: performs a no-operation on a global variable.
- `toggle_global_bit`: Sets a bit in a global variable, calls another routine, and then decrements the global variable.
- `wait_for_bit_set`: sets a bit in a global variable and enters a loop that waits until that bit is set.
- `nop_placeholder`: This routine is empty and performs no operations.
- `update_bitmask_variable`: Calculates a bitwise NOT of a bit shifted by a value derived from a register and stores the result in a global variable.
- `nop_padding`: This routine is empty and contains no executable instructions.
- `update_global_state`: Performs multiple bitwise and assignment operations on a global variable and calls a routine that waits for a specific bit to be set.
- `initialize_data_table`: Populates a memory range starting at a specific address with a repeating pattern of incrementing values and specific constants using a loop.
- `copy_color_data`: Copies a block of sixteen bytes from a source address to a global variable, affecting a hardware color register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine`: This routine is empty and performs no operations before returning.
