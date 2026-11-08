# Game 446 (Detroit): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00446). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802240 top-level bytes in 109 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 262 of 262 named (0 author symbols), 262 of 262 described (53 by rule, 178 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 195 of 195 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4052 of 4052 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1189974 |
| code | 12428 |
| data | 400046 |
| padding | 24688 |
| unused | 175104 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| SetPatch | 2 | 954 | 83 |
| endcli | 2 | 117 | 2 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| loadwb | 12 | 536 | 30 |
| Version | 11 | 574 | 18 |
| Detroit (1994)(Impressions)[h PSG](Disk 1 of 2).adf:HDinstall | 7 | 45 | 2 |
| ram-handler | 2 | 10 | 1 |
| Generic | 7 | 8 | 4 |
| Detroit (1994)(Impressions)[h PSG](Disk 1 of 2).adf:Detroit | 7 | 251 | 6 |
| Detroit (1994)(Impressions)[h PSG](Disk 1 of 2).adf:zappata | 6 | 27 | 2 |
| icon.library | 10 | 1375 | 109 |
| Detroit (1994)(Impressions)[h PSG](Disk 1 of 2).adf:bootblock | 1 | 18 | 1 |
| Detroit (1994)(Impressions)[h PSG](Disk 2 of 2).adf:bootblock | 1 | 18 | 1 |

## Cases handled

- detroit (1994)(impressions)[h psg](disk 1 of 2)/Detroit (1994)(Impressions)[h PSG](Disk 1 of 2).adf:mod.intro: high-entropy data file (7.82 bits/byte), classified as packed data
- detroit (1994)(impressions)[h psg](disk 1 of 2)/Detroit (1994)(Impressions)[h PSG](Disk 1 of 2).adf:DetFX2.snd: high-entropy data file (7.64 bits/byte), classified as packed data
- detroit (1994)(impressions)[h psg](disk 1 of 2)/Detroit (1994)(Impressions)[h PSG](Disk 1 of 2).adf:HDinstall: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- detroit (1994)(impressions)[h psg](disk 1 of 2)/Detroit (1994)(Impressions)[h PSG](Disk 1 of 2).adf:Detroit: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- detroit (1994)(impressions)[h psg](disk 1 of 2)/Detroit (1994)(Impressions)[h PSG](Disk 1 of 2).adf:zappata: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### SetPatch

- `apply_patch_list_at_entry`: Checks for a required Kickstart version, parses command line switches, and iterates through a patch list to apply modifications to system libraries.
- `test_bit_and_branch`: Loads an address into a1, calls another routine, and returns if the result in d0 is non-zero; otherwise it returns.
- `check_bit_and_return`: Tests bit 3 of a byte at offset 361 from a pointer in a6; if the bit is set, it clears d0 and returns, otherwise it branches to another routine.
- `allocate_copy_stub_and_install`: Copies a 51-word code stub into an allocated memory block and then invokes several library routines to install it.
- `probe_memory_size_and_mask`: Verifies a fixed low-memory byte holds an expected value, then probes memory downward from a fixed high address to locate where a self-referencing pointer test stops matching, and finally stores a bit mask derived from that result into a fixed-address byte.
- `mask_word_at_offset`: Performs a bitwise AND between the value at offset 360 of a structure in a6 and the constant 8, then returns.
- `free_patch_slots_clear_fields`: Clears a pointer field in a record and releases it through a library routine at offset minus 210 when it was set, then walks six consecutive pointer slots freeing each non-zero one while clearing a companion field, and returns 1.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `compare_and_jump_2`: Compares a word at offset 28 of a structure in a1 to 28 and 11, and if neither matches, jumps to an address stored in a table.
- `modify_structure_and_call_handler`: Checks if a byte at offset 12 of a structure pointed to by a pointer at offset 40 of a1 is equal to four; if so, it modifies several fields within that structure and calls a routine via a pointer at offset 4 of a6.
- `compare_pointer_and_branch`: Compares a longword at offset 44 of a1 to zero; if it is non-zero, it branches to another routine, otherwise it branches to another routine.
- `call_two_cia_routines`: Calls two other routines in turn and returns zero.
- `relink_flagged_list_entry`: Looks up an entry through the library routine at offset minus 276 and, when its top flag bit is set, unlinks it, clears the flag, and relinks it between two library calls that disable and enable the system, returning the entry it found.
- `compare_and_branch`: Compares the word at offset 28 of a1 with five and branches if they are equal.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `call_jump_vector_stub`: Calls the routine reached through a jump-table entry, which passes control on to a fixed address, and returns with no result.
- `save_console_lookup_result`: Calls the library routine at offset minus 96 and, when it returns a pointer, saves that pointer, the 32-bit value at offset 6 inside it, and the incoming base pointer into three global variables.
- `compare_stack_value`: Loads a longword from the stack into d0 and compares it to a constant; if d0 is higher, it branches to another routine.
- `execute_indirect_jump`: Loads an address from a location near the program counter and jumps to it.
- `execute_conditional_indirect_jumps`: Compares d0 against a constant; if d0 is not higher, it returns.
- `perform_zero_or_instruction`: Performs a bitwise OR of the constant 0 with register d0.
- `store_incoming_pointer_a1`: It saves the pointer value passed in register A1 into a global variable and then returns.
- `iterate_and_clear_list_elements`: Calls two library routines using a pointer from a structure, then iterates through a linked list of pointers, calling a library routine for each element and clearing a value at offset 36 if the element and a previous register value are non-zero.
- `invoke_two_library_routines`: Invokes two library routines in sequence and returns 1.
- `clears_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `call_indirect_handler`: Invokes the routine stored at a fixed offset from a base pointer and returns.
- `store_offset_adjusted_lookup`: Selects one of two constant offsets depending on whether a structure field at offset 22 in a1 equals 44, then calls the library routine at a fixed offset from a6 and, if it returns a pointer, stores the chosen offset plus that pointer into a global variable.
- `clear_d0_register`: Performs a logical OR on register d0 with zero.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `update_structure_fields`: Sets a0 to a pointer in d1, doubles it twice, loads a longword from offset 32 of the new a0 into d0, compares d0 to a constant, and if they are not equal, writes d0 to offsets 16 and 20 of the structure in a0.
- `jump_to_indirect_address`: Loads a longword from a location relative to the program counter into a0 and jumps to the address contained in a0.
- `disable_one_interrupt_source`: Calls two routines at fixed offsets from a base pointer, writes the value sixty-four to the interrupt enable register to disable an interrupt source, then calls a third routine and returns one.
- `store_argument_and_return_one`: Stores the value in a1 into a global variable and returns one.
- `clears_data_register_zero_2`: Sets the least significant byte of data register zero to zero.
- `clear_d0_register_4`: Clears the d0 register by performing a bitwise OR operation with zero.
- `clear_d0_register_2`: Performs four consecutive logical OR operations on register d0 with zero.
- `update_table_and_fill_memory`: Calls another routine, updates a table with the returned value, and if the value matches a specific constant and a related field is non-zero, fills a memory block with a repeating pattern using a loop.
- `add_lookup_result_to_cell`: Calls the library routine at offset minus 96 and, if it returns a non-zero value, adds that value into a fixed global cell twice.
- `compare_stack_constant`: Compares a longword on the stack to the constant 96754; if they are equal, it branches to another routine, otherwise it increments a byte at offset 359 of a structure in a6 and returns.
- `modify_linked_list_node`: Compares a value on the stack to a constant, and if it matches, performs bitwise operations on a value in a0 and updates a pointer at an offset within a structure.
- `clear_flag_bit_then_call`: Takes a flag word from a library lookup when a configuration bit is set, or from a fixed low-memory address otherwise.
- `clear_data_byte`: Performs a logical OR operation on a byte in register d0 with zero.
- `push_and_compare_a0`: Pushes a longword from a memory location to the stack, compares a0 to zero, and if non-zero, branches to another instruction; otherwise it clears the stack and pushes d2 before branching.
- `clears_data_register_zero_3`: Performs a bitwise OR operation on data register zero with zero.
- `branch_to_offset_2`: Pushes a longword from a relative offset onto the stack and branches to another location.
- `perform_zero_or_instruction_2`: Performs a bitwise OR on register d0 with zero.
- `push_and_compare_a1`: Pushes a longword from a memory location to the stack, compares a1 to zero, and if non-zero, branches to another instruction; otherwise it clears the stack and pushes d0 before branching.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clears_data_register_zero_4`: Performs a logical OR operation on data register zero using zero as the operand.
- `push_and_branch`: Pushes a longword from a relative offset and the constant 2 onto the stack, then branches to another location.
- `clears_data_register_d0_2`: Performs a logical OR operation on register d0 with zero.
- `conditional_push_and_branch`: Pushes a longword from a relative offset onto the stack, compares d0 to a maximum value, and branches to push a longword of all ones and branch to another location if d0 is not equal to all ones.
- `clears_data_register_zero_5`: Performs a logical OR operation on data register zero using a zero constant.
- `pushes_constant_and_branches`: Pushes a longword from a location relative to the program counter onto the stack and branches to another routine.
- `traverse_linked_list`: Calls a library routine, then iterates through a linked list by checking if a pointer is non-zero and if a word at offset 18 of the current node does not match a value in d0, eventually branching to another routine.
- `execute_library_function`: Swaps a0 and a6, calls a routine at an offset from a6, swaps them back, and returns the value in a0.
- `return_parameter`: Takes a single parameter and returns it unchanged without modifying it.
- `search_and_update_list`: Checks a bit in a structure at offset 411 of a6, calls a library function, and then iterates through a list pointed to by the return value to find a specific word value, updating the list pointer accordingly.
- `call_library_vector_636`: Library stub: loads a library base from a global variable and calls the library function at offset minus 636 of that base, returning its result to the caller.
- `clears_data_register_d0_3`: Performs a bitwise OR operation on data register d0 with zero.
- `dispatch_jump_table`: Loads a pointer from a relative offset into a0, compares the word at that pointer to 24736, and if not equal, jumps to the address stored in a0.
- `calculate_offset_and_call`: Saves a4 to the stack, loads a word from offset 2 of a structure in a0 into d0, calculates an address in a4 using that word as an offset, calls another routine, and moves d0 to d1 before returning.
- `allocate_and_copy_string`: Calls a library routine to allocate memory, searches for a null terminator in a buffer in d1, and if found, copies the string to a new location, updates a byte at an offset, and calls two other routines.
- `copy_and_clear_byte_2`: Copies a block of bytes from one memory location to another using a loop, stores a byte from d2 into the destination, clears a byte at an offset, and calls another routine.
- `copy_and_clear_byte`: Copies a block of bytes from one memory location to another using a loop, stores a byte from d2 into the destination, clears a byte at offset 1 of a structure pointed to by a value derived from a5, and performs an indirect jump.
- `restore_registers_and_return`: Restores registers d2 through d7 and a2 through a3 from the stack and then returns.
- `clear_data_register_d0`: Sets the least significant byte of data register d0 to zero.
- `call_set_signal`: Library stub: loads its arguments and the exec library base, calls SetSignal and returns its result to the caller.
- `decrement_and_jump`: Increments a byte at offset 359 of a structure in a6, decrements a word at offset 14 of a structure in a0, and if that result is not zero, clears a word at offset 40 of the structure in a0, decrements a word at offset 44 of the structure in a0, and jumps to another routine if the result is non-negative.
- `decrement_and_loop`: If the result of a comparison is negative, it decrements a word at offset 44 of a structure in a0 and branches to another routine, otherwise it branches to another routine.
- `call_semaphore_and_alert`: Calls a routine at an offset from a6, then calls another routine at an offset from a6 using a value from the stack.
- `insert_into_list`: Updates a pointer in a structure, modifies a linked list by updating pointers, and calls a library routine if a bit is not set in a value from the list, then jumps to another routine.
- `insert_into_doubly_linked_list`: Increments a word at an offset in a structure, calls a routine, and if the result is non-zero and a bit is set, updates pointers to insert a node into a doubly linked list.
- `increment_and_jump`: Increments a byte in a global variable, increments a word at offset 44 of a structure in a0, and if the byte increment results in zero, increments a word at offset 14 of the structure in a0 before jumping to an address relative to a6.
- `clear_bit_and_call_routines`: If a value at an offset in a0 is non-zero, it clears bit 4 of a value in a global variable, calls two routines, and then calls a third routine via a jump.
- `clear_d0_register_3`: Performs a bitwise OR operation on register d0 with zero.
- `call_stack_check_handler`: Performs the same stack-bounds and status-word test as its neighbours.
- `clear_d0_register_5`: Clears the d0 register using a logical OR operation with zero.
- `validate_stack_and_call_handler`: Checks if the current stack pointer is within the bounds defined by a structure at offset 420 of a6 and verifies a 16-bit status field at offset 44 of another structure at offset 412 of a6; if these conditions are met, it calls a routine through a pointer located at a relative offset from the program counter.
- `store_library_base_globals`: Copies the value from a1 into a global variable, copies the value from a6 into another global variable, and returns one.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `compare_and_jump`: Compares a long value at offset 4 of a structure in a1 to 1968, and if they are not equal, jumps to an absolute address in d0.
- `compare_and_update_value`: Compares a word at offset 28 of a structure in a1 to 9, checks if a1 is null, and if a specific long value at a calculated offset matches d0, it writes 2968 to offset 4 of a1.

### endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of d1 to d0 and branches to another location.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value in a field, then if a condition is met, writes a header and fills a buffer with sequential words.
- `move_d1_to_d0`: Moves the value from d1 to d0 and branches to another routine.

### loadwb

- `parse_command_string`: Parses a string by splitting it into tokens based on whitespace and special characters, handling quoted substrings and performing character replacements for specific escape sequences.
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
- `compare_workbench_string`: Compares a string at a pointer in a2 with a constant string, checking if the byte at offset 32 of the structure is 50, and performs a byte-by-byte comparison if it matches.
- `initialize_data_bytes`: Writes the value 46 to offset 17 of the structure in a2 and the value 32 to offsets 18, 19, 20, and 21 of the same structure, then branches to another routine.
- `compare_memory_blocks`: Copies a pointer from a2 to a0, then compares a block of memory starting at a pointer relative to the program counter with the memory at the pointer in a0, branching to another routine if the comparison fails or if the byte is zero.
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

### Version

- `parse_command_string_2`: Parses a command string by splitting it into tokens based on whitespace and handling quoted substrings, including converting specific character sequences into newline or escape characters, and stores the resulting pointers in a structure.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `check_library_version`: Parses command-line arguments to open a specified library or device and validates its version and revision numbers against provided thresholds.
- `process_message_reply_2`: Calls a routine to handle message processing by updating message fields and calling various functions based on flags, then calls a library vector to finalize the operation.
- `parse_integer_string`: Parses a string to convert it into an integer, handling optional sign characters and digit sequences.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
- `calculate_string_length`: Calculates the length of a null-terminated string by iterating until the null byte is found.
- `call_reschedule_2`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch_2`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_workbench_string_comparator`: Pushes two parameters and a pointer to a stack location onto the stack and calls a routine that compares a string at a pointer in a2 with a constant string, checking if the byte at offset 32 of the structure is 50, and performs a byte-b.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `verify_and_dispatch_checksum`: Invokes a function pointer from a global variable and computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_at_pointer`: Stores a byte from register d0 into the memory location pointed to by a6 and increments a6.

### Detroit (1994)(Impressions)[h PSG](Disk 1 of 2).adf:HDinstall

- `run_hdinstall_entry`: Program entry for the HDinstall tool.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### ram-handler

- `copy_memory_and_jump`: Calls a routine to copy memory from a source in d2 to a destination in d1, then performs an indirect jump to a location in a6.

### Generic

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.

### Detroit (1994)(Impressions)[h PSG](Disk 1 of 2).adf:Detroit

- `run_hdinstall_entry_2`: Executes a sequence of helper calls and a loop that calls a memory management function three times, clears a word in a shared record, and swaps a table entry with a RAM variable when it matches a fixed constant.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.
- `decode_bitstream_and_update_table`: Calls load_address_and_constant and decode_variable_length_bitstream, then enters a loop that calls another routine and updates a table at a pointer in a3 using values from d0 and d2, before potentially calling another routine and returning.
- `load_address_and_constant`: Loads a longword address into a1 and sets d0 to two.
- `decode_variable_length_bitstream`: Reads a bitstream from a source buffer, performing bitwise shifts and additions to extract values based on a variable-length encoding scheme.
- `perform_zero_or`: Performs a bitwise OR of the constant zero with register d0.

### Detroit (1994)(Impressions)[h PSG](Disk 1 of 2).adf:zappata

- `free_memory_clear_slot`: Saves the registers on the stack, calls two unnamed routines, then calls another unnamed routine and frees a memory block through exec.FreeMem.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `set_icon_bit_2`: As library code, it sets bit 3 of a byte at offset 14 of the structure in a6 and returns zero.
- `set_icon_bit`: As library code, it sets bit 3 of a byte at offset 14 of the structure pointed to by a6 and returns zero.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `update_icon_data`: As library code, it stores two values into a structure at offsets 42 and 34 from a2, calls another routine, and if the result is zero, calls a routine through a pointer at offset 4 of a6; otherwise it stores the result and the original a6 value into two global variables.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `clear_data_register_d0_3`: Clears the least significant byte of data register d0.
- `clear_data_register_d0_2`: Sets the least significant byte of register d0 to zero.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `manage_icon_chain_3`: As library code, it pushes a6 and a0 onto the stack, calls another routine, and returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `invoke_icon_chain_handler`: Pushes two pointers from the stack onto the stack and calls another routine, then returns.
- `trigger_icon_chain_recursion`: Saves two registers to the stack and calls perform_incremental_recursion to process an icon chain.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_handler_and_return`: This library code calls another routine and then returns.
- `call_and_clear_byte`: Clears a byte at offset 30 of the structure in a2, calls another routine with a pointer and a constant, and returns if the result is non-zero.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `clear_data_register_zero`: Clears the least significant byte of data register zero.
- `allocate_and_initialize_icon`: This library code calls a routine with a pointer from the stack, and if the result is non-zero, calls another routine with two pointers and then calls a third routine with two pointers if the result of the second call is non-zero.
- `copy_and_assign_fields`: As library code, it copies 11 words from one location to another using a loop, then copies several specific bytes and words from a structure in a6 to a structure in a2.
- `return_zero_stub`: Clears register d0 to zero and restores registers d2 through d4 and a2 from the stack before returning.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `allocate_icon_resources`: Allocates memory and manages image data for an icon structure by calling several subroutines to handle property updates and image allocation.
- `update_icon_image_and_properties`: Checks if a specific icon type is being processed, then updates its image pointer and properties by calling other routines to allocate or modify memory and structure offsets.
- `update_icon_properties`: Library code that calls a routine to update offsets 18 and 22 of a structure in a4, then conditionally calls a routine in a3 and a library routine to update offsets 50 and 54 of a structure in a2.
- `dispatch_icon_updates`: As library code, it calls a routine in a3, and if successful, enters a loop that calls another routine to update a pointer in a4, then checks a value at offset 112 of a structure in a2 to decide whether to perform a second sequence of routine calls and updates.
- `clear_d4_and_branch`: As library code, it sets d4 to zero and branches to another location.
- `restore_register_and_return`: Moves a longword from register d4 into d0, restores registers d2 through d5 and address registers a2 through a5 from the stack, unlocks a6, and returns.
- `update_icon_data_structure`: Checks a status flag at offset 16 of a structure and clears a value at offset 26 if a specific bit is set.
- `update_icon_attributes`: Checks flags in an icon structure to determine if specific attributes should be cleared or updated, then conditionally calls a function pointer to process various icon-related data, including a null-terminated list of pointers.
- `process_icon_table`: Library code that iterates through a null-terminated table of pointers, calling a routine for each entry, and conditionally executes a callback using a pointer from offset 50 of a structure in a2.
- `iterate_icon_table_entries_2`: Increments a pointer through a null-terminated list in a loop, calls a routine at the address in a4 using arguments from the stack, and then processes a table at an offset of 54 from a2 if the call returns a non-zero value.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `branch_to_middle_of_routine`: Pushes the contents of a2 onto the stack, loads a value from an offset of 12 from the stack pointer into a0, sets a2 to the value at offset 2 from a0, and branches to a different routine.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `allocate_and_setup_handler`: Library code that allocates a block of memory of size 65536, calls a routine with the new pointer, and then performs several operations including calling a routine with a pointer from offset 140 of the allocated block.
- `call_handler_with_stack_setup`: As library code, it pushes the value of a2 and an address from a structure in a2 onto the stack, pushes d2, calls another routine, and then either branches to another instruction or moves a2 to d0 before branching.
- `clear_registers_and_return`: Sets d0 to zero, restores d2, d3, and a2 from the stack, and returns.
- `decrement_and_call_handler`: As library code, it decrements a word at a pointer in a2, and if the result is not greater than zero, calls another routine with values from a0 and d2, then branches based on the result.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `allocate_and_setup_resources`: This library code calls a routine with a constant, and if the result is non-zero, calls another routine with a pointer and a constant, then calls a third routine with two pointers, and if that result is non-zero, calls a fourth routine with three pointers.
- `branch_to_offset`: Performs a short branch to a specific offset within another routine.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `perform_incremental_recursion`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `execute_and_checksum_buffer`: As library code, it pushes a value from offset 28 of the stack onto the stack, calls a routine, and then calls another routine using the result in d0 and a pointer from offset 24 of the stack.
- `search_byte_constant`: Library code that calls another routine, then checks if the result is non-zero before searching for the byte value 61 in a buffer pointed to by a3, returning the new pointer in d0.
- `pop_and_check_pointer`: Library code that pops a pointer from a table into a3, moves it to d0, and returns d0 if the pointer is non-zero, otherwise it returns zero.
- `invoke_handler_and_checksum`: This library code loads two values from the stack, calls a routine with the second value as an argument, and then branches to another routine.
- `compare_and_subtract_pointers`: Library code that calls a routine with a pointer from offset 123 of the stack, moves the result to d4, and if the result is non-zero, subtracts the original pointer in d2 from d4 before returning.
- `verify_and_call_subroutine`: Library code that calls a routine using a pointer in d2, compares the result to d5, and if equal, calls another routine using values from the stack, returning 1 in d0 if successful or 0 otherwise.
- `increment_and_branch_d2`: If the long value in d4 is not zero, it copies d4 to d2, increments d2 by one, and branches to another location; otherwise it returns.
- `clear_and_restore_registers`: Clears register d2 and returns if it is non-zero, otherwise it clears register d0 and restores registers d2 through d5 from the stack before returning.
- `process_string_and_call_routines`: This library code clears a byte at offset 30 of a structure in a2, calls a routine with a pointer and a constant, and if the result is non-zero, calls another routine with a pointer and then calls a third routine with three pointers.
- `call_routine_with_stack_parameters`: As library code, it pushes two pointers onto the stack, calls a routine with a pointer from offset 4 of a4, and if the result in d0 is non-zero, it modifies the stack pointer and branches to another location; otherwise it sets d3 to 1, increments a pointer in a4 by 3, and branches to another location.
- `set_d3_to_zero`: Library code that sets d3 to zero and branches to another location.
- `process_scaled_values`: Library code that calculates a value based on d3 and d2, uses a loop to call a routine with bytes from a table in a4, and subsequently calls two other routines using pointers and values from the stack.
- `clamp_value_to_range`: Compares a long value from the stack with the constant 48; if the value is greater than 48, it sets d0 to 48; if the value is less than 48, it sets d0 to 57; otherwise it sets d0 to the value from the stack and subtracts 48 from it.
- `set_d0_to_negative_one`: Sets d0 to negative one and returns.
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
- `allocate_and_initialize_structure`: As library code, it calls a routine with a constant, and if the result is non-zero, it writes 10 to offsets 14 and 14 of a structure and calls another routine with the result.
- `clear_d0_and_restore_registers`: Sets register d0 to zero and restores d2 and a2 from the stack before returning.
- `call_handler_with_stack_args`: Loads two values from the stack, pushes them onto the stack, calls a routine at an absolute address, and then returns either zero or the value in d0 based on a comparison with a value from the stack.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `call_library_function_with_args`: As library code, it loads d0 and d1 from the stack, pushes them onto the stack, calls a routine at offset 4c, and then sets d0 based on the result or zero.
- `return_d1_as_d0`: Copies the contents of register d1 into register d0 and returns.
- `perform_integer_division`: If d2 is not greater than 65535, it performs a series of operations involving swapping and dividing values in d1 and d0 using a1 and d2; otherwise, it returns.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_return`: Stores the byte from d0 into the memory location pointed to by a3 and increments a3 before returning.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `compare_and_update_d0`: As library code, it loads a pointer from offset 4 of the stack and a word from offset 8, compares a byte from the pointer to the word, and if they match, decrements the pointer and moves it into d0 before returning.
- `set_d0_and_branch`: Sets d0 to zero and branches to another routine.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `copy_null_terminated_string`: Copies bytes from a source pointer in a0 to a destination pointer in a1 in a loop until a zero byte is encountered, then decrements a0 and branches back to the start of the loop.
- `check_byte_equality`: As library code, it decrements a long value in d0, and if it is not less than zero, compares a byte from a pointer in a1 to a byte from a pointer in a0, returning zero if they match and the byte in a1 is zero.
- `set_d0_conditional`: Sets d0 to one if it is greater than zero, otherwise it sets d0 to 255 and returns.
- `test_and_decrement_d0`: As library code, it tests a byte at the pointer in a0, and if the byte is non-zero, decrements a long value in d0 and branches based on whether the result is greater than zero.
- `decrement_pointer_and_call`: Decrements the pointer in a0 by one and calls another routine.
- `copy_bytes_with_counters`: Decrements d0 and branches if it is less than zero; otherwise, it copies bytes from a1 to a0 in a loop until d0 reaches zero or a byte is non-zero, then performs a final byte clear and branches.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `restore_registers_from_stack`: As library code, it saves d2, d3, and a2 to the stack, loads d2, d3, and a2 from offsets 16, 20, and 24 of the stack, and branches to another instruction.
- `iterate_icon_list`: As library code, it enters a loop that calls another routine, checks d0, and if non-zero, performs sign extension and arithmetic on a byte from offset 7 of a structure in a2 before calling another routine with a new pointer from offset 12 of that structure, repeating until d0 is zero.
- `return_zero_with_extended_registers`: Sets register d0 to zero, restores d2, d3, and a2 from the stack, and returns.
- `propagate_icon_changes`: As library code, it performs several calls to other routines using values from the stack, performs arithmetic on a value from a structure, and updates a structure at offset 10.
- `return_negative_one`: Sets register d0 to negative one and restores the previous register state before returning.
- `save_registers_from_stack`: As library code, it saves d2, d3, d4, d5, and d6, and a2 to the stack, then loads d2, d3, and d4 from offsets 28, 32, and 36 of the stack before branching.
- `execute_stack_based_call`: As library code, it pushes a constant and two values from the stack, calls a routine at offset 0, and then updates d6 based on the result or a value from the stack.
- `propagate_icon_changes_2`: As library code, it calls another routine, processes a byte from offset 7 of a structure in a0, calls a second routine with a modified value, and then calls a third routine, repeating a loop based on a value at offset 12 of a structure in a0.
- `return_negative_one_2`: Sets register d0 to negative one and restores registers from the stack before returning.

### Detroit (1994)(Impressions)[h PSG](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Detroit (1994)(Impressions)[h PSG](Disk 2 of 2).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
