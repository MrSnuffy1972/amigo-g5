# Game 306 (World Rugby): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00306). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (927568 top-level bytes in 26 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 123 of 123 named (0 author symbols), 123 of 123 described (14 by rule, 105 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1630 of 1630 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 438784 bytes, 17 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3176 of 3423 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 13705 |
| code | 9906 |
| data | 453570 |
| padding | 2899 |
| unused | 447488 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| World Rugby.adf:Rugby | 1 | 46 | 3 |
| type | 2 | 117 | 2 |
| AGILE | 6 | 233 | 7 |
| wait | 2 | 117 | 2 |
| World Rugby.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 676 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 848 | 24 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 127 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 79 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 44 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 545 | 29 |
| World Rugby.adf:F-AGILE.an address#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| World Rugby.adf:F-AGILE.an address#unpacked (raw code file, load address unknown) | 1 | 562 | 17 |

## Cases handled

- World Rugby/World Rugby.adf:F-AGILE.an address: BK: ByteKiller-packed, unpacked with ancient (19092 -> 32256 bytes)
- World Rugby/World Rugby.adf:F-AGILE.an address: BK: ByteKiller-packed, unpacked with ancient (163608 -> 314368 bytes)
- World Rugby/World Rugby.adf:F-AGILE.an address: BK: ByteKiller-packed, unpacked with ancient (61772 -> 99328 bytes)
- World Rugby/World Rugby.adf:F-AGILE.an address: BK: ByteKiller-packed, unpacked with ancient (51700 -> 99328 bytes)
- World Rugby/World Rugby.adf:F-AGILE.an address: BK: ByteKiller-packed, unpacked with ancient (79460 -> 124416 bytes)
- runtime segment an address: 333 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 618 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- World Rugby/World Rugby.adf:F-AGILE.an address#unpacked: 78 executed PCs whose RAM was overwritten before any snapshot match its bytes 243600-244064: recovered as a unit run at an address
- World Rugby/World Rugby.adf:F-AGILE.an address#unpacked: 66 executed PCs whose RAM was overwritten before any snapshot match its bytes 211296-211712: recovered as a unit run at an address
- World Rugby/World Rugby.adf:F-AGILE.an address#unpacked: 44 executed PCs whose RAM was overwritten before any snapshot match its bytes 243872-244064: recovered as a unit run at an address
- World Rugby/World Rugby.adf:F-AGILE.an address#unpacked: 17 executed PCs whose RAM was overwritten before any snapshot match its bytes 259440-259584: recovered as a unit run at an address
- World Rugby/World Rugby.adf:F-AGILE.an address#unpacked: 219 executed PCs whose RAM was overwritten before any snapshot match its bytes 0-3744: recovered as a unit run at an address
- World Rugby/World Rugby.adf:F-AGILE.an address#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- World Rugby/World Rugby.adf:F-AGILE.an address#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- World Rugby/World Rugby.adf:Rugby: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- World Rugby/World Rugby.adf:s/AGILE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 438784 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### World Rugby.adf:Rugby

- `initialize_hardware_registers`: Sets several hardware registers to a fixed value, writes a longword to a fixed address, and copies a pointer from a local address to a global address.
- `copy_memory_block`: Copies 651 bytes from a source address to a destination address, then writes a longword to a fixed address and calls another routine.
- `expand_rle_stream_to_memory`: Expands a run-length stream into memory from a fixed low address, where a marker byte of 39 repeats the following byte a counted number of times.

### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_register_and_branch`: Moves the contents of d1 to d0 and then branches to a fixed offset.

### AGILE

- `free_memory_blocks_loop`: Calls another routine and a function pointer, then loops four times calling another routine and a memory-free function pointer, clears the longword before a given pointer, and returns.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `execute_memory_copy_loop`: Sets a register to zero, calls an unnamed routine, and enters a loop that calls another routine and, if a certain condition is met, adds a value to a memory location relative to a pointer.
- `load_address_and_constant`: Loads an address relative to the program counter into a1 and loads a constant into d0.
- `copy_memory_block_2`: Copies a block of memory backwards from a source pointer, decrementing a counter and updating a global variable, or otherwise returns.
- `push_and_restore_registers`: Sets a byte in d0 to zero, loads a byte from a pointer, modifies d0 and d1, pushes d0, d1, and d2 onto the stack, restores d0, d3, d5, d6, a3, and a4 from the stack, and branches to a relative address.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### wait

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value stored in a field, then if a condition is met, writes a header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_register_and_branch_2`: Moves the contents of d1 to d0 and then branches to a different location.

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
- `clear_global_array`: Clears 16 words (starting from a global variable) by writing zero to each, using a loop that decrements a counter.
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
- `tiny_ori_2`: Small routine of 1 instruction that merges 1 into d0.
- `modify_memory_bytes`: Sets certain bits in a byte at the address in a0, sets other bits in a byte at the address in a0, increments a pointer in a3 while writing a word, and sets certain bits in a byte at the address in a0.
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

- `copy_seven_longwords`: Copies six longwords from the address in a1 to the address in a0, then copies a seventh longword without incrementing the pointers.
- `copy_seven_longwords_to_fixed_address`: Copies seven longwords from the address in a1 to a fixed memory location starting at offset an address.
- `update_table_from_global`: Fetches a 32-bit value from a table using a global variable as an index, writes it into three consecutive words at offset 4d2e, performs this three times in a loop, and then updates the global variable by toggling its bit and fetching a new value from the table.
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
- `tiny_ori_ori_rts`: Small routine of 3 instructions that merges 0 into d6, then merges 0 into d7, then returns.
- `initialize_hardware_and_data_2`: Sets several global variables, performs a loop three times to store a 16-bit value and its high word into a table at 8-byte increments, and calls manipulate_global_byte.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `check_status_bit`: Loops until bit 6 of a byte at offset 2 of a structure pointed to by a6 is cleared, then returns a value with bits 0 and 1 set.
- `update_display_and_input`: Updates a global variable using a bitwise mask of hardware registers, and if a certain condition is met, updates several hardware registers from a table and then increments a global variable.
- `manipulate_global_byte`: If bit 3 of a global variable is set, it performs a bitwise NOT and bitwise shifts on the variable before setting the global variable to zero.
- `tiny_ori_3`: Small routine of 1 instruction that merges 193 into d0.
- `update_display_parameters`: Updates several hardware registers based on the value of a certain global variable, including setting bit 8 and specific bit patterns in registers at offsets 8e and 90.
- `update_joystick_state`: Reads the joystick and disk status from the CIAA, performs bitwise operations to modify a global variable based on the input, and then stores the result back to that global variable.
- `parse_resource_container`: Parses a structured data container by checking for specific four-byte magic identifiers and iterating through nested blocks.
- `loop_and_clear_memory`: Runs a loop 199 times, calling another routine six times and then clearing twenty-two consecutive bytes starting at address 46fc in each iteration.
- `copy_word_table`: Copies 20 consecutive words from a source table starting at a fixed address to a destination table pointed to by a1.
- `process_nested_data`: Uses a loop to copy bytes from a source buffer to a destination buffer starting at a fixed address, with logic that adjusts the copy length based on the values read from thesource.
- `tiny_ori_4`: Small routine of 1 instruction that merges 3 into d0.
- `set_bit_zero_in_d0`: Sets bit zero of a data register to one.
- `tiny_ori_5`: Small routine of 1 instruction that merges 11 into d0.
- `process_iterative_updates`: Calls several routines, performs a loop 13 times to update a value using a table and an offset, and then enters a loop that calls another routine until it returns a non-zero value.
- `copy_and_increment_table`: Calls three different routines, sets a register to a constant, loads a word from a memory address into a register, copies that word back to the same address, and then loops or branches.
- `check_and_clear_word`: Clears a 16-bit word at a global address and returns if the value was zero, otherwise it returns.
- `clear_memory_block_2`: Clears a large block of memory by looping 2000 times and writing zero to four 32-bit words in each iteration at an address stored in a global variable.
- `delay_and_set_registers`: Performs a loop 499 times, writes two values to a RAM area, sets several global variables, performs another loop 499 times, and writes a value to a RAM area.
- `mask_audio_interrupt_enable`: Reads the certain-set and certain-clear bits from the certain-set and certain-clear registers, applies a mask to clear certain bits, writes the result to the certain-set register, sets a bit in the certain-clear register, and clears a certain-clear register before returning via an interrupt return.

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
