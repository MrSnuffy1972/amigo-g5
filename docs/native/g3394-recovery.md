# Game 3394 (dizzy diamonds (19xx)(titanics entertainment)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03394). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 207 of 207 named (0 author symbols), 207 of 207 described (80 by rule, 69 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 10 of 10 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5695 of 5695 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 16360 |
| code | 23924 |
| data | 211124 |
| padding | 4080 |
| unused | 645632 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| delete | 1 | 600 | 3 |
| endcli | 2 | 208 | 3 |
| Echo | 2 | 241 | 10 |
| copy | 2 | 1685 | 5 |
| makedir | 1 | 112 | 1 |
| Cd | 2 | 494 | 12 |
| loadwb | 12 | 537 | 30 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 1805 | 26 |
| Icon.Library | 10 | 1421 | 110 |
| Dizzy Diamonds (19xx)(Titanics Entertainment).adf:DIZZY.EXE | 1 | 221 | 5 |
| Dizzy Diamonds (19xx)(Titanics Entertainment).adf:bootblock | 1 | 18 | 1 |

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### delete

- `delete_file_entry`: Attempts to delete a file using a specified name and flags, then reports the result.
- `delete_file_entry_2`: Attempts to delete a file entry and reports whether the operation succeeded or failed.
- `compare_strings_case_insensitive`: Compares two null-terminated strings pointed to by a0 and a1, treating uppercase and lowercase letters as equal, and returns the signed difference of the first non-matching characters or zero when they match up to the terminator.

### endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `initialize_game_state`: Sets up initial game data by modifying a global variable, performing arithmetic operations on registers, and calling another routine multiple times with different parameters.

### Echo

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value, writes a header with that value, fills a buffer with sequential words, and sets up a stack frame for a subsequent call.
- `frag_move_bra_2`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `update_sprite_position`: Updates sprite position and boundary values in a structure based on configuration data from a table, then performs conditional operations depending on the data read.
- `update_sprite_data`: Reads sprite configuration data from a table using an index, updates position and boundary values in a structure, and performs conditional calculations based on the read data.
- `frag_move_bra_3`: Short code fragment that copies the memory at an offset from a1 to d1, then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `update_position`: Updates a position value by adding an increment and clamping it between a minimum and maximum boundary, then stores the result in multiple locations.
- `frag_move_bra_4`: Short code fragment that copies the memory at an offset from a1 to d1, then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `update_position_data`: Copies position values between memory locations, performs comparisons and shifts on data, then calls a routine with specific parameters before branching to another location.
- `frag_move_lsl_tst_bne_moveq_movea_jsr_b`: Short code fragment that copies the memory at a1 to d1, then shifts d1 left by 2, then tests an indexed entry from a0, then branches if the last result was not zero, then loads 10 into d1, then copies 352 to d0, then copies the memory at an offset from a2 to a4, then calls the routine whose address is in a5, then jumps to another point in the code.
- `conditional_execute_routine`: Tests a global variable for zero value and branches if true, otherwise calls routines with different parameters based on additional conditions, then jumps to an address in a register.

### copy

- `perform_file_copy`: Validates source and destination file paths, checks for directory existence, and manages file handles and buffers to copy data between files.
- `call_two_callbacks_with_flag`: If the value in D0 is non-zero, it calls a callback via a pointer in A1 at offset -126 of the library base, then calls another routine from the same library at offset -828.
- `call_function_table`: The routine receives a count in D0 and a pointer to a table in A0.
- `check_bitmask_range`: The routine tests whether an input value in D0 lies within a numeric range and whether a specific bit in a bitmask is set, returning a flag indicating the result.
- `compare_strings_case_insensitive_2`: Compares two null-terminated strings pointed to by a0 and a1, treating uppercase and lowercase letters as equal, and returns the signed difference of the first non-matching characters or zero when they match up to the terminator.

### makedir

- `set_error_and_cleanup`: The routine initializes an error code and, if a library base is present, it clears a small stack buffer, opens a DOS library context, and iterates over a null-terminated list of strings.

### Cd

- `find_max_and_prepare_buffer_3`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra_5`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `initialize_game_state_4`: Sets up initial game state values in memory, including clearing and writing specific offsets, and performs several subroutine calls with parameters derived from memory locations.
- `initialize_game_state_2`: Reads a value from memory location pointed to by a1, shifts it left by two bits, tests a memory entry at offset zero from a0 plus the shifted value, and branches to a specific label if the entry is zero.
- `process_game_logic`: Loads a value from memory at offset 608 of a2, calls a routine with specific parameters, and branches to another label if the loaded value is not zero.
- `check_and_modify_flags_2`: Compares two values from memory locations relative to a1, modifies flags based on the comparison result, performs bitwise operations, and stores modified data.
- `update_and_store_values`: Updates values in memory locations relative to a1, performs arithmetic operations on data from memory, and stores results at specific offsets.
- `copy_and_process_data`: Copies data from memory locations relative to a1 and a2, performs arithmetic operations on the values, and stores results at specific offsets in the structure pointed to by a1.
- `frag_move_moveq_movea_jsr_tst_bne_jmp`: Short code fragment that copies the memory at an offset from a1 to d1, then loads 48 into d0, then copies the memory at an offset from a2 to a4, then calls the routine whose address is in a5, then tests the memory at an offset from a1, then branches if the last result was not zero, then loads 0 into d1, then jumps to the address in a6.
- `compare_and_modify_values`: Compares two values from memory locations relative to a1, modifies flags based on the comparison result, and performs bitwise operations on stored data.
- `update_and_compare`: Updates a counter value at an offset from a1, reads a byte from memory using a calculated index, compares it with a constant, and branches based on the comparison result.
- `frag_move_lsl_moveq_add_jmp`: Short code fragment that copies the memory at an offset from a1 to d1, then shifts d1 left by 2, then loads 0 into d2, then copies an indexed entry from a0 to d2, then adds the memory at an offset from a1 to d2, then copies the memory at an offset from a1 to d3, then shifts d3 left by 2, then copies d2 to an indexed entry from a0, then loads 255 into d1, then jumps to the address in a6.

### loadwb

- `initialize_workbench`: Initializes the workbench environment by allocating memory, opening libraries, and setting up global pointers.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `start_workbench_tool`: Decodes an XOR-scrambled string table in place, allocates a block and runs a long sequence of library calls to prepare and launch the tool.
- `frag_movea_bra`: Short code fragment that copies the memory at an offset from a7 to a1, then copies the memory at an offset from a7 to a0, then jumps to another point in the code.
- `build_linked_list_from_table`: Walks a table of pairs, allocating a 16-byte node for each entry, storing a looked-up value in the node and linking consecutive nodes through their first longword, and returns the head of the list or a partial list if an allocation or lookup fails.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `frag_move_pea_jsr_lea_rts`: Short code fragment that copies the memory at an offset from a7 to d0, then copies the memory at an offset from a7 to d1, then pushes the address of the memory at an offset from a7 onto the stack, then copies d1 to the stack, then copies d0 to the stack, then calls another routine, then loads the address of the memory at an offset from a7 into a7, then returns.
- `format_string_via_rawdofmt_then_output`: Calls exec RawDoFmt with a format template and arguments into a 140-byte stack buffer, measures the resulting string length and passes it to the output routine.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `validate_input_byte`: Checks if a byte at a specific offset in a structure matches a constant value, and if not, exits early.
- `frag_move_bra_6`: Short code fragment that copies 46 to the memory at an offset from a2, then copies 32 to the memory at an offset from a2 (4 times in a row), then jumps to another point in the code.
- `frag_movea_lea_tst_beq_cmpm_bne_bra`: Short code fragment that copies a2 to a0, then loads the address of a location in the code into a1, then tests the memory at a1, then branches if the last result was zero, then compares the memory at a1 (advancing a1) with the memory at a0 (advancing a0), then branches if the last result was not zero, then jumps to another point in the code.
- `frag_lea_movem_move_rts`: Short code fragment that loads the address of a location in the code into a2, then restores saved registers from the stack, then copies a location in the code to the stack, then returns.

### Disk-Validator

- `initialize_game_state_3`: Sets up initial game parameters including screen buffer pointers, clears memory areas, and configures data structures for rendering.

### Ram-Handler

- `frag_ori_movem_movea_move_jsr_suba_jmp`: Short code fragment that merges 12 into d0, then saves registers on the stack, then copies d1 to a0, then copies d2 to a1, then copies d3 to d0, then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then subtracts a0 from a0, then jumps to the address in a6.
- `calculate_and_store_indices`: Calculates new values by shifting and adding data from memory locations relative to a1 and a0, stores results at specific offsets in the structure.
- `calculate_and_store_values`: Calculates new values by shifting and adding data from memory locations relative to a1 and a0, stores results at specific offsets in the structure pointed to by a1, and performs conditional jumps based on comparisons.
- `frag_move_lsl_cmp_bge_moveq_jmp`: Short code fragment that copies the memory at an offset from a1 to d1, then shifts d1 left by 2, then copies an indexed entry from a0 to d2, then compares d2 with the memory at an offset from a1, then branches if the comparison was greater or equal, then copies 219 to the memory at an offset from a2, then loads 255 into d1, then jumps to the address in a6.
- `frag_tst_bne_move_lsl_moveq_jmp`: Short code fragment that tests the memory at an offset from a1, then branches if the last result was not zero, then copies the memory at an offset from a1 to d1, then shifts d1 left by 2, then tests an indexed entry from a0, then branches if the last result was not zero, then loads 0 into d1, then jumps to the address in a6.
- `frag_move_lsl_tst_bne_moveq_jmp`: Short code fragment that copies the memory at an offset from a1 to d1, then shifts d1 left by 2, then copies an indexed entry from a0 to the memory at an offset from a1, then tests the memory at an offset from a1, then branches if the last result was not zero, then copies 219 to the memory at an offset from a2, then loads 255 into d1, then jumps to the address in a6.
- `update_memory_locations_2`: Updates multiple memory locations using calculated indices from data at a1, performs arithmetic operations on values, and stores results in specific offsets.
- `update_memory_locations`: Updates multiple memory locations using calculated indices from data at a1, performs arithmetic operations on values, and stores results in arrays indexed by a0.
- `compute_and_store_coordinates`: Computes coordinate values by adding and shifting data from a1, stores the results at specific offsets, and calls another routine with a constant parameter.
- `frag_move_moveq_lea_jsr_tst_bne_jmp`: Short code fragment that copies the memory at an offset from a1 to d1, then loads 124 into d0, then loads the address of the memory at an offset from a4 into a4, then calls the routine whose address is in a5, then copies d1 to the memory at an offset from a1, then tests d1, then branches if the last result was not zero, then loads 0 into d1, then jumps to the address in a6.
- `load_and_compare_values`: Loads values from specific offsets in memory relative to a1, calls routines with different parameters, and performs arithmetic operations on stored values before branching.
- `initialize_and_process_2`: Initializes data values from memory locations relative to a1, calls routines with different parameters, and performs arithmetic operations on stored values.
- `check_and_modify_flags`: Compares two values from memory locations relative to a1, modifies flags based on the comparison result, performs bitwise operations, and stores modified flags at an offset in the structure pointed to by a1.
- `frag_move_lsl_moveq_cmp_bne_jmp`: Short code fragment that copies the memory at an offset from a1 to d1, then shifts d1 left by 2, then loads 253 into d2, then compares d2 with an indexed entry from a0, then branches if the last result was not zero, then copies 212 to the memory at an offset from a2, then loads 0 into d1, then jumps to the address in a6.
- `frag_moveq_move_lea_jsr_tst_bne_jmp`: Short code fragment that loads 254 into d2, then copies the memory at an offset from a1 to d1, then loads 124 into d0, then loads the address of the memory at an offset from a4 into a4, then calls the routine whose address is in a5, then copies d1 to the memory at an offset from a1, then tests d1, then branches if the last result was not zero, then loads 0 into d1, then jumps to the address in a6.
- `check_and_jump`: Loads a long integer from a location relative to a1, sets up a register with a constant, adjusts a4 to point to a location relative to itself, calls a routine through a5, tests the loaded value for zero, and branches to another address if it is not zero.
- `load_and_branch`: Loads a long integer from a memory location relative to a1, compares it with another value at a different offset of the same pointer, and branches to a specific address if they are not equal.
- `copy_and_modify_data`: Copies data between memory locations using calculated indices, performs arithmetic operations on stored values, and modifies flags based on comparison results.
- `initialize_and_process`: Initializes data values from memory locations relative to a1, calls routines with different parameters, and performs arithmetic operations on values before storing results at specific offsets in the structure pointed to by a1.
- `validate_and_update`: Loads data from memory using an index, performs bit manipulation on the loaded value, stores the result at an offset in the structure pointed to by a1, and validates the stored value against constants.
- `frag_moveq_movea_jsr_move_tst_bne_jmp`: Short code fragment that loads 12 into d1, then loads 36 into d0, then copies the memory at an offset from a2 to a4, then calls the routine whose address is in a5, then copies d1 to the memory at an offset from a1, then tests d1, then branches if the last result was not zero, then copies 221 to the memory at an offset from a2, then loads 0 into d1, then jumps to the address in a6.
- `update_and_compare_coordinates`: Updates coordinate values from memory locations relative to a1, performs arithmetic operations on these values, stores results at specific offsets in the structure pointed to by a1, and compares values with constants.
- `load_and_process_data`: Loads data from memory using an index, shifts the loaded value left by two bits, stores a value from a table at an offset in the structure pointed to by a1, and calls another routine with a constant parameter.
- `frag_move_lsl_clr_tst_bne_moveq_bra`: Short code fragment that copies the memory at an offset from a1 to d1, then shifts d1 left by 2, then copies an indexed entry from a0 to the memory at an offset from a1, then clears the memory at an offset from a2, then tests the memory at an offset from a1, then branches if the last result was not zero, then loads 0 into d1, then jumps to another point in the code.
- `process_memory_operations`: Processes multiple memory locations using calculated indices from data at a1, performs arithmetic operations on values, and stores results in specific offsets.
- `copy_and_shift_data`: Copies data between registers, calculates a memory address by adding a register value to a constant, and then jumps to another routine.

### Icon.Library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_icon_structure`: Updates icon linked list pointers and calls a library function to handle the icon structure, or sets a flag and returns zero if a condition is met.
- `frag_bset_moveq_rts`: Short code fragment that sets bit 3 of the memory at an offset from a6, then loads 0 into d0, then returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_icon_data`: Initializes icon data fields with pointer values from registers, calls a library function to process the icon, and handles an error case by calling exec.Alert.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `frag_move_jsr_addq_rts`: Short code fragment that copies a0 to the stack, then copies a6 to the stack, then calls another routine, then adds 8 to a7, then returns.
- `frag_movem_move_jsr_lea_rts`: Short code fragment that saves registers on the stack, then copies a6 to the stack, then calls another routine, then loads the address of the memory at an offset from a7 into a7, then returns.
- `frag_movem_move_jsr_lea_rts_2`: Short code fragment that saves registers on the stack, then copies a6 to the stack, then calls another routine, then loads the address of the memory at an offset from a7 into a7, then returns.
- `icon_library_save_a0_a1_call_helper`: Pushes a0, a1 and a6 onto the stack, calls a library helper routine, then removes the saved registers and returns to the caller.
- `frag_move_jsr_addq_rts_2`: Short code fragment that copies a0 to the stack, then copies a6 to the stack, then calls another routine, then adds 8 to a7, then returns.
- `frag_move_jsr_addq_rts_3`: Short code fragment that copies a0 to the stack, then copies a6 to the stack, then calls another routine, then adds 8 to a7, then returns.
- `frag_move_jsr_addq_rts_4`: Short code fragment that copies a6 to the stack, then calls another routine, then adds 4 to a7, then returns.
- `frag_movem_move_jsr_lea_rts_3`: Short code fragment that saves registers on the stack, then copies a6 to the stack, then calls another routine, then loads the address of the memory at an offset from a7 into a7, then returns.
- `push_args_and_call_init`: Pushes a0 and a6 onto the stack, calls a library initialization routine, removes the two pushed longwords, and returns.
- `icon_library_call_vector_at_7fc`: Pushes a0 and a6 on the stack, calls the library vector at offset 2044, then discards both saved registers and returns to the caller.
- `frag_movem_jsr_addq_rts`: Short code fragment that saves registers on the stack, then calls another routine, then adds 8 to a7, then returns.
- `frag_movem_jsr_addq_rts_2`: Short code fragment that saves registers on the stack, then calls another routine, then adds 8 to a7, then returns.
- `frag_movem_jsr_addq_rts_3`: Short code fragment that saves registers on the stack, then calls another routine, then adds 8 to a7, then returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `copy_icon_parameters`: Copies icon parameters from one structure to another using a loop, updates specific fields in the destination structure, and manages linked list connections.
- `copy_icon_attributes`: Copies multiple icon attributes from one structure to another using a loop, then checks for a null pointer and updates a field if it's not null.
- `frag_moveq_movem_unlk_rts`: Short code fragment that loads 0 into d0, then copies several registers to or from memory, then removes the stack frame held in a6, then returns.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `process_icon_data_2`: Processes icon parameters from a structure, updates icon coordinates and dimensions based on conditional checks, and stores computed values.
- `update_icon_data_2`: Processes icon parameters from a structure, updates icon coordinates and dimensions based on conditional checks, and stores computed values back to the structure.
- `iterate_and_update_icon_data_2`: Iterates through icon data entries, performs library calls to update each entry, and handles multiple conditional branches based on return values while updating counters.
- `iterate_and_update_icon_data`: Iterates through icon data entries, performs library calls to update each entry, and handles multiple conditional branches based on return values from those calls.
- `frag_moveq_bra`: Short code fragment that loads 0 into d4, then jumps to another point in the code.
- `frag_move_movem_unlk_rts`: Short code fragment that copies d4 to d0, then copies several registers to or from memory, then removes the stack frame held in a6, then returns.
- `process_icon_data_4`: Processes icon data by calling helper routines with specific parameters and updating internal state based on conditional checks.
- `process_icon_entry`: Processes an icon entry by checking flags, calling routines based on conditions, and updating a counter while handling potential null pointers.
- `process_icon_entries`: Processes icon table entries by iterating through pointers, calling a routine for each entry, and updating a counter while handling potential null entries.
- `process_icon_table_entries`: Walks through a table of icon pointers, processes each entry with a callback routine, and updates counters while handling potential null entries.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `frag_move_movea_lea_bra`: Short code fragment that copies a2 to the stack, then copies the memory at an offset from a7 to a0, then loads the address of the memory at an offset from a0 into a2, then jumps to another point in the code.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `process_icon_data_3`: Reads icon data from a structure, performs conditional operations based on the data, and calls functions with specific offsets and parameters from the structure.
- `push_and_call_function`: Pushes two long words onto the stack and calls a function with those values as parameters.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `decrement_and_store_pair`: Decrements a short counter through a pointer, and if the counter becomes zero or negative, calls another routine with the counter pointer and a value from the stack, then stores two values in a table using the counter as an index.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `setup_icon_data`: Allocates memory for icon data, initializes icon attributes, performs validation checks on icon parameters, and manages icon linking through conditional operations.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `perform_incremental_recursion`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `frag_movem_movea_move_jsr_addq_beq_bra`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a7 to a2, then copies the memory at an offset from a7 to d2, then copies d2 to the stack, then calls another routine, then copies d0 to d3, then copies a2 to d4, then adds 4 to a7, then branches if the last result was zero, then jumps to another point in the code.
- `load_and_compare`: Loads three values from registers into the stack, calls another routine, checks the result, and if not zero, compares a byte from a pointer with the value 61 and branches if not equal.
- `frag_movea_move_bne_moveq_movem_rts`: Short code fragment that copies the memory at a2 (advancing a2) to a3, then copies a3 to d0, then branches if the last result was not zero, then loads 0 into d0, then restores saved registers from the stack, then returns.
- `frag_movem_move_jsr_addq_bra`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a7 to d2, then copies the memory at an offset from a7 to d3, then copies d3 to the stack, then calls another routine, then copies d0 to d5, then adds 4 to a7, then jumps to another point in the code.
- `frag_pea_move_jsr_addq_beq_sub_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then copies d2 to the stack, then calls another routine, then copies d0 to d4, then adds 8 to a7, then branches if the last result was zero, then copies d4 to d0, then subtracts d2 from d0, then jumps to another point in the code.
- `validate_icon_data`: Tests icon data integrity by comparing computed checksums and validating sequence values, returning success or failure based on matching conditions.
- `frag_tst_beq_move_addq_bra`: Short code fragment that tests d4, then branches if the last result was zero, then copies d4 to d2, then adds 1 to d2, then jumps to another point in the code.
- `frag_moveq_tst_bne_movem_rts`: Short code fragment that loads 0 into d2, then tests d2, then branches if the last result was not zero, then loads 0 into d0, then restores saved registers from the stack, then returns.
- `process_string_attributes`: Processes string data by copying attributes, performing checksum calculations, and handling conditional logic based on string content and computed values.
- `increment_and_branch`: Increments a pointer by 5, pushes two values on the stack, calls a function with the pointer as parameter, tests the result, and if non-zero jumps to another routine.
- `frag_moveq_bra_2`: Short code fragment that loads 0 into d3, then jumps to another point in the code.
- `compute_string_checksum_2`: Iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern.
- `validate_character_range`: Validates if a character value falls within a specific range and returns an adjusted value based on comparison results.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `setup_icon_structure`: Initializes fields of an icon structure pointed to by a0, sets two word-sized fields to the value ten, then calls another routine with parameters derived from the structure and a2.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `process_icon_parameters`: Processes icon parameters by copying values from stack locations, performs a library call, and returns a comparison result between registers.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `setup_and_compare`: Sets up a stack frame with three long values, calls a routine using those values, compares the returned value with another memory entry, and returns zero if they are equal.
- `tiny_move_rts_3`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `perform_division_with_rotation`: Performs division operations on 32-bit values with bit rotation, handling cases where the divisor exceeds a threshold by using iterative bit manipulation.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `frag_move_bsr_rts`: Short code fragment that copies d2 to the stack, then copies d1 to d2, then copies d0 to d1, then calls another routine, then copies the top of the stack to d2, then returns.
- `frag_move_bsr_rts_2`: Short code fragment that copies d2 to the stack, then copies d1 to d2, then copies d0 to d1, then calls another routine, then copies d1 to d0, then copies the top of the stack to d2, then returns.
- `negate_result_on_flag`: Calls a helper for a value and negates that value when the flag the helper leaves in an address register is nonzero.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts_4`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `frag_movea_move_beq_cmp_bne_subq_rts`: Short code fragment that copies the memory at an offset from a7 to a0, then copies the memory at an offset from a7 to d0, then copies the memory at a0 (advancing a0) to d1, then branches if the last result was zero, then compares d1 with d0, then branches if the last result was not zero, then subtracts 1 from a0, then copies a0 to d0, then returns.
- `frag_moveq_bra_3`: Short code fragment that loads 0 into d0, then jumps to another point in the code.
- `frag_movem_move_bne_rts`: Short code fragment that copies several registers to or from memory, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then branches if the last result was not zero, then returns.
- `frag_movem_tst_bne_subq_bra`: Short code fragment that copies several registers to or from memory, then tests the memory at a0 (advancing a0), then branches if the last result was not zero, then subtracts 1 from a0, then jumps to another point in the code.
- `compare_string_segments`: Compares segments of two strings byte by byte, updates a counter, and returns zero when the strings match completely.
- `frag_moveq_bgt_rts`: Short code fragment that loads 1 into d0, then branches if the comparison was greater, then loads 255 into d0, then returns.
- `frag_movem_move_tst_beq_subq_bgt_bra`: Short code fragment that copies several registers to or from memory, then copies the memory at an offset from a7 to d0, then tests the memory at a0 (advancing a0), then branches if the last result was zero, then subtracts 1 from d0, then branches if the comparison was greater, then jumps to another point in the code.
- `frag_subq_bsr_rts`: Short code fragment that subtracts 1 from a0, then calls another routine, then returns.
- `frag_subq_blt_move_bne_ble_clr_bra`: Short code fragment that subtracts 1 from d0, then branches if the comparison was less, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then branches if the last result was not zero, then subtracts 1 from d0, then branches if the comparison was less or equal, then clears the memory at a0 (advancing a0), then jumps to another point in the code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `frag_movea_move_addq_clr_rts`: Short code fragment that copies the memory at an offset from a7 to a0, then copies a0 to the memory at a0, then adds 4 to the memory at a0, then clears the memory at an offset from a0, then copies a0 to the memory at an offset from a0, then returns.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `frag_movem_move_movea_bra`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a7 to d2, then copies the memory at an offset from a7 to d3, then copies the memory at an offset from a7 to a2, then jumps to another point in the code.
- `process_icon_chain`: Traverses a linked list of icon structures starting at address in a2, calling another routine with data from each structure and checking the result.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `process_icon_data`: Reads icon parameters from stack locations, performs calculations on icon coordinates and dimensions, and stores computed values back to the icon structure.
- `function_epilogue_4`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `frag_movem_move_moveq_bra`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a7 to d2, then copies the memory at an offset from a7 to d3, then copies the memory at an offset from a7 to d4, then loads 0 into d6, then jumps to another point in the code.
- `update_icon_with_params`: Prepares parameters for a routine call by pushing values onto the stack, including a fixed value of an address, and calls another routine.
- `update_icon_data`: Updates icon data by calling routines to process parameters, storing results in memory locations, and handling conditional branches based on return values.
- `function_epilogue_5`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.

### Dizzy Diamonds (19xx)(Titanics Entertainment).adf:DIZZY.EXE

- `initialize_game`: Initializes game memory structures and sets up data pointers for diamond-related operations, then returns immediately.
- `frag_movem_move_jsr_movea_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to d1, then copies a location in the code to d2, then copies 5120 to d3, then calls another routine, then restores saved registers from the stack, then copies a location in the code to a0, then returns.
- `copy_memory_blocks`: Copies data between memory locations using byte-level operations, with branching logic that depends on register values.
- `frag_moveq_lsl_bne_bsr_roxl_dbra_rts`: Short code fragment that loads 0 into d2, then shifts d0 left by 1, then branches if the last result was not zero, then calls another routine, then rotates d2 left through the extend flag by 1, then loops back while counting down d1, then returns.
- `frag_move_cmpa_bne_bsr_roxl_rts`: Short code fragment that copies the memory at a0 (advancing a0) to d0, then compares a0 with a location in the code, then branches if the last result was not zero, then calls another routine, then copies 16 to the condition codes, then rotates d0 left through the extend flag by 1, then returns.

### Dizzy Diamonds (19xx)(Titanics Entertainment).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
