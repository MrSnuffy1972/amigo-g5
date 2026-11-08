# Game 1063 (populous ii - trials of the olympian gods (1991)(electronic arts)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01063). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 41 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 165 of 165 named (0 author symbols), 165 of 165 described (35 by rule, 102 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 113 of 113 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3025 of 3025 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 442858 |
| code | 9600 |
| data | 395870 |
| padding | 12344 |
| unused | 40448 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts).adf:Install | 2 | 267 | 8 |
| EndCLI | 2 | 117 | 2 |
| LoadWB | 12 | 536 | 30 |
| Disk-Validator | 1 | 386 | 1 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts).adf:CREDITS.EXE | 2 | 167 | 7 |
| icon.library | 10 | 1375 | 109 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts).adf:populous.ii | 6 | 167 | 7 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts).adf:bootblock | 1 | 10 | 1 |

## Cases handled

- populous ii - trials of the olympian gods (1991)(electronic arts)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts).adf:awards.pak: high-entropy data file (7.70 bits/byte), classified as packed data
- populous ii - trials of the olympian gods (1991)(electronic arts)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts).adf:hands.pak: high-entropy data file (7.68 bits/byte), classified as packed data
- populous ii - trials of the olympian gods (1991)(electronic arts)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts).adf:end.pak: high-entropy data file (7.66 bits/byte), classified as packed data
- populous ii - trials of the olympian gods (1991)(electronic arts)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts).adf:judge.pak: high-entropy data file (7.68 bits/byte), classified as packed data
- populous ii - trials of the olympian gods (1991)(electronic arts)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts).adf:CREDITS.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- populous ii - trials of the olympian gods (1991)(electronic arts)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts).adf:populous.ii: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts).adf:Install

- `initialize_system_environment`: Initializes several global variables by calling library functions, allocates memory, and performs string operations before calling another routine.
- `alert_then_close_library`: Calls two library functions in sequence through the library base in a6 to report a problem and release a library.
- `invoke_two_function_pointers`: The routine calls two functions through pointers stored in a global variable.
- `open_titled_console_window`: Opens a console window with a fixed size and title text, then closes the file handle through a second library call.
- `read_input_line_until_marker`: Reads a line of user input, removes the trailing newline and repeats a check routine until it returns a FAIL or OK four-character code.
- `construct_path_and_copy_file`: Constructs a destination file path by appending a directory string to a source filename, then attempts to copy the file contents using a read and write loop.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `write_null_terminated_string`: Measures the length of the null-terminated string at the address in register A0, then writes that many bytes through the library function Write.

### EndCLI

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_d1_to_d0_and_branch`: It moves the value from register d1 to d0 and branches to another location.

### LoadWB

- `parse_command_string`: Parses a string by splitting it into tokens and handling quoted text or escaped characters, then stores the resulting pointers and processed characters into a structure.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `start_tool_from_resident_tag`: XOR-decodes a global string with one key applied twice, allocates a 160-byte block, and uses icon, intuition, and workbench library calls to start a tool.
- `compare_null_terminated_strings`: Compares two null-terminated strings byte by byte and returns minus one when the first is lower, one when it is higher, and zero when they match.
- `build_linked_list_from_table`: Walks a table of pointer and value pairs, allocating a fixed-size node for each entry through a memory allocation call and storing the looked-up value in it.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `compare_and_copy_string`: Compares a string at a global address with a string at the address in a0, and if they match, copies the string from a0 to the global address.
- `set_byte_values`: Writes the byte values 46, 32, 32, 32, and 32 to offsets 11 through 15 of the structure pointed to by a2, then branches.
- `compare_memory_blocks`: Compares a byte at the address in a1 with a byte at the address in a0, then branches if they are not equal.
- `load_stack_and_return`: Loads a pointer from a relative offset into a register, restores two registers from the stack, and returns.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Disk-Validator

- `update_display_and_bitmask_table`: Calculates a character representation of a value into a buffer, updates a bitmask table by iterating through a range of indices, and performs several calls to a subroutine to manipulate data structures and bitwise flags within a global state.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts).adf:CREDITS.EXE

- `parse_amigados_executable_hunks`: Checks for a specific magic constant and, if found, parses a data structure to perform memory relocations and byte copying.
- `accumulate_32bit_from_bytes`: The routine reads four consecutive bytes from a pointer in a register and builds a 32-bit value in a register by concatenating each byte into the result.
- `decode_and_copy_bytes`: The routine decodes a byte from a global variable using a bit stream produced by another routine and then copies a block of bytes from one memory area to another using two pointers, decrementing a counter until it reaches -1.
- `pop_byte_and_return`: Shifts the least significant bit of a register left by one, conditionally branches to a location outside the routine, otherwise pops a byte from the stack into that register and returns.
- `decrement_until_minus_one`: The routine calls another routine repeatedly, decrementing a short result until it reaches -1, then checks a byte from a global variable table indexed by the result plus one.
- `execute_conditional_logic`: Depending on a register value, the routine either calls another routine repeatedly until a result is zero, or performs a series of calls to another routine to decrement a value, then processes a data block by calling another routine to add values to a table.
- `decrement_and_execute_jump_table`: The routine decrements a value in a register in a loop, and if the value reaches zero, it uses a stack-based jump table to execute a routine.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `manage_library_list`: As library code, it checks a value at an offset in a structure; if zero, it removes an item from a linked list and calls another library routine, otherwise it sets bit 3 of a byte at offset 14 of the structure.
- `set_library_status_bit`: This library routine sets bit 3 of a byte at offset 14 of a structure pointed to by a6.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `call_library_function_with_error_handling`: This library routine calls a function via a pointer in a6, stores the result in a structure, and if the result is zero, calls another routine via a pointer from a global variable.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_zero_or_instruction`: This library routine performs a logical OR on register d0 with zero.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `recurse_icon_chain`: Saves two registers to the stack and calls another routine, then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `invoke_icon_chain_handler`: Pushes two pointers from the stack onto the stack and calls another routine, then returns.
- `execute_icon_chain_recursion`: Saves two pointers from registers onto the stack and calls a routine that performs incremental recursion until a termination condition is met.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_external_function`: Saves two address registers, calls another routine, and then returns.
- `call_and_clear_byte`: Clears a byte at offset 30 of the structure pointed to by a2, calls another routine, and returns if the result is non-zero.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `zero_data_padding`: This routine consists of a single instruction that performs a logical OR operation on a data register with zero, effectively leaving the register unchanged.
- `allocate_icon_resource`: This library code loads two values from the stack, calls a routine with the value in d2, and if the result is non-zero, pushes two arguments and calls another routine.
- `copy_icon_structure`: Copies 11 words from a source structure at an offset of a6 to a destination structure at a2, then updates a pointer at offset 424 of a structure pointed to by a pointer at offset 76 of a2 with the address of a2.
- `return_zero_stub_unlk`: Sets register d0 to zero, restores registers from a location offset from a6, unlinks a6, and returns.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `update_icon_structure_pointers_2`: Checks if a specific icon type is requested, then clears two fields in a structure pointed to by a4.
- `update_icon_structure_pointers`: Checks if a specific icon type is requested, then clears two fields in a structure pointed to by a4.
- `update_icon_pointers`: As library code, it calls a routine twice to update two different pointers in a structure at offset 18 and offset 22 of a4, then conditionally calls a function pointer and another routine to update offsets 50 and 54 of a structure in a2.
- `process_icon_updates`: As library code, it calls a routine in a loop, stores the result in a pointer in a4, and performs additional calls and checks based on the results and a value at offset 70 of a2.
- `clear_d4_and_branch`: Clears register d4 and branches to another location.
- `empty_routine`: Moves the contents of d4 into d0, restores registers from a location offset from a6, unlinks a6, and returns.
- `sync_icon_structure_contents`: Synchronizes an icon structure by checking flags and calling various callbacks to update data fields, lists of pointers, and nested structures.
- `update_icon_structure_contents`: Checks flags in a structure to conditionally clear an offset, then executes a series of callbacks via a function pointer to process various linked lists or pointers stored at offsets 38, 50, 54, and 70 of the structure.
- `process_icon_table`: As library code, it calls a routine twice to update offsets 22 and 26 of a structure in a2, then if offset 30 is zero, it calls a routine to update offset 50 of a2 and calls a function pointer, otherwise it iterates through a table to update values using a function pointer.
- `iterate_icon_table_entries_2`: Increments a pointer in a loop until it reaches a null value, then calls a routine using arguments from the stack, and if the result is non-zero, it iterates through a table at an offset from a2 and performs further calls.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `traverse_linked_list_node`: Pushes a2 onto the stack, loads an address from an offset of the stack pointer into a0, sets a2 to the address plus two, and branches to another routine.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `allocate_and_initialize_resource`: As library code, it calls a routine to allocate a resource of size 65536, then if successful, it calls another routine using offsets 142 and 140 of the new pointer, and finally calls a third routine using the pointer and a value from a global variable.
- `allocate_library_resource_2`: As library code, it pushes a pointer from a2 onto the stack and calls another routine, then returns d0 or a value from a2 based on the result.
- `clear_d0_and_restore_registers_2`: Sets register d0 to zero and restores registers d2, d3, and a2 from the stack before returning.
- `decrement_and_allocate_resource`: As library code, it decrements a 16-bit word at the address in a2, and if the result is not greater than zero, calls another routine and returns zero; otherwise it writes two longwords to a table.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `allocate_and_setup_resource`: This library code calls a routine with a constant, and if the result is non-zero, calculates a new address, calls another routine, and then performs a series of conditional calls to other routines.
- `branch_to_offset`: Performs a short branch to an offset within the library code.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `perform_incremental_recursion`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `call_function_with_argument`: As library code, it pushes a value from the stack onto the stack, calls a function pointer, and then moves the return value into d3 and a pointer into d4 before branching to another routine or performing a branch.
- `scan_string_for_character`: As library code, it calls another routine, then loops to compare bytes from a pointer in a3 with the character 61 until a null terminator is reached.
- `advance_pointer_and_check`: As library code, it loads a pointer from a2 into a3, moves the value at that pointer into d0, and branches to an earlier instruction if d0 is not zero, otherwise it returns.
- `call_function_pointer`: This library code loads two values from the stack into registers d2 and d3, pushes d3 onto the stack, calls a function pointer, and then moves the result from d0 into d5 before branching.
- `calculate_address_difference`: As library code, it calls a routine using a value from d2, then if the result is not zero, it subtracts the original value in d2 from the returned value in d0 and branches to another location.
- `execute_callback_with_checksum`: As library code, it calls a function pointer, then if the result matches a value in d5, it calls another function pointer and returns 1, otherwise it returns 0.
- `increment_d2_and_branch`: If register d4 is not zero, it copies d4 to d2, increments d2 by one, and branches to another location; otherwise it branches to another location.
- `clear_and_restore_registers`: Clears register d2 and checks if it is non-zero; if it is non-zero, it branches to another routine, otherwise it clears register d0, restores registers d2 through d5 from the stack, and returns.
- `process_encoded_data`: This library code clears a byte at offset 30 of a structure in a2, calls a routine with a pointer and a constant, and if the result is non-zero, calls another routine with three arguments and then branches.
- `execute_branching_logic`: As library code, it increments a pointer in a4, pushes two values onto the stack, calls a routine, and then either branches to another routine if d0 is non-zero or sets d3 to 1 and branches to another location.
- `clear_d3_and_branch`: As library code, it clears register d3 and branches to another location.
- `process_encoded_data_table`: As library code, it calculates a value based on d3 and d2, loops to call a function pointer for each byte in a table, and then performs several library calls using the results.
- `calculate_difference`: Compares a value from a stack pointer to 48, and if it is not equal to 48, it calculates the difference between 48 and that value and returns it in d0; otherwise it returns -1.
- `return_negative_one`: Sets d0 to negative one and returns.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `allocate_library_resource`: As library code, it calls another routine with a constant, and if the result is non-zero, it writes the result to two locations and calls another routine with a pointer to the result.
- `clear_d0_and_restore_registers`: This library routine clears register d0 and restores registers a2 and d2 from the stack before returning.
- `call_handler_with_stack_args`: Copies two longwords from the stack into registers, pushes them back onto the stack, calls a routine at an address stored in a register, and then returns zero if a comparison of a stack value and d0 fails, otherwise it returns d0.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `call_library_function`: As library code, it loads two longwords from the stack, pushes them onto the stack, and calls a routine at a specific address, then compares a value on the stack to a constant and returns either the result or zero.
- `copy_d1_to_d0`: Copies the contents of register d1 into register d0 and returns.
- `divide_long_by_word`: Performs a division of a 32-bit value in d1 by a 16-bit value in d2, using different logic paths based on whether d2 is greater than 65535.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_return`: Stores the byte from d0 into the memory location pointed to by a3 and increments a3 before returning.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `find_matching_byte`: As library code, it compares a byte from a structure at the address in a0 with a byte in d0, and if they match, decrements the pointer in a0 and returns it in d0.
- `clear_d0_and_branch`: Clears register d0 and branches to another location.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `search_for_zero_byte`: Loops through a memory range starting at the pointer in a0, checking for a zero byte, and decrements a0 before returning.
- `compare_string_bytes`: As library code, it compares a byte from a structure at the address in a1 with a byte at the address in a0, and returns zero if they match and the byte is zero.
- `set_conditional_d0`: Sets register d0 to one if it is greater than zero, otherwise sets it to 255 and returns.
- `check_byte_and_decrement`: As library code, it tests a byte at the address in a0, and if it is non-zero, decrements the value in d0 and branches if it is greater than zero.
- `decrement_pointer_and_call`: Decrements the pointer in a0 by one and calls another routine before returning.
- `copy_bytes_with_counter`: Copies bytes from a source pointer in a1 to a destination pointer in a0 while decrementing a counter in d0, and performs a final single byte write if the counter reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `load_icon_parameters`: Loads three values from the stack into registers d2, d3, and a2, then branches to another routine.
- `iterate_icon_list`: As library code, it repeatedly calls a routine using parameters from the stack, modifies a value in a2 based on the result, and loops if the result is non-zero.
- `return_zero_stub`: Sets register d0 to zero, restores registers from the stack, and returns.
- `propagate_icon_changes`: As library code, it calls another routine with three parameters, and if successful, performs several more calls to other routines to update values in a structure and perform calculations using a third routine.
- `return_negative_one_stub`: Sets register d0 to negative one, restores registers from the stack, and returns.
- `load_library_parameters`: Loads three values from the stack into registers d2, d3, and d4, then branches to another routine.
- `update_library_state`: As library code, it pushes a constant and two registers onto the stack, calls another routine, stores the result at an offset, and then conditionally updates a register from a stack value.
- `propagate_library_updates`: As library code, it calls a routine with parameters from the stack, performs arithmetic on a byte from a structure, calls another routine, and continues a loop based on the results.
- `return_negative_one_with_extra_registers`: Sets register d0 to negative one, restores a larger set of registers from the stack, and returns.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts).adf:populous.ii

- `parse_amiga_hunk_relocations`: Checks for a specific magic constant to identify an AmigaDOS executable hunk, then processes relocation entries by adding offsets to memory addresses.
- `accumulate_32bit_from_bytes_2`: Reads four consecutive bytes from a pointer in a0 and builds a 32-bit value in d0 by concatenating each byte into the result.
- `decode_and_copy_bytes_2`: Decodes a byte stream using another routine to determine a count and a bitmask, then copies a block of bytes from a source pointed to by a6 to a destination pointed to by a3 using a counter.
- `shift_and_pop_stack_byte`: Shifts the least significant bit of a register left by one, conditionally branches based on the result, otherwise pops a byte from the stack into that register and performs a signed arithmetic shift right by one before returning.
- `decrement_until_minus_one_2`: Calls another routine in a loop until it returns -1, then checks a byte from a table at an offset determined by the result; if that byte is non-zero, it calls another routine in a loop until a register is zero.
- `execute_complex_logic_blocks`: Performs different sequences of calls to another routine and data manipulations based on the value of d6, including looping through bit-stream decoding and processing a jump table or data block.
- `execute_jump_table`: Decrements d0 in a loop, then calculates an offset from a1, pushes several 32-bit constants onto the stack, and jumps to the address at the top of the stack.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
