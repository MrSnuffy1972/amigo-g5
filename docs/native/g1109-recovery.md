# Game 1109 (wing commander (1992)(mindscape)(disk 1 of 3)[cr ministry]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01109). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909424 top-level bytes in 41 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 17 of 17 units identical (bytes and relocations), 15 raw-word lines |
| Functions named and described | pass, 312 of 312 named (0 author symbols), 312 of 312 described (57 by rule, 252 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1100 of 1100 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9674 of 9683 instructions identical in start and length, 2 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 17352 |
| code | 30336 |
| data | 825886 |
| padding | 10762 |
| unused | 25088 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Setmap | 13 | 653 | 31 |
| Else | 2 | 117 | 2 |
| SetPatch | 24 | 833 | 45 |
| EndCli | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| FastMemFirst | 7 | 207 | 6 |
| Endif | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| setpatchV38.31 | 2 | 893 | 76 |
| Version | 11 | 574 | 18 |
| Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:intro.exe | 1 | 212 | 5 |
| d | 1 | 13 | 1 |
| gb | 1 | 4 | 1 |
| Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:InstallWingCommander | 2 | 3734 | 67 |
| Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:Wing | 8 | 253 | 7 |
| Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1827 | 45 |

## Cases handled

- wing commander (1992)(mindscape)(disk 1 of 3)[cr ministry]/Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:PLANET0.PKT: high-entropy data file (7.70 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)[cr ministry]/Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:ENDGAME.PKT: high-entropy data file (7.92 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)[cr ministry]/Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:FONTS.PKT: high-entropy data file (7.88 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)[cr ministry]/Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:PCSHIP03.PKT: high-entropy data file (7.95 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)[cr ministry]/Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:PCSHIP01.PKT: high-entropy data file (7.96 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)[cr ministry]/Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:MIDGAME2.PKT: high-entropy data file (7.94 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)[cr ministry]/Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:DEMO.PKT: high-entropy data file (7.97 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)[cr ministry]/Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:MIDGAME0.PKT: high-entropy data file (7.96 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)[cr ministry]/Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:PCSHIP02.PKT: high-entropy data file (7.93 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)[cr ministry]/Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:DEMOSAMP.PKT: high-entropy data file (7.97 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)[cr ministry]/Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:MIDGAME3.PKT: high-entropy data file (7.93 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)[cr ministry]/Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:PCSHIP00.PKT: high-entropy data file (7.96 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)[cr ministry]/Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:MIDGAME1.PKT: high-entropy data file (7.91 bits/byte), classified as packed data
- runtime segment an address: 889 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- wing commander (1992)(mindscape)(disk 1 of 3)[cr ministry]/Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:Wing: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:intro.exe

## Copy protection found

- none found

## Functions


### Setmap

- `parse_command_line_arguments`: Parses a string into individual tokens, handling quoted substrings and converting specific character sequences into control characters, then stores the resulting pointers and lengths into a structure.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `invoke_optional_handle_routines`: Calls a routine with each of several global handle values when they are set, passing one global by reference, then always calls it with a fixed handle and the incoming argument.
- `store_arguments_and_call_helpers`: Stores its two arguments into two global variables, then calls one helper with a fixed set of arguments that includes a global address and constants.
- `resolve_keymap_and_initialize_console`: Determines a keymap name from icon tool types, command-line arguments, or a default value, converts the string to lowercase, and attempts to open the corresponding keymap resource and the console device.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `pass_params_to_subroutine`: Calls another routine with a global value, its parameter and the address of an eight-byte stack slot.
- `call_global_library_function`: Calls one library function through a library base read from a global variable at a fixed offset, passing no arguments, and returns.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Else

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves a longword from d1 to d0 and then branches to another location.

### SetPatch

- `initialize_setpatch_environment`: Allocates a structure to store patch state, installs a resident handler, and performs system patches including exception vectors and trackdisk.
- `update_patch_state_and_cleanup`: Saves a parameter into a structure slot and then conditionally invokes a sequence of cleanup callbacks stored in a table, passing specific constants to each call before execution.
- `self_calling_guard`: The routine calls another routine unconditionally, then checks a global variable and, if non-zero, calls another routine with the input parameter, passes the result to another routine, and finally calls another routine unconditionally.
- `install_setpatch_handler`: Installs a resident patch handler and performs a series of conditional system patches including exception vectors, layers, and trackdisk.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_indirect_via_table`: The routine calls another routine through a pointer stored in a global variable.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `allocate_and_patch_memory`: Checks fields of a global structure and, if conditions are met, obtains a memory block and a code pointer via library calls, copies a small code stub from another routine, updates a pointer in the global structure, computes a checksum over a short array in the structure and stores its complement, then returns.
- `sum_and_jump_table`: Sums a series of 22 words from a table at offset 34 of a6, stores the bitwise NOT of the result in offset 82 of a6, and performs an indirect jump through a pointer at offset 62 of a5 after verifying a longword at offset 30 of a5.
- `disable_interrupts_and_patch`: The routine checks a global variable and calls another routine through a table.
- `check_flag_and_copy_word`: The routine calls another routine and tests its return value.
- `jump_on_comparison`: Loads a long value into d1, loads a pointer from offset 4, compares the value in d1 with a long value at offset 276 of that pointer, and if they are not equal, jumps to a location stored at offset 184 of the PC-relative address.
- `compare_and_return`: Compares d1 with the constant 255 and returns immediately if they are equal, otherwise it returns.
- `set_interrupt_and_jump`: Sets bit 15 of the INTENA register, compares a constant with a global variable, and if they differ, updates the global variable, modifies a value at offset 100, and performs an indirect jump to an address offset by 48 from a pointer in a5.
- `execute_indirect_jump`: Loads an address from a memory location, adds a constant to it, and jumps to that address.
- `initialize_patch_function_table`: Calculates an offset from a word in a structure, writes that offset to a global variable, and fills a 16-entry table with successive addresses of a routine, incrementing the pointer each iteration.
- `clear_d0_register`: Performs a bitwise OR on register d0 with zero.
- `call_subroutine_via_branch`: Calls a subroutine using a brief branch instruction.
- `repeat_subroutine_calls`: Calls another routine 15 times in succession and then returns.
- `return_input_value`: The routine receives a value in D0 and returns it unchanged.
- `check_and_jump_to_handler`: Compares a word at offset 20 of a global variable to 35, and if it is not less than or equal to that value, it performs an indirect jump using a pointer from a global variable.
- `allocate_and_copy_list`: Calculates a size based on a word at offset 14 of a structure in a0, calls a library routine to allocate memory, and if the result is non-zero, copies elements from the source structure to the new memory while calling another routine for each element.
- `branch_on_decrement`: Decrements a word in register d2 and branches if it is negative; otherwise, it decrements a long value in a3, loads a pointer from that address into a1, loads a long value from offset 4 of that pointer into d0, calls another routine, and branches back to the start of this routine.
- `process_and_set_bit`: Calculates a value by shifting a word from offset 14 of a4 and adding 16, calls another routine, and then sets bits 0 through 31 of a byte at offset 0 of a2.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `restore_processor_state`: Moves the contents of register a0 into register a5 and restores the processor state using an exception return.
- `execute_conditional_callbacks`: Calls a routine using a pointer from a table, and if the return value is non-zero, calls two more routines using pointers from another table, passing a value from a structure in a1 to the final call.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `scan_for_marker`: Iterates through an array of words starting from an input pointer.
- `compute_word_offset_2`: Reads a 16-bit value from a pointer in a0 and returns a new pointer by adding either a shifted version of the low byte or a shifted version of the high byte to the original pointer.
- `compute_word_offset`: Reads a 16-bit value from two bytes after an input pointer.
- `patch_global_pointers`: The routine sequentially calls a helper routine to compute offsets and stores the returned pointers into a series of global variables.
- `call_reschedule_2`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch_2`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_with_stack_pointer`: Loads two values from the stack, pushes a pointer to a stack location, and calls another routine using the two values and the stack pointer as arguments.
- `invoke_global_and_verify_checksum_2`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_and_increment`: Stores a byte from register d0 to the memory location pointed to by a3 and then increments a3.

### EndCli

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `move_and_branch_2`: Copies the contents of d1 to d0 and then branches to another location.

### Echo

- `find_max_and_prepare_buffer_3`: Walks a linked list of entries to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `transfer_register_and_branch`: Moves the contents of register d1 into register d0 and then performs a short branch to another location.

### FastMemFirst

- `initialize_fast_memory_and_parse_args`: Attempts to locate a fast memory block and retrieves the command line arguments from the operating system.
- `find_and_copy_fastmem_block`: Walks a linked list of memory blocks starting from a global variable, selects the first fast-memory block with a specific type tag and the first chip-memory block with another type tag, and if both are found and different it calls another routine on the fast block and then calls another routine with the list head, the fast block and the chip block.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.

### Endif

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `find_max_and_prepare_buffer_4`: Walks a linked list of entries to find the maximum value in a specific field, then writes a header and a sequence of words into a buffer.
- `transfer_register_and_branch_2`: Copies the contents of register d1 into register d0 and then performs a short branch to another location.

### setpatchV38.31

- `apply_patch_list_with_switches`: Checks the system version and, if a specific Kickstart version is met, iterates through a list of patch records to apply modifications to system libraries.
- `test_and_branch_routine`: Loads an address into a1, calls a library routine, and if the result in d0 is non-zero, branches to another routine; otherwise it returns.
- `test_bit_and_branch`: Tests bit 3 of a byte at offset 329 from a pointer in a6; if the bit is set, it branches to another routine, otherwise it clears d0 and returns.
- `allocate_copy_stub_and_install`: Copies a 51-word code stub into an allocated memory block and then invokes several library routines to install it.
- `probe_memory_size_and_mask`: Verifies a fixed low-memory byte holds an expected value, then probes memory downward from a fixed high address to locate where a self-referencing pointer test stops matching, and finally stores a bit mask derived from that result into a fixed-address byte.
- `free_patch_slots_clear_fields`: Clears a pointer field in a record and releases it through a library routine at offset minus 210 when it was set, then walks six consecutive pointer slots freeing each non-zero one while clearing a companion field, and returns 1.
- `clears_data_register_zero_2`: Performs a logical OR operation on data register zero with zero.
- `compare_and_jump_table`: Compares a word at offset 28 of a structure in a1 to two constants and branches if they match, otherwise it jumps to an address loaded from a table.
- `clear_message_fields`: Checks if a byte at offset 12 of a structure pointed to by a0 is 4; if so, it clears several fields within that structure and calls another routine.
- `branch_on_long_word`: Tests if the long word at offset 44 of a structure in a1 is non-zero; if it is, it branches to another routine, otherwise it branches to another routine.
- `call_two_cia_routines`: Calls two other routines in turn and returns zero.
- `relink_flagged_list_entry`: Looks up an entry through the library routine at offset minus 276 and, when its top flag bit is set, unlinks it, clears the flag, and relinks it between two library calls that disable and enable the system, returning the entry it found.
- `compare_and_branch`: Compares a 16-bit value at offset 28 of the structure in a1 to 5 and branches if they are equal.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `call_jump_vector_stub`: Calls the routine reached through a jump-table entry, which passes control on to a fixed address, and returns with no result.
- `save_console_lookup_result`: Calls the library routine at offset minus 96 and, when it returns a pointer, saves that pointer, the 32-bit value at offset 6 inside it, and the incoming base pointer into three global variables.
- `compare_and_branch_2`: Loads a long word from the stack into d0 and compares it to a constant; if d0 is higher, it branches to another routine.
- `execute_indirect_jump_2`: Loads an address from a location relative to the program counter and jumps to it.
- `execute_conditional_library_calls`: Compares d0 to a value from a table, and if d0 is not higher, calls a routine from a table, then optionally calls another routine from a second table if the first call returns a non-zero value.
- `store_offset_adjusted_lookup`: Selects one of two constant offsets depending on whether a structure field at offset 22 in a1 equals 44, then calls the library routine at a fixed offset from a6 and, if it returns a pointer, stores the chosen offset plus that pointer into a global variable.
- `clears_data_register_zero_3`: Sets the least significant byte of data register zero to zero.
- `clears_data_register_zero_4`: Sets the lowest byte of data register zero to zero.
- `update_structure_fields`: Calculates an address by doubling a pointer in a0 three times, loads a long word from offset 32 of that address into d0, and if d0 matches a constant, writes the value negative one to offsets 16 and 20 of that address.
- `jump_to_indirect_address`: Loads an address from a location relative to the program counter into a0 and jumps to it.
- `disable_one_interrupt_source`: Calls two routines at fixed offsets from a base pointer, writes the value sixty-four to the interrupt enable register to disable an interrupt source, then calls a third routine and returns one.
- `store_argument_and_return_one`: Stores the value in a1 into a global variable and returns one.
- `clear_data_register_zero`: Sets the lowest eight bits of data register zero to zero.
- `zeroes_data_register_zero`: Performs a logical OR operation on the lowest byte of data register zero using the constant zero.
- `clear_d0_register_quadruple`: Performs a bitwise OR on register d0 with zero four times.
- `update_table_and_fill_memory`: Calls another routine, updates a table with the returned value, and if the value matches a specific constant and a related field is non-zero, fills a memory block with a repeating pattern using a loop.
- `add_lookup_result_to_cell`: Calls the library routine at offset minus 96 and, if it returns a non-zero value, adds that value into a fixed global cell twice.
- `increment_byte_on_match`: Compares a long word on the stack to a constant; if it matches, it increments a byte at offset 295 of a structure in a6, otherwise it returns.
- `modify_linked_list_entry`: Compares a value on the stack to a constant, and if it matches, checks a bit in a field at offset 20 of a structure in a0 and, if set, updates a pointer at offset 44 of that structure to the value at offset 44 of the current structure.
- `clear_flag_bit_then_call`: Takes a flag word from a library lookup when a configuration bit is set, or from a fixed low-memory address otherwise.
- `clear_data_register_d0`: Performs a logical OR operation on data register d0 with the constant zero.
- `push_and_branch_if_nonzero`: Pushes a long word from a memory location onto the stack, compares a0 to zero, and if non-zero, branches to another routine; otherwise it clears the top of the stack and branches to another routine.
- `clear_data_register_d0_2`: Performs a logical OR operation on data register zero with zero to ensure it contains zero.
- `push_and_branch`: Pushes a long word from a memory location onto the stack and branches to an earlier instruction.
- `clear_data_register_d0_3`: Performs a logical OR operation on data register zero using a zero constant.
- `push_and_branch_if_nonzero_alt`: Pushes a long word from a memory location onto the stack, compares a1 to zero, and if non-zero, branches to another routine; otherwise it clears the top of the stack and branches to another routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_data_register_zero_2`: Performs a bitwise OR operation on register d0 with a zero constant.
- `push_two_values_and_branch`: Pushes a long word from a memory location onto the stack, pushes the constant 2 onto the stack, and branches to an earlier instruction.
- `clears_data_register_zero_5`: Performs a logical OR operation on the lowest byte of data register zero using a zero constant.
- `compare_and_branch_with_stack_push`: Pushes a long word from a memory location onto the stack, compares register d0 to a negative constant, and if d0 is not equal to that constant, pushes a negative constant onto the stack and branches to an earlier instruction.
- `clear_d0_register_2`: Performs a bitwise OR operation on register d0 with zero.
- `push_address_and_branch`: Pushes a long word from a memory location relative to the program counter onto the stack and branches to another routine.
- `call_library_and_traverse_list`: Calls a library routine using a6, then traverses a linked list from a structure in a0, comparing a word at offset 24 of each element with d0, and if they match, it updates a pointer and branches to another routine.
- `call_library_routine_with_exchange`: Exchanges d0 and a6, calls a library routine at an offset from the new a6, exchanges them back, and returns d0.
- `return_parameter`: Takes a single parameter and returns it unchanged without modifying it.
- `search_and_update_list`: Checks a bit in a structure at an offset from a base register, calls a library function, and then iterates through a list to find a specific word value before updating a pointer in that list.
- `call_library_vector_636`: Library stub: loads a library base from a global variable and calls the library function at offset minus 636 of that base, returning its result to the caller.
- `clears_data_register_zero_6`: Performs a logical OR operation with zero on data register zero.
- `execute_indirect_jump_3`: Loads a pointer from an offset relative to the program counter into a6, compares a word at that pointer to 24832, and if they are equal, performs an indirect jump using the pointer.
- `call_offset_routine`: Pushes a4 onto the stack, loads a word from offset 2 of a0 into d0, calculates an offset based on that word and offset 2 of a0 into a4, calls another routine, moves d0 to d1, restores a4 from the stack, and returns.
- `allocate_and_shift_string`: Calls a library routine to allocate memory, checks if a string in d1 is non-empty, and if so, calculates its length, calls another library routine to allocate a buffer, and then shifts the string into that buffer before calling another library routine.
- `shift_string_and_clear_byte`: Counts non-zero bytes in a string starting at a3, then copies bytes from the string to a destination, shifts the destination pointer, and calls another routine.
- `copy_and_clear_byte`: Copies a block of bytes from one memory location to another using a loop, stores a byte from register d2 into the destination, clears a byte at an offset from a pointer, and performs an indirect jump.
- `restore_registers_and_unlk`: Restores registers d2 through d7 and a2 through a3 from the stack and unlocks register a5 before returning.
- `clear_data_register_d0_4`: Clears the least significant byte of data register zero.
- `call_set_signal`: Library stub: loads its arguments and the exec library base, calls SetSignal and returns its result to the caller.
- `decrement_and_jump_list`: Increments a byte at offset 127 of a structure in a6, decrements a word at offset 14 of a structure in a0, and if non-zero, branches to an earlier instruction; otherwise it clears a long word at offset 40 of the structure in a0, decrements a word at offset 44, and if non-negative, jumps to another routine.
- `decrement_and_branch`: Tests if the value in d0 is negative; if so, it branches to another routine, otherwise it decrements a word at offset 44 of a structure in a0 and branches to another routine.
- `execute_library_calls`: Saves registers, calls a routine at a fixed offset from a library base, calls another routine at a different offset from the same library base, restores registers, and branches to another routine.
- `update_linked_list_node`: Updates a long word in a structure, modifies a pointer in a linked list, and if a bit is clear in a value at offset 8 of a structure, it calls a library routine and then jumps to another library routine.
- `update_linked_list`: Increments a 16-bit value at an offset in a structure, calls a routine, and if the result is non-zero and a specific bit is set, updates a pointer in a linked list structure within a loop.
- `increment_patch_counters`: Increments a byte at offset 295 of a structure pointed to by a6, increments a 16-bit word at offset 44 of a structure in a0, and if the result is non-zero, increments a 16-bit word at offset 14 of the structure in a0 before jumping to an offset relative to a6.
- `process_conditional_message`: Checks a value at an offset in a structure, and if non-zero, clears a bit in a value from a library, calls two routines, and then jumps to a third routine.
- `clear_data_register_zero_3`: Performs a bitwise OR operation on data register zero with zero.
- `call_stack_check_handler`: Performs the same stack-bounds and status-word test as its neighbours.
- `clears_data_register_zero_7`: Performs a bitwise OR operation on the least significant byte of data register zero using a zero constant.
- `validate_stack_bounds_and_call_handler`: Checks if the current stack pointer is within the bounds defined by two values in a structure pointed to by an offset in a6; if the bounds check passes and a specific word in another structure equals negative one, it calls a routine through a global pointer and returns.
- `store_library_base_globals`: Copies the second address register and the base register into two global slots and returns the value one.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `compare_structure_value`: Compares a 32-bit longword at offset 4 of a structure in a1 to the constant 2000, and if they are equal, jumps to the instruction immediately following this routine; otherwise it performs an absolute long jump to zero.
- `check_and_update_value`: Compares a value at an offset in a structure against a constant, and if it matches a specific condition, updates a 16-bit value in that structure and branches back to the start.

### Version

- `parse_command_string`: Parses a string into individual tokens by handling spaces, tabs, and quoted substrings, including converting specific character sequences into newline or escape characters, and stores the resulting pointers in a structure.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `check_library_version`: Parses command-line arguments to open a specified library or device and validates its version and revision numbers against provided thresholds.
- `process_message_reply_2`: Calls a routine to handle message processing by updating message fields and calling various functions based on flags, then calls a library vector to finalize the operation.
- `parse_integer_string`: Parses a string to convert it into an integer, handling optional sign characters and digit sequences.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
- `calculate_string_length`: Calculates the length of a null-terminated string by iterating until the null byte is found.
- `call_reschedule_3`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch_3`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_444_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_552_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `invoke_global_and_verify_checksum`: Calls a function pointer stored in a global variable and computes a rolling checksum over a stack-allocated buffer until a null terminator is found.
- `store_byte_at_pointer`: Stores a byte from register d0 into the memory location pointed to by a6, then increments a6.

### Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:intro.exe

- `initialize_system_and_unpack_data`: Writes two words to a global variable, sets a byte at a specific memory location to 129, and then copies 100 long words from a source table to a destination buffer using a loop.
- `set_stack_and_jump`: Sets the stack pointer to a specific address, clears the status register, and performs an indirect jump using an offset from a register.
- `copy_and_unpack_bitstream`: Copies a block of memory from a source pointer in a1 to a destination pointer in a2, where the block size is determined by a value read from a2, then enters a loop to unpack a variable-length bitstream into a destination buffer using bit-shifting and carry propagation.
- `unpack_variable_length_bitstream`: Reads a series of multi-byte values from a pointer in a2, where each value is processed through a sequence of bit-shifting and carry-propagation logic to determine subsequent lengths or counts, and writes the resulting bytes to the destination pointed to by a0 in a loop.
- `unpack_nested_data_structure`: Reads a series of nested, multi-byte values from a pointer in a2, where each value determines the length or count of subsequent data to be processed through a series of arithmetic expansions.

### d

- `add_byte_to_pointer`: Adds a byte from a register to the value at a pointer and stores the result back at that pointer.

### gb

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:InstallWingCommander

- `initialize_installation_environment`: Allocates a task record, initializes a large memory block with zeros, and determines the stack size based on system information.
- `release_install_task_resources`: Saves an incoming value into a per-task record, then conditionally calls a routine at offset minus 732 if the library version is greater than 35.
- `execute_installation_sequence`: Validates the stack pointer against a threshold, then performs a sequence of file operations including directory creation, file allocation, and data copying.
- `allocate_and_copy_file_data`: Checks if a stack pointer is within a specific range, then allocates memory, copies a sequence of bytes from a source address to the new memory block, and calls several routines to manage the data and memory.
- `search_table_for_matching_short`: Iterates through a table of entries, where each entry is sixteen bytes long, to find a matching sixteen-bit value at offset thirteen of each entry.
- `process_installation_file_table`: Iterates through a table of sixteen-byte entries to find specific file types, then performs file operations including memory allocation, file copying, and directory manipulation based on the entry type.
- `copy_file_buffer_to_memory`: Allocates memory and reads a file into a buffer, then iterates through the data in chunks of up to 8192 bytes to copy it into a destination memory area using a loop.
- `copy_and_parse_path`: Copies a string into a local buffer and calls split_path_into_parts to parse it.
- `ensure_directory_structure`: Checks for the existence of a directory and, if missing, creates it and any necessary parent directories by parsing a path string and inserting forward slash characters into the path components.
- `return_immediately`: The routine executes a return instruction immediately upon entry, performing no other operations despite the instruction count suggesting additional code may be present or filtered.
- `call_external_function_via_null_pointer`: Decrements the stack pointer by 4, attempts to call a function at an address stored in a global variable, and then restores the stack pointer before returning.
- `free_install_memory_lists`: Frees two linked lists of memory blocks by calling the library free routine at offset minus 210 on each node, and clears the head of the first list.
- `split_path_into_components`: Splits a slash-separated path into components by writing a zero byte after each separator and recording where each component starts, for up to 16 components.
- `split_path_into_parts`: Copies a string into a local buffer and parses it to separate the volume, directory, filename, and extension into four provided destination pointers.
- `append_separator_if_missing`: Compares a pointer returned by another routine to a parameter, then if the byte before that pointer is not a slash, colon, or backslash, it copies a byte from offset 1176 of a global structure to that location and calls another routine.
- `copy_string_to_buffer`: The routine copies bytes from a source pointer in A1 to a destination pointer in A0 until a zero byte is encountered, writing each byte to the destination and advancing both pointers.
- `convert_integer_to_string`: Repeatedly calls a fixed-point division routine to extract digits, converting them to ASCII characters.
- `convert_binary_to_string`: The routine converts a binary integer into a string representation by repeatedly extracting bits and appending characters to a buffer, then copies the result to an output pointer and returns the length.
- `convert_hex_to_string`: The routine converts a hexadecimal integer into a null-terminated ASCII string using a lookup table.
- `parse_signed_integer`: The routine reads a signed decimal integer from a zero-terminated string pointed to by a register, handling an optional leading plus or minus sign.
- `allocate_file_handle_record`: Checks a flag bit in a file record and, when it is clear, calls a routine to obtain a buffer and sets the record's fields from a global value.
- `convert_string_to_uppercase`: Converts each letter of a zero-terminated string to upper case in place, using a character-class table to decide which bytes are lower-case letters.
- `write_to_file_record`: Writes data to a file record after checking the record's flags.
- `perform_register_manipulation`: Tests bit 6 of d6 against d0, then loads d0 from an offset of 4 from the stack, loads a0 from an offset of 8 from the stack, and loads d1 from an offset of 12 from the stack.
- `read_and_strip_newlines`: Reads data from a source, strips carriage return characters, and returns the length of the processed data.
- `seek_file_record`: Seeks within a file record to the position given by the mode parameter, storing the DOS error code when the seek fails.
- `close_file_record`: Closes a file record by calling the DOS close routine when it is open, releasing its buffers when they are present, and clearing its handle.
- `open_file_record`: Adds a new file record to a linked list, copies a filename into the record, and translates access mode flags into internal record flags.
- `normalize_fixed_point`: The routine takes two 32-bit inputs and normalizes a fixed-point value by scaling the numerator with a variable shift derived from the denominator’s magnitude, performing a division with remainder and a borrow correction, then returns a 64-bit result composed of a signed quotient and a scaled remainder.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `calculate_file_offset`: Performs a series of signed multiplications and additions using values from registers to compute a 32-bit result, then returns.
- `close_all_file_records`: Sets a 32-bit value to 1 at offset 1200 of a global structure and walks a linked list of records starting at offset 1592 of that structure, calling either a routine at offset 11408 or a routine at offset 11400 based on flags in the record, then calls another routine when the list is exhausted.
- `parse_command_line_and_open_streams`: Splits a command-line string into up to 32 argument strings by handling whitespace, double quotes, and star-escaped characters, then initializes input and output streams using a table of device identifiers.
- `move_and_load_registers`: Moves the contents of d0 to d5 and then loads d0 from an offset of 4 from the stack.
- `process_break_signal`: Copies a string into a stack buffer, checks for a specific task or signal, and updates a global variable based on the result of a library call.
- `find_file_record_by_index`: Returns the file record at the given index in the linked list, walking the list from its head.
- `handle_break_signal`: Calls a routine to process a signal, and if a specific bit is set and a value at an offset in a4 is not 1, it either jumps to another routine or clears a value at an offset in a4 and calls a routine using a pointer stored there.
- `find_byte_in_buffer`: The routine searches for a specific byte value within a memory buffer of a given length.
- `append_string_to_buffer`: Finds the terminating zero byte of a destination buffer in a0 and copies bytes from a source pointer in a1 into that location until a zero byte is copied, then returns the original destination pointer in d0.
- `append_string_to_buffer_2`: Locates the null terminator of a destination buffer and copies bytes from a source pointer into that location until a null byte is encountered, returning the original destination pointer.
- `copy_bytes_with_termination`: Copies bytes from a source pointer in a1 to a destination pointer in a0 until a zero byte is encountered, then fills the remaining space up to a specified count in d0 with zero bytes.
- `copy_byte_block_safe`: Copies a specified number of bytes from a source in a1 to a destination in a0, handling overlapping memory regions by copying backwards if the source address is higher than the destination.
- `load_pointer_to_a0`: Loads a 32-bit value from an offset of 4 from the stack into a0.
- `unlink_file_record`: Searches a linked list of structures for an entry containing a specific pointer and updates the list by removing that entry, adjusting the size and pointers of adjacent elements, and calling a callback function.
- `load_stack_parameter_to_d0`: Loads a 32-bit value from an offset of 4 bytes from the stack pointer into register d0.
- `allocate_memory_block`: Searches a linked list of memory pools for a free block of a specified size and splits the block if it is larger than requested.
- `process_file_records`: Iterates through a linked list of records starting from a global structure, calls a routine for each record, and performs conditional logic based on flags and pointer comparisons before jumping to another routine.
- `append_byte_to_buffer`: Appends a byte to a buffer and keeps its used count in a structure field.
- `call_and_return_flag_status`: Calls a routine with the incoming parameter and a stack slot, then returns minus one when a flag field in a global structure has bit 4 or bit 5 set, and zero otherwise.
- `read_line_into_buffer`: Reads characters into the buffer until a newline or the end of input, then terminates the buffer with a zero byte.
- `read_next_input_byte`: Reads the next byte from an input stream, refilling its buffer through helper calls when it is empty and recording end-of-file or error flags.
- `load_address_to_a0`: Loads a long word from an offset of 4 from the stack into register a0.
- `calculate_file_offset_2`: Calculates a file offset by counting newline characters in a buffer and adjusting the result based on specific flags and a length field.
- `reset_structure_fields`: Clears several fields within a structure, performs bitwise operations on a field based on a flag, and updates a value based on the result of another routine.
- `load_multiple_registers`: Loads four values from the stack into registers d0, d1, a0, and a1 using offsets 8, 12, 16, and 20 respectively.
- `copy_data_to_buffer`: Copies characters from a source to a destination buffer while checking structure flags and handling newlines.
- `copy_string_data`: Copies a sequence of bytes from a source buffer to a destination buffer based on a specified length, updating a counter and internal state flags in a structure.
- `update_file_node_state`: Resets the position and status fields of one file record, or when given no record, checks each record in the list.
- `load_argument_pointer`: Loads a pointer from a location four bytes above the current stack pointer into address register zero.
- `validate_file_record_state`: Checks the flags and contents of a file record structure to return a status code indicating success or failure.
- `open_stream_from_mode_string`: Parses a two-character access mode made of read, write, append and optional plus signs into flag bits, then calls another routine to open the file.
- `find_or_allocate_free_node`: Walks a linked list hanging off a global structure looking for a node whose count field is zero.
- `parse_format_string_and_print`: Parses a format string to identify conversion specifiers, flags, and width or precision arguments, then converts corresponding integer or pointer values from a parameter list into ASCII strings using helper routines.
- `parse_format_string_print`: The routine parses a format string to identify conversion specifiers and flags.
- `delete_file_save_error_code`: Calls the DOS library to delete a file.
- `lookup_value_by_index`: Compares a value in d0 against several constants and, based on the result, fetches a byte from one of three tables located at offsets from a4 and returns it in d0.
- `push_registers_and_call_routine`: Pushes the contents of two registers onto the stack, calls another routine, and then returns.

### Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:Wing

- `initialize_system_memory`: Performs a series of function calls, decrements a counter in a loop, and checks a specific memory pattern to update a global variable.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `expand_and_copy_bitstream_2`: Calls a routine twice to load addresses, then enters a loop that calls another routine to determine a count, calls a routine to expand a bitstream, and adds the resulting values to a table at offset 0 of a structure in a3.
- `load_address_and_constant`: Loads an address into a1 using the program counter and sets d0 to two.
- `expand_and_copy_bitstream`: Reads a bitstream from a source buffer, expanding each bit into a multi-bit sequence based on a lookup table, and writes the resulting bytes into a destination buffer.
- `push_registers_and_branch`: Performs bitwise OR operations on d6 and d0, pushes d0, d1, and d2 onto the stack pointed to by a4, restores several registers from the stack, and branches to another location.
- `perform_logical_or`: Performs a bitwise logical OR on register d0 with zero.

### Wing Commander (1992)(Mindscape)(Disk 1 of 3)[cr Ministry].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics_and_audio_state_2`: Initializes a graphics and audio state structure by copying data from a table, setting specific bitmasks in the hardware registers, and clearing a series of memory locations via a loop.
- `clears_data_register_zero`: Performs a logical OR operation on the least significant byte of data register zero with zero.
- `initialize_graphics_and_audio_state`: Clears several memory tables, initializes a structure with coordinate and transformation data, and performs a series of checks on hardware status registers to synchronize audio and graphics processing.
- `calculate_clipping_rectangle`: Clamps a set of input coordinates against a bounding box, calculates the resulting width and height, and uses these dimensions to compute a memory offset into a table for a sprite or object structure.
- `apply_complex_coordinate_transformations`: Performs four sequential loops that apply a complex 2D rotation and scaling transformation to sets of nine coordinate pairs stored in tables, writing the results to a structure pointed to by a5.
- `initialize_and_process_vertex_data`: Sets two 16-bit values to 88 and a 32-bit value to a negative constant in a structure, then iterates through four separate tables of index pairs to fetch coordinate data and calls clamp_and_interpolate_rectangles for each pair.
- `clamp_and_interpolate_rectangles`: Updates a bounding box in a structure with the minimum and maximum values from four input registers, then performs linear interpolation to clamp these values within a specified range defined by a second structure.
- `calculate_structure_offsets`: Performs arithmetic on registers to calculate a value, then stores several 16-bit and 32-bit values into a structure pointed to by a6 based on conditions and bit tests.
- `increment_and_dispatch_sequence`: Increments a 16-bit value at offset 24 of the structure in a5 and performs a series of conditional checks to either increment various 16-bit words at offsets 34, 36, 38, or 40, update a global variable with a constant, or call other routines including decrement_and_search_table, process_data_sequence, decrement_and_verify_table, decrement_data_sequence, bulk_copy_data_blocks, and dispatch_state_handler_sequence.
- `decrement_and_search_table`: Decrements a 16-bit value at offset 30 of a structure in a5 and, if it reaches zero, iterates through a table to find a value matching specific bit patterns.
- `process_data_sequence`: Decrements a 16-bit word at offset 46 of a structure in a5 and, if the result is zero, sets that word to 2 and calls another routine eleven times using various memory addresses in a0.
- `increment_table_pointer`: Checks a value at a pointer in a0 against three different bitmasks and increments the pointer by 256, 16, or 1 respectively if the bits are not set.
- `decrement_and_verify_table`: Decrements a 16-bit value at offset 30 of a structure in a5 and, if zero, checks a table for specific bit patterns and subtracts from a pointer if they are found.
- `decrement_data_sequence`: Decrements a 16-bit word at offset 46 of a structure in a5 and, if the result is zero, sets that word to 2 and calls another routine eleven times using various memory addresses in a0 while clearing d1.
- `subtract_table_indices`: Checks if a value at a pointer in a0 matches specific bitmasks and subtracts 256, 16, or 1 from that pointer if a match is found, then sets a 16-bit value at offset 32 of a structure in a5 if no matches occurred.
- `bulk_copy_data_blocks`: Uses three loops to copy multiple bytes from a source table to three different destination memory areas based on indices retrieved from a table.
- `dispatch_state_handler_sequence`: Reads a state index from a structure and executes a specific subroutine based on that index, incrementing the index or updating a pointer and counter accordingly.
- `copy_structured_data`: Iterates through a loop to copy a single byte and seven specific offset bytes from a source table to a destination structure pointed to by a0.
- `setup_parameter_block`: If a bit in a status byte is not set, it stores three 32-bit and three 16-bit values into a structure pointed to by a6, derived from a base address in a5.
- `setup_offset_parameter_block`: If a bit in a status byte is not set, it calculates two 32-bit addresses and stores them along with a 32-bit constant and three 16-bit values into a structure in a6.
- `setup_centered_parameter_block`: If a bit in a status byte is not set, it clears a 16-bit value in a6 and stores two 32-bit addresses, a 32-bit constant, and four 16-bit values into a structure in a6.
- `initialize_structure_3500`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 plus 3500 into offset 80, sets offset 76 and 84 to that pointer minus 70, writes a long value to offset 64, writes 118 to offsets 98, 100, and 102, and writes 1611 to offset 88.
- `initialize_structure_7000`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 plus 7000 into offset 80, sets offset 76 and 84 to that pointer minus 70, writes a long value to offset 64, writes 118 to offsets 98, 100, and 102, and writes 1611 to offset 88.
- `initialize_structure_10500`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 plus 10500 into offset 80, sets offset 76 and 84 to that pointer minus 70, writes a long value to offset 64, writes 118 to offsets 98, 100, and 102, and writes 1611 to offset 88.
- `initialize_structure_offset_68`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 into offset 80, sets offset 76 and 84 to that pointer plus 68, writes a long value to offset 64, writes 116 to offsets 98, 100, and 102, and writes 1612 to offset 88.
- `initialize_structure_3500_offset_68`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 plus 3500 into offset 80, sets offset 76 and 84 to that pointer plus 68, writes a long value to offset 64, writes 116 to offsets 98, 100, and 102, and writes 1612 to offset 88.
- `initialize_structure_7000_offset_68`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 plus 7000 into offset 80, sets offset 76 and 84 to that pointer plus 68, writes a long value to offset 64, writes 116 to offsets 98, 100, and 102, and writes 1612 to offset 88.
- `initialize_structure_10500_offset_68`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 plus 10500 into offset 80, sets offset 76 and 84 to that pointer plus 68, writes a long value to offset 64, writes 116 to offsets 98, 100, and 102, and writes 1612 to offset 88.
- `initialize_structure_offset_210`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 into offset 80, sets offset 76 and 84 to that pointer plus 210, writes a long value to offset 64, writes 118 to offsets 98, 100, and 102, and writes 1611 to offset 88.
- `initialize_structure_3500_offset_210`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 plus 3500 into offset 80, sets offset 76 and 84 to that pointer plus 210, writes a long value to offset 64, writes 118 to offsets 98, 100, and 102, and writes 1611 to offset 88.
- `initialize_structure_7000_offset_210`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 plus 7000 into offset 80, sets offset 76 and 84 to that pointer plus 210, writes a long value to offset 64, writes 118 to offsets 98, 100, and 102, and writes 1611 to offset 88.
- `initialize_structure_data`: Checks bit 14 of a byte at offset 2 of a structure in a6, then if clear, writes a 32-bit constant to offset 64, several 16-bit values to offsets 88, 98, 100, and 102, and stores three pointers at offsets 76, 80, and 84 of the structure in a6.
- `update_structure_pointers`: Checks bit 14 of a byte at offset 2 of a structure in a6, then if clear, writes a 32-bit constant to offset 64, several 16-bit values to offsets 88, 100, and 102, and stores two pointers at offsets 80 and 84 of the structure in a6.
- `set_structure_values`: Checks bit 14 of a byte at offset 2 of a structure in a6, then if clear, writes a 32-bit constant to offset 40, several 16-bit values to offsets 58, 64, and 66, and stores two pointers at offsets 50 and 54 of the structure in a6.
- `copy_table_entries`: Calculates an index by incrementing a value at a memory location, masking it, and shifting it, then uses this index to copy twenty 16-bit values from a table to a destination buffer at a specific offset.
- `enable_audio_interrupt_and_wait`: Sets a bit in the interrupt control register, checks a byte at a specific memory location, and calls another routine before returning from an interrupt context.
- `process_audio_and_registers`: Finds the maximum byte value in a list of 128 bytes, uses that value to calculate an offset for a destination buffer, writes 30 32-bit words to a series of memory locations, and clears several bytes and words in memory and the audio registers.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_audio_registers_2`: Updates audio hardware registers by fetching values from a table and writing them to the audio channel registers, then updates a DMA controller and a global counter.
- `check_video_status`: Compares a byte from a hardware register with itself and, if they are not equal, enters a loop that decrements a counter before returning.
- `update_audio_registers`: Checks a byte at a specific address, and if it is not equal to itself, it enters a loop that calls another routine four times with different addresses and pointer values.
- `update_sprite_and_dma_parameters`: Fetches a pointer from a table based on a nibble-encoded index, then updates a structure with new coordinate and property data.
- `update_offset_value`: Checks a 16-bit value at offset 22 of a structure; if non-zero, it adds or subtracts a byte from offset 21 to the 16-bit value at offset 16 depending on a byte at offset 20, then updates a value at offset 6 of a structure in a5.
- `calculate_offset_value`: Calculates a value based on a byte from a structure and a multiplication, then either adds or subtracts this from a 16-bit value at offset 16 of the structure and stores the result at offset 6 of a5, finally updating a byte at offset 25 of the structure.
- `update_sprite_property_and_coordinate`: Processes a nibble-encoded command to modify a structure's coordinate or property data, including clamping values to specific ranges, updating a 16-bit coordinate via addition or subtraction, and modifying hardware register bits via a pointer.
