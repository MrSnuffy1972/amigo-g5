# Game 1136 (blade of destiny - realms of arkania (1992)(u. s. gold)[cr csl][b](disk 1 of 8)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01136). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 25 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 173 of 173 named (0 author symbols), 173 of 173 described (37 by rule, 113 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 83 of 83 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3557 of 3557 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 180397 |
| code | 10600 |
| data | 513073 |
| padding | 7098 |
| unused | 189952 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Blade of Destiny - Realms Of Arkania (1992)(U. S. Gold)[cr CSL][b](Disk 1 of 8)[a].adf:Blade-Main | 8 | 45 | 2 |
| AddBuffers | 2 | 117 | 2 |
| Assign | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| Copy | 2 | 117 | 2 |
| Makedir | 2 | 117 | 2 |
| LoadWB | 12 | 475 | 30 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Blade of Destiny - Realms Of Arkania (1992)(U. S. Gold)[cr CSL][b](Disk 1 of 8)[a].adf:Blade-Intro | 2 | 267 | 8 |
| Blade of Destiny - Realms Of Arkania (1992)(U. S. Gold)[cr CSL][b](Disk 1 of 8)[a].adf:HardDisk Install | 2 | 267 | 8 |
| icon.library | 15 | 1414 | 108 |
| Blade of Destiny - Realms Of Arkania (1992)(U. S. Gold)[cr CSL][b](Disk 1 of 8)[a].adf:Crystal | 5 | 45 | 2 |
| Blade of Destiny - Realms Of Arkania (1992)(U. S. Gold)[cr CSL][b](Disk 1 of 8)[a].adf:Character-Generation | 10 | 45 | 2 |
| Blade of Destiny - Realms Of Arkania (1992)(U. S. Gold)[cr CSL][b](Disk 1 of 8)[a].adf:bootblock | 1 | 18 | 1 |

## Cases handled

- blade of destiny - realms of arkania (1992)(u. s. gold)[cr csl][b](disk 1 of 8)[a]/Blade of Destiny - Realms Of Arkania (1992)(U. S. Gold)[cr CSL][b](Disk 1 of 8)[a].adf:Blade-Main: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- blade of destiny - realms of arkania (1992)(u. s. gold)[cr csl][b](disk 1 of 8)[a]/Blade of Destiny - Realms Of Arkania (1992)(U. S. Gold)[cr CSL][b](Disk 1 of 8)[a].adf:Blade-Intro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- blade of destiny - realms of arkania (1992)(u. s. gold)[cr csl][b](disk 1 of 8)[a]/Blade of Destiny - Realms Of Arkania (1992)(U. S. Gold)[cr CSL][b](Disk 1 of 8)[a].adf:HardDisk Install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- blade of destiny - realms of arkania (1992)(u. s. gold)[cr csl][b](disk 1 of 8)[a]/Blade of Destiny - Realms Of Arkania (1992)(U. S. Gold)[cr CSL][b](Disk 1 of 8)[a].adf:Crystal: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- blade of destiny - realms of arkania (1992)(u. s. gold)[cr csl][b](disk 1 of 8)[a]/Blade of Destiny - Realms Of Arkania (1992)(U. S. Gold)[cr CSL][b](Disk 1 of 8)[a].adf:Character-Generation: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Blade of Destiny - Realms Of Arkania (1992)(U. S. Gold)[cr CSL][b](Disk 1 of 8)[a].adf:Blade-Intro

## Copy protection found

- none found

## Functions


### Blade of Destiny - Realms Of Arkania (1992)(U. S. Gold)[cr CSL][b](Disk 1 of 8)[a].adf:Blade-Main

- `initialize_system_memory`: Performs a series of function calls, decrements a counter in a loop, and checks a specific memory pattern to update a global variable.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### AddBuffers

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `branch_to_offset`: Moves the value from d1 to d0 and branches to an instruction at a fixed offset.

### Assign

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value in a specific field, then writes a header and a sequence of words into a buffer.
- `move_d1_to_d0_and_branch`: Moves the contents of register d1 to d0 and branches to another location.

### EndCLI

- `find_max_and_prepare_buffer_3`: Walks a linked list to find the maximum value in a specific field, then writes a header and a sequence of incrementing words into a buffer.
- `move_and_branch`: Moves a longword from register d1 to d0 and then branches to another routine.

### Copy

- `find_max_and_prepare_buffer_4`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `move_and_branch_2`: Copies the contents of register d1 into register d0 and performs a relative branch.

### Makedir

- `find_max_and_prepare_buffer_5`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of incrementing words into a buffer.
- `move_and_branch_3`: Transfers a longword from d1 to d0 and executes a short branch.

### LoadWB

- `parse_workbench_path_string`: Parses a string from a buffer, handling quoted text and converting specific character sequences into newline or escape characters, then stores the processed result into a new buffer.
- `execute_library_callbacks`: Calls a function pointer from a global variable, then checks two other pointers in a global structure to call them via the library in a6 if they are non-zero.
- `invoke_two_function_pointers`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `call_library_function_2`: Loads an address into a1, calls a library function with d0 set to zero, stores the result in a global variable, and returns if the result is not zero.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_dup_lock`: Library stub: loads its arguments and the dos library base, calls DupLock and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `initialize_workbench_loading`: Performs a series of checks and calls, including XORing a global buffer with a constant and invoking various table-based or string-based routines.
- `compare_strings`: The routine compares two null-terminated strings byte by byte.
- `build_linked_list_from_table`: The routine walks a table of pointers using a global variable as a base, allocates a node via another routine and links the nodes into a chain.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_find_task`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_put_msg`: Library stub: loads its arguments and the exec library base, calls PutMsg and returns its result to the caller.
- `call_get_msg`: Library stub: loads its arguments and the exec library base, calls GetMsg and returns its result to the caller.
- `call_wait_port`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_set_function`: Library stub: loads its arguments and the exec library base, calls SetFunction and returns its result to the caller.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_copy_mem`: Library stub: loads its arguments and the exec library base, calls CopyMem and returns its result to the caller.
- `compare_and_copy_memory`: Compares bytes from a source pointer in a0 and a byte from a table at a relative offset, then copies the bytes if they are not equal.
- `load_stack_and_return`: Loads a pointer from a relative offset into a2, restores two registers from the stack, and returns.
- `pass_params_to_subroutine`: Calls another routine with a global value, its parameter and the address of an eight-byte stack slot.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Disk-Validator

- `format_and_update_disk_status`: Converts a value from a global variable into a character string and stores it in a buffer, then updates several fields in a structure including a counter, a bitmask, and a series of values derived from a loop that processes data from a table.

### Ram-Handler

- `execute_indirect_jump`: Sets bit 2 of d0, calls a routine via a pointer at an offset from a base in a6, and then performs an indirect jump using a value in a6.

### Blade of Destiny - Realms Of Arkania (1992)(U. S. Gold)[cr CSL][b](Disk 1 of 8)[a].adf:Blade-Intro

- `apply_amiga_dos_relocations`: Parses a list of data blocks to identify and process specific hunk types, performing memory allocation and applying relocation offsets to data segments.
- `process_list_and_call_library`: Decrements a register in a loop, performs pointer arithmetic on a list, and calls a library function based on a comparison.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_data_blocks`: Reads a value from a pointer in a0, calls another routine to calculate an offset, and then enters a loop that calls another routine to copy bytes to a destination in a3 based on calculated counts.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `extract_bits_and_copy`: Performs bitwise shifts and rotations on data from a pointer, updates a hardware register, and copies bytes to a destination while decrementing a counter.
- `perform_trap_operation`: Moves a word from an offset of a2 into d4, sets bit 3 of d3, executes a trap, and performs a logical OR of d0 into d1.

### Blade of Destiny - Realms Of Arkania (1992)(U. S. Gold)[cr CSL][b](Disk 1 of 8)[a].adf:HardDisk Install

- `process_amiga_hunk_relocations`: Iterates through a list of data blocks, identifying specific hunk types to perform memory allocation, data copying, and relocation adjustments.
- `process_memory_and_counter`: Decrements d1 and branches if it is not negative, performs arithmetic on a pointer in a1, calls a routine using a pointer from a6, and updates a value in a4 based on a calculation involving d0 and a1.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `copy_data_with_logic`: Calculates an offset based on a value from a pointer, calls two different routines, and copies bytes to a destination using a loop.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `subtract_word`: Subtracts the value in d4 from d0.
- `extract_bit_stream`: Enters a loop that performs bitwise shifts and rotations on data from a pointer in a0, updates a hardware register, and copies bytes to a destination while decrementing a counter in d0.
- `execute_trap`: Moves a word from an offset of 1548 in a2 to d4, performs a bitwise OR on d0 with 152, calls a trap, and performs a bitwise OR on d1 with the result of the trap.

### icon.library

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_library_list`: As library code, it checks a word at offset 32 of a structure, and if zero, calls a routine to manipulate a linked list by updating pointers at offsets 0 and 4 of the structure's elements; otherwise, it sets bit 3 of a byte at offset 14 of the structure in a6.
- `set_library_bit`: As library code, it sets bit 3 of a byte at offset 14 of the structure pointed to by a6 and returns 0.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `call_library_function`: As library code, it stores two 32-bit values from a structure into a new structure, calls a library function, and if that function returns 0, calls another library function via a pointer in a global variable.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_bitwise_or`: Performs a bitwise OR on d0 with 0.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `iterate_icon_chain`: Saves two registers to the stack and calls another routine with the same first argument and a second argument increased by 140, then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `manage_icon_chain_4`: This library routine calls another routine and then returns.
- `manage_icon_chain_3`: Saves the current stack pointer and a pointer from a register onto the stack, calls another routine, and then returns.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_verify_and_calculate_offset`: Saves registers a0 and a1 to the stack, calls verify_and_calculate_offset, restores the registers, and returns.
- `call_process_formatted_string_2`: Saves two registers to the stack, calls process_formatted_string_2, and then returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `clear_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `update_icon_resource`: This library code calls another routine, and if the result is non-zero, it copies 44 bytes from a source structure to a destination structure, writes several longwords to the destination structure, and if a specific value at an offset is zero, it writes a pointer to the structure into another location.
- `copy_and_update_structure`: As library code, it copies 44 bytes from a source structure in a6 to a destination structure in a2 using a loop, then copies several longwords and a byte from the source to the destination, and finally updates a pointer in a structure if a specific longword is non-zero.
- `return_zero_and_unlock`: This library routine returns zero and unlocks the resource pointed to by a6.
- `copy_icon_data_to_stack`: Library code that copies 43 bytes from a structure pointed to by a0 into a local stack buffer, then copies several fields from the same structure into local stack slots and calls another routine with the first two arguments and a pointer to a two-word stack header.
- `initialize_icon_resource_structure`: Allocates memory and populates an icon resource structure by calling several library functions to load data, assign pointers, and manage memory for various icon components.
- `populate_icon_resource_data`: Checks if a resource structure matches a specific type and ID, then sequentially calls other routines to allocate and assign memory addresses to various offsets within the structure, including a loop that populates a list of pointers.
- `update_icon_attributes`: Library code that calls another routine using values from the stack to update offsets 18 and 22 of a structure, and then performs a series of conditional calls to other routines to update various fields in a structure based on the presence of pointers and specific values.
- `release_resource_list`: Library code that loops through a list of items, calling a routine for each to release them, and then updates a global variable with a value from a structure if a specific field is non-zero.
- `clear_d6_and_branch`: Clears register d6 and branches to another location.
- `unlock_and_return`: This library routine unlocks the resource pointed to by a6 and returns.
- `process_icon_structure_callbacks_2`: Iterates through a structure to execute a series of conditional callbacks based on the presence of pointers at various offsets, including a loop that iterates through a list of pointers.
- `process_icon_structure_callbacks`: Updates a field in a structure from another field, then conditionally clears a value at offset 26 based on a bitmask check at offset 16.
- `iterate_icon_handlers_2`: Library code that iterates through a linked list of handlers, calling each one with arguments from the stack and potentially passing a scale.
- `iterate_and_call_list`: As library code, it increments a pointer in a loop until it reaches a null terminator, then calls a function pointer in a3, and if the result is non-zero, it iterates through a list of pointers in a2, calling the same function pointer for each and performing further calls based on the results.
- `process_icon_list_recursively`: Library code that iterates through a list of pointers, calling functions and updating a local stack variable, then executes a callback via a pointer or table.
- `process_chain_with_calls`: As library code, it saves registers, retrieves two values from the stack, calls one function with a modified pointer, and if the result is non-zero, calls another function with the result.
- `restore_registers_and_return`: This library routine restores d2 and d3 from the stack and returns.
- `incremental_offset_recurse`: Library code that calls another routine with the same first argument and the second argument increased by 140, then returns.
- `execute_library_chain`: As library code, it calls a routine with a pointer from a global variable and a constant, then if the result is non-zero, it calls another routine with an offset of 142 from the result, and if that result is zero, it calls a third routine with an offset of 140 from the result and the original value from the stack.
- `library_subroutine_call`: This library routine pushes a pointer to a constant onto the stack, pushes a pointer from a2, pushes d2, and calls another routine.
- `clear_d0_and_restore_registers_2`: Sets register d0 to zero and restores registers d2 through d4 and a2 from the stack before returning.
- `decrement_and_store_pair`: Library code that decrements a 16-bit counter through a pointer, and if the result is not zero, calls another routine and stores two 32-bit values into a table based on the counter and a base address.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, computes an address from a table base and the counter, and stores two 32-bit values from registers and the stack into consecutive slots of that table.
- `process_icon_resource_chain`: This library code calls another routine, and if the result is non-zero, it performs a series of conditional calls to other routines using values from the stack and the result of the first call, eventually returning either a pointer or zero.
- `move_d6_to_d0_and_branch`: Moves the contents of register d6 to d0 and branches to another location.
- `call_lib_func_and_return_zero`: Invokes a function pointer retrieved from a global variable associated with library index -210.
- `incremental_offset_recurse_2`: Library code that calls another routine with the same first argument and the second argument increased by 78, then returns.
- `compare_icon_data`: This library routine calls another routine, then iterates through a list of pointers, calling another routine for each and checking if a byte at an offset equals 61.
- `restore_registers_and_return_2`: Clears d0 and restores d2, d3, a2, and a3 from the stack before returning.
- `verify_and_calculate_offset`: As library code, it calls a routine with a value from the stack, then if a second value from the stack is non-zero, it calls another routine with that value and a constant, and finally if that result is non-zero, it subtracts the second value from the result.
- `verify_icon_match`: This library routine calls another routine, compares d6 to d5, and if they are equal, calls another routine with d5, d3, and d2 as arguments.
- `update_register_d2`: If register d4 is non-zero, it sets d2 to the value of d4 plus one and branches; otherwise it branches to another location.
- `clear_d2_and_branch`: Clears register d2 and branches to another location.
- `restore_registers_and_return_3`: Clears d0 and restores d2, d3, d4, d5, d6, and a3 from the stack before returning.
- `process_formatted_string_2`: Library code that clears a byte at an offset in a structure, calls another routine, and if the return value is non-zero, calls two more routines using values from the stack before branching to another location.
- `branch_on_result`: Library code that adds 5 to a2, pushes two values onto the stack, calls another routine, and then branches to another location if the return value in d0 is non-zero; otherwise it sets d5 to 1, increments a2 by 3, and branches to another location.
- `clear_d5_and_branch`: Clears register d5 and branches to another location.
- `process_formatted_string`: Library code that calculates an offset based on d5, enters a loop to call a routine with a byte from a table, and then conditionally calls two other routines.
- `clamp_value_to_range`: Compares a value from the stack with 48; if the value is greater than 48, d0 is set to 48, if it is less than 48, d0 is set to 57, otherwise d0 is set to the value minus 48.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `complex_library_traversal`: Library code that calls another routine and uses the result to conditionally call a second routine or perform a cleanup call.
- `call_library_function_with_parameter`: As library code, it calls a routine using a pointer from the stack, and if the result is non-zero, writes the value 10 to offsets 14 and 20 of a structure and calls another routine.
- `return_zero`: As library code, it sets d0 to 0, restores registers d2, d3, and a2 from the stack, and returns.
- `compare_library_result`: As library code, it calls a routine using a pointer from the stack, moves the result to d1, and compares it to a value at offset 36 of the stack, setting d0 to zero if they are not equal.
- `move_d1_to_d0`: As library code, it moves the value from d1 to d0, restores registers d2 and d3 from the stack, and returns.
- `call_icon_handler_with_validation`: Pushes three arguments onto the stack and calls a function pointer, then compares the returned value against a fourth argument; if they do not match, the routine returns zero, otherwise it returns the result of the call.
- `transfer_register_and_restore_stack`: Moves a longword from register d1 to register d0 and restores registers d2 and d3 from the stack before returning.
- `perform_division_logic`: If d2 is not greater than 65535, it performs several arithmetic operations involving swapping bits and dividing values using d1 and d2, then returns.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_return`: Stores the byte from d0 at the address pointed to by a3 and increments a3 before returning.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `compare_byte_and_decrement`: Loads a pointer and a byte from the stack, compares the byte at the pointer to the value from the stack, and if they match, decrements the pointer and returns; otherwise it returns.
- `clear_d0_and_branch`: Clears register d0 and branches to another location.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `check_string_byte`: Restores a0 and a1 from the stack, tests the byte pointed to by a0, and if it is not zero, decrements a0 and branches to another routine; otherwise it branches to a different location.
- `compare_memory_and_return_zero`: Decrements a value from the stack, and if it is not negative, compares a byte from a table with a byte from another table; if they match and the table byte is zero, it sets d0 to zero and returns; otherwise it returns.
- `return_conditional_constant`: Sets d0 to 1 if it is greater than zero, otherwise sets d0 to 255 and returns.
- `check_byte_and_decrement`: Checks a byte at the pointer in a0; if the byte is non-zero, it decrements d0 and branches; otherwise it branches to another location.
- `decrement_and_copy`: As library code, it decrements the pointer in a0 by 1 and calls another routine.
- `copy_bytes_with_counters`: Decrements d0 and loops while d0 is not less than or equal to 0, copying bytes from a1 to a0; if the loop finishes, it clears the byte at a0 and branches.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `conditional_library_branching`: The library code checks a parameter and, if non-zero, calls another routine and potentially calls a third routine using a value from a pointer offset.
- `iterate_icon_handlers`: Library code that iterates through a linked list of handlers, calling each one with arguments from the stack and potentially passing a scaled byte from a structure as a new argument.
- `set_d0_and_branch`: As library code, it sets d0 to 1 and branches to an instruction at a fixed offset.
- `clear_d0_and_restore_registers`: Clears register d0 and restores registers d2 through d5 and a2 from the stack before returning.
- `allocate_icon_resource`: Library code that performs a series of conditional calls to allocate memory or resources, updates a value at offset 10 of a structure, and returns either a pointer or negative one.
- `return_negative_one`: Sets register d0 to negative one and restores the previous register states from the stack before returning.
- `process_icon_attribute`: This library code calls another routine using values from the stack, stores the result at offset 12 of a structure, and then conditionally calls another routine using a value derived from a byte at offset 7 of that structure before calling a third routine to store a value at offset 8.
- `update_icon_attribute`: Library code that stores a value at offset 12 of a structure, performs conditional calls to calculate a value based on a byte at offset 7 of another structure, and writes the result to offset 8 of that structure.
- `move_a3_to_d0_and_branch`: As library code, it moves the value from a3 to d0 and branches to an instruction at a fixed offset.
- `return_negative_one_2`: Sets the first data register to negative one and restores multiple registers from the stack before returning.

### Blade of Destiny - Realms Of Arkania (1992)(U. S. Gold)[cr CSL][b](Disk 1 of 8)[a].adf:Crystal

- `init_and_check_pattern`: Invokes several routines and loops three times, then clears a memory location and, when a value matches a specific pattern, stores it and zeroes a global variable.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.

### Blade of Destiny - Realms Of Arkania (1992)(U. S. Gold)[cr CSL][b](Disk 1 of 8)[a].adf:Character-Generation

- `initialize_game_state`: Executes a sequence of function calls and checks a global pointer to swap a memory value if a specific constant is found.
- `decrement_until_minus_one_3`: Decrements d7 in a loop by adding 4 to a pointer derived from a constant until d7 reaches negative one, then returns.

### Blade of Destiny - Realms Of Arkania (1992)(U. S. Gold)[cr CSL][b](Disk 1 of 8)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
