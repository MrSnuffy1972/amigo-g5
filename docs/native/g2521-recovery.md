# Game 2521 (Escape from Colditz): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02521). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 49 raw-word lines |
| Functions named and described | pass, 171 of 171 named (0 author symbols), 171 of 171 described (9 by rule, 160 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1134 of 1134 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5057 of 6574 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 34097 |
| code | 24530 |
| data | 819507 |
| padding | 10 |
| unused | 23040 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Escape from Colditz.adf:bootblock | 1 | 165 | 6 |
| logged disk load from Escape from Colditz.adf boot-block load | 1 | 155 | 2 |
| logged disk load from Escape from Colditz.adf boot-block load | 1 | 6253 | 162 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- Escape from Colditz/Escape from Colditz.adf: custom-format (non-DOS) disk, 1696 non-empty blocks: matched against track-loaded segments
- Escape from Colditz/Escape from Colditz.adf: the boot block reads 8704 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- Escape from Colditz/Escape from Colditz.adf: the boot block reads 56320 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Escape from Colditz/Escape from Colditz.adf: 2 logged loads cover 65024 bytes of the image
- Escape from Colditz/Escape from Colditz.adf: blocks 121-491 (bytes 61952-251904) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.09)
- Escape from Colditz/Escape from Colditz.adf: blocks 494-1302 (bytes 252928-667136) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.59)
- Escape from Colditz/Escape from Colditz.adf: blocks 1304-1709 (bytes 667648-875520) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.61)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `mask_and_wait_for_equality` (bootblock): The routine masks a global variable, writes a constant to a global variable, computes a value from an input register and stores it in a global variable, then repeatedly calls another routine until the sum of two input registers equals the routine's output register, after which it masks the first global variable again, sets a bit in it, and returns a combined value from two input registers. Kind: disk hardware timing/checksum trap. Pass path: returns CONCAT44(in_D0,in_D1).
- `verify_and_decode_tile_data_2_2` (bootblock): Polls a hardware status register for readiness, then scans a data table for a matching header. Kind: disk/track verification and data decoding. Pass path: The loop that performs the XOR checksum verification and the subsequent bitwise decoding/expansion of data into the destination buffer (unaff_A2)..

## Functions


### Escape from Colditz.adf:bootblock

- `initialize_custom_hardware`: The routine configures Amiga custom chip registers including display control, color, DMA, and interrupt settings.
- `mask_and_wait_for_equality`: The routine masks a global variable, writes a constant to a global variable, computes a value from an input register and stores it in a global variable, then repeatedly calls another routine until the sum of two input registers equals the routine's output register, after which it masks the first global variable again, sets a bit in it, and returns a combined value from two input registers.
- `poll_status_and_adjust_counter`: The routine polls a status byte for specific bit flags, clearing or setting bits based on comparisons between a local variable and a global counter.
- `clear_flags_and_wait`: The routine modifies a byte pointed to by an address register by clearing and setting specific bits, then writes constant values to three offsets from that base.
- `verify_and_decode_tile_data_2_2`: Polls a hardware status register for readiness, then scans a data table for a matching header.
- `empty_routine`: The routine contains five instructions and performs no observable work; it ends with a return.

### logged disk load from Escape from Colditz.adf boot-block load

- `shift_and_merge_word`: The routine reads a word from a memory offset relative to a pointer, combines it with a register value, and stores the result through another pointer.
- `loop_shift_and_call`: The routine decrements a counter in a register and loops until it reaches zero.

### logged disk load from Escape from Colditz.adf boot-block load

- `copy_self_to_ram`: Copies a block of code from the routine's own location into a global memory buffer using a fixed iteration count.
- `copy_code_to_global_buffer`: Copies a block of instructions from the routine's own address into a global memory buffer using a fixed iteration count.
- `clear_flag_and_store_constants`: The routine clears a specific bit in a byte pointed to by a register.
- `store_offset_and_sum`: The routine writes a constant to a field in a structure pointed to by a register, adds an input value to a second field from another register and stores the result, copies a word from a register into a third field, and returns the sum of the input value with a register value.
- `check_flag_and_increment_counter`: The routine checks a bit in a byte; if clear, it writes constants to structure fields and masks a short field.
- `check_flag_and_set_coords`: The routine checks a bit in a byte pointed to by a register; if clear, it writes constants to structure fields and returns a calculated difference.
- `copy_sprite_attribute_table`: The routine sets a byte at a specific offset in a structure and then repeatedly copies a fixed set of word fields from a source table to a destination table, advancing both pointers each iteration until a counter reaches zero.
- `copy_single_sprite_attributes`: The routine sets a byte at a specific offset in a structure, calls another routine, and then copies a fixed set of word fields from a source table to a destination table without looping.
- `initialize_object_fields`: The routine reads two values from an input structure using offsets, adds a global offset to each, and writes the results into a destination structure at fixed offsets.
- `calculate_offset_from_flags`: The routine checks flags in a structure and a global variable to determine an offset.
- `store_three_fields`: The routine calls another routine and then stores three values from registers into consecutive fields of a structure pointed to by a register.
- `decode_compressed_byte_stream`: The routine processes a byte buffer, handling run-length encoding by copying bytes or repeating values based on negative byte indicators.
- `search_for_value_in_table`: The routine receives a value and a pointer to a table of 16-bit entries.
- `decode_bitstream_to_word_table`: The routine decodes a bitstream from a byte buffer into a table of 16-bit words.
- `clear_audio_channels`: The routine clears a set of audio channel volume registers and writes zero to a table of pointers, then sets a global variable to zero.
- `wait_for_disk_ready`: Polls the CIA port A register to wait for the disk drive to become ready, using a nested loop with a counter to handle timing or retries before returning.
- `mute_audio_channels`: The routine writes to the four audio volume registers and to DMACON, disabling audio DMA. It then writes a constant to a global variable six times, the last write storing a non-zero value.
- `initialize_audio_state`: The routine checks a global variable and, if non-zero, increments it and performs a series of operations including calling other routines, delaying execution, and copying data between memory locations.
- `process_audio_command`: Parses an audio command structure to configure sound parameters, update a global state variable, and enable DMA channels for the Amiga audio hardware.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `process_audio_command_2`: Interprets audio command bytes to update music playback state, including adjusting tempo, handling note offsets, and managing loop counters.
- `wait_for_disk_ready_2`: Polls the CIA port A register to wait for the disk drive to become ready, using a nested loop with a counter to handle timing or retries.
- `loop_clear_bit_and_poll`: The routine enters an infinite loop that repeatedly calls other routines, sets a global variable to a specific value, and clears the most significant bit of a byte at a fixed memory address.
- `update_entity_state_flags`: Checks a memory pointer against a threshold and sets a flag in a global variable if exceeded.
- `update_entity_physics_and_flags`: Clamps a coordinate value from an entity structure and sets a flag if it exceeds a threshold.
- `update_object_physics_and_state`: Calculates new coordinates and state flags for an object based on table lookups and conditional logic, then updates global variables and calls routines to handle collision detection and position updates.
- `compute_adjusted_difference`: The routine calls another routine to obtain a value, then conditionally subtracts one from it based on register and global variable states.
- `tiny_add_rts`: Small routine of 2 instructions that adds d3 to d1, then returns.
- `update_state_flags_and_table`: Checks register values against bounds to conditionally set bits in global variables or store values into a table, then calls another routine and performs additional flag updates based on further bounds checks.
- `set_or_clear_bit_and_update_table`: The routine sets or clears a specific bit in a byte pointed to by a pointer, using a bit index derived from a register.
- `setup_object_render_state`: Configures an object structure by calculating screen coordinates, offsets, and rendering flags based on global state and table lookups.
- `configure_sprite_animation`: The routine initializes a sprite structure by calculating screen coordinates and offsets based on global state and table lookups.
- `initialize_object_position`: The routine configures an object structure by calculating screen coordinates and offsets based on global state and table lookups.
- `compare_table_and_struct_pairs`: The routine loads two short values from a table indexed by a word in D0 and two short values from a structure pointed to by A5.
- `set_object_flags_by_table`: The routine computes an index from two input words and a global flag, looks up a table entry, and tests a byte at an offset from that entry.
- `check_position_bounds`: The routine checks if a global variable matches an input word and if two other input words fall within ranges defined by global variables.
- `check_tile_collision`: Iterates through a list of entities to check for overlap with a specific tile coordinate.
- `check_collision_and_update_state`: The routine iterates through a table of objects to detect overlaps with a target position, checking specific flag bits and coordinate bounds.
- `process_entity_state_flags`: Iterates through a list of entity data structures to evaluate complex conditional flags.
- `update_object_state_flags`: The routine modifies status bytes and bit flags within object data structures based on conditional checks.
- `update_entity_state_flags_2`: The routine modifies byte flags within two data structures based on conditional checks and global state variables.
- `clear_object_data_fields`: The routine clears two bits in a byte pointed to by a register.
- `load_win_music_and_init`: Loads the victory music file DF1:WHENWIN.MUS from disk into memory.
- `decrement_and_compare_byte`: The routine computes an address from an input pointer and a word offset, then decrements a byte at that address and checks if it became zero.
- `return_combined_registers`: The routine calls another routine and then returns the concatenation of the two input values it received in registers D0 and D1.
- `check_flag_and_configure_blitter`: The routine checks a bit in a byte pointed to by a register and, if clear, sets a bit in a global table and configures blitter registers with specific values.
- `return_immediately`: The routine contains five instructions and performs no observable work; it ends immediately without reading or writing memory or calling other routines.
- `search_table_for_value`: The routine iterates through a table of entries, comparing a global short variable with values stored in the table.
- `initialize_message_struct`: It then stores additional constants into the destination structure and returns.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `check_joystick_input`: The routine checks a global variable and, if zero, reads the CIAA_PRA register to determine joystick or mouse button states.
- `decrement_counter_and_check_flags`: The routine checks a global flag and a byte indexed by a short field in a structure.
- `decrement_specific_counter`: The routine checks a global flag and a byte in a structure.
- `process_input_state`: The routine modifies a global variable based on register values and conditional checks, calls several other routines, and loops while decrementing the global variable until specific conditions are met.
- `process_input_and_poll_state`: The routine sets global variables, calls several other routines including input processing and polling loops, and checks a global variable against specific constants.
- `set_global_and_return_concat`: The routine calls another routine, stores a constant value into a global variable, calls another routine, and returns a 64-bit value formed by concatenating the two input arguments.
- `copy_word_block`: The routine copies a block of 16-bit words from a calculated source address to a fixed destination table in memory, decrementing a counter until it reaches zero.
- `copy_word_table`: The routine copies a block of 16-bit words from a source pointer in A0 to a fixed destination table in memory, decrementing a counter in D7 until it reaches zero.
- `set_flag_and_store_word`: The routine checks a bit in a byte pointed to by a register.
- `handle_game_over_sequence`: The routine manages the game over state by configuring object rendering and sprite animations, then polling hardware registers to wait for disk drive readiness.
- `copy_word_blocks`: The routine copies four words from a source pointer to a destination pointer.
- `initialize_object_table_entries`: The routine calculates offsets and iterates to populate a table with values returned by helper routines.
- `copy_word_and_add_offset`: The routine reads a word from an offset in a structure pointed to by a register, calls another routine, then writes that word back to the same offset in another structure and adds a short value from a global variable to a short field in the second structure.
- `populate_object_table_entries_2`: The routine loops ten times, calling another routine each iteration and writing the returned values into consecutive entries of a table pointed to by a register.
- `populate_object_table_entries`: The routine loops ten times, calling another routine each iteration and writing the returned values into consecutive entries of a table pointed to by an address in a register.
- `initialize_custom_registers`: The routine initializes various custom chip registers, including COP1LCH, COP2LCH, COPJMP2, DMACON, INTENA, and others, by writing specific values to them.
- `loop_forever`: The routine enters an infinite loop without performing any operations.
- `store_byte_via_pointer`: The routine stores a byte from a register through a pointer.
- `update_object_position_and_state`: Calculates new coordinates for an object based on input flags and table lookups, then updates its state by comparing against global variables and processing joystick input.
- `compare_tile_patterns`: The routine retrieves pattern data from two tables using indices derived from input registers and a structure pointer.
- `compare_table_to_global`: The routine compares a short from a table with a short from a global variable, advancing through the table and the global variable in lockstep.
- `compare_tile_patterns_2`: Retrieves pattern data from two tables using indices derived from input registers and a structure pointer.
- `decrement_byte_and_clear_flag`: The routine reads a 16-bit index from a structure, computes an address in a byte table using that index and an input word, then decrements the byte at that address.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `process_joystick_input_and_update_objects`: Reads the CIAA_PRA register to check joystick fire buttons and disk status, then iterates through a table of object structures to update flags, counters, and fields based on input state and internal logic.
- `initialize_object_table`: The routine iterates through a table of structures, setting specific bits and copying fields from a source register.
- `decode_object_table`: The routine walks a two-dimensional table pointed to by a0 and compares each entry against a global table of 16-bit patterns.
- `decrement_global_counter`: The routine stores a word from a register into a global variable.
- `search_tile_in_table`: The routine looks up a pointer from a table using a value derived from an input register and then scans a two-dimensional table of bytes.
- `fetch_table_entry_word`: Calculates an offset using a value from a structure field and a global variable, then returns a word from a global table at that computed index.
- `clear_object_flags_and_copy_fields`: The routine clears a global variable and sets a flag in a byte pointed to by a pointer stored in the caller's structure.
- `skip_if_flag_set`: The routine checks a flag in a byte pointed to by a pointer in A1.
- `sort_index_array`: The routine initializes an array of shorts with sequential values.
- `sort_index_array_2`: The routine initializes an array of shorts with sequential values.
- `toggle_bit_and_configure_blitter`: The routine checks flags in registers and a global table, toggles a bit in a byte at an offset, and configures blitter registers if a condition is met.
- `decode_message_lengths_2`: The routine processes a block of four-byte entries using a bitmask in a register to decide which entries to handle.
- `decode_message_lengths`: The routine processes a block of four-byte entries using a bitmask in a register to decide which entries to handle.
- `configure_blitter_and_check_table`: The routine checks flags in a structure and a global variable.
- `set_bit_and_configure_blitter`: The routine sets a specific bit in a byte pointed to by a pointer, stores values in global variables, and if a condition is met, configures blitter registers with specific values.
- `search_table_and_configure_blitter`: The routine iterates through a table of pointers, checking if coordinates fall within a range, and configures blitter registers if a match is found or after the loop completes.
- `loop_call_routine`: The routine calls another routine repeatedly in a loop, decrementing a counter until it reaches zero.
- `decrement_loop_counter`: Reads a global word variable and repeatedly calls another routine while decrementing the value until it reaches -1.
- `decrement_loop_counter_2`: Reads a global word variable and calls another routine in a loop, decrementing the variable until it reaches -1.
- `decrement_loop`: The routine loads a short value from a global variable and then repeatedly calls another routine while decrementing the value until it reaches -1.
- `process_message_queue`: The routine processes a message queue stored in a global variable, dequeuing entries and updating flags in the global variable and a per-message state structure.
- `process_message_queue_2`: The routine iterates through a data structure, checking byte values and calling a helper function to process entries until a termination condition is met.
- `execute_self_modifying_loop`: The routine sets a global variable to point to another routine and executes a loop that decrements a counter.
- `process_entity_animation`: Updates entity state flags and retrieves animation frame data from a lookup table based on position and status checks.
- `update_object_animation_state`: The routine iterates through object records, checking flags and position bounds to determine if an update is needed.
- `execute_callback_table`: The routine checks a global short counter.
- `execute_callback_table_2`: The routine checks a global short counter.
- `copy_object_fields_to_state`: The routine copies several fields from a source structure pointed to by a0 into a state structure pointed to by a6, including a 32-bit value, two 32-bit values, and several 16-bit values.
- `initialize_structure_from_table`: The routine writes a constant to the first word of a table pointed to by a0, reads a word from that table and uses it to compute a short value stored in a global structure, copies several words and a long from the table into fixed offsets of a global structure, and clears a long field in that structure.
- `initialize_object_from_template`: The routine copies fields from a template structure pointed to by a0 into a working object pointed to by a6.
- `initialize_entity_sprites`: The routine initializes global state variables and clears memory blocks before configuring sprite animation structures.
- `clear_memory_block`: Clears a block of memory starting at the address in a0 one byte at a time, using a dbra loop counter, and returns.
- `fill_memory_with_bytes`: The routine iterates a counter in D7, writing the value 255 to a memory location pointed to by A0 and incrementing the pointer by one byte in each iteration until the counter reaches -1.
- `setup_structure_with_index`: The routine sets a flag byte by ORing in a bit, clears several bytes in a structure, writes a constant into one byte, and copies two values from a global table indexed by a register into fields of the structure.
- `set_memory_flags`: The routine sets a flag byte by ORing in a bit, clears several bytes in a structure, writes a constant into one byte, and copies two values from a pointer into fields of the structure.
- `copy_word_triple_step`: The routine copies 16-bit words from a source pointer in A0 to a destination pointer in A1, advancing the destination by three words each iteration while advancing the source by one word, repeating until a counter in D7 reaches -1.
- `clear_tables_and_recurse`: The routine clears a table of words, adds constants to a series of integers, and updates global variables.
- `load_byte_and_configure_blitter`: The routine loads a byte from a table using an index, compares it with a global variable, and configures blitter registers if they differ.
- `reset_global_and_call`: The routine clears a global variable to zero, calls another routine, and returns a 32-bit value from register D0.
- `configure_blitter_for_bit`: The routine compares an input byte with a value in a global array.
- `configure_blitter_for_byte`: The routine compares an input byte with a global variable.
- `configure_blitter_for_byte_alt`: The routine compares an input byte with a global variable.
- `configure_blitter_registers`: The routine compares an input byte with a global variable.
- `process_entity_list_and_update_state`: Iterates through a linked list of entity structures to check for negative values and update bit flags based on internal state.
- `copy_bytes_from_table_2`: The routine copies 64 bytes from a table pointed to by a global variable offset by the low word of the first argument shifted left by six.
- `copy_bytes_from_table`: The routine copies 64 bytes from a table pointed to by a global variable offset by the low word of the first argument shifted left by six, into a buffer pointed to by the second argument, writing one byte at a time with a stride of 40 bytes between successive writes.
- `poll_until_global_clears_2`: The routine calls another routine and then repeatedly calls another routine while checking a global variable, returning when the variable is zero.
- `poll_until_global_clears`: The routine calls another routine and then repeatedly calls another routine while checking a global variable.
- `propagate_table_value`: The routine loads a short from a global variable and uses it as a loop counter.
- `decrement_global_loop`: The routine reads a short value from a global variable, then repeatedly calls another routine while decrementing the value until it reaches -1.
- `animate_and_blend_palette`: The routine checks a counter in a data structure and, if active, copies color data from a source buffer to a destination buffer.
- `set_or_clear_bit_and_update_table_2`: The routine sets or clears a specific bit in a byte pointed to by a pointer, using a bit index derived from a register.
- `clear_memory_table_2`: The routine clears a table pointed to by a pointer by writing a byte value to a series of fixed offsets and then stepping the pointer while decrementing a counter.
- `clear_memory_table`: The routine clears a table pointed to by a pointer by writing a byte value to a series of fixed offsets and then stepping the pointer while decrementing a counter.
- `copy_word_table_2`: The routine copies sixteen 16-bit words from a table derived from a global variable to two destinations: a fixed memory area and another global table.
- `dispatch_global_function`: The routine checks a bit in a global variable and, if clear, calls a function pointer from a table indexed by that variable.
- `empty_loop_routine`: The routine contains no reachable instructions and consists of an infinite loop that never executes any operation.
- `write_address_sequence`: The routine loads several global variables and then writes a sequence of 32-bit values to a memory location.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 44 into d1, then loads 3 into d7.
- `store_word_pair_loop`: The routine writes a 32-bit value from a register to memory via a pointer, storing the low word then the high word, advances the pointer by four bytes, adds the original value to another register value, and repeats while decrementing an unsigned word counter until it wraps to zero.
- `store_word_pair_loop_2`: Stores a 32-bit value from a register to memory via a pointer, writing the low word then the high word, advances the pointer by four bytes, adds the original value to another register value, and repeats while decrementing an unsigned word counter until it wraps to zero.
- `clear_keyboard_interrupt_state`: The routine checks a condition involving a custom chip register and a global variable.
- `check_interrupt_status_and_acknowledge`: The routine reads the Amiga interrupt enable and request registers, masks the request bits, and if a specific request bit is set it calls another routine, increments a global variable, and writes an acknowledge value to a custom chip register.
- `execute_unnamed_routine`: An executed routine consisting of twenty instructions that calls another routine.
- `update_global_state_and_loop_2`: Stores several values from registers into global variables and enters a loop that repeatedly calls another routine until a specific global value is met.
- `update_global_state_and_loop`: Stores several values from registers into global variables and enters a loop that repeatedly calls another routine until a specific global variable becomes non-negative.
- `initialize_hardware_and_call_routines`: Loads constants into registers, calls two different routines, and writes three specific byte values to an offset of a pointer.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d1, then returns.
- `tiny_tst_rts`: Small routine of 2 instructions that tests d4, then returns.
- `decrement_and_jump`: Decrements a register, and based on the result, either branches to another routine or jumps to a long address.
- `execute_nested_conditional_loops`: Runs nested loops that involve calling other routines to update global variables and performing bitwise XOR operations on a global byte.
- `branch_on_global_decrement`: Loads a pointer into a register, restores registers, decrements a global byte, and performs conditional branches based on the result.
- `update_masked_byte`: Determines a value based on two global variables and stores the bitwise complement of that value into a byte at an offset from a pointer.
- `loop_and_check_bit`: Calls another routine, performs a loop that squares a register value multiple times, and checks a specific bit in the CIAA Port A register.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d4, then returns.
- `wait_for_ciaa_bit_clear`: Polls a bit in the CIAA Port A register in a loop and clears a global variable when the loop terminates.
- `update_longword_with_offset`: Tests a longword at a memory location, calls another routine, and then adds or subtracts a value from that longword based on the result of a subtraction.
- `set_and_branch_byte`: Loads a byte from memory into a register, sets its highest bit, and branches to another location.
- `write_inverted_bytes`: Performs a loop that writes the bitwise complement of two different values, derived from a global variable, into a table at an offset from a pointer.
- `store_byte_in_loop`: Executes a loop that stores two different bytes into a table at an offset from a pointer, while decrementing counters.
- `loop_and_store_byte`: Stores a byte from a register into a table and executes two nested loops using multiplication and decrementing counters.
- `wait_for_bit_clear`: Writes several values to a memory block and enters a loop that waits until a specific bit in a memory location is set.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `nop_padding_sequence_2`: This routine is empty and consists of two no-operation instructions.
- `process_masked_data_table`: Checks a global variable against a constant and, if a condition is met, performs a loop that populates a destination table by combining values from two source tables using bitwise operations and shifts.
- `combine_masked_longword_pair`: Returns a value built from the two longwords addressed by the pointer, each masked by a register value, with the first shifted left by one bit.
- `xor_sum_table`: Iterates through a table of longwords, performing a bitwise XOR on all values, and returns the result masked by a register.
- `update_hardware_registers`: Calculates values based on registers and a table to update several memory locations, including hardware-related offsets and a bitmask for a port.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
