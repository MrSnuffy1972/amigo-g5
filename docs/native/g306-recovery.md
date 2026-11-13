# Game 306 (World Rugby): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00306). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (927568 top-level bytes in 28 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 17 of 17 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 198 of 198 named (0 author symbols), 198 of 198 described (13 by rule, 179 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1630 of 1630 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 960000 bytes, 30 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4705 of 5102 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 13705 |
| code | 10242 |
| data | 453228 |
| padding | 2905 |
| unused | 447488 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| World Rugby.adf:Rugby | 1 | 178 | 17 |
| type | 2 | 117 | 2 |
| AGILE | 6 | 232 | 6 |
| wait | 2 | 392 | 12 |
| World Rugby.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 476 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 843 | 22 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 127 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 84 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 44 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 538 | 24 |
| World Rugby.adf:Rugby#unpacked | 1 | 637 | 13 |
| AGILE#unpacked | 2 | 843 | 45 |
| World Rugby.adf:F-AGILE.an address#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| World Rugby.adf:F-AGILE.an address#unpacked (raw code file, load address unknown) | 1 | 562 | 17 |

## Cases handled

- World Rugby/World Rugby.adf:F-AGILE.an address: ByteKiller Data (xfd)-packed, unpacked with ancient (19092 -> 32256 bytes)
- World Rugby/World Rugby.adf:F-AGILE.an address: ByteKiller Data (xfd)-packed, unpacked with ancient (163608 -> 314368 bytes)
- World Rugby/World Rugby.adf:F-AGILE.an address: ByteKiller Data (xfd)-packed, unpacked with ancient (61772 -> 99328 bytes)
- World Rugby/World Rugby.adf:F-AGILE.an address: ByteKiller Data (xfd)-packed, unpacked with ancient (51700 -> 99328 bytes)
- World Rugby/World Rugby.adf:F-AGILE.an address: ByteKiller Data (xfd)-packed, unpacked with ancient (79460 -> 124416 bytes)
- runtime segment an address: 333 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 618 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- World Rugby/World Rugby.adf:F-AGILE.an address#unpacked: 78 executed PCs whose RAM was overwritten before any snapshot match its bytes 243600-244064: recovered as a unit run at an address
- World Rugby/World Rugby.adf:F-AGILE.an address#unpacked: 66 executed PCs whose RAM was overwritten before any snapshot match its bytes 211296-211712: recovered as a unit run at an address
- World Rugby/World Rugby.adf:F-AGILE.an address#unpacked: 44 executed PCs whose RAM was overwritten before any snapshot match its bytes 243872-244064: recovered as a unit run at an address
- World Rugby/World Rugby.adf:F-AGILE.an address#unpacked: 17 executed PCs whose RAM was overwritten before any snapshot match its bytes 259440-259584: recovered as a unit run at an address
- World Rugby/World Rugby.adf:F-AGILE.an address#unpacked: 219 executed PCs whose RAM was overwritten before any snapshot match its bytes 0-3744: recovered as a unit run at an address
- World Rugby/World Rugby.adf:Rugby: crunched executable unpacked with ancient scan (DefJam Cruncher 3.2 Pro (xfd), 7480 -> 521216 bytes), 6 of 6 windows confirmed in reference RAM, loaded at an address: recovered as u13
- World Rugby/World Rugby.adf:s/AGILE: crunched executable decrunched by XFDMaster (Imploder (xfd), 28676 -> 48408 bytes), 18 of 20 windows confirmed in reference RAM: recovered as a hunk executable
- World Rugby/World Rugby.adf:F-AGILE.an address#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- World Rugby/World Rugby.adf:F-AGILE.an address#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- World Rugby/World Rugby.adf:Rugby: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- World Rugby/World Rugby.adf:s/AGILE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 3 never-loaded region(s), 960000 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_status_bits` (Rugby#unpacked): Loads a6 and a5 with specific addresses, calls another routine, checks bit 4 of a byte at offset 3841 of a5, and if set, calls another routine, sets a byte at offset 21 of a3 to zero, and enters a loop that calls another routine based on the result of a division of d0 by two. Kind: disk hardware status/timing check. Pass path: the branch where (uVar2 & 1) != 0 (the 'if' block is skipped).

## Functions


### World Rugby.adf:Rugby

- `initialize_hardware_registers`: Sets several hardware registers to a fixed value, writes a longword to a fixed address, and copies a pointer from a local address to a global address.
- `copy_memory_block`: Copies 651 bytes from a source address to a destination address, then writes a longword to a fixed address and calls another routine.
- `expand_rle_stream_to_memory`: Expands a run-length stream into memory from a fixed low address, where a marker byte of 39 repeats the following byte a counted number of times.
- `adjust_and_branch_pointer`: Adds a constant to a pointer in a0, compares that pointer to a value in a1, and if it is greater than the value in a1, branches to another location; otherwise, it adds a constant from a relative offset to a pointer in a0 and branches to another location.
- `copy_and_decode_rle_bitstream`: Copies a block of bytes from a source to a destination using a loop, then decodes a bitstream to expand run-length encoded data into memory by testing bits and calling subroutines to determine expansion lengths and values.
- `process_bit_loop_and_offset_counter_2`: Loads eight into register d1, calls another routine, and then either branches to a new location or adds ten to register d2.
- `process_bit_loop_and_offset_counter`: Loads twelve into register d1, calls another routine, and then either branches to a new location or adds one hundred and nine to register d2 before branching.
- `execute_conditional_bit_branch`: Sets d1 to 15, calls another routine, modifies d2 and d3, shifts d0 right by one, and then either calls another routine or branches based on the carry flag.
- `shift_and_branch_bit_alt`: Shifts the longword in d0 right by 1, then branches if the carry flag is set, otherwise calls another routine, and if the carry flag is clear after that call, sets d5 to 0 and d1 to 3 before branching.
- `shift_and_branch_bit`: Shifts the longword in d0 right by 1, then branches if the carry flag is set, otherwise calls another routine, and if the carry flag is clear after that call, sets d5 to 72 and d1 to 6 before branching.
- `decode_rle_bitstream`: Decodes a bitstream by testing bits in a longword and calls expand_rle_data or expand_rle_stream_to_memory based on the bit state and subsequent values.
- `store_and_transform_word`: The routine stores a word into a memory location using a combination of a register and a high byte from another register, then returns a bitwise shifted version of the loaded word.
- `prepare_bit_shift`: Sets d3 to four and d1 to one, calls another routine, adds d2 to d3, sets d1 to nine, and branches to another routine.
- `shift_and_store_bit_loop`: Shifts a longword in d0 right by one bit and calls store_and_transform_word if the result is zero, repeating this process a specified number of times based on a value in d1.
- `setup_shift_loop`: Sets d1 to eight, calls another routine, adds fourteen to d3 which contains d2, sets d1 to nine, and branches to another routine.
- `test_bit_and_call_expansion`: Tests the highest bit of a longword in register d0 and calls expand_rle_data if the bit is set, otherwise it loads the constants 2 and 9 into registers d3 and d1 and branches to another routine.
- `expand_rle_data`: Calls another routine, moves a byte from a table at a negative offset from a2 to a decremented a2, and loops using d3 until it reaches zero, then compares a2 to a1 and branches.

### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_register_and_branch`: Moves the contents of d1 to d0 and then branches to a fixed offset.

### AGILE

- `free_memory_blocks_loop`: Calls another routine and a function pointer, then loops four times calling another routine and a memory-free function pointer, clears the longword before a given pointer, and returns.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `execute_memory_copy_loop`: Sets a register to zero, calls an unnamed routine, and enters a loop that calls another routine and, if a certain condition is met, adds a value to a memory location relative to a pointer.
- `load_address_and_constant`: Loads an address relative to the program counter into a1 and loads a constant into d0.
- `copy_memory_block_2`: Copies a block of memory backwards from a source pointer, decrementing a counter and updating a global variable, or otherwise returns.
- `push_data_to_stack`: Modifies a byte in d0 using bitwise operations, pushes that byte, a byte from d1, and a word from d2 onto the stack, then restores several registers and branches to another routine.

### wait

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value stored in a field, then if a condition is met, writes a header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_register_and_branch_2`: Moves the contents of d1 to d0 and then branches to a different location.
- `parse_numeric_string_and_update_state_2`: Parses a numeric string from a table into a structure by iterating through characters and converting them to integer values.
- `process_indexed_byte_data`: Loads a longword from a pointer in a1, uses it as an index into a structure in a0 to fetch a longword and a byte, and then performs conditional logic involving two calls to other routines and updating several values in the structure at a1.
- `shift_and_compare_longword`: Loads a longword from a pointer in a1 into d1, shifts d1 left by 2, tests a longword at an offset determined by d1 from a structure in a0, and if that longword is not zero, moves a longword from offset 216 of a1 to offset 212 of a1 before branching.
- `update_timer_sequence_interval`: Divides a value at offset 216 of a structure by two, calls another routine, and stores the result at offset 212; it then calls another routine with a constant of 60 and subtracts that result from a value previously stored at offset 216 to update offset 208.
- `parse_numeric_string_and_update_state`: Reads a value from a table into a structure, then enters a loop to validate and parse characters from a string into numeric values.
- `validate_and_call_subroutine`: Checks if two bytes retrieved from a table via offsets in a1 and a0 fall within specific ranges, and if both conditions are met, calls another routine with a specific value in d0 and a pointer in a4.
- `update_timer_and_sequence_data`: Reads values from a table using an index stored at offset 220 of a structure, performs arithmetic to calculate new values, and writes them to offsets 212, 220, 224, 228, and 236 of the structure.
- `execute_wait_sequence`: Sets values in a1, compares a longword at offset 228 of a1 to d1, and if not greater, calls another routine multiple times using various offsets from a2 and d0 values before looping.
- `add_and_branch`: Adds a longword from a structure at offset zero of a1 to d1 and branches to another routine.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.

### World Rugby.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_jump_table_and_globals`: Performs several operations including setting global variables, calling a trap, and a loop that copies 16 words from a source pointer to a destination pointer.
- `initialize_memory_pointer`: Loads the address 3000 into a0, clears the longword at that address, and then branches to a different location.
- `copy_and_initialize_structure`: Copies 60 words from a global variable to a destination at address 2000 and then writes several 16-bit or 32-bit values to various offsets within a structure pointed to by a6.
- `execute_jump_table_entry`: Clears d0, sets d1 to 1, sets d2 to 63, clears d3, calculates a pointer in a0, loads a pointer in a1, calls another routine, and then either returns or jumps to a fixed address.
- `setup_jump_table_entry`: Clears d0, sets d1 to 2, sets d2 to 630, sets d3 to 32768, loads a pointer into a0 and a1, calls another routine, and otherwise returns or branches to a distant address.
- `setup_jump_table_entry_2`: Clears d0, sets d1 to 3, sets d2 to 194, sets d3 to 32768, loads a pointer into a0 and a1, calls another routine, and otherwise returns or branches to a distant address.
- `setup_jump_table_entry_3`: Clears d0, sets d1 to 4, sets d2 to 194, sets d3 to 32768, loads a pointer into a0 and a1, calls another routine, and otherwise returns or branches to a distant address.
- `setup_jump_table_entry_4`: Clears d0, sets d1 to 5, sets d2 to 243, sets d3 to 32768, loads a pointer into a0 and a1, calls another routine, and otherwise returns or branches to a distant address.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `tiny_clr_rte`: Small routine of 2 instructions that clears (a7), then returns from the exception.
- `set_stack_pointer_and_return`: Writes 2000 to the location pointed to by a7 and then returns from a sub-routine.
- `clear_color_register`: Sets d0 to 15, clears the longword at the address in a0, and loops using dbra.
- `check_audio_and_status`: Clears register d1, calls one routine, checks if the high byte of a value from a hardware register is 60, and if so calls another routine; otherwise it checks bit 6 of a global variable and calls a third routine before returning.
- `process_audio_data`: Iterates through a buffer to find a specific byte value, then uses a loop to write zero values to a destination calculated from a global variable and a table.
- `reset_globals_to_constant`: Writes zero to five global variables and writes a constant value to a fifth global variable.
- `update_audio_parameters_2`: Updates a global variable and calls another routine multiple times, then executes a loop that decrements a register, writes several 32-bit values to a specific memory range, and executes a loop that clears a global variable until it reaches a certain value.
- `update_audio_parameters`: Calculates an offset into a table to update audio register or DMA settings, including updating or clearing certain audio-related structure members and potentially updating a global variable or a counter.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_attribute_counter`: Checks a type flag in a structure and either increments or decrements a 16-bit counter, or updates it based on a value or a table lookup, while clamping the result or checking against a limit, and stores a masked version of the result into a separate structure offset.
- `update_counter_value`: Subtracts a byte value from a 16-bit word at an offset of 18 from a pointer, clamps the result to zero if negative, and stores the result at an offset of 8 from another pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_handler_and_trap`: Stores a routine address in a global variable, executes a trap instruction, and returns with a cleared register D0.
- `initialize_hardware_registers_2`: Sets certain global variables, then calls several routines to load table entries, advance offsets, advance indices, and perform blitter operations, and finally sets a pointer to a global variable and calls another routine before returning.
- `load_two_table_entries`: Loops over two table entries, loading each into register D0 and calling two routines to process it, then returns.
- `advance_two_offsets_wrapping_sixty`: Advances an offset word in two structure entries by four on each of two passes.
- `load_table_row_advance_index`: Copies three words from tables at a shared index into global variables, then advances the index by four within a 5t-entry mask.
- `copy_256_words_into_mirror`: Copies 256 words from a block of memory into a block 256 words further along, while enabling DMA channels via a custom register, and then calls another routine.
- `start_blitter_fill_and_wait`: Points the blitter destination at a global buffer, writes the blitter control, modulo and size registers to start a small blit, and waits until the blitter is no longer busy.
- `mask_table_entries_into_buffer`: Runs 158 times, taking each source word from a table and substituting a default value when it is zero.
- `blit_masked_row_wait`: Sets the blitter modulo, first-word mask and control registers for a masked row copy, starts a blit of a fixed size, and waits until the blitter is free.
- `blit_row_and_advance_offset`: Starts a masked blit from a source pointer offset by an index, waits until the blitter is free, then advances the index by two.
- `scale_column_positions_into_buffer`: Walks 99 entries and, for each nonzero entry, computes a destination offset by scaling a value with a parameter, then stores the entry at that offset.
- `render_text_lines_into_buffer`: Renders text lines into a buffer, first computing a centering offset from the string length, then copying eight glyph bytes per character from a font table at 80-byte row spacing.
- `compute_centered_text_offset`: Counts the characters of a zero-terminated string by subtracting each one from a starting value of eighty, then stores half the remainder as a centering offset.
- `branch_to_target_2`: Performs a relative branch to another instruction.
- `call_and_return_registers`: Calls another routine and returns the concatenated result in registers.
- `set_audio_register_value`: Calls another routine and, if a register or certain value is non-zero, calculates an offset into a table to read a 16-bit value and writes it to two different global variables, otherwise it sets a global variable to a constant and returns.
- `calculate_buffer_offsets`: If a certain global variable is zero, it sets it to one, modifies a global variable, and calculates a series of offsets based on a global variable and stores them in several memory locations.
- `update_audio_channels`: Clears the four audio channel values when a stop flag is set; otherwise it counts down a step timer and, at zero, reloads it and clears the channel state.
- `spin_on_volatile_byte`: Repeats an empty loop that compares a byte with itself, so it only ends if the hardware changes that byte between reads.
- `advance_channel_sequence_pointer`: Checks if a sound channel's current event or step counter has expired, and if so, loads a new event entry from a table, updating the channel's pointer, step count, and frequency-related parameters.
- `interpret_music_channel_commands`: Iterates through a command stream to update a single audio channel's properties, such as pointer, period, and volume, or to switch instrument/sample data or check for a specific four-letter signature in a data block.
- `initialize_four_sound_channels`: Disables the four audio channel interrupts, then initialises four channel structures from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_bit_mask`: Performs a bitwise AND on a byte at an offset from a pointer in a1 with a byte in d5 and stores the result back to that location.
- `parse_and_unpack_resource_data`: Validates a resource header with specific magic markers, then iterates through a data block to find a specific segment type; if found, it performs a bitwise nibble-swapping transformation while unpacking bytes into a destination buffer, and calls another routine if a certain body marker is encountered.
- `copy_table_to_array_2`: Loops 200 times, calling a routine or copying memory blocks during each iteration, then returns.
- `copy_table_to_array`: Copies twenty consecutive 16-bit values from a global table into a destination array pointed to by a1.
- `copy_memory_blocks`: Uses nested loops to copy bytes from a source to a destination, with the number of bytes copied determined by values within the source buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_clr_rts`: Small routine of 2 instructions that clears (a0), then returns.
- `copy_seven_longwords`: Copies six longwords from the address in a1 to the address in a0, then copies a seventh longword without incrementing the pointers.
- `copy_seven_longwords_to_fixed_address`: Copies seven longwords from the address in a1 to a fixed memory location starting at offset an address.
- `update_table_from_global`: Fetches a 32-bit value from a table using a global variable as an index, writes it into three consecutive words at offset 4d2e, performs this three times in a loop, and then updates the global variable by toggling its bit and fetching a new value from the table.
- `tiny_ori_ori_rts`: Small routine of 3 instructions that merges 0 into d7, then merges 0 into d7, then returns.
- `initialize_hardware_and_table`: Sets several global variables, writes a 32-bit value into three consecutive words at offset 4d2e three times in a loop, and writes several values to a specific RAM area including hardware-related constants.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_and_loop`: Calls another routine, modifies a pointer in a register, and then enters a loop that repeats a certain number of times before returning.
- `copy_data_to_array`: Copies 20 consecutive 16-bit values from a global table into a destination array pointed to by a1.
- `decompress_data_stream`: Processes a data stream by copying bytes and handling control characters to expand or repeat data into a destination buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_audio_hardware`: Sets specific values in hardware registers at offsets 96, 9c, and 9a relative to a pointer in a0 after running a loop 499 times.
- `mask_audio_interrupts`: Reads two hardware registers, performs bitwise AND operations with specific masks, and writes the result to a hardware register at offset 9c relative to a0, then clears a hardware register at offset a8 relative to a0 and returns via an interrupt return.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_update_logic`: Performs a short branch to a location within the routine that updates a value using a table and an offset via a loop, then enters a loop that calls another routine.
- `decrement_nibbles_in_memory`: Calls a routine and, if a global variable is non-zero, loops sixteen times to decrement each of the three nibbles within each word at address 4006, then increments the global variable and wraps it to zero if it reaches 17.
- `increment_nibble_values`: Loops sixteen times to compare nibbles between two memory locations and increments the nibbles in the first location if they do not match the second, then increments a global variable and resets it to zero if it reaches seventeen.
- `clear_memory_block`: Sets eight consecutive word locations starting at offset 4006 to zero.
- `copy_long_block`: Copies eight consecutive long words from a source pointed to by a1 to a destination pointed to by a2.
- `update_table_entry_loop`: Fetches a 32-bit value from a table using a global variable as an index, then enters a loop that writes this value into a 3-word sequence at offset 6 of a pointer-based structure, increments the pointer, and repeats twice before toggling a bit in the global variable and fetching a new value from the table into said global variable.
- `tiny_or_ori_rts`: Small routine of 3 instructions that merges d0 into d0, then merges 0 into d7, then returns.
- `initialize_hardware_and_data_2`: Sets several global variables, performs a loop three times to store a 16-bit value and its high word into a table at 8-byte increments, and calls manipulate_global_byte.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `wait_for_bit_clear`: The routine loops until bit 6 of a byte at offset 2 of a structure in a6 is cleared, then returns.
- `update_display_and_input`: Updates a global variable using a bitwise mask of hardware registers, and if a certain condition is met, updates several hardware registers from a table and then increments a global variable.
- `manipulate_global_byte`: If bit 3 of a global variable is set, it performs a bitwise NOT and bitwise shifts on the variable before setting the global variable to zero.
- `update_display_parameters`: Updates several hardware registers based on the value of a certain global variable, including setting bit 8 and specific bit patterns in registers at offsets 8e and 90.
- `update_joystick_state`: Reads the joystick and disk status from the CIAA, performs bitwise operations to modify a global variable based on the input, and then stores the result back to that global variable.
- `parse_resource_container`: Parses a structured data container by checking for specific four-byte magic identifiers and iterating through nested blocks.
- `loop_and_clear_memory`: Runs a loop 199 times, calling another routine six times and then clearing twenty-two consecutive bytes starting at address 46fc in each iteration.
- `copy_word_table`: Copies 20 consecutive words from a source table starting at a fixed address to a destination table pointed to by a1.
- `process_nested_data`: Uses a loop to copy bytes from a source buffer to a destination buffer starting at a fixed address, with logic that adjusts the copy length based on the values read from thesource.
- `copy_and_increment_table`: Calls three different routines, sets a register to a constant, loads a word from a memory address into a register, copies that word back to the same address, and then loops or branches.
- `check_and_clear_word`: Clears a 16-bit word at a global address and returns if the value was zero, otherwise it returns.
- `clear_memory_block_2`: Clears a large block of memory by looping 2000 times and writing zero to four 32-bit words in each iteration at an address stored in a global variable.
- `delay_and_set_registers`: Performs a loop 499 times, writes two values to a RAM area, sets several global variables, performs another loop 499 times, and writes a value to a RAM area.
- `mask_audio_interrupt_enable`: Reads the certain-set and certain-clear bits from the certain-set and certain-clear registers, applies a mask to clear certain bits, writes the result to the certain-set register, sets a bit in the certain-clear register, and clears a certain-clear register before returning via an interrupt return.

### World Rugby.adf:Rugby#unpacked

- `decompress_bitstream_data_2`: Initializes several values in a structure, then iterates through a table to perform bitwise operations and bitstream decompression by reading bits from a control word to copy data from a source buffer to a destination.
- `write_two_bytes`: Writes the bytes 253 and 231 to the memory location pointed to by a5 and then branches to another routine.
- `write_bytes_and_loop`: The routine writes two bytes to a memory location using a pointer in a5 and then enters a loop that decrements a counter from 3328 until it reaches zero.
- `process_status_bits`: Loads a6 and a5 with specific addresses, calls another routine, checks bit 4 of a byte at offset 3841 of a5, and if set, calls another routine, sets a byte at offset 21 of a3 to zero, and enters a loop that calls another routine based on the result of a division of d0 by two.
- `toggle_status_bits`: The routine clears bit 2 of a byte in a structure in a5, sets bit 0 of a byte at offset 21 of a structure in a3, and if bit 0 of d0 is clear, it sets bit 2 of the byte in a5 and clears bit 0 of the byte in a3.
- `compare_and_loop_counts`: Compares a word in d0 with a byte at offset 21 of a3, and depending on the comparison, enters one of two loops that call other routines or executes two separate routine calls.
- `modify_byte_flags_2`: Increments a byte at offset 21 of a structure in a3, clears bit 1 of a byte at the address in a5, and then branches to another routine.
- `modify_byte_flags`: The routine decrements a byte at offset 21 of a structure in a3, then sets and clears bits 1 and 0 of a byte in a structure in a5.
- `loop_and_test_bit`: The routine enters a loop that decrements a counter from 4864, then loops until bit 5 of a byte at offset 3841 of a structure in a5 is set.
- `process_data_table`: The routine initializes several values in a structure in a6, loops through a table to find specific markers, performs bitwise operations and arithmetic on data, and writes results to a structure in a3.
- `decompress_bitstream_data`: Copies a block of data from a source buffer to a destination, then performs bitstream decompression by reading bits from a control word to determine how many bytes to copy from a lookup table into the destination buffer.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_and_load_words`: The routine loops a specified number of times, shifting d0 right by one bit each iteration and loading a new word from a pointer in a0 and shifting it right if d0 is not zero.

### AGILE#unpacked

- `execute_system_trap`: Saves registers to the stack, pushes a pointer to the stack, moves a value from the stack to offset 128, executes a trap, restores the registers, clears d0, and returns.
- `initialize_system_state`: Writes four words to a table using values derived from a longword, sets a value at offset 240 of the custom chip structure, and if a bit in a custom register is not set, writes a longword to offset 128 of the custom chip structure; it then calls several routines, including one via a jump through a pointer, and finally branches to another location.
- `process_table_entries`: Runs a loop twice to read a word from a table, multiply it by eight, and use the result to offset a pointer to a second table, then stores a word from that table into a global variable, calls another routine, and repeats the process with different values before calling a third routine.
- `increment_and_wrap_values`: The routine loops through a list of words, adding 4 to each word and wrapping the value by subtracting 60 if it exceeds 60.
- `update_indexed_data`: Copies four words from two different source tables into a destination structure at specific offsets, increments a global index, and if the index is below 255, sets a global variable to one, otherwise sets it to 65536.
- `store_constant_long`: The routine stores the constant value 65536 into a memory location pointed to by a0 and returns.
- `copy_data_with_offset`: Copies 256 words from two source locations to a destination offset 510 words ahead of each source in a loop.
- `configure_dma_and_return`: Loads the custom chip base address into a2, retrieves a pointer from a global variable and its offset 156 to write a longword to offset 128 of the custom chip, tests a word at offset 136 of the custom chip, writes 33744 to offset 150 of the custom chip, sets d0 to 0, and calls another routine before returning from an interrupt.
- `setup_blt_transfer`: Configures the Blitter with a specific size, address, and control parameters, then waits for the DMA controller to finish the operation.
- `apply_masked_data`: Loops 157 times to read a value from a table, performs a bitwise AND with a mask, and stores the result at an offset calculated from another table into a destination buffer.
- `configure_blt_parameters`: Sets Blitter control registers, address, and size, then waits for the DMA controller to clear a specific bit.
- `update_blt_address_incrementally`: Configures a Blitter transfer and then increments a global variable by 2, wrapping it to 0 if it reaches 108.
- `calculate_and_store_indices`: Performs a loop of 99 iterations where it calculates a destination index using multiplication and division of values from a table and stores a word from another table at that calculated position.
- `copy_data_blocks_2`: Loads a pointer from a relative address into a0, calls another routine, and enters a loop that moves bytes from the address in a0 to d0, branching based on the sign and zero flags.
- `copy_data_blocks`: Performs a loop that copies bytes from a source table to a destination table using offsets derived from several registers, and increments a register before repeating the process.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_and_test`: Saves a pointer to the stack, initializes a register to 50, tests a byte at the pointer location, and if that byte is not zero, decrements the register and branches back to the start of the routine.
- `divide_and_store_half`: Divides the value in d7 by 2 and stores the result in a global variable.
- `jump_to_next_instruction`: Performs a short branch to the instruction immediately following this one.
- `jump_to_reset_audio`: Performs a short branch to another routine.
- `execute_subroutine_wrapper`: Saves the current registers to the stack, calls another routine, restores the registers, and returns.
- `call_reset_audio`: Saves registers to the stack, calls another routine, restores the registers, and returns.
- `initialize_pointer_table_2`: Pushes d0 to the stack, calls another routine, restores d0, and if d0 is not zero, calculates an offset into a table using d0, loads two words from that table into d0 and d1, and then copies those words to a location pointed to by a6 before calling another routine.
- `initialize_pointer_table`: If a global variable is zero, sets it to one, sets bit 1 of the CIAA_PRA register, and then calculates and stores several pointers into a table using various arithmetic operations involving multiplication and bit shifts.
- `enable_dma_channels`: Sets a byte in a global variable to 1 and enables four DMA channels by writing to the DMA control register.
- `reset_audio_volumes`: Checks a word at offset 4 of a pointer in a0 and, if it is not zero, calls another routine and then sets four audio volume registers to zero.
- `configure_audio_and_dma`: Decrements a byte at a pointer in a0 and, if not zero, loops to call another routine four times to load values into a5, then calls another routine four times to update four longword addresses in the custom chip registers, and finally calls another routine twice to update the DMA controller and four audio channel registers with values from a0, a1, a2, and a3.
- `frag_move_cmp_beq_rts`: Short code fragment that copies the memory at an offset from a6 to d6, then compares d6 with the memory at an offset from a6, then branches if the last result was zero, then returns.
- `update_sprite_data_structure`: Processes a data structure in a0 by reading bytes from a pointer, performing arithmetic with a multiplier and a base address, and updating various fields including a pointer, several bytes, and a 16-bit counter.
- `decrement_and_branch_index`: Decrements a byte at offset 26 of the structure in a0 and, if the result is not zero, branches to another routine; otherwise it branches to a different location.
- `process_sprite_command_stream`: Parses a command stream to update sprite metadata, including updating the DMA controller with a word from offset 32 of the structure in a0 and performing table lookups to update sprite pointers and attributes in the structure.
- `update_sprite_metadata_from_table_2_2`: If the byte in d0 is 229, it updates the DMA controller with a word from offset 32 of the structure in a0, performs a table lookup using val.
- `update_sprite_metadata_from_table_2`: If the byte in d0 is 229, it updates the DMA controller with a word from offset 32 of the structure in a0, performs a table lookup using values from a1 to calculate a pointer and data fields, and stores these results into the structure in a0 at offsets 60, 68, and 64.
- `update_sprite_metadata_from_table`: If the byte in d0 is 228, it calculates an index into a table using a value from a1, retrieves a pointer and several data fields from that table, and updates the structure in a0 at offsets 60, 64, 68, and 48.
- `update_sprite_data_and_dma`: If the byte in d0 is 230, it performs a bitwise OR of a word from offset 32 of the structure in a0 into the memory pointed to by a5, then calculates a new pointer and data values from a table using bytes from a1 and stores them into offsets 60, 64, 68, and 4 of the structure in a0.
- `update_sprite_and_dma_state`: Checks if a byte in d0 equals 231, then updates a byte in the structure at offset 38 from a byte at offset 1 of a1.
- `update_indexed_byte_and_offset`: Compares a byte in d0 to 232; if they are equal, it copies a byte from an offset of 1 in a1 to an offset of 26 in a0, increments a longword at offset 48 of a0 by 2, and branches to another routine.
- `update_structure_index_and_data`: If a byte in d0 equals 227, it increments a value at offset 48 of the structure in a0 by 3, copies two bytes from a pointer in a1 to offsets 27 and 28 of the structure, and then fetches a byte from a table using a pointer at offset 18 and an offset at offset 48.
- `update_structure_from_table`: Decrements a byte at offset 23 of a structure in a0, and if it is not zero, moves a byte from offset 24 to offset 23.
- `update_value_based_on_byte`: Compares a byte in d0 to 225 and 224; if d0 equals 224, it loads a byte from offset 1 of a1 into d0, subtracts 5 from d0, stores the result in a longword at offset 14 of a0, and branches to another routine.
- `update_entity_state_and_offset`: Increments a longword at offset 14 of the structure in a0, updates a byte at offset 45, and calculates a value by indexing into a table using a combination of bytes from offsets 43, 8, and 44.
- `update_scaled_values_2`: Moves bytes from a0 into d5, d4, and d1, performs arithmetic on d4, and checks bits in d7 to conditionally modify d1 and set bit 5 of d7.
- `update_scaled_values`: Performs arithmetic on bytes within a structure, updates several fields including a bit-flipped status byte, and if a specific bit is set in the structure, calculates a new longword value by multiplying a byte by 2048 and adding it to a longword at offset 56.
- `update_scaled_value`: Calculates a new value by multiplying the negation of a byte in d1 by 2048 and adding it to a longword at offset 56 of a structure in a0, then stores the result back at that offset and returns a byte from offset 45 of the structure in d0.
- `initialize_audio_channel_structures`: Disables DMA and interrupts, then loops four times to initialize a structure by setting various offsets with zero or fixed values, calculating a bitmask from a table, and populating the structure with pointers and values derived from a data table.

### World Rugby.adf:F-AGILE.an address#unpacked (raw code file, load address unknown)

- `branch_to_target`: Branches to a relative address and returns from a sub-routine.
- `compare_and_increment_nibbles`: Calls another routine and, if a global variable is non-zero, loops sixteen times to compare nibbles between two memory locations and increments the nibbles in the first location if they do not match the target.
- `fade_nibbles_toward_target`: When its counter is nonzero, steps each of three four-bit fields in sixteen consecutive words one unit toward the matching field in a target table.
- `update_table_from_global_alt`: Fetches a 32-bit value from a table using a global variable as an index, writes it into three consecutive words at offset 6 of a pointer derived from the global variable three times in a loop, and then updates the global variable by toggling its bit and fetching a new value from the table.
- `initialize_hardware_and_data`: Sets several global variables, performs a loop three times to store a 32-bit value and its high word into a table at 8-byte increments, and sets several hardware registers including bitplane or copper-related registers and DMA or interrupt-related registers before returning.
- `update_input_and_display_state`: Reads the keyboard serial register and updates certain CIA or custom hardware registers, while simultaneously adjusting display settings and decrementing nibble-based counters or checking display state codes, then returns.
- `update_display_parameters_2`: Updates several hardware registers based on the value of a global variable, including setting bit 8 and specific bit patterns in registers at offsets 8e and 90.
- `update_joystick_state_2`: Reads the CIAA or custom register, checks specific bits related to disk status and certain joystick button or direction states, and updates a global variable using bitwise masking and bit-flipping logic.
- `parse_resource_container_2`: Checks for a specific four-byte magic identifier at the start of a buffer, then iterates through nested blocks by checking for specific four-byte magic identifiers.
- `loop_and_clear_memory_2`: Runs a loop 199 times, calling another routine six times and then clearing twenty-two consecutive bytes starting at a fixed address in each iteration.
- `copy_word_table_2`: Copies 30 consecutive words from a source table starting at a global address to a destination table pointed to by a1.
- `unpack_packbits_stream_40_bytes`: Decodes a PackBits-style run-length stream, where a negative control byte repeats the next byte a counted number of times and a non-negative control byte copies that many literal bytes.
- `update_memory_word_via_loop`: Calls three different routines, then executes a loop that reads a word from a global memory location, writes that same word back to the same location, and repeats this process or certain number of times before branching to another routine.
- `clear_and_test_word`: Clears a 16-bit word in a global variable and returns immediately if the value is zero; otherwise, it returns.
- `clear_memory_block_3`: Loads a pointer from a global variable and clears a block of memory by looping 2000 times and writing zero to four 32-bit words in each iteration, then returns.
- `delay_and_set_registers_2`: Performs a loop 499 times, writes two values to a RAM area, sets several global or hardware registers, performs another loop 499 times, and writes a value to a RAM area.
- `mask_audio_interrupt_mask`: Reads the certain-bit-mask and volume-level from the audio or custom chip registers, applies a bitwise mask to clear certain bits, writes the result to the audio or custom chip register, sets a bit in a custom chip register, and clears a custom chip register before returning via an interrupt return.
