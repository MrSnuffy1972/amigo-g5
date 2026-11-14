# Game 1718 (venus the flytrap (1990)(gremlin)[cr qtx][t +6 inner-city]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01718). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (930928 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 137 raw-word lines |
| Functions named and described | pass, 145 of 145 named (0 author symbols), 145 of 145 described (4 by rule, 112 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2254 of 2254 (100.00%) |
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
| Venus the Flytrap (1990)(Gremlin)[cr QTX][t +6 Inner-City].adf:bootblock | 1 | 265 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3531 | 61 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1737 | 38 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 189 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 177 | 5 |
| Venus the Flytrap (1990)(Gremlin)[cr QTX][t +6 Inner-City].adf bytes 1024-3584 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- venus the flytrap (1990)(gremlin)[cr qtx][t +6 inner-city]/Venus the Flytrap (1990)(Gremlin)[cr QTX][t +6 Inner-City].adf: custom-format (non-DOS) disk, 1755 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 1344 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 600 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 182 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 128 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- venus the flytrap (1990)(gremlin)[cr qtx][t +6 inner-city]/Venus the Flytrap (1990)(Gremlin)[cr QTX][t +6 Inner-City].adf: bytes 1024-3584 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 address+address
- venus the flytrap (1990)(gremlin)[cr qtx][t +6 inner-city]/Venus the Flytrap (1990)(Gremlin)[cr QTX][t +6 Inner-City].adf: 1 logged loads cover 2560 bytes of the image
- venus the flytrap (1990)(gremlin)[cr qtx][t +6 inner-city]/Venus the Flytrap (1990)(Gremlin)[cr QTX][t +6 Inner-City].adf: blocks 8-26 (bytes 4096-13824) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.40)
- venus the flytrap (1990)(gremlin)[cr qtx][t +6 inner-city]/Venus the Flytrap (1990)(Gremlin)[cr QTX][t +6 Inner-City].adf: blocks 28-28 (bytes 14336-14848) outside the filesystem hold data, not code (opcode density 0.016, entropy 1.53)
- 1 never-loaded region(s), 2560 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 885760 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_dma_and_registers` (runtime segment an address): Updates several memory locations and hardware registers, including a DMA controller, and enters a loop that toggles a bit in a memory location while checking a condition. Kind: disk hardware/timing check. Pass path: the loop completes normally without triggering a sabotage/error state.

## Functions


### Venus the Flytrap (1990)(Gremlin)[cr QTX][t +6 Inner-City].adf:bootblock

- `boot_entry_enable_dma_interrupts`: Boot block entry that writes to the DMA control and interrupt enable registers and makes one call to a subroutine.
- `address_taken_routine_undecoded`: The decompiled body was unavailable; the record shows 15 instructions and a single call to another routine, and another routine takes its address.
- `store_handler_and_trap_zero`: Stores the address of a routine in a shared variable and then raises trap zero before returning.
- `split_position_by_twenty_two`: Splits a position into a quotient and remainder by 22, folding the remainder into the range zero to ten.
- `toggle_bit_two_and_store`: Steps a counter by an offset until it passes a limit held in a structure.
- `poll_status_until_zero`: Clears bit two of a flag byte, then repeatedly stores a word and calls a subroutine until a check returns zero.
- `forward_call_return_value`: Calls one subroutine and returns the value already on the stack.
- `sync_table_entry_with_target`: Sets bit two of a flag byte and clears it again when a field is zero.
- `step_until_values_match`: Calls a subroutine and then steps a stored value toward a second value, calling a subroutine on each comparison, until the two are equal.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `wait_port_bit_then_clear`: Calls a subroutine while bit four of a hardware status byte is set, then zeroes a word through a pointer.
- `wait_port_bit_clear`: Spins in a loop until bit four of a hardware status byte becomes clear, then writes zero to a word through a pointer.
- `count_word_to_limit`: Calls a subroutine and increments a counter word until it reaches 79.
- `increment_word_to_limit`: Increments a counter word until it reaches seventy-nine, looping while it stays below that limit.
- `clear_flag_bit_and_count`: Clears bit one of a flag byte, calls a subroutine, then increments a counter word.
- `set_flag_bit_and_decrement`: Sets bit one of a flag byte, calls a subroutine, then decrements a counter word.
- `pulse_low_flag_bit`: Clears and then sets bit zero of a flag byte, then calls a subroutine.
- `delay_long_count`: Counts a word down from 12288 to zero as a busy-wait delay before returning.
- `wait_port_bit_five_clear`: Spins while bit five of a hardware status byte stays set, then returns.
- `read_track_via_disk_dma`: Programs the disk sync word, length and pointer registers and starts a read, then waits on a flag bit before finishing the transfer.
- `write_disk_control_idle`: Writes a fixed disk control value to a register and returns.
- `scan_sync_and_decode_sector`: Scans track data for the sync word and decodes the sector that follows through subroutines.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_global_counter`: It loads a value from a global variable into a register, adds one to it, and stores the incremented result back to the same global variable.
- `xor_fold_longword_table_masked`: XORs consecutive longwords from a pointer into an accumulator, with the count taken from a register shifted right by two.
- `copy_word_to_colour_register`: Copies a word into a colour register and repeats while a flag bit is set, then clears the flag and calls two subroutines.
- `clear_bits_seven_and_three`: Sets all bits of a byte, clears bit seven, calls a subroutine, clears bit three, then calls a subroutine again.
- `toggle_bit_three_around_call`: Sets bit seven and bit three of a byte, calls a subroutine, clears bit three, calls another subroutine, then sets bit three again.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_graphics_and_hardware_state`: Updates hardware bitblit registers and graphics parameters by calculating bitplane, area, and dimension values from a data table.
- `copy_memory_block`: Checks a global variable and, if a certain condition is met, copies a block of memory from one location to another using a loop.
- `loop_until_constant_value`: Repeatedly executes another routine until a specific register reaches a constant value.
- `process_object_type_event`: Checks an object type or flag and executes a specific handler, such as updating a counter, resetting object state, or traversing a table to update or remove object properties.
- `initialize_data_structure`: Sets several fields of a memory block to specific constant values and modifies them based on a global variable.
- `update_pointer_and_check_value`: Increments a value at a pointer offset and checks a byte at a specific memory location, potentially updating other memory or a global variable.
- `update_global_state`: Checks a global variable and performs a series of conditional assignments to that same variable.
- `reset_global_data`: Checks if a global variable is neither zero nor one, then writes several fixed values to a global variable and calls another routine within a loop.
- `update_state_parameters`: Modifies various fields in a structure, including an index and a short value, based on the value of a control byte.
- `update_entity_state_parameters`: Updates various fields within an entity structure based on a control byte.
- `modify_pointer_data`: Checks if a value at a pointer falls within a specific range, and if not, writes three fixed values to different offsets of that pointer.
- `initialize_audio_sequence`: Checks a global variable and updates a structure with specific values through multiple calls to other routines.
- `initialize_audio_sequence_extended`: Updates a structure with a sequence of incremental values by calling other routines and modifying memory via pointers.
- `initialize_global_state`: Checks a global variable and a memory location, then performs multiple writes to update various offsets within a structure.
- `process_audio_loop`: Iterates through a loop based on a global variable, modifies a value at a pointer offset, and loads a byte from a table into a structure.
- `setup_blt_registers`: Configures several hardware registers used for bitblit operations based on values found in a structure.
- `calculate_pointer_bounds`: Calculates a value based on a global variable and writes a 32-bit value composed of two 16-bit parts to a memory location via a pointer.
- `update_global_from_pointer`: Checks if a global variable is non-zero and then updates that global variable using a value loaded from a pointer offset.
- `calculate_blt_parameters`: Performs complex arithmetic and bitwise shifts on values from a pointer and a global variable to calculate a result.
- `calculate_complex_value`: Performs a series of arithmetic operations and bitwise shifts based on values fetched from a pointer and a global variable to store a result in a global variable.
- `update_blt_parameters_3`: Checks various conditions in a structure and updates hardware registers or memory values based on the contents of a pointer.
- `update_blt_graphics_parameters`: Calculates and writes bitplane, area, and dimension parameters to a structure or table based on specific byte patterns and global state.
- `update_blt_sprite_parameters`: Calculates and updates Blitter hardware registers and memory addresses based on data retrieved from a table.
- `calculate_address_from_table`: Performs a series of checks on a table and calculates a new value for a global variable based on certain byte patterns.
- `update_event_list_counters`: Decrements a global counter and, if it reaches zero, iterates through a list of event records to update their internal offsets and counters.
- `call_other_routine`: Calls another routine and returns.
- `spin_forever`: Enters an endless loop that never returns.
- `loop_repeated_calls`: Repeatedly calls another routine a fixed number of times in a loop.
- `decrement_table_values`: Iterates through a table of words, decrementing certain bitfields or bytes within each word and storing the result back.
- `set_hardware_parameters`: Sets several global variables to fixed values and enters a loop that updates a pointer and a global variable with fixed offsets and values.
- `process_data_structure`: Checks a global variable and a pointer against specific values before searching a table to initialize a data structure.
- `check_bitmask`: Checks the status of specific bits in a global variable and updates that variable based on the results.
- `update_global_bitmask`: Checks specific bits of a global variable and updates a global variable with a new value based on those checks.
- `copy_structured_data_blocks`: Copies eight blocks of data from a source table to a destination, with the byte order of the source elements depending on a global variable.
- `populate_table_entries`: Checks a global variable and, through a series of calls to another routine, writes specific values to offsets of a pointer if certain conditions are met.
- `initialize_table_entry`: Searches a table for an empty entry and populates it with specific values based on a global variable.
- `update_audio_parameters`: Updates a global variable using values fetched from a table and sets hardware-related or global values to constants.
- `update_audio_parameters_2`: Configures the audio hardware by updating period, length, and volume registers for two channels and enabling a DMA channel.
- `configure_audio_hardware`: Updates multiple audio registers including period, length, and volume for two channels and enables a DMA channel.
- `decrement_and_update_dma_control`: Decrements two consecutive global bytes; if the first byte reaches zero, it writes eight to the DMA control register, and if the second byte reaches zero, it writes four to the DMA control register, otherwise it returns.
- `branch_on_global_state`: Evaluates a global variable through a series of conditional branches and potentially modifies it or calls another routine.
- `manipulate_global_buffer`: Performs complex byte-level manipulations and concatenations on a global variable based on its current value.
- `populate_data_table`: Searches a table for an empty entry and initializes it with several values and pointers derived from a provided source.
- `claim_free_sprite_slot`: Scans a fixed-size table of 16-word records, starting at a fixed base, for the first slot whose leading word is zero; if one is found it is marked in use and initialized from the caller's 11-byte parameter block (copying two position words, offsetting them, and setting a flag byte), then the routine returns.
- `initialize_table_entry_from_pointer`: Searches a table for a zeroed entry and populates that entry's fields with data or pointers derived from a provided pointer.
- `update_sprite_attribute_table`: Iterates through a list of sprite data to calculate and write X/Y coordinates and attribute data, including priority and mask bits, to the hardware sprite attribute registers.
- `update_sprite_attributes`: Calculates and writes X/Y coordinates and attribute data, including priority and mask bits, to a sprite attribute table.
- `read_joystick_fire_state`: Clears four shared state bytes, reads the joystick port and fire-button status, and records them in shared variables.
- `decode_joystick_status_bits`: Decodes bits from a memory location into direction and button flags, storing each in a shared variable.
- `update_mouse_position_counters`: Reads the mouse port movement register and subtracts the change from two stored position counters, clamping each to its range.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `update_display_table`: Calculates an offset from a global variable and writes a series of bytes to a table or memory area, then updates a pointer or global variable.
- `update_object_state_and_list`: Updates a global state variable and iterates through an object table to modify specific entries based on the current state.
- `initialize_object_list`: Iterates through a list of pointers until a null-terminated entry is found, then initializes that entry with specific values and clears a separate table.
- `find_and_reset_object_entry`: Iterates through a table of structures to find the first entry where a specific field is zero, then initializes that entry and a separate pointer-based buffer with a fixed set of values and a global variable.
- `reset_object_state`: Checks a value at a pointer, potentially clearing a global variable, and then writes several fixed values to that pointer or its offset.
- `parse_data_table`: Iterates through a structured data table using specific marker values to identify segments.
- `update_blt_parameters_2`: Updates a series of global variables and registers with a sequence of calculated or constant values.
- `update_blt_parameters`: Updates a series of global variables and registers with a sequence of calculated or constant values, including a loop that increments a pointer and a counter.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `compute_address_table`: Calculates several memory addresses based on a global variable and stores the lower and upper halves of these addresses into a series of global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_table_values_2`: Iterates through a table and subtracts a constant value from each word stored there.
- `traverse_and_update_table`: Iterates through a source table using pointer arithmetic and sentinel values to identify specific entries.
- `update_table_data`: Uses values from a pointer to perform arithmetic and table lookups, updating a global variable based on the results.
- `check_table_bounds`: Performs arithmetic on values retrieved from a table and compares them against a global variable.
- `update_entity_state_and_position`: Calculates an index into a table using values from a structure in a0 and a global variable, then updates the structure's state, position, and attributes based on specific byte values.
- `update_entity_state_and_bounds`: Iterates through a table of structures to check for specific type identifiers and update state flags or pointer offsets.
- `process_entity_state_transitions`: Iterates through an entity table to evaluate state flags and update object properties based on specific type identifiers.
- `process_input_event_data`: Parses a byte value from a data structure to determine an action, subsequently updating a global state and either calling specific subroutines or modifying table entries based on the input type.
- `initialize_memory_word`: Sets a word at a specific offset in a buffer to a constant value and clears a global variable.
- `check_bounds_and_set_flag`: Checks if a certain value or a pointer-based range falls within a specific boundary and sets a global variable to one or zero accordingly.
- `validate_and_initialize_entry`: Checks if a pointer-based range falls within specific boundaries and, if certain conditions are met, initializes a table entry with a pointer to a data table and a constant value.
- `initialize_table_entry_2`: Iterates through a table of entries until a zero sentinel is found, then initializes that entry with specific constant values and data from another source.
- `increment_short_value`: Adds sixteen to a sixteen-bit value stored at an offset from a pointer.
- `update_global_with_table_lookup`: Calculates an offset using a global variable and a table, then checks if a value from that table or another offset is less than the current global variable to potentially update a global flag.
- `validate_table_index`: Calculates an index using a global variable and a table, then checks if values retrieved from the resulting address are within specific bounds.
- `execute_recursive_counter_increment`: Executes a recursive routine that increments a global variable by a fixed amount if it is non-zero, or resets it to zero, or executes a sequence of recursive calls with conditional increments.
- `lookup_table_entry`: Performs complex arithmetic on a global variable to calculate a pointer into a table and retrieves a masked value from it.
- `conditionally_update_globals`: Compares a global word to 1 and branches to another routine if equal; otherwise, it clears a byte at a global address, calls another routine with a value in d5, and if a byte at a global address is non-zero, increments a global byte by 4, then repeats this with a different d5 value and a different increment.
- `execute_sequential_updates`: Clears a byte at a global address, calls another routine with a value in d5, and if a byte at a global address is non-zero, increments a global byte by 4; it then repeats this process with a different value in d5 and a different increment value.
- `calculate_table_index`: Clears a byte at a global address, performs arithmetic using values from several global variables and a table, and if certain conditions are met, sets bytes at two different global addresses and calls another routine via a tail branch.
- `execute_nested_loops`: Runs a nested loop structure that increments a global variable through several iterations.
- `check_conditional_table_entry`: Uses a pointer to check if a value within a specific range exists in a table, potentially triggering a recursive call.
- `write_pattern_to_memory`: Writes a repeating sequence of two specific bytes to a fixed memory location using an incrementing pointer.
- `copy_memory_blocks_2`: Copies multiple blocks of data from one memory region to another using nested loops.
- `update_state_and_call`: Updates a global variable through a sequence of increments and calls another routine.
- `copy_memory_blocks`: Copies certain words from a source pointer to a destination pointer, where the destination address is calculated from a global variable.
- `execute_repeated_routines`: Executes another routine a fixed number of times through a combination of direct calls and a loop.
- `copy_swapped_data_blocks_2`: Copies data between two buffers, toggling the order of specific byte offsets based on the value of a global variable.
- `initialize_data_table`: Fills a memory range with a constant value and then performs a second pass to write specific values into a different table offset.
- `write_alternating_values`: Iterates through a memory range, writing different pairs of 16-bit values to a memory location based on whether a character in a string is null.
- `initialize_and_swap_data_blocks`: Sets a global variable and recursively iterates through four null-terminated strings.
- `copy_swapped_data_sequences_2`: Iterates through three null-terminated strings to call another routine, then copies eight blocks of data from a source table to a destination using a byte-swapping pattern determined by a global variable.
- `copy_swapped_data_sequences`: Iterates through two null-terminated strings to call another routine, then copies eight blocks of data from a source table to a destination using a conditional byte order.
- `copy_swapped_data_blocks_4`: Copies data from a source table to a destination buffer using a loop that iterates a fixed number of times.
- `perform_swapped_data_copy`: Copies data between two locations while swapping the order of specific byte offsets based on a global variable.
- `copy_swapped_data_blocks_3`: Copies data from a source table to a destination buffer using a specific byte selection pattern that toggles based on a global variable.
- `perform_swapped_data_copy_variant`: Performs a data copy or swap between a source and destination pointer, where the source or destination or certain offsets are selected based on a global variable.
- `copy_swapped_data_blocks`: Iterates through a source buffer to copy data into a destination buffer, where the specific byte selection from the source is toggled based on the state of a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_dma_and_registers`: Updates several memory locations and hardware registers, including a DMA controller, and enters a loop that toggles a bit in a memory location while checking a condition.
- `update_status_and_wait`: Sets a bit in a byte, conditionally clears it, and then enters a loop that waits until a value in a structure matches another value in that same structure.
- `poll_and_clear_word`: Polls a bit in a memory location within a loop and clears a word in a destination buffer once the bit is no longer set.
- `clear_bit_and_increment`: Clears the second bit of a byte through a pointer and then increments a sixteen-bit value through another pointer.
- `set_bit_and_decrement`: Sets the second bit of a byte through a pointer and then decrements a sixteen-bit value through another pointer.
- `set_bit_low`: Clears the lowest bit and then sets the lowest bit of a byte through a pointer.
- `decrement_register_to_zero`: Decrements a register or variable from a fixed value down to zero using a loop.
- `wait_for_bit_clear`: Polls a specific bit in a memory location until it is cleared.
- `wait_for_hardware_bit`: Writes values to hardware registers, waits for a bit in a global variable to be set, and then waits for a bit in a memory location to be cleared.
- `process_data_stream`: Searches for a specific word in a memory range and performs bitwise transformations and data copying based on values in a table.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `modify_byte_bits`: Clears the highest bit and then clears the third bit of a byte through a pointer.
- `toggle_byte_bits`: Sets the highest and third bits, clears the third bit, and then sets the third bit again of a byte through a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_state_block_and_scan`: Writes default values to a block of ten fields, sets a flag bit, and runs a scan over a table that keeps the largest value found.
- `update_channel_tick_registers`: On each tick while a flag is set, writes combined control values and decrements a countdown.
- `initialize_audio_channels_and_scan`: Writes default values to a state block, scans a table for its maximum, then programs audio channel pointers and volumes, DMA enables and interrupt registers.
- `process_event_record_list`: Copies one record from a source address into a buffer and returns early when its marker word is the end code.
- `mask_status_field_seven_bits`: Masks a shared status byte with a fixed field mask, shifts the result right by seven and returns the argument unchanged.
