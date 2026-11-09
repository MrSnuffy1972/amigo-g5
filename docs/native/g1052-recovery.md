# Game 1052 (colonization (1995)(microprose)(m3)(disk 1 of 3)[h pgs]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01052). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 82 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 20 of 20 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 285 of 285 named (0 author symbols), 285 of 285 described (71 by rule, 172 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 210 of 210 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 9068 bytes, 18 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5710 of 5867 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 27462 |
| code | 14284 |
| data | 695470 |
| padding | 17472 |
| unused | 146432 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| More | 23 | 45 | 2 |
| SetPatchAGA | 6 | 45 | 2 |
| Else | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| FastMemFirst | 7 | 207 | 6 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| LoadWB | 12 | 538 | 30 |
| Version | 11 | 574 | 18 |
| SetPatchECS | 28 | 45 | 2 |
| Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:PGSINTRO | 6 | 45 | 2 |
| Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:Colonization | 102 | 253 | 7 |
| 9 | 1 | 2 | 1 |
| 6 | 1 | 2 | 1 |
| 8 | 1 | 2 | 1 |
| Icon.library | 10 | 1375 | 109 |
| Diskfont.library | 15 | 1040 | 76 |
| Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:bootblock | 1 | 18 | 1 |
| Snd.bin#unpacked (raw code file, load address unknown) | 1 | 1206 | 18 |

## Cases handled

- colonization (1995)(microprose)(m3)(disk 1 of 3)[h pgs]/Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:Exe/ToolIcons2.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (343 -> 2160 bytes)
- colonization (1995)(microprose)(m3)(disk 1 of 3)[h pgs]/Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:Exe/Customiz.pik: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (23220 -> 40384 bytes)
- colonization (1995)(microprose)(m3)(disk 1 of 3)[h pgs]/Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:Exe/Background.pik: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (47212 -> 51200 bytes)
- colonization (1995)(microprose)(m3)(disk 1 of 3)[h pgs]/Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:Exe/Terrain.ss: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2738 -> 10602 bytes)
- colonization (1995)(microprose)(m3)(disk 1 of 3)[h pgs]/Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:Exe/Cloud.ss: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2369 -> 4366 bytes)
- colonization (1995)(microprose)(m3)(disk 1 of 3)[h pgs]/Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:Exe/Phys0.ss: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (12994 -> 25130 bytes)
- colonization (1995)(microprose)(m3)(disk 1 of 3)[h pgs]/Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:Exe/ToolIcons.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (337 -> 2160 bytes)
- colonization (1995)(microprose)(m3)(disk 1 of 3)[h pgs]/Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:Exe/Snd.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5015 -> 9068 bytes)
- colonization (1995)(microprose)(m3)(disk 1 of 3)[h pgs]/Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:Exe/WoodTile.ss: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (355 -> 518 bytes)
- colonization (1995)(microprose)(m3)(disk 1 of 3)[h pgs]/Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:Exe/Col4.mod: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (31977 -> 38426 bytes)
- colonization (1995)(microprose)(m3)(disk 1 of 3)[h pgs]/Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:Exe/Icons.ss: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9287 -> 22514 bytes)
- colonization (1995)(microprose)(m3)(disk 1 of 3)[h pgs]/Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:Exe/OpenMenu.pik: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8927 -> 40000 bytes)
- colonization (1995)(microprose)(m3)(disk 1 of 3)[h pgs]/Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:Exe/Snd.bin#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- colonization (1995)(microprose)(m3)(disk 1 of 3)[h pgs]/Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:C/More: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- colonization (1995)(microprose)(m3)(disk 1 of 3)[h pgs]/Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:C/SetPatchAGA: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:PGSINTRO

## Copy protection found

- none found

## Functions


### More

- `initialize_more_data`: Performs a loop of function calls and checks a value in a structure to potentially update a global variable with a specific bitmask.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### SetPatchAGA

- `initialize_aga_patch`: Executes a loop of function calls and checks a structure value against a specific bitmask to update a global variable.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.

### Else

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of a long register to another long register and then branches to a different address.

### EndCLI

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value stored in a field and, if a condition is met, writes a header and a sequence of incrementing words into a buffer, then copies a fixed number of words from a temporary area to a global buffer, installs a function pointer, and returns success or failure.
- `copy_register_and_branch`: Copies the contents of register d1 to register d0 and then branches to another routine.

### Echo

- `find_max_and_prepare_buffer_3`: Walks a linked list of entries to find the maximum value stored in a field and, if a condition is met, writes a header and a sequence of words into a buffer, copies a block of sixteen words from a temporary area to a global buffer, installs a function pointer, and returns success or failure depending on the conditions checked during the loop.
- `copy_register_to_register`: Copies a longword from register d1 to register d0 and branches to another routine.

### FastMemFirst

- `initialize_fast_memory_and_parse_args`: Locates a fast memory block and parses the command line string into a linked list of null-terminated argument strings, handling quoted strings and converting certain characters like 'n' or 'e' to newline or escape characters, then calls another routine to finalize setup.
- `find_and_copy_fastmem_block`: The routine walks a linked list of memory blocks starting from a global variable, selects the first fast-memory block with a specific type tag and the first chip-memory block with another type tag, and if both are found and different it calls another routine on the fast block and then calls another routine with the list head, the fast block and the or chip block.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `find_max_and_prepare_buffer_4`: Walks a linked list of entries to find the maximum value stored in a field and, if a condition is met, writes a header and a sequence of words into a buffer, copies a temporary area to a global buffer, and installs a function pointer and a stack frame for a subsequent call.
- `copy_register_to_register_2`: Copies the contents of one 32-bit register to another and branches to a different routine.

### LoadWB

- `parse_string_and_update_structure`: Parses a string or a pointer to a string, handling quoted strings and escape sequences, and copies the resulting characters into a destination buffer while updating a structure with pointers to the processed data and calling other routines to manage memory or string-related tasks.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `initialize_workbench_data`: Performs complex data initialization involving string comparisons, linked list construction, and multiple library function calls to manage system resources and memory.
- `compare_null_terminated_strings`: Compares two null-terminated strings byte by byte and returns minus one when the first is lower, one when it is higher, and zero when they match.
- `build_linked_list_from_table`: Walks a table of pointer and value pairs, allocating a fixed-size node for each entry through a memory allocation call and storing the looked-up value in it.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `compare_memory_blocks_2`: As part of library code, clears d0, compares a pointer in a2 against a null pointer, saves registers, and if the pointer is valid, checks a byte at an offset in a2, then loops to compare memory or copies a pointer from a2 to a0 and compares memory blocks.
- `set_buffer_values`: Writes the value 46 to a byte at offset 17 of the structure in a2, then writes the value 32 to bytes at offsets 18, 19, 20, and 21 of that same structure, then branches to another routine.
- `compare_memory_blocks`: Copies a pointer from a2 to a0, loads a pointer to a certain offset relative to the program counter into a1, and then loops to compare words between the memory at a0 and a1, terminating the loop if a mismatch is found or if a zero-byte is encountered, otherwise it branches back to the start of the loop or to another routine.
- `load_stack_and_return`: Loads a pointer from a relative offset into a register, restores two registers from the stack, and returns.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_other_routine_with_stack`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot, then returns.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Version

- `parse_string_and_allocate_message`: Allocates a message structure and parses a string or a pointer to a string, handling quoted strings and certain escape-like character replacements, then calls another routine to process the resulting string.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `check_library_version`: Parses command-line arguments to open a specified library or device and validates its version and revision numbers against provided thresholds.
- `process_message_and_finalize`: Calls a routine to handle message processing by updating message fields and calling various functions based on flags, then calls a library vector to finalize the operation.
- `parse_integer_string`: Parses a string to convert it into an integer, handling optional sign characters and digit sequences.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
- `calculate_string_length`: Calculates the length of a null-terminated string by iterating until the null byte is found.
- `call_reschedule_2`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch_2`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `checksum_and_dispatch_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found; if the inverted checksum is non-zero, it calls another routine with the buffer and the inverted checksum.
- `store_byte_and_return_2`: Stores a byte from register d0 to the memory location pointed to by a6 and then returns.

### SetPatchECS

- `initialize_ecs_patch`: Executes a loop of function calls and checks a structure value against a different bitmask to update a global-variable.
- `decrement_register_until_negative_one`: Decrements a 16-bit register value until it reaches negative one and then returns.

### Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:PGSINTRO

- `initialize_exec_library_and_check_memory`: Calls a function pointer a certain number of times and then calls a routine via a pointer found in the Exec library structure.
- `decrement_until_minus_one_alt`: Decrements a local variable until it or a register reaches negative one and then returns.

### Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:Colonization

- `initialize_patch_large`: Executes a loop of function calls and checks a structure value against a bitmask to update a global variable.
- `decrement_register_until_negative_one_2`: Decrements a 16-bit value in d7 until it reaches negative one and then returns.
- `expand_and_accumulate_data`: Expands a data buffer and, if a certain condition is met, loops to add values from a source buffer to a destination buffer using a pointer.
- `load_constant_to_d0`: Loads a constant into d0 and loads a relative address into a1.
- `copy_and_expand_data_buffer`: Copies a block of bytes from a source buffer to a destination buffer while performing a bit-shifting or expansion-like operation on the data, and then potentially copies additional bytes from a secondary source or table-driven lookup if certain conditions or loop counters are met.
- `set_registers_and_branch`: Sets bits in d6 and d0, pushes d0, d1, and d2 onto the stack, restores registers from the stack, and branches to another location.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### 9

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 100 into d0, then returns.

### 6

- `set_d0_to_constant`: Loads a constant into d0 and returns immediately.

### 8

- `set_return_value`: Loads the constant one hundred into d0 and returns.

### Icon.library

- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_library_refcount`: If a certain bit in a structure is not set, it swaps two pointers in a linked list and calls a library routine with a new address, otherwise it sets bit 3 of a byte at offset 14 of the structure in a6 and returns.
- `set_bit_in_structure`: This library routine sets bit 3 of a byte at an offset of 14 from the address in a6 and returns with d0 cleared.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `update_library_data`: This library routine copies a pointer from a0 and a library base from a6 into a structure, calls a function via a pointer in a6, and if the return value is zero, calls another function via a global pointer.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `clear_d0_register`: Performs a logical or operation on d0 with zero.
- `clear_d0_byte`: Clears the lower byte of register d0.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `manage_icon_chain_3`: This library routine pushes a0 and a6 onto the stack, calls another routine, and then cleans up the stack before returning.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `manage_icon_chain_4`: This library routine calls another routine to manage an icon chain, saving a6 and a0 to the stack before the call.
- `save_and_call_recursive_routine`: Saves any existing register or pointer values to the stack and calls a routine that performs incremental recursion, then restores the stack pointer and returns.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_push_string_pointer`: Saves registers a0 and a1 to the stack, calls push_string_pointer, restores the saved registers, and returns.
- `call_string_validator`: Saves registers a0 and a1 to the stack, calls a routine to validate a string, and then returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `clear_data_register_d0`: Clears the least significant byte of data register d0.
- `update_icon_structure`: This library code calls another routine, and if the return value is non-zero, calls another routine with a pointer and a constant, and if that return value is non-zero, copies a block of data from a stack-allocated area to a destination area, then writes several values to a structure and potentially updates a pointer in a structure before returning.
- `copy_and_update_structure`: Library code that copies 11 words from a source area to a destination area using a loop, then writes several 32-bit or 8-bit values from a source area to a destination area, and finally updates a pointer in a structure if a certain condition is met.
- `return_zero_stub`: The routine is an empty stub that clears a register and restores any previously saved registers before returning a constant zero.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `update_icon_resource_pointers_2`: Checks if a specific icon structure or resource type is being accessed, then recursively updates or populates a list of pointers or resource.
- `update_icon_resource_pointers`: Checks if a specific icon structure or resource type is being accessed, then recursively updates or populates a list of pointers or resource handles within that structure.
- `propagate_icon_change_4`: As part of library code, saves several registers to the stack, calls another routine, and if the return value is not negative, updates a pointer in a4 and repeats the process or calls another routine to update icon structure data.
- `dispatch_icon_event`: Library code that calls a routine, and if the return value is non-zero, clears a stack slot, calls another routine, and if that returns a non-zero value, updates a pointer in a structure and calls another routine, repeating this in a loop until a certain count is reached.
- `set_d4_to_zero`: Sets register d4 to zero and then branches to another instruction.
- `move_d4_to_d0_and_restore_registers`: Moves the contents of d4 into d0 and restores d2 through d5 and a2 through a5 from the stack before returning.
- `update_icon_data_and_attributes`: Checks a library routine with a constant, and if a certain condition is met, clears a pointer at offset 26 of a structure and then calls a routine to update or traverse a list of pointers or sub-structures, potentially updating a counter or a global variable, before restoring a previously saved value to offset 26 of the structure.
- `validate_and_update_icon_attributes`: Checks certain flags and pointers within an icon structure, potentially clearing a pointer at offset 26, and then iterates through a table of pointers to call a function for each entry while updating a counter.
- `iterate_icon_table`: As part of library code, saves registers, calls another routine, and if the return value is not zero, checks a pointer in a2, and if certain conditions are met, calls another routine or updates a counter and calls a callback.
- `iterate_icon_table_entries_2`: In library code, increments a pointer through a null-terminated table, increments a value at a certain offset in a stack-based structure, calls a routine, and then iterates through a list of pointers, calling that routine for each entry and updating a value at a certain offset in the stack-based structure.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `traverse_linked_list_nodes`: In library code, pushes a register to the stack, loads a pointer from an offset, increments that pointer, and branches to another routine.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `update_icon_property_3`: Library code that calls another routine with a pointer and a constant, then if the return value is non-zero, calls another routine with a pointer and a calculated offset, and if a certain condition is met, calls another routine with a pointer, a calculated offset, and a constant.
- `update_icon_property_2`: As part of library code, pushes a pointer onto the stack, moves a pointer from a2 to the stack, pushes d2 onto the stack, calls another routine, and if d0 is not zero, moves a2 into d0 and branches to another routine.
- `clear_d0_register_2`: Clears the d0 register and restores d2, d3, and a2 from the stack before returning immediately.
- `update_icon_property_counter`: Library code that decrements a short counter through a pointer and, if the result is not positive, calls another routine and writes a constant to an offset of 14 in a structure.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `validate_and_update_icon_resource`: Validates a resource pointer and, if successful, checks a secondary pointer or resource type, then conditionally updates a list of pointers or resource properties using a sequence of conditional library calls.
- `branch_to_target`: In library code, performs a short branch to another location.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `perform_incremental_recursion`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `compare_string_suffix_2`: Library code that restores registers from a pointer and a value, calls another routine, and then branches to another location.
- `compare_string_and_suffix`: Library code that saves registers, calls another routine, and then loops to compare bytes from a buffer and check for a specific character.
- `compare_string_suffix`: As part of library code, moves a pointer from a2 into a3, moves a3 into d0, and if d0 is not zero, restores registers from the stack and returns.
- `push_string_pointer`: Library code that loads two pointers from the stack, pushes a third pointer onto the stack, calls another routine, and then restores the stack pointer and moves the return value into a register.
- `verify_string_library_call`: As parts of library code, pushes a pointer onto the stack, pushes d2 onto the stack, calls a library routine, and if d0 is not zero, calculates a new d0 by subtracting d2 from d0 before branching to another routine.
- `verify_string_and_library_call`: Library code that saves registers, calls a library routine, and loops to check if a return value matches a certain value, potentially calling another library routine and updating a register.
- `increment_d2_if_not_zero`: Checks if d4 is zero, otherwise copies d4 to d2 and increments d2, then branches to another instruction.
- `verify_string_library_call_2`: Library code that checks if a register is zero, and if so, clears a register and restores registers from the stack before returning.
- `copy_and_validate_string`: Library code that clears a byte at an offset in a buffer, calls a routine to check a string, and if a certain condition is met, calls further routines to check a second string and then calls a routine to copy a range of bytes or clear a buffer or otherwise branches to another routine.
- `process_string_and_table_lookup_2`: Library code that pushes two pointers onto the stack, calls another routine, and if the return value is non-zero, clears a register and branches to another location.
- `clear_d3_and_branch`: Clears register d3 and branches to a location within the calling routine.
- `process_string_and_table_lookup`: Library code that calculates an offset, loops to check a byte from a pointer, and if certain conditions are met, calls several routines to perform table-based or string-based operations.
- `compare_and_set_d0`: Loads a value from the stack, compares it against a constant, and sets d0 to either a specific constant or a calculated value based on the input, then branches.
- `return_negative_one`: This library code sets register d0 to negative one and returns.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `update_icon_property`: Library code that calls another routine with a pointer, and if the return value is non-zero, writes a constant to an offset of 14 in a structure pointed to by a0 and writes a constant to a pointer in a2, then calls another routine before returning.
- `restore_registers_and_return`: This library routine clears d0, restores a2 and d2 from the stack, and returns.
- `execute_library_call`: This library routine loads values from the stack, pushes them onto the stack, calls a routine, and then checks a value on the stack to either branch or clear d0 and branch elsewhere.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `transfer_stack_values`: Loads two 32-bit values from the stack, pushes them onto the stack, pushes a third value, calls a library routine, and then sets d0 to zero or branches to another instruction.
- `copy_d1_to_d0`: In library code, copies a 32-bit value from d1 to d0 and returns.
- `calculate_scaled_division`: This library code checks if d2 is greater than 65535, and if not, performs a series of operations involving swapping or clearing register d1, dividing d2 by d1, and or dividing d2 by d0, before returning.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_return`: This library code stores the byte from d0 into the address pointed to by a3 and then increments a3 before returning.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `branch_to_offset_if_match`: Loads a pointer and a value from the stack, checks if the byte at the pointer matches a certain value, and if so, decrements the pointer and returns it in d0.
- `branch_to_offset`: This library code clears register d0 and then branches to an offset of 36 bytes.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `scan_string_and_decrement`: This library code restores registers a0 and a1 from the stack, checks if the byte pointed to by a0 is non-zero, and if so, decrements a0 and loops back to a previous instruction in this routine.
- `compare_string_bytes`: Library code that saves registers, decrements a value from the stack, and if it is non-zero, compares bytes from two memory buffers, returning zero if a certain byte is zero or if the buffers match.
- `set_d0_based_on_sign`: Sets d0 to one or negative one depending on whether a certain condition is met, then returns.
- `check_string_null`: Library code that saves registers, checks if the byte at the pointer in a0 is null, and if not, decrements a counter and checks if a certain value is greater than zero.
- `decrement_string_pointer`: Library code decrements the pointer in a0 by one, calls a routine that copies a string with a length limit, and then returns.
- `copy_string_with_limit`: This library code decrements d0 and, if d0 is not negative, copies a byte from the address in a1 to the address in a0 and increments a0 and a1, looping if d0 is not zero; it then decrements d0 again and, if d0 is not negative, clears a byte at the address in a0 and branches to a later instruction in this routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `load_icon_list_pointers`: Library code that saves registers and loads three values from the stack into d2, d3, and a2 before branching to a new location.
- `traverse_icon_list`: Library code that calls another routine, and if the return value is non-zero, extracts a byte from a structure, sign-extends it, and calls another routine with a pointer-based offset, repeating this in a loop until a pointer in a2 becomes null or a certain condition is met.
- `return_zero_with_restore`: In library code, sets d0 to zero, restores registers d2, d3, and a2 from the stack, and returns.
- `propagate_icon_change_2`: Library code that saves registers, checks a pointer, and if non-zero, calls a library routine with several parameters, then performs further library calls to update a structure, and finally returns a value or negative one.
- `return_negative_one_with_restore`: In library code, sets d0 to negative one, restores registers d2, d3, d4, d5, and a2 from the stack, and returns.
- `load_icon_structure_data`: Library code that saves registers and loads three values from the stack into d2, d3, and d4 before branching to a new location.
- `execute_library_call_2`: Pushes a constant onto the stack, pushes two registers, calls a library routine, and then checks a value in a structure or register before branching.
- `propagate_icon_change_3`: Library code that calls a routine, and if the return value is non-zero, extracts a byte from a structure, sign-extends it, and calls another routine with a pointer, then calls another routine again, potentially updating a pointer in a structure and repeating this in a loop.
- `return_negative_one_with_restore_extended`: In library code, sets d0 to negative one, restores registers d2, d3, d4, d5, d6, and a2 from thestack, and returns.

### Diskfont.library

- `branch_to_address`: This library code branches to a relative offset of 80 bytes.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `manage_font_structure`: Library code that updates pointers in a structure, calls a routine, and if the return value is non-zero, calls another routine, and if that also returns non-zero, modifies a pointer in a structure, clears a value, and sets a new pointer before returning.
- `free_font_memory`: Library code calls a routine to free a memory block, calculates a new pointer or size using a value from a structure in a6, and then calls another routine to free a second memory block.
- `free_font_resources_2`: This library code iterates through a linked list, and if a certain condition is met, it removes an item from the list, calls a library function, and potentially removes another item from the list before decrementing a counter in a structure.
- `free_font_resources`: This library code checks certain values in a structure, calls library functions to free memory and close a library, and then updates a linked list or pointer chain with a value from the stack before returning.
- `set_bit_in_structure_2`: This library code sets bit 7 of a byte at an offset of 14 from the address in a6 and then clears register d0.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `branch_on_bit_test`: This library code decrements a word at an offset of 38 from a6, and if not zero, tests bit 7 of a byte at an offset of 14 from a6, and if that bit is set, jumps to an address relative to a6.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `update_font_table_from_string_2_2`: Iterates through a linked list of string-based font definitions, copying the string bytes into a destination buffer and updating a font table entry with a type, a pointer, and a size, or otherwise returns immediately.
- `prepare_string_pointer`: Library code that loads a pointer from an offset of a4 into a1 and sets d4 to one, then branches to another instruction.
- `count_null_terminated_string`: Counts the number of bytes in a null-terminated string and, if the length plus ten does not exceed a certain limit, copies the string or a portion of it to a destination buffer, while updating a structure or table with the new length and metadata.
- `update_font_table_from_string_2`: Iterates through a string, copying bytes to a destination buffer, and updates a font table entry with a type, a pointer, and a size, while incrementing a global counter.
- `parse_font_string_and_update_table`: Searches for a string pointer, and if found, calculates its length and copies the string or its characters into a destination buffer while updating a tracking structure or table.
- `count_null_terminated_string_2`: Counts the number of bytes in a null-terminated string and checks if the string length or a calculated offset-based value falls within a certain range relative to other registers, otherwise it returns.
- `copy_font_buffer_and_verify_read`: Copies a sequence of bytes from a source buffer to a destination buffer, then calls a routine to read from a disk or file, checking if the number of bytes read matches a specific value and if a certain word in a structure equals a specific constant before proceeding.
- `update_font_table_from_string`: Checks if a string or buffer certain length, then updates a font table entry by writing a type, a pointer, and a size, while incrementing a counter in a global variable.
- `return_zero_stub_2`: Clears the d0 register and restores several registers from the stack before returning.
- `calculate_font_offset`: Library code that calculates a value based on a pointer, compares bytes from two different buffers, and if certain bits match, calculates a value, then returns or branches to another routine.
- `update_font_index_logic`: Library code that calculates a value based on a pointer, compares bytes from two different buffers, and if certain bits match, calculates a new offset and sets a pointer to a global table.
- `update_font_index_and_table_offset`: In a library routine, shifts a value in d4 left by two, uses it as an offset to subtract a value from a table at a fixed address into d2, and loops while incrementing d4 and checking bitmask conditions.
- `updates_font_index_and_table_offset`: Shifts a value in d0 left by two, uses it as an offset to subtract a value from a table at a fixed address into d2, increments d4, and loops if d4 is less than seven.
- `return_constant_value`: This library routine returns a constant value without using any arguments or registers.
- `update_font_buffer_metadata`: Checks if a font buffer or character count has reached a certain limit, and if so, updates a structure or table with a new pointer, increments a counter, and sets a flag bit.
- `push_font_pointer`: This library code pushes a pointer onto the stack, calls another routine, and then loops if the return value is not certain, otherwise it returns.
- `reallocate_font_buffer_font_data`: Calculates a new memory allocation size for a font buffer, checks if the buffer or character count meets certain alignment or range criteria, and if so, performs a search or copy operation to relocate or update font-related metadata and pointers within a structure.
- `clear_registers_and_branch`: Clears d4 and d3, loads a constant into d2, and branches to another routine.
- `reallocate_font_buffer_3`: Iteratively calculates a required buffer size or checks if an existing buffer or font entry needs resizing, and if so, calls routines to deallocate and reallocate memory or update font metadata.
- `reallocate_font_buffer_2`: In a library routine, pushes d4 to the stack, calls a routine to unload a segment, calculates a new size by multiplying d2 by a constant, and clears a certain amount of memory at an offset relative to a new pointer.
- `reallocate_font_buffer`: Calculates a new buffer size based on a font or character count, clears a certain amount of memory at an offset, and uses a loop to find or reallocate a memory block that can accommodate the required size, potentially updating a structure or table with the new pointer and size.
- `build_disk_font_table`: Calculates the size of a font or character set by traversing a linked list, allocates memory for a new table, and populates it with pointer and size information or copies character data from an existing source if certain size or value conditions are met.
- `load_disk_font_table`: As library code, this routine builds a table for a disk font.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `call_forbid_2`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit_2`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem_2`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `load_pointer_and_set_d0`: As part of library code, this routine loads a pointer from a location offset by 4 from the stack into a0 and loads a value from a location offset by 8 from the stack into d0, then jumps to another routine.
- `skip_to_string_terminator_and_call`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `set_pointer_and_zero_d0`: As part of library code, this routine loads a pointer from a location offset by 4 from the stack into a0, clears d0, and jumps to another routine.
- `count_null_terminated_bytes`: As part of library code, this routine increments a counter while advancing a pointer and reading bytes until a zero byte is encountered, returning the final count.
- `compare_library_bytes`: As part of library code, moves a pointer from a stack offset into a1 and another pointer from a stack offset into a0, then jumps to another routine.
- `check_null_byte`: Tests if the byte pointed to by a0 is zero, and if not, jumps to a different location; otherwise, it sets d0 to zero and jumps to another routine.
- `check_null_byte_2`: In a library routine, moves a byte from a pointer in a0 to d0, compares a byte from a pointer in a1 to d0, and otherwise moves a byte from a pointer in a1 to d0 and checks if a byte at a pointer in a0 is greater or equal to d0.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `load_pointer_and_check_byte`: As part of library code, moves a pointer from a stack offset into a0, moves a byte from a stack offset into d0, and then branches to another routine.
- `compare_and_assign_pointer`: Compares two bytes in d0 and d1, and if they are not equal, jumps to a different location; otherwise, it copies the pointer in a0 to d0 and jumps to another routine.
- `compare_and_assign_pointer_2`: In a library routine, increments a pointer in a0, moves a byte from that pointer into d1, and if that byte is not zero, clears d0 and returns.
- `initialize_pointer_and_search`: As part of library code, saves registers to the stack, loads a pointer from a stack offset into d3, loads a byte from a stack offset into d2, pushes d3 onto the stack, calls another routine, moves d0 into a0, decrements a0, and then adds d3 to a0 before branching to another routine.
- `find_matching_byte`: Compares a byte from a pointer in a0 to a byte in d2, and if they are not equal, jumps to a different location; otherwise, it copies the pointer in a0 to d0 and jumps to another routine.
- `find_matching_byte_2`: In a library routine, decrements a pointer in a0, compares a pointer in a0 to d3, and if a pointer in a0 is not greater or equal, clears d0 and restores d2 and d3 from the stack before returning.
- `search_for_pointer`: Library code that pushes a pointer and a constant onto the stack, calls another routine, adjusts the stack, and repeats this process for a different constant before branching to another routine.
- `increment_returned_value`: This library routine returns the value passed in D0 incremented by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `call_other_routine_2`: This library code saves registers a0 and a1 to the stack, calls another routine, restores a0 and a1, and then returns.
- `call_another_routine`: This library routine is library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Colonization (1995)(MicroProse)(M3)(Disk 1 of 3)[h PGS].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Snd.bin#unpacked (raw code file, load address unknown)

- `branch_to_other_routines`: Performs a branch to a distant address, clears a register, and then performs two more branches to other addresses.
- `update_audio_dta_parameters`: Iterates through a table of audio parameters, updating the audio period and certain DMA or certain registers, and sets a bit in the CIAA_PRA register.
- `update_audio_parameter_structure`: Updates a structure or table entry by calculating a new distance or range, setting certain bits in a control byte, and updating hardware registers including a DMA or or audio-related register.
- `fill_zero_data_area`: This routine performs a series of or instructions that do not change the value of d0.
- `update_audio_dma`: Iterates through a table of audio parameters, updating the audio period, low-level or certain, and DMA controller registers, and calls a routine that executes a fixed number of instructions to modify the audio-related data or counters.
- `update_audio_parameters_3`: Sets a hardware register to a constant, calls a routine that copies a table, and then performs a series of operations that include clearing a certain memory location, updating a table with new pointers, and clearing several memory or table entries.
- `copy_table_and_set_global`: If a certain pointer is not null, it clears that pointer, copies 64 words from a source table to a destination table, and then checks a global variable to set it to a specific constant or a value that modifies a hardware register.
- `fill_zero_data_area_long`: This routine performs a series of or instructions that do not change the value of d0, then returns.
- `configure_hardware_interrupts`: Checks a pointer in a table, and if not null, updates the audio period register, modifies the interrupt enable bit in a hardware register, and calls other routines to update audio parameters or hardware registers using a table of values.
- `check_pointer_and_save`: This routine checks a word at a certain offset, and if it is not zero, calls another routine and saves a large block of registers to the stack before calling another routine and finally restores registers and returns.
- `update_audio_parameters_2_2_2`: Iterates through a table of audio parameter structures to decrement certain counters, update pointers, and modify audio registers or parameters.
- `update_audio_parameters_2_2`: Iterates through a table of audio parameter structures to decrement certain counters, update pointers, and modify audio registers or parameters based on table-driven logic, including conditional pointer updates and register masking.
- `update_audio_parameters_2`: Updates audio registers and parameters by iterating through a table, including logic to modify certain registers, check for null pointers, and branch or jump based on certain conditions or register values.
- `fill_zero_data_area_short`: This routine performs a series of or instructions that do not change thed0.
- `update_audio_parameters`: Checks a global variable and, if certain conditions are met, updates several audio registers or copies a block of data from a source structure to the audio registers.
- `modify_registers_and_memory`: This routine performs several or instructions, increments a pointer in a4, clears a bit in a memory location offset by 444 from a4, and writes a value to a memory location offset by 444 from a5.
- `modify_stack_and_bit_flags`: Increments a byte at the current stack pointer location, clears a bit in a structure member pointed to by a6, and performs a series of or operations on d0 before clearing a bit in a structure member pointed to by a4.
- `modify_register_and_memory_byte`: Sets the high bits of a byte at the address pointed to by a0 to certain values and modifies a bit in a global variable or structure member at an offset relative to a4, then writes a constant value to a memory location using a pointer and a register-based offset.
