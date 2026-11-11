# Game 1699 (blues brothers, the (1991)(titus)[cr qtx][h cobra]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01699). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (926064 top-level bytes in 65 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 138 of 138 named (0 author symbols), 138 of 138 described (13 by rule, 64 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1202 of 1209 (99.42%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3378 of 3385 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 655532 |
| code | 12564 |
| data | 143651 |
| padding | 16013 |
| unused | 98304 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Blues Brothers, The (1991)(Titus)[cr QTX][h Cobra].adf:blues_brothers | 1 | 197 | 38 |
| Blues Brothers, The (1991)(Titus)[cr QTX][h Cobra].adf:benny | 2 | 85 | 3 |
| Blues Brothers, The (1991)(Titus)[cr QTX][h Cobra].adf:bootblock | 1 | 180 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 580 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 39 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1330 | 49 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 333 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 630 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- blues brothers, the (1991)(titus)[cr qtx][h cobra]/Blues Brothers, The (1991)(Titus)[cr QTX][h Cobra].adf: file egout.tbl (header 9856 bytes, blocks hold 4880): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address (play-long-joy): 89 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 354 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f100): 260 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 381 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (24 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- blues brothers, the (1991)(titus)[cr qtx][h cobra]/Blues Brothers, The (1991)(Titus)[cr QTX][h Cobra].adf: blocks 6-10 (bytes 3072-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.37)
- blues brothers, the (1991)(titus)[cr qtx][h cobra]/Blues Brothers, The (1991)(Titus)[cr QTX][h Cobra].adf: blocks 1070-1070 (bytes 547840-548352) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.37)
- blues brothers, the (1991)(titus)[cr qtx][h cobra]/Blues Brothers, The (1991)(Titus)[cr QTX][h Cobra].adf: blocks 1738-1741 (bytes 889856-891904) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.66)
- blues brothers, the (1991)(titus)[cr qtx][h cobra]/Blues Brothers, The (1991)(Titus)[cr QTX][h Cobra].adf: blocks 1749-1759 (bytes 895488-901120) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.46)
- blues brothers, the (1991)(titus)[cr qtx][h cobra]/Blues Brothers, The (1991)(Titus)[cr QTX][h Cobra].adf:blues_brothers: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- blues brothers, the (1991)(titus)[cr qtx][h cobra]/Blues Brothers, The (1991)(Titus)[cr QTX][h Cobra].adf:benny: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_data_table` (runtime segment an address (play-long-joy)): Calculates an index into a table, copies a block of data, and performs a loop that compares values and updates a global variable. Kind: disk/track validation and data decryption/reconstruction. Pass path: u6__l0_2AC2.
- `copy_table_entry_block` (runtime segment an address (play-long-joy)): Calculates an index into a table and copies a fixed-size block of data from the resulting pointer to a destination address. Kind: table-driven data copy/lookup. Pass path: the branch where cmpi.l #an address, (a0) or cmpi.l #an address, (a0) succeeds, leading to the data being loaded into d0/d1.

## Functions


### Blues Brothers, The (1991)(Titus)[cr QTX][h Cobra].adf:blues_brothers

- `init_hardware_at_entry`: Program entry point whose decompiled body timed out, so only its instruction facts are available.
- `chain_two_subroutine_calls`: A short routine of nine instructions that calls two other routines.
- `wrap_one_subroutine_call`: A six-instruction routine whose only recovered behaviour is a single call to another routine.
- `relay_shared_routine_call`: A five-instruction routine that is entered from four different places and makes one call to another routine.
- `invoke_three_subroutines`: A four-instruction routine that makes calls into three other routines.
- `invoke_two_subroutines`: A three-instruction routine that makes calls into two other routines.
- `delegate_to_one_subroutine`: A three-instruction routine that makes a single call to another routine.
- `delegate_single_call_stub`: A two-instruction routine that makes a single call to another routine.
- `forward_to_shared_routine`: Three-instruction routine with one call into another routine, and it is reached from two different callers.
- `forward_to_one_routine`: Two-instruction routine whose only recovered behaviour is one call into another routine, with a single caller.
- `coordinate_three_subroutine_calls`: Twelve-instruction routine that calls three other routines and is reached from three different callers.
- `sequence_three_subroutine_calls`: Five-instruction routine that makes calls into three other routines and is reached from two callers.
- `redirect_to_one_routine`: Two-instruction routine that makes one call into another routine and is reached from two callers.
- `dispatch_three_routines_shared`: Six-instruction routine that calls three other routines and is reached from five callers.
- `stub_two_instruction_return`: Two-instruction routine with a single caller and no recovered calls, so it appears to be a trivial stub.
- `chain_single_call_stub`: Two-instruction routine that makes one call into another routine and is reached from a single caller.
- `invoke_two_routines_once`: Five-instruction routine that calls two other routines and is reached from one caller.
- `run_three_routine_calls`: Four-instruction routine that makes three calls into other routines and has a single caller.
- `invoke_pair_of_routines`: Three-instruction routine that calls two other routines and has a single caller.
- `invoke_pair_then_return`: Four-instruction routine that makes two calls into other routines and has a single caller.
- `chain_two_helper_calls`: Four-instruction routine that calls two helper routines and is reached from one caller.
- `wrap_two_routine_calls`: Six-instruction routine that calls two other routines and has a single caller.
- `fan_in_two_routine_calls`: Three-instruction routine that calls two other routines and is reached from four different callers.
- `delegate_to_three_routines`: Four-instruction routine that calls three other routines and is reached from one caller.
- `relay_shared_single_call`: Three-instruction routine that makes one call into another routine and is reached from three callers.
- `forward_from_three_callers`: Three-instruction routine with one call into another routine, reached from three callers.
- `combine_two_routine_calls`: Six-instruction routine that calls two other routines and is reached from four callers.
- `delegate_six_instruction_call`: Six-instruction routine that makes one call into another routine and has a single caller.
- `relay_single_call_stub`: Two-instruction routine with one call into another routine and a single caller.
- `wrap_single_call_helper`: Two-instruction routine that makes one call into another routine and has a single caller.
- `sequence_three_helper_calls`: Five-instruction routine that calls three other routines in sequence and has a single caller.
- `delegate_single_call_helper`: Two-instruction routine that makes one call into another routine and is entered from one caller.
- `forward_shared_helper_call`: Five-instruction routine with one call into another routine, reached from two callers.
- `run_two_helper_calls`: Seven-instruction routine that calls two other routines and has a single caller.
- `coordinate_three_helper_calls`: Twenty-three-instruction routine that calls three other routines and is reached from two callers.
- `stub_short_leaf_routine`: Five-instruction routine with a single caller and no recovered calls, so it appears to be a leaf stub.
- `forward_eight_instruction_call`: Eight-instruction routine that makes one call into another routine and is reached from two callers.
- `chain_three_instruction_call`: Three-instruction routine with one call into another routine and a single caller.

### Blues Brothers, The (1991)(Titus)[cr QTX][h Cobra].adf:benny

- `set_background_color_entry`: Entry point of the benny file, with sixty-nine instructions that write the background colour register and make two calls into other routines.
- `short_called_helper_block`: Five-instruction routine entered only by a call from one caller, with no recovered calls of its own.
- `run_eleven_instruction_helper`: Eleven-instruction routine entered from a single caller that makes no recovered calls.

### Blues Brothers, The (1991)(Titus)[cr QTX][h Cobra].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `setup_boot_libraries_and_hardware`: Routine of about a hundred instructions that calls exec functions to allocate and free memory and open libraries, including graphics.library.
- `configure_display_window_registers`: Twenty-four-instruction routine that writes the display window start and stop registers, the data fetch start and stop registers, the bitplane control and modulo registers, and the copper pointer.
- `invoke_two_setup_routines`: Eighteen-instruction routine that calls two other routines and is reached from two callers and from a direct address reference.
- `stub_minimal_return_routine`: Two-instruction routine with no recovered calls and two callers, so it appears to be a minimal return stub.
- `run_block_then_call_helper`: Seventeen-instruction routine that makes one call into another routine and is reached from two callers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_global_pointer`: Stores a specific constant value into a global variable.
- `initialize_system_hardware`: Configures DMA, interrupts, and video hardware while performing various table initializations and data interleaving operations.
- `copy_interleaved_records`: Reads a length from a global variable and copies that many four-byte records from one location to another, interleaving the source words into consecutive destination words.
- `pack_and_interleave_table`: Reads a byte length from a global table and packs its bytes into two separate sequences of 16-bit words by shifting each byte into the high half.
- `initialize_table_and_pointer`: It writes the value sixteen into two consecutive words of a global table in a loop that runs six times, then stores the address of that table into two consecutive global variables.
- `increment_global_variable`: Adds three to the value stored in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_addq_movem_unlk_rts`: Short code fragment that adds 3 to a fixed memory address, then restores saved registers from the stack, then removes the stack frame held in a6, then returns.
- `copy_value_into_sequence`: Copies a value from a pointer into a sequence of consecutive word locations a fixed number of times, selecting the destination from a global value.
- `store_word_in_indexed_table`: Writes a 16-bit value from a register into a 32-bit slot of a table, using the low half of the input shifted left by two bits as an index.
- `setup_coprocessor_constants`: Clears a global variable, then writes a 16-bit constant and a value of one hundred into two adjacent 32-bit words, and finally stores a 16-bit constant into a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_copper_list_pointer`: Nine-instruction routine that touches the copper pointer register and the copper jump strobe, which together restart the copper list.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_two_24bit_constants`: Stores two different 24-bit constants into two separate global variables, overwriting the first with the second constant.
- `init_blitter_slots_and_wait`: Initializes four short-value slots to their maximum value and sets two other global values, then waits until a specific bit in a hardware status register clears.
- `toggle_and_reload_table`: Toggles a global flag between two states and, depending on which state is active, initializes or reloads a set of four short-value slots in a table.
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
- `initialize_object_pool_entries`: Calculates spatial bounds and populates a memory block with structured data entries.
- `fill_memory_block_with_pattern`: Fills a block of memory with a repeating pattern, writing several fixed values to specific offsets of each entry.
- `call_and_return`: Calls another routine and returns the value stored in a register.
- `process_data_table`: Calculates an index into a table, copies a block of data, and performs a loop that compares values and updates a global variable.
- `return_immediately`: Returns at once without doing anything.
- `copy_table_entry_block`: Calculates an index into a table and copies a fixed-size block of data from the resulting pointer to a destination address.
- `decrypt_and_copy_data`: Performs a bitwise XOR-based decryption loop on a block of memory using a key derived from a data stream.
- `load_and_discard_register`: It loads the value at the address held in a register into another register and then returns without using it.
- `modify_bits_and_delay`: Sets specific bits in a byte through a pointer and executes a long decrementing loop.
- `modify_byte_and_loop`: It modifies specific bits of a byte through a pointer and executes a decrementing loop.
- `tiny_ori`: Small routine of 1 instruction that merges 184 into d3.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d6.
- `call_subroutines`: Calls two other routines and returns.
- `poll_joystick_input_loop`: Initializes a data structure and enters a loop that monitors a specific memory location for value changes.
- `process_input_and_wait`: Decrypts data and then enters a loop that waits for a specific value range to be met in memory.
- `perform_bit_manipulation`: Performs complex bitwise shifts and masking using registers and returns a calculated byte.
- `merge_tile_bitplanes`: Scans a memory buffer for a specific marker to locate a data table, then iterates through the table to combine bytes from two different memory sources into a new buffer.
- `reconstruct_tile_data`: Scans a memory buffer for a specific marker to locate a data table, then iterates through the table to build new values.
- `reconstruct_tile_palette_data`: Scans a memory buffer for a specific marker to locate a data table, then iterates through the table to build new values.
- `combine_memory_tables`: It searches a table for a specific marker and then populates a destination buffer by combining bytes from two different memory sources.
- `recurse_on_global_match`: Performs a self-referential call if a global variable matches a specific value and enters an infinite loop that compares a global variable against itself.
- `execute_recursive_call`: The routine performs a self-referential call and enters an infinite loop that checks a global variable against itself.
- `nop_placeholder`: This routine is empty and performs no operations.
- `loop_until_bit_clears`: Repeatedly writes values to a memory location through a pointer while checking a bit in a global variable until it is cleared, then sets that variable to zero.
- `decrement_and_recurse`: Writes values to a memory location through a pointer and decrements a global variable if it is non-zero, then calls itself recursively.
- `update_video_pointer`: It is an unreachable block of code that contains an infinite loop.
- `increment_until_limit`: Writes values to a memory location through a pointer and increments a global variable if it has not reached a specific value, then calls itself recursively.
- `wait_for_value_range`: Loops until a specific byte value within a 16-bit word at a given offset changes from one value to another, then stores a constant into a third offset.
- `nop_padding_sequence`: This routine is empty and consists of non-functional instructions used as padding within the executable segment.
- `update_audio_parameters`: Increments a global variable and, upon reaching a threshold, performs a series of recursive calls, executes delay loops, and copies values from a table into several memory locations before updating hardware audio registers.
- `update_audio_channel_parameters`: Calculates new volume and period values for an audio channel based on a lookup table and updates the hardware audio registers and DMA settings.
- `compare_channel_target_value`: Copies a 12-bit value from a channel record into a field and sets a flag when it is below a stored target.
- `update_joystick_axis_state`: Processes joystick input by updating axis position values based on direction flags and magnitude.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_pointer_and_trap`: Stores the address of another routine in a global variable and then raises a system trap with no arguments.
- `setup_copper_and_dma_channels`: Waits on a status bit, makes about a dozen calls into other routines, and writes DMA control and copper constants.
- `tiny_ori_3`: Small routine of 1 instruction that merges 5 into d1.
- `copy_sixteen_words_to_block`: Copies sixteen 16-bit words from a fixed source table into a block at a set offset from an address register, then returns.
- `stub_empty_leaf_return`: Returns immediately with no body, so it is an empty leaf routine.
- `copy_table_on_button_step`: When the mouse button bit is set, advances a step counter.
- `tiny_ori_4`: Small routine of 1 instruction that merges 2 into d0.
- `build_copper_list_entries`: Fills a copper list with thirty-three wait and move pairs, each waiting for a successive scan line and writing a value to a colour register.
- `step_colors_toward_source`: Walks a 32-entry colour source table and moves a working colour one step per red, green and blue nibble toward each source value, storing each result in a palette block.
- `fill_bitplane_pointer_copper_list`: Writes eight register and data word pairs into a fixed copper list area, with the data words holding the high and low halves of each bitplane pointer.
- `interleave_three_source_planes`: Copies three source regions of 2120 bytes each into three bitplanes spaced 8 kilobytes apart, one byte at a time, in a single loop.
- `stash_fields_and_reset_limits`: Copies three fields from a state record into globals, saves one more global, and sets three fields of another record to the maximum 16-bit value.
- `copy_block_to_second_buffer`: Copies a fixed block of 179 bytes from one table into a second buffer, one byte at a time, and returns.
- `render_text_string_glyphs`: On every eighteenth call it reads characters from a string: code 2 advances to the next line, zero ends the string, and other characters index a six-byte glyph table and copy glyph rows to the bitmap.
- `tiny_ori_5`: Small routine of 1 instruction that merges 104 into d4.
- `tiny_ori_6`: Small routine of 1 instruction that merges 2 into d4.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 117 into d0, then returns.
- `clear_sixteen_words`: Clears sixteen words starting at a block addressed through the base register at a fixed offset.
- `clear_bitmap_region_bytes`: Writes zero to about 40 kilobytes of consecutive bytes, one at a time, starting at a fixed location in bitmap memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `proxy_call_and_concatenate`: Calls another routine and concatenates the resulting values from two registers into a single return value.
- `call_and_return_registers`: Calls another routine and returns the concatenated results from two registers.
- `load_channel_data`: If a value is non-zero, it calculates a pointer based on that value to load a word from memory into a global variable and another location; otherwise, it sets a flag and a global variable.
- `calculate_memory_offsets`: If a flag is not set, it sets the flag and performs a series of calculations to store derived offsets into several global variables.
- `update_audio_channels`: If a stop flag is set, it clears a global variable; otherwise, it decrements a timer, reloads it when zero, and updates the four audio channel registers using data from several other routines.
- `spin_on_volatile_byte`: Repeats an empty loop that compares a byte with itself, so it only ends if the hardware changes that byte between reads.
- `advance_channel_sequence_pointer`: Advances one sound channel's event pointer through its data.
- `interpret_music_channel_commands`: Interprets a music command stream for one channel, where special command bytes select instrument or sample data from a table.
- `initialize_four_sound_channels`: Disables audio interrupts and initializes four channel structures from a table by setting their counters, periods, volumes, and DMA properties.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d4.
- `return_constant_value`: Returns a constant value of negative one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
