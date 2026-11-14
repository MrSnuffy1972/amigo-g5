# Game 2059 (speedball 2 - brutal deluxe (1990)(imageworks)[cr csl][t +3 skillion]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02059). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (917072 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | FAIL, 117 of 145 named (0 author symbols), 117 of 145 described (3 by rule, 110 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1231 of 1231 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1024 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2760 of 2760 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4909 |
| code | 12020 |
| data | 899015 |
| padding | 616 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +3 Skillion].adf:bootblock | 1 | 77 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 468 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1854 | 107 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 361 | 15 |
| Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +3 Skillion].adf bytes 1024-2048 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- speedball 2 - brutal deluxe (1990)(imageworks)[cr csl][t +3 skillion]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +3 Skillion].adf: custom-format (non-DOS) disk, 1757 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 131 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 768 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 332 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- speedball 2 - brutal deluxe (1990)(imageworks)[cr csl][t +3 skillion]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +3 Skillion].adf: bytes 1024-2048 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate+address
- speedball 2 - brutal deluxe (1990)(imageworks)[cr csl][t +3 skillion]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +3 Skillion].adf: 1 logged loads cover 1024 bytes of the image
- speedball 2 - brutal deluxe (1990)(imageworks)[cr csl][t +3 skillion]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +3 Skillion].adf: 3 track-loaded blocks found in recovered runtime segments and classified through them
- speedball 2 - brutal deluxe (1990)(imageworks)[cr csl][t +3 skillion]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +3 Skillion].adf: blocks 22-1759 (bytes 11264-901120) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.46)
- 1 never-loaded region(s), 1024 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 7168 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `init_and_poll_view` (runtime segment an address): Initializes a buffer by clearing two words and then, if a specific byte matches a value, calls another routine. Kind: disk hardware/track validation loop. Pass path: the loop continues (sVar2 decrements) until the check fails or the retry limit is reached.
- `set_audio_dma_control` (runtime segment an address): The routine writes to a global variable, another global variable, a custom chip register, a custom chip register, a global variable, and a global variable. Kind: disk hardware/sync check. Pass path: the branch following the call that does not lead to a failure/error state (e.g., the bne.b an address or the bsr.w an address depending on the specific logic flow).
- `set_global_flags` (runtime segment an address): It writes two constants to the same global variable in sequence, leaving the second value in place. Kind: disk hardware/track check. Pass path: the instruction immediately following the call (e.g., moveq #an address, d4).
- `modify_global_bitmask` (runtime segment an address): Checks a value in memory and, if negative, updates a global variable by setting and clearing a specific bit based on a value from a table. Kind: checksum or hardware validation trap. Pass path: the branch at an address (the 'beq.b' instruction implies the check must result in a non-zero/non-equal state to proceed normally, or the 'beq' skips the failure path).
- `set_bit_in_global_variable` (runtime segment an address): It reads a 16-bit value from a table pointed to by a register, adds a constant, and masks the result to 5 bits to use as a bit index. Kind: bit-manipulation flag/checksum trap. Pass path: The bit being set (via the bitwise OR operation) likely acts as a 'validation flag' in a global state variable. If the bit is not set correctly by this routine, subsequent code will fail a conditional check (e.g., `btst` or `tst.b`) and tri.
- `set_flag_and_call_twice` (runtime segment an address): It stores a constant byte into a global variable, then calls another routine twice in sequence. Kind: disk hardware/integrity check. Pass path: the instruction following the second call (the code continues normally if the flag is set correctly).
- `read_port_into_status_byte` (runtime segment an address): It calls another routine to obtain a byte, clears bit one of that byte when a secondary value from the same call is non-negative, and then forces bit zero high before storing the result into a global variable. Kind: disk hardware/status check. Pass path: extraout_D1b >= 0.
- `get_ciab_mask` (runtime segment an address): Calls another routine to retrieve a value and returns it. Kind: disk hardware/CIAB_PRB check. Pass path: the code continues execution following the 'after' instructions provided in the call sites.
- `compute_ciab_mask_from_table` (runtime segment an address): It derives a bit mask from a global variable by clearing one bit chosen by a frame-relative index, and also clears a second bit when a related table entry has its low bit set. Kind: disk hardware/CIAB_PRB lookup. Pass path: the branch where the bitmask check does not trigger a modification or where the returned mask is used for valid data access.

## Functions


### Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +3 Skillion].adf:bootblock

- `copy_block_then_trap`: Boot-block entry that copies 513 longwords from the start of the boot block to a fixed address, then raises a trap.
- `open_device_and_send_request`: Initialises status globals, then performs system-table lookups.
- `write_five_constant_longwords`: Writes five fixed longword constants into a global block and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_tables`: Populates various data structures through memory copies, clears specific table entries based on a condition, and enters an infinite loop after performing conditional global variable assignments.
- `initialize_game_data_tables`: Performs a series of large memory block copies to populate various data structures and tables from source locations to destination offsets.
- `copy_data_to_global_offset`: It copies a block of data from a source to a fixed destination, and then copies another block of data to a destination calculated by adding a global variable to a base address.
- `copy_and_clear_data`: Copies a block of data from one memory location to another based on a condition and then clears a sequence of words in a table.
- `clear_memory_table`: It performs a calculation on a global variable and then fills a table with zeros.
- `execute_unreachable_instructions`: Executes three instructions located within a code gap.
- `copy_data_to_offset_block`: It copies a large block of data to a fixed destination and then copies a second block of data to a destination offset by a global variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `sub_0460`: undescribed.
- `assign_register_to_global_recursively`: Repeatedly assigns a value from a register to a global variable by calling itself until a specific condition is met.
- `assign_constant_sequence`: Stores a sequence of large constant values into a global variable and then calls another routine.
- `initialize_data_tables`: Fills a byte table with divided values, a short table with multiplied values, and copies data from one table to another.
- `populate_scaled_table`: Fills a table with values calculated by multiplying an incrementing index by a constant, repeating this process several times.
- `tiny_addq_addq_rts`: Small routine of 3 instructions that adds 2 to a7, then adds 2 to (a7), then returns.
- `average_structure_field_pairs`: Iterates through a list of pointers to calculate the arithmetic mean of two word-sized fields and stores the result in a third field for two separate pairs of values.
- `average_structure_fields`: Iterates through a list of structures to calculate the average of two word fields and stores the result in a third field.
- `nested_loop_with_calls`: It executes a series of nested loops that perform bitwise operations and call another routine multiple times.
- `process_bit_depth_table`: It iterates through a table, performing bitwise checks and subtractions on values based on their bit patterns, while calling another routine.
- `wait_for_non_zero`: Enters a loop that waits for a global variable to become zero before setting it to a specific value.
- `execute_infinite_loop_3`: The routine is empty as it consists only of an infinite loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_padding`: This routine is empty and performs no operations.
- `poll_interrupt_status`: Enters a nested loop structure that checks specific bits in a memory location and calls another routine if corresponding bits are set in a different memory location.
- `update_table_entry`: Calculates an offset into a table based on input values, updates a byte, and either performs a series of conditional data merges or inserts a new entry into a list.
- `update_coordinate_data`: Updates 32-bit values and 16-bit values in various memory locations based on conditional checks of a structure.
- `conditionally_call_routine`: This routine checks a global word value and calls another routine if that value is zero.
- `update_hardware_state_structure`: Updates a multi-word data structure using values from memory and modifies a global variable based on a conditional check.
- `update_hardware_configuration`: Updates several fields in a structure and modifies hardware registers related to interrupts and DMA based on conditional logic.
- `decrement_and_recurse_counter_2`: Checks a byte value at a specific offset within a structure and decrements it if it is non-zero.
- `decrement_and_recurse_counter`: Checks a byte value at a specific offset within a structure and decrements it if it is non-zero.
- `process_state_transitions`: Manages complex state changes within a structure by updating various word and byte fields based on comparisons with a table of values.
- `sub_05EA`: undescribed.
- `initialize_data_structure`: Clears a block of memory, updates several fields within a structure using pointers and offsets, and modifies DMA and interrupt enable registers.
- `update_game_state_and_hardware`: Updates hardware registers for interrupts and DMA while managing state counters and structure fields.
- `parse_and_update_entity_state`: Parses a data stream using a switch-based command loop to update structure fields and timers.
- `process_interpolated_animation_frames`: Iterates through a data structure to decrement frame counters and update position or state offsets based on table lookups.
- `sub_0AF8`: undescribed.
- `process_event_table_and_timers`: Iterates through a data table using a switch statement to update various structure fields and manages multiple decrementing countdown timers.
- `process_event_table_and_timers_2`: Parses a data table using a switch-case structure to update various state fields and pointers.
- `parse_and_execute_script_data`: Iterates through a data block to calculate a sum and then processes a sequence of commands via a switch-based dispatcher.
- `parse_and_execute_bytecode`: Reads a sequence of bytes from a buffer and executes a custom instruction set via a switch-based dispatcher.
- `process_interpolated_animation_frames_2`: Iterates through a data structure to decrement frame counters and update position or state values based on table lookups.
- `sub_0B8A`: undescribed.
- `sub_0B90`: undescribed.
- `process_entity_timers_and_state`: Iterates through a list of data structures to decrement internal counters and update position or state values based on timer expirations.
- `parse_command_script_data`: Iterates through a data table to parse command opcodes and execute corresponding logic.
- `process_event_stream`: Iterates through a data stream to parse command bytes and update internal state structures.
- `parse_event_data_stream`: Iterates through a data stream to extract command identifiers and parameters, updating internal state structures and managing counters.
- `process_event_data_stream`: Parses a data stream containing command identifiers and associated parameters to update internal state structures.
- `sub_0C12`: undescribed.
- `sub_0C1E`: undescribed.
- `sub_0C2A`: undescribed.
- `sub_0C32`: undescribed.
- `sub_0C4C`: undescribed.
- `sub_0C52`: undescribed.
- `sub_0C64`: undescribed.
- `load_entity_attribute_data`: Calculates an offset into a table to load entity properties into a structure.
- `assign_global_pointer`: Assigns the address of a global variable to a pointer located at an offset within a structure.
- `process_event_data_stream_2`: Parses a data stream from a table to update various state structures and counters.
- `copy_word_field_triples`: Copies two word fields to two other offsets in a structure, advancing the base pointer two bytes per pass, and repeats for three passes.
- `copy_word_field_pairs`: Copies two word fields to two other offsets in a structure, advancing the base pointer two bytes per pass, and repeats for two passes.
- `sub_15C8`: undescribed.
- `store_constant_words`: Stores two specific constant words into two consecutive memory locations.
- `sub_1620`: undescribed.
- `sub_165A`: undescribed.
- `sub_167A`: undescribed.
- `sub_16D8`: undescribed.
- `execute_infinite_loop`: Enters an infinite loop that performs no other actions.
- `update_global_word`: Increments a global word and performs a bitwise XOR on another global value.
- `update_joystick_status`: Reads input from joystick and mouse hardware registers and updates a global variable with a masked version of the input.
- `return_empty_routine`: This routine is empty and performs no operations before returning.
- `mask_and_distribute_words`: Copies a sequence of words to a destination while simultaneously masking each word and storing the result into three separate memory locations via incrementing pointers.
- `copy_and_mask_words`: Copies words from a source to a destination while simultaneously storing a masked version of each word into three different memory locations.
- `sub_22C0`: undescribed.
- `sub_2338`: undescribed.
- `sub_2394`: undescribed.
- `sub_23BA`: undescribed.
- `sub_2454`: undescribed.
- `sub_24E6`: undescribed.
- `sub_250C`: undescribed.
- `sub_252A`: undescribed.
- `sub_255A`: undescribed.
- `write_multiple_word_values`: If a value is non-negative, it writes that value to a specific offset in a structure and then performs multiple consecutive writes of the same value to another offset in a global structure.
- `swap_global_values`: Swaps the contents of a global variable with a temporary value based on conditional checks of the variable's sign.
- `wait_for_signature`: Polls a global variable in a loop until it matches a specific 32-bit pattern and another variable matches a specific 16-bit pattern.
- `reset_global_variable`: Sets a global variable to zero and then calls another routine.
- `sub_26B8`: undescribed.
- `sub_26E8`: undescribed.
- `perform_scaled_division_and_store`: Calls a routine to perform a scaled division and stores the resulting value into a memory location.
- `calculate_scaled_division`: Performs a division-like calculation using a loop and arithmetic operations to determine a value based on input parameters.
- `process_dma_validation_loop`: Runs a fixed loop of four iterations, clearing a small block of fields and calling helper routines to validate and repack a 32-bit result.
- `init_and_poll_view`: Initializes a buffer by clearing two words and then, if a specific byte matches a value, calls another routine.
- `render_park_view`: The routine writes a constant to a memory location derived from a pointer, then repeatedly calls other routines to update a flag, toggle a flag, and compute a checksum using a decrementing counter.
- `validate_input_with_retry`: Repeatedly calls helper functions to check a flag and decrement a counter, returning early if the flag is clear or the counter expires.
- `validate_object_state`: Performs complex conditional checks on structure fields and global flags, calling various helper routines to verify state and return specific status codes.
- `empty_stub_routine`: It performs no observable work and simply returns.
- `set_flag_and_increment_counter`: It sets a bit in a 16-bit word located a few bytes before a pointer in A6, using a bit index derived from D3.
- `clear_table_entries`: It clears a table by writing zero to successive entries.
- `repack_result_word_and_byte`: Calls a routine and repacks its 32-bit result, placing the low word first and the top byte of the result in the second word.
- `interleave_word_bits`: It reads two consecutive words from a pointer and returns a value formed from their low bits.
- `process_and_pass_value`: It calls a routine to produce a 32-bit value, passes that value to another routine, and then returns.
- `xor_accumulate_and_mask`: It XORs a sequence of nine words into an accumulator and returns the result masked to retain alternating bits.
- `compute_xor_checksum`: It iterates through a sequence of words pointed to by an input pointer, XORing each into an accumulator.
- `fill_memory_pattern`: The routine clears a block of memory by writing a repeated 32-bit pattern to each word.
- `initialize_object_state`: Calls another routine and uses its result to compute three integers stored at fixed offsets in a structure, then writes two constant words and a derived word into it.
- `set_object_transforms`: Writes several fields of a structure from the results of unnamed routines, storing constant words and copying values between the results, and calls a routine that waits for the blitter to finish.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `call_then_store_constants`: It calls another routine, then stores a sequence of constant values into global variables before returning.
- `modify_byte_based_on_flag`: It calls another routine twice and then modifies a byte based on the value of the preceding byte in memory.
- `clear_flag_byte`: Reads a byte pointed to by a pointer and a flag from the preceding byte.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `set_audio_dma_control`: The routine writes to a global variable, another global variable, a custom chip register, a custom chip register, a global variable, and a global variable.
- `check_status_flags`: Repeatedly checks a bit in a structure and writes two constant values to a global variable based on the result.
- `set_global_flags`: It writes two constants to the same global variable in sequence, leaving the second value in place.
- `modify_global_bitmask`: Checks a value in memory and, if negative, updates a global variable by setting and clearing a specific bit based on a value from a table.
- `set_bit_in_global_variable`: It reads a 16-bit value from a table pointed to by a register, adds a constant, and masks the result to 5 bits to use as a bit index.
- `set_flag_and_call_twice`: It stores a constant byte into a global variable, then calls another routine twice in sequence.
- `update_table_entry_2`: Reads a 16-bit value from a table via an index in a structure, calls another routine when the value is negative, then spins a number of times derived from the difference between two 16-bit fields before writing a value from a register back into the table and calling another routine.
- `poll_status_with_timeout`: Repeatedly polls a global status byte up to eighty-five times, clearing a word in a table indexed by a value from the caller's stack frame if a specific bit is set, and returns a fixed non-zero value if the loop completes without success.
- `read_port_into_status_byte`: It calls another routine to obtain a byte, clears bit one of that byte when a secondary value from the same call is non-negative, and then forces bit zero high before storing the result into a global variable.
- `get_ciab_mask`: Calls another routine to retrieve a value and returns it.
- `compute_ciab_mask_from_table`: It derives a bit mask from a global variable by clearing one bit chosen by a frame-relative index, and also clears a second bit when a related table entry has its low bit set.
- `wait_ciab_timer`: Waits until a global variable's low bit clears, then waits until another routine stops returning one, using the CIAB timer channel A to pace the busy-wait loops.
- `set_timer_constants`: Checks if a global variable's low bit is clear and a register value is not one, then writes three constant values to a global variable.
- `set_ciab_timer`: It writes three constant values to a global variable in sequence.
- `call_unpack_resource_container`: Calls a routine that parses a structured data container to extract and decompress embedded data blocks.
- `unpack_resource_container`: Parses a structured data container by locating specific magic markers to extract and decompress embedded data blocks.
- `copy_variable_length_data`: Iterates through a buffer to copy or move blocks of data of varying lengths based on values found within the buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_complex_conditional_logic`: This routine performs a series of conditional checks on a global variable and executes another routine twice before entering a loop.
- `execute_sequential_tasks`: Performs a series of conditional checks on a global variable and executes a sequence of subroutines, including data merging and memory clearing operations.
- `execute_conditional_loop`: Checks a global variable against specific bit patterns and characters before executing a subroutine twice and entering a loop that calls another routine sixteen times.
- `run_conditional_loop`: Checks a global variable against specific bit patterns and characters before executing a subroutine twice and entering a loop that calls another routine multiple times.
- `execute_fixed_loop`: Verifies a global variable's status and then executes a subroutine twice followed by a loop that runs for fifteen iterations.
- `run_variable_loop`: Verifies a global variable's status and then executes a subroutine twice followed by a loop that runs for a fixed number of iterations.
- `call_other_routine_twice`: This routine calls another routine two times.
- `validate_and_update_global_variable`: Performs conditional checks on a global variable based on specific bit patterns and values, returning early if certain criteria are met, or updating the variable to a specific non-zero value if it is zero.
- `process_string_data`: Iterates through a buffer, performing conditional indirect calls and data merging based on the contents of the buffer and a global variable.
- `execute_indirect_routine`: This routine calls another routine using a pointer stored in a register.
- `merge_data_into_tables`: It iterates through a source table and merges five 16-bit values into specific offsets of a destination table using bitwise operations.
- `copy_data_to_tables`: It iterates through a source table and copies five 16-bit values into specific offsets of a destination table.
- `copy_data_from_tables`: It iterates through a source table and copies five 16-bit values from specific offsets of a destination table into the source.
- `clear_memory_block`: It clears a large block of memory by writing zero to successive 32-bit words in a loop.
- `execute_infinite_loop_2`: The routine enters an unconditional infinite loop, preventing further execution of the calling thread.
