# Game 326 (Trump Castle): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00326). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901206 top-level bytes in 33 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 476 of 476 named (0 author symbols), 476 of 476 described (171 by rule, 303 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2422 of 2422 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 23040 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 32691 of 32694 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 426163 |
| code | 126698 |
| data | 107717 |
| padding | 6644 |
| unused | 233984 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Trump Castle.adf:type | 2 | 363 | 12 |
| Trump Castle.adf:Trump_Castle | 3 | 32313 | 460 |
| Trump Castle.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |
| allfruit (raw code file, load address unknown) | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |

## Cases handled

- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Trump Castle/Trump Castle.adf:data/allfruit: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 displacement
- RAMSCAN: 1 confirmed routine segment(s) from the run-end RAM image, 22 bytes, added as run-time units
- 1 never-loaded region(s), 23040 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `generate_ascii_character` (Trump_Castle): Calculates an ASCII character value using a combination of arithmetic operations on registers and checks if the result falls within certain ranges. Kind: checksum/manual lookup trap. Pass path: the branch where uStack_c == uStack_10, resulting in the return or exit via u1__sub_0_20620.
- `process_input_and_execute` (Trump_Castle): Converts numerical values to ASCII characters and stores them in a buffer, then executes bytecode sequences while managing state variables and handling input events through recursive function calls and library interactions. Kind: input validation/check. Pass path: The loop continues until the correct input is entered (matching *(short *)(unaff_A5 + -0xc)) or 3 incorrect attempts are made, which triggers the 'Wrong too many times!!!' message and resets the check..
- `validate_global_state_range` (Trump_Castle): Checks if a global variable falls within a large number of specific, non-contiguous valid ranges. Kind: range-based checksum/value trap. Pass path: the branchs where the conditional checks fail (i.e., the code skips the nested `if` blocks or proceeds to subsequent logic, rather than entering the failure-state nested blocks).

## Functions


### Trump Castle.adf:type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block`: copies a fixed number of words from a source pointer to a destination pointer.
- `initialize_game_state_2`: Initializes various game state variables and clears multiple memory locations.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_moveq_move_bra`: Short code fragment that loads 255 into d1, then copies d1 to the memory at an offset from a2, then jumps to another point in the code.
- `frag_moveq_move_bra_2`: Short code fragment that loads 255 into d1, then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `process_bytecode_entry`: Reads a bytecode value from a table using an index from a structure, updates counters in the structure, and calls a function multiple times based on conditions involving the read value and flags in the structure.
- `process_bytecode_value`: Reads a bytecode value from a table using an index from a structure, updates counters in the structure, and calls a function multiple times.
- `execute_bytecode_sequence`: Executes bytecode sequences by managing counters and memory locations, calling functions with specific parameters based on conditional checks.
- `initialize_and_execute`: Initializes a global variable with the value one, then performs a series of function calls with specific parameters, updating memory locations and checking conditions before branching to different code sections based on comparison results.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_move_movea_jsr_jmp`: Short code fragment that copies 232 to d0, then copies the memory at an offset from a2 to a4, then calls the routine whose address is in a5, then jumps to the address in a6.

### Trump Castle.adf:Trump_Castle

- `initialize_memory_and_dispatch_calls`: Performs a zero-fill loop on a memory range and conditionally calls a function pointer or a library function based on a global variable or a return value.
- `initialize_config_and_string_tables`: Initializes or resets global data tables and string metadata, or shifts existing table entries and clears memory if certain validation checks fail.
- `process_string_instructions`: Executes a sequence of instructions that modifies a set of local or global counters based on certain return values.
- `parse_string_tags_to_buffer`: Sets global flags and calls a routine multiple times to process string tags or buffers using a local stack buffer.
- `initialize_table_limits`: Sets or resets certain memory table or array limits and initializes a range of memory or table entries to a constant maximum value.
- `update_global_state`: Sets certain global flags to a constant value and calls a routine that appends strings to a local buffer.
- `initialize_string_table_and_validate_data`: populates a table with string fragments or values and performs a check or loop involving string copying and pointer-based validation.
- `generate_ascii_character`: Calculates an ASCII character value using a combination of arithmetic operations on registers and checks if the result falls within certain ranges.
- `convert_and_store_value`: Converts a numerical value to an ASCII character representation and stores it at a fixed offset in a register.
- `process_input_and_execute`: Converts numerical values to ASCII characters and stores them in a buffer, then executes bytecode sequences while managing state variables and handling input events through recursive function calls and library interactions.
- `execute_fixed_sequence`: Executes a fixed sequence of three routine calls, including one with a zeroed argument.
- `traverse_and_update_structure`: Recursively traverses a linked or nested structure to check for a zeroed entry, then updates a target structure with new or cleared data if a match is found.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d1.
- `initialize_global_flags_and_structures`: Initializes a global state or structure by setting several flags and calling a routine to allocate or configure memory-related structures.
- `evaluate_global_flag_and_update`: initializes a local structure with a specific set of constant and global values, calls a library function, and then updates a global variable based on the returned value and a conditional bitwise or operation.
- `modify_global_bit`: sets or clears the least significant bit of a byte in a global variable based on the input parameter.
- `dispatch_conditional_calls`: Checks various bits of a global variable and calls different routines or library functions with specific arguments based on those bits.
- `process_event_handler`: Iteratively processes a pointer from a global structure, calls a routine with a bit-shifted value, and modifies a global variable based on certain values found in a table.
- `process_event_loop`: Iteratively retrieves a pointer from a global structure to call a routine with a bit-shifted value, while updating a global variable based on a certain value.
- `traverse_linked_routine_chain`: Follows a chain of pointers retrieved from a global structure, calling another routine for each non-zero pointer found.
- `execute_table_iterations`: Iterates a fixed number of times, calling a routine with a pointer from a table and an incrementing counter.
- `call_library_with_offset_parameter`: Calls a library function using a pointer or base address retrieved from a global variable plus a fixed offset, passing a constant length or count as a second argument.
- `execute_string_parsing_sequence`: Calls a routine using a pointer from a global structure and then invokes a routine that parses a list of string tags using a local buffer.
- `execute_conditional_fill`: loads data from a pointer into a local stack, calls a library function, and potentially calls another library function if a parameter is non-zero.
- `store_parameters_to_stack`: Stores passed arguments onto the stack and calls another routine with a pointer to a local stack variable.
- `execute_recursive_function`: Executes a function pointer from a library, then recursively calls itself using a pointer from a global table or a provided argument.
- `copy_words_with_stride`: copies a sequence of words from a source or a table to a destination buffer using a stride-based loop.
- `calculate_and_call_range`: Calculates a range or count from two parameters and calls a library routine with that calculated value and a range of parameters.
- `call_library_with_range`: Calls a library routine with a range or count, then recursively calls another routine using the return value.
- `populate_array_from_result`: Calculates a value using a routine, then iterates through a loop to store certain values into an array if a condition is met.
- `iterate_and_modify_table`: Iterates a certain number of times, performing calculations and updating a table or memory via a pointer.
- `get_library_result`: calls a library function and returns the value written to a local variable.
- `insert_word_into_array`: Retrieves a word from a table and inserts it into a memory array by shifting existing array elements backwards via a decrementing loop.
- `copy_memory_backwards`: Decrements a byte and copies a word from one memory location to another until a certain value is reached, then calls another routine.
- `update_memory_with_register`: Stores a value from a register into a memory location and then performs an indirect library call.
- `call_library_with_count`: Calculates a count from two parameters and calls a library routine using a pointer from a global table.
- `call_library_with_parameters`: Calls a library routine using a specific range or count and then recursively calls another routine.
- `execute_library_calls`: calls a library function twice using a pointer retrieved from a global variable.
- `traverse_pointer_chain`: Follows a chain of pointers by loading a value from a memory address, adding an offset, and dereferencing the resulting pointer twice.
- `build_string_structure_list_2`: Calls a routine to build a list of structures from a string and then calls another routine using the returned pointer.
- `parse_string_tags_and_validate_metadata`: Parses a string tag sequence to extract or validate configuration-related metadata, such as string lengths and buffer limits.
- `evaluate_state_transition`: Evaluates a chain of pointers and checks certain bitmask conditions or value ranges to determine a new state, potentially updating a global variable or calling other routines based on the result.
- `frag_move_bra`: Short code fragment that copies 34 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies 162 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_3`: Short code fragment that copies 66 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_4`: Short code fragment that copies 98 to the memory at an offset from a5, then jumps to another point in the code.
- `process_game_state`: Updates a global variable based on conditional logic and table data, then calls several routines including table iterations, bit modification, and library functions before returning the value of the global variable.
- `traverse_linked_lists`: Iterates through a nested linked list structure, calling another routine for each element or pointer encountered.
- `initialize_blinker_struct`: Initializes a structure or object or table with specific integer values and sets certain function pointers, including a pointer to itself, while also performing a series of initial data-population calls using hardcoded parameters or previously computed values.
- `initialize_game_state`: Initializes several global variables and structures, clears a small array, and configures an initial state for a conditional logic branch.
- `empty_routine`: the routine contains no executable instructions and performs no operations.
- `empty_routine_2`: The routine contains no executable instructions and performs no operations.
- `load_data_from_pointer`: Clears a 16-bit word at the address pointed to by a5, loads the value at the address in a0 into d0, and then loads the longword at the same address in a0 into d0.
- `configure_global_parameters`: Sets a local variable based on an input value and calls a routine multiple times with a pointer to a global variable and fixed parameters.
- `calculate_variable_offset_and_bounds`: Calculates or updates a global variable and certain stack-based offsets or boundaries by repeatedly calling other routines and checking bitmask-filtered return values.
- `execute_iterative_audio_setup`: Calculates a loop count based on a masked return value, then repeatedly initializes an audio timer with a masked global value and executes a delay-based routine a fixed number of times.
- `reconfigure_data_buffer_layout`: This routine copies and reorders blocks of data from one location in a global buffer to another, using a stride-based offset determined by the input parameter.
- `update_graphics_primitives`: updates or draws a series or sequence of primitive shapes or lines by updating a destination buffer or display area.
- `execute_bytecode_sequence_3`: Prepares and executes a series of bytecode operations by pushing parameters onto the stack and calling functions, with conditional execution based on a global variable's value.
- `execute_bytecode_sequence_2`: Prepares and executes a series of bytecode operations by pushing parameters onto the stack and calling functions.
- `execute_bytecode_operations`: Executes multiple bytecode operations by preparing parameters and calling a recursive function with various values, while managing stack frames and handling conditional execution paths.
- `handle_bytecode_execution`: Processes bytecode execution commands with varying parameters, managing stack frames and handling conditional logic paths through comparisons against a global variable.
- `execute_bytecode_operations_2`: Processes a series of bytecode execution commands with varying parameters, managing stack frames and handling conditional logic paths through multiple function calls.
- `execute_multiple_operations_2`: Processes a series of structured parameter sets by calling another routine with varying arguments, managing stack frames, and handling conditional execution paths through recursive function calls with specific parameters.
- `execute_multiple_operations_3`: Processes a series of structured parameter sets by calling another routine with varying arguments and handling recursive function execution with global state updates, then returns.
- `execute_multiple_operations`: Executes multiple operations by calling functions with various parameters, managing stack frames, and handling conditional execution paths.
- `initialize_and_store`: Sets up a stack frame with a link instruction, pushes two values onto the stack, calls a library function, and stores the result from the library call into a global variable at a specific offset from the stack pointer.
- `frag_move_jsr_clr_bra`: Short code fragment that copies 160 to d1, then copies the memory at an offset from a5 to d0, then calls multiply_32_bit_values, then copies d0 to the memory at an offset from a5, then copies the memory at an offset from a5 to the memory at an offset from a5, then clears the memory at an offset from a5, then jumps to another point in the code.
- `copy_sprite_data`: Copies sprite attribute data from a table to a destination structure, advancing pointers and incrementing a counter through a loop that repeats fourteen times.
- `process_and_call_function`: Processes parameters and calls a library function repeatedly while updating memory values, then performs a final call with a fixed parameter.
- `execute_bytecode_recursive`: Updates a counter in a local structure, prepares parameters for bytecode execution, calls a function to execute bytecode with those parameters, and then calls another function to process recursive operations using the same parameters.
- `update_card_values_and_check_patterns`: Updates a table of five integer values by incrementing or decrementing them based on a provided parameter, then checks the new values against hardcoded patterns to update a secondary temporary table and a global flag.
- `frag_move_ext_bra`: Short code fragment that copies the memory at an offset from a5 to d0, then sign-extends d0, then jumps to another point in the code.
- `frag_cmpi_bne_move_ext_asl_lea_bra`: Short code fragment that compares the memory at an offset from a5 with 4, then branches if the last result was not zero, then copies the memory at an offset from a5 to d0, then sign-extends d0, then shifts d0 left by 1, then loads the address of the memory at an offset from a5 into a0, then copies 2 to an indexed entry from a0, then jumps to another point in the code.
- `frag_cmpi_bne_move_ext_asl_lea_bra_2`: Short code fragment that compares the memory at an offset from a5 with 8, then branches if the last result was not zero, then copies the memory at an offset from a5 to d0, then sign-extends d0, then shifts d0 left by 1, then loads the address of the memory at an offset from a5 into a0, then copies 6 to an indexed entry from a0, then jumps to another point in the code.
- `check_conditions_and_write`: Compares specific values in a local structure with constants, branches if the comparisons fail, checks a condition in memory, and writes a fixed value to an address calculated from another memory location using a shifted index.
- `frag_move_ext_asl_lea_bra`: Short code fragment that copies the memory at an offset from a5 to d0, then sign-extends d0, then shifts d0 left by 1, then loads the address of the memory at an offset from a5 into a0, then copies 65535 to an indexed entry from a0, then copies 1 to a fixed memory address, then jumps to another point in the code.
- `frag_cmpi_bne_move_ext_asl_lea_bra_3`: Short code fragment that compares the memory at an offset from a5 with 7, then branches if the last result was not zero, then copies the memory at an offset from a5 to d0, then sign-extends d0, then shifts d0 left by 1, then loads the address of the memory at an offset from a5 into a0, then copies 6 to an indexed entry from a0, then jumps to another point in the code.
- `validate_and_store_value`: Compares specific values in a local structure with constants, branches if the comparisons fail, checks a condition in memory, and stores a fixed value to an address calculated from another memory location using a shifted index.
- `frag_move_ext_asl_lea_bra_2`: Short code fragment that copies the memory at an offset from a5 to d0, then sign-extends d0, then shifts d0 left by 1, then loads the address of the memory at an offset from a5 into a0, then copies 65535 to an indexed entry from a0, then copies 1 to a fixed memory address, then jumps to another point in the code.
- `verify_and_update_memory`: Compares specific values in a local structure with constants, branches if the comparisons fail, calculates an address using a shifted index, and writes a fixed value to that memory location.
- `frag_move_ext_asl_lea_bra_3`: Short code fragment that copies the memory at an offset from a5 to d0, then sign-extends d0, then shifts d0 left by 1, then loads the address of the memory at an offset from a5 into a0, then copies 2 to an indexed entry from a0, then jumps to another point in the code.
- `validate_and_write_value`: Compares specific values in a local structure with constants, branches if the comparisons fail, calculates an address using a shifted index, and writes a fixed value to that memory location.
- `frag_cmpi_bne_move_ext_asl_lea_bra_4`: Short code fragment that compares the memory at an offset from a5 with 3, then branches if the last result was not zero, then copies the memory at an offset from a5 to d0, then sign-extends d0, then shifts d0 left by 1, then loads the address of the memory at an offset from a5 into a0, then copies 7 to an indexed entry from a0, then jumps to another point in the code.
- `frag_cmpi_bne_move_ext_asl_lea_bra_5`: Short code fragment that compares the memory at an offset from a5 with 5, then branches if the last result was not zero, then copies the memory at an offset from a5 to d0, then sign-extends d0, then shifts d0 left by 1, then loads the address of the memory at an offset from a5 into a0, then copies 8 to an indexed entry from a0, then jumps to another point in the code.
- `frag_cmp_bcc_asl_move_jmp`: Short code fragment that compares d0 with 10, then branches if the carry flag is clear, then shifts d0 left by 1, then copies a location in the code to d0, then jumps to another point in the code.
- `frag_cmpi_bge_clr_bra`: Short code fragment that compares the memory at an offset from a5 with 2, then branches if the comparison was greater or equal, then clears the memory at an offset from a5, then jumps to another point in the code.
- `update_card_pattern_4`: Increments a value in a table based on an index from a global variable, then checks if the value exceeds ten and adjusts it accordingly.
- `update_card_pattern_5`: Updates card pattern values based on comparisons with global variables and stores results in a local array.
- `update_card_pattern`: Updates card pattern values based on comparisons with global variables and stores results in a local array.
- `frag_move_ext_bra_2`: Short code fragment that copies the memory at an offset from a5 to d0, then sign-extends d0, then jumps to another point in the code.
- `update_pattern_entry`: Updates a table entry based on comparison of global variables with constants, then performs a sign-extension and left shift operation on a value from the structure before storing a word at a calculated address.
- `frag_move_ext_asl_lea_bra_4`: Short code fragment that copies the memory at an offset from a5 to d0, then sign-extends d0, then shifts d0 left by 1, then loads the address of the memory at an offset from a5 into a0, then copies 65535 to an indexed entry from a0, then copies 1 to a fixed memory address, then jumps to another point in the code.
- `check_and_update_pattern_2`: Compares values from two global variables with specific constants and updates a table entry based on a counter value, then branches to the end of the routine.
- `check_and_update_pattern`: Checks two global variables and updates a table entry based on a counter value, then branches to the end of the routine.
- `update_tile_pattern`: Updates a tile pattern value based on conditions involving multiple global variables, then performs a series of operations on a counter and related data structures before returning.
- `update_sprite_state`: Reads state values from specific offsets relative to a5, performs conditional checks based on those values, and writes a 16-bit value to a memory location calculated using another offset from a5.
- `update_game_state`: Updates game state variables based on conditional checks involving multiple offsets from a5 register, then performs arithmetic operations and value assignments to global data structures.
- `update_game_state_12`: Reads state variables from offsets -22(a5) and -20(a5) to determine which value to write at an address calculated from offset -10(a5) and register d0.
- `update_game_state_10`: Updates internal game state variables based on conditional checks involving multiple offsets from a5, then writes specific values to a memory location determined by a calculated index.
- `check_game_conditions`: Tests two global variables for specific values and performs a table lookup using a third variable to update a game state value.
- `update_game_state_2`: Updates internal game state variables based on conditional checks involving multiple offsets from a5, then writes specific values to a memory location calculated using a word value from a5 as an index.
- `frag_cmp_bcc_asl_move_jmp_2`: Short code fragment that compares d0 with 9, then branches if the carry flag is clear, then shifts d0 left by 1, then copies a location in the code to d0, then jumps to another point in the code.
- `frag_cmpi_bge_clr_bra_2`: Short code fragment that compares the memory at an offset from a5 with 2, then branches if the comparison was greater or equal, then clears the memory at an offset from a5, then jumps to another point in the code.
- `increment_card_counter_2`: Increments a card value counter at a calculated offset, resets it to one if it reaches nine, then increments a tracking counter.
- `update_card_values_2`: Updates card value counters based on conditions and copies the results to a target array while managing counter overflows.
- `update_card_counters`: Increments and resets card value counters based on conditions, then copies updated values to a target array while managing counters and conditions.
- `frag_move_ext_bra_3`: Short code fragment that copies the memory at an offset from a5 to d0, then sign-extends d0, then jumps to another point in the code.
- `update_game_state_3`: Reads specific word values from offsets within a structure referenced by a5, compares them against fixed constants, and writes a predefined value to a table entry calculated from a word at offset -12 of the same structure.
- `frag_move_ext_asl_lea_bra_5`: Short code fragment that copies the memory at an offset from a5 to d0, then sign-extends d0, then shifts d0 left by 1, then loads the address of the memory at an offset from a5 into a0, then copies 3 to an indexed entry from a0, then jumps to another point in the code.
- `validate_and_write_value_2`: Checks specific word values at offsets from a5 and branches if any do not match expected constants, then calculates an address using a value from a5 and stores a constant word value at that address.
- `validate_and_store_value_2`: Checks two word values at specific offsets from a5 and branches if either does not match expected constants, then calculates an address using a word value, extends it to long, shifts it left by one bit, loads that address into a0, and stores the value 180 at an offset in the structure pointed to by a0.
- `frag_cmpi_bne_move_ext_asl_lea_bra_6`: Short code fragment that compares the memory at an offset from a5 with 9, then branches if the last result was not zero, then copies the memory at an offset from a5 to d0, then sign-extends d0, then shifts d0 left by 1, then loads the address of the memory at an offset from a5 into a0, then copies 12 to an indexed entry from a0, then jumps to another point in the code.
- `validate_and_store_value_3`: Checks two word values in a global structure and branches if either does not equal three, then stores a word value at an address calculated from a pointer and a register.
- `validate_and_store_another_value`: Checks two word values at specific offsets from a5 and branches if either does not match expected constants, then calculates an address using a word value, extends it to long, shifts it left by one bit, loads that address into a0, and stores the value 175 at an offset in the structure pointed to by a0.
- `frag_cmpi_bne_move_ext_asl_lea_bra_7`: Short code fragment that compares the memory at an offset from a5 with 8, then branches if the last result was not zero, then copies the memory at an offset from a5 to d0, then sign-extends d0, then shifts d0 left by 1, then loads the address of the memory at an offset from a5 into a0, then copies 7 to an indexed entry from a0, then jumps to another point in the code.
- `update_game_state_4`: Updates a game state value based on conditions involving multiple variables and a table index, then performs a series of operations including incrementing counters and copying values between arrays.
- `frag_cmpi_bne_move_ext_asl_lea_bra_8`: Short code fragment that compares the memory at an offset from a5 with 4, then branches if the last result was not zero, then copies the memory at an offset from a5 to d0, then sign-extends d0, then shifts d0 left by 1, then loads the address of the memory at an offset from a5 into a0, then copies 2 to an indexed entry from a0, then jumps to another point in the code.
- `frag_cmp_bcc_asl_move_jmp_3`: Short code fragment that compares d0 with 9, then branches if the carry flag is clear, then shifts d0 left by 1, then copies a location in the code to d0, then jumps to another point in the code.
- `frag_clr_bra`: Short code fragment that clears the memory at an offset from a5, then jumps to another point in the code.
- `increment_card_counter`: Increments a card counter at a calculated offset and updates its value based on conditions, then increments a global index and loops until the index reaches five.
- `frag_move_ext_bra_4`: Short code fragment that copies the memory at an offset from a5 to d0, then sign-extends d0, then jumps to another point in the code.
- `update_game_state_11`: Reads multiple game state variables from specific offsets relative to a5, performs conditional checks based on their values, and writes a 16-bit value to a table at offset -10 relative to a5.
- `update_game_state_9`: Reads game state variables from offsets -22 and -20 relative to a5, then writes a value to a table at offset -10 based on conditional checks.
- `frag_cmpi_bne_move_ext_asl_lea_bra_9`: Short code fragment that compares the memory at an offset from a5 with 3, then branches if the last result was not zero, then copies the memory at an offset from a5 to d0, then sign-extends d0, then shifts d0 left by 1, then loads the address of the memory at an offset from a5 into a0, then copies 1 to an indexed entry from a0, then jumps to another point in the code.
- `update_game_state_5`: Updates internal game state variables based on conditional checks involving multiple offsets from a5, then performs a series of operations on a table and global variables before returning.
- `update_tile_flags`: Updates tile flags based on multiple conditions involving values at specific offsets, then performs a loop to adjust counters and copy data to a global table.
- `update_game_state_6`: Reads a value from a global structure, performs conditional checks on it, and writes a specific value to an array element identified by that value.
- `frag_subq_cmp_bcc_asl_move_jmp`: Short code fragment that subtracts 2 from d0, then compares d0 with 7, then branches if the carry flag is clear, then shifts d0 left by 1, then copies a location in the code to d0, then jumps to another point in the code.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `update_pattern_values`: Updates pattern values based on multiple conditional checks involving global variables, then clears a memory region before branching to another routine.
- `update_pattern_flags`: Updates pattern flags based on conditions involving multiple global variables, then clears a memory region before branching to another routine.
- `frag_cmp_bcc_asl_move_jmp_4`: Short code fragment that compares d0 with 9, then branches if the carry flag is clear, then shifts d0 left by 1, then copies a location in the code to d0, then jumps to another point in the code.
- `frag_clr_bra_2`: Short code fragment that clears the memory at an offset from a5, then jumps to another point in the code.
- `update_card_pattern_2`: Increments a counter at a calculated offset in a table, updates card values based on pattern matching, and manages game state counters until reaching a limit of five iterations.
- `frag_move_ext_bra_5`: Short code fragment that copies the memory at an offset from a5 to d0, then sign-extends d0, then jumps to another point in the code.
- `check_and_set_value`: Compares values at specific offsets of a global structure with constants 6 and 3, and if both comparisons are false, loads a word value, extends it to long, shifts it left by one bit, uses it as an offset to access another global structure, and stores the value 40 into that location.
- `frag_cmpi_bne_move_ext_asl_lea_bra_10`: Short code fragment that compares the memory at an offset from a5 with 6, then branches if the last result was not zero, then copies the memory at an offset from a5 to d0, then sign-extends d0, then shifts d0 left by 1, then loads the address of the memory at an offset from a5 into a0, then copies 1 to an indexed entry from a0, then jumps to another point in the code.
- `frag_cmpi_bne_move_ext_asl_lea_bra_11`: Short code fragment that compares the memory at an offset from a5 with 3, then branches if the last result was not zero, then copies the memory at an offset from a5 to d0, then sign-extends d0, then shifts d0 left by 1, then loads the address of the memory at an offset from a5 into a0, then copies 1 to an indexed entry from a0, then jumps to another point in the code.
- `check_game_state_2`: Compares values from global variables with constants 3, 6, and 1, and based on the results, updates a table entry with the value 8 at an offset calculated from another global variable.
- `frag_cmpi_bne_move_ext_asl_lea_bra_12`: Short code fragment that compares the memory at an offset from a5 with 3, then branches if the last result was not zero, then copies the memory at an offset from a5 to d0, then sign-extends d0, then shifts d0 left by 1, then loads the address of the memory at an offset from a5 into a0, then copies 1 to an indexed entry from a0, then jumps to another point in the code.
- `update_game_state_7`: Updates internal game state variables based on conditional checks and increments counters.
- `check_game_state`: Checks specific game state values at offsets -16 and -14 relative to a5, and based on the results, either branches to another location or continues execution to update a table entry at offset -10 from a5.
- `frag_move_ext_asl_lea_bra_6`: Short code fragment that copies the memory at an offset from a5 to d0, then sign-extends d0, then shifts d0 left by 1, then loads the address of the memory at an offset from a5 into a0, then copies 1 to an indexed entry from a0, then jumps to another point in the code.
- `frag_cmp_bcc_asl_move_jmp_5`: Short code fragment that compares d0 with 7, then branches if the carry flag is clear, then shifts d0 left by 1, then copies a location in the code to d0, then jumps to another point in the code.
- `frag_cmpi_bge_clr_bra_3`: Short code fragment that compares the memory at an offset from a5 with 2, then branches if the comparison was greater or equal, then clears the memory at an offset from a5, then jumps to another point in the code.
- `increment_and_update_pattern`: Increments a counter at a calculated offset, updates a value to one if it reaches seven, then increments another counter and repeats until it exceeds four.
- `update_card_pattern_3`: Updates card pattern values based on comparison results and manages counters, then copies updated values to a target array while handling conditional logic for specific card combinations.
- `update_card_values`: Updates card values based on game state and pattern matching, then copies updated values to a target array while managing counters and conditional logic.
- `initialize_and_populate_data_buffer`: Initializes a buffer by checking for valid pointers or certain input conditions, then copies a sequence of words or or larger data blocks from a source table or memory range into a destination array, or updates a structure with a specific amount of data or a calculated range.
- `handle_exit_prompt`: Checks a global variable against several constant values and recursively calls another routine with a pointer to that global variable.
- `update_global_and_recurse`: Updates a global variable by adding a parameter and then executes a sequence of recursive calls using values or pointers retrieved from a table and a conditional branch that selects different constants or registers.
- `empty_routine_3`: the routine is empty and performs no operations or data transfers.
- `process_event_chain`: Iteratively traverses a linked chain of routine pointers and processes event handlers.
- `traverse_state_tree_recursively`: A recursive routine that updates a global state variable and a secondary value based on a lookup table, then calls itself or other routines with modified parameters or or new state-dependent offsets.
- `frag_addi_bra`: Short code fragment that adds 17 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_addi_bra_2`: Short code fragment that adds 34 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_addi_bra_3`: Short code fragment that adds 51 to the memory at an offset from a5, then jumps to another point in the code.
- `update_position_and_call`: Updates position coordinates and calls a range calculation function with multiple parameters.
- `branch_on_global_value`: Checks a global variable and sets local stack values before calling a routine with a calculated range or a multiplication routine.
- `frag_move_bra_5`: Short code fragment that copies 1 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_6`: Short code fragment that copies 5 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_7`: Short code fragment that copies 25 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_8`: Short code fragment that copies 100 to the memory at an offset from a5, then jumps to another point in the code.
- `initialize_counter_2`: Sets a global variable to 500, updates it based on conditions involving another global variable's value and the counter itself, then returns.
- `frag_move_bra_9`: Short code fragment that copies 1 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_10`: Short code fragment that copies 5 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_11`: Short code fragment that copies 25 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_12`: Short code fragment that copies 100 to the memory at an offset from a5, then jumps to another point in the code.
- `initialize_counter`: Sets a global variable to 500, updates it based on conditions involving another global variable's value and the counter itself, then returns the counter's value.
- `update_state_table_entry`: updates or resets values in a table or global variable based on a range-checked state or index.
- `parse_string_tag_list`: Parses a string containing a delimited list of substrings separated by pipes and enclosed in brackets, storing the resulting string pointers into a table.
- `initialize_game_data_tables`: Initializes several memory-mapped tables or arrays to zero and populates them with data from hardcoded or pointer-based sources.
- `clear_register_d1`: Sets the d1 register to zero.
- `initialize_game_state_and_process_events`: Initializes a large memory-mapped table or array to zero and enters a main event loop that dispatches or handles certain event codes.
- `initialize_audio_and_library_calls`: Initializes an audio timer with specific parameters and performs two library function calls with fixed arguments, then branches to a loop that updates game state variables.
- `frag_pea_move_jsr_lea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then copies a fixed memory address to the stack, then pushes the address of a fixed memory address onto the stack (4 times in a row), then calls call_library_with_range, then loads the address of the memory at an offset from a7 into a7, then jumps to another point in the code.
- `initialize_game_variables`: Updates two word-sized global variables with values derived from a global counter, then initializes an audio timer using those values.
- `initialize_audio_and_library_calls_2`: Updates two global word variables with processed values from a routine, initializes an audio timer with those values, and performs two library calls with fixed parameters and a global variable reference.
- `initialize_audio_and_library_calls_3`: Updates two word-sized global variables using bitwise and and addition operations, then initializes an audio timer with those values before making two library calls with specific parameters.
- `update_timer_parameters`: Reads a global variable, masks its lower 4 bits, adds fixed offsets, and stores the results to two word-sized slots in a global structure.
- `push_and_call`: Pushes several values onto the stack and calls a library routine, then adjusts the stack pointer and branches to another location.
- `initialize_game_state_3`: Updates global variables with calculated values based on random inputs, initializes audio timer with specific parameters, and sets up state variables for game progression.
- `frag_move_muls_add_bra`: Short code fragment that copies the memory at an offset from a5 to d0, then multiplies d0 by 18 (signed), then adds 50 to d0, then copies d0 to the memory at an offset from a5, then jumps to another point in the code.
- `update_position_and_state`: Updates position and state variables based on calculations involving multiplication, division, and conditional logic.
- `update_game_state_8`: Updates game state variables including counters and coordinates based on calculations involving multiplication, division, and conditional logic.
- `initialize_betting_limits_and_history_2`: Sets or resets betting limits and clears a 2D array or tracking history.
- `initialize_betting_limits_and_history`: Sets betting or chip value limits based on a global configuration and clears a 2D array used for tracking recent results.
- `execute_recursive_calls`: Performs a sequence of recursive or indirect calls using a global variable and a passed parameter, returning a value obtained from a previous routine execution.
- `update_global_state_2`: Updates a global variable or a value in a table using a sequence of conditional assignments based on a parameter and the result of a recursive or nested routine call.
- `fetch_word_and_decrement_pointer`: Loads a 16-bit word from a table using an offset derived from a base address in a0 and a value in d0 into d0, then performs a bitwise OR on a byte at the address pointed to by a5 while decrementing the pointer in a5.
- `traverse_node_tree`: Iterates through a multi-dimensional table or tree structure to check for non-zero values, recursively processing or clearing certain nodes or parameters based on the presence of values or a loop-driven state flag.
- `calculate_masked_sum`: Iterates through a table while masking certain bits and returns a calculated value based on masked register contents.
- `update_attribute_table`: Iterates through a table of values to check or modify certain parameters or pointers.
- `calculate_payout_constant`: calculates a constant value based on a comparison of input parameters and checks for even or odd evenness, then uses the result to index into a table and calls another routine to handle the resulting value or a recursive call.
- `traverse_state_tree`: recursively calls another routine with modified parameters based on the value of an input parameter.
- `calculate_recursive_range`: Calculates a value based on an input parameter and calls another routine with a range or a recursive call.
- `initialize_betting_system_state`: Resets or configures a range of betting values and clears associated memory-mapped buffers or arrays.
- `initialize_betting_state_and_reset_tables`: Resets several memory tables and arrays to zero, then configures betting limits based on a retrieved value.
- `initialize_betting_range_and_history`: Sets up or resets a betting range or chip value based on a configuration value and clears several memory-mapped buffers or arrays.
- `calculate_and_format_payoff`: Checks a table of bet amounts and multipliers, then calls a routine to format a string containing the calculated payoff amount.
- `format_bet_payoff_string`: Checks if a payoff or bet amount is zero or otherwise valid, then formats a string containing a bet amount and its corresponding payoff into a buffer.
- `frag_move_pea_jsr_addq_bra`: Short code fragment that copies 2 to the memory at an offset from a5, then copies 1 to the memory at an offset from a5, then pushes the address of a location in the code onto the stack, then pushes the address of the memory at an offset from a5 onto the stack, then calls append_string_to_buffer, then adds 8 to a7, then jumps to another point in the code.
- `frag_move_pea_jsr_addq_bra_2`: Short code fragment that copies 3 to the memory at an offset from a5, then copies 2 to the memory at an offset from a5, then pushes the address of a location in the code onto the stack, then pushes the address of the memory at an offset from a5 onto the stack, then calls append_string_to_buffer, then adds 8 to a7, then jumps to another point in the code.
- `format_bet_payoff_string_2`: Initializes two global variables with values 7 and 6, then conditionally formats a payoff string based on table lookups and arithmetic operations, storing the result in a buffer.
- `format_bet_string`: Formats a string containing a bet amount and potential payoff, then appends or copies it to a buffer.
- `frag_move_pea_jsr_addq_bra_3`: Short code fragment that copies 7 to the memory at an offset from a5, then copies 5 to the memory at an offset from a5, then pushes the address of a location in the code onto the stack, then pushes the address of the memory at an offset from a5 onto the stack, then calls append_string_to_buffer, then adds 8 to a7, then jumps to another point in the code.
- `format_payout_string`: Initializes two global variables with values 7 and 6, then conditionally executes bytecode based on table lookups and performs string formatting operations using buffer manipulation routines.
- `update_global_state_3`: Executes a series of calls involving a global variable and a provided parameter, returning the result of a previously executed routine.
- `update_polygonal_state_tree`: Recursively updates a state tree or linked structure by modifying a global variable and branching or recalculating node values based on a provided or calculated index.
- `update_state_tree_node_recursive`: Recursively traverses a state tree or linked structure, updating node values and character-related lookup tables based on conditional checks.
- `frag_move_bra_13`: Short code fragment that copies 6 to the memory at an offset from a5, then copies 5 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_14`: Short code fragment that copies 3 to the memory at an offset from a5, then copies 2 to the memory at an offset from a5, then jumps to another point in the code.
- `process_polygonal_updates`: Processes conditional polygonal state updates based on signed multiplication and division results, then traverses a state tree with specific parameters.
- `frag_move_bra_15`: Short code fragment that copies 45 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_16`: Short code fragment that copies 58 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_17`: Short code fragment that copies 71 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_18`: Short code fragment that copies 84 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_19`: Short code fragment that copies 97 to the memory at an offset from a5, then jumps to another point in the code.
- `update_polygonal_state`: Processes polygonal state updates based on conditional checks and signed multiplication results, then traverses a state tree with specific parameters.
- `frag_move_bra_20`: Short code fragment that copies 45 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_21`: Short code fragment that copies 58 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_22`: Short code fragment that copies 71 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_23`: Short code fragment that copies 84 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_24`: Short code fragment that copies 97 to the memory at an offset from a5, then jumps to another point in the code.
- `update_and_traverse_6`: Updates global variables based on signed division and multiplication results, then conditionally traverses a state tree with specific parameters.
- `update_state_tree_node`: This routine recursively traverses a state tree, updating node values and character-related lookup tables based on conditional checks.
- `frag_move_bra_25`: Short code fragment that copies 6 to the memory at an offset from a5, then copies 5 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_26`: Short code fragment that copies 3 to the memory at an offset from a5, then copies 2 to the memory at an offset from a5, then jumps to another point in the code.
- `update_and_traverse_5`: Updates global variables based on signed division and multiplication results, then conditionally traverses a state tree with specific parameters.
- `frag_move_bra_27`: Short code fragment that copies 45 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_28`: Short code fragment that copies 58 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_29`: Short code fragment that copies 71 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_30`: Short code fragment that copies 84 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_31`: Short code fragment that copies 97 to the memory at an offset from a5, then jumps to another point in the code.
- `update_and_traverse_4`: Updates global variables based on signed division and multiplication results, then conditionally traverses a state tree with specific parameters.
- `frag_move_bra_32`: Short code fragment that copies 45 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_33`: Short code fragment that copies 58 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_34`: Short code fragment that copies 71 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_35`: Short code fragment that copies 84 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_36`: Short code fragment that copies 97 to the memory at an offset from a5, then jumps to another point in the code.
- `update_and_traverse_3`: Updates global variables based on signed division and multiplication results, then conditionally traverses a state tree with specific parameters.
- `frag_move_clr_bra`: Short code fragment that copies 45 to the memory at an offset from a5, then clears the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_37`: Short code fragment that copies 58 to the memory at an offset from a5, then copies 1 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_38`: Short code fragment that copies 71 to the memory at an offset from a5, then copies 2 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_39`: Short code fragment that copies 84 to the memory at an offset from a5, then copies 3 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_40`: Short code fragment that copies 97 to the memory at an offset from a5, then copies 4 to the memory at an offset from a5, then jumps to another point in the code.
- `update_and_traverse_2`: Updates a global variable based on signed division and multiplication results, then conditionally traverses a state tree with specific parameters.
- `evaluate_recursive_state_tree`: This routine implements a recursive tree traversal that updates a table of values based on a conditional check of a lookup or state array.
- `frag_move_bra_41`: Short code fragment that copies 7 to the memory at an offset from a5, then copies 5 to the memory at an offset from a5 (2 times in a row), then jumps to another point in the code.
- `frag_move_bra_42`: Short code fragment that copies 7 to the memory at an offset from a5, then copies 6 to the memory at an offset from a5 (2 times in a row), then jumps to another point in the code.
- `frag_move_bra_43`: Short code fragment that copies 7 to the memory at an offset from a5, then copies 6 to the memory at an offset from a5, then copies 8 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_44`: Short code fragment that copies 7 to the memory at an offset from a5, then copies 5 to the memory at an offset from a5, then copies 9 to the memory at an offset from a5, then jumps to another point in the code.
- `update_and_traverse`: Updates a global variable based on signed division and multiplication results, then conditionally traverses a state tree with specific parameters depending on loop iteration.
- `validate_global_state_range`: Checks if a global variable falls within a large number of specific, non-contiguous valid ranges.
- `parse_string_tags`: Iteratively parses string tags and copies string data into a buffer, potentially adjusting the buffer length based on values in a table.
- `process_string_tags`: Iteratively checks a table of values to determine if certain string tags should be parsed or appended to a buffer.
- `setup_bet_variables`: Initializes two global variables with the values six and five, then calls a routine that processes bet-related payouts using conditional table lookups.
- `process_bet_payouts`: Sets up two global variables with values 6 and 5, processes multiple conditional table lookups for bet-related data, executes bytecode operations when conditions are met, and formats payout strings using buffer manipulation routines.
- `execute_conditional_ranges`: Checks certain bits of input parameters and calls a library routine with various ranges or a function pointer from a table.
- `populate_and_update_table`: This routine populates a table with a cumulative sum calculated from a loop, and subsequently updates certain memory locations using values derived from a global variable and library-calculated ranges.
- `initialize_game_data_tables_2`: Initializes a sequence of memory-linked tables or arrays by calculating offsets and propagating values through iterative loops.
- `populate_array_with_random_bits`: Iteratively calls a routine to get values, masks certain bits, and stores those masked values into a local array while updating a global pointer.
- `traverse_function_pointer_table`: This routine recursively or iteratively traverses a table of function pointers, using offsets or calculated indices to call subsequent routines.
- `execute_indirect_calls_2`: Processes multiple indirect function calls based on values from a local structure, with conditional execution paths that depend on a global variable and a local flag.
- `execute_indirect_calls_3`: Processes multiple indirect function calls based on values from a local structure, with conditional execution paths that depend on a global variable.
- `process_indirect_calls`: Executes multiple indirect function calls using parameters loaded from a local structure, with conditional execution paths based on a global variable and a local flag.
- `execute_indirect_calls`: Processes multiple indirect function calls based on values from a local structure, with conditional execution paths that depend on a global variable and a counter in the structure.
- `call_with_offset_index`: Reads a value from a table using an offset derived from a global variable and input parameters, then calls another routine with that value.
- `process_poker_game_logic`: Updates game state, evaluates hand patterns, and manages betting or win/loss strings based on certain card or hand conditions.
- `evaluate_hand_pattern`: Calculates a score or hand rank by analyzing the frequency of certain values within an array.
- `evaluate_set_pattern`: calculates the frequency of certain remainder values from a provided array and returns a priority-based score or pattern type based on those frequencies.
- `execute_nested_loops`: Executes a series of nested loops that perform library calls and pointer dereferencing or updating a global variable.
- `update_poker_hand_logic`: Updates a global variable or memory buffer by iterating through a series of calculated offsets and values, or checking hand-strength-related strings like Flush and Full House against a pointer-based table.
- `initialize_game_data_tables_3`: Initializes a series of memory-mapped tables or arrays by calculating offsets and storing incremental or fixed values.
- `initialize_bet_and_card_tables`: Sets up initial bet minimums and maximums based on a global state, clears a local array, and implements a loop that checks a global variable against specific range boundaries to potentially execute a sequence of conditional updates or string-based or pointer-based table modifications.
- `modify_global_tables`: iteratively calls routines to update or write values to different offsets within a global table or array structure.
- `calculate_weighted_score_sum`: iterates through a table of short integers to accumulate a weighted sum based on whether certain values match a target or fall within a specific range.
- `process_string_and_pointers`: Calculates pointer offsets and calls a routine with a pointer to a local stack variable and a string length or function pointer.
- `branch_and_iterate_with_pointer_access`: Checks if a parameter is zero to branch and call another routine with a different constant, then enters a loop that passes a local stack variable address to another routine while iterating through a range of values or a loop count determined by another routine's return value.
- `execute_delay_loop`: Performs a simple loop that increments a register until it reaches a fixed value.
- `update_card_display_table`: Iterates through a table to update string buffers or values based on card-related string references and calculated offsets.
- `frag_pea_jsr_addq_bra`: Short code fragment that pushes the address of a location in the code onto the stack, then pushes the address of the memory at an offset from a5 onto the stack, then calls append_string_to_buffer, then adds 8 to a7, then jumps to another point in the code.
- `frag_pea_jsr_addq_bra_2`: Short code fragment that pushes the address of a location in the code onto the stack, then pushes the address of the memory at an offset from a5 onto the stack, then calls append_string_to_buffer, then adds 8 to a7, then jumps to another point in the code.
- `frag_pea_jsr_addq_bra_3`: Short code fragment that pushes the address of a location in the code onto the stack, then pushes the address of the memory at an offset from a5 onto the stack, then calls append_string_to_buffer, then adds 8 to a7, then jumps to another point in the code.
- `update_card_display`: Processes card display logic by modifying a counter at offset -2 relative to a5, checking conditions based on values at offsets 10 and 14, and updating a global variable at offset -6.
- `build_string_structure_list`: Iterates through a null-terminated string to allocate and populate a linked list of structures, where each structure contains a pointer to a string-related object and an integer offset.
- `setup_audio_timer`: Initializes an audio timer structure and configures audio device parameters or nodes using library calls.
- `initialize_and_process`: Initializes a structure and processes data based on conditional checks, then calls multiple library functions with computed parameters.
- `initialize_audio_timer_struct`: Initializes a memory structure with pointers and constant values, then configures timer and audio device parameters or node values.
- `initialize_audio_timer`: Stores three passed parameters into a global variable.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator, and returns the original destination pointer.
- `compute_string_length`: Receives a pointer to a zero-terminated byte string, walks forward byte by byte until it finds the terminator, and returns the number of bytes before the terminator.
- `update_global_value`: Calculates a new value by adding a constant to a value retrieved from a global variable and stores the result back to a global variable.
- `store_word_to_global`: Stores a 32-bit value into a global memory location.
- `unpack_nested_stream_data`: Recursively calls a routine to extract bytes and writes them to a destination buffer, continuing until a counter or a return value of negative one is encountered.
- `update_structure_with_seek`: Performs a seek operation and updates a value in a structure based on conditional range validation or pointer manipulation.
- `calculate_array_difference`: Calculates a value based on the difference between two array elements and a file or stream seek or a third value.
- `subtract_array_difference`: If the first element of an array is non-zero, subtracts the difference between the first two elements from a stored longword and returns it.
- `insert_bytes_to_buffer`: Iteratively inserts bytes from a source pointer into a destination buffer, checking for valid pointer ranges and stopping if a failure or a certain count is reached.
- `increment_stream_pointer`: Increments a pointer stored in a memory location while extracting a byte from the pointed-to address, or recursively calls itself if a boundary or a certain value is met.
- `process_stream_and_index`: Reads data from a stream and updates a structure or a pointer-based index or flag based on certain range checks.
- `find_pointer_in_list`: Searches a list for a zeroed entry and updates a structure with a result if a match is found.
- `search_and_update_table`: Searches a table for a matching string and, if a match is found, updates a structure with a result and a flag.
- `compare_two_byte_strings`: Compares two byte strings byte by byte, returning a negative or positive value if a difference is found, or zero if they match or a limit is reached.
- `perform_no_operation`: This routine is empty and performs no operations before returning.
- `execute_indirect_call`: Performs an indirect call to a pointer stored in a register.
- `seek_file_position`: Performs a seek operation on a file or stream, updating a status variable or returning an error if the target position or pointer is invalid.
- `create_file_with_flags`: Calls a routine to create or truncate a file using a specific flag.
- `search_and_modify_table_entry`: Iteratively searches a table for a null entry while checking a loop limit, then conditionally performs a sequence of file-related operations and updates the table or a secondary value with new status or parameter data.
- `read_file_stream`: Reads a block of data from a file or stream, updating a status variable and checking for invalid file or buffer offsets.
- `clear_memory_and_execute_library_calls`: Clears a block of memory by zeroing a range of bytes, updates a global variable, and conditionally calls functions using a pointer from a global structure.
- `clear_and_execute`: Clears memory and executes library calls related to window data parsing and system operations.
- `do_nothing_routine`: this routine is empty and performs no operations before returning.
- `parse_and_set_window_data`: Performs a complex sequence of library calls and pointer manipulations, including writing a specific string or constant to a memory location and updating a pointer-based structure.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `parse_string_format_list`: Parses a source string to extract and store a sequence of formatted or quoted strings into a destination array.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `append_string_to_buffer`: finds the null terminator of a destination string and copies a source string to that location, stopping if a maximum buffer length is reached.
- `insert_string_at_end`: Finds the end of a destination string and writes a source string into that position for a limited count, ensuring a terminator is present.
- `copy_limited_string_with_pad`: Copies bytes from a source pointer to a destination pointer for a given count, stopping on a zero byte, and fills any remaining space with zero bytes.
- `update_window_pointer`: Updates a global variable with a result from a function call, then traverses a pointer-based structure to assign a new value to a field in a provided pointer.
- `set_and_restore_pointer`: Stores a pointer into a global variable and subsequently restores the original value to that same location.
- `store_and_execute_bytecode`: Stores a parameter into a global variable, calls a routine that parses a custom instruction stream, and then clears the stored global variable.
- `update_pointer_and_increment`: Increments a value at a pointer location and then stores a new byte at the address previously pointed to by that location.
- `copy_table_values_to_pointer`: Iteratively copies bytes from a table or memory location to a destination pointer using values returned by another routine, and optionally writes a constant value if certain sign-based or parameter-based conditions are met.
- `parse_bytecode_instructions`: Parses a custom instruction stream to update a destination pointer or a global variable using a lookup or arithmetic routine.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `handle_signed_division_result`: Performs a signed division and negates the result if the dividend or certain register input is negative.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1, returning a 64-bit result.
- `insert_byte_into_buffer`: Checks if a pointer-based range is valid, then stores a byte at the pointer's location and increments the pointer.
- `iterate_address_range`: Increments a value and calls another routine recursively until a certain limit is reached.
- `clear_and_traverse_node_data`: Checks a pointer and a flag in a structure, potentially calls a routine recursively, and then clears the contents of the structure or node.
- `update_index_range`: Updates a pointer or value in a structure based on range validation and conditional logic.
- `find_zero_in_list`: Iterates through a list of pointers, checking a byte at an offset, and returns a pointer to a zeroed entry if a certain byte is null.
- `validate_and_update_index`: Checks if an index is within a certain range and updates a structure with a new pointer or a certain value or flag based on the result of a table check.
- `traverse_and_clear_list`: Iteratively traverses a linked list and calls another routine on each node until a null pointer is encountered, then clears a global pointer.
- `traverse_linked_list`: Iteratively traverses a linked list and calls another routine on each node until a null pointer is encountered.
- `insert_into_list`: Inserts a new node into a linked list by updating a global pointer and copying a value from a source pointer.
- `forward_list_insertion`: Acts as a thin wrapper for adding a node to the head of a list by passing its argument directly to a linked list insertion helper.
- `search_and_replace_list_node`: Searches a linked list for a specific node and replaces a pointer in the list or a global variable with the pointer from the found node.
- `check_valid_index`: Checks if an index is within the bounds of a table and if the entry at that index is non-zero, returning a boolean-like value.
- `execute_file_operations`: Executes a file-related operation and updates a global variable with a result or returns an error code if the operation fails.
- `write_file_stream`: Writes a block of data to a file or stream, checking for valid stream alignment and updating a status variable if an error occurs or if the stream is invalid.
- `check_stream_buffer_status`: Checks if a certain bit is set in the return value of a function call and conditionally executes another routine or returns a status.
- `fetch_output_stream_and_write`: Gets the output stream through a DOS wrapper and passes it to a write helper, then makes a second helper call with the constant one.
- `execute_conditional_pointer`: Checks if a pointer in a structure is non-zero and, if so, calls the routine pointed to by that address.
- `execute_recursive_tasks`: Recursively calls another routine while performing a series of conditional pointer dereferences and indirect function calls.
- `execute_recursive_tasks_2`: Executes recursive tasks by checking memory conditions, calling library functions with parameters from memory locations, and managing stack pointers.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `process_and_clear_entry`: Validates if an index is within a certain range and checks if a pointer-referenced value is non-zero, then potentially calls another routine before clearing that value.
- `multiply_32_bit_values`: Returns a 64-bit product formed from the high and low 16-bit halves of two 32-bit operands.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_indirect_function`: Loads a function pointer from an offset within a global pointer and executes it.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `execute_indirect_function`: calculates a function pointer by applying two offsets to a pointer retrieved from a global variable, then executes that function.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `initialize_struct_fields`: Checks a library function return value and, if successful, initializes several fields in a new memory structure with a pointer, a constant, and a byte value.
- `initialize_structure_with_library_call`: Calls a library function and, if successful, initializes certain fields in a new memory location with a pointer, a constant, and a value from another location.
- `initialize_struct_members`: Copies a pointer and some bytes to a new memory location and then calls another routine.
- `set_pointer_with_offset`: Stores a pointer to a memory location into another memory location with a fixed offset applied to that pointer.
- `release_and_reset_record`: If the pointer at offset 10 is set, calls an unnamed release routine on the record first.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `update_node_with_value`: If a certain pointer is non-zero, writes a constant byte and a word or dword to a destination offset within a structure.
- `update_target_data`: Calls another routine and, if the result is non-zero, writes a byte and a word or dword to a destination offset.
- `update_memory_values`: Stores a byte and a word or dword into a memory location using an offset from a pointer.
- `set_max_values`: Sets a byte and two dwords at a given pointer offset to maximum possible values before calling another routine.
- `initialize_memory_with_max_values`: Sets a byte and two dwords to certain maximum values and calls another routine.
- `call_indirect_library_function`: Loads a library function pointer from a global variable offset and calls it.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `process_pointer_chain_and_structs`: Copies a block of data from a global source, then uses a returned pointer to write new values to a structure and recursively calls other routines.
- `call_library_function_with_argument`: Calls another routine passing the input pointer as an argument via a library function offset.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_indirect_library_function_2`: Calls a library function using a pointer retrieved from a global variable offset.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_indirect_library_function_3`: Executes a library function located at an offset from a pointer stored in a global variable.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_indirect_library_function_4`: Performs an indirect call to a library function using a pointer from a global variable.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_indirect_library_function_5`: Retrieves a function pointer from a global variable and executes it.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_indirect_library_function_6`: Uses a pointer from a global variable to call a library function at a specific offset.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_indirect_library_function_7`: Invokes a library function via a pointer dereferenced from a global variable.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_indirect_library_function_8`: Calls a library function using a pointer offset from a global variable base.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `execute_indirect_function_2`: Loads a function pointer from a memory offset and executes it.
- `call_library_vector_426`: Library stub: loads a library base from a global variable and calls the library function at offset minus 426 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `execute_indirect_function_3`: loads a function pointer from a table or a global pointer offset and calls it.
- `execute_indirect_function_4`: loads a function pointer from a nested table offset and executes it.
- `execute_indirect_function_pointer`: loads a function pointer from a table or pointer-chain and executes it.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `execute_indirect_function_5`: loads a function pointer from a table and executes it.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_2`: loads a function pointer from a table located at a fixed offset from a global pointer and executes it.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `execute_indirect_function_6`: Retrieves a function pointer from a nested table or structure and executes it.
- `execute_indirect_function_7`: Loads a function pointer from amemory offset and executes it.
- `execute_indirect_function_pointer_3`: Loads a function pointer from a secondary offset within a table and executes it.
- `execute_indirect_function_8`: Retrieves a function pointer from a nested pointer or table structure and executes it.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `execute_indirect_pointer`: Retrieves a pointer from a table using an offset from a global variable and executes it.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `execute_indirect_function_9`: Loads a function pointer from a nested offset within a global table and executes it.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `execute_indirect_pointer_large_offset`: Retrieves a pointer from a table using a large offset from a global variable and executes it.
- `execute_indirect_function_pointer_4`: loads a function pointer from a nested pointer or table and executes it.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_5`: Loads a function pointer from a nested pointer or table structure and calls it.

### Trump Castle.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_jump`: Copies a byte from a source address to a fixed offset in a register, performs a subtraction operation, then jumps to a fixed address.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_tst_move_rte`: Short code fragment that tests a fixed memory address (2 times in a row), then copies 16271 to a fixed memory address, then returns from the exception.
