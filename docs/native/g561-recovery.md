# Game 561 (Realms of Arkania - Blade of Destiny): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00561). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (7208960 top-level bytes in 43 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 16 of 16 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 173 of 173 named (0 author symbols), 173 of 173 described (37 by rule, 113 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 83 of 83 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 372722 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3557 of 3557 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 5589286 |
| code | 10600 |
| data | 1272179 |
| padding | 10239 |
| unused | 326656 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 1 of 8).adf:Blade-Main | 8 | 45 | 2 |
| AddBuffers | 2 | 117 | 2 |
| Assign | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| Copy | 2 | 117 | 2 |
| Makedir | 2 | 117 | 2 |
| LoadWB | 12 | 475 | 30 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 1 of 8).adf:Blade-Intro | 2 | 267 | 8 |
| Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 1 of 8).adf:HardDisk Install | 2 | 267 | 8 |
| icon.library | 15 | 1414 | 108 |
| Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 1 of 8).adf:Crystal | 5 | 45 | 2 |
| Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 1 of 8).adf:Character-Generation | 10 | 45 | 2 |
| Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 1 of 8).adf:bootblock | 1 | 18 | 1 |
| Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 2 of 8).adf:ROA-Intro.DAT (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl](disk 2 of 8)/Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 2 of 8).adf:ROA-Intro.DAT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl](disk 2 of 8)/Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 2 of 8).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl](disk 3 of 8)/Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 3 of 8).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl](disk 4 of 8)/Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 4 of 8).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl](disk 5 of 8)/Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 5 of 8).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl](disk 6 of 8)/Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 6 of 8).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl](disk 7 of 8)/Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 7 of 8).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl](disk 8 of 8)/Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 8 of 8).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl](disk 1 of 8)/Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 1 of 8).adf:Blade-Main: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl](disk 1 of 8)/Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 1 of 8).adf:Blade-Intro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl](disk 1 of 8)/Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 1 of 8).adf:HardDisk Install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl](disk 1 of 8)/Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 1 of 8).adf:Crystal: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- realms of arkania - blade of destiny (1992)(u. s. gold)[cr csl](disk 1 of 8)/Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 1 of 8).adf:Character-Generation: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 372722 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 1 of 8).adf:Blade-Intro
- Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 2 of 8).adf:ROA-Intro.DAT (raw code file, load address unknown)

## Copy protection found

- none found

## Functions


### Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 1 of 8).adf:Blade-Main

- `run_init_calls_and_swap_slots`: Saves the stack pointer at offset 60 on the stack, then repeatedly calls a routine with a counter and decrements that counter until it reaches three.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### AddBuffers

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_d1_to_d0_branch`: Copies d1 into d0 and branches to the routine at offset 260.

### Assign

- `find_max_and_prepare_buffer_2`: Walks a linked list of table entries to find the largest value in a field, then, if a check succeeds, writes a header with a running value into a buffer, sets up a stack frame, copies sixteen words between a temporary area and a global buffer, installs a function pointer, and returns success or failure depending on conditions checked during the loop.
- `move_d1_to_d0`: Copies the value in register d1 into register d0 and then branches into the middle of another routine.

### EndCLI

- `find_max_and_prepare_buffer_3`: Walks a linked list of table entries to find the largest value in a field, then stores that value and a sequence of incrementing words into a structure, sets up a stack frame, copies sixteen words between a temporary area and a global buffer, installs a function pointer, and returns success or failure depending on the conditions checked during the loop.
- `copy_d1_to_d0_and_branch`: Copies the 32-bit value in d1 into d0, then branches unconditionally to the routine at offset 104.

### Copy

- `find_max_and_prepare_buffer_4`: Walks a linked list of table entries to find the largest value in a field, then writes that value plus a sequence of incrementing words into a structure, sets up a stack frame with a function pointer, copies sixteen words between a temporary area and a global buffer, and returns success or failure depending on the conditions checked during the loop.
- `transfer_d1_to_d0`: Copies the value in register d1 into register d0 and then branches to the routine that swaps the game slots.

### Makedir

- `find_max_and_prepare_buffer_5`: Walks a linked list of directory entries to find the largest value in a field, then writes a small header and fills a buffer with sequential words, copies sixteen words between a temporary area and a global buffer, installs a function pointer, and returns success or failure depending on the conditions checked during the loop.
- `transfer_d1_to_d0_2`: Copies the value in register d1 into register d0 and then branches to the routine that swaps memory slots.

### LoadWB

- `parse_command_line_arguments`: Parses the command-line arguments into individual tokens, storing pointers to each token in a table and converting quoted strings and escape sequences, then calls the workbench-loading initialization routine.
- `restore_stack_and_call_routine`: Restores the stack pointer from the saved argument, sets the address register base from a global variable, then calls a routine via a pointer at a negative offset from the base and returns the original argument.
- `invoke_two_function_pointers`: Loads two function pointers from fixed negative offsets from a base register and calls each of them in turn, then returns.
- `call_dos_and_store_pointer`: Loads the address of a data table into a register, clears d0, and calls a dos.library routine via a6.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_dup_lock`: Library stub: loads its arguments and the dos library base, calls DupLock and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `initialize_workbench_loading`: Performs a series of initialization checks and calls, including XORing a global buffer with a constant and invoking various table-based or string-based routines to set up workbench loading.
- `compare_strings_byte_by_byte`: Compares two null-terminated strings byte by byte, returning a negative value if the first is lexicographically less than the second, a positive value if greater, and zero if they are equal.
- `build_linked_list_from_table`: Walks a table of pointers using the argument as an index, allocates a node via another routine, links the nodes into a chain, writes a value obtained from another routine into each node, and returns the head of the chain, stopping when the table entry is zero or allocation fails.
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
- `check_a2_and_save_registers`: Compares the 32-bit value in a2 with the constant -1 and, when they are equal, saves registers a0 and a1 on the stack, copies a2 into a0, and loads the address of a byte table into a1.
- `load_stack_and_return`: Loads a pointer from a relative offset into a register, restores two registers from the stack, and returns.
- `pass_params_to_subroutine`: Calls another routine with a global value, its parameter and the address of an eight-byte stack slot.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found, and if the inverted checksum is non-zero passes the buffer and checksum to another routine.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Disk-Validator

- `initialize_disk_validator`: Initializes a disk validator by storing a checksum byte and two digit characters into a result buffer, resets several counters and tables, then repeatedly calls a helper routine to process entries until a termination condition is met.

### Ram-Handler

- `forward_args_and_call_vector`: Copies d2 into a1 and d3 into d0, then calls the routine at offset -624 of the library base in a6 and jumps to the value in a6.

### Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 1 of 8).adf:Blade-Intro

- `process_hunk_table`: Parses a hunk table, matching the executable and relocation type codes and copying their payload data into a buffer, then calls the AmigaOS loader to relocate the code.
- `decrement_counter_and_return`: Decrements a 16-bit counter in d1 until it reaches zero, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `compare_table_bytes_and_jump`: Tests the byte at the table in a1 and, if it is zero, compares the byte in a0 with the byte in a1 before jumping back to the earlier part of the routine.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `rotate_loop_with_carry`: Performs a 32-bit rotate-left loop controlled by a shift count in d4 and a counter in d0, pulling carry bits from a stack slot when the shift exceeds 31.
- `store_word_and_trap`: Stores a word from the address in a2 at offset 3084 into d4, sets bit 3 of d3, and invokes trap 6.

### Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 1 of 8).adf:HardDisk Install

- `process_hunk_relocations`: Parses a sequence of AmigaDOS hunk records, copying the data of each executable and relocation hunk into newly allocated memory and fixing up the relocation entries, then stores the total number of hunks processed.
- `decrement_counter_and_call`: Decrements a word counter in d1 and returns when it reaches zero, otherwise continues processing a table entry and calls a library routine at offset -210.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `copy_bytes_backward_loop`: Reads a byte from the length pointer in a0, and if non-zero subtracts one and shifts it to compute a destination offset.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `subtract_d4_from_d0`: Subtracts the value in d4 from the value in d0 and returns.
- `rotate_carry_accumulate`: Performs a 32-bit rotate-left on a value in d5 using a shift count in d4, pulling carry bits from a stack slot, and accumulates the result into a 32-bit accumulator in d3 while decrementing a counter in d0.
- `read_timer_and_trigger_trap`: Reads a 16-bit value from offset 192 in the structure pointed to by a2 into d4, sets bits 0, 3, 5 and 7 of d0, triggers trap 6, then combines d0 and d1 with a bitwise OR.

### icon.library

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `insert_object_into_list`: Checks a flag in the structure in a6, and when clear, inserts the object into a doubly linked list and calls a library routine at offset -210.
- `set_flag_in_struct_and_return`: Sets bit 3 of the byte at offset 14 in the structure pointed to by a6 and returns zero.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `open_icon_and_store_result`: Stores a0 at offset 42 and a6 at offset 34 of the structure in a2, then calls a library function with a request code of 33.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `or_d0_with_zero`: Performs a bitwise OR of d0 with zero, which leaves the value unchanged.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `call_icon_entry_698`: Saves the a0 and a6 registers on the stack, then calls the icon.library entry point at offset 698 with the stack arguments and restores the stack before returning.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_entry_point`: Saves the a0 and a6 registers on the stack, calls the icon.library entry point at offset 1922, discards the two saved values, and returns.
- `call_icon_entry_820`: Saves the a0 and a6 registers on the stack, then calls the icon.library entry point at offset 820 with the stack arguments and restores the stack before returning.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_other_routine_2`: This library code saves two address registers on the stack, calls another routine, removes its four arguments from the stack, and returns.
- `call_other_routine_3`: Saves two address registers on the stack, calls the routine at offset eight hundred fourteen, restores the stack pointer, and returns without performing any other operations.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `store_zero_byte`: This routine consists of a single instruction that writes a zero byte to the memory location pointed to by the register in data0, then returns.
- `copy_and_init_icon`: Reads two pointers from its frame, calls a routine with the first pointer, and when the result is nonzero copies 44 bytes from a buffer pointed to by the second pointer into a structure, then stores several fields from its stack into the structure and returns the pointer.
- `copy_and_move_struct_fields`: Copies 44 bytes from a source buffer pointed to by a6 into a destination buffer pointed to by a2 using a decrementing counter, then transfers eight fields from the source structure to fixed offsets of the destination structure and returns.
- `return_zero_stub_2`: This library routine clears a 32-bit result register to zero, restores the caller's registers from the stack, unwinds the frame, and returns.
- `copy_icon_data_to_stack`: Copies forty-four bytes from offset ninety-six of the structure pointed to by the third argument into a local stack buffer using a decremented counter loop, then stores eight fields from that structure into local stack slots and calls another routine with the first two arguments and a pointer to a two-word stack header.
- `initialize_new_icon`: Initializes a new icon structure by clearing its ID and type fields, then repeatedly calls the icon creation routine to fill its input and output buffers.
- `create_new_icon`: Initializes a new icon structure by clearing its ID and type fields, then repeatedly calls the icon creation routine to fill its input and output message lists, returning zero on success or one if the icon could not be created.
- `process_icon_node_entry_2`: Loads three arguments from the stack, calls a routine, and stores its result into a structure field unless the result equals negative one, then branches based on several zero-checked fields while issuing further calls and storing their results.
- `update_icon_structures`: Calls two library routines in sequence and stores the second result into a structure field.
- `set_zero_and_branch`: Loads the value zero into a result register and then branches into the middle of another routine, so it only sets up a value before handing off to the following code.
- `restore_registers_and_return_3`: This library routine restores the caller's registers from the stack, unwinds the frame, and returns without touching any result register.
- `apply_icon_function_to_screen`: Fetches a screen from a global table and, if it exists, applies an icon library function to it, then restores the previous screen and returns.
- `process_icon_list_recursively_3`: Stores the next pointer from a structure into a stack slot, calls a library routine to process it, and if the result is non-zero and the flags at offset 17 indicate children, recursively processes the child list at offset 36 before returning the accumulated value.
- `process_icon_list_recursively_2`: Walks a null-terminated list of pointers stored in a structure, invoking a library function on each entry and updating a local counter, then performs cleanup calls and stores the final counter value.
- `advance_pointer_until_null`: Advances a pointer and its associated counter by four bytes in a loop until the pointer reaches a null entry, then invokes a library function on the first argument and branches to further processing.
- `process_icon_list_recursively`: This library routine walks a null-terminated list of pointers, invoking a library function on each entry and updating a local counter, then runs a callback through a pointer and stores a result into a structure before returning.
- `call_routine_on_adjusted_pointer`: Reads two arguments from the stack, increments the second pointer by two, and calls a routine on that adjusted value, repeating the call while the returned result is nonzero.
- `restore_registers_and_return_4`: This library routine restores the caller's registers from the stack and returns without setting any result value.
- `incremental_offset_recurse`: Reads two arguments from the stack, pushes the second one increased by a fixed offset, and calls a library routine on that adjusted value before returning.
- `lookup_resident_and_call_helpers`: Looks up a resident by name and, when the lookup succeeds, calls a helper with a pointer to the found structure and the original argument, then calls another helper with the same pointer and a different argument, returning the first result only when that second call is non-zero.
- `call_icon_entry_72e`: Pushes a constant and a structure offset onto the stack, calls the entry point at offset 72E, and returns zero if the result is empty otherwise returning the structure base.
- `return_zero`: Clears register d0 and returns without modifying any data.
- `decrement_and_store_pair`: Decrements a short counter through a pointer and, while it stays positive, calls a library routine at offset -256.
- `decrement_counter_and_store_pair`: This library routine decrements a short counter through a pointer, computes an address from a table base and the counter, and stores two 32-bit values from a register and the stack into consecutive slots of that table, then returns one.
- `lookup_and_process_resident`: Looks up a resident by name and, when the lookup succeeds, calls a routine on the found structure, then branches through several helper routines depending on their results, finally clearing the result register and returning.
- `copy_register_and_branch`: Copies the value from one result register into another and then branches into the middle of another routine, so it only relabels a value before handing off to the following code.
- `call_lib_func_and_return_zero`: This library routine pushes a value from a global variable, invokes a library function via a computed address, clears a 32-bit result register to zero, restores the caller's registers, and returns.
- `incremental_offset_recurse_2`: Reads two arguments from the stack, pushes the second one increased by a constant, and calls a library routine on that adjusted value before returning.
- `find_equals_in_string_list`: Reads two pointers from the stack, then walks a list of entries calling a comparison routine on each until it finds a byte equal to the equals sign and returns the address just past it.
- `clears_registers_d2_d3`: Clears the lower 16 bits of register d2 and the lower 16 bits of register d3, then returns.
- `compute_counter_difference`: Reads a pointer and a counter from the stack, compares the counter against zero, and when it is non-zero computes the difference between a value returned by a helper and the counter, returning that difference.
- `compare_string_list_entries`: Reads pointers from the stack, compares a computed value against a reference, and calls a comparison routine repeatedly, returning one when a match is found and zero otherwise.
- `increment_and_branch`: Tests a register and, when it is nonzero, increments it and branches to another routine; otherwise it branches away without modifying the value.
- `zero_register_and_branch`: Loads the value zero into register d2 and then branches to another routine.
- `restore_registers_and_return_5`: Restores the register contents saved on the stack and returns.
- `format_string_from_digits`: Clears a byte at offset thirty of the structure pointed to by the first argument, then calls a routine with the second argument and a constant; if that call is nonzero it performs three more calls and returns the first argument, otherwise it parses digits and returns the first argument.
- `push_constants_and_call`: Increments the pointer in a2 by five, pushes two constant addresses onto the stack, and calls a routine at a fixed library offset before returning through a branch.
- `clear_register_and_branch`: Loads zero into a data register and then branches into the middle of another routine without returning.
- `parse_decimal_and_format`: Converts a sequence of ASCII digits into a decimal number, then calls a formatted output routine with the result and a format string before restoring the saved registers.
- `clamp_value_to_decimal`: Clamps a value loaded from the stack into a decimal range by subtracting 48, wrapping negative results up to 9, and otherwise returning the value unchanged.
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
- `conditional_library_chain`: Reads four arguments from the stack, calls a library routine at offset 198, and if the result is non-zero calls a routine at offset 72E that may call a second routine at offset 18 before returning the value.
- `allocate_and_initialize_object`: Saves callee-saved registers, calls a library routine at offset -256, and when the result is nonzero stores a constant into the structure and calls another library routine.
- `restore_registers_and_return_zero`: Loads the saved registers d2, d3 and a2 from the stack, sets d0 to zero, and returns.
- `compare_result_and_maybe_zero`: Saves callee-saved registers, calls a library routine at offset -42, and returns zero when the result differs from a passed value.
- `copy_d1_to_d0_and_restore`: Copies the value from d1 into d0, restores the saved registers d2 and d3 from the stack, and returns.
- `compare_result_and_maybe_zero_48`: Saves callee-saved registers, calls a library routine at offset -48, and returns zero when the result differs from a passed value.
- `restore_registers_and_return`: Restores registers d2 and d3 from the stack and returns.
- `divide_32bit_values`: Performs 32-bit division of the value in d1 by the divisor in d2, returning the quotient in d1 and the remainder in d0, or rotating the value left sixteen times when the divisor exceeds 65535.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_through_pointer`: Stores the byte in d0 through the pointer in a3 and then returns to the caller.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `compare_byte_sequence`: Reads a pointer and a counter from the stack, then compares successive bytes from the pointed-to string against a reference byte, decrementing the counter until a match fails or the counter is exhausted.
- `load_zero_and_branch`: Loads the value zero into register d0 and then branches to a routine located at offset 24, which is part of the icon.library shared library.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `skip_first_byte_string`: Loads two destination and source pointers from the stack, skips a single byte when the source is non-zero, then decrements the pointer to point just before the string.
- `compare_string_pairs`: Reads two pointers and a counter from the stack, then compares corresponding bytes from each string while decrementing the counter, returning zero when a byte is zero and otherwise continuing until the counter is exhausted.
- `select_value_by_comparison`: Loads the value one into a result register, and if the preceding comparison indicates a positive condition it branches away, otherwise it overwrites the register with the value 255 and returns.
- `decrement_pointer_then_copy_string_2`: Reads two pointers and a counter from the stack, skips a leading byte, then decrements the counter while it stays positive, branching to another routine when it reaches zero.
- `decrement_pointer_then_copy_string`: Decrements a destination pointer by one and then calls a routine that copies a byte string, after which it returns.
- `copy_byte_string_loop`: Copies bytes from the source pointer in a1 to the destination pointer in a0, looping while the source counter in d0 is positive and writing a terminating zero byte at the end.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `conditional_library_branching`: The library code checks a parameter and, if non-zero, calls another routine and potentially calls a third routine using a value from a pointer offset.
- `process_icon_structures`: Reads three pointers from the stack, sets a register to one, and calls a library routine at offset -48.
- `set_one_and_branch`: Sets d0 to one and branches to the routine at offset 232.
- `return_zero_stub`: This library routine clears a 32-bit result register to zero, restores the caller's registers from the stack, and returns.
- `process_icon_node_chain`: Reads four arguments from the stack and calls a library routine at offset 198, then if the result is non-zero calls a routine at offset 72E that returns a 32-bit value.
- `return_neg_one_stub`: This library routine sets a 32-bit result register to all ones, restores the caller's registers from the stack, and returns.
- `process_icon_node_entry`: This library code loads three arguments from the stack, clears a register, and returns that register when the fourth argument is zero.
- `update_icon_node_entry`: Stores a pointer from the stack at offset 12 of the structure in a2, then calls a library routine at offset -42 and returns -1 if the result is zero.
- `copy_a3_to_d0_branch`: Copies a3 into d0 and branches to the routine at offset 670.
- `restore_registers_and_return_2`: Loads the value -1 into d0 and restores the callee-saved registers d2 through d7 and a2 and a3 from the stack, then returns.

### Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 1 of 8).adf:Crystal

- `init_and_check_pattern`: Saves registers, runs several initialization routines, loops three times invoking a handler from a global structure, then zeroes a location and swaps a memory slot when a specific constant pattern is found.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.

### Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 1 of 8).adf:Character-Generation

- `init_and_swap_slot`: Saves registers, runs several initialization routines, loops eight times invoking a handler from a global structure, then zeroes a location and swaps a memory slot when a specific constant pattern is found.
- `decrement_until_minus_one_3`: Copies the counter from d6 into d7, loads a pointer from a table, doubles it twice, then decrements the counter until it reaches negative one before returning.

### Realms of Arkania - Blade of Destiny (1992)(U. S. Gold)[cr CSL](Disk 1 of 8).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
