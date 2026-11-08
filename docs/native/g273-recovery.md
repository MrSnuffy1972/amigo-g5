# Game 273 (JamesPond v2.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00273). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (959070 top-level bytes in 13 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 72 of 72 units identical (bytes and relocations), 441 raw-word lines |
| Functions named and described | pass, 391 of 391 named (0 author symbols), 391 of 391 described (7 by rule, 272 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5904 of 6204 (95.16%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9769 of 10401 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 35328 |
| code | 6232 |
| data | 81571 |
| padding | 867 |
| unused | 835072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| JamesPond.Slave | 1 | 414 | 51 |
| Disk.1:bootblock | 1 | 28 | 3 |
| logged disk load from Disk.1 boot-block load | 1 | 0 | 0 |
| logged disk load from Disk.1 boot-block load | 1 | 107 | 2 |
| logged disk load from Disk.1 | 1 | 37 | 4 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 691 | 16 |
| logged disk load from Disk.1 | 1 | 517 | 11 |
| logged disk load from Disk.1 | 1 | 831 | 21 |
| logged disk load from Disk.1 | 1 | 1104 | 31 |
| logged disk load from Disk.1 | 1 | 1171 | 34 |
| logged disk load from Disk.1 | 1 | 1034 | 48 |
| logged disk load from Disk.1 | 1 | 1008 | 21 |
| logged disk load from Disk.1 | 1 | 757 | 28 |
| logged disk load from Disk.1 | 1 | 422 | 19 |
| logged disk load from Disk.1 | 1 | 202 | 7 |
| logged disk load from Disk.1 | 1 | 724 | 6 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 136 | 3 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 434 | 33 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 187 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 580 | 35 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 5 |

## Cases handled

- JamesPond_v2.0/JamesPond/Disk.1: file j1 (header 64832 bytes, blocks hold 11224): its blocks are judged as raw blocks outside the filesystem
- JamesPond_v2.0/JamesPond/Disk.1: the boot block reads 5120 bytes from disk offset 749056 to an address (found statically); recovered as a unit
- JamesPond_v2.0/JamesPond/Disk.1: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- 66 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 40 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 26 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 489 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- JamesPond_v2.0/JamesPond/Highs: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- JamesPond_v2.0/JamesPond/Disk.1: 66 logged loads cover 0 bytes of the image
- JamesPond_v2.0/JamesPond/Disk.1: blocks 748-757 (bytes 382976-388096) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.20)
- JamesPond_v2.0/JamesPond/Disk.1: blocks 759-768 (bytes 388608-393728) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.73)
- JamesPond_v2.0/JamesPond/Disk.1: blocks 770-779 (bytes 394240-399360) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.89)
- JamesPond_v2.0/JamesPond/Disk.1: blocks 781-790 (bytes 399872-404992) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.52)
- JamesPond_v2.0/JamesPond/Disk.1: blocks 792-801 (bytes 405504-410624) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.00)
- JamesPond_v2.0/JamesPond/Disk.1: blocks 803-812 (bytes 411136-416256) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- JamesPond_v2.0/JamesPond/Disk.1: blocks 814-823 (bytes 416768-421888) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.89)
- JamesPond_v2.0/JamesPond/Disk.1: blocks 825-834 (bytes 422400-427520) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- JamesPond_v2.0/JamesPond/Disk.1: blocks 836-845 (bytes 428032-433152) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- JamesPond_v2.0/JamesPond/Disk.1: blocks 847-856 (bytes 433664-438784) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- JamesPond_v2.0/JamesPond/Disk.1: blocks 858-879 (bytes 439296-450560) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.44)
- JamesPond_v2.0/JamesPond/Disk.1: blocks 883-883 (bytes 452096-452608) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.90)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `program_disk_dma_transfer` (Disk.1 +an address loaded at an address (5120 bytes)): Touches the disk DMA pointer and length registers, the audio and disk control register, the disk control port register, the DMA control register and the interrupt request register in a fifty-three-instruction routine called once. Kind: disk hardware/parameter validation loader. Pass path: the branch following the call (the code continues to the next instruction in the hunk).
- `update_audio_dma` (runtime segment an address (play-long-joy)): A large routine that modifies multiple audio channel registers, DMA control, and interrupt enable registers. Kind: disk hardware/track validation check. Pass path: the branch at an address (beq.b an address) is taken when the test fails, implying the 'not equal' path or the fall-through is the valid execution flow.

## Functions


### JamesPond.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `run_setup_then_service_loop`: Calls a setup routine through a pointer table, clears a block of longwords and writes a sequence of values through further calls.
- `call_slot_routine_repeatedly`: Calls the function stored at a fixed offset in an object fourteen times in a row, once per count.
- `accumulate_scaled_counter_and_call`: Multiplies a 16-bit field by a fixed factor, adds the product into a 32-bit counter, clears the field, then calls the function stored at a fixed offset in an object.
- `branch_on_flag_call_routine`: Tests a flag word and calls one of two routines accordingly, then returns the incoming register value unchanged.
- `process_block_in_chunks`: Splits a block length into chunks of at most a fixed size and calls the function stored at a fixed offset in an object for each chunk, advancing two addresses by each chunk length.
- `call_handler_with_scaled_size`: Multiplies a count by a fixed size, calls the function stored at a fixed offset in an object with the product and count, then adds the result into a global variable and clears it.
- `patch_pattern_with_jump`: Scans a fixed memory range word by word for a fixed instruction pattern and overwrites each match with an absolute jump to a routine.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `store_word_and_clear_field`: Calls a routine, then stores a passed word at a fixed offset in an object and clears a second field there.
- `store_word_at_offset`: Waits for a status clear and then stores a word from a register into a memory location at a specific offset from a pointer.
- `set_word_to_max`: Waits for a status clear and then stores a constant word value of all ones into a memory location at a specific offset from a pointer.
- `set_global_max`: Waits for a status clear and then stores a constant word value of all ones into a global variable.
- `store_multiple_values`: Waits for a status clear, stores a longword and a word into memory at specific offsets from a pointer, waits for a status clear again, and then stores a constant into a global variable.
- `set_global_to_constant`: Stores a fixed value in a global variable and returns.
- `enable_dma_channel`: Stores a constant value into a global variable and modifies the DMA controller register.
- `call_routine_then_store_constant`: Calls a routine, then stores a fixed value in a global variable.
- `store_routine_result_global`: Calls a routine and stores the value it returns in a global variable.
- `store_long_to_global`: Waits for a status clear and then stores a longword from a register into a global variable.
- `sync_and_store_register`: Waits for a hardware status flag to clear before storing the contents of a register into a global variable.
- `store_routine_register_result`: Calls a routine and copies an address register it leaves behind into a global variable.
- `wait_and_store_register`: Waits for a status flag in a global variable to clear and then stores a register value into a global variable.
- `store_register_to_global`: Waits for a hardware status bit to clear before copying the contents of a register into a global variable.
- `assign_status_to_global`: Stores the result of a status wait routine into a global variable.
- `sync_and_store_register_2`: Waits for a hardware status bit to clear before storing the contents of an address register into a global variable.
- `store_register_to_global_2`: Waits for a status bit to clear before copying the contents of a register into a global variable.
- `store_word_to_global`: Waits for a status clear and then stores a word into a global variable.
- `store_long_to_global_2`: Waits for a status clear and then stores a long word into a global variable.
- `set_global_constant`: Waits for a status clear and then stores a specific constant value into a global variable.
- `set_global_constant_2`: Waits for a status clear and then stores a different specific constant value into a global variable.
- `set_global_constant_3`: Waits for a status clear and then stores another specific constant value into a global variable.
- `nop_after_wait`: Waits for a status clear and then performs a redundant move of a global variable to itself.
- `wait_for_blit_completion`: Waits for a hardware status flag to clear and then performs a redundant load of a global variable into itself.
- `wait_and_store_result`: Waits for a status flag in a global variable to clear and then stores a value from a register into a global variable.
- `call_routine_clear_global`: Calls a routine, then clears a global variable.
- `clear_global_variable_2`: Waits for a status clear and then stores zero into a global variable.
- `clear_global_variable`: Waits for a status flag in a global variable to clear and then sets a global variable to zero.
- `set_global_value`: Waits for a status flag in a global variable to clear and then stores a constant value into a global variable.
- `call_routine_store_field`: Calls a routine, then stores a register value passed in at a fixed offset in an object.
- `store_long_to_offset`: Waits for a status clear and then stores a long word into a memory location calculated from a register and an offset.
- `store_register_via_pointer`: Waits for a status flag to clear before storing a value from a register into a memory location calculated from another register plus an offset.
- `call_table_routine_pair`: Calls the function stored at a fixed offset in a table and returns the two data register values unchanged.
- `call_table_routine_if_clear`: Calls the function stored at a second fixed offset in a table only when a flag word is zero, then returns the data registers unchanged.
- `unpack_bitstream_backwards`: Decodes a packed bit stream backwards from the end of a buffer.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bit_buffer_right`: Shifts a longword bit buffer right by a count, loading the previous stream longword with a set top bit whenever the buffer empties.
- `stub_infinite_loop`: Contains only an endless loop with no other effect, so it is a stub with no recoverable behaviour.
- `rotate_and_store_byte`: Performs a bitwise rotation on a byte and stores the result into a global variable after waiting for a hardware status flag to clear.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `wait_until_status_clear`: Waits in two loops until a status flag in a global variable clears, then returns the data register unchanged.
- `wait_until_status_clear_2`: Waits in two loops until a status flag in a global variable clears.

### Disk.1:bootblock

- `call_library_from_bootblock`: Boot block entry that makes one Amiga OS library call by its offset and passes control through a jump-table entry.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `issue_do_io_request`: Calls the exec DoIO library function and touches the bitplane control register, according to the facts.

### logged disk load from Disk.1 boot-block load

- `call_two_unnamed_routines`: Sixteen-instruction routine, called from two places, that makes two calls to unnamed routines.
- `call_three_unnamed_routines`: Thirty-eight-instruction routine, called from three places, that makes three calls to unnamed routines.

### logged disk load from Disk.1

- `initialize_structure_fields`: Populates several fields of a structure with a global variable and sets specific offsets to zero or other constant values.
- `validate_table_entries`: Iterates through a table and returns if it encounters a negative value in a specific offset.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_immediately_6`: The routine is empty and performs no operations before returning to the caller.

### logged disk load from Disk.1

- `initialize_data_structure_6`: Uses a global variable to index into tables and populates a data structure with retrieved values.
- `initialize_data_structure_2`: Populates a series of offsets within a structure using values from registers and a global variable.
- `initialize_data_structure_4`: Populates a series of offsets in a memory block with values from a global variable and a value from a calculated memory address.
- `scan_memory_list`: Iterates through a list of memory locations to check if a value at a specific offset is negative.
- `update_entity_attributes`: Iterates through a data table to check specific bit flags and conditions within a loop.
- `copy_indexed_data_block`: Calculates a destination address using a global variable and a modulo-based offset, then copies a block of words from a source table to that destination.
- `copy_data_block`: Calculates a destination address based on a global variable and an input value, then copies sixteen words from a source table to that destination in a loop.
- `clear_data_block`: Calculates a destination address based on a global variable and an input value, then fills a block of memory with zeros and a specific word value in a loop.
- `update_object_properties`: Iterates through a list of structures to modify values in a table based on specific flag values and pointer checks.
- `initialize_global_table`: Stores a series of specific constant values into a global variable sequentially.
- `toggle_status_bits`: Decrements a global variable and, if it reaches a specific threshold, calls another routine and toggles two bits in two different global variables.
- `initialize_data_structure_5`: Determines a value based on a global variable and populates a memory block with that value and other constants.
- `search_and_initialize_table`: Searches a table for a matching word and, if found, populates a series of global variables and memory locations with specific data.
- `process_indexed_data`: Decrements a global variable and uses it to index into tables to retrieve values, which are then used to initialize a data structure.
- `process_pointer_list`: Checks a value in a structure, modifies a word, and enters a loop that compares values from a table and updates a global variable based on the results.
- `parse_amigados_hunk_list`: Iterates through a list of pointers to process data structures by comparing specific byte values against known hunk type codes.

### logged disk load from Disk.1

- `parse_amigados_hunk_headers`: Iterates through a memory structure to interpret and validate AmigaDOS hunk type codes.
- `parse_amigados_hunks`: Iterates through a memory structure to identify and process AmigaDOS hunk types, such as executable code or relocation data.
- `update_linked_structure`: Traverses a linked structure using a bitmask table and updates various fields within a data block based on the results.
- `increment_structure_offsets`: Increments two specific word values within a data block by a calculated amount derived from a bitmask table.
- `parse_bitmask_table`: Reads a series of offsets from a table to traverse a linked structure, accumulating a bitmask based on the high bit of each byte encountered.
- `clear_structure_fields`: Sets four specific offsets within a structure to zero or two.
- `search_table_entries`: Iterates through a table of data to find an entry matching specific coordinate and boundary conditions.
- `validate_coordinate_range`: Checks if a value derived from a structure offset falls within a specific range and updates a global variable.
- `toggle_status_flag`: Checks a bit in a structure and updates a global variable based on its value and another structure field, then toggles a byte via XOR.
- `process_state_machine`: Updates a structure based on a global variable and enters a loop that modifies structure fields based on bitwise flags.
- `validate_coordinate_bounds`: Checks if a set of coordinates or dimensions falls within specified minimum and maximum limits.

### logged disk load from Disk.1

- `update_object_data`: Compares structure values to determine a status and fetches new data from a table using an offset.
- `reset_global_status`: Sets a structure field to all ones and resets a global variable to a specific constant.
- `update_object_state_and_timers`: Updates an object's status, timer, and coordinate data based on its current state and type.
- `reset_status_and_variable`: Resets a structure field to all ones and sets a global variable to a specific constant.
- `decrement_and_check_value`: Decrements a 16-bit value in a structure and updates a global variable or a structure field based on the result.
- `update_accumulated_values`: Adds a value to a global variable and a value pointed to by a register, then resets the global variable through a sequence of assignments.
- `increment_accumulated_values`: Adds a value to both a global variable and a value pointed to by a register.
- `set_global_constants`: Sets a global variable to zero, then to all ones, then to a specific constant.
- `validate_object_state`: Checks various bytes and shorts within a structure to determine a status, updating a global variable accordingly.
- `check_byte_threshold`: Checks if a byte at a specific offset in a structure is less than ten and updates a global variable.
- `check_minimum_value`: Checks if a byte at a specific offset in a structure is less than twelve and updates a global variable.
- `decrement_and_check_counter`: Decrements a value in a structure and performs conditional updates to other structure fields or calls another routine if the value reaches zero.
- `update_coordinate_table`: Calculates new values based on a global variable and a table entry, then updates structure fields and an index.
- `process_object_logic`: Evaluates structure fields and a global variable to perform conditional updates to structure members or bitwise operations on a byte.
- `manage_structure_lifecycle`: Decrements a counter in a structure and performs complex conditional updates to various structure fields and global variables based on multiple status checks.
- `update_game_state_counters`: Modifies several global variables by adding or subtracting values from constants and clamped totals.
- `clamp_subtraction_result`: Subtracts a value from a global variable and ensures the result does not fall below zero.
- `increment_clamped_global`: Adds a value to a global variable if it is non-negative, then resets the global variable to zero if it exceeds a specific threshold.
- `subtract_from_constant`: Subtracts a value from a constant and stores the result in a global variable.
- `add_to_constant`: Adds a value to a constant and stores the result in a global variable.
- `update_memory_via_logic`: Performs complex bitwise logic to determine an index and updates a memory location through a pointer.

### logged disk load from Disk.1

- `update_global_flags`: Modifies a global variable using bitwise logic based on the state of other registers and conditional checks.
- `apply_bitwise_mask`: Modifies a global variable based on bitwise operations between two other values.
- `update_conditional_word`: Modifies a global word based on bitwise logic applied to other registers and conditional checks.
- `dispatch_bitmask_actions`: Checks various bits within a global variable and calls different subroutines depending on which bits are set.
- `decrement_or_reset_counter`: Decrements a global variable or performs a series of register manipulations and subroutine calls if the variable reaches specific values.
- `traverse_data_table`: Increments a global index and uses the value at that index to jump to a new position within a memory range.
- `clear_global_value`: Checks if a global value is negative and performs several assignments to registers and a global variable.
- `process_conditional_logic_2`: Performs a series of conditional checks on a global variable to modify its value through arithmetic and bitwise masking.
- `shift_and_call`: Performs a bitwise shift on a value and calls another routine.
- `modify_global_pointer_2`: Adds a constant to a global variable and performs bitwise shifts and comparisons.
- `modify_global_pointer`: Adds a constant to a global variable and performs bitwise shifts and comparisons.
- `process_conditional_logic`: Performs a series of conditional checks on a global variable to modify its value through arithmetic and bitwise masking.
- `decrement_word_with_bounds`: Decrements a 16-bit global variable and ensures it stays within a specific range by clamping it to zero or two.
- `empty_routine`: This routine is empty and performs no operations before returning.
- `validate_and_set_value`: Checks a global variable against several conditions and sets it to specific constant values.
- `initialize_system_state`: Performs a sequence of register assignments, bitwise masking on a pointer, and multiple subroutine calls.
- `process_complex_state`: Evaluates a global variable through multiple conditional branches and performs memory writes or calls other routines.
- `increment_and_clamp_global`: Increments a global variable and applies clamping logic based on various arithmetic comparisons.
- `set_global_constant_4`: Sets a global variable to a specific value and calls another routine.
- `calculate_clamped_values`: Performs arithmetic on two values derived from a global variable and applies clamping logic to ensure they stay within specific ranges.
- `decrement_global_variable`: Decrements a global variable and resets it to a specific value if it falls below zero.
- `reset_global_variable`: Sets a global variable to a constant value and performs conditional checks against zero.
- `update_state_variable`: Checks a global variable against specific bit patterns and values, potentially calling another routine or assigning a new constant.
- `clamp_global_variable`: Sets a global variable to one if its current value exceeds a specific threshold.
- `set_status_flag`: Checks a global variable and sets another global variable to one of two different constant values based on a bitwise test.
- `process_data_table`: Iterates through a table of data, decrementing byte values and performing conditional updates to word values based on specific byte markers.
- `modify_table_entries`: Iterates through a data structure to decrement bytes and adjust word values using a global variable as a multiplier.
- `map_character_to_values`: Compares a character in a register against several constants and sets a global variable to different values based on the match.
- `process_text_and_initialize_memory`: Copies a block of data from one memory location to another and iterates through a string to calculate a sum based on character values.
- `initialize_memory_blocks`: Fills three distinct areas of memory with a repeating pattern of constant values using a loop.
- `initialize_memory_and_call_subroutines`: Fills a block of memory with all ones using a loop and then performs a series of conditional calls to another routine while updating a global variable.

### logged disk load from Disk.1

- `return_offset_value`: Checks if a global variable plus a constant is non-negative and returns that value plus eight.
- `call_one_unnamed_routine`: Thirteen-instruction routine, called from two places, that makes one call to an unnamed routine.
- `forward_to_unnamed_routine`: Single-instruction routine whose address is taken by a pc-relative load, and which makes one call to an unnamed routine.
- `enable_dma_channels`: Writes the DMA control register and makes five calls to unnamed routines.
- `call_two_unnamed_routines_2`: Nine-instruction routine, called once, that makes two calls to unnamed routines.
- `program_blitter_registers`: Touches the blitter source pointer, destination pointer, destination modulo and size registers, and makes two calls to unnamed routines.
- `call_one_unnamed_routine_2`: Sixty-two-instruction routine, called from one place, that makes one call to an unnamed routine.
- `configure_blitter_masks_and_dma`: Touches the blitter first and last word masks, modulo and control registers, and the DMA control register.
- `call_two_unnamed_routines_3`: Thirty-nine-instruction routine, called from two places, that makes two calls to unnamed routines.
- `program_blitter_and_dma`: Touches the blitter pointer, modulo and size registers and the DMA control register, and makes three calls to unnamed routines.
- `call_one_unnamed_routine_3`: Nineteen-instruction routine, called from one place, that makes one call to an unnamed routine.
- `program_blitter_and_call`: Touches the blitter masks, pointers, modulo, control and size registers across a body of 132 instructions, and makes one call to an unnamed routine.
- `load_unrecovered_routine_address`: Twenty-eight-instruction routine whose address is loaded pc-relative and which is called once.
- `enable_dma_and_call_two`: Touches the DMA control register and makes two calls to unnamed routines.
- `set_blitter_control_and_dma`: Touches the second blitter control register and the DMA control register.
- `decrement_and_blitter_check`: Decrements a global variable and performs conditional logic involving a blitter routine and a range check on the resulting value.
- `tiny_ori_subi_andi`: Small routine of 3 instructions that merges 3 into d1, then subtracts 3 from d5, then masks d1 with 0.
- `program_blitter_registers_and_dma`: Touches the blitter masks, pointers, modulo, control and size registers and the DMA control register, and makes no calls according to the facts.
- `program_blitter_registers_and_dma_2`: Touches the blitter masks, pointers, modulo, control and size registers and the DMA control register, and makes no calls according to the facts.
- `program_blitter_registers_then_call`: Touches the blitter masks, pointers, modulo, control and size registers and the DMA control register, then makes one call to an unnamed routine.
- `call_two_unnamed_routines_4`: Seven-instruction routine, called once, that makes two calls to unnamed routines.
- `call_one_unnamed_routine_4`: Thirty-one-instruction routine, called from two places, that makes one call to an unnamed routine.
- `update_display_parameters`: Performs bitwise masking on memory locations, calls a display routine, and modifies a global variable based on a comparison of calculated values.
- `set_display_window_and_colours`: Touches the collision control register, the background colour register, the display window start register and a custom chip register at an offset, and makes nine calls to unnamed routines.
- `render_referenced_text_messages`: References the text strings 500,000, EXTRA!, EXIT OPEN! and HURRY UP!, and makes three calls to unnamed routines.
- `call_one_unnamed_routine_5`: Seventeen-instruction routine, called once, that makes one call to an unnamed routine.
- `process_unrecovered_leaf_routine`: Twenty-one-instruction routine, called once, with no calls listed in the facts.
- `process_unrecovered_leaf_routine_2`: Sixty-five-instruction routine, called once, with no calls listed in the facts.
- `process_unrecovered_leaf_routine_3`: Nineteen-instruction routine, called once, with no calls listed in the facts.
- `forward_to_unnamed_routine_2`: Three-instruction routine, called once, that makes one call to an unnamed routine.
- `call_three_unnamed_routines_2`: Fifty-one-instruction routine, called once, that makes three calls to unnamed routines.
- `call_one_unnamed_routine_6`: Thirty-one-instruction routine, called once, that makes one call to an unnamed routine.
- `process_unrecovered_leaf_routine_4`: Twenty-seven-instruction routine, called once, with no calls listed in the facts.
- `process_unrecovered_leaf_routine_5`: Forty-two-instruction routine, called once, with no calls listed in the facts.

### logged disk load from Disk.1

- `accumulate_shifted_values`: Iterates through a list, right-shifting values and adding them to a destination buffer until a counter reaches zero.
- `execute_sequence_of_calls`: Sets a global variable to a specific value and executes three consecutive subroutines.
- `return_immediately_5`: the routine performs no operations and returns immediately or exits without any side effects.
- `set_display_window_and_call`: Touches the display window start register and makes two calls to unnamed routines.
- `execute_main_game_loop`: Configures display hardware and enters a loop that sequentially updates game state, renders text, and manages DMA transfers.
- `update_state_and_pattern`: Modifies a global variable through arithmetic and uses a loop to write a pattern into a table.
- `perform_blitter_copy`: Configures blitter parameters and performs a block memory transfer using a loop.
- `copy_and_setup_copper_list`: Copies a block of data to a new location, populates several global variables with specific values, and searches a table to replace a specific word before updating copper pointers.
- `fetch_indirect_data`: Uses a global variable as an index into a table to fetch a value, then uses that value to fetch another value from a fixed memory location.
- `scan_list_and_offset_global`: Scans a list of words for a specific bit pattern and then increments a global variable by a fixed amount based on the scan results and an initial value.
- `process_data_table_sequence`: Iterates through a data table using a pointer, checking a bitmask condition on each element until a specific bit is set or a counter limit is reached.
- `setup_blitter_parameters`: Sets several hardware registers used for blitter operations, including control, mode, depth, size, and DMA enable registers.
- `setup_blitter_parameters_alt`: Configures blitter hardware registers with a different set of values than the first routine.
- `fill_table_with_pattern`: Modifies a global variable through arithmetic and then uses a loop to write a calculated pattern of words into a table at a fixed memory location.
- `process_string_data`: Reads characters from a memory location derived from a global variable and performs conditional arithmetic and calls to another routine based on the data.
- `set_hardware_registers`: Writes specific constant values to hardware registers or memory locations.
- `call_routine_conditionally`: Checks the lowest bit of a global variable and calls another routine regardless of the result.
- `configure_display_and_copper`: Touches the bitplane modulo and control registers, the data fetch start and stop registers, the display window start and stop registers, the DMA control register and the copper jump strobe, and makes one call to an unnamed routine.
- `set_copper_list_pointers`: Touches the high location registers of copper lists one and two.
- `decrement_uninitialized_value`: An unreachable routine that attempts to decrement an uninitialized value in a loop.
- `run_unrecovered_single_instruction`: Single-instruction routine, called once, with no calls or registers listed in the facts.
- `access_background_colour_register`: Touches the background colour register in a seven-instruction routine that is called once.
- `run_unrecovered_tiny_leaf`: Five-instruction routine, executed in the reference run, with no calls or registers listed in the facts.
- `run_unrecovered_tiny_leaf_2`: Six-instruction routine, unreached in the static flow and called once, with no calls or registers listed in the facts.
- `configure_cia_timer_interrupts`: Touches the high and low halves of CIA-A timer B, its control and interrupt control registers, the copper list one high register, the DMA control register, and the interrupt enable and request registers.
- `access_cia_interrupt_registers`: Touches the CIA-A control register A and interrupt control register, together with the interrupt request register.
- `forward_to_unnamed_routine_3`: Three-instruction routine whose address is loaded pc-relative and which makes one call to an unnamed routine.
- `access_colour_and_raster_position`: Touches the background colour register and the vertical and horizontal beam position read register.
- `access_keyboard_serial_register`: Touches the CIA-A serial data register, which the facts identify as the keyboard serial register, along with the CIA-A interrupt control and interrupt request registers.
- `access_joystick_and_fire_state`: Touches the joystick movement register, the CIA-A port register holding fire buttons and disk status, the interrupt request registers, the playfield control register and colour register 31.
- `access_cia_interrupt_registers_3`: Performs a read-back operation on the interrupt request registers.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `configure_beam_control_and_call`: Touches the beam control register in a forty-three-instruction routine that makes three calls to unnamed routines.
- `call_one_unnamed_routine_7`: Three-instruction routine, unreached in the static flow, that makes one call to an unnamed routine.
- `access_background_colour_register_2`: Touches the background colour register in a thirteen-instruction routine called from two places.
- `access_colour_then_call`: Touches the background colour register and makes one call to an unnamed routine in a three-instruction routine.
- `check_and_clear_byte`: Compares a global variable to a specific character and, if it matches, clears the variable and calls another routine.
- `call_one_unnamed_routine_8`: Eight-instruction routine that makes one call to an unnamed routine and is called from one place.
- `empty_routine_2`: This routine contains no executable instructions and performs no operations before returning.
- `process_unrecovered_leaf_routine_7`: Fifteen-instruction routine, called once, with no calls listed in the facts.
- `program_blitter_registers_and_dma_3`: Touches the blitter control, modulo, destination pointer and size registers and the DMA control register in a nine-instruction routine called once.
- `run_unrecovered_tiny_leaf_3`: Two-instruction routine, called once, with no calls or registers listed in the facts.
- `loop_and_call_subroutines`: Executes a loop that calls another routine and performs arithmetic on a global variable before calling several other subroutines.
- `convert_number_to_string`: Performs a series of loops to build a numeric representation in a global variable and then replaces specific characters with spaces.
- `access_colour_then_call_2`: Touches the background colour register in a sixty-five-instruction routine that makes two calls to unnamed routines.
- `run_unrecovered_tiny_leaf_4`: Five-instruction routine, called once, with no calls or registers listed in the facts.
- `process_unrecovered_leaf_routine_8`: Eleven-instruction routine, called once, with no calls listed in the facts.
- `calculate_sprite_attribute_offsets`: Calculates memory offsets for sprite data based on a series of mathematical transformations and nested loops.

### logged disk load from Disk.1

- `process_bitmapped_data_tables`: Generates bitmapped data through bitwise operations and expands values from a source table into a destination buffer using bit-shifting.
- `update_game_state_parameters`: Updates a global state structure and various data tables based on a provided index value.
- `set_global_and_copy`: Sets a global variable to one and calls a routine to copy multiple data blocks between various locations.
- `copy_data_blocks`: Copies multiple blocks of data from two source locations to three different destination locations using a loop.
- `fill_incrementing_table`: Fills a table with incrementing values based on a global variable and updates two other global variables.
- `store_doubled_value`: Stores a global variable and its doubled value into two different memory locations.
- `calculate_and_store_offsets`: Performs arithmetic on a global variable to calculate several values and stores them in a series of global variables.
- `generate_bitmapped_data`: Uses nested loops to perform bitwise operations on source data and store the results into a destination buffer.
- `expand_and_copy_data_table`: This routine reads data from a source table, performs a bit-shifting expansion on the values, and writes the resulting sequence into a destination buffer.
- `write_constant_to_pointer`: Checks if a global variable is zero and then writes two words of data to a memory location pointed to by a register.
- `populate_attribute_table`: Iterates through a data structure to write values into a table based on specific conditional checks.
- `initialize_memory_structures`: Sets multiple global variables to specific constants and clears a block of memory using a loop.
- `call_conditional_routine`: Calls another routine regardless of the value held in a global variable.
- `calculate_bitmask_from_linked_list`: Traverses a linked list of bytes, summing their values and accumulating a bitmask based on whether the high bit of each byte is set.
- `increment_global_variable`: Adds a series of constant values to a global variable.
- `conditionally_update_variable`: Adds different constant values to a global variable based on whether that variable is zero.
- `search_table_for_value`: Compares a value from a pointer against a global variable and performs conditional updates to that variable based on results from a table.
- `increment_global_on_match`: Compares values from a table against a global variable and increments the global variable if specific conditions are met.
- `do_nothing`: The routine is empty and performs no operations before returning.
- `fetch_table_value`: Calculates an offset using two input words to fetch a short integer from a table and compares it against a global variable.
- `process_state_variable`: Evaluates a global variable against several specific constant values to determine control flow.

### logged disk load from Disk.1

- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `update_global_with_function_result`: Performs conditional checks on a global variable and then updates it using a bitwise combination of a function return value and another register.
- `set_global_via_function`: Checks a global variable against several constants and, if the condition is met, updates it using a bitwise combination of a function return value and another register.
- `update_game_state_and_render`: Updates various internal state variables and executes a sequence of subroutines to handle logic, text rendering, and DMA management.
- `configure_audio_and_dma`: Updates multiple audio hardware registers, manages DMA controller settings, and configures interrupt enable registers.
- `decrement_register_to_zero`: Decrements a register from a starting value until it reaches negative one.
- `set_audio_channel_zero_sample`: Touches the audio channel zero location and length registers in a five-instruction routine that makes one call to an unnamed routine.
- `set_audio_channel_one_sample`: Touches the audio channel one location and length registers in a five-instruction routine that makes one call to an unnamed routine.
- `set_audio_channel_two_sample`: Touches the audio channel two location and length registers in a five-instruction routine that makes one call to an unnamed routine.
- `set_audio_channel_three_sample`: Touches the audio channel three location and length registers in a five-instruction routine that makes one call to an unnamed routine.
- `set_audio_channel_zero_volume`: Touches the audio channel zero volume register and the DMA control register in a three-instruction routine called once.
- `set_audio_channel_one_volume`: Touches the audio channel one volume register and the DMA control register in a three-instruction routine called once.
- `set_audio_volumes_two_three`: Touches the audio channel two and three volume registers and the DMA control register in a six-instruction routine called from two places.
- `call_eight_unnamed_routines`: Twenty-three-instruction routine, called from four places, whose address is loaded pc-relative and which makes eight calls to unnamed routines.
- `update_audio_state`: Decrements a global variable and conditionally updates audio channel samples and volumes based on the remaining value.
- `update_audio_volume_state`: Updates a global variable based on a value returned from another routine and modifies the third audio volume register.
- `update_on_value_change`: When a stored value is nonzero and differs from the incoming value, it stores the new value, calls two unnamed routines and sets a global to one.
- `update_sprite_pointers`: Updates the hardware sprite pointer registers by iterating through a table of data.
- `set_sprite_zero_pointer`: Touches the sprite zero high location register in a forty-two-instruction routine called once.
- `set_sprite_pointers_two_three`: Touches the sprite two and sprite three high location registers in an eighty-seven-instruction routine called once.
- `setup_blt_parameters`: Calculates a memory address based on a global variable and input values, then stores the result and several derived values into a table.
- `parse_and_load_hunks`: Iterates through a table of data to identify and process memory segments by comparing values against AmigaDOS hunk type codes.
- `clear_blt_data`: Checks a value at an offset from a pointer and, if non-zero, clears it and performs arithmetic on other values at that offset.
- `decrement_and_call_loop`: Decrements a global variable and calls another routine repeatedly until the variable reaches a specific value.
- `sort_and_copy_table`: Copies data from one table to another and then performs a selection sort on a list of values stored in a table.
- `zero_memory_blocks`: Fills three distinct memory areas with zeros using loops of specific lengths.
- `call_one_unnamed_routine_9`: Thirty-eight-instruction routine, called once, that makes one call to an unnamed routine.
- `update_pointer_and_check_bytes`: Increments a value at a pointer, updates a stored pointer, and checks a sequence of bytes for specific bit patterns.

### logged disk load from Disk.1

- `empty_routine_3`: This routine contains no instructions and performs no operations.
- `clear_byte_sequence`: If a pointer retrieved from a memory location is not null, it clears three consecutive bytes starting from that pointer.
- `calculate_table_index`: Iterates through a table to find a matching value, then performs arithmetic to store a calculated result into a table entry.
- `iterate_and_call_routine`: Iterates through a table in increments of four and calls another routine if a value at a specific offset is non-zero.
- `zero_short_array`: Fills a sequence of memory starting at a specific offset with zeros using a loop.
- `scan_and_clear_pointers`: Iterates through a table and calls another routine to clear three bytes if a non-zero value is found at a specific offset.
- `calculate_coordinate_offset`: Performs bitwise operations and arithmetic on a global variable to return a combined 64-bit value.
- `write_multiple_globals`: Calls another routine and then stores three different values into a global variable.
- `execute_infinite_loop`: Runs an infinite loop that repeatedly calls other routines, updates a global variable, and modifies display settings.
- `call_one_unnamed_routine_10`: Twelve-instruction routine, called once, that makes one call to an unnamed routine.
- `call_two_unnamed_routines_5`: Fourteen-instruction routine that makes two calls to unnamed routines.
- `call_routines_with_text_reference`: Routine of 143 instructions, called from three places, that references the text DESIGN,PROGRAMMING and makes seven calls to unnamed routines.
- `process_complex_logic`: Performs a large sequence of conditional checks, calls various routines, and manipulates a global variable through a loop.
- `process_unrecovered_leaf_routine_9`: Seven-instruction routine, called once, with no calls listed in the facts.
- `execute_eleven_instructions`: A routine consisting of eleven instructions that is called by another routine.
- `process_text_reference`: This routine is invoked by a function that handles text references.
- `process_unrecovered_leaf_routine_10`: Twelve-instruction routine, called once, with no calls listed in the facts.
- `process_unrecovered_leaf_routine_11`: Forty-six-instruction routine, called once, with no calls listed in the facts.
- `set_copper_list_two_pointer`: Touches the high location register of copper list two in a nineteen-instruction routine called once.

### logged disk load from Disk.1

- `process_unrecovered_leaf_routine_12`: Twenty-two-instruction routine, executed in the reference run, with no calls, registers or callers listed in the facts.
- `ensure_positive_global`: Checks if a global variable is zero or negative and sets it to one if so.
- `fill_memory_with_ones`: Fills a range of memory starting at a specific address with the value zeroxffff using a loop.
- `populate_data_tables`: Iterates through a table to calculate values and write them into a destination table using various arithmetic operations and conditional logic.
- `populate_data_table_2`: Searches a table for a value and writes a series of calculated constants and global variable contents into a destination table.
- `populate_data_table`: Writes a series of values and global variable contents into a table pointed to by a register.
- `increment_and_check_table`: Increments a global variable with a wrap-around check and performs multiple null checks against a table.

### logged disk load from Disk.1

- `run_unrecovered_loop_entry`: Eight-instruction code entered at a loop head, with one caller and no calls listed in the facts.
- `process_unrecovered_leaf_routine_13`: Eight-instruction routine, called once, with no calls listed in the facts.
- `access_fire_button_port`: Touches the CIA-A port register that holds the fire buttons and disk status, and makes five calls to unnamed routines in a forty-three-instruction routine.
- `program_disk_dma_transfer`: Touches the disk DMA pointer and length registers, the audio and disk control register, the disk control port register, the DMA control register and the interrupt request register in a fifty-three-instruction routine called once.
- `read_vertical_position_register`: Touches the vertical position read register in a fifteen-instruction routine called once.
- `process_unrecovered_large_routine`: A 597-instruction routine whose address is loaded absolutely and which is called once.

### logged disk load from Disk.1

- `initialize_memory_block`: Copies a block of data from one location to another and initializes several memory locations with specific constant values.
- `setup_audio_and_object`: Clears a memory block, initializes an object structure with various constants, and modifies audio and DMA hardware registers.
- `initialize_data_structure`: Clears a block of memory, updates several fields in a structure, and modifies hardware registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_invalid_instruction`: This routine contains a single instruction that results in invalid data and terminates control flow.
- `external_dispatch_entry_a`: Performs a jump to subroutine through an absolute long address, pushing a return address on the stack before control transfers.
- `call_external_routine`: calls another routine and then terminates execution.
- `call_external_routine_2`: calls another routine and then executes invalid data.
- `external_dispatch_entry_b`: Transfers execution to another routine using an absolute long subroutine call and relies on that callee to continue execution.
- `update_global_values`: This routine modifies several global variables using values returned from other routines and interacts with hardware registers for bitplane depth and size.
- `configure_blt_graphics_parameters`: Configures the Blitter hardware registers to set up a graphics transfer operation.
- `parse_data_stream_6`: Iterates through a data stream by checking byte values and performs arithmetic on a global variable and bit flag searches.
- `parse_data_stream_5`: Interprets a data stream by performing arithmetic on a global variable and branching based on byte values.
- `parse_data_stream`: Processes a data stream by checking for specific byte values and updating a global variable through recursive calls and pointer arithmetic.
- `parse_data_stream_2`: Iterates through a data stream, performing arithmetic on a global variable based on character comparisons and nested routine calls.
- `parse_data_stream_3`: A routine that interprets a sequence of bytes to modify a global variable using various arithmetic and conditional logic.
- `parse_data_stream_4`: Processes a sequence of bytes to update a global variable through a series of conditional checks and pointer increments.
- `search_bit_flags`: This routine iterates through a table of six words to check for specific bit flags and performs arithmetic based on the number of matches found.
- `execute_single_instruction`: The routine executes exactly one instruction and then returns.
- `configure_blt_registers`: This routine modifies several bitplane and area registers and performs arithmetic on a global variable using a value from another routine.
- `external_dispatch_entry_c`: Invokes a different absolute long call target with a single jump to subroutine instruction and no other local operations.
- `call_external_routine_3`: Calls another routine and then terminates execution.
- `jsr_to_higher_expansion_sub`: Performs a single jump-to-subroutine to one fixed absolute address residing in expansion memory and then continues after that call returns.
- `jsr_to_lower_expansion_sub`: Performs a single jump-to-subroutine to a different fixed absolute address in expansion memory lying about two hundred forty bytes below the target used by the companion routine.
- `call_external_routine_4`: executes another routine and then stops.
- `update_blt_size`: The routine writes a specific value to a global variable and interacts with the BLTSIZE hardware register.
- `perform_no_operation_3`: The routine performs a redundant assignment to a global variable and interacts with the COP2LCH register.
- `initialize_hardware_registers`: Sets multiple memory locations to a specific value and configures CIAA, DMA, and interrupt hardware registers.
- `modify_cia_interrupt`: The routine performs bitwise operations on a global variable and interacts with CIAA and interrupt request hardware.
- `modify_interrupt_state`: Interacts with interrupt request hardware and modifies a global variable based on status flags.
- `process_character_input`: Compares a returned character against a table to modify a global variable through bitwise inversion or specific value assignments.
- `execute_delay_loop`: Performs a countdown loop that updates a global variable with masked values for a fixed number of iterations.
- `delay_via_loop`: The routine executes a countdown loop that repeatedly updates a global variable with masked values.
- `check_character_input`: The routine compares an input byte against specific character values and sets a global variable based on the result.
- `configure_blt_registers_2`: The routine writes a value to a global variable and interacts with the BLTCON0 and BLTCON1 hardware registers.
- `execute_invalid_instruction_2`: This routine contains a single instruction that triggers a control flow error.
- `execute_empty_instruction`: This routine consists of a single instruction and lacks available pseudo-code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_position_and_offset`: Calculates a new memory offset based on a multiplier and updates a global variable.
- `execute_invalid_instruction_3`: Contains a single instruction that results in invalid data.
- `call_external_routine_5`: This routine performs a single call to another function and then terminates.
- `execute_short_routine`: A routine consisting of twenty instructions.
- `copy_memory_blocks`: Copies data from a source pointer to multiple destination offsets using a calculated length, then performs a second copy loop from a fixed memory location to a destination pointer.
- `copy_memory_blocks_2`: Iterates through a list of descriptors to perform multiple memory copy operations using specified lengths and offsets.
- `execute_single_instruction_2`: A routine consisting of one instruction.
- `return_immediately_2`: An empty routine that performs no actions.
- `return_no_op_stub`: The routine consists of a single instruction that immediately returns to its caller, performing no work.
- `execute_short_routine_2`: A routine consisting of nine instructions.
- `copy_memory_to_global`: Copies four consecutive words from a memory location pointed to by a register into a global variable.
- `perform_memory_operations`: Executes multiple subroutines and performs a bitwise XOR operation on a byte in memory.
- `poll_hardware_status`: Performs a loop checking specific bits in hardware registers and a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `trigger_audio_update`: This routine consists of a single instruction that calls a routine to modify audio channel registers, DMA control, and interrupt enable registers.
- `execute_single_instruction_3`: A single instruction routine that calls another routine.
- `trigger_audio_update_2`: This routine consists of a single instruction and executes an external call to a routine that modifies audio channel registers, DMA control, and interrupt enable registers.
- `call_subroutine`: The routine executes a single instruction that performs a subroutine call to another routine.
- `call_subroutine_2`: Executes a single instruction to call another routine.
- `manipulate_interrupt_registers`: This routine modifies interrupt mask and request registers and interacts with custom hardware.
- `execute_subroutine_call`: Performs a sequence of instructions and calls another routine.
- `configure_interrupt_dma`: A seven instruction routine that calls a routine to manipulate DMA and interrupt hardware registers.
- `configure_hardware_interrupts_and_dma`: This routine manipulates the DMA control, interrupt enable, and interrupt request hardware registers.
- `execute_subroutine`: Calls another routine and consists of seven instructions.
- `trigger_audio_updates`: Calls a routine that modifies audio channel registers, DMA control, and interrupt enable registers twice.
- `update_audio_dma`: A large routine that modifies multiple audio channel registers, DMA control, and interrupt enable registers.
- `toggle_audio_dma_interrupts`: Routine of 43 instructions that writes the Amiga custom chip DMA control, interrupt enable, and interrupt request registers, using the set/clear bit 15 convention to enable or disable audio DMA channels and interrupts.
- `configure_dma_and_interrupts`: Configures DMA channels and interrupt settings by writing to the DMA control and interrupt enable hardware registers.
- `update_animation_timers`: Decrements several internal counters and updates position or frame offsets based on the values of these timers.
- `configure_hardware_interrupts`: This routine calls another routine to set up DMA channels and interrupt enable hardware registers.
- `configure_dma_interrupts`: Performs hardware configuration for DMA and interrupts through a series of five instructions.
- `configure_dma_interrupts_2`: Executes five instructions to set up DMA channels and interrupt enable registers.
- `configure_dma_interrupts_3`: Uses twelve instructions to configure DMA and interrupt hardware settings.
- `configure_dma_interrupts_4`: A twenty-instruction routine that manages DMA and interrupt hardware configuration.
- `configure_dma_interrupts_5`: A two-instruction routine that configures DMA and interrupt settings.
- `process_input_event_queue`: Iterates through two decrementing counters to process queued input events or timer updates.
- `call_dma_interrupt_config`: This routine calls another routine that configures DMA channels and interrupt settings by writing to hardware registers.
- `configure_hardware_interfaces`: Calls a routine that writes to DMA control and interrupt enable hardware registers.
- `call_hardware_configuration`: Executes a two-instruction sequence that calls a routine to configure DMA and interrupt settings.
- `configure_dma_interrupts_6`: Configures DMA channels and interrupt settings by calling two separate routines.
- `configure_hardware_interrupts_2`: A two-instruction routine that calls another routine to set up DMA channels and interrupt settings.
- `apply_dma_interrupt_settings`: Executes a small number of instructions to configure DMA and interrupts.
- `set_dma_interrupt_parameters`: Configures DMA and interrupt hardware settings through a series of instructions.
- `manage_dma_interrupt_hardware`: A routine that writes to DMA control and interrupt enable registers.
- `call_dma_interrupt_configuration`: A five-instruction routine that calls a routine to configure DMA channels and interrupt settings by writing to hardware registers.
- `perform_dma_interrupt_setup`: A ten-instruction routine that configures DMA channels and interrupt settings.
- `call_dma_interrupt_configuration_2`: An unreached routine consisting of six instructions that calls another routine to configure DMA channels and interrupt settings via hardware registers.
- `execute_seven_instructions`: This routine performs seven instructions and returns to its single caller.
- `execute_seven_instructions_2`: The routine performs seven instructions and is invoked by a single unnamed caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine`: This routine is empty and performs no operations before returning to the caller.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `do_nothing`: The routine is empty and performs no operations before returning.
- `perform_no_operation_2`: This routine is empty and performs no actions before returning.
- `perform_no_operation`: This routine is empty and performs no actions.
