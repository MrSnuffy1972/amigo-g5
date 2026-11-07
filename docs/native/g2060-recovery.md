# Game 2060 (speedball 2 - brutal deluxe (1990)(imageworks)[cr csl][t +5 betrayal]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02060). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (917072 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 128 of 128 named (0 author symbols), 128 of 128 described (5 by rule, 122 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1231 of 1231 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1024 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2877 of 2877 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10522 |
| code | 10968 |
| data | 895046 |
| padding | 536 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +5 Betrayal].adf:bootblock | 1 | 94 | 3 |
| logged disk load from Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +5 Betrayal].adf boot-block load | 1 | 155 | 2 |
| logged disk load from Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +5 Betrayal].adf boot-block load | 1 | 155 | 2 |
| logged disk load from Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +5 Betrayal].adf boot-block load | 1 | 155 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 463 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1494 | 85 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 361 | 15 |
| Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +5 Betrayal].adf bytes 1024-2048 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- speedball 2 - brutal deluxe (1990)(imageworks)[cr csl][t +5 betrayal]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +5 Betrayal].adf: custom-format (non-DOS) disk, 1754 non-empty blocks: matched against track-loaded segments
- speedball 2 - brutal deluxe (1990)(imageworks)[cr csl][t +5 betrayal]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +5 Betrayal].adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- speedball 2 - brutal deluxe (1990)(imageworks)[cr csl][t +5 betrayal]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +5 Betrayal].adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- speedball 2 - brutal deluxe (1990)(imageworks)[cr csl][t +5 betrayal]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +5 Betrayal].adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- runtime segment an address: 131 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 768 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 332 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- speedball 2 - brutal deluxe (1990)(imageworks)[cr csl][t +5 betrayal]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +5 Betrayal].adf: bytes 1024-2048 look like code no run loaded: recovered position-independently (load address unknown)
- speedball 2 - brutal deluxe (1990)(imageworks)[cr csl][t +5 betrayal]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +5 Betrayal].adf: 4 logged loads cover 6656 bytes of the image
- speedball 2 - brutal deluxe (1990)(imageworks)[cr csl][t +5 betrayal]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +5 Betrayal].adf: 3 track-loaded blocks found in recovered runtime segments and classified through them
- speedball 2 - brutal deluxe (1990)(imageworks)[cr csl][t +5 betrayal]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +5 Betrayal].adf: blocks 7-10 (bytes 3584-5632) outside the filesystem hold data, not code (opcode density 0.023, entropy 6.98)
- speedball 2 - brutal deluxe (1990)(imageworks)[cr csl][t +5 betrayal]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +5 Betrayal].adf: blocks 22-1759 (bytes 11264-901120) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.46)
- 1 never-loaded region(s), 1024 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `init_and_poll_view` (runtime segment an address): Initializes a buffer by clearing two words and then, if a specific byte matches a value, calls another routine. Kind: disk track/hardware validation loop. Pass path: the loop continues (sVar2 decrementing) until the check fails or the loop limit is reached; a successful check allows the loop to proceed to the next iteration or exit via the break condition if the hardware state is valid..
- `set_audio_dma_control` (runtime segment an address): The routine writes to a global variable, another global variable, a custom chip register, a custom chip register, a global variable, and a global variable. Kind: disk hardware/sync check. Pass path: the branch following the call that does not lead to a crash or error loop, typically indicated by the 'bne' or 'beq' logic in the caller.
- `set_global_flags` (runtime segment an address): It writes two constants to the same global variable in sequence, leaving the second value in place. Kind: disk hardware/status flag setter. Pass path: execution continues to the instruction immediately following the call (e.g., moveq #an address, d4).
- `modify_hardware_control` (runtime segment an address): Writes a word to a hardware register, tests a value at an offset from a pointer, and conditionally sets a register to a constant value. Kind: disk hardware/track check. Pass path: bsr.w an address.
- `set_bit_in_global_variable` (runtime segment an address): It reads a 16-bit value from a table pointed to by a register, adds a constant, and masks the result to 5 bits to use as a bit index. Kind: bit-masking status flag update. Pass path: the bit remains set or the subsequent logic proceeds assuming the hardware state is valid.
- `set_flag_and_call_twice` (runtime segment an address): It stores a constant byte into a global variable, then calls another routine twice in sequence. Kind: disk hardware/integrity check. Pass path: the instruction following the second call (u5__sub_2F6E).
- `read_port_into_status_byte` (runtime segment an address): It calls another routine to obtain a byte, clears bit one of that byte when a secondary value from the same call is non-negative, and then forces bit zero high before storing the result into a global variable. Kind: disk hardware/status check. Pass path: extraout_D1b >= 0.
- `update_ciab_mask` (runtime segment an address): Calls another routine and stores its return value into a global variable. Kind: disk hardware/CIAB_PRB check. Pass path: sub_2E7A (caller 1) or sub_2920 (caller 2).
- `compute_ciab_mask_from_table` (runtime segment an address): It derives a bit mask from a global variable by clearing one bit chosen by a frame-relative index, and also clears a second bit when a related table entry has its low bit set. Kind: disk hardware/CIAB_PRB lookup. Pass path: the branch where the bitmask is correctly computed and the subsequent code continues execution without triggering a crash or error state.

## Functions


### Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +5 Betrayal].adf:bootblock

- `initialize_boot_environment`: The routine sets up memory pointers, copies code to a specific address, allocates memory via OS library calls, and configures DMA registers before jumping to the main program.
- `initialize_dma_and_jump`: Configures DMA control registers and executes OS library calls to set up hardware access.
- `write_custom_register_sequence`: Writes a specific sequence of constant values to custom chip registers.

### logged disk load from Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +5 Betrayal].adf boot-block load

- `shift_and_combine_bits`: Reads a word from a memory offset, combines it with a register value, and returns a right-shifted result with the high bit set.
- `iterate_bit_shift_loop`: Iterates a counter downwards, shifting a value right by one bit each cycle.

### logged disk load from Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +5 Betrayal].adf boot-block load

- `shift_and_combine_bits`: Reads a word from a memory offset, combines it with a register value, and returns a right-shifted result with the high bit set.
- `iterate_bit_shift_loop`: Iterates a counter downwards, shifting a value right by one bit each cycle.

### logged disk load from Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CSL][t +5 Betrayal].adf boot-block load

- `shift_and_combine_bits`: Reads a word from a memory offset, combines it with a register value, and returns a right-shifted result with the high bit set.
- `iterate_bit_shift_loop`: Iterates a counter downwards, shifting a value right by one bit each cycle.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_data`: Populates various data structures through memory block copies, initializes tables with calculated values, and enters an infinite loop if a specific global constant is met.
- `initialize_game_data_tables`: Performs a series of large memory block copies to populate various data structures and tables from source locations to destination offsets.
- `copy_data_to_global_offset`: It copies a block of data from a source to a fixed destination, and then copies another block of data to a destination calculated by adding a global variable to a base address.
- `copy_and_clear_memory`: Copies a block of data from one memory location to another and subsequently clears a series of words in a different memory block.
- `clear_memory_table`: It performs a calculation on a global variable and then fills a table with zeros.
- `execute_recursive_calls`: The routine calls itself twice before returning.
- `copy_two_data_blocks`: If a global variable is non-zero, it copies a block of data to a fixed destination and then copies a second block of data to a destination offset by that same global variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `assign_register_to_global_recursively`: Repeatedly assigns a value from a register to a global variable while calling itself recursively until the register value is less than zero.
- `assign_constant_sequence`: Stores a sequence of large constant values into a global variable and then calls another routine.
- `initialize_data_tables`: Fills a byte table with divided values, a short table with multiplied values, and copies data from one table to another.
- `populate_scaled_table`: Fills a table with values calculated by multiplying an incrementing index by a constant, repeating this process several times.
- `tiny_addq_addq_rts`: Small routine of 3 instructions that adds 2 to a7, then adds 2 to (a7), then returns.
- `branch_to_offset`: Loads a word into a register and branches to another routine.
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
- `conditionally_call_routine`: checks a global word value and calls another routine if that value is zero.
- `execute_sequential_pointer_calls`: Loads three consecutive memory addresses into a register and calls another routine for each address.
- `update_hardware_configuration`: Updates several fields in a structure and modifies hardware registers related to interrupts and DMA based on conditional logic.
- `load_addresses_and_call`: Loads four different addresses into a register and calls another routine after each load.
- `check_and_decrement_byte`: Tests a value at an offset, and if it is non-zero, decrements a byte at another offset and calls another routine.
- `process_state_transitions`: Manages complex state changes within a structure by updating various word and byte fields based on comparisons with a table of values.
- `initialize_data_structure`: Clears a block of memory, updates several fields within a structure using pointers and offsets, and modifies DMA and interrupt enable registers.
- `update_game_state_and_hardware`: Updates hardware registers for interrupts and DMA while managing state counters and structure fields.
- `transfer_byte_to_offset`: Loads a byte from a memory location pointed to by a register and stores it as a word at a fixed offset from another register before performing a short branch.
- `fetch_byte_and_branch`: Loads a byte from the address pointed to by a register into another register and performs a short relative branch.
- `multiply_and_store_result`: loads a byte from a pointer, multiplies it by a word from a pointer, and stores the result at an offset from another pointer before branching.
- `divide_and_store_quotient`: loads a byte from a pointer, divides a constant by that byte, and stores the quotient in a location pointed to by a register before branching.
- `calculate_scaled_sum`: iterates through a sequence of bytes to calculate a sum, multiplies that sum by a value from a pointer, and stores the result at an offset from another pointer before branching.
- `parse_and_execute_bytecode`: Reads a sequence of bytes from a data stream to update a structure and execute a series of nested sub-routines.
- `fetch_byte_and_branch_2`: Loads a byte from a memory location pointed to by a register into a data register and performs a short relative branch.
- `process_event_queues`: Iterates through two separate decrementing timer queues to trigger specific state updates or recursive calls.
- `fetch_and_index_table`: Pops a byte from a pointer into a register, performs arithmetic on it, and uses the result to index into a table located at a fixed memory address.
- `load_pointer_and_branch`: Loads a longword from a pointer into a register and increments that pointer.
- `store_and_branch_byte`: Moves a byte from a pointer, conditionally extends it to a word, adds it to a value at an offset, and branches to another routine.
- `store_word_and_branch`: Stores a word from a register into an offset of a pointer and then performs a short branch.
- `parse_entity_data_stream`: Reads a data stream to populate an entity structure by fetching values from a table based on an index.
- `assign_global_pointer`: Assigns the address of a global variable to a pointer located at an offset within a structure.
- `fetch_and_branch_table_entry`: Fetches a byte from a pointer, performs arithmetic on it, retrieves a value from a table using a pointer and the modified byte, and branches to another routine.
- `copy_word_field_triples`: Copies two word fields to two other offsets in a structure, advancing the base pointer two bytes per pass, and repeats for three passes.
- `copy_word_field_pairs`: Copies two word fields to two other offsets in a structure, advancing the base pointer two bytes per pass, and repeats for two passes.
- `calculate_coordinate_offsets`: Performs arithmetic on multiple global variables to update a set of related 32-bit values.
- `initialize_constant_values`: Stores two large constant words into two consecutive global variables.
- `store_word_to_offset`: Stores a word from a register into a memory location at a specific offset from a pointer.
- `tiny_subq_sub`: Small routine of 2 instructions that subtracts 1 from (a0)+, then subtracts d3 from (a3).
- `initialize_hardware_registers`: Writes a series of constant values to specific hardware register addresses, including DMA and interrupt control registers.
- `execute_infinite_loop`: Enters an infinite loop that performs no other actions.
- `update_global_word`: Increments a global word and performs a bitwise XOR on another global value.
- `tiny_ori`: Small routine of 1 instruction that merges 170 into d5.
- `update_joystick_status`: Reads input from joystick and mouse hardware registers and updates a global variable with a masked version of the input.
- `return_empty_routine`: This routine is empty and performs no operations before returning.
- `mask_and_distribute_words`: Copies a sequence of words to a destination while simultaneously extracting the lower twelve bits of each word and storing that masked value into three separate memory locations at fixed offsets.
- `copy_and_mask_words`: Copies words from a source to a destination while simultaneously storing a masked version of each word into three different memory locations.
- `write_multiple_word_values`: If a value is non-negative, it writes that value to a specific offset in a structure and then performs multiple consecutive writes of the same value to another offset in a global structure.
- `swap_global_values`: Swaps the contents of a global variable with a temporary value based on conditional checks of the variable's sign.
- `poll_global_status`: Repeatedly calls another routine until specific values are found in two global variables.
- `reset_global_variable`: Sets a global variable to zero and then calls another routine.
- `perform_scaled_division_and_store`: Calls a routine to perform a scaled division and stores the resulting value into a memory location.
- `calculate_scaled_division`: Performs a division-like calculation using a loop and arithmetic operations to determine a value based on input parameters.
- `process_dma_validation_loop`: Runs a fixed loop of four iterations, clearing a small block of fields and calling helper routines to validate and repack a 32-bit result.
- `init_and_poll_view`: Initializes a buffer by clearing two words and then, if a specific byte matches a value, calls another routine.
- `render_park_view`: The routine writes a constant to a memory location derived from a pointer, then repeatedly calls other routines to update a flag, toggle a flag, and compute a checksum using a decrementing counter.
- `validate_input_with_retry`: Repeatedly calls helper functions to check a flag and decrement a counter, returning early if the flag is clear or the counter expires.
- `process_complex_object_state`: Performs extensive conditional checks on structure fields and input flags, involving bitwise mixing, checksum calculation, and object transform updates.
- `empty_stub_routine`: It performs no observable work and simply returns.
- `set_flag_and_increment_counter`: It sets a bit in a 16-bit word located a few bytes before a pointer in A6, using a bit index derived from D3.
- `clear_table_entries`: It clears a table by writing zero to successive entries.
- `repack_result_word_and_byte`: Calls a routine and repacks its 32-bit result, placing the low word first and the top byte of the result in the second word.
- `interleave_word_bits`: It reads two consecutive words from a pointer and returns a value formed from their low bits.
- `process_and_pass_value`: It calls a routine to produce a 32-bit value, passes that value to another routine, and then returns.
- `load_register_and_constant`: It loads an address from a table into a register and sets a second register to a constant value.
- `compute_xor_checksum`: It iterates through a sequence of words pointed to by an input pointer, XORing each into an accumulator.
- `fill_memory_pattern`: The routine clears a block of memory by writing a repeated 32-bit pattern to each word.
- `initialize_object_state`: Calls another routine and uses its result to compute three integers stored at fixed offsets in a structure, then writes two constant words and a derived word into it.
- `set_object_transforms`: Writes several fields of a structure from the results of unnamed routines, storing constant words and copying values between the results, and calls a routine that waits for the blitter to finish.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `call_then_store_constants`: It calls another routine, then stores a sequence of constant values into global variables before returning.
- `call_bitwise_mix_twice`: It pushes a register to the stack, shifts its value, calls another routine, restores the value, and calls that same routine again.
- `clear_flag_byte`: Reads a byte pointed to by a pointer and a flag from the preceding byte.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `set_audio_dma_control`: The routine writes to a global variable, another global variable, a custom chip register, a custom chip register, a global variable, and a global variable.
- `check_hardware_status`: Calls another routine, tests a bit in a hardware register, and performs conditional branches and register assignments.
- `set_global_flags`: It writes two constants to the same global variable in sequence, leaving the second value in place.
- `modify_hardware_control`: Writes a word to a hardware register, tests a value at an offset from a pointer, and conditionally sets a register to a constant value.
- `set_bit_in_global_variable`: It reads a 16-bit value from a table pointed to by a register, adds a constant, and masks the result to 5 bits to use as a bit index.
- `set_flag_and_call_twice`: It stores a constant byte into a global variable, then calls another routine twice in sequence.
- `update_table_entry_2`: Reads a 16-bit value from a table via an index in a structure, calls another routine when the value is negative, then spins a number of times derived from the difference between two 16-bit fields before writing a value from a register back into the table and calling another routine.
- `poll_status_with_timeout`: Repeatedly polls a global status byte up to eighty-five times, clearing a word in a table indexed by a value from the caller's stack frame if a specific bit is set, and returns a fixed non-zero value if the loop completes without success.
- `read_port_into_status_byte`: It calls another routine to obtain a byte, clears bit one of that byte when a secondary value from the same call is non-negative, and then forces bit zero high before storing the result into a global variable.
- `update_ciab_mask`: Calls another routine and stores its return value into a global variable.
- `compute_ciab_mask_from_table`: It derives a bit mask from a global variable by clearing one bit chosen by a frame-relative index, and also clears a second bit when a related table entry has its low bit set.
- `wait_ciab_timer`: Waits until a global variable's low bit clears, then waits until another routine stops returning one, using the CIAB timer channel A to pace the busy-wait loops.
- `test_ciab_bit`: Tests a bit in the CIAB register and performs conditional branches and a decrement on a register.
- `set_ciab_timer`: It writes three constant values to a global variable in sequence.
- `call_unpack_resource_chunk`: Calls a routine that parses a structured data block to extract and decompress embedded data.
- `unpack_compressed_resource_format`: Validates a custom data structure by checking for specific magic markers and copies a header block to a destination buffer.
- `copy_variable_length_data`: Iterates through a buffer to copy or move data based on signed byte values and a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_complex_conditional_logic`: This routine performs a series of conditional checks on a global variable and executes another routine twice before entering a loop.
- `execute_sequential_tasks`: Performs a series of conditional checks on a global variable and executes a sequence of subroutines, including data merging and memory clearing operations.
- `execute_conditional_loop`: Checks a global variable against specific bit patterns and characters before executing a subroutine twice and entering a loop that calls another routine sixteen times.
- `run_conditional_loop`: Checks a global variable against specific bit patterns and characters before executing a subroutine twice and entering a loop that calls another routine multiple times.
- `execute_fixed_loop`: Verifies a global variable's status and then executes a subroutine twice followed by a loop that runs for fifteen iterations.
- `run_variable_loop`: Verifies a global variable's status and then executes a subroutine twice followed by a loop that runs for a fixed number of iterations.
- `call_other_routine_twice`: This routine calls another routine two times.
- `validate_and_update_global_variable`: Performs conditional checks on a global variable based on specific bit patterns and values, returning early if certain conditions are met.
- `process_string_data`: Iterates through a buffer, performing conditional indirect calls and data merging based on the contents of the buffer and a global variable.
- `execute_indirect_routine`: This routine calls another routine using a pointer stored in a register.
- `merge_data_into_tables`: It iterates through a source table and merges five 16-bit values into specific offsets of a destination table using bitwise operations.
- `copy_data_to_tables`: It iterates through a source table and copies five 16-bit values into specific offsets of a destination table.
- `copy_data_from_tables`: It iterates through a source table and copies five 16-bit values from specific offsets of a destination table into the source.
- `clear_memory_block`: It clears a large block of memory by writing zero to successive 32-bit words in a loop.
- `execute_infinite_loop_2`: The routine enters an unconditional infinite loop, preventing further execution of the calling thread.
