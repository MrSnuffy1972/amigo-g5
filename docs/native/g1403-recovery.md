# Game 1403 (bug bomber (1991)(kingsoft)[cr ip][t +8 legend]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01403). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (918944 top-level bytes in 19 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 331 of 331 named (0 author symbols), 331 of 331 described (56 by rule, 268 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1168 of 1168 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 346014 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4824 of 4824 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 113918 |
| code | 17928 |
| data | 402537 |
| padding | 3633 |
| unused | 380928 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Bug Bomber (1991)(Kingsoft)[cr IP][t +8 Legend].adf:bugbomber | 1 | 116 | 5 |
| disk-validator | 1 | 386 | 1 |
| Bug Bomber (1991)(Kingsoft)[cr IP][t +8 Legend].adf:IP.txt | 2 | 24 | 4 |
| Bug Bomber (1991)(Kingsoft)[cr IP][t +8 Legend].adf:-l-Bug | 3 | 938 | 78 |
| Bug Bomber (1991)(Kingsoft)[cr IP][t +8 Legend].adf:ip! | 2 | 116 | 3 |
| Bug Bomber (1991)(Kingsoft)[cr IP][t +8 Legend].adf:iconx | 12 | 806 | 37 |
| Bug Bomber (1991)(Kingsoft)[cr IP][t +8 Legend].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1117 | 56 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 102 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1183 | 131 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Bug Bomber (1991)(Kingsoft)[cr IP][t +8 Legend].adf:mainpart (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 3 64 KB pages each with >= 29491 distinct PCs executed, 49152 of its 66777 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 3 64 KB pages each with >= 29491 distinct PCs executed, 79211 of its 96850 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 167 64 KB pages each with >= 29491 distinct PCs executed, 4277312 of its 4410519 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 102 64 KB pages each with >= 29491 distinct PCs executed, 1772741 of its 1794741 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference crashes at an address (HLE illegal instruction an address at an address: through vector 0, as before (vector 4 is 0 again)): 2658 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address (play-rs-m1500): 315 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 27 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 636 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- bug bomber (1991)(kingsoft)[cr ip][t +8 legend]/Bug Bomber (1991)(Kingsoft)[cr IP][t +8 Legend].adf:ip!: Time Cruncher 1.7 (xfd) stream found by ancient scan (1324 -> 2048 bytes) but only 1 of 1 sampled windows are in reference RAM: not used
- bug bomber (1991)(kingsoft)[cr ip][t +8 legend]/Bug Bomber (1991)(Kingsoft)[cr IP][t +8 Legend].adf:mainpart: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 5 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 branch displacement, 1 address, 1 immediate+address, 1 immediate
- 1 never-loaded region(s), 346014 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `modify_memory_and_flags` (-l-Bug): Sets bits in a global variable and writes specific values to memory locations using an offset. Kind: checksum/integrity trap. Pass path: the routine continues execution normally after the bsr calls, implying the 'modification' of memory/flags is a setup for a subsequent validation check or a way to hook a handler..
- `perform_memory_xor` (runtime segment an address (play-rs-m1500)): Calculates a character value from a global variable and performs a bitwise XOR operation on several non-contiguous memory locations. Kind: checksum trap. Pass path: the routine continues to the XOR operations which decrypt/validate the next stage of code.
- `update_status_and_memory` (runtime segment an address (play-rs-m1500)): Updates several global variables from a data table, performs a loop to clear a block of memory, and modifies a global variable via a bitwise OR. Kind: disk hardware/status check. Pass path: The routine updates internal state variables (uRam/DAT) based on hardware status; a successful check allows the program to proceed with the loaded data, whereas a failure (implied by the conditional branches and hardware-specific offsets) w.

## Functions


### Bug Bomber (1991)(Kingsoft)[cr IP][t +8 Legend].adf:bugbomber

- `initialize_program_environment`: Opens a file, allocates memory, and copies a block of code to a new location before entering an infinite loop if validation fails.
- `update_palette_color`: Updates a color register by copying and negating a block of data.
- `copy_and_negate_bytes`: Copies a block of data from one location to another and performs a loop that handles negated byte values.
- `copy_and_compare_table`: Negates a byte, performs a loop to copy bytes from one location to another, and then compares a longword at a specific address against a constant before jumping to a fixed address.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### disk-validator

- `format_display_string`: Converts a numeric value into a character representation and stores it in a buffer.

### Bug Bomber (1991)(Kingsoft)[cr IP][t +8 Legend].adf:IP.txt

- `call_library_entry`: Invokes another routine and checks the zero flag.
- `open_dos_library`: Calls the exec library function OldOpenLibrary to load dos.library.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Bug Bomber (1991)(Kingsoft)[cr IP][t +8 Legend].adf:-l-Bug

- `initialize_system_state`: Sets several global variables to specific values and calls another routine eight times.
- `execute_sequence_of_routines`: Calls another routine eight times in succession and then stores a constant value in a global variable.
- `return_data_register_value`: This routine is empty and simply returns the current value held in a data register to the caller.
- `return_data_register`: Returns the current value held in a data register.
- `return_register_value_3`: An empty routine that returns the current value held in a data register.
- `return_register_value_2`: This is an empty routine that returns the current value held in a data register.
- `return_register_value`: An empty routine that returns the value currently in a register.
- `return_uninitialized_byte`: This routine is empty and returns an uninitialized byte value from a register.
- `return_undefined_byte`: This routine is empty and returns an undefined byte value.
- `return_immediately`: This routine is an empty subroutine that performs no operations and immediately returns to its caller.
- `return_empty_subroutine`: This routine performs no operations and immediately returns to its caller.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `perform_bit_rotation`: Performs a bitwise rotation and negation on a value.
- `initialize_memory_tables`: Clears a large block of memory and populates several tables with specific constant values.
- `modify_memory_and_flags`: Sets bits in a global variable and writes specific values to memory locations using an offset.
- `search_and_update_tables`: Searches through a data block for a specific byte value, then uses the result to populate a table with calculated offsets and clears several other memory locations.
- `reset_audio_volumes`: Sets four memory locations to zero and sets a global variable to fifteen.
- `prepare_parameter_pairs`: Loads several pairs of memory addresses into registers and calls another routine multiple times.
- `modify_and_clear_byte`: Checks a byte, calculates a new value by shifting the byte and subtracting it from a constant, stores that value in a new location, and clears the original byte.
- `increment_byte_with_limit`: Increments a byte through a pointer if its value is less than a specific threshold.
- `check_and_replace_byte`: Compares a byte at a specific memory location to a constant and replaces it with a different byte if the comparison fails.
- `add_and_loop_bytes`: Adds a byte to a memory location and enters a loop that modifies a register and increments a pointer based on a comparison.
- `clamp_difference_value`: Calculates the difference between two bytes, clamps the result within a specific range, and stores it in a global variable.
- `negate_and_branch`: Subtracts one byte from another, moves the result to a register, negates it as a word, and performs a relative branch.
- `clamp_signed_difference`: Calculates the difference between two bytes, converts it to a word, and clamps the result within a specific range before storing it in a global variable.
- `negate_subtracted_value`: Subtracts a byte from a register, stores the result in another register, and then negates that register as a word.
- `clamp_value_to_range`: Clamps a calculated value within a specific range and stores the result in a global variable.
- `combine_lives_and_flag_bits`: Clears d4, saves d1, reads a byte from absolute address 4868 into d4, reads a byte from 4867 into d1, masks d1 with 4, shifts it left 6, ORs it into d4, restores d1 and returns.
- `extract_bits_to_global`: Extracts a byte and a specific bit from a passed longword and stores them into a global variable.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `update_global_bit`: Extracts a specific bit from a register and stores the result into a global variable.
- `recurse_and_copy_data`: Recursively calls itself based on a condition and performs nested loops to copy multiple data blocks from a source table to a destination buffer.
- `check_limit_and_recurse`: Compares a value in a register against a constant and calls itself if the condition is met.
- `copy_data_blocks`: Performs a nested loop to copy multiple blocks of data from a source table to a destination buffer using calculated offsets.
- `toggle_table_bits`: Iterates through a table of structures to find a specific entry based on a series of conditional checks and then performs a bitwise NOT operation on a byte at a calculated offset within that entry.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `check_unlimited_lives`: Compares a value from a hardware register against a range of values stored in global variables.
- `wait_for_input`: Polls a global variable in a loop until a specific bit is set, effectively waiting for a hardware status change.
- `initialize_data_block`: Executes a loop twice to write several constant values and register contents into a block of memory at various offsets.
- `wait_for_bit`: Enters a loop that continues as long as a specific bit in a byte at a memory offset is set.
- `process_input_and_update`: Checks a hardware register, manages counters, and updates various memory locations and structure fields based on a character comparison.
- `search_character_table`: Increments a long value and iterates through a table to compare a byte against a character set.
- `update_structure_fields`: Repeatedly calls another routine and modifies multiple fields within a structure, including pointers and longwords.
- `decrement_and_branch`: Decrements a word in a memory location and branches if it is not zero, otherwise resets the value and increments a long variable.
- `update_pointer_and_compare`: Increments a long value at a memory location, updates a pointer, and compares a byte at that pointer to a constant before branching.
- `process_byte_stream`: Iterates through a sequence of bytes, adding each byte to a counter and storing the result into a memory block pointed to by a register.
- `load_pointer_and_branch`: Loads a long pointer into a register and performs a short branch to another routine.
- `write_conditional_bytes`: Loops through a block of memory, writing specific byte values to three different offsets within each iteration based on the value of the current byte.
- `write_constant_values`: Writes a byte value to a memory location pointed to by a register, writes the same byte to an offset of that pointer, writes a different byte to another offset, and branches to another routine.
- `write_byte_sequence`: Loads a byte into a register and writes it to two consecutive memory locations via a pointer.
- `update_data_structures`: Updates several global variables and performs a loop that populates a structure with values from a pointer and a table.
- `update_pointer_and_check_byte`: Loads a pointer from a global variable, checks the byte at that pointer, and either branches based on the sign or performs a series of pointer updates and global variable writes.
- `store_buffer_data`: Stores three consecutive bytes from a source pointer into three separate global variables and increments a fourth global variable.
- `initialize_data_table`: Fills a large memory area with a repeating pattern of short integers using nested loops.
- `modify_short_array`: Iterates through an array of short integers and applies a mathematical transformation to each element.
- `increment_and_check_limit`: Increments a long value at a specific address, compares it to a constant, and clears the value if the comparison fails.
- `update_variable_values`: Modifies various global variables based on the contents of a byte and a word through conditional logic and nibble extraction.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `branch_on_nibble`: Extracts a nibble from a byte and performs conditional branches based on its value.
- `slide_channel_value_down`: Subtracts the record's parameter byte from its running value, clamps the result to a lower limit, and copies it to the output field.
- `slide_channel_value_up`: Adds a parameter byte to a running value, clamps the result to an upper limit, and copies it to an output field.
- `clamp_incremented_value`: Extracts the high nibble of a byte from the stack, adds it to a value at an offset, clamps the result to a maximum of sixty-four, and stores the result in another location.
- `decrement_counter_clamped_zero`: Subtracts the low nibble of a parameter byte from a 16-bit counter, clamps the counter at zero, and copies it to an output field.
- `branch_on_constant`: Compares a word loaded from a relative offset against several constant values and branches to different locations based on the result.
- `fetch_table_entry_high`: Searches a table for a matching word and stores a word from the table at an offset determined by the high nibble of a byte.
- `fetch_table_entry_low`: Searches a table for a matching word and stores a word from the table at an offset determined by the low nibble of a byte.
- `copy_word_to_offset_2`: Copies a 16-bit word from one memory location to another location at a specific offset.
- `fetch_table_entry`: Calculates an index using a value from the stack, loads a word from a table using that index, and compares it to another value.
- `copy_word_to_offset`: Stores a 16-bit value from a register into a memory location at a specific offset.
- `configure_audio_and_dma`: Updates audio channel registers and enables DMA channels by writing to the DMA controller register.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `parse_command_byte_data`: Extracts a command identifier from a data structure to perform table lookups and update state variables.
- `process_state_command_byte`: Evaluates a command byte from a data structure to update a global state word, a counter, or a bitmask.
- `negate_state_and_store_count`: Bitwise-inverts a state word and stores the step's parameter byte minus one into a counter.
- `store_byte_to_offset`: Stores a byte from a memory location into another memory location at a specific offset.
- `invert_state_word`: Performs a bitwise NOT operation on a global word.
- `set_state_bit_from_param`: Copies bit zero of a parameter byte into bit one of a global word while preserving the other bits.
- `reset_counter_if_nonzero`: If the low nibble of a parameter byte is non-zero, it clears a global word and stores that nibble into another global variable.

### Bug Bomber (1991)(Kingsoft)[cr IP][t +8 Legend].adf:ip!

- `expand_bitstream_to_memory`: Reads a bitstream from a source table and expands it by shifting and masking bits to populate a destination memory buffer.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_right_bits_with_early_exit`: The routine shifts a 32-bit value right by one bit repeatedly, loading a new 32-bit word from a table pointed to by a pointer each time the shifted value becomes zero, and stops early if a counter reaches -1.

### Bug Bomber (1991)(Kingsoft)[cr IP][t +8 Legend].adf:iconx

- `initialize_execution_environment`: Prepares the application state by retrieving system pointers, allocating memory, and parsing command-line arguments.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start.
- `build_wait_and_endcli_command`: Builds a command string from a template and the given name, including a wait command with a long timeout and an endcli redirected to NIL, after checking whether the name starts with a dot.
- `install_library_functions`: Creates and installs a library from a descriptor: it makes the library structure, sets up its function table, and returns the result.
- `scan_ram_entries_for_names`: Allocates a buffer and walks a linked list of entries, copying the entries whose name bytes are above a specific control range into the buffer and passing them to helper routines.
- `concatenate_strings_and_call_helper`: Copies a source string into a destination buffer, appends a second string to that buffer, and then passes the resulting buffer and the total length to another routine.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the terminating zero of a string, then passes that position and a second argument to a helper routine.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_to_address`: Stores a single byte from a register into a memory location pointed to by another register.

### Bug Bomber (1991)(Kingsoft)[cr IP][t +8 Legend].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_ori_rts`: Small routine of 2 instructions that merges 48 into d3, then returns.
- `execute_conditional_loops`: Executes a sequence of calls and loops that continue based on the return value of another routine.
- `copy_and_clear_data_blocks`: Copies a series of data blocks from a source pointer to a destination address, then performs a second pass to zero out a subsequent block of memory.
- `initialize_level_parameter_tables`: Clears specific memory regions and populates a series of data structures with varying configuration values based on a global state.
- `update_memory_bytes`: Iterates through a nested structure in memory to replace specific byte values with a new value.
- `set_global_flags`: Sets a global variable to several different constant values through a series of conditional checks and calls to another routine.
- `process_recursive_flags`: Reads a bitmask from a global variable and performs recursive calls based on which bits are set.
- `update_blitter_from_flags`: Checks bits 6 through 11 of a register and, for each set bit, calculates a destination address and size using values from a structure in a0 and a table, then calls update_blitter_parameters.
- `execute_recursive_calls`: The routine calls itself three times in succession before returning.
- `update_blitter_parameters`: Calculates values based on table lookups and register inputs, then updates several hardware registers and memory locations, including bitplane and area parameters.
- `set_blitter_offset`: Updates several hardware registers and memory locations with a calculated offset and specific bitplane parameters.
- `set_blitter_source`: Updates hardware registers and memory locations using a provided pointer and a calculated offset.
- `update_blitter_render_state`: Calculates a combined value from two memory tables and updates a structure via a pointer.
- `set_blitter_parameters`: Writes values to a pointer and updates several blitter-related hardware registers and memory locations.
- `update_entities_on_vblank`: Iterates through an entity list, checking if an entity's vertical position plus 80 exceeds the current vertical scanline; if the vertical sync bit is set or the position condition is met, it calls update_entity_hardware_parameters and decrements a global counter until the counter reaches zero.
- `validate_entity_list_pointer`: Checks if the 16-bit value in d0 is equal to 65535; if so, it branches to another routine, otherwise it increments the pointer in a0 by 8 and branches to another routine.
- `process_entity_vertical_sync`: Iterates through a table of 16-bit values, comparing each value plus 50 to the vertical scanline position; if the condition is met, it calls another routine to update hardware parameters and decrements a global counter, repeating this process until the counter reaches zero or the vertical blanking bit is set.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `process_entity_list_loop`: Loads a 16-bit value from a pointer in a0 into d0; if the value is negative, it decrements a global variable and branches to another routine, otherwise it calls update_entity_hardware_parameters, decrements a global variable, and loops back to itself if the global variable is not zero.
- `validate_and_increment_entity_pointer`: Compares a 16-bit value in register d0 to negative one; if it is not negative one, it increments the pointer in a0 by eight bytes and branches to another routine, otherwise it branches to a different location.
- `load_entity_pointer`: Loads the address of a table into a register and branches to another routine.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `wait_for_vposr_range`: Enters a loop that waits for a global variable to fall within a specific range relative to the vertical position register.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `update_entity_hardware_parameters`: Writes specific values to a memory structure and waits for a status bit to clear before updating several fields within that structure.
- `wait_and_update_entity_parameters`: Loops until bit 6 of a byte at offset 2 of the structure in a6 is cleared, then stores d0 and d1 into offsets 84 and 80, and writes constants 24320 to offset 100, 3521 to offset 88, and 5104 to offset 64 of the structure in a6 before returning.
- `initialize_memory_buffer`: Copies a byte from one memory location to another while zeroing out subsequent bytes in a loop.
- `execute_blitter_sequence`: Performs a nested loop that calls routines to update blitter source and offset parameters based on values in a data table.
- `handle_blitter_command_byte`: Compares a byte in register d5 to one and, if it is not equal to one, branches to another location; otherwise, it calls another routine and then branches to a subsequent instruction.
- `process_blitter_command_loop`: Calculates a value from a byte in register d5, calls another routine, and increments two counters, returning when the first counter reaches twenty and the second counter reaches thirteen.
- `toggle_coprocessor_dma_state`: Updates coprocessor jump and latch registers before enabling DMA channels.
- `wait_for_condition_and_set_flag`: Enters a loop that decrements a counter while checking a bitmask against a global variable.
- `decrement_counter_loop`: Decrements a local variable through a loop until it reaches zero.
- `process_recursive_value`: Decrements a global variable through a loop and recursively calls itself if the variable is non-zero.
- `set_global_constants`: Assigns specific constant values to a global variable.
- `process_input_and_update_state`: Reads input data from a table and updates several global variables based on specific bitwise conditions and button states.
- `process_audio_bitmask`: Iterates through bits in a word, performs actions based on specific bit settings, and decrements several global counters.
- `update_audio_channel_pointers`: Checks a global variable before writing values to the four audio channel low-address registers.
- `check_audio_initialization_status`: Tests a byte in a global variable and branches to another routine if the byte is non-zero.
- `initialize_audio_pointer`: Tests a byte in a global variable and, if non-zero, loads a constant into a register and sets up a pointer to a global variable.
- `set_audio_channel_three_low_pointer`: Loads a pointer to the third audio channel low register into register a1, loads the address of a global variable into register a3, and sets register d4 to three.
- `reset_global_bits`: Checks specific bits of a global variable and sets the variable to one or zero based on those checks.
- `calculate_vertical_position_offset`: Reads the vertical display position from the custom chip, adds the value from a hardware register, and stores the resulting byte into a global variable after applying a constant offset.
- `perform_memory_xor`: Calculates a character value from a global variable and performs a bitwise XOR operation on several non-contiguous memory locations.
- `xor_memory_blocks`: Calculates an offset based on a global variable and performs a bitwise XOR operation on sixteen consecutive 32-bit memory locations.
- `initialize_and_process_lookup_tables`: Clears a block of memory and populates a table using values from a global array.
- `process_lookup_table_indices`: Calculates an offset into a table using a value from a global variable, performs a bitwise XOR operation, and then iterates through a secondary table to increment values at specific indices based on comparison results.
- `jump_to_validate_and_update_table_entr`: Jump stub: passes control straight on to validate_and_update_table_entry.
- `validate_and_update_table_entry`: Calculates an offset into a table by multiplying a value from a pointer in a0 by a constant, then calls return_bitwise_xor to generate a value used to index into a table at a fixed address.
- `process_table_entry_with_validation`: Decrements a word from a pointer in a0 and, if the result is non-negative, calls another routine and checks a byte in a table at offset 27744; if that byte is non-zero, it adds 121 to d0 and branches.
- `validate_and_modify_table_entry`: Loads a table pointer, decrements a count from a word in a0, and if the count is not zero, calls another routine and then checks a byte in the table at an offset provided by d0; if that byte is not zero, it adds 121 to d0 and branches to another location, otherwise it clears d1 and returns.
- `clear_table_byte_at_offset`: Writes the value two to a memory location determined by adding a register to a pointer, then iterates through a range of offsets in a global table to clear specific bytes if they match the value three.
- `nop_padding_sequence_2`: The routine is empty and performs no operations.
- `return_bitwise_xor`: Returns a value constructed from the input registers and the result of a bitwise XOR operation on a global variable with itself.
- `execute_countdown_loop`: Decrements a register or variable from a large constant value down to zero.
- `initialize_data_tables`: Populates two separate memory tables with calculated integer values using loops.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_system_sequence`: Calls a routine that clears a memory block and executes a series of function calls via pointers and a null-terminated string.
- `execute_system_sequence`: Clears a block of memory, performs several function calls via pointers, and iterates through a null-terminated string to execute further calls.
- `execute_branch_sequence`: Calls another routine and then performs a short branch to a different location.
- `write_memory_constants_2`: Checks a value at a pointer, copies two words to specific memory locations if the value is not zero, and increments the pointer.
- `set_global_address`: Stores a specific constant value into a global variable.
- `set_global_address_alt`: Stores a different constant value into a global variable.
- `write_memory_constants`: Writes several constant word and byte values to specific memory locations and returns.
- `branch_to_offset`: Performs a short relative branch to a specific instruction offset.
- `execute_string_callbacks`: Iterates through a null-terminated string to execute a sequence of function calls via pointers.
- `store_constant_values`: Stores three specific constant values into three consecutive memory locations.
- `update_global_pointers`: Writes two 16-bit values to consecutive memory locations and jumps to another routine.
- `copy_memory_block`: Copies a block of data from one memory location to another and then calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_subroutine`: Performs a short branch to another routine.
- `reset_state_and_initialize_globals`: Clears several global variables and then performs a series of recursive calls to the same routine.
- `branch_to_offset_5`: Performs a short branch to a specific location.
- `update_object_state_from_command`: Parses a multi-byte command from a register to modify various fields within an object structure in a table.
- `branch_to_address`: Performs a relative branch to another location.
- `reset_hardware_and_globals`: Clears several global variables and resets hardware registers including audio period registers and DMA control.
- `jump_to_reset_system_state`: Jump stub: passes control straight on to reset_system_state.
- `jump_to_update_sprite_properties`: Jump stub: passes control straight on to update_sprite_properties.
- `jump_to_reset_system_state_2`: Jump stub: passes control straight on to reset_system_state.
- `jump_to_compare_and_set_flags`: Jump stub: passes control straight on to compare_and_set_flags.
- `branch_to_offset_13`: Performs a short relative branch to a location at a fixed offset.
- `jump_to_reset_system_state_3`: Jump stub: passes control straight on to reset_system_state.
- `jump_to_table_index_updater`: Performs a short branch to the routine that updates a table index.
- `update_state_structure_2`: Sets a byte in a global variable to one, stores a word from d0 and a long from a1 into a structure pointed to by a5, and clears a long at a specific offset within that structure.
- `update_attribute_structure`: Checks a flag in an attribute table to determine a base pointer, then compares values from a source structure against an existing attribute structure.
- `jump_to_set_global_and_call`: Jump stub: passes control straight on to set_global_and_call.
- `configure_audio_and_animation`: Sets audio period registers based on a bitmask in a structure, calls load_and_call_addresses and decrement_and_loop_animation, and updates audio period values from a table before returning.
- `decrement_and_loop_animation`: Decrements a word in a structure and, if it becomes negative, enters a nested loop that repeatedly calls another routine based on a byte check.
- `tiny_addq`: Small routine of 1 instruction that adds 4 to a0.
- `update_entity_animation_state`: Processes entity animation frames by reading command bytes from a data table to update sprite properties, frame indices, and state attributes.
- `branch_to_offset_3`: Performs a short relative branch to a specific instruction offset within another routine.
- `branch_to_offset_4`: Performs a short relative branch to a specific offset within another routine.
- `advance_entity_animation_state`: Iterates through animation frame data by reading command bytes to update entity properties, sprite indices, and frame counters.
- `branch_to_offset_two_c_zero`: Performs a short branch to a specific offset.
- `branch_to_animation_logic`: Performs a short relative branch to a location within another routine.
- `branch_to_animation_logic_2`: Performs a short relative branch into the middle of another routine.
- `branch_to_animation_routine`: Performs a short relative branch into a different location within the same segment.
- `branch_to_animation_routine_2`: performs a short relative branch into another routine.
- `update_entity_state_logic`: Processes entity state transitions by reading values from a table and updating corresponding structure attributes.
- `sync_entity_animation_state_2`: Copies a 32-bit word from offset 136 to offset 40 and a 16-bit word from offset 168 to offset 104 of the structure pointed to by a5, then branches to another routine.
- `return_immediately_2`: Returns immediately via a short branch.
- `process_entity_animation_state`: Updates an entity's animation frame and sprite properties by reading from a data table and performing a multi-case switch based on the retrieved byte.
- `branch_to_offset_2d6`: Performs a short branch to a specific offset within another routine.
- `update_animation_frame_index_2`: Reads a byte from offset 13 of the structure in a6, masks the lower two bits, multiplies the result by two, and uses it as an index to store a word from offset 14 of the structure into a table located at a fixed memory address.
- `reset_entity_animation_state`: Clears a 16-bit word at offset 26 of the structure pointed to by a6 and sets a byte at offset 72 of the structure pointed to by a0 to 255, then returns.
- `process_entity_animation_frames`: Updates entity state and animation properties by iterating through frame data and applying transformations based on a command byte.
- `reset_animation_state`: Clears a 16-bit word in a structure pointed to by a6 and sets a byte at offset 72 of a structure pointed to by a0, then returns.
- `update_pointer_data_2`: Moves a longword from one location to another, updates a byte at an offset, and fetches a value from a table using a modified index to store back into a pointer.
- `sync_entity_animation_state`: Copies a 32-bit word from offset 136 to offset 40 of the structure pointed to by a5, copies a 16-bit word from offset 168 to offset 104 of the same structure, and branches to another routine.
- `update_entity_direction_and_state_2`: Checks a global flag and, if zero, sets it to one and copies three bytes from the structure in a6 to offsets 29, 30, and 31 of the same structure.
- `sync_entity_state_and_clear_counters`: Copies a byte from offset 29 of the structure in a6 to offset 28 of the same structure, clears the byte at offset 11, clears the word at offset 0 of the structure in a1, and branches to another routine.
- `update_animation_frame_index`: Reads a byte from an offset of 13 in a structure, masks the lower two bits, multiplies the result by two, and uses it as an index to store a 16-bit word from a structure into a table located via a PC-relative address, then branches to another routine.
- `initialize_entity_animation_frame`: Calculates an index using a byte from offset 14 of a structure, then uses that index to select and write a byte from offset 13 and a byte from offset 15 of the same structure into a target structure at offset 72 and 73 respectively.
- `update_entity_attribute_list`: Reads a sequence of eight values from a memory table and updates a corresponding set of state structures.
- `clear_byte_at_offset`: Sets a single byte to zero at a specific offset from a value held in a register.
- `update_sequence_and_load_pointers`: Decrements or resets a counter in a structure, then reads a sequence of eight 16-bit values from a table and uses them as indices to load a series of 32-bit pointers and 8-bit values into a second structure.
- `resolve_entity_index_array`: Updates an index and transfers a word to two structures, then reads a sequence of eight 16-bit values from a table to populate multiple pointer and status fields within a structure.
- `update_entity_animation_frames`: Increments a 16-bit frame counter in a structure in a5 and updates several coordinate and state values by reading from a table indexed by the frame counter and entity properties.
- `update_entity_sprite_attributes`: Increments a 16-bit counter in a structure in a5 and updates several sprite-related fields by reading indices from a table and calculating memory addresses.
- `clear_memory_byte`: Sets a byte to zero at an offset from a value held in a register.
- `frag_tst_beq_bmi_subq_bra`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was zero, then branches if the last result was negative, then subtracts 1 from the memory at an offset from a6, then jumps to another point in the code.
- `reset_and_increment_attribute_counter`: Sets a 16-bit value at offset 34 of a structure in a6 to the maximum unsigned value, increments a 16-bit value at offset 4 of a structure in a5, and branches to another routine.
- `decrement_and_transfer_attribute`: Decrements a 16-bit value from an offset of 2 in a0 by 1, stores the result at offset 34 of a6, copies a 16-bit word from a0 to offset 4 of a5, and branches to another routine.
- `update_entity_index_and_transfer_data`: Copies a 16-bit word from a0 to offsets 6 and 16 of two different structures pointed to by a5 and a6, increments a 16-bit index at offset 4 of the structure in a5, and branches to update_entity_attribute_list.
- `update_entity_direction_and_state`: Increments a 16-bit value at offset 4 of the structure in a5 and updates several bytes in the structure in a6 by copying values from the structure in a0.
- `copy_and_reset_entity_state`: Copies a byte from offset 29 of a structure in a6 to offset 28 of the same structure, sets a byte at offset 28 to zero, clears a 16-bit word at offset 0 of a structure in a1, and branches to another routine.
- `load_and_call_addresses`: Loads four different addresses into a register and calls a routine after each load.
- `update_entity_state_logic_2`: Updates an entity's internal timers, position, and movement vectors based on decremented counters and scaling calculations.
- `branch_to_offset_2`: Performs a short relative branch to a different location in the code.
- `jump_to_offset`: Executes a short relative branch to another part of the program.
- `transfer_execution_flow`: Redirects the program flow via a short relative branch to a specific instruction.
- `branch_to_offset_6`: Performs a short relative branch to a location within the current segment.
- `branch_to_offset_7`: Performs a short relative branch to a specific instruction offset within the current segment.
- `update_state_and_dispatch`: Updates a byte in a structure based on its current value and increments a counter, then performs an indirect call using a value retrieved from a table.
- `jump_to_process_entity_transition`: Jump stub: passes control straight on to process_entity_transition.
- `branch_to_subroutine_2`: Performs a short relative branch to a specific location within another routine.
- `branch_to_offset_8`: Performs a short relative branch to a specific location within the current segment.
- `branch_to_offset_9`: Performs a short relative branch to a specific offset.
- `reset_entity_and_dispatch_2`: Clears several fields in a structure, increments a value at an offset, and performs an indirect call using a value retrieved from a table.
- `jump_to_animation_handler`: Returns immediately by performing a short branch to another location.
- `branch_to_offset_10`: Performs a short relative branch to a specific instruction offset within the current code segment.
- `branch_to_offset_12`: Performs a short relative branch to a fixed offset within the code segment.
- `calculate_value_and_dispatch`: Performs a calculation involving a table lookup and scaling, stores the result in a structure, and executes an indirect call via a table index.
- `branch_to_offset_11`: Performs a short relative branch to a specific offset within the code segment.
- `dispatch_via_table`: Updates a counter and a byte within a structure, then uses a value from a table to perform an indirect call.
- `jump_to_update_entity_data_and_dispatc`: Jump stub: passes control straight on to update_entity_data_and_dispatch.
- `branch_to_increment_entity_timer_and`: Returns immediately by performing a short branch to the increment_entity_timer_and_update_state routine.
- `jump_to_update_entity_state`: Jump stub: passes control straight on to update_entity_state.
- `branch_to_address_2`: Performs a short relative branch to a specific memory address.
- `process_entity_transition_2`: Copies values between structure offsets, calculates a new pointer using a bitwise operation, and performs an indirect call based on a byte value.
- `branch_to_jump_table`: Performs a short branch to a location stored in a register.
- `jump_to_update_state_with_conditional`: Jump stub: passes control straight on to update_state_with_conditional.
- `branch_to_stack_offset`: Performs a short relative branch to an address calculated by adding an offset to the current stack pointer.
- `branch_to_indirect_address`: Performs a short relative branch to an address stored in a pointer register.
- `update_entity_behavior_state`: Updates an entity's internal state machine, timers, and movement vectors using fixed-point arithmetic.
- `jump_to_check_and_decrement_byte`: Jump stub: passes control straight on to check_and_decrement_byte.
- `jump_to_compare_and_move_data`: Jump stub: passes control straight on to compare_and_move_data.
- `branch_to_jump_table_2`: Reads a byte from a structure at offset 12 in a6, clears that byte, and performs an indirect jump to an address calculated by shifting that byte left by two and adding it to a jump table.
- `jump_to_check_and_decrement_byte_2`: Jump stub: passes control straight on to check_and_decrement_byte.
- `update_entity_scaling_and_state`: Calculates a scaled value by looking up a base value in a table using an index derived from bytes at offsets 4 and 13 of two structures, then updates various counters, timers, and state flags within those structures.
- `jump_to_dispatch_table_entry`: Jump stub: passes control straight on to dispatch_table_entry.
- `update_sprite_attribute_and_state_2`: Copies a byte from offset 5 of the structure in a5 to offset 12 of the structure in a6, shifts a byte from offset 9 of the structure in a5 left by 4 bits and ORs it into offset 14 of the structure in a6, then calls another routine.
- `jump_to_update_entity_state_2`: Jump stub: passes control straight on to update_entity_state.
- `jump_to_update_entity_state_3`: Jump stub: passes control straight on to update_entity_state.
- `jump_to_update_entity_state_4`: Jump stub: passes control straight on to update_entity_state.
- `jump_to_update_entity_state_5`: Jump stub: passes control straight on to update_entity_state.
- `update_entity_state`: Updates an entity's internal counters, timers, and movement vectors based on decremented frame or tick values.
- `update_entity_data_and_dispatch`: Copies bytes and words between structures, increments a value, and performs an indirect call based on a table lookup.
- `increment_entity_timer_and_update_state`: Adds a 16-bit value from a source structure to a 16-bit value in a target structure, updates a 16-bit value at offset 4 of a third structure with the result, increments a 16-bit counter at offset 16 of the target structure, and then branches to either update_entity_state or update_entity_state_logic_2.
- `update_entity_physics_and_state`: Updates an entity's internal timers, movement vectors, and position through fixed-point arithmetic and decremented counters.
- `decrement_and_check_counter`: Selects a pointer from a table based on bits in a global variable, decrements a word at an offset from that pointer, and resets a byte to a specific value if the word becomes negative.
- `compare_and_move_data`: Compares a byte from a table to a value and, if a condition is met, moves a word to a destination before branching.
- `compare_and_update_index`: Compares a byte from a source structure to a byte at offset 24 of a destination structure; if the source byte is less than the destination byte, it writes a 16-bit word from a source structure to offset 16 of the destination structure, otherwise it branches to another routine.
- `check_and_decrement_byte`: Tests a byte in a structure, branches if zero, compares it to a constant, and decrements it if it is not that constant.
- `store_byte_and_increment`: Stores a byte into a structure, increments a word at a specific offset, and branches.
- `decrement_and_store_data`: Decrements a byte from a table, stores the result into a structure, moves a word to a destination, and branches.
- `process_entity_logic`: Evaluates multiple conditional checks to update counters and bytes within a structure before dispatching an action through an indirect call.
- `update_entity_counter`: Stores a byte from a register into a table, performs arithmetic on a word from a pointer, and stores the result into another location in a structure.
- `dispatch_conditional_action`: Compares a byte to a constant, moves two bytes from different structures into data registers, clears a word in a structure, and branches.
- `dispatch_entity_action`: Copies a byte to a structure, increments a counter, and performs an indirect call using a value retrieved from a table.
- `update_sprite_attribute_and_state`: Copies a byte from offset 5 of a structure in a5 to offset 12 of a structure in a6, then updates offset 14 of the structure in a6 by ORing it with the value from offset 9 of the structure in a5 shifted left by 4 bits.
- `calculate_entity_scaling_and_update_state`: Calculates a scaled value by looking up a base value in a table using an index derived from bytes at offsets 4 and 13 of two structures, then multiplies it by a value from offset 10 of the first structure plus 256.
- `update_state_with_conditional`: Copies a word to a structure, conditionally updates another word based on a zero check, and performs an indirect call using a table lookup.
- `update_entity_animation_frame`: Updates an entity's animation state by copying bytes from a source structure into a target structure and incrementing a frame counter.
- `reset_entity_and_dispatch`: Resets several fields in a structure, increments a counter, and performs an indirect call using a value retrieved from a table.
- `process_entity_transition`: Copies values between structure offsets, calculates a new pointer using a bitwise operation, and performs an indirect call based on a byte from a table.
- `update_entity_animation_state_2`: Updates an animation index and pointer within a structure in a5, then branches to a jump table based on a byte value retrieved from the animation data.
- `dispatch_table_entry`: Copies a word to a table location based on a masked byte, increments a counter, and performs an indirect call using a value from a table.
- `update_sprite_properties`: Updates a sprite or object structure in a table based on a decoded command or value in a register.
- `compare_and_set_flags`: Compares a byte with a stored value and sets two global variables to specific values based on the comparison and a high-byte check.
- `clear_and_copy_bytes`: Copies a byte from one memory location to another, clears a byte at a specific offset, and sets a value through a pointer to zero.
- `load_address_to_a0`: Loads the address of a data structure located at a fixed offset from the program counter into register a0 and returns immediately.
- `update_table_index`: Sets a byte in a structure to one, calculates a new pointer using a table lookup based on a bit-shifted value from d0, and stores this pointer in a structure, then clears a long value and sets a byte in a structure before returning.
- `update_state_structure`: Sets a byte to one in a global variable, stores a word from d0 and a long from a1 into a structure pointed to by a5, clears a long at offset 132 of that structure, and stores a byte from d0 at offset 102 of that structure before returning.
- `resolve_attribute_table_pointer`: Loads a pointer from a table, checks a flag at offset 464 of that structure, and if the flag is zero, calculates a new pointer by adding an offset from the structure to the base address and returns.
- `validate_and_call_attribute_handler`: Loads a pointer from a structure, calculates an index by shifting a value left by 3, and checks if the byte at that index is 251; if it is not, it returns, otherwise it loads a word from the structure at that index into a register and calls another routine.
- `update_object_attribute_table`: Checks a condition based on a value in a table and a sign bit, then updates a structure's pointer, byte, and word values using data from a source list.
- `clear_memory_fields`: Sets a byte to zero and then sets a longword at a specific offset to zero.
- `reset_system_state`: Clears several global variables and hardware registers including audio volumes, DMA control, and interrupt settings.
- `set_global_and_call`: Sets a global variable to the result of a bitwise OR between a register and a constant, then calls another routine.
- `update_status_and_memory`: Updates several global variables from a data table, performs a loop to clear a block of memory, and modifies a global variable via a bitwise OR.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_sequential_library_calls`: Calls a library function three times with different pointer arguments, updates two global variables with specific values, restores registers from a table, and jumps to a location stored in a jump table.
- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
