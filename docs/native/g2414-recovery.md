# Game 2414 (f-16 combat pilot (1989)(digital integration)[m]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02414). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (938288 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 285 of 285 named (0 author symbols), 285 of 285 described (4 by rule, 279 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1453 of 1453 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 104960 bytes, 22 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5751 of 5921 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 15189 |
| code | 23500 |
| data | 870370 |
| padding | 45 |
| unused | 29184 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| F-16 Combat Pilot (1989)(Digital Integration)[m].adf:bootblock | 1 | 15 | 1 |
| logged disk load from F-16 Combat Pilot (1989)(Digital Integration)[m].adf boot-block load | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 596 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 29 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 44 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 490 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3885 | 234 |
| F-16 Combat Pilot (1989)(Digital Integration)[m].adf bytes 26112-87552 (code never loaded by a reference run) | 1 | 0 | 0 |
| F-16 Combat Pilot (1989)(Digital Integration)[m].adf bytes 116224-125952 (code never loaded by a reference run) | 1 | 826 | 22 |
| F-16 Combat Pilot (1989)(Digital Integration)[m].adf bytes 314368-348160 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- f-16 combat pilot (1989)(digital integration)[m]/F-16 Combat Pilot (1989)(Digital Integration)[m].adf: custom-format (non-DOS) disk, 1701 non-empty blocks: matched against track-loaded segments
- f-16 combat pilot (1989)(digital integration)[m]/F-16 Combat Pilot (1989)(Digital Integration)[m].adf: the boot block reads 8192 bytes from disk offset 15360 to an address (found statically); recovered as a unit
- runtime segment an address: 69 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 27 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 37 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 16 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 166 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1138 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- f-16 combat pilot (1989)(digital integration)[m]/F-16 Combat Pilot (1989)(Digital Integration)[m].adf: bytes 26112-87552 look like code no run loaded: recovered position-independently (load address unknown)
- f-16 combat pilot (1989)(digital integration)[m]/F-16 Combat Pilot (1989)(Digital Integration)[m].adf: bytes 116224-125952 look like code no run loaded: recovered position-independently (load address unknown)
- f-16 combat pilot (1989)(digital integration)[m]/F-16 Combat Pilot (1989)(Digital Integration)[m].adf: bytes 314368-348160 look like code no run loaded: recovered position-independently (load address unknown)
- f-16 combat pilot (1989)(digital integration)[m]/F-16 Combat Pilot (1989)(Digital Integration)[m].adf: 4 logged loads cover 113152 bytes of the image
- f-16 combat pilot (1989)(digital integration)[m]/F-16 Combat Pilot (1989)(Digital Integration)[m].adf: 6 track-loaded blocks found in recovered runtime segments and classified through them
- f-16 combat pilot (1989)(digital integration)[m]/F-16 Combat Pilot (1989)(Digital Integration)[m].adf: blocks 2-6 (bytes 1024-3584) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.30)
- f-16 combat pilot (1989)(digital integration)[m]/F-16 Combat Pilot (1989)(Digital Integration)[m].adf: blocks 8-11 (bytes 4096-6144) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.89)
- f-16 combat pilot (1989)(digital integration)[m]/F-16 Combat Pilot (1989)(Digital Integration)[m].adf: blocks 13-17 (bytes 6656-9216) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.30)
- f-16 combat pilot (1989)(digital integration)[m]/F-16 Combat Pilot (1989)(Digital Integration)[m].adf: blocks 46-48 (bytes 23552-25088) outside the filesystem hold data, not code (opcode density 0.046, entropy 5.09)
- f-16 combat pilot (1989)(digital integration)[m]/F-16 Combat Pilot (1989)(Digital Integration)[m].adf: blocks 174-174 (bytes 89088-89600) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- f-16 combat pilot (1989)(digital integration)[m]/F-16 Combat Pilot (1989)(Digital Integration)[m].adf: blocks 176-225 (bytes 90112-115712) outside the filesystem hold data, not code (opcode density 0.054, entropy 5.63)
- f-16 combat pilot (1989)(digital integration)[m]/F-16 Combat Pilot (1989)(Digital Integration)[m].adf: blocks 250-501 (bytes 128000-257024) outside the filesystem hold data, not code (opcode density 0.030, entropy 5.83)
- f-16 combat pilot (1989)(digital integration)[m]/F-16 Combat Pilot (1989)(Digital Integration)[m].adf: blocks 504-524 (bytes 258048-268800) outside the filesystem hold data, not code (opcode density 0.034, entropy 5.30)
- f-16 combat pilot (1989)(digital integration)[m]/F-16 Combat Pilot (1989)(Digital Integration)[m].adf: blocks 526-609 (bytes 269312-312320) outside the filesystem hold data, not code (opcode density 0.012, entropy 5.03)
- f-16 combat pilot (1989)(digital integration)[m]/F-16 Combat Pilot (1989)(Digital Integration)[m].adf: blocks 705-789 (bytes 360960-404480) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.38)
- f-16 combat pilot (1989)(digital integration)[m]/F-16 Combat Pilot (1989)(Digital Integration)[m].adf: blocks 807-1759 (bytes 413184-901120) outside the filesystem hold data, not code (opcode density 0.014, entropy 6.40)
- 2 never-loaded region(s), 95232 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 5632 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### F-16 Combat Pilot (1989)(Digital Integration)[m].adf:bootblock

- `initialize_boot_io`: Sets hardware color registers and performs an I/O operation via a library call before entering an infinite loop.

### logged disk load from F-16 Combat Pilot (1989)(Digital Integration)[m].adf boot-block load

- `calculate_relative_address_2`: A fifteen-instruction routine that modifies a color register and calls another routine to perform a PC-relative address calculation.
- `calculate_relative_address`: Performs a PC-relative address calculation using five instructions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_state_2`: Clears memory blocks, resets multiple global variables to specific constants, and performs a system trap.
- `store_ascii_digit`: Converts the lower four bits of a byte into an ASCII digit and stores the result at a memory location pointed to by a register.
- `store_ascii_nibble_2`: Takes the lower four bits of a byte, adds an offset to convert it to an ASCII digit, and stores it at a pointer location.
- `store_ascii_nibble`: Extracts the lower four bits of a byte, adds an offset to convert it to an ASCII digit, and stores the result at a memory location pointed to by a register.
- `convert_nibble_to_ascii`: Takes the lower nibble of a byte, adds an offset to convert it to an ASCII character, and stores the result at a pointer location.
- `clear_memory_via_subroutine`: Calls a routine that clears a block of memory twice.
- `initialize_game_state_and_reset_engine`: Resets multiple global variables to zero or specific constants and clears memory blocks.
- `format_countdown_string`: Decrements a value in a loop and writes two ASCII characters to a memory location, representing a character and a colon followed by a value.
- `loop_until_zero`: Repeatedly calls another routine in a loop until a specific global variable reaches zero.
- `reset_global_variables`: Sets several global variables and memory locations to zero or specific constant values.
- `execute_conditional_sequence`: Performs a series of subroutine calls and modifies a global variable based on conditional checks and bitwise operations.
- `calculate_distance_and_branch`: Calculates a value based on the difference between global variables, computes a sum of squares, and calls various routines based on the result or a return value.
- `fill_memory_with_ones`: If a global variable meets specific conditions, it performs a loop that fills multiple memory blocks with the value one.
- `clear_memory_block`: Clears a block of memory by writing zeros to a range of addresses starting from a base pointer offset by a constant value.
- `initialize_pointer_table`: Stores a series of four-byte values from a memory range into a destination table.
- `execute_unknown_routine`: An eight-instruction routine called by one other routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_blocks`: Clears two distinct blocks of memory with zeros and updates several global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_shifted_sum`: Performs a bitwise left shift and addition on values returned from another routine before returning a shifted unsigned short.
- `update_global_with_bit_manipulation`: Performs a sequence of left shifts and conditional XOR operations on a global variable based on its sign bit.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_registers`: Sets multiple hardware registers including audio volumes, DMA control, and interrupt enable registers, while also clearing several memory locations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_state_and_assets`: Initializes several global variables and configuration parameters from a data structure.
- `load_constant_to_register`: Loads a word value into a data register.
- `copy_memory_block`: Copies a block of memory from the address in a0 to the address in a1 one byte at a time, using a dbra loop counter, and returns.
- `parse_compressed_data_stream`: Reads a sequence of bytes from a memory-mapped table to extract and decode bit-packed values.
- `copy_memory_block_2`: Copies a block of bytes from a source pointer to a destination pointer for a fixed count.
- `initialize_global_sequence`: Writes a specific sequence of bytes to a global variable and performs a conditional check on a value at an offset from a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_register_byte`: Stores a single byte from a data register into a global variable.
- `configure_display_and_interrupts`: Sets up hardware registers for bitplane modification, copper instruction timing, and display enable, while also configuring interrupt enable and DMA control bits.
- `execute_infinite_loop`: An infinite loop that prevents further execution.
- `copy_longword_block`: Copies eight consecutive longwords from a source address to a destination address.
- `initialize_system_state`: Clears three memory areas with zeros, updates hardware input, and resets a global variable.
- `clear_multiple_tables`: Clears three separate memory areas by writing zeros to them in a loop.
- `compare_and_update_value`: Compares a value in a register with a global variable and updates that variable if they are not equal.
- `process_hardware_input`: Checks specific bits in a hardware register and updates memory locations based on a calculated index derived from a global variable.
- `set_hardware_bits`: Writes bytes to two calculated memory locations and waits for a specific bit to be set in a hardware register before clearing another register location.
- `update_joystick_coordinates`: Reads joystick or mouse input from hardware registers and updates several global variables and memory locations with the resulting coordinate data.
- `perform_no_operation`: The routine executes a sequence of instructions that results in no state change or functional output before returning.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `loop_on_condition`: Sets a global variable to zero and enters a loop that continues as long as a short integer is greater than zero.
- `perform_complex_loop`: A large routine that sets a global variable to zero and enters a loop based on a short integer value.
- `loop_on_short`: Sets a global variable to zero and enters a loop that continues while a short integer remains positive.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `execute_infinite_loop_2`: The routine enters an endless loop that prevents further execution of the program flow.
- `return_from_subroutine_3`: This routine is empty and performs no operations before returning.
- `update_pointer_tables`: Increments a global variable and uses it to calculate and store several memory offsets and values from a table.
- `return_from_subroutine_4`: The routine is empty and performs no operations before returning.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `execute_complex_state_machine`: Updates a global variable through a series of recursive subroutine calls, conditional branches, and table lookups.
- `execute_multi_stage_indirect_calls`: Executes a series of recursive calls, a loop of indirect calls, and multiple conditional branches that lead to further routine calls and a final assignment to a global variable.
- `process_state_machine_logic`: Updates a global variable through a series of conditional checks, table lookups, and recursive subroutine calls.
- `dispatch_state_machine_transitions`: Updates a global state variable by performing conditional checks, table lookups, and recursive subroutine calls.
- `process_state_machine_transitions`: Updates a global state variable by performing conditional checks, table lookups, and recursive subroutine calls.
- `update_state_machine_logic`: Updates a global state variable by performing conditional checks, table lookups, and arithmetic operations.
- `process_conditional_calls`: Performs a sequence of subroutine calls and modifies a global variable through a table-based lookup and conditional logic.
- `recursively_update_state_machine`: Updates a global state variable through a series of recursive calls and table lookups.
- `set_global_and_branch`: Sets a global variable to a constant and performs conditional branches based on its value before assigning a final value from a table.
- `wait_and_clear_flag`: waits for a specific memory location to become non-zero and then clears it.
- `recurse_until_null`: Recursively calls itself as long as a global variable is non-zero.
- `clear_status_flag`: Sets a global variable to zero and clears a specific byte in memory within a loop.
- `clear_data_fields`: stores zero into an offset of a pointer and then updates a global variable with zero.
- `execute_data_copy_loop`: Performs a series of calls and arithmetic operations on a global variable within a loop.
- `call_indirect_pointer`: loads a pointer from a global variable and executes the routine at that address.
- `copy_block_of_data`: copies a block of data from a source to a destination using a loop with a fixed count.
- `copy_large_block`: copies a large sequence of words from a source to a destination using a loop.
- `clear_memory_block_2`: Clears a specific block of memory by writing zeros to a range of addresses starting from an offset relative to a pointer passed in a register.
- `format_numeric_string`: Converts an integer into a decimal ASCII string and stores it in a memory buffer.
- `convert_integer_to_string`: Converts a numeric value into a sequence of ASCII characters stored in a memory buffer.
- `dispatch_indirect_call`: Iterates through a pointer until a specific character condition is met, then executes a function from a jump table.
- `return_empty_stack`: The routine returns without performing any operations or modifying any registers.
- `dispatch_indirect_call_2`: Iterates through a memory buffer until a specific character is found, then executes a function from a jump table.
- `execute_command_sequence`: Iterates through a sequence of bytes, executing a routine for each byte that meets a specific character condition.
- `execute_indirect_jump_table`: Iterates through a sequence of bytes, recursively calling itself for each byte that meets a specific bitwise condition.
- `traverse_and_dispatch_table`: Iterates through a sequence of bytes, storing each byte into a global variable and recursively calling itself until a byte with a value of thirty-one or less is encountered.
- `execute_command_sequence_2`: Iterates through a sequence of bytes, executing a routine for each byte that meets a specific character condition.
- `dispatch_indirect_call_3`: Handles word-aligned pointers to iterate through a buffer and execute a function from a jump table.
- `nop_sequence_execution`: The routine executes a sequence of fourteen instructions that perform no functional state changes or data transfers.
- `update_global_with_pointer_call`: calls a function through a pointer derived from a global variable and then adds a value to that global variable with bounds checking.
- `update_bitmask_buffer`: Performs bitwise logical operations to update specific bytes within a buffer using values retrieved from a lookup table.
- `copy_data_block`: Copies eight consecutive longwords from a source address to a destination address and then enters an infinite loop.
- `execute_infinite_loop_sequence`: Calls another routine three times before entering an infinite loop.
- `perform_memory_operations`: Copies a block of eight longwords, clears a memory block, updates a global variable, and clamps two values within specific ranges.
- `execute_nested_loop_calls`: Repeatedly executes a routine through a pointer in a global variable within a loop structure.
- `process_bitstream_data`: performs a complex nested loop that manipulates bits from a source buffer to populate a table of sixteen-bit values.
- `copy_memory_blocks`: copies data from a source buffer to a destination buffer, handling signed length values by either copying forward or repeating a single byte.
- `update_global_from_offset_2`: Loads a byte from an offset relative to a register into a global variable and enters a loop that calls another routine multiple times.
- `loop_until_non_zero`: Repeatedly calls a routine while a global variable remains non-zero.
- `loop_and_reset_global`: Executes a routine in a loop while a global variable is non-zero, then clears the global variable and calls another routine.
- `execute_indirect_function`: Executes a function retrieved from a pointer stored in a global variable.
- `perform_no_operation_2`: This routine is empty and performs no actions before returning.
- `process_nested_calls`: Performs a sequence of recursive calls and executes a function through a pointer in a global variable.
- `nop_placeholder`: The routine is empty and performs no operations.
- `execute_sequence_of_calls`: Executes a series of subroutines and resets a global variable to zero.
- `reset_and_call_subroutines`: Sets a global variable to zero and then executes a sequence of seven consecutive subroutine calls.
- `update_input_status`: Updates a global variable with system input and performs conditional increments based on register results.
- `increment_on_error`: Increments a global variable if a software interrupt results in a negative value.
- `execute_sequential_empty_calls`: Calls several empty routines in succession before returning to the caller.
- `update_global_value`: Triggers a software interrupt and updates a global variable with the resulting value if it is non-negative, otherwise increments the global variable.
- `increment_global`: increments a global variable by one.
- `get_system_input`: triggers a software interrupt and updates a global variable with the returned value if it is non-negative, otherwise increments the global variable.
- `increment_global_on_negative_result`: Triggers a software interrupt and increments a global variable if the resulting value in a data register is negative.
- `increment_global_on_trap_error`: Triggers a software interrupt and increments a global variable if the resulting value in a register is negative.
- `increment_on_negative`: Triggers a software interrupt and increments a global variable only if the resulting value in a register is negative.
- `increment_global_value`: Increments a global variable by one.
- `increment_global_on_trap_error_2`: Triggers a software interrupt and increments a global variable only if the resulting value in a data register is negative.
- `increment_global_variable`: Increments the value of a global variable by one.
- `increment_on_double_exception`: Triggers two software interrupts and increments a global variable if the second interrupt result is non-zero.
- `store_and_increment_on_exception`: Stores a value from a register into a global variable, triggers two software interrupts, and increments the global variable if the second interrupt result is non-zero.
- `increment_on_trap_error`: Executes a software interrupt and increments a global variable only if the resulting value in a data register is negative.
- `increment_global_variable_2`: Increments a single global variable by one.
- `loop_and_copy_data`: Enters an infinite loop after copying data from a source to a destination and storing the iteration count in a global variable.
- `copy_and_rotate_bitmaps`: Copies data blocks while counting iterations and performs bitwise circular rotations and inversions on bitmap data.
- `rotate_bitmaps`: Performs a bitwise circular rotation on blocks of data.
- `copy_data_blocks`: Copies data from a source to a destination using nested loops and specific offsets.
- `xor_data_blocks`: Performs a bitwise XOR operation between two data blocks using nested loops.
- `copy_and_count_blocks`: Copies data from a source to a destination and stores the number of iterations in a global variable.
- `copy_data_with_limit`: Copies data from a source to a destination and stores the iteration count in a global variable up to a limit.
- `copy_memory_blocks_2`: Copies data from one memory location to another using nested loops and specific offsets.
- `merge_bitmapped_data`: Performs a nested loop to merge bytes from a source buffer into a destination buffer using bitwise OR and AND operations with a calculated mask.
- `copy_structured_data`: Performs a nested loop to copy specific words from a source table to a destination table based on two counter values.
- `copy_memory_block_5`: Performs a nested loop to copy data from a source buffer to a destination buffer.
- `rotate_and_mask_bitmaps`: Performs a bitwise circular rotation on blocks of data and calculates their bitwise inverse.
- `perform_bit_shift_array_copy`: Copies data from a source buffer to a destination buffer while performing bitwise left or right shifts on each word.
- `set_global_values`: Stores specific constant values into a global variable based on a bit test of a register.
- `dispatch_pointer_data_2`: Iterates through a data structure to execute a routine via an indirect jump based on a calculated index.
- `update_global_state_2`: Performs a bitwise OR operation on a global variable and executes a routine through an indirect jump.
- `dispatch_pointer_data`: Processes a pointer from a register, iterates through a data structure, and executes a routine through an indirect jump.
- `traverse_data_structures`: Calculates an index, iterates through a list of pointers, and executes a function retrieved from a jump table.
- `process_pointer_list`: Iterates through a table of pointers, performs bitwise rotations on bitmap data, and executes a routine via an indirect jump table.
- `update_global_pointer`: Traverses a series of pointers stored in a global variable and performs arithmetic on a value before loading a new pointer from a table.
- `execute_indirect_jump_table_2`: Iterates through a list of pointers until a termination condition is met, then executes a function retrieved from a jump table via an indirect call.
- `iterate_pointer_list`: Iterates through a list of pointers stored in a global table and calls another routine for each entry until a terminator is reached.
- `initialize_data_copy_parameters`: Resets a global variable to zero and uses it as a pointer to store its own value into a memory location.
- `copy_and_modify_table`: Iterates through a source table to copy specific word offsets into a destination table while applying conditional modifications to the source data.
- `copy_table_blocks`: Performs a nested loop to copy blocks of data from a source table to a destination table using word-sized increments.
- `scan_table_for_terminator`: Iterates through a table of integers, returning when it encounters a specific termination value or a specific counter limit.
- `process_table_data`: Iterates through a table of words, performing bitwise operations and comparisons on elements until a termination value is encountered.
- `search_list_for_range_match`: Iterates through a list of four-word structures until it finds an entry where a global variable's doubled value is contained within the bounds defined by the second and fourth words of the structure, or until a terminator value is encountered.
- `update_global_value_2`: Compares a value in a register to a global variable and updates the global variable if they differ.
- `check_and_branch_value`: Compares a register to a global variable and, if they are not equal, updates the global variable and calls another routine twice.
- `update_global_flags`: Modifies a global variable and another memory location based on the state of several individual bytes.
- `clamp_and_call_pointer`: Clamps a global variable between zero and two hundred fifty-five, then uses the value to call a function pointer.
- `check_and_clear_flag`: Checks a global variable, clears it if it is non-zero, and then calls another routine.
- `update_clamped_values`: Updates two separate values by clamping them within specific ranges and recursively calls itself if the values change.
- `reset_flag_and_recurse`: Checks a global byte for a non-zero value; if set, it clears the byte and calls itself recursively before returning.
- `transfer_global_data`: Copies a block of data from a source to a destination using a loop with a fixed count.
- `recurse_and_assign_global`: The routine calls itself recursively, performs a redundant assignment of a global variable to itself, and then calls itself again before returning.
- `update_bitmask_data`: Calculates a destination address using offsets and base pointers to perform bitwise AND and OR operations on multiple memory locations.
- `compute_signed_distance_and_loop`: Calculates the signed difference between two pairs of values and enters a recursive loop that decrements a value until it reaches zero.
- `wait_for_zero`: Enters a loop that continues as long as a global variable is non-zero.
- `configure_dma_transfer`: Sets up memory addresses and control values in specific hardware registers and global variables to prepare for a DMA operation.
- `initialize_data_pointers`: Stores several memory addresses and constant values into a series of global variables.
- `update_interrupt_registers`: Writes two values to a global variable and modifies the interrupt enable register.
- `modify_global_state`: Performs bitwise operations on a global variable and updates another global variable based on a bit test.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 128 into d5, then merges 128 into d5.
- `execute_indirect_via_offset`: Sets a global variable, enters a loop until it becomes zero, and then executes a routine using a pointer from a table at a calculated offset.
- `set_constant_value`: Calls another routine and then stores the value ten into a global variable.
- `loop_indirect_calls`: Repeatedly executes a function through a pointer in a global variable within a loop.
- `execute_conditional_indirect_calls`: Performs recursive calls and a loop of indirect calls through a table at a calculated offset, contingent on a bitwise check of a global variable.
- `perform_no_operation_3`: This routine is empty and performs no actions.
- `set_constant_value_nine`: Calls another routine and then stores the value nine into a global variable.
- `execute_indirect_loops`: Executes a series of nested loops and conditional branches that include multiple indirect function calls and a final call through a calculated offset in a table.
- `execute_looping_indirect_calls`: Executes a loop of indirect calls and a final indirect call to a function at a calculated offset.
- `check_global_value`: Compares a global variable against a constant and executes a routine if the condition is met.
- `execute_recursive_indirect_calls`: Performs multiple recursive calls and a loop of indirect calls through a pointer in a global variable, ending with a call to a function at a calculated offset.
- `dispatch_indirect_calls`: Performs multiple indirect calls by traversing pointers and searching a list of structures for a range match based on a global variable.
- `execute_indirect_call_3`: Performs an indirect call by traversing a pointer sequence until a termination condition is met, then jumps to a function address retrieved from a table.
- `execute_nested_conditional_calls`: Performs recursive calls, a loop of indirect calls, and a final indirect call at a calculated offset, with conditional logic based on a global variable.
- `set_constant_value_one`: Calls another routine and then stores the value one into a global variable.
- `process_table_and_recurse`: Sums bytes from a memory block, performs recursive calls, and executes indirect calls based on a table scan and a character comparison.
- `execute_infinite_recursion`: The routine calls itself twice before returning, resulting in an infinite loop of recursive calls.
- `process_indirect_jump_table`: Performs bitwise operations on a global variable and uses a calculated index to execute a routine from a jump table.
- `dispatch_table_calls`: Executes a series of indirect calls and table dispatches based on scanning a table for a terminator and checking register values.
- `execute_indirect_dispatch`: Iterates through a pointer to find a specific character condition and then executes a function from a jump table.
- `compare_memory_ranges`: Compares bytes in two memory locations using a loop and sets a global variable before potentially entering infinite recursion.
- `execute_complex_indirect_chain`: Executes a sequence of recursive calls, a loop of indirect calls, and a final indirect call using a value from a register and a base offset.
- `execute_indirect_call_2`: Performs an indirect function call using an address calculated from a register and a base offset.
- `return_no_op`: The routine consists of a single return instruction and performs no work.
- `nop_padding_sequence_2`: This routine consists of three instructions that perform no functional operation, serving as a data gap or padding within the executable segment.
- `fill_memory_block`: Fills a block of four memory locations starting at a specific address with the value one.
- `conditional_memory_fill`: Iterates through a table and writes two bytes of one value to specific memory locations if the existing bytes do not match certain criteria.
- `populate_jump_table`: Iterates through a byte array to populate a table of word values by adding a global variable to values found at specific offsets.
- `branch_on_byte_bits`: Loads a byte into a global variable and performs conditional recursive calls based on the bitwise state of that byte.
- `assign_global_values`: Stores several constant values into a global variable and performs recursive calls.
- `process_indirect_jump_table_alt`: Modifies a global variable using a bitwise mask and executes a routine via an indirect jump based on a calculated offset.
- `call_conditional_function`: Checks if two returned values are below specific thresholds before calling a function through a pointer stored in a global variable.
- `execute_infinite_recursion_2`: The routine calls itself repeatedly without a base case or termination condition.
- `traverse_nested_list`: Iterates through a data structure using a pointer, checking for specific termination values to control loop flow and recursion.
- `transform_global_value`: Performs a series of arithmetic shifts and subtractions on a global variable.
- `calculate_scaled_offset`: Multiplies the input value by four and adds nine.
- `loop_and_call_routines`: Iterates through a table using a scaled offset, checking for specific byte values before calling another routine multiple times.
- `iterate_and_call_routines`: Iterates through a data table up to five times, calling another routine multiple times if a specific byte value is encountered.
- `execute_nested_loops`: Performs nested loops involving bitwise shifts and checks a memory location via a pointer to conditionally call another routine.
- `validate_index_value`: Checks a byte from a memory location against specific values and performs a decrementing loop if a condition is met.
- `apply_bitmask_to_buffer`: Calculates a destination address using a base pointer, a global variable, and an offset derived from input parameters.
- `call_other_routine_twice`: Calls another routine two times in succession.
- `call_other_routine_six_times`: Calls another routine six times in succession.
- `execute_nested_loop_calls_2`: Performs multiple recursive calls and a loop of nested indirect calls, concluding with an indirect call to a function at a calculated offset.
- `execute_indirect_call_4`: Iterates through a list of four-word structures to find a range match and then calls a routine via a pointer offset.
- `set_global_flag`: Calls another routine and then sets a global variable to one.
- `load_routine_address`: Stores the address of another routine into a global variable and then calls that routine twice.
- `update_global_state`: Modifies a global variable with specific constants and executes another routine based on a comparison.
- `execute_recursive_pointer_calls`: Executes recursive calls and a loop of indirect calls through a pointer in a global variable, finishing with a call to a calculated offset.
- `reset_global_pointer`: Sets a global variable to its own address and then clears it through a series of assignments.
- `transform_xor_data`: Iterates through a memory range and performs bitwise XOR operations on data blocks based on a condition.
- `xor_data_blocks_2`: Performs a bitwise XOR operation between two blocks of data.
- `execute_xor_data_transformation`: Triggers a bitwise XOR operation between two memory blocks.
- `update_bitfield_table`: Iterates through a table to clear or set bytes based on bits in a global variable and a comparison with another byte.
- `fetch_and_accumulate_data`: Fetches a longword from a memory location calculated from a global variable and then performs a series of arithmetic accumulations into a global variable.
- `fetch_indirect_value`: Performs arithmetic on a global variable to calculate an address and fetches a longword from that location.
- `calculate_indirect_address`: Performs arithmetic on a global variable and uses the result to fetch a value from a calculated memory offset.
- `execute_indirect_calls`: Iterates through a pointer until a specific character condition is met, then executes functions from a jump table and copies data blocks.
- `initialize_global_flags`: Sets a global variable to a specific address, performs bitwise OR operations, and clamps the result to a minimum value.
- `loop_fixed_iterations`: Executes a loop that runs ten times, interspersed with several recursive calls.
- `process_conditional_data`: Checks values at various memory locations and performs a loop that calls another routine based on calculated offsets and multipliers.
- `shift_and_copy_buffer`: Copies data from a source buffer to a destination buffer while applying a bitwise shift to each word.
- `merge_bitmapped_data_2`: Performs a nested loop to merge bytes from a source buffer into a destination buffer using bitwise OR and AND operations.
- `return_immediately_stub`: The routine consists only of a return with no other operations, so it performs no work when called.
- `populate_table_from_data`: Iterates through a data table to check bit flags and stores bytes into a destination table based on specific conditions.
- `populate_table_with_offset`: Iterates through a data table and stores values into a destination table at incrementing word offsets based on bitwise checks.
- `update_bit_flags`: Iterates through a table to set bits in a global variable based on values found in the table.
- `calculate_accumulated_values`: Clears a table of words and then performs a loop to accumulate products and sums into a global variable.
- `store_loop_results`: Executes a loop fifteen times, storing a word value into a memory location at a specific offset during each iteration.
- `execute_conditional_call`: Calls another routine and performs subsequent actions based on the returned byte value.
- `reset_global_sequence`: Performs bitwise shifts and multiple calls to other routines while resetting a global variable through a sequence of assignments.
- `update_table_entries_2`: Iterates through a table backwards, updating bytes at specific offsets using values from another routine and a matching pair search.
- `initialize_data_buffers`: Fills multiple memory offsets within a structure with specific byte values and updates them via another routine.
- `update_table_entries`: Iterates through a table backwards, updating bytes at specific offsets using values from another routine and a matching pair search.
- `fill_buffer_with_constants`: Writes constant byte values into four different offsets of a memory block using a loop.
- `search_matching_pairs`: Iterates through memory to find a match between two sets of values derived from bitwise operations and indirect calls.
- `copy_memory_block_3`: Copies a block of 148 bytes from one memory location to another.
- `subtract_from_global`: Subtracts two constant values from a global variable.
- `add_to_global`: Adds two constant values to a global variable.
- `copy_log_string`: Copies a string from one memory location to another and appends a specific suffix.
- `write_dot_elog`: Writes the string '.E LOG' into a memory location pointed to by a register.
- `copy_gm_string`: Copies characters from a source address to a destination address and appends a short string.
- `write_dot_gm`: Writes the string '.GM' into a memory location pointed to by a register.
- `convert_ascii_string_to_integer`: Iterates through a pointer-based string of characters, accumulating a numeric value by multiplying the current total by ten and adding the difference between the character and the digit zero.
- `find_word_in_table`: Iterates through a table of word-sized elements until a match is found, then returns the value following that match.
- `find_word_in_list`: Iterates through a list of words until a match for a given value is found, then returns the following word.
- `dereference_offset_pointer`: Returns a longword located at an offset from a pointer stored in a register.
- `check_flag_and_assign`: Tests a bit in a global variable and, if set, assigns specific constant values to that variable.
- `search_and_load_bytes`: Searches a table for a matching byte pair and, if found, loads three consecutive bytes from the table into a global variable.
- `update_global_from_offset`: Loads three consecutive bytes from an offset of a pointer into a global variable.
- `search_and_decrement_table_entry`: Iterates through a table using an index derived from another routine, checking a byte at an offset and decrementing it if it is non-zero.
- `initialize_data_block`: Fills a series of memory blocks with zero bytes and word-sized maximum values.
- `increment_table_values`: Performs a series of calculations and then loops ten times to find a valid entry in a table.
- `increment_table_entry_with_validation`: Loops through a table to find an entry where a specific byte is greater than eight and not equal to negative one, then increments that byte.
- `increment_table_entry_simple`: Searches a table for an entry that satisfies specific byte conditions and increments the value at a calculated offset.
- `calculate_and_copy_data`: Computes a value based on the results of two other routines and then copies a block of memory of a fixed size to a new location.
- `copy_memory_block_4`: Copies a specific number of bytes from a source location to a destination starting at a global variable.
- `update_pointer_via_offset`: Loads a word from a memory location at a fixed offset relative to a global variable and stores the result back into that global variable.
- `execute_indirect_call`: Calculates an address using a global variable and a base offset, then calls the resulting function pointer.
- `update_global_status_flags`: Assigns values to a global variable and performs several recursive calls to other routines.
- `execute_indirect_jump`: Calls another routine and then performs an indirect call to a function pointer located at an offset derived from a global variable.
- `update_status_flags`: Checks multiple global variables and updates a global value based on their non-zero status and the result of another routine.
- `set_byte_by_threshold`: Compares a global variable against a threshold and sets a byte at a specific memory location to either one or two.
- `set_byte_if_not_two`: Checks if a byte at a specific index is not two, and if so, sets it to one.
- `count_set_bits_in_data`: Retrieves data from a table and counts how many bits are set within a specific bitmask, then uses the count to select a value from another table.
- `accumulate_boolean_results`: Calls other routines to retrieve bitwise results and stores a combined value into a global variable.
- `process_conditional_tables`: Fills memory, conditionally writes bytes to a table based on existing values, and performs table searches and decrements.

### F-16 Combat Pilot (1989)(Digital Integration)[m].adf bytes 116224-125952 (code never loaded by a reference run)

- `sort_and_copy_list`: Finds the minimum value in a list of paired elements and performs a memory shift to reorganize the data.
- `compare_signed_integer_ranges`: Compares two signed values against a stored range defined by two previously held values.
- `add_repeatedly_until_limit`: Repeatedly adds a value to another in a loop and stores the resulting value in a memory location.
- `increment_value_conditionally`: Increments a value and stores it in a memory location if a specific comparison condition is met.
- `subtract_repeatedly_until_limit`: Repeatedly subtracts a value from another in a loop and stores the resulting value in a memory location.
- `update_memory_via_mask`: Updates a memory location with a value derived from arithmetic operations based on conditional checks.
- `apply_memory_mask`: Applies a bitwise AND mask to a range of memory words.
- `mask_memory_range`: Applies a bitwise AND mask to a range of memory words, using values from a table to determine the mask.
- `clear_and_mask_memory_blocks`: Clears a range of memory by writing zeros to specific word offsets within a sequence of blocks, then applies a bitwise AND mask to the first and last blocks of that sequence.
- `mask_memory_descending`: Iterates backwards through a memory range, applying a bitwise AND mask to each word and its associated offsets.
- `mask_memory_values`: Performs a bitwise AND operation between a 16-bit register and four specific word locations in memory pointed to by a register.
- `mask_memory_strided`: Applies a bitwise AND mask to a sequence of words at specific offsets within a memory range.
- `mask_memory_ascending`: Iterates forwards through a memory range, applying a bitwise AND mask to each word and its associated offsets.
- `mask_memory_values_2`: Applies a bitwise AND mask to four words at fixed offsets from a pointer held in a register.
- `mask_memory_values_3`: Updates four memory words by performing a bitwise AND with a value from a register.
- `return_from_subroutine`: Performs a single return instruction to exit the current execution flow.
- `compare_and_branch_values`: Compares multiple values and performs conditional returns based on the results of signed arithmetic differences.
- `apply_bitwise_mask`: Performs bitwise AND operations on structured data using inverted values retrieved from a source table.
- `return_to_caller`: The routine is empty and performs no operations before returning to the calling routine.
- `apply_complex_mask`: Uses a loop to apply bitwise masks to memory words based on values retrieved from a table.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning to its caller.
- `update_memory_bitmask_tables`: Updates multiple sets of global memory locations with one of two possible 16-bit values based on the bit state of a register.
