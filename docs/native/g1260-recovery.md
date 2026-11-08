# Game 1260 (realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl][a](disk 1 of 8)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01260). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 26 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 173 of 173 named (0 author symbols), 173 of 173 described (37 by rule, 113 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 83 of 83 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3557 of 3557 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 180474 |
| code | 10600 |
| data | 513609 |
| padding | 7509 |
| unused | 188928 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL][a](Disk 1 of 8).adf:Blade-Main | 8 | 45 | 2 |
| AddBuffers | 2 | 117 | 2 |
| Assign | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| Copy | 2 | 117 | 2 |
| Makedir | 2 | 117 | 2 |
| LoadWB | 12 | 475 | 30 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL][a](Disk 1 of 8).adf:Blade-Intro | 2 | 267 | 8 |
| Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL][a](Disk 1 of 8).adf:HardDisk Install | 2 | 267 | 8 |
| icon.library | 15 | 1414 | 108 |
| Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL][a](Disk 1 of 8).adf:Crystal | 5 | 45 | 2 |
| Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL][a](Disk 1 of 8).adf:Character-Generation | 10 | 45 | 2 |
| Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL][a](Disk 1 of 8).adf:bootblock | 1 | 18 | 1 |

## Cases handled

- realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl][a](disk 1 of 8)/Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL][a](Disk 1 of 8).adf:Blade-Main: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl][a](disk 1 of 8)/Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL][a](Disk 1 of 8).adf:Blade-Intro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl][a](disk 1 of 8)/Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL][a](Disk 1 of 8).adf:HardDisk Install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl][a](disk 1 of 8)/Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL][a](Disk 1 of 8).adf:Crystal: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl][a](disk 1 of 8)/Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL][a](Disk 1 of 8).adf:Character-Generation: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL][a](Disk 1 of 8).adf:Blade-Intro

## Copy protection found

- none found

## Functions


### Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL][a](Disk 1 of 8).adf:Blade-Main

- `initialize_system_memory`: Performs a series of function calls, decrements a counter in a loop, and checks a specific memory pattern to update a global variable.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### AddBuffers

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `branch_with_move`: Moves the contents of d1 into d0 and branches to another location.

### Assign

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, then if a condition is met, writes a header and fills a buffer with sequential words.
- `move_d1_to_d0_and_branch`: Moves the contents of d1 to d0 and branches to another location.

### EndCLI

- `find_max_and_prepare_buffer_3`: Walks a linked list of entries to find the maximum value in a field, then if a condition is met, writes a header and fills a buffer with sequential words.
- `move_and_branch`: Moves a longword from register d1 to d0 and then performs a relative branch.

### Copy

- `find_max_and_prepare_buffer_4`: Walks a linked list of entries to find the maximum value in a field and, if a condition is met, writes a header and fills a buffer with sequential words.
- `move_and_branch_2`: Copies the contents of register d1 into d0 and branches to another location.

### Makedir

- `find_max_and_prepare_buffer_5`: Walks a linked list of entries to find the maximum value in a specific field, then writes a header and a sequence of incrementing words into a buffer.
- `move_and_branch_3`: Transfers a longword from d1 to d0 and executes a short branch.

### LoadWB

- `parse_workbench_string`: Parses a string from a buffer, handling quoted text and converting specific character sequences into newline or escape characters, then stores the processed result into a new buffer.
- `execute_callback_chain`: Stores a value from the stack into a global variable, then calls a function pointer from a global variable, followed by two more function calls from a different global variable if certain conditions are met.
- `invoke_two_function_pointers`: Calls two function pointers at fixed negative offsets from a base register, then returns.
- `call_and_store_result`: Loads an address into a1, calls a routine using a pointer from a6, stores the result from d0 into a global variable at offset 8, and returns if the result is zero.
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
- `compare_and_copy_memory`: Compares a block of memory starting at the address in a2 with a second block of memory starting at a location relative to the program counter, and if they are not equal, copies bytes from the first block to the second block in a loop.
- `load_stack_and_return`: Loads a pointer from a relative offset into a2, restores two registers from the stack, and returns.
- `pass_params_to_subroutine`: Calls another routine with a global value, its parameter and the address of an eight-byte stack slot.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Disk-Validator

- `update_disk_validation_data`: Calculates a value based on a global variable, stores it in a structure, and performs a series of calls to update various data fields and bitmasks within a control structure.

### Ram-Handler

- `execute_indirect_jump`: Sets bit 2 of d0, calls a routine at an offset of minus 656 from a pointer in a6, and then performs an indirect jump using the address in a6.

### Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL][a](Disk 1 of 8).adf:Blade-Intro

- `relocate_amiga_hunks`: Parses a data stream containing AmigaDOS hunk type codes to perform memory relocation and data copying.
- `decrement_and_call_library`: Decrements a word in d1 in a loop, performs pointer arithmetic on a1, calls a routine using a pointer from a6, and updates a value in a4 based on a calculation involving a pointer.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_bit_stream_2`: Processes a stream of data from a pointer in a0 by performing arithmetic and bitwise shifts, calling another routine multiple times to update a pointer in a3, and writing bytes to the memory at a3 in a loop.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_stream`: Performs bitwise shifts and rotations on registers d1 and d5 in a loop, updates a pointer in a3, and writes the final value of a3 to a hardware register.
- `perform_trap_operation`: Moves a 16-bit word from an offset of 3084 in a2 to d4, sets bit 3 of d3, calls a trap, and performs a logical OR of d0 into d1.

### Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL][a](Disk 1 of 8).adf:HardDisk Install

- `process_amiga_hunk_data`: Parses a data stream containing AmigaDOS hunk types, performing memory allocation and relocation updates by calling other routines.
- `process_memory_allocation`: Decrements d1 in a loop, performs several pointer manipulations, calls a routine at an offset of minus 526 from a6, and calls another routine at an offset of minus 161 from a6.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `copy_data_with_checks`: Checks a byte from a pointer in a0, calculates an offset, and enters a loop that calls two different routines to copy bytes from a source to a destination based on a counter in d1.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `subtract_word`: Subtracts the value in d4 from d0.
- `extract_bit_stream`: Performs bitwise extraction from a data stream pointed to by a0, using shifts and rotations to process variable-length bit sequences.
- `execute_trap`: Moves a word from an offset of 3084 bytes from a2 into d4, performs a bitwise OR on d0 with 152, calls a trap, and performs a bitwise OR on d1 with d0.

### icon.library

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_icon_structure`: As library code, it checks a word at offset 32 of a6; if zero, it calls a routine at an offset of minus 526 from a6 and updates a linked list, otherwise it sets bit 3 of a byte at offset 14 of the structure in a6.
- `set_icon_bit`: As library code, it sets bit 3 of a byte at offset 14 of the structure pointed to by a6 and returns 0.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `call_library_function`: As library code, it stores two 32-bit values from a structure into a stack frame, calls a library function via a vector in a6, and if the result is zero, calls another library function using a pointer from a global variable.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_bitwise_or`: Performs a bitwise OR on d0 with 0.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `traverse_icon_chain`: Saves two registers to the stack and calls another routine with the same first argument and a second argument increased by 140, then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `allocate_icon_resource_5`: Saves two registers onto the stack, calls a routine to allocate 65536 bytes, restores the registers, and returns.
- `traverse_icon_chain_2`: Saves two registers to the stack, calls another routine with the same first argument and an incremented second argument, then restores the registers and returns.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_process_resource_data`: Saves registers a0 and a1 to the stack, calls process_resource_data, and then returns.
- `call_other_routine_2`: Saves two registers to the stack, calls another routine, then restores the registers and returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `update_icon_resource`: As library code, it calls a routine to allocate memory, then if successful, calls another routine to populate a structure, copies 44 bytes from a stack buffer to a structure at an offset from a pointer, and finally calls a routine to update a pointer in a structure if a specific field is zero.
- `copy_and_update_structure`: Copies 44 bytes from a source to a destination, then copies several fields from a stack-based structure to a destination structure, and finally updates a pointer in a structure if a specific field is non-zero.
- `return_zero_stub`: Clears the d0 register to zero and restores several registers from the stack before returning.
- `copy_icon_data_to_stack`: The library code copies 43 bytes from a structure pointed to by the third argument into a local stack buffer, then copies several fields from the same structure into local stack slots and calls another routine with the first two arguments and a pointer to a two-word stack header.
- `initialize_icon_resource_structure`: Allocates memory and populates an icon resource structure by calling several routines to load data, handle bitmask flags, and manage pointer arrays for image or metadata components.
- `populate_icon_resource_data`: Checks if a resource structure matches a specific type and version, then iteratively calls other routines to allocate and assign memory addresses or data pointers to various offsets within the structure and its associated buffers.
- `execute_icon_callbacks`: Library code calls another routine twice to update two longwords in a structure, then performs a series of conditional checks and calls to other routines using values from a structure and the stack.
- `process_library_callbacks`: As library code, it loops through a list of items, calling a function for each, and then performs a second set of conditional calls to a function and another routine based on the contents of a structure at offset 70.
- `clear_d6_and_branch`: Clears the d6 register and branches to another location.
- `return_void`: This library routine restores registers a2 through a5 and d2 through d6 from the stack and returns.
- `traverse_icon_hierarchy`: Recursively traverses a hierarchical icon structure by calling functions on child pointers found at various offsets within the structure.
- `process_icon_hierarchy`: Iterates through a hierarchical icon structure by recursively calling a function on child pointers found at offsets 50, 54, and 70 of the icon structure.
- `process_icon_list_recursively_2`: Library code that iterates through a list of pointers, calling functions and updating a local stack variable, then executes a callback via a pointer.
- `iterate_and_call_function`: Increments a pointer in a loop until it reaches zero, then calls a function via a pointer and performs further conditional calls and pointer updates.
- `process_icon_list_recursively`: Library code that iterates through a list of pointers, calling functions and updating a local stack variable, then executes a callback via a pointer or table.
- `execute_conditional_library_calls`: Saves registers, retrieves two values from the stack, and calls two different routines conditionally based on the result of the first call.
- `restore_registers_only`: This library routine restores registers d2 and d3 from the stack and returns.
- `incremental_offset_recurse`: As library code, it calls another routine with the same first argument and the second argument increased by 140, then returns.
- `allocate_icon_resource_3`: As library code, it calls another routine with a pointer to a value and a constant, then if the result is non-zero, it calls a routine with a pointer to a structure at offset 142, calls another routine with a pointer to a structure at offset 140 and the result of the first call, and if that result is non-zero, calls a third routine with a pointer to the structure and a constant.
- `call_and_conditionally_return_pointer`: Calls another routine with a pointer from a structure in a2 plus an offset of 140, then if the result is zero it restores the stack and returns; otherwise it moves the pointer from a2 into d0 and returns.
- `return_zero_minimal`: This library routine returns zero and restores registers a2 and d2 through d4 from the stack.
- `decrement_and_store_resource`: As library code, it decrements a 16-bit value through a pointer, and if the result is zero or less, it calls another routine; otherwise, it stores two 32-bit values into a table at an offset determined by the pointer and a base address.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, computes an address from a table base and the counter, and stores two 32-bit values from registers and the stack into consecutive slots of that table.
- `allocate_icon_resource_4`: As library code, it calls a routine to allocate 65536 bytes, then conditionally calls several other routines to initialize data and manage structures based on the success of those calls, eventually returning zero if certain conditions are met.
- `move_d6_to_d0_and_branch`: Moves the contents of d6 to d0 and branches to another location.
- `call_lib_func_and_return_zero`: Invokes a function pointer retrieved from a global variable associated with library index -210.
- `incremental_offset_recurse_short`: As library code, it calls another routine with the same first argument and the second argument increased by 78, then returns.
- `compare_memory_and_set_d0_2`: As library code, it decrements a value from the stack and compares a byte from a table to a byte from another memory location; if they match, it sets d0.
- `restore_registers_and_return`: Clears d0 and restores d2, d3, a2, and a3 from the stack before returning.
- `process_resource_data`: As library code, it calls another routine with a pointer from the stack, then if a value from the stack is non-zero, it calls another routine with a pointer to a constant and that value, and if the result is non-zero, it subtracts the value from the result and branches.
- `update_d2_from_d4_2`: As library code, it checks if d4 is zero; if not, it increments d4, moves the result to d2, and branches to another location, otherwise it returns.
- `update_d2_from_d4`: As library code, it checks if d4 is zero; if not, it increments d4, moves the result to d2, and branches to another location, otherwise it branches elsewhere.
- `clear_d2_and_branch`: Clears the d2 register and branches to another location.
- `restore_registers_and_return_2`: Clears d0 and restores d2, d3, d4, d5, d6, and a3 from the stack before returning.
- `process_icon_data_fields`: Library code clears a byte at an offset determined by adding 30 to a pointer, calls three different routines using values from the stack, and branches based on the result of the first call.
- `branch_on_d0_result`: Library code adds five to a register, pushes two values onto the stack, calls another routine, and then branches based on the value in d0.
- `clear_d5_and_branch`: Clears register d5 and branches to another location.
- `compute_string_checksum_2`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern.
- `compare_and_clamp_value`: Compares a value from the stack with 48 and 57, returning 48, 57, or the original value based on the comparison.
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
- `allocate_icon_resource`: As library code, it calls a routine with a pointer to 65536, and if the result is non-zero, it writes 10 to specific offsets and calls another routine.
- `return_zero`: As library code, it sets d0 to 0, restores d2, d3, and a2 from the stack, and returns.
- `verify_library_result`: As library code, it calls a routine using parameters from the stack and returns the result in d0 if it matches a value from the stack, otherwise it returns zero.
- `return_d1_in_d0`: As library code, it moves the value from d1 to d0, restores d2 and d3 from the stack, and returns.
- `call_and_validate_function`: Pushes three arguments onto the stack and calls a function via a pointer, then compares the returned value against a fourth argument; if they do not match, the routine returns zero, otherwise it returns the result of the call.
- `transfer_d1_to_d0_and_restore_registers`: Moves the contents of register d1 into register d0 and restores registers d2 and d3 from the stack before returning.
- `perform_division_math`: If d2 is not greater than 65535, it performs a series of divisions and swaps using d1 and d0.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_return`: Stores the byte from d0 into the address pointed to by a3 and increments a3 before returning.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `compare_byte_and_decrement_pointer`: As library code, it compares a byte from a structure to a value from the stack; if they match and the byte is zero, it decrements the pointer and returns, otherwise it branches elsewhere.
- `clear_d0_and_branch`: Clears the d0 register and branches to another location.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `check_string_byte`: Checks if the byte at the pointer in a0 is non-zero, and if so, decrements a0 and branches back to the start of the routine.
- `compare_memory_and_set_d0`: As library code, it decrements a value from the stack and compares a byte from a table to a byte from another memory location; if they match and the byte is zero, it sets d0 to zero and returns, otherwise it branches elsewhere.
- `set_d0_by_comparison`: Sets d0 to 1 if it is greater than 0, otherwise sets d0 to 255 and returns.
- `test_byte_and_decrement_d0`: As library code, it tests a byte at the address in a0; if the byte is not zero, it decrements d0 and branches to another location, otherwise it branches elsewhere.
- `decrement_a0_and_call`: Decrements the a0 pointer by 1 and calls another routine.
- `copy_bytes_with_counter`: Copies bytes from a1 to a0 while decrementing a counter in d0, and clears a byte at a0 if the counter reaches zero.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `conditional_library_branching`: The library code checks a parameter and, if non-zero, calls another routine and potentially calls a third routine using a value from a pointer offset.
- `iterate_icon_structure`: As library code, it calls a function via a pointer, and if the result is non-zero, it retrieves a byte from a structure, scales it, and calls another function via a pointer using the scaled value and other parameters from the structure.
- `branch_with_constant`: As library code, it sets d0 to 1 and branches to another location.
- `return_zero_2`: This library routine returns zero and restores registers a2 and d2 through d5 from the stack.
- `allocate_icon_resource_2`: Library code that performs a series of conditional calls to allocate memory and resources, eventually storing a result at offset 10 of a structure pointed to by a global variable.
- `return_negative_one`: This library routine returns negative one and restores registers a2 and d2 through d6 from the stack.
- `process_icon_data`: This library code calls another routine using three values from the stack, stores the result at offset 64 of the stack, and then either branches to another routine or moves a value from a stack offset into a register.
- `update_icon_attribute`: Library code that stores a value at offset 12 of a structure, performs calculations based on a byte at offset 7 of another structure, and calls other routines to update specific offsets in a structure.
- `branch_with_register`: As library code, it moves the contents of a3 into d0 and branches to another location.
- `return_negative_one_extended`: This library routine returns negative one and restores registers a2, a3, and d2 through d7 from the stack.

### Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL][a](Disk 1 of 8).adf:Crystal

- `init_and_check_pattern`: Invokes several routines and loops three times, then clears a memory location and, when a value matches a specific pattern, stores it and zeroes a global variable.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.

### Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL][a](Disk 1 of 8).adf:Character-Generation

- `initialize_game_state`: Executes a sequence of function calls and checks a global pointer to swap a memory value if a specific constant is found.
- `decrement_until_minus_one_3`: Decrements a local variable in d7 using a loop until it reaches negative one and then returns.

### Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL][a](Disk 1 of 8).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
