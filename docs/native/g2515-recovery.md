# Game 2515 (escape from colditz (1990)(digital magic)(m4)[cr agl][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02515). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 49 raw-word lines |
| Functions named and described | pass, 171 of 171 named (0 author symbols), 171 of 171 described (9 by rule, 160 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1112 of 1112 (100.00%) |
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
| Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL][a].adf:bootblock | 1 | 165 | 6 |
| logged disk load from Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL][a].adf boot-block load | 1 | 155 | 2 |
| logged disk load from Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL][a].adf boot-block load | 1 | 6253 | 162 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- escape from colditz (1990)(digital magic)(m4)[cr agl][a]/Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL][a].adf: custom-format (non-DOS) disk, 1696 non-empty blocks: matched against track-loaded segments
- escape from colditz (1990)(digital magic)(m4)[cr agl][a]/Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL][a].adf: the boot block reads 8704 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- escape from colditz (1990)(digital magic)(m4)[cr agl][a]/Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL][a].adf: the boot block reads 56320 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- escape from colditz (1990)(digital magic)(m4)[cr agl][a]/Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL][a].adf: 2 logged loads cover 65024 bytes of the image
- escape from colditz (1990)(digital magic)(m4)[cr agl][a]/Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL][a].adf: blocks 121-491 (bytes 61952-251904) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.09)
- escape from colditz (1990)(digital magic)(m4)[cr agl][a]/Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL][a].adf: blocks 494-1302 (bytes 252928-667136) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.59)
- escape from colditz (1990)(digital magic)(m4)[cr agl][a]/Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL][a].adf: blocks 1304-1709 (bytes 667648-875520) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.61)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `mask_and_wait_for_equality` (bootblock): The routine masks a global variable, writes a constant to a global variable, computes a value from an input register and stores it in a global variable, then repeatedly calls another routine until the sum of two input registers equals the routine's output register, after which it masks the first global variable again, sets a bit in it, and returns a combined value from two input registers. Kind: disk hardware timing/checksum check. Pass path: returns CONCAT44(in_D0,in_D1).
- `decode_encrypted_data_block_2` (bootblock): Synchronizes with the display raster beam and scans a memory table for a specific signature. Kind: disk/track/checksum check. Pass path: the loop that performs bitwise decryption/transformation of data into the destination buffer (pbVar9) and returns.

## Functions


### Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL][a].adf:bootblock

- `initialize_custom_hardware`: The routine configures Amiga custom chip registers including display control, color, DMA, and interrupt settings.
- `mask_and_wait_for_equality`: The routine masks a global variable, writes a constant to a global variable, computes a value from an input register and stores it in a global variable, then repeatedly calls another routine until the sum of two input registers equals the routine's output register, after which it masks the first global variable again, sets a bit in it, and returns a combined value from two input registers.
- `wait_for_copper_raster`: The routine polls a hardware register to synchronize with the display raster beam.
- `clear_flags_and_wait`: The routine clears a bit in a byte pointed to by an address register, sets a different bit in the same byte, writes constant values to three offsets from that base, then spins until a bit in one offset clears and spins again until a bit in another offset clears.
- `decode_encrypted_data_block_2`: Synchronizes with the display raster beam and scans a memory table for a specific signature.
- `empty_routine`: The routine contains five instructions and performs no observable work; it ends with a return.

### logged disk load from Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL][a].adf boot-block load

- `combine_and_shift_word`: The routine reads a word from a memory offset, combines it with an unassigned register value, and stores the result through a pointer.
- `loop_shift_and_call`: The routine decrements a counter in a register and loops until it reaches zero.

### logged disk load from Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL][a].adf boot-block load

- `copy_self_and_trap`: Copies a block of code from the routine's own instruction stream into a global memory buffer and triggers a trap instruction.
- `copy_code_block_to_global`: Copies a fixed number of words from the routine's own instruction stream into a global memory buffer.
- `clear_flag_and_store_constants`: The routine clears a specific bit in a byte pointed to by a register.
- `store_offset_and_sum`: The routine writes a constant to a field in a structure pointed to by a register, adds an input value to a second field from another register and stores the result, copies a word from a register into a third field, and returns the sum of the input value with a register value.
- `check_flag_and_increment_counter`: The routine checks a flag in a structure and, if clear, writes constant bytes to specific offsets.
- `check_flag_and_write_bytes`: The routine checks a bit in a byte pointed to by a register.
- `copy_sprite_attribute_table`: The routine sets a specific byte in a structure and then iteratively copies a fixed set of word fields from a source table to a destination table, advancing both pointers each iteration until a counter reaches zero.
- `copy_single_sprite_attributes`: The routine sets a specific byte in a structure, calls another routine, and then copies a fixed set of word fields from a source table to a destination table without iteration.
- `initialize_object_fields`: The routine reads two values from an input structure using offsets, adds a global offset to each, and writes the results into a destination structure at fixed offsets.
- `calculate_offset_from_flags`: The routine checks flag bits in a structure to determine a calculation path, iterating through a table of words or checking coordinate bounds.
- `store_three_fields`: The routine calls another routine and then stores three values from registers into consecutive fields of a structure pointed to by a register.
- `decode_byte_stream_loop`: The routine iterates over a byte array, constructing a 32-bit value from the bytes.
- `search_for_value_in_table`: The routine receives a value and a pointer to a table of 16-bit entries.
- `decode_bitstream_to_word_table`: The routine decodes a bitstream from a byte buffer into a table of 16-bit words.
- `sort_and_clear_memory`: The routine iterates over a byte table, comparing bytes and decrementing counters, and on a specific condition it writes zero to a series of memory locations and updates a pointer table before clearing a global variable.
- `clear_audio_channels`: The routine clears a set of audio channel volume registers and writes zero to a table of pointers, then sets a global variable to zero.
- `mute_audio_channels`: The routine writes to the four audio volume registers and to DMACON, disabling audio DMA. It then writes a constant to a global variable six times, the last write storing a non-zero value.
- `initialize_audio_state`: The routine checks a global flag and, if set, calls audio processing routines and inserts delays.
- `process_audio_command_stream`: Parses a byte stream to configure Amiga audio DMA registers, handling commands for channel selection, volume adjustment, and data loading from lookup tables.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `process_audio_command_stream_2`: Interprets a byte from a command structure to perform actions such as loading a music file, adjusting audio parameters, or modifying state variables.
- `clear_audio_and_return`: Calls a routine to clear audio channel volumes and reset a pointer table, then returns to the caller.
- `poll_custom_and_clear_flag`: The routine repeatedly calls other subroutines and checks a bit in a data structure pointed to by a register.
- `update_entity_state_flags`: The routine checks a global counter and sets a bit in a status byte if the counter exceeds a threshold.
- `update_entity_animation_state`: The routine manages entity animation and state flags by checking a global counter threshold and updating status bits.
- `update_entity_position_and_collision`: Calculates new entity coordinates based on velocity and flags, then updates collision state and tile indices using lookup tables.
- `compute_adjusted_difference`: The routine calls another routine to obtain a value, then conditionally subtracts one from it based on register and global variable states.
- `tiny_add_rts`: Small routine of 2 instructions that adds d3 to d1, then returns.
- `update_state_flags_and_values`: Checks register values against ranges to conditionally set bits in global variables or store values to memory offsets, then calls another routine and updates additional flags and pointers based on further range checks.
- `set_or_clear_bit_and_update_table`: The routine sets or clears a specific bit in a byte pointed to by a pointer, using a bit index derived from a register.
- `configure_object_sprite_data`: The routine initializes an object structure by setting flags, calculating screen coordinates, and populating a lookup table with precomputed values.
- `initialize_object_sprite_data`: Configures an object structure by setting flags, calculating screen coordinates, and populating a lookup table with precomputed values.
- `initialize_object_position`: The routine configures an object structure by setting flags, calculating screen coordinates, and populating a lookup table with precomputed values.
- `compare_structure_to_table`: The routine loads two short values from a table indexed by a word in D0 and two short values from a structure pointed to by A5.
- `set_object_flags_by_table`: The routine computes an index from two input words and a global flag, looks up a table entry, and tests a byte at an offset from that entry.
- `check_position_bounds`: The routine checks if a global variable matches an input word and if two other input words fall within specific ranges defined by global variables.
- `check_collision_and_update_flags`: The routine iterates through a list of entities to check for character matches and coordinate overlaps within specific bounds.
- `check_collision_bounds`: Iterates through a list of objects to detect overlaps with a target entity's position and dimensions.
- `process_entity_state_flags`: The routine iterates through a loop, checking specific bits in byte fields of data structures pointed to by registers.
- `update_object_state_flags`: The routine modifies status flags and data fields within object structures passed as parameters, checking specific bits to determine execution flow.
- `update_entity_state_flags_2`: Modifies byte flags in two data structures based on conditional checks and global state variables.
- `clear_object_data_fields`: The routine clears specific bits in a byte pointed to by a register.
- `load_and_play_win_music`: Loads the DF1:WHENWIN.MUS file into memory and initializes audio playback routines for the victory sequence.
- `decrement_and_compare_byte`: The routine computes an address from an input pointer and a word offset, then decrements a byte at that address and checks if it became zero.
- `return_combined_registers`: The routine calls another routine and then returns the concatenation of the two input values it received in registers D0 and D1.
- `set_bit_and_configure_blitter`: The routine checks a bit in a byte pointer; if clear, it sets a bit in a global table and configures blitter registers with specific values.
- `return_immediately`: The routine executes five instructions that perform no observable work, such as reading or writing memory or calling other routines, before returning immediately.
- `search_table_for_value`: The routine iterates through a table of entries, comparing a global short value with each entry.
- `initialize_struct_fields`: The routine stores specific constants and copies values between fields of two structures pointed to by registers, setting up data for subsequent use.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `check_joystick_input`: The routine checks a global variable and, if zero, reads the CIAA_PRA register to determine joystick or mouse fire button status.
- `decrement_counter_and_check_flags`: The routine checks a global flag and a byte indexed by a short field in a structure.
- `decrement_specific_counter`: The routine checks a global flag and a byte in a structure.
- `process_global_state`: The routine modifies a global variable based on input registers, calls several other routines, and performs a loop that decrements the global variable until a condition is met before returning a value.
- `process_global_state_loop`: The routine modifies a global variable based on input registers, calls several other routines, and performs a loop that decrements the global variable while checking its value against specific constants.
- `set_global_and_return_concat`: The routine calls another routine, stores a constant value into a global variable, calls another routine, and returns a 64-bit value formed by concatenating the two input arguments.
- `copy_word_block`: The routine copies a block of 16-bit words from a source address calculated from a register to a fixed destination address in memory, decrementing a counter until the copy is complete.
- `copy_word_table`: The routine copies a block of 16-bit words from a source pointer in a register to a fixed destination table in memory, decrementing a counter until it reaches zero.
- `set_flag_and_store_word`: The routine checks a bit in a byte pointed to by a register.
- `play_game_over_music`: The routine initializes object tables and checks joystick input via the CIAA_PRA register.
- `copy_word_blocks`: The routine copies four words from a source pointer to a destination pointer repeatedly.
- `initialize_object_table_entries`: The routine calculates offsets and iterates to populate a table with data derived from global variables and structure fields.
- `copy_word_and_add_offset`: The routine reads a word from an offset in a structure pointed to by a register, calls another routine, then writes that word back to the same offset in another structure and adds a short value from a global variable to a short field in the second structure.
- `populate_object_table_entries_2`: The routine loops ten times, calling another routine each iteration and writing the returned values into consecutive entries of a table pointed to by a register.
- `populate_object_table_entries`: The routine loops ten times, calling another routine each iteration and writing the returned values into consecutive entries of a table pointed to by an address in a register.
- `initialize_custom_registers`: The routine initializes various custom chip registers, including COP1LCH, COP2LCH, COPJMP2, DMACON, DMACONR, INTENA, INTENAR, and INTREQ. It sets specific values to configure the Amiga's hardware for interrupt handling and DMA operations.
- `infinite_loop_halt`: The routine contains an unconditional infinite loop that prevents execution from proceeding.
- `store_byte_via_pointer`: The routine stores a byte from a register through a pointer.
- `update_tile_collision_state`: Calculates new tile coordinates based on input flags and structure data, then checks for collisions using lookup tables.
- `check_tile_collision`: The routine retrieves tile data from lookup tables based on input indices and a structure pointer, then compares four words from those tables against a mask derived from a flag and shift value.
- `compare_table_to_global`: The routine compares a short from a table with a short from a global variable, advancing through the table and the global variable in lockstep.
- `check_tile_collision_2`: Retrieves tile data from lookup tables based on input indices and a structure pointer, then compares four words from those tables.
- `decrement_byte_and_clear_flag`: The routine reads a 16-bit index from a structure, computes an address in a byte table using that index and an input word, then decrements the byte at that address.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `process_object_state_and_flags`: The routine manipulates flags and data within object structures, checks conditions against global variables and tables, and updates state fields based on comparisons and bit operations.
- `initialize_object_table`: The routine iterates through a table of structures, setting specific bits and copying fields from a source structure.
- `decode_object_table`: The routine walks a two-dimensional table pointed to by a0 and compares each entry against a global table of 16-bit patterns.
- `decrement_global_counter`: The routine stores a value from a register into a global variable.
- `search_tile_in_table`: The routine looks up a pointer from a table using a value derived from an input register and then scans a two-dimensional table of bytes.
- `fetch_table_entry_word`: Calculates an offset using a value from a structure pointed to by A5 and a global variable.
- `clear_object_flags_and_copy_fields`: The routine clears a global variable and sets a flag in a byte pointed to by a pointer stored in the caller's structure.
- `skip_if_flag_set`: The routine checks a flag in a byte pointed to by a pointer in A1.
- `sort_index_array`: The routine initializes an array of shorts with sequential values.
- `sort_index_array_2`: The routine initializes an array of shorts with sequential values.
- `check_flags_and_configure_blitter`: The routine checks bits in a byte pointer and a global table; if conditions are met, it toggles a bit in a byte at an offset and sets a bit in the byte pointer, then configures blitter registers if a global variable matches a calculated value.
- `decode_message_lengths_2`: The routine processes a block of four-byte entries using a bitmask in a register to decide which entries to handle.
- `decode_message_lengths`: The routine processes a block of four-byte entries using a bitmask in a register to decide which entries to handle.
- `configure_blitter_and_loop`: The routine checks flags in a structure and a global variable.
- `set_bit_and_configure_blitter_2`: The routine sets a specific bit in a byte pointed to by a pointer, stores a value in a global variable, and conditionally configures blitter registers with specific values if a register value does not match a global variable.
- `search_table_and_configure_blitter`: The routine iterates through a table of pointers, checking if coordinates within a range match values in the pointed structures.
- `loop_call_routine`: The routine calls another routine repeatedly in a loop.
- `decrement_loop_counter`: Reads a global 16-bit counter and repeatedly calls another routine while decrementing the value until it reaches -1.
- `loop_until_negative_one`: Reads a 16-bit value from a global variable and repeatedly calls another routine while decrementing the value until it equals negative one.
- `decrement_loop`: The routine loads a short value from a global variable and then repeatedly calls another routine while decrementing the value until it reaches -1.
- `update_entity_state_timers`: The routine processes a global entity structure by decrementing internal counters and updating flag bytes based on timer expiration.
- `scan_table_for_negative_byte`: Iterates through a data table starting at a global pointer, checking each byte for a negative value.
- `execute_self_modifying_loop`: The routine sets a global variable to point to another routine and executes a loop that decrements a counter while advancing a code pointer.
- `process_entity_animation_state`: Updates entity flags and animation indices based on position checks and state bits, then retrieves animation data from a lookup table.
- `update_object_animation_state`: The routine iterates through object records, checking flags and positional bounds to determine if an update is required.
- `execute_callback_table`: The routine checks a global short counter.
- `execute_callback_table_2`: The routine checks a global short counter.
- `copy_object_fields_to_state`: The routine copies several fields from a source structure pointed to by a0 into a state structure pointed to by a6, including a 32-bit value, two 32-bit values, and several 16-bit values.
- `initialize_structure_from_table`: The routine writes a constant to the first word of a table pointed to by a0, reads a word from that table and uses it to compute a short value stored in a global structure, copies several words and a long from the table into fixed offsets of a global structure, and clears a long field in that structure.
- `initialize_object_from_template`: The routine copies fields from a template structure pointed to by a0 into a working object pointed to by a6.
- `initialize_level_entity_data`: The routine initializes global state variables and clears memory blocks before configuring entity structures with specific flags and coordinates.
- `clear_memory_block`: Clears a block of memory starting at the address in a0 one byte at a time, using a dbra loop counter, and returns.
- `fill_memory_with_ff`: The routine iterates a counter in D7, writing the byte value 255 to the memory location pointed to by A0, incrementing the pointer by one byte in each iteration until the counter reaches -1.
- `setup_structure_from_index`: The routine sets a flag byte by ORing in a bit, clears several bytes in a structure, writes a constant into one byte, and copies two values from a pointer calculated using an index in D6 into fields of the structure.
- `set_memory_flags`: The routine sets a flag byte by ORing in a bit, clears several bytes in a structure, writes a constant into one byte, and copies two values from a pointer into fields of the structure.
- `copy_word_triple_step`: The routine copies 16-bit words from a source pointer in A0 to a destination pointer in A1, advancing the destination by three words each iteration while advancing the source by one word, repeating until a counter in D7 reaches -1.
- `clear_tables_and_recurse`: The routine clears a small table of words, adds a constant to a series of integers, and updates a global variable.
- `load_byte_and_configure_blitter`: The routine loads a byte from a table indexed by a register value and configures blitter registers with specific values if the register value differs from a global variable.
- `reset_global_and_call`: The routine clears a global variable to zero, calls another routine and returns a 32-bit value from register D0.
- `configure_blitter_for_index`: The routine compares an input byte with a value in a global array indexed by a register.
- `configure_blitter_for_value`: The routine compares an input byte with a global variable.
- `configure_blitter_for_alt_value`: The routine compares an input byte with a global variable.
- `configure_blitter_registers`: The routine compares an input byte with a global variable.
- `process_entity_list_and_update_state`: Iterates through a linked list of entities to check for matches and update flags based on coordinate bounds.
- `copy_bytes_from_table_2`: The routine calculates a source address by offsetting a global variable pointer using an input value.
- `copy_bytes_from_table`: The routine copies 64 bytes from a table pointed to by a global variable offset by the low word of the first argument shifted left by six, into a buffer pointed to by the second argument, writing one byte at a time with a stride of 40 bytes between successive writes.
- `poll_until_global_clears_2`: The routine calls another routine, then repeatedly calls another routine while checking a global variable, breaking the loop when the variable is zero.
- `poll_until_global_clears`: The routine calls another routine, then repeatedly calls another routine while checking a global variable.
- `propagate_table_value`: The routine loads a short from a global variable and uses it as a loop counter.
- `decrement_global_loop`: The routine reads a short value from a global variable, then repeatedly calls another routine while decrementing the value until it reaches -1.
- `interpolate_color_palette`: The routine checks a counter in a data structure and, if active, copies color data from a source buffer to a destination buffer.
- `set_or_clear_bit_and_update_table_2`: The routine sets or clears a specific bit in a byte pointed to by a pointer, using a bit index derived from a register.
- `clear_memory_table_2`: The routine clears a table pointed to by a pointer by writing a byte value to a series of fixed offsets and then stepping the pointer while decrementing a counter.
- `clear_memory_table`: The routine clears a table pointed to by a pointer by writing a byte value to a series of fixed offsets and then stepping the pointer while decrementing a counter.
- `copy_word_table_2`: The routine copies sixteen 16-bit words from a table derived from a global variable to two destinations: a fixed memory area and another global table.
- `call_indirect_function`: The routine checks a bit in a global variable and, if clear, calls a function pointer from a table indexed by that variable.
- `empty_loop_routine`: The routine contains no reachable instructions and consists of an infinite loop that never executes any operation.
- `store_calculated_words`: The routine calculates a starting value from several global variables and stores three 32-bit values to a global memory location.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 44 into d1, then loads 3 into d7.
- `store_word_pair_loop`: The routine writes a 32-bit value from a register to memory via a pointer, storing the low word then the high word, advances the pointer by four bytes, adds the original value to another register value, and repeats while decrementing an unsigned word counter until it wraps to zero.
- `store_word_pair_loop_2`: Writes a 32-bit value to memory via a pointer by storing the low word and high word separately, advances the pointer by four bytes, adds the value to another register, and repeats while decrementing a counter until it wraps to zero.
- `clear_keyboard_interrupt_state`: The routine checks a condition involving a custom chip register and a global variable.
- `check_interrupt_status_and_acknowledge`: The routine reads the Amiga interrupt enable and request registers, masks the request bits, and if a specific request bit is set it calls another routine, increments a global variable, and writes an acknowledge value to a custom chip register.
- `execute_unnamed_routine`: An executed routine consisting of twenty instructions that calls another unnamed routine.
- `store_register_state_to_globals`: Stores values from several registers into global variables and performs a calculation based on a pointer to update a global integer.
- `initialize_global_state`: Sets several global variables based on register values and enters a loop that calls another routine until a condition is met.
- `initialize_hardware_and_call`: Sets a constant in a register, loads hardware base addresses, calls two different routines, and writes three specific bytes to a memory offset.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d1, then returns.
- `tiny_tst_rts`: Small routine of 2 instructions that tests d4, then returns.
- `decrement_and_jump`: Decrements a register and branches to one of two locations based on the result.
- `execute_nested_loops`: Runs a nested loop structure that updates several global variables and performs conditional branching and bitwise XOR operations.
- `branch_and_decrement`: Loads a pointer into a register, restores registers from the stack, decrements a global variable, and performs conditional branching.
- `update_masked_byte`: Determines a value based on two global variables and stores the bitwise complement of that value into a byte at an offset from a pointer.
- `check_button_status`: Calls another routine, performs multiple multiplications, checks a bit in a hardware register, and executes a loop.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d4, then returns.
- `wait_for_ciaa_bit_clear`: Polls a bit in the CIAA Port A register in a loop and clears a global variable when the loop terminates.
- `update_global_value`: Moves a value to a register, checks a longword, calls another routine, and conditionally updates a global longword with a calculated difference.
- `set_byte_and_branch`: Loads a byte into a register, sets its highest bit, and branches to another location.
- `write_inverted_bytes`: Enters a loop that writes the bitwise complement of two different values into a table at a specific offset.
- `store_alternating_bytes`: Executes a loop that stores two different bytes into a table at a specific offset, controlled by a decrementing counter.
- `loop_and_store_byte`: Stores a byte from a register into a table and executes two nested loops using multiplication and decrementing counters.
- `write_memory_and_wait`: Writes several values to memory offsets from a pointer and enters a loop that waits for a specific bit in a memory location to be set.
- `nop_padding`: This routine is empty and performs no operations.
- `nop_padding_sequence`: This routine is empty and consists only of two no-operation instructions.
- `process_masked_data`: Performs a series of checks on global variables and uses a loop to populate a table by combining values from two other tables using bitwise operations and shifts.
- `combine_masked_longword_pair`: Returns a value built from the two longwords addressed by the pointer, each masked by a register value, with the first shifted left by one bit.
- `xor_sum_table`: Iterates through a table of longwords, performing a bitwise XOR on all values, and returns the result masked by a register.
- `update_hardware_registers`: Calculates values based on registers and a table to update several memory locations, including hardware-related offsets and a bitmask for a port.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
