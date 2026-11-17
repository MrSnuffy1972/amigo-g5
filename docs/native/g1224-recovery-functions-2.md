# Game 1224 (lords of the rising sun (1989)(mirrorsoft)(disk 1 of 2)[a2]): code recovery summary

## Functions (part 2 of 2)

### Lords of the Rising Sun (1989)(MirrorSoft)(Disk 1 of 2)[a2].adf:Echo
(continued)

- `frag_move_bra_16`: Short code fragment that copies the memory at an offset from a1 to d1, then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `validate_position_range`: Validates position range by comparing calculated values with stored limits, performs arithmetic operations on position components, and updates a counter field in a structure based on comparison results.
- `frag_move_bra_17`: Short code fragment that copies the memory at an offset from a1 to d1, then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `update_position_and_call`: Updates several position fields in a structure, compares values, calculates an index from a table, reads a byte from that index, calls a library routine with a fixed parameter, and updates a counter before branching.
- `frag_move_lsl_tst_bne_moveq_movea_jsr_b`: Short code fragment that copies the memory at a1 to d1, then shifts d1 left by 2, then tests an indexed entry from a0, then branches if the last result was not zero, then loads 10 into d1, then copies 352 to d0, then copies the memory at an offset from a2 to a4, then calls the routine whose address is in a5, then jumps to another point in the code.
- `check_and_call_library`: Tests a global flag, calls library routines with different parameters if the flag is set, performs conditional checks on returned values, and jumps to another routine.

### Lords of the Rising Sun (1989)(MirrorSoft)(Disk 1 of 2)[a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `setup_and_clear_flag`: Saves a4 to stack, loads a4 from program counter relative address, sets up stack frame, tests a global flag, clears it if set, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_16`: Empty routine: returns to the caller immediately without doing anything.
- `frag_movea_move_addq_jmp_102`: Short code fragment that copies the memory at an offset from a7 to a1, then copies a fixed memory address to a6, then copies the top of the stack to the memory at an offset from a7, then adds 4 to a7, then jumps to another point in the code.
- `frag_movea_move_addq_jmp_103`: Short code fragment that copies the memory at an offset from a7 to a1, then copies a fixed memory address to a6, then copies the top of the stack to the memory at an offset from a7, then adds 4 to a7, then jumps to another point in the code.
- `frag_movea_move_addq_jmp_104`: Short code fragment that copies the memory at an offset from a7 to a0, then copies a fixed memory address to a6, then copies the top of the stack to the memory at an offset from a7, then adds 4 to a7, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_17`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_18`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_19`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_20`: Empty routine: returns to the caller immediately without doing anything.
- `frag_movea_move_addq_jmp_105`: Short code fragment that copies the memory at an offset from a7 to a1, then copies a fixed memory address to a6, then copies the top of the stack to the memory at an offset from a7, then adds 4 to a7, then jumps to another point in the code.
- `frag_movea_move_addq_jmp_106`: Short code fragment that copies the memory at an offset from a7 to a1, then copies a fixed memory address to a6, then copies the top of the stack to the memory at an offset from a7, then adds 4 to a7, then jumps to another point in the code.
- `call_make_functions_2`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `prepare_for_library_call`: Moves several registers from stack, sets up a6 to point to a library function, updates the stack pointer, and jumps to that library function.
- `frag_movea_move_addq_jmp_107`: Short code fragment that copies the memory at an offset from a7 to a1, then copies the memory at an offset from a7 to d0, then copies a fixed memory address to a6, then copies the top of the stack to the memory at an offset from a7, then adds 8 to a7, then jumps to another point in the code.
- `frag_movea_move_addq_jmp_108`: Short code fragment that copies the memory at an offset from a7 to a1, then copies a fixed memory address to a6, then copies the top of the stack to the memory at an offset from a7, then adds 4 to a7, then jumps to another point in the code.
- `call_set_function_2`: Library stub: loads its arguments and the exec library base, calls SetFunction and returns its result to the caller.
- `frag_movea_move_addq_jmp_109`: Short code fragment that copies the memory at an offset from a7 to a1, then copies a fixed memory address to a6, then copies the top of the stack to the memory at an offset from a7, then adds 4 to a7, then jumps to another point in the code.
- `empty_stub_21`: Empty routine: returns to the caller immediately without doing anything.
- `frag_link_movea_cmpa_bne_moveq_bra`: Short code fragment that creates a stack frame in a5, then copies the memory at an offset from a5 to a3, then compares a3 with 0, then branches if the last result was not zero, then loads 0 into d0, then jumps to another point in the code.
- `frag_move_jsr_tst_bne_moveq_bra`: Short code fragment that copies the memory at an offset from a5 to the stack, then copies 65537 to the stack, then calls another routine, then copies d0 to the memory at an offset from a5, then tests the memory at an offset from a5, then branches if the last result was not zero, then loads 0 into d0, then jumps to another point in the code.
- `frag_movea_move_bra_4`: Short code fragment that copies the memory at an offset from a5 to a3, then copies 5 to the memory at an offset from a3, then copies the memory at an offset from a5 to d2, then copies d2 to the memory at an offset from a3, then copies the memory at an offset from a5 to a2, then copies a2 to the memory at an offset from a3, then copies the memory at an offset from a5 to d0, then jumps to another point in the code.
- `frag_unlk_movea_addq_jmp_23`: Short code fragment that removes the stack frame held in a5, then copies the top of the stack to a0, then adds 8 to a7, then jumps to the address in a0.
- `frag_link_move_moveq_bsr_bra`: Short code fragment that creates a stack frame in a5, then copies the memory at an offset from a5 to the stack, then loads 48 into d2, then copies d2 to the stack, then calls another routine, then jumps to another point in the code.
- `frag_unlk_movea_addq_jmp_24`: Short code fragment that removes the stack frame held in a5, then copies the top of the stack to a0, then adds 4 to a7, then jumps to the address in a0.
- `initialize_and_process_entry`: Sets up a stack frame, initializes a structure with specific values, calls another routine with parameters from the structure, and returns.
- `frag_link_move_bsr_unlk_movea_addq_jmp`: Short code fragment that creates a stack frame in a5, then copies the memory at an offset from a5 to the stack, then calls another routine, then removes the stack frame held in a5, then copies the top of the stack to a0, then adds 4 to a7, then jumps to the address in a0.
- `function_epilogue_12`: Function exit sequence shared or jumped to by a routine: removes the stack frame with unlk, then returns to the caller.
- `frag_adda_lea_move_rts`: Short code fragment that adds the memory at a4 to a7, then loads the address of a location in the code into a0, then copies the memory at an offset from a4 to the memory at an offset from a0, then returns.
- `frag_movea_move_addq_jmp_110`: Short code fragment that copies the memory at an offset from a7 to a0, then copies a location in the code to a6, then copies the top of the stack to the memory at an offset from a7, then adds 4 to a7, then jumps to another point in the code.
- `frag_movea_move_addq_jmp_111`: Short code fragment that copies the memory at an offset from a7 to a0, then copies a location in the code to a6, then copies the top of the stack to the memory at an offset from a7, then adds 4 to a7, then jumps to another point in the code.
- `frag_move_movea_addq_jmp_33`: Short code fragment that copies the memory at an offset from a7 to d0, then copies a location in the code to a6, then copies the top of the stack to the memory at an offset from a7, then adds 4 to a7, then jumps to another point in the code.
- `frag_movea_move_addq_jmp_112`: Short code fragment that copies the memory at an offset from a7 to a0, then copies a location in the code to a6, then copies the top of the stack to the memory at an offset from a7, then adds 4 to a7, then jumps to another point in the code.
- `execute_branch_loop`: Pushes a4 onto the stack, loads a new address into a4 from a relative offset, links a5, branches to an earlier instruction in the routine, then unlinks a5, restores a4 from the stack, and returns.
- `update_table_entry_3`: Updates a table entry by calculating an address from input parameters, stores a memory address at that location, and performs conditional operations to manage table state.
- `update_table_entry_2`: Reads a word value from a5, shifts it left by 2 bits, calculates an address in a3, updates a table entry at that address with another address, and returns.
- `process_and_update_table`: Initializes several fields in a structure, performs comparisons between values from a table and the structure, updates table entries based on comparison results, and returns.
- `process_and_update_state`: Processes state information by updating multiple fields in structures, performs comparisons between values and table entries, and manages counters and indices for state tracking through a series of conditional operations.
- `update_position_and_compare`: Updates a position value by adding two components together and stores the result.
- `process_state_information`: Processes state information by pushing registers onto the stack, calculating an address using a table lookup, and calling another routine with parameters from the stack.
- `calculate_and_store_coordinates`: Calculates coordinate values by multiplying and dividing components, then stores the results in a structure.
- `update_property_with_calculation`: Calculates a value using multiplication and addition operations, then passes it to a function for processing.
- `update_another_property_with_calculation`: Calculates a value using multiplication and addition operations, then passes it to a function for processing.
- `update_third_property_with_calculation`: Calculates a value using multiplication and addition operations, then passes it to a function for processing.
- `frag_lea_subq_cmpi_bls_moveq_asl_move_j_2`: Short code fragment that loads the address of a location in the code into a2, then subtracts 1 from d2, then compares d2 with 2, then branches if the comparison was lower or same (unsigned), then loads 255 into d2, then shifts d2 left by 1, then copies an indexed entry from a2 to d2, then calls another routine, then adds 1 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_unlk_movea_lea_jmp_6`: Short code fragment that removes the stack frame held in a5, then copies the top of the stack to a4, then copies the top of the stack to a0, then loads the address of the memory at an offset from a7 into a7, then jumps to the address in a0.
- `update_unit_statistics`: Updates unit statistics including health, position, and flags by performing arithmetic operations on stored values, calls library functions to modify data, and updates multiple fields in a structure based on calculated results.
- `update_unit_stats`: Updates unit statistics including health, position, and flags based on current state and calculations.
- `process_unit_coordinates`: Processes unit coordinates by calculating scaled values from input parameters, storing results in structures, and performing conditional operations based on global flags.
- `update_unit_status`: Updates unit status by calculating and storing coordinates, modifying state information, and performing conditional operations based on global variables and table lookups.
- `calculate_unit_position`: Calculates unit position coordinates by performing arithmetic operations on memory values, scaling and dividing results, and updating structure fields with calculated values.
- `update_unit_position`: Updates unit position coordinates by calculating scaled values from input parameters and storing results in a structure, then performs conditional updates to related global variables based on state information.
- `frag_move_movea_link_tst_beq_bra`: Short code fragment that copies a4 to the stack, then copies a location in the code to a4, then creates a stack frame in a5, then tests the memory at an offset from a5, then branches if the last result was zero, then copies 7 to the memory at an offset from a5, then jumps to another point in the code.
- `update_table_entries_2`: Calculates addresses from input parameters, stores memory addresses at calculated locations, and performs conditional updates based on table data.
- `update_table_entries`: Updates table entries by calculating addresses from input parameters, storing memory addresses at calculated locations, and performing conditional operations based on comparison results.
- `update_unit_table_values`: Updates entries in a global table based on input parameters and stores pointers to data structures at calculated offsets.
- `adjust_unit_value_and_update`: Calculates a value based on input parameters and stores it in a global table.
- `adjust_unit_value`: Adjusts a unit value by calculating a difference between input parameters, performs arithmetic operations on the result, and stores the updated value back to a specific field in a structure.
- `frag_tst_beq_move_bra`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then tests the memory at an offset from a5, then branches if the last result was zero, then copies 23 to a fixed memory address, then jumps to another point in the code.
- `frag_move_unlk_movea_lea_jmp_rts`: Short code fragment that copies 19 to a fixed memory address, then removes the stack frame held in a5, then copies the top of the stack to a4, then copies the top of the stack to a0, then loads the address of the memory at an offset from a7 into a7, then jumps to the address in a0, then removes the stack frame held in a5, then copies the top of the stack to a4, then returns.
- `frag_lea_move_rts_15`: Short code fragment that loads the address of a location in the code into a0, then copies the memory at an offset from a4 to the memory at an offset from a0, then returns.
- `frag_movea_move_addq_jmp_113`: Short code fragment that copies the memory at an offset from a7 to a1, then copies a location in the code to a6, then copies the top of the stack to the memory at an offset from a7, then adds 4 to a7, then jumps to another point in the code.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_function_with_parameters`: Prepares parameters for a library function call by moving values from the stack to registers, then jumps to a specific address in the library with those parameters.
- `frag_movea_move_addq_jmp_114`: Short code fragment that copies the memory at an offset from a7 to a0, then copies a location in the code to a6, then copies the top of the stack to the memory at an offset from a7, then adds 4 to a7, then jumps to another point in the code.
- `frag_movea_addq_jmp_2`: Short code fragment that copies the top of the stack to a0, then adds 4 to a7, then jumps to the address in a0.
- `frag_movea_move_addq_jmp_115`: Short code fragment that copies the memory at an offset from a7 to a0, then copies a location in the code to a6, then copies the top of the stack to the memory at an offset from a7, then adds 4 to a7, then jumps to another point in the code.
- `frag_movea_move_addq_jmp_116`: Short code fragment that copies the memory at an offset from a7 to a0, then copies the memory at an offset from a7 to d0, then copies a location in the code to a6, then copies the top of the stack to the memory at an offset from a7, then adds 6 to a7, then jumps to another point in the code.
- `empty_stub_22`: Empty routine: returns to the caller immediately without doing anything.
