# Game 1425 (chaos strikes back (1991)(on-line)[cr betrayal](disk 1 of 2)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01425). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905056 top-level bytes in 30 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 24 of 24 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 231 of 231 named (0 author symbols), 231 of 231 described (61 by rule, 165 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2279 of 2279 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 597084 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5696 of 5696 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 92166 |
| code | 18342 |
| data | 689298 |
| padding | 7970 |
| unused | 97280 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:BETRAYAL! | 8 | 233 | 7 |
| Type | 2 | 117 | 2 |
| SetPatch | 24 | 833 | 45 |
| EndCLI | 2 | 117 | 2 |
| FastMemFirst | 7 | 207 | 6 |
| ENDIF | 1 | 2 | 1 |
| IF | 2 | 117 | 2 |
| Version | 11 | 574 | 18 |
| Disk-Validator | 1 | 386 | 1 |
| Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:BJELoad_R | 3 | 2063 | 125 |
| Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 508 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 526 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:APPB.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:SWSH.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:CNFG.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:GRF1.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:APPA.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:Graphics.DAT (raw code file, load address unknown) | 1 | 0 | 0 |
| Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:VDEO.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:ANIM.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:MEM1.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:USIO.FTL (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- chaos strikes back (1991)(on-line)[cr betrayal](disk 1 of 2)/Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:Dungeon.DAT: high-entropy data file (7.65 bits/byte), classified as packed data
- runtime segment an address: 464 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 352 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- chaos strikes back (1991)(on-line)[cr betrayal](disk 1 of 2)/Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:APPB.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- chaos strikes back (1991)(on-line)[cr betrayal](disk 1 of 2)/Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:SWSH.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- chaos strikes back (1991)(on-line)[cr betrayal](disk 1 of 2)/Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:CNFG.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- chaos strikes back (1991)(on-line)[cr betrayal](disk 1 of 2)/Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:GRF1.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- chaos strikes back (1991)(on-line)[cr betrayal](disk 1 of 2)/Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:APPA.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- chaos strikes back (1991)(on-line)[cr betrayal](disk 1 of 2)/Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:Graphics.DAT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- chaos strikes back (1991)(on-line)[cr betrayal](disk 1 of 2)/Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:VDEO.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- chaos strikes back (1991)(on-line)[cr betrayal](disk 1 of 2)/Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:ANIM.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- chaos strikes back (1991)(on-line)[cr betrayal](disk 1 of 2)/Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:MEM1.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- chaos strikes back (1991)(on-line)[cr betrayal](disk 1 of 2)/Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:USIO.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- chaos strikes back (1991)(on-line)[cr betrayal](disk 1 of 2)/Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:BETRAYAL!: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 10 never-loaded region(s), 597084 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:BETRAYAL!

- `run_setup_loop`: Performs a small sequence of setup operations, repeatedly invokes another routine and a function stored in a global variable table while decrementing a counter, then writes zero through a pointer.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `expand_and_accumulate_data`: Calls a routine multiple times to load data, then enters a loop that calls a routine to retrieve values, uses those values to index into a table, and adds the retrieved value to the table entry at the calculated offset.
- `load_constants`: Loads a constant value of 2 into register d0 and loads a memory address into register a1.
- `expand_compressed_data`: Decompresses a data stream using a variable-length encoding scheme where bytes are expanded based on bit-shifts and carry flags, then copies the resulting bytes into a destination buffer.
- `push_registers_to_stack`: Clears a byte at the address in a4, pushes d0, d1, and d2 to the stack, and restores several registers from the stack before branching.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the word from d1 to d0 and then performs a relative branch.

### SetPatch

- `initialize_setpatch_environment`: Allocates a memory structure, installs a resident patch handler via system hooks, and performs string parsing to process command-line arguments or configuration parameters.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `self_calling_guard`: Calls another routine unconditionally, then checks a global variable and, if non-zero, calls another routine with the input parameter, passes the result to another routine, and finally calls another routine unconditionally.
- `install_setpatch_handler`: Installs a resident patch handler and configures system-level hooks including exception vectors, layers, and trackdisk.
- `perform_zero_operation`: Performs a bitwise OR operation on d0 with zero.
- `call_indirect_via_table`: The routine calls another routine through a pointer stored in a global variable.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `allocate_and_patch_memory`: Checks fields of a global structure and, if conditions are met, obtains a memory block and a code pointer via library calls, copies a small code stub from another routine into the target, updates a pointer in the global structure, computes a checksum over a short array in the structure and stores its complement, then returns.
- `sum_and_jump`: Sums twenty-two 16-bit values from a table at offset 34 of a6, stores the bitwise NOT of the sum at offset 82 of a6, and then performs an indirect jump based on a comparison of a value at offset 30 of a5.
- `disable_interrupts_and_patch`: Checks a global variable and calls another routine through a table.
- `check_flag_and_copy_word`: Calls a routine and tests its return value; if non-zero, it copies a 32-bit word from offset 276 of a global variable into another global variable.
- `compare_and_jump`: Loads a long value into d1, loads a pointer from a constant, compares d1 to a value at an offset of that pointer, and if they are equal, jumps to a location stored in another pointer.
- `compare_and_exit`: Compares register d1 to the constant 255 and, if they are equal, branches to another location; otherwise it returns.
- `patch_interrupt_enable`: Sets the interrupt enable bit in the INTENA register and, if a specific constant is not found in a global variable, updates that variable and jumps to a location calculated from a table.
- `jump_to_offset_address`: Adds 136 to a value retrieved from a memory location and jumps to that address.
- `initialize_patch_function_table`: Writes a computed offset to a global variable and fills a 16-entry table with successive addresses of a routine, incrementing the pointer each iteration.
- `perform_logical_or`: Performs a bitwise OR of the value zero with d0.
- `call_subroutine`: Calls a subroutine using a brief jump to subroutine instruction.
- `execute_repeated_calls`: Calls another routine fifteen times in succession and then executes a no-operation instruction.
- `return_input_value`: Pushes d0 onto the stack, pushes a pointer to the stack, pops d0 from the stack, subtracts d0 from a value on the stack, pops d0 from the stack, adds a constant to a value on the stack, pushes another pointer onto the stack, and returns.
- `check_and_call_patch`: Compares a word at offset 20 of a structure in a6 with 35, and if it is less than or equal, performs an indirect jump using an offset from a6.
- `allocate_and_copy_structures`: Calculates a new size based on a word from a structure, calls a library routine to allocate memory, and then loops to copy data from a source structure to the new memory block while updating the source and destination pointers.
- `modify_register_and_call_2`: Decrements d2 and, if it is not negative, decrements a3, loads a1 from a pointer in a3, loads d0 from an offset in a3, calls another routine using an offset from a6, and then branches.
- `modify_register_and_call`: Calculates a value by shifting and adding to a value from a table, calls another routine, and then sets bit 31 of a value at a location pointed to by a2.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `transfer_stack_and_return`: Moves the contents of register a0 to register a5, writes the contents of register d1 to the stack at offset 2, masks the lower 5 bits of the stack pointer, and returns from an interrupt.
- `execute_conditional_callbacks`: Calls a routine from a table using a pointer at offset 68 of a structure in a1, and if the return value in d0 is non-zero, calls two more routines from other global table locations using a1 and a2.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `scan_for_marker`: Iterates through an array of words starting from an input pointer.
- `compute_word_offset_2`: Reads a 16-bit value from two bytes after an input pointer and returns a new pointer based on whether the low byte or the high byte of that value is non-zero.
- `compute_word_offset`: Reads a 16-bit value from two bytes after an input pointer.
- `patch_global_pointers`: Sequentially calls a helper routine to compute offsets and stores the returned pointers into a series of global variables.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### EndCLI

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a specific field, then writes a header and a sequence of words into a buffer.
- `move_and_branch_2`: Moves the contents of register d1 into register d0 and then branches to another location.

### FastMemFirst

- `initialize_fast_memory_and_parse_args`: Locates a fast memory block and retrieves the command line arguments from the operating system.
- `find_and_copy_fastmem_block`: The routine walks a linked list of memory blocks starting from a global variable, selects the first fast-memory block with a specific type tag and the first chip-memory block with another type tag, and if both are found and different it calls another routine on the fast block and then calls another routine with the list head, the fast block and the or chip block.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_138_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.

### ENDIF

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### IF

- `find_max_and_prepare_buffer_3`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `move_and_branch_3`: Moves a long value from register d1 to d0 and then branches to another location.

### Version

- `parse_command_string`: Allocates memory and parses a command string by splitting it into tokens based on whitespace, handling quoted substrings and converting specific character sequences into control characters.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `check_library_version`: Parses command-line arguments to open a specified library or device and validates its version and revision numbers against provided thresholds.
- `process_message_reply_2`: Calls a routine to handle message processing by updating message fields and calling various functions based on flags, then calls a library vector to finalize the operation.
- `parse_integer_string`: Parses a string to convert it into an integer, handling optional sign characters and digit sequences.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
- `calculate_string_length`: Calculates the length of a null-terminated string by iterating until the null byte is found.
- `call_reschedule_2`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch_2`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_indirect_with_stack_pointer`: Calls a function pointer stored in a global variable while passing two parameters and a pointer to a stack location as arguments.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `invoke_global_and_verify_checksum`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_at_pointer`: Stores a byte from d0 into the memory location pointed to by a3 and increments a3.

### Disk-Validator

- `update_disk_validation_data`: Calculates a character representation of a value from a global structure, writes it to a table, and performs a series of bitwise operations and memory updates involving multiple calls to other routines to manage a bitmask and a sequence of data values.

### Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:BJELoad_R

- `jump_to_loader_entry`: Performs a long jump to a target address located at a relative offset from the program counter.
- `execute_recursive_stack_operation`: Performs a recursive call to itself while passing a constant value and a stack-allocated buffer.
- `initialize_data_structure`: Populates a structure with several constant values and pointers, then updates a linked list or table entry based on the result of a function call.
- `update_structure_field`: Pops a pointer from the stack, checks a longword at offset 20 of the structure pointed to by a5, and if non-zero, copies a longword from offset 24 of that structure to the address at offset 24 of the structure pointed to by a2.
- `transfer_structure_data`: Copies data between offsets of two structures and conditionally updates a field based on the result of another routine.
- `execute_conditional_logic`: Initializes a local structure with a constant value and zero, calls a routine with a pointer to this structure, and then conditionally calls another routine based on the result.
- `dispatch_conditional_data`: Calls two different routines, and if the second returns a non-zero value, it increments a word at offset 48 of that value and branches to another location.
- `process_conditional_data_copy`: Calls three different routines based on the return values of previous calls, then performs a byte-by-byte copy from a stack buffer to a destination structure if a comparison between a value at offset 46 and a value at offset 14 of a structure succeeds, finally calling another routine with a pointer to that structure.
- `execute_conditional_calls`: Checks two different conditions and executes corresponding routines if the values are non-zero.
- `decrement_and_branch`: Decrements a short value at an offset within a structure and, if it reaches zero and a specific bit is set, executes multiple routines using the structure pointer.
- `set_status_bit`: Sets a specific bit in a byte located at an offset within a provided structure.
- `handle_complex_logic`: Performs a sequence of conditional checks and calls multiple routines, potentially updating a value in d4 and returning it.
- `validate_and_process_structure`: Validates a structure by checking specific byte and word values, then iterates through a table of 12-byte entries to track and update three specific indices based on type identifiers.
- `process_conditional_pointer`: Checks a value at offset 24 within a structure and, if non-zero, calls another routine before clearing that value, then calls a third routine.
- `initialize_pointer_list`: Sets a pointer to point to the next element in a list, clears the following element, and stores the original pointer address.
- `traverse_linked_list`: Calls a routine, then traverses a linked list by following pointers in a structure until a null pointer is encountered or a value at offset 8 matches a provided longword, finally calling another routine and returning the found pointer.
- `move_and_branch_4`: Copies a pointer from a memory location into a register and then branches to another routine.
- `call_and_branch`: Calls another routine, clears register d0, and then branches to another routine.
- `insert_node_into_list`: Swaps pointers to insert one structure into another, updating a link field within the structure.
- `link_two_structures`: Sets the first element of a structure to point to another structure and updates a link field in that second structure.
- `execute_sequence_of_calls`: Executes a sequence of three routine calls, including one with a specific constant argument.
- `process_global_data_loop`: Initializes several global variables and enters a loop that repeatedly calls another routine until a counter reaches a limit.
- `call_multiple_offsets`: Calls three different routines using the provided parameter and a version of the parameter with an offset applied.
- `iterate_and_call_nested_structures`: Traverses a linked list of structures, and for each structure where the word at offset 72 is zero, calls another routine with a pointer to the next structure in the list, repeating this process in a loop.
- `verify_magic_signature`: Compares a 16-bit word, a 16-bit word, and a 32-bit long at specific offsets within a structure pointed to by a global variable to verify a signature.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a5, then returns.
- `calculate_weighted_sum`: Calculates a sum by iterating through a table and adding the product of a byte and its index, then performs a second loop using a different table and a count from a structure.
- `sum_word_array`: Iterates through a table of 16-bit words, adding each word to a running total and decrementing a counter by two each time.
- `sum_byte_array`: Iterates through a table of bytes, adding each byte to a running total and decrementing a counter by one each time.
- `update_linked_structure`: Calls another routine with an offset of a pointer, and if the result is non-zero, updates a pointer in a structure and calls a third routine.
- `execute_conditional_recursion`: Checks if a parameter is non-zero and, if so, performs a recursive call using values derived from that parameter, then calls another routine before returning.
- `open_and_store_result`: Calls a routine with a constant parameter, and if the result is non-zero, stores that result in a location pointed to by a value at offset 12 of the structure in a5 before calling another routine.
- `execute_conditional_routine`: Calls a routine, then conditionally calls another routine if a parameter is non-zero, before calling a final routine.
- `execute_routine_with_constant`: Calls a routine with two parameters and a third constant parameter, surrounded by other routine calls.
- `compare_read_result`: Calls a routine with three parameters, then compares the result to a value at offset 12 of the structure in a5, returning 1 if they are not equal and 0 otherwise.
- `process_string_and_output`: Calculates a length, then uses that length to call two different routines with a parameter and the length.
- `read_input_and_process`: Calls two routines, and if the second returns a non-zero value, pushes a constant onto the stack, calls a third routine with the result, and finally calls a fourth routine before returning a byte from the stack.
- `call_and_branch_2`: Calls a routine, sets d0 to zero, and then branches to another location.
- `read_data_and_extend`: Calls two routines, and if the second returns a non-zero value, pushes a constant and a pointer onto the stack, calls a third routine, calls a fourth routine, and returns the signed extension of a byte from the stack.
- `call_routine_and_clear_d0`: Calls another routine, clears register d0, and performs an unconditional short branch.
- `handle_fatal_error`: Calls a routine, then calls another routine with the address of a global variable as a parameter, and then enters an infinite loop.
- `execute_callback_chain`: Calls a function pointer using three arguments and performs multiple indirect jumps and calls using a table.
- `call_subroutine_with_arguments`: Pushes three values from a structure onto the stack and calls another routine using a pointer from a structure.
- `execute_stack_dispatch`: Calls two external routines and returns.
- `transfer_register_to_d0`: Moves the contents of a register through a pointer into a data register and returns.
- `call_indirect_pointer`: Calculates an address using an offset from a pointer and executes the code at that location.
- `dispatch_via_stack`: Pops an address from the stack and jumps to it after pushing two other values from registers onto the stack.
- `dispatch_three_pointers`: Pops three addresses from the stack into registers and jumps to the first address.
- `jump_to_offset_address_2`: Loads a pointer from a global variable, calculates an address using a negative offset from that pointer, and jumps to it.
- `jump_handler_10`: Loads a pointer from a global variable and an offset of 64 from that pointer into registers, sets d0 to 16, and jumps to the address in a1.
- `jump_handler_17`: Loads a pointer from a global variable and an offset of 68 from that pointer into registers, sets d0 to 17, and jumps to the address in a1.
- `jump_handler_18`: Loads a pointer from a global variable and an offset of 72 from that pointer into registers, sets d0 to 18, and jumps to the address in a1.
- `jump_handler_19`: Loads a pointer from a global variable and an offset of 76 from that pointer into registers, sets d0 to 19, and jumps to the address in a1.
- `jump_handler_20`: Loads a pointer from a global variable and an offset of 80 from that pointer into registers, sets d0 to 20, and jumps to the address in a1.
- `jump_handler_22`: Loads a pointer from a global variable and an offset of 88 from that pointer into registers, sets d0 to 22, and jumps to the address in a1.
- `jump_handler_23`: Loads a pointer from a global variable and an offset of 92 from that pointer into registers, sets d0 to 23, and jumps to the address in a1.
- `jump_handler_21`: Loads a pointer from a global variable and an offset of 84 from that pointer into registers, sets d0 to 21, and jumps to the address in a1.
- `jump_handler_4`: Loads a pointer from a global variable and an offset of 16 from that pointer into registers, sets d0 to 4, and jumps to the address in a1.
- `jump_handler_6`: Loads a pointer from a global variable and an offset of 24 from that pointer into registers, sets d0 to 6, and jumps to the address in a1.
- `dispatch_handler_7`: Loads a pointer from offset negative 28 of a structure pointed to by a global variable into a1, sets d0 to 7, and jumps to the address in a1.
- `dispatch_handler_5`: Loads a pointer from offset negative 20 of a structure pointed to by a global variable into a1, sets d0 to 5, and jumps to the address in a1.
- `dispatch_handler_11`: Loads a pointer from offset negative 44 of a structure pointed to by a global variable into a1, sets d0 to 11, and jumps to the address in a1.
- `dispatch_handler_12`: Loads a pointer from offset negative 48 of a structure pointed to by a global variable into a1, sets d0 to 12, and jumps to the address in a1.
- `dispatch_handler_13`: Loads a pointer from offset negative 52 of a structure pointed to by a global variable into a1, sets d0 to 13, and jumps to the address in a1.
- `dispatch_handler_14`: Loads a pointer from offset negative 56 of a structure pointed to by a global variable into a1, sets d0 to 14, and jumps to the address in a1.
- `dispatch_handler_8`: Loads a pointer from offset negative 32 of a structure pointed to by a global variable into a1, sets d0 to 8, and jumps to the address in a1.
- `dispatch_handler_25`: Loads a pointer from offset negative 100 of a structure pointed to by a global variable into a1, sets d0 to 25, and jumps to the address in a1.
- `dispatch_handler_26`: Loads a pointer from offset negative 104 of a structure pointed to by a global variable into a1, sets d0 to 26, and jumps to the address in a1.
- `dispatch_handler_27`: Loads a pointer from offset negative 108 of a structure pointed to by a global variable into a1, sets d0 to 27, and jumps to the address in a1.
- `process_complex_data`: Performs multiple indirect calls and updates various fields in a structure, including setting specific multi-byte constants and managing nested pointers.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `manage_directory_info_and_files`: Retrieves directory information, checks if a specific field is non-zero, and performs a series of file operations including opening, locking, and duplicating file handles.
- `retrieve_directory_info`: Retrieves the current directory and calls a library function via a global pointer and offset.
- `copy_d0_to_d5`: Copies the long value from register d0 to register d5.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `copy_memory_block`: Copies a block of memory from a source to a destination, using a forward copy if the destination is higher than the source and a backward copy if the destination is lower than the source.
- `load_address_constant`: Loads the constant value seven million nine hundred ninety-nine thousand nine hundred ninety-nine into register a4 and returns.
- `parse_string_into_tokens`: Parses a string by identifying whitespace or quoted substrings, copies the resulting tokens into a new buffer, and stores an array of pointers to these tokens in a structure.
- `set_d0_and_branch`: Sets register d0 to the value thirty-two thousand seven hundred sixty-seven and branches to another location.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `initialize_window_structure`: Calls several routines to populate a structure with data, including a value from a global variable, a pointer from a structure, and several values retrieved through library calls, before storing a result in a global variable and clearing it.
- `copy_d0_to_d5_2`: Copies the 32-bit value from register d0 into register d5.
- `execute_cleanup_callback`: Checks a global variable and, if non-zero, executes the function pointed to by that value, then calls another routine.
- `cleanup_resources`: Iterates through a table to close files, unlocks resources, and executes several function pointers stored in a data structure.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `process_table_entry`: Calculates an offset into a table based on a value from a structure, checks bounds and nullity, and either calls another routine or sets a status byte and returns a failure code.
- `jump_to_close_wrapper`: Performs an indirect jump to a routine that calls the close function.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `jump_to_current_dir_wrapper`: Performs an indirect jump to a routine that calls the current directory function.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `jump_to_input_wrapper`: Performs an indirect jump to a routine that calls the input function.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `jump_to_open_wrapper`: Performs an indirect jump to a routine that calls the open function.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `jump_to_output_wrapper`: Performs an indirect jump to a routine that calls the output function.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_wait_for_char`: Wrapper that calls the dos library function WaitForChar and returns its result to the caller.
- `jump_to_write_wrapper`: Performs an indirect jump to a routine that calls a write function.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `jump_to_library_offset_414`: Performs an indirect jump to a library function at an offset of minus 414 from a base address stored in a global variable.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `jump_to_library_offset_198`: Performs an indirect jump to a library function at an offset of minus 198 from a base address stored in a global variable.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `jump_to_library_offset_210`: Performs an indirect jump to a library function at an offset of minus 210 from a base address stored in a global variable.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `jump_to_library_offset_552`: Performs an indirect jump to a library function at an offset of minus 552 from a base address stored in a global variable.
- `call_library_vector_552_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.

### Chaos Strikes Back (1991)(On-Line)[cr Betrayal](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_audio_and_data_tables`: Initializes several data structures by clearing memory buffers, populating tables with calculated sequences of values, and updating audio hardware registers.
- `check_interrupt_request`: Reads the interrupt request register, checks if bit 5 is set, and if so, writes that bit value back to the register.
- `swap_global_values`: Swaps two 32-bit values stored in a table, then stores the original first value as two 16-bit words at a different location in the table.
- `process_data_structure`: Sets a global variable to 4, calls another routine, sets the global variable to 8, and then performs a loop that updates values in a structure pointed to by a1 using data from a0.
- `process_data_loop`: Iterates through a loop to update values in a structure pointed to by a1, performing arithmetic and conditional calls to other routines, and modifies data at a0 based on the results.
- `store_longword_split`: Stores a 32-bit value from register a0 into a structure at register a1, splitting it into two 16-bit words at offsets 0 and 4.
- `calculate_vpos_offset`: Adds the value from the video position register to a constant, then masks the result with 7ff and stores it in register d4.
- `calculate_masked_value`: Performs arithmetic on register d6 and d5, then masks the result of d5 with 3 and adds a constant value before returning.
- `fill_and_copy_data`: Executes a nested loop that writes constant values and copies data from a table into a destination buffer, while calling other routines and writing values to a2.
- `fill_and_copy_buffers`: Sets a global variable to 4, calls another routine, sets the global variable to 8, and then performs a loop that writes constant values to a destination buffer and copies data from a source table.
- `update_audio_registers`: Processes a data stream to update various hardware registers including DMA audio registers and bitplane control registers through a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_channels`: Checks a value in d0 to select one of four audio channel configurations, then updates the corresponding hardware registers and memory structures with frequency, period, and volume data.
- `initialize_channel_structure`: Clears several fields of a channel structure, stores a pointer taken from a second structure, and sets a fixed longword, a mask byte and a flag byte.
- `update_music_channels`: Performs a loop to write calculated values into a table, then sequentially calls another routine multiple times with different pointer offsets, updating global variables based on the results.
- `step_music_channel_sequence`: Reads the next sequence event for one channel, loading its instrument and state from data tables, and updates the channel's pitch and volume counters.
- `set_bits_in_register`: Performs a series of bitwise OR operations on register d0 using several immediate byte values.
- `mask_and_store_byte`: Masks the lower four bits of a byte in d1 and stores the result at offset 47 of a structure in a6.
- `set_ciaa_bit`: Tests a byte in register d1 and, if non-zero, sets bit 0 of the CIAA Port A Data register before returning.
- `clear_joystick_bit`: Clears bit 0 of the CIAA Port A Data register.
- `store_table_pointer`: Masks its input to six bits, multiplies it by sixteen, adds a fixed base address, and stores the result as a pointer in a structure field.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
