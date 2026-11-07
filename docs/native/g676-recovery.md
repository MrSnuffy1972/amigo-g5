# Game 676 (Storm Master): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00676). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902704 top-level bytes in 50 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 427 of 427 named (0 author symbols), 427 of 427 described (18 by rule, 406 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5257 of 5257 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8187 of 9098 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 38686 |
| code | 36552 |
| data | 768727 |
| padding | 10611 |
| unused | 48128 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Storm Master.adf:T.X | 4 | 8760 | 417 |
| Storm Master.adf:Crystal | 2 | 85 | 3 |
| Storm Master.adf:bootblock | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 229 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- Storm Master/Storm Master.adf:FINISH.CO: high-entropy data file (7.64 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:CONSEIL.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:EVEQUE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:TLEO.CO: high-entropy data file (7.69 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:MAIN.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:AVENT.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:VOL.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:ACCUEIL.CO: high-entropy data file (7.71 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:MASTER.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:SANCHO.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:APROJEC.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:COMMER.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:POLIT.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:PARCH.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:BVENT.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:OFFICIER.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:BOSTROM.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:MC2.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:ANUAGE.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:TEXTE.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:MESSE.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:LEO.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:LOGO.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:TEXTE2.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:FINISHG.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:PRES3.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:EXPLOSE.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:CARTE.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:ASHIP.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- Storm Master/Storm Master.adf:PANNEAU.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 201 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Storm Master/Storm Master.adf: blocks 1738-1759 (bytes 889856-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.35)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Storm Master.adf:T.X

- `program_entry_jump`: The program entry point, consisting of a single instruction that transfers control to another routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_immediately`: Performs no operations and returns straight away.
- `allocate_memory_and_init_state`: Sets a global variable to one, opens a library and allocates memory through library calls, and on failure displays an out-of-memory text and returns zero.
- `open_keyboard_input_device`: Finds the current task, adds a port structure, and opens the keyboard input device, returning zero on failure.
- `call_four_routines_in_sequence`: Calls another routine four times in succession and then returns.
- `copy_global_into_register_slot`: Copies the value of a global variable into a fixed storage location and returns.
- `return_without_action_5`: This routine contains a single return instruction and performs no operations.
- `setup_relocation_tables`: Initializes several global pointers from an offset in a structure, allocates memory through a library, and sets up pointer offsets, zeroed words, and display-related values.
- `copy_block_and_reset_state`: Copies a block of words from one global region to another, then sets several global pointers and counters from the copy's end and fixed values.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `search_word_table_for_value`: Scans a table of 16-bit words, comparing each entry against the value in the data register, and returns the index of the first match or negative one if none is found.
- `return_without_effect`: Returns to its caller immediately without touching any data, registers or memory.
- `scale_table_entries_into_longs`: Multiplies a value from a global table by a scalar and adds entries from a source table, storing each 32-bit result into a destination table as two 16-bit halves.
- `update_pointer_and_string_tables`: Walks a table of word entries, writing character bytes into a buffer and storing pointer and value fields into a second table until a sentinel value is reached.
- `return_without_action`: This routine consists of a single return instruction and performs no operations on any data.
- `call_indirect_function_pointer`: Calls a function through a pointer read from a global variable at a fixed offset, then returns the data register value.
- `wait_until_flag_set`: Busy-waits, repeatedly reading a global variable until bit 4 is set, then returns.
- `wait_for_global_bit`: Busy-waits, repeatedly reading a global variable until its lowest bit is set, then returns the data register value unchanged.
- `set_interrupt_enable_bit`: Calls two routines, then stores a fixed byte value into a global variable.
- `transform_word_table_entries`: Walks a table of pointers and, for each of four pointed-to blocks, writes four 16-bit halves of a computed value (a 16-bit multiplication plus an offset read from a second table), storing them at strided positions.
- `initialize_serial_and_call_handlers`: Sets a bit in a global variable, writes two byte values through a pointer, and calls a series of routines, including one through a function pointer from a global table.
- `call_routines_then_update_globals`: Calls another routine twice, then stores constants into a global variable, calls a routine through a pointer read from the global variable, and stores another global value before calling a second pointer.
- `zero_globals_and_call_pointer`: Clears a global variable, calls a routine through a pointer read from a table at an offset given by that global, stores a handler pointer into another global, and calls a second pointer; it also brackets the work with supervisor-state entry and exit library calls.
- `call_pointer_routines_sequence`: Calls a function through a pointer taken from a global variable, stores its result into another global variable, then calls a second function through a pointer with the caller's address register passed along.
- `increment_counter_and_call_four`: Increments a 32-bit global counter, then calls one routine four times in a countdown loop from three down to zero, followed by three further routine calls.
- `clear_expired_channel_flags`: If a global flag is clear, it indexes a per-channel record using a register value and decrements that record's countdown byte when it is nonzero and the associated flag is set.
- `update_paired_channel_registers_2`: Reads a record and a channel index, then either stores a flag and value into indexed global tables or updates indexed hardware-register words for that channel.
- `return_immediately_empty`: This routine is empty: it contains only a return instruction and performs no operations.
- `return_without_action_2`: This routine is empty: its body consists only of a return instruction, so calling it performs no work.
- `return_immediately_empty_2`: This routine contains only a return instruction and performs no operations, though it is entered during execution from outside the static flow.
- `return_without_action_3`: This routine is empty: it contains only a return and performs no work.
- `expand_tagged_records_to_buffers`: Scans a table of linked records and, for each entry whose tag byte is 1 or 2 and whose secondary flag is 1, copies a block of 16-bit words into a chained set of buffers with fixed row-length wrapping.
- `copy_bitplane_tiles_to_buffers`: Walks a table of records and, for each one, copies 16-bit words from a source block into four separate destination buffers with a fixed row stride, or else runs an LZ-style bit-stream decoder that unpacks compressed words into the same kind of buffers.
- `return_constant_3000`: Returns the constant value 3000 in the result register and does nothing else.
- `tiny_move_rts`: Small routine of 2 instructions that copies 100 to d0, then returns.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies 0 to d0, then returns.
- `return_duplicated_global_byte`: Returns a 16-bit value formed by placing a global variable's byte in both halves of the result, and reads a timer-related hardware register.
- `return_immediately_2`: Performs no operations and returns immediately.
- `copy_block_until_null`: Walks a table of source and destination pointer pairs, copying a fixed block of words from each source to its destination until a null source pointer is found.
- `call_indirect_function`: Calls another routine, then makes an indirect call through a table entry in a global variable at a fixed negative offset, and returns the concatenated contents of two registers.
- `call_draw_and_move`: Calls another routine, then calls the graphics Draw and Move library functions through library pointers stored in a global variable.
- `call_indirect_through_table`: Calls another routine, then calls a routine loaded through a pointer read from a table at a fixed offset, and returns the values held in the data registers.
- `empty_routine`: Contains no executable operations and simply returns.
- `store_byte_and_call_graphics`: Stores the low byte of the data register into a global variable, then calls a graphics library routine through an offset from a stored base pointer and returns the register value.
- `store_byte_and_call_indirect`: Stores the low byte of the data register into a global variable, then calls a routine through a function pointer obtained from a stored base pointer and offset, and returns the register value.
- `remap_value_and_store_global`: Replaces a register value of 3 with 4, stores the low byte of the result in a global variable, then calls a graphics library routine through a stored base pointer and offset.
- `iterate_until_value`: Calls a routine repeatedly until it returns a non-zero byte, then calls a second routine repeatedly until it returns zero, and returns the first result.
- `dispatch_char_and_update_state`: Stores the low byte of the data register into a global variable, updates a few global counters based on whether that byte is a carriage return, a line feed, or another value, and then calls routines through function pointers obtained from stored bases.
- `traverse_string_null`: Walks a null-terminated buffer, calling another routine for each non-null byte until the terminator is reached.
- `clamp_and_scale_two_globals`: Clamps two register values to upper limits of 39 and 24 respectively, then stores each multiplied by a constant (9 and 8) into two global variables.
- `return_immediately_empty_3`: This routine is empty: it performs no operations and returns to its caller at once.
- `update_clamped_position_values`: Reads a global variable, adds a value derived from it to two global 16-bit variables, clamps each within fixed minimum and maximum bounds, and stores a masked bit pattern of the first variable into a third global variable.
- `double_input_register_values`: Doubles the low 16 bits of two input registers and stores each result in its own global 16-bit variable.
- `return_halved_global_value`: Returns a global 16-bit variable shifted right by one bit.
- `scan_bits_and_reset_flags`: Scans a 12-entry global block for the first differing bit position between paired bytes and, when found, copies a selected global byte into a global variable.
- `call_handler_and_set_flags`: Calls a function through a pointer stored in a global variable, then rebuilds a global flag byte by testing bits of a status byte and setting corresponding bits in the flag byte.
- `encode_port_state_flags`: Reads a global variable and a sign-tested byte, then sets bits in the low word of a result longword according to the state of those flags.
- `encode_mouse_movement_flags`: Reads a global variable and a sign-tested register byte, then sets bits in the low word of a result longword to encode the state of the mouse port movement.
- `map_port_bits_to_flags`: Reads a global variable and a sign-tested register byte, and sets bits in a result longword to encode the state of the joystick port movement register.
- `return_immediately_empty_stub_2`: The routine is empty: it contains no work and only returns to its caller.
- `return_immediately_empty_stub`: This routine is empty: it contains no work and simply returns to its caller.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `call_library_and_map_zero`: Calls a library function through a stored base pointer and replaces a zero result with minus one, returning that value.
- `return_zero_value`: Returns zero and does nothing else.
- `lock_and_unlock_conditionally`: Calls a function through a pointer from a global variable, and if it returns nonzero, calls a second function through another pointer from the same variable with an argument register.
- `open_and_close_file`: Calls a function through a pointer from a global variable, and if a global flag byte is nonzero, calls two further functions through pointers from the same variable and then loops indefinitely.
- `call_library_fallback_minus_one`: Calls a library function through a stored base pointer and returns its result, substituting minus one when that result is zero.
- `open_file_or_fail`: Calls a routine and then invokes a library function through a pointer from a table; if the result is zero it returns minus one, otherwise it returns the result.
- `call_delete_file`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `call_external_function_pointer`: Calls a function through a pointer located at a fixed negative offset from a base pointer held in a global variable.
- `write_then_set_flag`: Calls a routine, invokes a library function through a pointer from a table, and then sets a global variable to one.
- `wait_for_disk_prompt_loop`: Loops until a global variable is non-zero and not equal to a sentinel, then checks a stored key code.
- `move_and_clear_data`: Walks four source pointers in nested loops, copying each 16-bit word to a destination table and writing zero to the source location, then advances all pointers by a fixed row stride.
- `copy_memory_block`: Walks a table and four destination pointers in nested loops, copying 16-bit words from the table to the destination locations and advancing all pointers by a fixed row stride.
- `return_immediately_stub`: Contains only a return instruction and does nothing else.
- `poll_message_until_flagged`: Repeatedly calls a library function through a stored base pointer to fetch a message, then checks a flag field in the returned structure, looping until the flag bit is set.
- `return_stack_slot_value`: Loads a value from the stack frame at the caller's return position and returns it without modifying any state.
- `lock_directory_or_run_fallback`: Invokes a library function through a pointer from a table; if it returns zero it calls two routines in sequence, otherwise it stores the returned value in a global longword and calls another library function.
- `return_zero_value_2`: Sets the return value to zero and returns, with no other effects.
- `wait_for_flag_scale`: Repeatedly calls a routine until a global variable is no longer minus one, then computes the difference of two globals, multiplies it by a global word, and divides by one thousand.
- `lock_and_inspect_volume`: Calls a library function through a stored base pointer and stores the returned value in a global variable if nonzero, then calls two further library functions through the same base; otherwise it returns minus one.
- `swap_saved_rect_and_flag`: Saves five words of a global block and a global counter, compares a global flag's sign to select one of two global values, and if the counter differs from the selected value it resets the five-word block to fixed constants, calls another routine, swaps the counter with a second global, restores the saved words, and calls another routine twice.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `swap_saved_rect_and_flag_2`: Performs the same operation as another routine in this set: it saves five words of a global block and a global counter, conditionally resets the block to fixed constants and calls another routine, swaps the counter with a second global, restores the saved words, and calls another routine twice.
- `swap_saved_rect_and_flag_3`: Repeats the same conditional save, reset, swap, restore and double call sequence over a five-word global block and a global counter, returning the saved data registers unchanged.
- `swap_and_redraw_viewport`: Checks a global flag to choose one of two stored dimension values, and if it differs from a stored size, saves a rectangle of five global values, swaps two stored size values, restores the saved rectangle, and calls another routine twice.
- `swap_and_redraw_viewport_2`: Checks whether a global variable differs from one of two alternate values chosen by the sign of another global variable.
- `reset_or_swap_viewport_params`: Saves six global words, then compares a global longword against one of two stored constants chosen by the sign of another global.
- `swap_and_initialize_params`: Saves six global words, and if the value in the data register differs from a stored constant, it sets five global words to fixed values.
- `update_buffer_indices_recursively`: Recursively copies a five-word entry between two pointer-based tables, adjusts the copied values by a scaled offset derived from a global variable, and updates the buffer start and end index globals.
- `adjust_table_entries_by_scaled_value`: Computes a scaled negative value from a global variable and adds it to five consecutive 32-bit entries of a table, then calls another routine.
- `reset_block_and_call`: Writes fixed constants into a five-word global block, then calls another routine and returns the saved data registers unchanged.
- `reset_block_and_call_alt`: Writes the same fixed constants into the five-word global block and calls another routine, returning the saved data registers unchanged.
- `set_fixed_block_values`: Writes five fixed constant words into a global block and then calls another routine.
- `allocate_and_initialize_header`: Allocates a block and, if the allocation succeeds, computes several sequential offsets and sizes from global values and a structure field, then writes a header value and dimension words into the block.
- `allocate_block_and_write_header`: Computes a row size from a structure field, derives several sequential offsets and sizes from global values, stores the input pointer in a global, writes a fixed word and two differences into the block header, and stores a size value.
- `allocate_block_from_pool`: Subtracts a size computed from a register from a global pointer after rounding it, and if the result stays above a limit, stores the size word at the new pointer and returns that pointer advanced past the header; otherwise it returns zero.
- `allocate_block_from_pool_alias`: Performs the same rounded size subtraction from a global pointer, storing the size word and returning the advanced pointer when within limit, or zero when the limit is exceeded.
- `allocate_block_from_pool_alias_two`: Repeats the identical size-rounded pool subtraction, writing the size word and returning the advanced pointer on success, or zero on failure.
- `allocate_block_from_pool_alias_three`: Carries out the same rounded size subtraction against a global pointer with a bounds check, storing the size word and returning the adjusted pointer or zero.
- `allocate_block_from_pool_alias_four`: Applies the same size-rounding pool allocation: decrements a global pointer, stores the size word on success and returns the advanced pointer, otherwise returns zero.
- `allocate_from_pool_downward`: Rounds an input size up and subtracts it from a global pointer, storing the size word and returning the advanced pointer when the result stays above a global limit, otherwise returning zero.
- `return_stack_argument`: Returns a 32-bit value read from the stack argument area without modifying any data.
- `check_word_in_range`: Reads a 16-bit value from another routine and returns 0 if it lies strictly between two fixed bounds, otherwise returns 65535.
- `return_constant_word`: Returns a fixed 16-bit constant value.
- `save_hardware_and_call_helpers`: Stores an address register into a global variable, then calls six other routines in succession and returns a word from a data register.
- `save_and_reset_hardware`: Copies several hardware and global values into saved variables, then writes to the interrupt, DMA, ADKCON and CIAB registers to disable interrupts and DMA and set control values.
- `clear_bits_and_wait`: Clears bits in a global variable, then busy-loops a 100-iteration counter while repeatedly clearing another bit, and finally calls two other routines.
- `delay_countdown_loop`: Counts a 16-bit variable down from 3000 to minus one, busy-waiting for a fixed number of iterations.
- `wait_while_flag_set`: Loops repeatedly until a bit of a global variable, masked with 32, is clear, then returns.
- `poll_flag_until_clear`: Repeatedly sets low bits of a global variable and calls another routine while a specific bit remains set, then returns.
- `return_immediately_3`: An empty routine that performs no work and returns straight away.
- `repeat_pulse_and_busy_wait`: Waits while one bit of a global variable is set by repeatedly calling another routine, then performs a fixed number of passes that clear and set low bits of a global variable around calls to another routine, followed by a long countdown delay.
- `pulse_bits_with_delays`: Runs a 79-iteration loop that clears and sets bits in a global variable while calling a routine each pass, then busy-waits a long countdown before returning.
- `count_down_delay_loop`: Decrements a 16-bit counter starting at 15000 until it reaches minus one, then returns.
- `pulse_serial_port_until_ready`: Repeatedly clears a bit in a global variable, writes control words and a longword to a custom chip block, then polls a status register for a bit with a bounded count before writing a control value.
- `wait_for_flag_bit`: Spins in a loop until a specific bit in a global variable becomes set, then returns.
- `mask_global_and_write_stack_words`: Clears one bit in a global variable and sets its top bit, then writes four words derived from global variables with a constant ORed in to stack offsets.
- `scan_stream_for_sync_word`: Scans a table of longwords bit by bit for a fixed 16-bit sync pattern, and once found repeatedly calls another routine while a global byte is non-zero.
- `loop_while_global_nonzero`: Repeatedly calls another routine while a global byte is non-zero, then returns.
- `scan_longs_for_sync_word`: Scans a sequence of longwords bit by bit for a fixed 16-bit marker value, and on a match stores the word index through an address register and sets a global flag to 1, or clears the flag when the destination is the global variable itself.
- `store_word_and_toggle_flag`: Stores a 16-bit value from a register through an address register, then sets a global variable to 1 or 0 depending on whether the address register equals that global variable's address.
- `tiny_nop`: Small routine of 1 instruction that does nothing for one instruction.
- `return_immediately_empty_handler`: This routine is empty: its only instruction returns to the caller, so it reads and writes no data and makes no calls.
- `add_offsets_on_nonzero_result`: Calls another routine that returns a 16-bit value and, when that value is non-zero, adds fixed constants to two global 32-bit values.
- `return_concatenated_registers`: Calls another routine, then returns a 64-bit value formed by concatenating two 32-bit register values, with the first register in the upper half.
- `call_single_routine_return`: Calls one other routine and then returns to its caller.
- `clip_and_blit_masked_tile`: Clips a rectangular 16-pixel-wide region against bounds held in global variables, then copies bitplane words from a source table into the destination planes with a shifted mask and a precomputed clip-mask table.
- `copy_pixel_words_rows`: Computes a source offset from a global, selects one of several pointer tables, and copies groups of 2-byte words from a source buffer into four destination row pointers in a loop.
- `tiny_ori_2`: Small routine of 1 instruction that merges 117 into d0.
- `setup_blt_params`: Computes a destination address from table entries and global variables, then writes masks, modulo, control and pointer values to blitter hardware registers, waiting for the blitter busy bit to clear between writes.
- `update_blt_attributes`: Loops over a count derived from a global variable, writing blitter pointer and size values to hardware registers while waiting for a busy flag to clear on each pass.
- `copy_tiled_words_to_planes`: Copies a rectangular block of 16-bit words from a source table of plane pointers into four destination bitplanes, one word from each plane per iteration, advancing all pointers row by row with a stride adjustment.
- `start_blitter_window_transfer`: Computes a clipped window width from global bounds, waits on the blitter busy flag, programs blitter registers with masks, modulos, source and destination pointers and sizes, and then starts a blit operation.
- `conditionally_start_blit`: Tests two bits of a byte at a pointer; when both are clear, it computes an address from registers, writes blitter mask, control and pointer values, and starts the blit after waiting for the busy flag.
- `compute_blit_window_mask_table`: Computes a clipped horizontal blit window from pointer-based record bounds and fill-width values, then loops over the window writing bytes read through a pointer into a table indexed by the source values.
- `clip_and_store_sprite_bounds`: Reads a record pointed to by the argument block, tests its coordinates against several global limit variables, and returns early if the record lies outside them.
- `clip_and_queue_blit_window`: Reads a record through a pointer held in the input structure, clips its horizontal and vertical bounds against stored global limits, and stores the clipped window into global variables.
- `store_flag_through_pointer_check`: When a global byte is zero and an address register matches one of four reference addresses, it stores a global byte through that address register at a fixed offset.
- `configure_audio_channel`: Enters supervisor state through a library call, then computes a channel bit mask and a clamped volume value and writes volume, period and a sample pointer to the audio hardware registers.
- `clear_channel_flag_if_negative`: Tests a per-channel flag byte and, if its sign bit is set, clears that flag byte, clears a byte through a pointer, and computes a channel bit mask shifted left by seven into a global variable.
- `test_bits_and_reset_global`: Masks a global variable with itself and recursively calls another routine for each of four specific bits that are set.
- `decrement_channel_counter`: Decrements a per-channel word counter through a pointer and, when it reaches zero, clears a byte through a pointer, clears the per-channel flag byte, and computes a channel bit mask shifted left by seven into a global variable.
- `call_routine_four_times`: Calls another routine four times in succession and returns.
- `save_registers_then_call`: Calls a neighbouring routine that saves four register values into global variables, calls a routine through a function pointer from a global table, and clears a block of memory.
- `conditionally_call_interrupt_update`: If a global variable is non-zero, it calls another routine that updates interrupt state from a word in a data register, then returns the value held in the data register.
- `forward_clamped_value_to_globals`: Passes a signed word from a data register to another routine, which clamps it to a maximum of 100 and stores it in several global variables and a hardware volume-related register.
- `compute_scaled_seven_bit_value`: Masks a value in a register to seven bits, multiplies it by eight, subtracts seven, and stores the result in a global variable.
- `conditionally_set_handler_pointer`: If a global byte is non-zero, stores a handler pointer into a global variable and writes two constant words to other global variables.
- `clamp_value_and_store`: Clamps a signed word from a data register to a maximum of 100, then stores it to several global variables and to a hardware volume-related location.
- `configure_audio_channel_registers`: Writes a pointer, a word value and a constant control word into custom chip registers through an address register, then stores derived values into global variables and selects one of two constant words depending on the sign of an input value.
- `write_two_words_via_address`: Writes two constant words through an address register to fixed offsets within a structure.
- `setup_audio_interrupt_state`: Saves four register values into global variables, calls a routine through a function pointer held in a global table, and clears a block of global bytes.
- `update_interrupt_state_from_word`: If a global variable is non-zero, it branches on the sign and zero state of a data register word: for negative values it stores a fixed constant and a masked byte into globals, for zero it saves a global, clears bits and fixed values into globals and a byte, and for positive values it stores other constants and conditionally reloads a global.
- `conditionally_set_control_bits`: If a global byte is non-zero, it stores a handler pointer into a global variable and writes two constant words to global variables.
- `dispatch_on_interrupt_status_bits`: When a bit in a global variable is set, ANDs it with another global word and jumps through one of four function pointers according to which of four bits is set.
- `dispatch_by_flag_bits`: Tests bits of a data register and, depending on which is set, makes an indirect call or jump through a pointer stored in a global variable, otherwise it returns the stack value unchanged.
- `dispatch_call_on_flag_bits`: Tests two bits of a data register and, if bit 7 is set, makes an indirect call through a pointer stored in a global variable; otherwise, if bit 9 is set, it makes a second indirect call through another global pointer.
- `conditionally_jump_through_pointer`: Tests bit 9 of a data register and, if it is set, makes an indirect call through a pointer stored in a global variable and returns its result; otherwise it returns a value from the stack unchanged.
- `dispatch_on_masked_flag_bits`: Ands two words through an address register with a mask and, depending on which of four bits is set, jumps indirectly through a pointer to one of four other routines.
- `initialize_interrupt_vector_state`: Sets two global variables to fixed constant values, stores a handler address and zero into fields of a structure pointed to by an address register, and clears a global variable and another handler slot.
- `clear_custom_chip_registers`: Writes a fixed value to a global variable, combines it with a constant, and stores zeros to two hardware-register offsets relative to the pointer passed in an address register, then clears two further globals.
- `clear_custom_interrupt_vectors`: Writes a zero-valued word through a pointer register into the hardware interrupt vector slot at offset 154 and a second slot at offset 150, then clears two global variables and stores a null handler pointer into a structure offset, after first setting a few global bits.
- `clear_custom_chip_sprite_registers`: Writes a value to a global variable and a register at offset 154 through a pointer, zeroes two further globals, and stores a value into a field at offset 68 of an indirectly referenced structure.
- `clear_copper_and_audio_vectors`: Clears two stored handler pointers and a global variable, then stores the handler pointer and a zeroed pointer into hardware-register offsets of a structure addressed by an address register.
- `configure_custom_chip_registers`: Writes a fixed set of values to a global variable and stores zero and a handler pointer into hardware-style registers addressed through the address register, then zeroes two more globals.
- `configure_custom_chip_registers_2`: Performs the same sequence as the other routine: writes fixed values to a global variable, stores zero and a handler pointer through the address register into register-offset locations, then zeroes two more globals and stores a pointer into a field of a table entry.
- `initialize_timer_and_pointers`: Writes constants to global variables, stores a routine pointer and a value into a structure through an address register, and clears two global variables and a pointer.
- `advance_music_event_stream`: Decrements a countdown timer and, when it expires, reads the next command byte from a data stream and dispatches on its type.
- `advance_sound_stream_and_dispatch`: Decrements a countdown held in a global variable and, when it expires, reads the next command byte from a sound command stream and dispatches on its code through a switch.
- `parse_sound_command_stream`: Reads bytes from a sound command stream in a loop and dispatches on each command code through a switch.
- `process_sound_channel_commands`: Runs a byte-driven command loop that dispatches on a command code through a switch.
- `dispatch_sound_command_stream`: Reads bytes from a sound command stream through a pointer held in a global variable and dispatches on each command code via a switch.
- `store_handler_pointer_and_advance`: Adds 4 to a global counter, stores an input argument into another global variable, and writes a computed code pointer (the low byte of a data register minus 128) into a field of a structure reached through a global pointer.
- `dispatch_sound_command_stream_2`: Reads bytes from a sound command stream in a loop, storing per-channel values into a table and writing hardware audio channel registers for sample-based playback.
- `advance_sound_channel_stream`: Reads sound command bytes from a stream through a pointer, looping on command codes via a switch; for some codes it selects a per-channel table entry and programs the audio channel's location and length registers, then updates channel bookkeeping and returns to the caller.
- `dispatch_sound_command_stream_3`: Reads command bytes from a sound command stream through a pointer and dispatches on each command code using a switch.
- `start_sound_channel_playback`: Reads a sound command stream and, for the non-zero-flagged path, loads a channel's audio pointer and length from a table and writes them to the Paula channel 0 registers, then ORs a bit mask into two global flag words.
- `decode_command_stream_to_custom_regs`: Reads a byte-coded command stream through a pointer, looping over it and dispatching on each command byte.
- `dispatch_sound_command_byte`: Reads a byte from a sound command stream, handles a 255 escape by fetching a following byte, and either returns a command offset to its caller or dispatches on the command code through a jump table.
- `set_bit0_pointer_and_width`: If bit 0 of a byte at offset 3 of the object is set, computes a width from two fields before the pointer and stores it at offset 164, stores the pointer plus a field at offset 160, and sets a word at offset 154 to 128.
- `set_bit0_handler_and_mask`: If bit 0 of the byte at offset 3 of the object is set, stores a routine pointer into a global variable.
- `set_mode1_and_masks`: Stores the value 1 at offset 150 and the value 128 at offsets 154 and 156 of the object.
- `set_bit1_pointer_and_width`: If bit 1 of a byte at offset 3 of the object is set, computes a width from two fields before the pointer and stores it at offset 180, stores the pointer plus a field at offset 176, and sets a word at offset 154 to 256.
- `set_bit1_handler_and_mask`: If bit 1 of the byte at offset 3 of the object is set, stores a routine pointer into a global variable.
- `set_mode2_and_masks`: Stores the value 2 at offset 150 and the value 256 at offsets 154 and 156 of the object.
- `set_bit2_pointer_and_width`: If bit 2 of a byte at offset 3 of the object is set, computes a width from two fields before the pointer and stores it at offset 196, stores the pointer plus a field at offset 192, and sets a word at offset 154 to 512.
- `set_bit2_handler_and_mask`: If bit 2 of the byte at offset 3 of the object is set, stores a routine pointer into a global variable.
- `set_mode4_and_masks`: Stores the value 4 at offset 150 and the value 512 at offsets 154 and 156 of the object.
- `decrement_counter_set_bit3_mode`: If bit 3 of a byte at offset 3 of the object is set, decrements a global counter and, when it becomes negative, stores the value 8 at offset 150 and 1024 at offset 154.
- `return_immediately_empty_5`: This routine is empty and only returns immediately without touching any data.
- `dispatch_handler_stub`: The routine is a single-instruction stub that is referenced by address and by a stored handler pointer, and is reached from two other routines.
- `copy_byte_via_pointer`: Moves a value through absolute-addressed memory operations and multiplies by a constant, with its handler pointer stored in data.
- `return_immediately_no_op`: The routine consists of a single instruction and performs no work on its data.
- `clears_and_subtracts_memory`: This single-instruction routine is reached through a handler pointer and is referenced by three other routines via absolute addressing.
- `clears_and_ors_memory`: This single-instruction routine is reached through a handler pointer and is referenced by three other routines via absolute addressing.
- `moves_value_to_memory`: This single-instruction routine is reached through a handler pointer and is called by six other routines.
- `moves_value_from_memory`: This single-instruction routine is reached through a handler pointer and is called by five other routines.
- `jump_to_call_four_routines_in_sequence`: Jump stub: passes control straight on to call_four_routines_in_sequence.
- `update_linked_record_entries`: Initializes several global pointers and counters, then walks a linked list of records, decrementing per-record counters and, when one reaches zero, swapping stored values and calling helper routines.
- `dispatch_indirect_jump_table`: Reads a byte from a pointer, scales it by two, and uses it as an offset into a table to jump indirectly to the handler found there.
- `increment_and_store_pointer`: Returns a byte from a pointer and stores the incremented pointer into a memory location.
- `read_three_bytes_advance_pointer`: Reads three consecutive bytes from a pointer, stores the pointer advanced by three at a stack offset, and returns the three bytes packed together.
- `read_short_at_pointer`: Returns a short integer read from a memory address held in a register.
- `read_three_bytes`: Reads three consecutive bytes from a pointer and returns them as a single packed value.
- `clear_linked_list_node`: Checks if a global variable is zero and compares a value against another global before potentially zeroing a short integer at a specific offset from a pointer.
- `get_dereferenced_value`: Stores a pointer in a global variable, calls another routine, and returns a value from that pointer or a returned short.
- `select_pointer_byte_when_zero`: When the test word in a data register is zero, returns the byte at the address register; otherwise returns the value already in the result register.
- `conditionally_return_pointer_target`: If a certain flag is zero, it returns a value from a pointer, otherwise it returns a register value.
- `conditionally_return_three_bytes`: If a certain register value is zero, it builds a 32-bit value from three bytes at a pointer and returns it; otherwise it returns the incoming value unchanged.
- `conditionally_return_pointer_byte`: If a certain flag is non-zero, it returns a byte loaded from a pointer, otherwise it returns a register value.
- `copy_register_to_register`: A single-instruction routine that copies one data register into another.
- `handler_eight_instruction_routine`: A routine of eight instructions reached through a handler pointer stored in data.
- `dispatch_indirect_jump_call`: Reads a signed word from a table indexed by a byte pointed to by an address register, adds it to a base value, and calls the resulting address as an indirect jump.
- `call_tiny_move`: This routine consists of a single instruction that calls another routine.
- `execute_eight_instructions`: This routine consists of eight instructions and is called by two unnamed routines.
- `execute_tiny_move`: A three instruction routine that calls another routine and a move routine.
- `update_globals_via_handler`: Calls a handler multiple times to update several global variables and memory locations using a value from a register.
- `store_register_to_globals`: Calls a handler multiple times to store a register value into several global variables and a memory location.
- `execute_two_calls`: Sets a global variable to two and calls another routine twice.
- `iterate_input_stream`: Traverses a null-terminated string by repeatedly calling another routine until the end of the string is reached.
- `set_global_from_function`: Calls two routines, then stores a returned value into a global byte-sized location, substituting 100 when a global flag is nonzero and the first result is zero.
- `process_input_stream`: Parses character data to update various global variables and coordinate tables based on specific control characters.
- `update_linked_list_node`: Searches a sorted table and updates a doubly linked list node by unlinking an existing element and initializing a new structure with modified values.
- `delete_and_reorganize_linked_list`: Removes an entry from a memory block by shifting subsequent data and updates a linked structure by traversing nodes to adjust pointers.
- `remove_linked_list_entry`: Removes an entry from a memory block by shifting subsequent data and updating a linked structure by traversing nodes to adjust pointers.
- `process_linked_records`: Iterates through a data structure to evaluate conditions and update global variables, linked record entries, and indirect jump tables.
- `update_sorted_table_and_offsets`: Searches a sorted table for a specific value and, if found, shifts table elements to remove or adjust entries.
- `append_to_linked_list_2`: Clears a word field in a structure, copies bytes and words between two structures, and links the new entry into a global word-linked chain, ending with a call to a cross-product routine.
- `store_byte_via_offset`: Stores a byte from a source pointer into a global buffer at an offset given by a signed word read from the same pointer.
- `set_global_if_zero`: Checks if a global variable is zero and sets it to one if it is.
- `update_if_different`: Compares a value from a pointer to a value at a calculated offset and updates that offset if they differ.
- `update_table_values`: If a certain value is non-zero, this routine writes two words to memory locations calculated using a global variable and an offset, then sets a high bit in a byte at a pointer.
- `modify_bit_flags`: Clears the second bit and sets the highest bit of a byte located at an offset from a global variable, then appends a node to a linked list.
- `set_bit_in_byte`: Sets a specific bit in a byte located at an offset determined by a pointer relative to a global variable.
- `append_to_linked_list`: Clears a word field in a structure at an offset from the given pointer, then walks a global chain of word-linked entries and appends the new index to the end, or sets the global head if the chain is empty.
- `remove_from_linked_list`: Traverses a linked list to find a specific value and updates a pointer or replaces a node if a match is found.
- `compute_cross_product_components`: Computes three signed byte cross-product components from six byte fields of a structure and stores each result into three further byte fields of the same structure.
- `parse_coordinate_stream`: Parses a data stream to update global coordinate variables through a series of relative offset calculations.
- `parse_coordinate_delta_stream`: Parses a stream of variable-length data to update object positions using relative coordinate offsets.
- `apply_coordinate_delta_stream`: Updates object positions by iterating through a stream of relative coordinate offsets and applying them to current values.
- `process_data_stream`: This routine parses a data stream containing variable-length instruction sets or coordinate updates.
- `sync_and_copy_data_blocks`: Synchronizes a series of word values from a source buffer to two destination locations while optionally appending data from a global variable.
- `validate_table_entry_conditions`: Checks if a value retrieved from a table is non-zero, then verifies that two subsequent table lookups match a value from a global variable and a value from another memory location.
- `compare_table_entry`: Compares a value from a table against a global variable and checks if another table entry matches a specific value.
- `search_linked_list`: Traverses a linked list to check if any node contains a value equal to a global variable and a specific short value.
- `insert_into_list_structure`: Swaps or inserts a value into a linked list structure using a global index, while updating pointers and clearing certain memory offsets.
- `set_byte_to_max`: Stores the value zero-ff into a specific byte offset relative to a pointer.
- `store_byte_via_offset_pointer`: Calls another routine, then stores a byte from a register at a negative offset from an address register.
- `compute_scaled_position_offset`: Computes a position difference between two structures, scales the X and Y components by a bit-shifted factor divided by a third component, and stores the adjusted coordinates and a derived value into global variables.
- `select_conditional_long_value`: Returns a 64-bit register pair where the high word is either unchanged or replaced by a sign-extended byte from a structure, depending on the sign of another byte in that structure.
- `calculate_indirect_offset`: Retrieves a value from a nested memory structure and adds an offset derived from a table and a register.
- `calculate_pointer_offset`: Calculates a pointer or value by adding an offset and a value retrieved from a table or memory location.
- `process_list_nodes`: Updates a global variable based on a condition and iterates through a list to unlink or relink nodes.
- `write_byte_to_memory`: Writes a single byte with the value one to a specific memory location.
- `clear_flag_and_return`: Loops while a 16-bit value read through a pointer is non-zero, calling another routine on each pass that may unlink list nodes.
- `unlink_or_relink_list_node`: Checks a global flag and a node byte; it marks and unlinks the node from a list when the byte is non-negative, otherwise moves the node to a global head pointer, then calls another routine.
- `unlink_list_node`: Locates a node in a word-linked list by index, searching from a global head pointer, and redirects the predecessor's link to skip that node.
- `write_multiple_words`: Writes a specific word value to three consecutive word locations using an indirect handler.
- `add_to_multiple_words`: Adds a specific word value to three consecutive word locations using an indirect handler.
- `copy_two_longwords_to_globals`: Copies two longwords from memory addressed by a pointer register into two global variables.
- `offset_chained_entries_by_delta`: Computes per-axis differences between three words at a pointer and three global values, then if any difference is nonzero, walks a global chain of entries, marks zero flag bytes as 2, and adds the differences to three word fields of each entry.
- `add_offsets_and_clear_flag`: Adds three word offsets from a source table into three word fields of a structure, walks a chain of entries in another structure marking eligible flag bytes as 2, and clears the top bit of a flag byte.
- `wait_and_update_record_tables`: Waits until a global flag is set, calls a set of routines to update global state and fixed block values, and walks linked lists of records held in tables, relinking and copying their fields.
- `update_clipped_record_tables`: Walks linked lists of records held in tables, moving or relinking entries, and when a record's flagged position and size are updated it stores the computed bounds and copies fields into a fresh entry.
- `store_position_fields_into_record`: Calls other routines, then copies a word from one record field to another at a register-derived offset, writes two longwords and a word from global tables into the record, and stores a global byte into a further field.
- `copy_word_and_swap_global`: Copies a 16-bit value from one structure offset to another structure offset, after first calling another routine.
- `copy_fields_into_offset_record`: Calls two other routines, then copies a word within a record at an offset taken from a data register, writes two longwords and a word from global tables, and stores a byte from a global variable, before calling two more routines.
- `initialize_four_global_words`: Sets two global words to the maximum positive value and two other global words to the minimum negative value.
- `expand_global_bounds_from_entry`: If a flag word at an entry is nonnegative, reads a structure referenced from the entry and updates four global min and max words with the entry's coordinates and the structure's extent values, widening the bounds where exceeded.
- `relink_node_in_list`: Walks a linked chain of records, using a pointer offset from a register to find the insertion point by comparing 16-bit and byte fields, then splices the record at that position and clears a flag byte.
- `traverse_records_and_queue_visible`: Walks a linked list of records through pointers held in tables, and for each record that passes a flag test and a bounds check against stored limits, calls another routine and sets a global variable.
- `initialize_buffer_constants`: Copies four 16-bit values from fixed table locations into consecutive words at a buffer pointer, advances the pointer by four words, and calls another routine if the pointer has not passed a threshold.
- `iterate_table_blocks_calling_routine`: Loops over a block of records while the current pointer is below a limit, copying four 16-bit fields from each record into global words and calling another routine for each record.
- `clamp_rect_bounds_to_limits`: Tests four 16-bit values from a record at an offset against stored limit globals and returns early if the record lies outside them.
- `expand_rect_bounds_global`: Checks a record's position and size against stored global bounds and an indirectly referenced offset table, and if they overlap, stores the clamped minimum and maximum values into globals and sets a flag.
- `intersect_region_with_record`: When a global flag is zero, it initializes four global region values from a record's fields and an offset table.
- `store_word_indirectly`: Uses an indirect handler to store a word into a memory location relative to a pointer.
- `process_instruction_stream`: Iterates through a linked list of data structures and executes specific operations based on a command byte.
- `process_linked_list_elements`: Iterates through a linked list of data structures, comparing specific field values against a reference.
- `iterate_table_elements`: Iterates through a table of data using a counter, calling another routine for each non-zero element found.
- `update_bounding_box`: Calculates the minimum and maximum coordinate extents for a set of objects by iterating through a table of data.
- `update_bounding_box_3`: Calculates a bounding box by processing coordinate offsets and comparing them against boundary limits.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `update_bounding_box_2`: Calculates and updates a bounding box by processing coordinate offsets and checking them against predefined boundary limits.
- `traverse_linked_instruction_list`: Iterates through a linked list of data structures by following pointers stored within the elements.
- `build_instruction_sequence`: Iterates through a source data list to populate a new sequence of values into a destination buffer.
- `set_global_one`: Sets a global variable to one.
- `copy_words_backward_pointer`: Reads a count from a block and writes its words and bytes into a destination area, moving backward from a computed offset until the count underflows.
- `search_table_for_value`: Iterates through a table to find a matching short value, using a count from a table entry to limit the search.
- `check_table_range`: Verifies if a value, when added to a base, falls within a range defined by a table entry.
- `empty_stub_return`: An empty routine that does nothing but return.
- `clear_table_entry`: Writes zero to a memory location determined by an offset in a table.
- `clear_byte_and_check`: Sets a byte at a specific offset to zero and returns if a global variable is zero.
- `set_table_entry`: Writes one to a memory location determined by an offset in a table.
- `copy_and_mask_byte`: Copies a word from one location to another and then clears the most significant bit of a byte at a specific offset.
- `clear_lowest_bit`: Clears the least significant bit of a byte at a specific offset.
- `clear_second_lowest_bit`: Clears the second least significant bit of a byte at a specific offset.
- `store_byte_to_global`: Calls another routine, then stores the low byte of a data register into a global variable.
- `update_sorted_table`: Updates a sorted table by inserting a new value or modifying an existing one, potentially shifting elements to maintain order.
- `set_global_to_one`: Calls another routine, stores the low two bits of a data register into a global variable, then overwrites that global with the value one.
- `set_global_pointers_from_value`: Reads a global byte, shifts it left by five, adds it to the address of another global, and stores that computed address into two global pointer variables.
- `copy_data_on_state`: Tests a global state byte against several values and returns early for some; otherwise copies a counted block of words through a global pointer, optionally copies a fixed block, and sets several global bytes based on a register value.
- `clear_global_variable_sequence`: Writes a byte value of 255 to a global variable and then writes zero to it twice, leaving the variable set to zero.
- `check_nibble_sum_overflow`: Adds the low nibbles of two register values and returns early if the sum exceeds fifteen.
- `compare_low_nibbles_conditionally`: Subtracts the low nibble of one data register from the low nibble of another, returning early if the result is negative or at least sixteen and otherwise continuing through the remaining logic.
- `update_state_and_copy_data`: Tests a global state byte against several specific values and returns early if matched; otherwise, it initializes pointers, performs a word-wise copy of a counted block, and optionally copies a fixed-size block.
- `compare_and_pack_nibbles`: Loops 64 times over two parallel word tables in a global area, comparing each pair of words nibble by nibble.
- `copy_words_and_store_nibbles`: Returns early when a global variable holds one of four specific values; otherwise it clears a global, copies a block of 16-bit words from a source pointer into a destination, and conditionally copies a fixed-size block of global bytes.
- `forward_to_another_routine`: Calls another routine and then returns without changing any data.
- `call_single_routine_return_2`: Calls one other routine and then returns to its caller without doing anything else.
- `build_and_store_command_block`: Compares a byte against several table entries and, when none match, writes a command block of bytes and words through a pointer, setting the first byte to a command value.
- `fetch_indexed_table_value`: Accesses a nested data structure by calculating an address through multiple pointer offsets and an indexed register.
- `calculate_indexed_address`: Calculates an address using multiple offsets and returns the value stored at that location plus the address itself.
- `store_indexed_values`: Calculates an offset based on a register and stores an integer and a word into a global table and another memory location.
- `calculate_division_parameters`: Performs calculations involving a value and its shifts, stores the results in several global variables, and conditionally calls another routine.
- `execute_indirect_dispatch`: Calls a routine via an indirect table and then updates several global variables, including a division operation if a specific global value is non-zero.
- `update_command_parameters`: Sets several global variables using a value from a table and, if certain conditions are met, updates a pointer and a flag.
- `call_call_routine_four_times`: Call stub: calls call_routine_four_times as a subroutine and returns to its own caller when it returns.
- `resolve_indirect_call_target`: Iterates through a null-terminated byte sequence to construct a word-sized target address.
- `jump_to_retry_until_nonnegative`: Jump stub: passes control straight on to retry_until_nonnegative.
- `call_loop_until_nonnegative`: Calls another routine that executes repeatedly until a value is no longer negative.
- `update_sprite_coordinates`: Updates a table of coordinate values by adding offsets or applying transformations based on input data.
- `update_sprite_positions_and_blit`: Updates a list of coordinate structures by applying offsets and scaling, then triggers a windowed blit operation.
- `reset_global_variable`: Checks if a certain global variable and another global variable meet certain conditions, then clears that global variable.
- `clear_global_variable`: Performs several calls to other routines and then sets a global variable to zero.
- `assign_pointer_on_null`: Checks a value from a table and, if it is zero, stores that pointer into a global variable.
- `store_indirect_handler_result`: Calls another routine and stores its return value into a global variable.
- `call_tiny_move_routine`: Calls another routine and then returns, passing no data of its own.
- `tiny_move`: Small routine of 1 instruction that copies d7 to d6.
- `dispatch_indirect_table_handler`: Reads a byte through a pointer held in an address register, uses it as an index into a table addressed relative to another register, and jumps to the entry it selects.
- `execute_indirect_call`: Swaps two global variables and then executes a routine through an indirect table lookup.
- `tiny_move_ext_rts`: Small routine of 3 instructions that copies (a3)+ to d7, then sign-extends d7, then returns.
- `return_empty_routine`: This routine is empty and performs no operations, returning immediately to its caller.
- `copy_string_data`: Copies a null-terminated string from a source address to a destination address.
- `return_without_effect_2`: This routine is empty: it performs no work and immediately returns to its caller.
- `perform_no_operation`: The routine executes a sequence of instructions that results in no state change or an immediate return, effectively acting as a null operation.
- `return_immediately_4`: Executes a return instruction at once, with no other operations or data modifications.
- `return_empty_routine_2`: The routine contains no executable instructions and performs no operations before returning.
- `return_empty_routine_3`: Performs no operations and returns to the caller.
- `return_from_subroutine`: The routine performs a single return instruction to exit the current execution flow.
- `return_immediately_5`: The routine performs no operations and returns to the caller immediately.
- `call_then_return`: Calls another routine, which copies one data register into another, and then returns to its own caller.
- `call_copy_register_return`: Calls another routine that copies one data register into another, then returns to its caller without further action.
- `compare_registers_not_equal`: Calls another routine that copies one data register into another, then compares two data registers for inequality and returns; the comparison result is not used afterward.
- `compare_registers_equal`: Calls another routine and then checks if two registers contain the same value.
- `compare_registers_and_branch_2`: Calls another routine and then returns if the value in one register is less than the value in another.
- `compare_registers_and_branch`: Calls another routine that copies one data register into another, then compares two word registers and returns; the comparison result is not used afterward.
- `compare_d6_d7`: Calls another routine and then returns if the original 16-bit value is not greater than another 16-bit value.
- `call_tiny_move_2`: This routine executes a single call to another routine before returning.
- `invoke_tiny_move`: This routine calls another routine that performs a small data movement operation.
- `call_tiny_move_3`: This routine executes a single subroutine call and then returns.
- `invoke_tiny_move_2`: Calls a routine that performs a small data movement operation.
- `store_linear_hash`: Computes a linear hash and stores the resulting 16-bit value into a global variable.
- `calculate_pseudo_random`: Performs a multiplication and addition on a 16-bit value from a register to produce a new value.
- `call_reset_after_check`: Stops immediately if a global variable is zero, then calls another routine, which scans a table, resets part of it or a global variable to zero when a condition matches, and returns.
- `call_handler_and_update_flags`: Calls a handler through a function pointer held in a global variable, then rebuilds a global flag byte by testing bits of a status byte and setting or clearing the corresponding flags.
- `dispatch_by_value`: Checks a register against several constant values and either performs a linked list traversal or calls another routine.
- `forward_to_another_routine_empty`: Calls another routine and then returns, with no other operations.
- `tiny_move_rts_3`: Small routine of 2 instructions that copies d7 to -(a4), then returns.
- `return_without_action_4`: The routine performs no data operations and immediately returns to its caller.
- `return_immediately_empty_4`: This routine is empty: it performs no work and returns to its caller immediately.
- `store_byte_at_offset`: Stores a byte from a register into a memory location calculated from a pointer and an offset.
- `store_word_at_offset`: Stores a 16-bit value from a register into a memory location calculated from a pointer and an offset.
- `write_byte_at_offset`: Stores a byte from a data register to a memory address formed by adding an offset read through an address register to a base value held in another register.
- `write_word_at_offset`: Writes a two-byte word to a destination calculated by adding an offset from a pointer to a base address.
- `store_byte_at_offset_2`: Stores a byte into a memory location calculated by adding an offset from a pointer to a global variable.
- `store_word_at_global_offset`: Stores a 16-bit value into a memory location calculated by adding an offset from a pointer to a global variable.
- `copy_string_to_offset`: Copies a null-terminated string from a global source to a destination calculated by adding an offset from a pointer to a global variable.
- `store_byte_indirect`: Stores a byte into a memory location determined by a pointer, an offset, and a secondary pointer lookup.
- `add_word_to_destination`: Adds a word to an existing word at a destination calculated using a base address and an offset.
- `add_byte_to_destination`: Adds a byte to an existing byte at a destination calculated using a base address and an offset.
- `increment_memory_at_offset`: Adds a 16-bit value to a 16-bit value stored at a memory location determined by a base address plus an offset, where the offset is loaded from a pointer-based memory access.
- `add_byte_to_global_offset`: Adds a byte to an existing byte at a destination calculated by adding an offset from a pointer to a global variable.
- `call_sequence_of_three_routines`: Calls three other routines in sequence, passing the result of the first into the second, then calls a third routine and returns.
- `build_path_and_initialize`: Clears several global variables, copies a string into a buffer, appends another string to that buffer, and then calls routines to lock the directory, initialize decoder tables, and perform further setup.
- `initialize_data_structures`: Stores constant values into fields of a structure and a global variable, then fills a table by repeatedly adding a fixed step to a running value and storing the results as words at computed positions until a limit is reached.
- `initialize_decoder_state_and_tables`: Clears and seeds several global variables and table pointers, computes derived buffer offsets from stored counts, and conditionally runs a decoding loop that unpacks control bytes into a buffer before returning a 64-bit result.
- `decode_packed_stream_to_buffer`: Sets a global variable to 1, optionally raises a second global value based on a bitmask, then loops fetching packed control bytes from a helper routine and expanding them into repeated or literal byte writes to a destination.
- `select_table_byte_pointer`: Compares two address-register values and, if the first is not greater than the second, calls another routine and reloads an address register from a table.
- `write_byte_with_bounds_check`: Computes a destination address from a global variable, a base register and a word offset, and writes a byte from a data register there only if an index comparison and a decremented counter check pass; otherwise it returns without writing.
- `decode_run_length_bytes_to_buffer`: Decodes a stream of packed bytes by repeatedly fetching control values through a helper routine, writing literal bytes to a destination pointer and copying back-referenced runs of previously written bytes.
- `tiny_moveq`: Small routine of 1 instruction that loads 3 into d0.
- `negate_difference_if_negative`: Subtracts one signed byte from another and returns the first byte directly if the result is non-negative.
- `return_empty_pair`: Calls another routine and returns a 64-bit value formed from two 32-bit registers, with no other data access.
- `search_sorted_table`: Performs a binary search over a table of pointers to word values, comparing each dereferenced word with a search value held in a register, and returns as soon as a match is found or the search bounds cross.
- `initialize_list_structure`: Walks a memory range in steps of six bytes, storing an increasing word offset and a zero long word at each entry, then clears one word, sets a global variable to 1 and stores 6 into another global variable.
- `insert_sorted_record_and_link`: Inserts a pointer into a sorted table of pointers by shifting larger entries up, then updates counters and a list link in a global record.
- `unlink_and_initialize_node`: Removes an element from a doubly linked list and initializes a new node structure with various default values and properties.
- `allocate_and_link_list_node`: Computes a new node position from a global base value, zeroes a block of 16-bit words after it, and fills the node's fields from the input structure, with a fallback path that removes an existing node when the free space is exhausted.
- `remove_and_reorganize_linked_entries`: Removes an entry from a memory block by shifting subsequent data and updates a linked structure by traversing nodes to adjust pointers.
- `dispatch_result_with_retry`: Tests bits of an input value and, depending on them, calls other routines in a loop until a non-negative result is obtained, storing an intermediate value in a global variable and returning the final 32-bit result.
- `return_result_register`: Calls a routine and returns the 32-bit value held in the first data register unchanged.
- `retry_until_nonnegative`: Repeatedly calls a routine and tests its result, looping and calling a second routine each time the result is negative until a nonnegative value is obtained, then returns the data register value.
- `retry_open_until_nonnegative`: Repeatedly calls another routine and stores its result in a global variable, looping until that value is non-negative, then returns a result from a data register.
- `retry_call_until_nonnegative`: Calls another routine and stores its result in a global variable, repeating the call while the stored value is negative, then returns the last result obtained from a further call.
- `loop_call_until_nonnegative`: Calls a routine and tests its result, and while the result is negative it calls another routine and reloads two address registers, then returns a 64-bit value built from two data registers.
- `update_channel_countdown_state`: Sets a global byte to one, increments it with wraparound, and then decrements and conditionally clears a global countdown, calling other routines on specific transitions.
- `update_global_and_copy`: Returns early if a global byte is negative; otherwise it decrements a countdown global, handles a zero-state case, and when a 32-bit value differs from a stored copy it updates that copy and calls other routines.
- `decrement_counter_and_clear_fields`: Decrements a word counter in a structure addressed by an address register and, while it is nonzero, adds two word fields with clamping and conditionally updates a third field before calling another routine.
- `call_four_routines_sequentially`: Calls four other routines one after another in fixed order and then returns.
- `clear_structure_fields`: Through an address register, writes zero to three byte offsets and zero to two word offsets of the pointed-to block.
- `update_clamped_channel_state`: Checks three global counters and, depending on their values, adds a global value to another, clamps it to a maximum, and decrements a counter, or decrements another counter and returns.
- `return_to_caller`: Immediately returns to the calling routine without performing any operation on data.
- `loads_address_and_processes_data`: This 17-instruction routine is reached through a handler pointer and is called by one other routine.
- `loads_address_and_moves`: This two-instruction routine is reached through a handler pointer and is called by four other routines and a data pointer.
- `moves_data_in_long_sequence`: This 254-instruction routine is reached through a handler pointer and is called by one other routine.
- `moves_data_in_sequence`: This 128-instruction routine is reached through a handler pointer and is called by one other routine and a data pointer.
- `moves_and_loads_data_sequence`: This 160-instruction routine is reached through a handler pointer and is called by three other routines.
- `dispatch_handler_pointer_check`: A four-instruction routine whose entry is referenced by absolute compare, load-effective-address, move and test instructions, and by a handler pointer stored in data.
- `dispatch_handler_pointer_check_alt`: A four-instruction routine referenced by absolute compare, load-effective-address, move and test instructions and by a handler pointer in data, called by six unnamed routines.
- `dispatch_handler_table_entry`: The routine is a 4-instruction stub whose only evidence is that its address is referenced by absolute compare, load-effective-address, move and test instructions and stored as a pointer in the data hunk, with six unnamed routines calling it.
- `compare_and_test_table_entry`: This four-instruction routine takes the address of a data table entry, compares it against a value, and tests a stored value at that location.
- `copy_data_block_absolute`: A 64-instruction routine that performs absolute-addressed move operations and is reached through a handler pointer in data.
- `copy_data_block_absolute_alt`: A 64-instruction routine that performs absolute-addressed move operations and is reached through a handler pointer in data, called by one unnamed routine.
- `move_data_absolute_long`: A 325-instruction routine that performs absolute-addressed move operations and is reached through a handler pointer in data, called by one unnamed routine.

### Storm Master.adf:Crystal

- `intro_bytekiller_depack`: The program entry routine that calls a bit-reader refill and a multi-bit read to decode a packed stream, and writes to the COLOR00 colour register.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Storm Master.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_nested_data_blocks`: Copies multiple blocks of data from a source table into a destination buffer using nested loops.
- `check_string_and_update`: Checks if a string starts with specific characters and, if so, writes two bytes to an offset within a structure pointed to by a register.
- `expand_data_table`: Iterates through a source list to copy multi-byte data blocks from a table into two destination buffers based on index values.
- `transfer_memory_block`: copies a block of data from one memory location to another using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_conditional_loop`: Repeatedly calls another routine as long as a specific register value equals seven, then updates that register with a value from a different register.
