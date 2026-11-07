# Game 2061 (speedball 2 - brutal deluxe (1990)(imageworks)[cr dc]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02061). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (912080 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 117 of 117 named (0 author symbols), 117 of 117 described (4 by rule, 111 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1231 of 1231 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1024 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2355 of 2355 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 960 |
| code | 10364 |
| data | 900756 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr DC].adf:bootblock | 1 | 81 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 460 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 520 | 29 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 921 | 51 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 361 | 15 |
| Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr DC].adf bytes 1024-2048 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- speedball 2 - brutal deluxe (1990)(imageworks)[cr dc]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr DC].adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 131 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 382 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f150): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 385 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 332 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- speedball 2 - brutal deluxe (1990)(imageworks)[cr dc]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr DC].adf: bytes 1024-2048 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying code at an address (6 bytes change between runtime segment an address (f150) and runtime segment an address): both forms recovered; writer: runtime segment an address (f150) +0, runtime segment an address +6
- speedball 2 - brutal deluxe (1990)(imageworks)[cr dc]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr DC].adf: 1 logged loads cover 1024 bytes of the image
- speedball 2 - brutal deluxe (1990)(imageworks)[cr dc]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr DC].adf: 3 track-loaded blocks found in recovered runtime segments and classified through them
- speedball 2 - brutal deluxe (1990)(imageworks)[cr dc]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr DC].adf: blocks 7-1759 (bytes 3584-901120) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.47)
- 1 never-loaded region(s), 1024 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `init_and_poll_view` (runtime segment an address): Initializes a buffer by clearing two words and then, if a specific byte matches a value, calls another routine. Kind: disk track/hardware validation loop. Pass path: the loop continues (sVar2 decrementing) as long as bVar3 is false and sVar2 > -1.
- `set_audio_dma_control` (runtime segment an address): The routine writes to a global variable, another global variable, a custom chip register, a custom chip register, a global variable, and a global variable. Kind: disk hardware/track validation. Pass path: bne.b an address (for sub_0722 @ 788) or bne.b an address (for sub_08E4 @ 8EE).
- `set_global_flags` (runtime segment an address): It writes two constants to the same global variable in sequence, leaving the second value in place. Kind: disk hardware/parameter initialization. Pass path: the instruction immediately following the call (e.g., moveq #an address, d4).
- `write_hardware_register_and_branch` (runtime segment an address): Writes a long value to a hardware register, tests a value from a frame-relative offset, and branches based on the result. Kind: disk hardware status/sync check. Pass path: beq.b an address.
- `set_bit_in_global_variable` (runtime segment an address): It reads a 16-bit value from a table pointed to by a register, adds a constant, and masks the result to 5 bits to use as a bit index. Kind: bitmask-based integrity or hardware state check. Pass path: the bit remains set or the subsequent logic proceeds without a crash/reset.
- `set_flag_and_call_twice` (runtime segment an address): It stores a constant byte into a global variable, then calls another routine twice in sequence. Kind: checksum or hardware validation trap. Pass path: the instruction following the second call (the code path that does NOT branch away).
- `read_port_into_status_byte` (runtime segment an address): It calls another routine to obtain a byte, clears bit one of that byte when a secondary value from the same call is non-negative, and then forces bit zero high before storing the result into a global variable. Kind: disk hardware/status check. Pass path: extraout_D1b >= 0.
- `call_and_return_result` (runtime segment an address): Calls another routine and returns its result to the caller. Kind: disk hardware/track check. Pass path: d0 = 0 (or specific success code).
- `compute_ciab_mask_from_table` (runtime segment an address): It derives a bit mask from a global variable by clearing one bit chosen by a frame-relative index, and also clears a second bit when a related table entry has its low bit set. Kind: disk hardware/CIAB_PRB check. Pass path: the branch where the bitmask is not further modified by the conditional bit-clear (the 'if' condition evaluates to false).

## Functions


### Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr DC].adf:bootblock

- `initialize_boot_and_copy_self_2`: The routine stores constants into a global variable and RAM locations, tests bits in a global variable and conditionally calls library routines.
- `initialize_boot_and_copy_self`: The routine stores constants into a global variable and RAM locations, tests bits in a global variable and conditionally calls library routines, writes to COLOR00 and DMACON hardware registers, copies a block of its own code to chip memory, and returns the result of an indirect call.
- `write_bootblock_constants`: The routine stores a series of constant values into a global variable and into RAM locations, tests bits in a global variable and conditionally stores further constants, then returns without using the input register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_data_structures`: Populates various data structures through memory copies, clears a small table, and enters an infinite loop after performing conditional assignments to a global variable.
- `initialize_game_data_tables`: Performs a series of large memory block copies to populate various data structures and tables from source locations to destination offsets.
- `copy_data_to_global_offset`: It copies a block of data from a source to a fixed destination, and then copies another block of data to a destination calculated by adding a global variable to a base address.
- `process_conditional_data`: Checks a global variable to decide between executing a sequence of subroutines or copying a block of memory, then clears a small table and updates a global pointer.
- `process_conditional_data_2`: Checks a global variable to decide between executing a sequence of subroutines or copying a block of memory, then clears a small table.
- `copy_data_to_offset_block`: It copies a large block of data to a fixed destination and then copies a second block of data to a destination offset by a global variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_global_variable_loop`: Repeatedly assigns a value from a register to a global variable while calling another routine, terminating when the register value is negative or the called routine returns zero.
- `assign_constant_sequence`: Stores a sequence of large constant values into a global variable and then calls another routine.
- `populate_scaled_table`: Fills a table with values calculated by multiplying an incrementing index by a constant, repeating this process several times.
- `populate_nested_scaled_tables`: Fills a series of tables by multiplying an incrementing index by a constant.
- `tiny_addq_addq_rts`: Small routine of 3 instructions that adds 2 to a7, then adds 2 to (a7), then returns.
- `load_address_and_branch`: The routine loads a word into a register, moves a constant into another register, and performs a relative branch.
- `average_structure_fields`: Iterates through a list of structures to calculate the average of two word fields and stores the result in a third field.
- `nested_loop_with_calls`: It executes a series of nested loops that perform bitwise operations and call another routine multiple times.
- `process_bit_depth_table`: It iterates through a table, performing bitwise checks and subtractions on values based on their bit patterns, while calling another routine.
- `wait_for_non_zero`: Enters a loop that waits for a global variable to become zero before setting it to a specific value.
- `execute_infinite_loop_2`: The routine is empty as it consists only of an infinite loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_padding_sequence`: This routine is empty and performs no operations.
- `poll_interrupt_status`: Enters a nested loop structure that checks specific bits in a memory location and calls another routine if corresponding bits are set in a different memory location.
- `update_table_entry`: Calculates an offset into a table based on input values, updates a byte, and either performs a series of conditional data merges or inserts a new entry into a list.
- `update_coordinate_data`: Updates 32-bit values and 16-bit values in various memory locations based on conditional checks of a structure.
- `conditionally_call_routine`: Checks a global word value and calls another routine if that value is zero.
- `load_and_call_pointers`: The routine loads four consecutive memory addresses into a register and executes a subroutine after the first three loads.
- `update_hardware_configuration`: Updates several fields in a structure and modifies hardware registers related to interrupts and DMA based on conditional logic.
- `load_addresses_and_call`: Loads three different addresses into a register and calls another routine after each load.
- `check_and_decrement_byte`: The routine tests a long value, branches if zero, moves a long value to a register, and decrements a byte in a structure before calling another routine.
- `process_state_transitions`: Manages complex state changes within a structure by updating various word and byte fields based on comparisons with a table of values.
- `initialize_data_structure`: Clears a block of memory, updates several fields within a structure using pointers and offsets, and modifies DMA and interrupt enable registers.
- `update_game_state_and_hardware`: Updates hardware registers for interrupts and DMA while managing state counters and structure fields.
- `store_byte_and_branch`: Loads a byte from a pointer into a register, stores it at an offset from a pointer, and branches to another location.
- `load_byte_and_branch`: Loads a byte from a pointer into a register and branches to another location.
- `calculate_multiplied_value`: Loads a byte from a pointer, multiplies it by a word from a pointer, and stores the result at an offset from another pointer before branching.
- `calculate_division_result`: Loads a byte from a pointer, divides a constant by that byte, and stores the result in a location pointed to by a register before branching.
- `calculate_summed_product`: Iterates through a list of bytes to calculate a sum, multiplies that sum by a value from a pointer, and stores the result at an offset from another pointer.
- `parse_and_initialize_entity_data`: Reads a data stream to populate an entity structure, including copying values from a table and initializing several internal counters and state flags.
- `fetch_byte_and_branch`: Loads a byte from a memory location pointed to by a register into a data register and performs a short relative branch.
- `process_event_queues`: Iterates through two separate decrementing counters to process queued events or delayed actions.
- `fetch_and_lookup_value`: Pops a byte from a pointer into a register, doubles it twice, and uses the result to fetch a longword from a table located at a fixed memory address.
- `pop_pointer_and_branch`: Loads a longword from a pointer into a register and increments that pointer before branching to another location.
- `add_and_branch_offset`: Fetches a byte from a pointer, conditionally extends it to a word, adds it to a value at an offset from a pointer, and branches.
- `store_word_and_branch`: Stores a word from a register into a memory location using an offset from a pointer and then branches to another location.
- `load_entity_attribute_data`: Retrieves entity data from a table using an index and populates a structure with specific attributes and pointers.
- `assign_global_pointer`: Assigns the address of a global variable to a pointer located at an offset within a structure.
- `fetch_and_branch_table_entry`: Fetches a byte from a pointer, performs arithmetic on it, and uses it to index into a table to load a new pointer before branching.
- `copy_word_field_triples`: Copies two word fields to two other offsets in a structure, advancing the base pointer two bytes per pass, and repeats for three passes.
- `copy_word_field_pairs`: Copies two word fields to two other offsets in a structure, advancing the base pointer two bytes per pass, and repeats for two passes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_system_state`: Modifies several global long and word values, updates the status register, and checks a bit in a hardware register before conditionally moving a value to a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_short`: Performs a short relative branch.
- `modify_global_and_call`: Performs a bitwise XOR on a global variable and calls another routine if a specific bit in a global variable is set.
- `tiny_ori`: Small routine of 1 instruction that merges 28 into d6.
- `update_joystick_status`: Reads input from joystick and mouse hardware registers and updates a global variable with a masked version of the input.
- `return_empty_routine`: This routine is empty and performs no operations before returning.
- `distribute_masked_word_data`: Iterates through a source buffer to copy words to a destination while simultaneously storing a bit-masked version of each word into three separate memory locations.
- `copy_and_mask_words`: Copies words from a source to a destination while simultaneously storing a masked version of each word into three different memory locations.
- `write_multiple_word_values`: If a value is non-negative, it writes that value to a specific offset in a structure and then performs multiple consecutive writes of the same value to another offset in a global structure.
- `swap_global_values`: Swaps the contents of a global variable with a temporary value based on conditional checks of the variable's sign.
- `check_memory_and_call_division`: Loops until specific values are found in memory, then calls a routine to perform a scaled division and returns a value.
- `reset_global_variable`: Sets a global variable to zero and then calls another routine.
- `call_scaled_division`: Calls a routine to perform a scaled division calculation.
- `calculate_scaled_division`: Performs a division-like calculation using a loop and arithmetic operations to determine a value based on input parameters.
- `process_dma_validation_loop`: Runs a fixed loop of four iterations, clearing a small block of fields and calling helper routines to validate and repack a 32-bit result.
- `init_and_poll_view`: Initializes a buffer by clearing two words and then, if a specific byte matches a value, calls another routine.
- `render_park_view`: The routine writes a constant to a memory location derived from a pointer, then repeatedly calls other routines to update a flag, toggle a flag, and compute a checksum using a decrementing counter.
- `validate_input_with_retry`: Repeatedly calls helper functions to check a flag and decrement a counter, returning early if the flag is clear or the counter expires.
- `process_object_state`: Performs complex conditional checks on structure fields and hardware bits, updates object transforms, and writes constant values to a global variable.
- `empty_stub_routine`: It performs no observable work and simply returns.
- `set_flag_and_increment_counter`: It sets a bit in a 16-bit word located a few bytes before a pointer in A6, using a bit index derived from D3.
- `clear_table_entries`: It clears a table by writing zero to successive entries.
- `repack_result_word_and_byte`: Calls a routine and repacks its 32-bit result, placing the low word first and the top byte of the result in the second word.
- `interleave_word_bits`: It reads two consecutive words from a pointer and returns a value formed from their low bits.
- `process_and_pass_value`: It calls a routine to produce a 32-bit value, passes that value to another routine, and then returns.
- `load_pointer_and_count`: It loads a value from an offset of a pointer into a register and sets a second register to a constant value.
- `compute_xor_checksum`: It iterates through a sequence of words pointed to by an input pointer, XORing each into an accumulator.
- `fill_memory_pattern`: The routine clears a block of memory by writing a repeated 32-bit pattern to each word.
- `initialize_object_state`: Calls another routine and uses its result to compute three integers stored at fixed offsets in a structure, then writes two constant words and a derived word into it.
- `set_object_transforms`: Writes several fields of a structure from the results of unnamed routines, storing constant words and copying values between the results, and calls a routine that waits for the blitter to finish.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `call_then_store_constants`: It calls another routine, then stores a sequence of constant values into global variables before returning.
- `shift_and_double_call`: Pushes a longword to the stack, performs a logical right shift on it, calls another routine, restores the original longword from the stack, and calls that same routine again.
- `clear_flag_byte`: Reads a byte pointed to by a pointer and a flag from the preceding byte.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `set_audio_dma_control`: The routine writes to a global variable, another global variable, a custom chip register, a custom chip register, a global variable, and a global variable.
- `check_hardware_and_call`: Loads a hardware register address, calls another routine, tests a bit in a hardware register, and conditionally calls another routine.
- `set_global_flags`: It writes two constants to the same global variable in sequence, leaving the second value in place.
- `write_hardware_register_and_branch`: Writes a long value to a hardware register, tests a value from a frame-relative offset, and branches based on the result.
- `set_bit_in_global_variable`: It reads a 16-bit value from a table pointed to by a register, adds a constant, and masks the result to 5 bits to use as a bit index.
- `set_flag_and_call_twice`: It stores a constant byte into a global variable, then calls another routine twice in sequence.
- `update_table_entry_2`: Reads a 16-bit value from a table via an index in a structure, calls another routine when the value is negative, then spins a number of times derived from the difference between two 16-bit fields before writing a value from a register back into the table and calling another routine.
- `poll_status_with_timeout`: Repeatedly polls a global status byte up to eighty-five times, clearing a word in a table indexed by a value from the caller's stack frame if a specific bit is set, and returns a fixed non-zero value if the loop completes without success.
- `read_port_into_status_byte`: It calls another routine to obtain a byte, clears bit one of that byte when a secondary value from the same call is non-negative, and then forces bit zero high before storing the result into a global variable.
- `call_and_return_result`: Calls another routine and returns its result to the caller.
- `compute_ciab_mask_from_table`: It derives a bit mask from a global variable by clearing one bit chosen by a frame-relative index, and also clears a second bit when a related table entry has its low bit set.
- `wait_ciab_timer`: Waits until a global variable's low bit clears, then waits until another routine stops returning one, using the CIAB timer channel A to pace the busy-wait loops.
- `test_hardware_bit_and_decrement`: Tests a bit in a hardware register and branches, then decrements a register and branches again based on the zero flag.
- `set_ciab_timer`: It writes three constant values to a global variable in sequence.
- `call_subroutine`: The routine calls another subroutine and then returns.
- `unpack_resource_chunk`: Parses a structured data block by identifying specific magic markers to locate and copy sub-sections.
- `process_string_data`: Iterates through a buffer, performing conditional indirect calls and data merging based on the contents of the buffer and a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_complex_conditional_logic`: This routine performs a series of conditional checks on a global variable and executes another routine twice before entering a loop.
- `execute_sequential_tasks`: Performs a series of conditional checks on a global variable and executes a sequence of subroutines, including data merging and memory clearing operations.
- `execute_conditional_sequence`: Executes a series of subroutines and enters a loop that runs sixteen times if a global variable meets specific bitwise and character criteria.
- `run_conditional_loop`: Checks a global variable against specific bit patterns and characters before executing a subroutine twice and entering a loop that calls another routine multiple times.
- `execute_variable_sequence`: Executes a series of subroutines and enters a loop that runs fifteen times if a global variable meets specific bitwise and character criteria.
- `run_variable_loop`: Verifies a global variable's status and then executes a subroutine twice followed by a loop that runs for a fixed number of iterations.
- `call_other_routine_twice`: This routine calls another routine two times.
- `validate_and_reset_global_variable`: It performs conditional checks on a global variable based on specific bit patterns and values, returning early if certain criteria are met.
- `scan_text_with_recursive_dispatch`: Walks a zero-terminated byte buffer, skipping bytes below the 'A' threshold and, for each byte at or above it, repeatedly invokes the routine itself (five times, plus once more when a flag bit of the global variable is set), with the loop continuing only while the global variable's bit conditions hold.
- `execute_indirect_routine`: This routine calls another routine using a pointer stored in a register.
- `merge_data_into_tables`: It iterates through a source table and merges five 16-bit values into specific offsets of a destination table using bitwise operations.
- `copy_data_to_tables`: It iterates through a source table and copies five 16-bit values into specific offsets of a destination table.
- `copy_data_from_tables`: It iterates through a source table and copies five 16-bit values from specific offsets of a destination table into the source.
- `clear_memory_block`: It clears a large block of memory by writing zero to successive 32-bit words in a loop.
- `execute_infinite_loop`: The routine enters an unconditional infinite loop, preventing further execution of the calling thread.
