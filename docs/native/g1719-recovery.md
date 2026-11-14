# Game 1719 (venus the flytrap (1990)(gremlin)[cr qtx][t +6 inner-city][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01719). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (930928 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 137 raw-word lines |
| Functions named and described | pass, 145 of 145 named (0 author symbols), 145 of 145 described (4 by rule, 113 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2255 of 2255 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2560 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5899 of 5899 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1588 |
| code | 26636 |
| data | 901166 |
| padding | 2 |
| unused | 1536 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Venus the Flytrap (1990)(Gremlin)[cr QTX][t +6 Inner-City][a].adf:bootblock | 1 | 265 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3531 | 61 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1737 | 38 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 189 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 177 | 5 |
| Venus the Flytrap (1990)(Gremlin)[cr QTX][t +6 Inner-City][a].adf bytes 1024-3584 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- venus the flytrap (1990)(gremlin)[cr qtx][t +6 inner-city][a]/Venus the Flytrap (1990)(Gremlin)[cr QTX][t +6 Inner-City][a].adf: custom-format (non-DOS) disk, 1755 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 1344 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 600 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 182 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 129 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- venus the flytrap (1990)(gremlin)[cr qtx][t +6 inner-city][a]/Venus the Flytrap (1990)(Gremlin)[cr QTX][t +6 Inner-City][a].adf: bytes 1024-3584 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 address+address
- venus the flytrap (1990)(gremlin)[cr qtx][t +6 inner-city][a]/Venus the Flytrap (1990)(Gremlin)[cr QTX][t +6 Inner-City][a].adf: 1 logged loads cover 2560 bytes of the image
- venus the flytrap (1990)(gremlin)[cr qtx][t +6 inner-city][a]/Venus the Flytrap (1990)(Gremlin)[cr QTX][t +6 Inner-City][a].adf: blocks 8-26 (bytes 4096-13824) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.40)
- venus the flytrap (1990)(gremlin)[cr qtx][t +6 inner-city][a]/Venus the Flytrap (1990)(Gremlin)[cr QTX][t +6 Inner-City][a].adf: blocks 28-28 (bytes 14336-14848) outside the filesystem hold data, not code (opcode density 0.016, entropy 1.53)
- 1 never-loaded region(s), 2560 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 885760 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `split_position_by_twenty_two` (bootblock): Splits a position into a quotient and remainder by 22, folding the remainder into the range zero to ten. Kind: disk/track check. Pass path: the loop continues and eventually reaches the end of the loop (sRam010003ca < 8) or the code_r0x01000170 exit point depending on the specific hardware response, but the logic specifically checks for a non-zero return from u0__sub_0272 (the .
- `update_dma_and_registers` (runtime segment an address): Updates several memory locations and hardware registers, including a DMA controller, and enters a loop that toggles a bit in a memory location while checking a condition. Kind: track/sector-dependent timing or checksum check. Pass path: The loop continues or terminates based on the calculated `iRam00015bf6` (derived from `in_D1 + in_A0`), where the loop exit condition `iRam00015bf6 <= in_A0` and the conditional bit-toggling of `bRam00bfd100` (masking/setting bit 2) suggest.

## Functions


### Venus the Flytrap (1990)(Gremlin)[cr QTX][t +6 Inner-City][a].adf:bootblock

- `boot_entry_configure_dma_interrupts`: Boot block entry that writes to the DMA control and interrupt enable registers and makes one call to a subroutine.
- `invoke_subroutine_decompile_failed`: The decompiled body was unavailable; the record shows 15 instructions and a single call to another routine, and a caller takes its address.
- `store_handler_and_trap_zero`: Stores the address of a routine in a shared variable and then raises trap zero before returning.
- `split_position_by_twenty_two`: Splits a position into a quotient and remainder by 22, folding the remainder into the range zero to ten.
- `toggle_bit_two_and_store`: Steps a counter by an offset until it passes a limit held in a structure.
- `poll_status_until_zero`: Clears bit two of a flag byte, then repeatedly stores a word and calls a subroutine until a check returns zero.
- `forward_call_return_value`: Calls one subroutine and returns the value already on the stack.
- `sync_table_entry_with_target`: Sets bit two of a flag byte and clears it again when a field is zero.
- `step_until_values_match`: Calls a subroutine and then steps a stored value toward a second value, calling a subroutine on each comparison, until the two are equal.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `wait_port_bit_then_clear`: Calls a subroutine while bit four of a hardware status byte is set, then zeroes a word through a pointer.
- `wait_and_clear_bit`: Polls a hardware status byte until a specific bit is cleared, then writes zero to a memory location through a pointer.
- `count_word_to_limit`: Calls a subroutine and increments a counter word until it reaches 79.
- `increment_until_limit`: Increments a word at a memory location repeatedly until it reaches a specific value.
- `clear_flag_bit_and_count`: Clears bit one of a flag byte, calls a subroutine, then increments a counter word.
- `set_flag_bit_and_decrement`: Sets bit one of a flag byte, calls a subroutine, then decrements a counter word.
- `pulse_low_flag_bit`: Clears and then sets bit zero of a flag byte, then calls a subroutine.
- `delay_long_count`: Counts a word down from 12288 to zero as a busy-wait delay before returning.
- `wait_port_bit_five_clear`: Spins while bit five of a hardware status byte stays set, then returns.
- `read_track_via_disk_dma`: Programs the disk sync word, length and pointer registers and starts a read, then waits on a flag bit before finishing the transfer.
- `write_disk_control_idle`: Writes a fixed disk control value to a register and returns.
- `scan_sync_and_decode_sector`: Scans track data for the sync word and decodes the sector that follows through subroutines.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `xor_fold_longword_table_masked`: XORs consecutive longwords from a pointer into an accumulator, with the count taken from a register shifted right by two.
- `copy_word_to_colour_register`: Copies a word into a colour register and repeats while a flag bit is set, then clears the flag and calls two subroutines.
- `clear_bits_seven_and_three`: Sets all bits of a byte, clears bit seven, calls a subroutine, clears bit three, then calls a subroutine again.
- `toggle_bit_three_around_call`: Sets bit seven and bit three of a byte, calls a subroutine, clears bit three, calls another subroutine, then sets bit three again.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_blt_parameters_and_table_2`: Updates a table or a global variable with a calculated value based on a pointer and a global variable, while also modifying certain hardware registers.
- `copy_memory_block`: Checks a global variable and, if a certain condition is met, copies a block of memory from one location to another using a loop.
- `loop_until_constant_value`: Repeatedly executes another routine until a register or certain memory location reaches a specific constant value.
- `process_object_type_event`: Checks an object type or flag and executes a specific handler, such as updating a counter, resetting object state, or traversing a table to update or remove object properties.
- `initialize_memory_structure`: Writes a series of constant values and offsets to a memory structure pointed to by a register.
- `update_pointer_and_check_value`: Increments a value at a pointer offset and checks a byte at a specific memory location, potentially updating other memory or a global variable.
- `toggle_global_byte`: Checks if a global variable equals one and, if so, resets it to zero before setting it to one.
- `reset_object_state_conditional`: Checks if a global variable is neither zero nor one, and if so, resets that variable to a sequence of fixed values.
- `update_state_based_on_type`: Modifies a pointer or a value in a structure based on a type-check or a specific byte value.
- `update_object_properties`: Updates or resets object-specific attributes, such as position or state, based on a type-check or a specific byte value.
- `set_buffer_elements`: Checks if a value at a pointer is within a certain range, and if so, writes zero or a fixed value to specific offsets in that pointer's buffer.
- `initialize_state_sequence`: Checks a global variable and, if certain conditions are met, executes a sequence of calls that reset a global variable and write certain values to a memory location via a pointer.
- `initialize_state_and_set_table`: Checks if a global variable is zero, then repeatedly calls a routine that initializes memory, setting a byte in a table or array to a sequential value (zero through three) while updating a global variable with a specific constant before each call.
- `initialize_global_state`: Checks a global variable and, if certain conditions or zero-values are met, initializes several memory locations and a global variable with new values.
- `update_global_and_memory`: Updates a global variable in a loop and, if certain conditions are met, modifies a value at a pointer and copies a byte from a table into a memory location.
- `configure_blt_registers`: Sets several hardware registers, including certain bitplane and size registers, to specific constant values.
- `calculate_pointer_bounds`: Calculates a value based on a global variable and writes a 32-bit value composed of two 16-bit parts to a memory location via a pointer.
- `update_global_from_pointer`: Checks if a global variable is non-zero and then updates that global variable using a value loaded from a pointer offset.
- `update_global_offset`: Calculates a new value based on a pointer and a global variable, then stores it in a global variable using conditional logic.
- `calculate_global_offset`: Performs a series of conditional checks on a pointer and a global variable to calculate and store a new value in a global variable.
- `process_pointer_and_bitmask`: Checks a pointer and a global variable, then potentially clears the high bit of a value at a pointer and updates a global variable with a value from a pointer.
- `update_blt_parameters_and_table`: Updates a table or global variable with a calculated value based on a pointer and a global variable, while also modifying certain hardware registers or memory locations using conditional logic or loop-based pointer increments.
- `update_blt_parameters_from_table`: Updates a global variable or a table entry using a calculated offset or value derived from a pointer-based table lookup.
- `calculate_address_from_table`: Performs a series of checks on a table and calculates a new value for a global variable based on certain byte patterns.
- `update_event_list_and_bounds`: Decrements a counter and, if zero, resets it to a new value and processes a list of event records.
- `call_subroutine`: executes another routine and returns.
- `spin_forever`: Enters an endless loop that never returns.
- `loop_spin_forever`: Repeatedly calls a routine a fixed number of times, where each iteration calls another routine multiple times.
- `decrement_table_values`: Iterates through a table of words, decrementing certain bitfields or bytes within each word and storing the result back.
- `set_hardware_parameters`: Sets a global variable to a fixed value and then enters a loop that updates a pointer and a global variable with a fixed offset.
- `search_and_initialize_table`: Checks a pointer against several values, then searches a table to initialize a structure if a certain condition is met.
- `update_bitmask_state`: Tests individual bits of a global variable and overwrites that same variable with the boolean result of each bit check.
- `update_global_bitmask`: Checks specific bits of a global variable and updates a global variable with a new value based on those checks.
- `copy_conditional_memory_block`: Checks a global variable and copies a block of bytes from a source table to a destination, swapping the source or destination or using different offsets depending on the global variable's value.
- `assign_table_entry_id`: Recursively calls another routine to find an empty table entry, then writes a specific identifier or ID to a field within that entry's memory structure.
- `find_empty_table_entry`: Searches a table for a null entry and initializes that entry with a set of values.
- `update_audio_parameters`: Updates a global variable using a value fetched from a table and sets certain hardware-related or global values to constants.
- `call_audio_update`: Calls another routine that updates audio-related registers and enables a DMA channel.
- `update_audio_registers`: Updates several audio-related registers and enables a DMA channel.
- `decrement_and_update_dma_control`: Decrements two consecutive global bytes; if the first byte reaches zero, it writes eight to the DMA control register, and if the second byte reaches zero, it writes four to the DMA control register.
- `branch_on_global_state`: Evaluates a global variable through a series of conditional branches and potentially modifies it or calls another routine.
- `update_global_buffer`: Modifies a global variable by adding or subtracting values and shifting bytes, potentially wrapping or resetting certain bytes.
- `initialize_table_entry`: Searches a table for an empty entry and populates its fields with specific constants, pointers, and values from a source pointer.
- `find_and_initialize_entry`: Searches a table for a zeroed entry and populates that entry's fields with data or pointers derived from a provided pointer.
- `allocate_table_entry`: Iterates through a table to find an entry where the first word is zero, then initializes that entry's fields with a pointer, a global variable, and several constant or offset-derived values.
- `update_sprite_position_and_attributes`: Updates a table of sprite or attribute data by calculating new horizontal or vertical positions and setting attribute flags.
- `update_sprite_attributes`: Calculates new horizontal or vertical sprite positions and attribute flags, then writes these values to a table of sprite or attribute data.
- `read_joystick_fire_state`: Clears several shared state bytes, reads the joystick port and fire-button status, and records them in shared variables while writing to a port control register.
- `decode_joystick_status_bits`: Decodes bits from a memory location into direction and button flags, storing each in a shared variable.
- `update_mouse_position_counters`: Reads the mouse port movement register and subtracts the change from two stored position counters, clamping each to its range.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `update_display_table`: Calculates an offset from a global variable and writes a series of bytes to a table or memory area, then updates a pointer or global variable.
- `update_object_state_and_position`: Checks a global variable to determine a branch or loop, then iterates through an object table to reset or modify object properties or positions based on certain flag values.
- `initialize_object_list`: Iterates through a list of pointers until a null-terminated entry is found, then initializes that entry with specific values and clears a separate table.
- `find_and_reset_object_entry`: Iterates through a table of structures to find the first entry where a specific field is zero, then initializes that entry and a separate pointer-based buffer with a fixed set of values and a global variable.
- `reset_object_state`: Checks a value at a pointer, potentially clearing a global variable, and then writes several fixed values to that pointer or its offset.
- `search_and_copy_table_entry`: Iterates through a table or memory range looking for specific sentinel or marker values, then copies a block of data or a pointer-based entry to a destination buffer.
- `update_blt_parameters_2`: Updates a series of global variables and registers with a sequence of calculated or constant values, including a loop that increments a pointer.
- `update_blt_parameters`: Updates a series of global variables and registers with a sequence of calculated or constant values, including a loop that increments a pointer and a counter.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `compute_address_table`: Calculates several memory addresses based on a global variable and stores the lower and upper halves of these addresses into a series of global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_table_values_2`: Iterates through a table and subtracts a constant value from each word stored there.
- `traverse_and_update_table`: Iterates through a linked list or table, checking for specific sentinel or termination values, and copies or updates data in a secondary buffer.
- `traverse_table_bounds`: Iterates through a table or linked list, performing bounds checks and pointer-based calculations to update a global variable.
- `validate_table_bounds`: Checks if a pointer or index falls within a specific range and, if so, calculates a secondary offset to verify if certain table entries point back to a valid or previously visited memory area.
- `update_entity_state_and_bounds_2_2`: Calculates an index into a table using values from a structure in a0 and a global variable, then performs conditional updates to the structure's fields and calls several routines in a loop based on bitmasking a value from the structure.
- `update_entity_state_and_bounds`: Iterates through a table of structures to check for specific type identifiers and update state flags or pointer offsets.
- `update_entity_state_and_bounds_2`: Iterates through a table of structures to check for specific type identifiers and update state flags or pointer offsets.
- `process_entity_event_and_update_state`: Checks a condition on a pointer-referenced buffer and, if certain criteria are met, resets several fields in that buffer before branching to different state-update routines based on a byte-value check.
- `initialize_memory_and_global`: Sets a byte at a specific offset in a pointer-referenced buffer to a constant and resets a global variable to zero.
- `check_bounds_and_set_flag`: Checks if a certain value or a pointer-based range falls within a specific boundary and sets a global variable to one or zero accordingly.
- `update_table_entry_with_bounds_check`: Checks certain bytes in a table and, if certain conditions or bounds checks pass, updates a pointer-referenced entry with a new pointer and constant values.
- `initialize_table_entry_2`: Iterates through a table of shorts and, if a zero-value entry is found, initializes several fields in that entry with constant values or data from another source.
- `increment_value_at_offset`: Adds a constant value to a short integer stored at an offset of two bytes from a pointer passed in a register.
- `update_global_with_table_lookup`: Calculates an offset using a global variable and a table, then checks if a value from that table or another offset is less than the current global variable to potentially update a global flag.
- `update_global_with_bounds_check`: Updates a global variable using a table lookup and checks if a certain value or offset falls within a specific range.
- `execute_recursive_counter_increment`: Executes a recursive routine that increments a global variable by a fixed amount if it is non-zero, or resets it to zero, or executes a sequence of recursive calls with conditional increments.
- `traverse_and_update_table_2`: Iterates through a table or list, performing complex pointer arithmetic and checking if a value or pointer is within a certain range.
- `conditional_dual_updates`: Checks if a global variable equals 1; if so, it calls another routine, otherwise it clears a byte in a global variable, calls another routine with a value in d5, and if a byte at a global address is non-zero, increments that global variable by 4; it then repeats this process with a different value in d5 and a different increment for the global variable.
- `execute_dual_updates`: Clears a byte in a global variable, calls another routine with a value in d5, and if a byte at a global address is non-zero, increments that global variable by 4; it then repeats this process with a different value in d5 and a different increment for the global variable.
- `calculate_table_index`: Clears a byte at a global address, performs arithmetic using values from several global variables and a table, and if certain conditions are met, sets bytes at two different global addresses and calls another routine via a tail branch.
- `initialize_global_with_loop`: Sets a global variable to zero and increments it through a nested loop structure until a certain value is reached.
- `traverse_and_update_table_recursive`: Performs a table lookup and, if certain conditions are met, recursively calls itself.
- `write_pattern_to_memory`: Writes a repeating sequence of two specific bytes to a fixed memory location using an incrementing pointer.
- `copy_memory_blocks_with_offset`: Copies certain words from a source memory block to a destination memory block while applying an offset.
- `initialize_and_increment_global`: Initializes a structure with specific values and increments a global variable or calls another routine.
- `copy_memory_blocks`: Copies certain words from a source pointer to a destination pointer, where the destination address is calculated from a global variable.
- `execute_repeated_routines`: Executes a sequence of calls to another routine a fixed number of times.
- `perform_swapped_data_copy_variant_2`: Performs a conditional data copy or swap between a source and destination pointer, where the source or destination or certain offsets are selected based on a global variable.
- `initialize_table_with_constants`: Writes a constant value to a range of memory and then writes a different constant or a value from a table to another range of memory.
- `write_alternating_values`: Iterates through a memory range, writing different pairs of 16-bit values to a memory location based on whether a character in a string is null.
- `update_data_buffer_interleaved`: Checks a global variable to determine a data-swapping or copying pattern, then iterates through a loop to move or swap bytes between a source table and a destination buffer using a fixed offset.
- `swap_interleaved_data`: Iterates through a fixed number of loops to copy or swap bytes between a source table and a destination pointer based on the value of a global variable.
- `copy_interleaved_data_loop`: Iterates through two strings and copies or swaps bytes between a destination pointer and a source table using an offset, depending on a global variable.
- `copy_interleaved_data_blocks_2`: Iterates through a string and then copies or swaps bytes between a source and destination pointer using an offset, depending on whether a global variable is zero.
- `copy_interleaved_data_blocks`: copies a fixed number of bytes from a source pointer to a destination pointer, where the source or destination or the order of byte-copying or the offset-skipping is determined by the value of a global variable.
- `copy_swapped_data_to_destination_2`: Checks a global variable and a character, then copies or swaps bytes between a source and destination pointer using a loop.
- `copy_swapped_data_to_destination`: Copies bytes from a source pointer to a destination pointer, swapping or choosing certain offsets based on a global variable.
- `perform_swapped_data_copy_variant`: Performs a data copy or swap between a source and destination pointer, where the source or destination or certain offsets are selected based on a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_dma_and_registers`: Updates several memory locations and hardware registers, including a DMA controller, and enters a loop that toggles a bit in a memory location while checking a condition.
- `modify_bits_and_loop_on_value`: Sets or clears a bit in a byte and enters a loop that compares a value in a structure against another value in that structure.
- `test_bit_and_clear_word`: Tests a specific bit in a byte located at an offset from a pointer in a register.
- `clear_bit_and_increment`: Clears the least significant bit of a byte at a pointer and then increments a short integer at another pointer.
- `set_bit_and_decrement`: Sets the second bit of a byte at a pointer and then decrements a short integer at another pointer.
- `set_bit_zero_and_call`: Clears the least significant bit of a byte in a pointer and then calls another routine.
- `decrement_register_to_zero`: Decrements a register or variable from a fixed value down to zero using a loop.
- `wait_for_bit_clear`: Enters a loop that continues as long as a specific bit in a memory location remains set.
- `wait_for_hardware_bit`: Writes certain values to a hardware register, waits for a bit in a global variable to be set, and then clears a register or writes a value.
- `process_data_table`: Searches a table for a specific word, then performs a bitwise masking and copying operation or copies bytes from a table based on a comparison.
- `increment_global_counter`: Loads a value from a global variable into a register, adds one to it, and stores the incremented result back to the same global variable.
- `clear_bits_seven_and_three_2`: Sets all bits of a byte, clears bit seven, calls a subroutine, clears bit three, and calls a subroutine again.
- `toggle_bit_three_around_call_2`: Sets bit seven and bit three of a byte, calls a subroutine, clears bit three, calls another routine, and then sets bit three again.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_state_block_and_scan`: Writes default values to a block of ten fields, sets a flag bit, and runs a scan over a table that keeps the largest value found.
- `update_channel_tick_registers`: On each tick while a flag is set, writes combined control values and decrements a countdown.
- `initialize_audio_channels_and_scan`: Writes default values to a state block, scans a table for its maximum, then programs audio channel pointers and volumes, DMA enables and interrupt registers.
- `process_event_record_list`: Copies one record from a source address into a buffer and returns early when its marker word is the end code.
- `mask_status_field_seven_bits`: Masks a shared status byte with a fixed field mask, shifts the result right by seven and returns the argument unchanged.
