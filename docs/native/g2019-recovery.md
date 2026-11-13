# Game 2019 (populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip](disk 1 of 2)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02019). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 18 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 761 of 761 named (0 author symbols), 761 of 761 described (42 by rule, 137 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3 of 3 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 19500 of 21347 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 46345 |
| code | 78954 |
| data | 264704 |
| padding | 3789 |
| unused | 507392 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2)[a].adf:run | 2 | 458 | 12 |
| run | 2 | 458 | 12 |
| EndCLI | 2 | 182 | 3 |
| LoadWB | 12 | 535 | 30 |
| Disk-Validator | 1 | 386 | 1 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2)[a].adf:IP.txt | 2 | 24 | 4 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2)[a].adf:CREDITS.EXE | 2 | 167 | 7 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2)[a].adf:patch | 1 | 99 | 3 |
| icon.library | 10 | 1421 | 110 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2)[a].adf:POPULOUS.II | 6 | 17460 | 570 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2)[a].adf:ip_intro | 1 | 132 | 7 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2)[a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EB8) A7=an address (A7)=an address, slid to exec.library -an address): 494 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip](disk 1 of 2)[a]/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2)[a].adf:CREDITS.EXE: Old ProPack (RNC) (xfd) stream found by ancient scan (11888 -> 26856 bytes) but only 0 of 12 sampled windows are in reference RAM: not used
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip](disk 1 of 2)[a]/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2)[a].adf:patch: TetraPack 2.2 (xfd) stream found by ancient scan (392 -> 112 bytes) but only 0 of 1 sampled windows are in reference RAM: not used
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip](disk 1 of 2)[a]/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2)[a].adf:CREDITS.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2)[a].adf:ip_intro

## Copy protection found

- none found

## Functions


### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2)[a].adf:run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: Copies sixteen longs from 536 bytes past the base register into the buffer at the pointer, when the pointer is nonzero.
- `initialize_entity_structure`: Initializes a large data structure by setting multiple fields to zero or specific values, updates an attribute using update_entity_attribute_and_increment, and performs a loop to copy a sequence of bytes from one memory location to another.
- `update_entity_attribute_and_increment`: Fetches a byte from a table using an index stored in a structure, stores it at offset 20 of that structure, and sets offset 24 to the index plus one.
- `update_entity_attribute_from_table`: Calculates a destination offset within a structure in a1 by adding a value from offset 12 and a constant to a base value, then copies a byte from a table in a0 to the calculated location.
- `update_entity_state_and_call_handler`: Updates several fields in a structure pointed to by a1, including setting a byte at an offset calculated from a1, and copies data between structures pointed to by a0 and a1.
- `branch_with_offset`: Adds a long value from an offset of 54 in a6 to 1 and branches to another routine.
- `check_structure_pointer`: Tests a longword at offset 40 of a structure in a1; if non-zero, it branches to another routine, otherwise it branches to a third routine.
- `test_structure_bit`: Tests bit 7 of a longword at offset 48 of a structure in a1; if set, it sets bit 24 of that same longword and branches to another routine, otherwise it branches to a third routine.
- `execute_subroutine_and_update_structure`: Copies a longword from offset 64 to offset 108 of a structure in a1, then calls a routine using parameters loaded from a table in a2, and finally stores the result from d1 into offset 28 of the structure in a1.
- `reset_structure_and_update_pointers`: Clears several fields in a table at a0 based on an index derived from a1, updates a longword in a1, and copies multiple longwords from a1 to a new structure location.
- `update_structure_and_decrement_counters`: Calls several routines using values from a table in a2, updates a longword in a1, and conditionally decrements a value in a table at a0 based on a counter in a1.

### run

- `run_program_entry`: Program entry: it makes a library call, then walks a linked list keeping the largest value it finds.
- `copy_data_to_stack`: If a pointer is non-zero, this routine copies sixteen bytes from a source offset into a destination buffer and then calls another routine.
- `initialize_entity_structure_2`: Initializes a structure by zeroing several fields, updating an attribute via a table lookup, and setting a pointer to a data block.
- `update_entity_state_from_table`: Fetches a byte from a table using a value in a structure as an index, stores it at offset 20 of that structure, and updates a secondary offset at offset 24.
- `update_entity_offset_and_copy_byte`: Calculates two new offset values based on existing fields in a structure, stores them at offsets 22 and 32, and then copies a single byte from a table in a0 to a destination address calculated using the new offsets.
- `update_entity_state_and_call_handler_2`: Updates several fields in a structure pointed to by a1 using values from a0, including setting a byte to 10 and clearing a longword at offset 12.
- `load_offset_and_branch`: Loads the value one into a register, adds a longword from an offset of 84 in the structure pointed to by a1 into that register, and branches to another routine.
- `check_structure_pointer_2`: Tests a longword at offset 40 of a structure in a1; if non-zero, it branches to another routine, otherwise it branches to a third routine.
- `test_structure_bit_2`: Tests bit 7 of a longword at offset 48 of a structure in a1; if set, it sets bit 24 of that same longword and branches to another routine, otherwise it branches to a third routine.
- `execute_external_function_and_update`: Copies a longword from offset 64 to offset 108 of a structure in a1, then calls a routine using parameters loaded from a1 and a2.
- `reset_structure_and_update_counters`: Updates a table entry using values from a structure, calls two routines, and clears several fields in a table at a0 based on an index from a1.
- `update_structure_and_decrement_counters_2`: Performs multiple calls via a5 using parameters from a2, modifies a longword in a1, and decrements a value in a table at a0 if a specific counter in a1 is non-zero.

### EndCLI

- `run_program_entry_2`: Program entry: it makes a library call, then walks a linked list keeping the largest value it finds.
- `copy_data_to_stack_2`: This routine copies sixteen bytes from a memory location to a destination and then executes another routine.
- `modify_table_and_call_routines`: Performs bitwise operations on a value in a2, calls several routines, and conditionally decrements a value in a table at a0.

### LoadWB

- `start_loadwb_program`: Program entry for LoadWB. It calls DOS and exec library functions to get the current directory, open files, allocate memory, find its task, wait on its message port and report an alert on failure.
- `restore_patch_state`: Stores the new parameter in a slot of the task structure, runs cleanup calls when the matching flags are set, and sets fields before each call.
- `load_workbench_icon`: Finds the named resident and reports a failure if it is missing.
- `compare_strings`: Compares two zero-terminated byte strings one byte at a time.
- `build_list_from_table`: Walks a linked table, allocating an 8-byte node for each entry and storing the result of a call on that entry's value in the node.
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
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `pass_params_with_stack_pointer`: Takes two 32-bit parameters and calls one unnamed routine with them plus a pointer to a fixed stack location, then returns.
- `format_buffer_then_forward`: Calls a library function through a global pointer, which the listing attributes to exec RawDoFmt, to fill a 140-byte stack buffer.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compare_memory_blocks`: Compares a byte at an offset of a2 with a constant, and if equal, compares a block of memory starting at a2 with a block starting at a pointer in a1.
- `write_dot_and_spaces`: Writes a period and four spaces into five consecutive bytes starting at offset 17 of the structure at the pointer.
- `compare_against_global_string`: Walks the global string and the string at the pointer together, returning when the global string ends or the characters differ.
- `return_without_operation`: The pseudo-code contains only a return, so the routine performs no observable work.

### Disk-Validator

- `scan_bit_table_entries`: Program entry of the Disk-Validator file with 386 instructions; only part of the pseudo-code is shown.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2)[a].adf:IP.txt

- `call_conditional_library_entry`: Program entry of IP.txt.
- `open_dos_library`: Calls the exec library function OldOpenLibrary with the dos.library name, which the listing shows as referenced text, and stores the returned handle in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2)[a].adf:CREDITS.EXE

- `apply_hunk_relocations_then_jump`: Program entry of CREDITS.EXE that compares a 32-bit header value with AmigaDOS hunk type codes and, when they match, adds a base value to 32-bit words in a relocation hunk, then writes encoded instruction words onto the stack and transfers control to them.
- `read_big_endian_longword`: Reads four consecutive bytes from the address in an address register and combines them into one 32-bit value with the first byte in the highest position.
- `copy_bytes_backwards_by_count`: When its flag routine returns set, it decodes a count from a global table by reading bits returned by another routine, then copies count plus one bytes one at a time from a source pointer to a destination pointer, moving backwards through memory until the counter reaches minus one.
- `return_immediately`: The routine executes five instructions and performs no observable work before returning to the caller.
- `call_until_result_zero`: Calls a routine once and uses its short result plus one as an index into a global byte table.
- `dispatch_mode_call_loops`: Checks whether a 16-bit input register equals two.
- `spin_until_counter_zero`: Decrements the 32-bit value in a data register until it reaches zero and then returns.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2)[a].adf:patch

- `access_palette_register_entry`: Program entry of the patch file.
- `fetch_longword_store_low_byte`: Reads the 32-bit value stored four bytes before the address in an address register and writes a 16-bit word, built from the low byte of that value and the high byte of a data register, to the location in a second address register.
- `shift_bits_with_refill`: Shifts a 32-bit value in a data register right by one bit per iteration for a count held in another register.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code in libs/icon.library.
- `decrement_library_refcount`: Library code in libs/icon.library that decrements a 16-bit reference counter at a fixed offset from the structure pointer.
- `unlink_node_or_mark_pending`: Library code in libs/icon.library that removes the node from a doubly linked list by joining its previous and next nodes and calls a library function when a 16-bit field is zero.
- `set_flag_bit_return_zero`: Library code in libs/icon.library.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `call_library_vector_or_alert`: Library code in libs/icon.library that stores two register values in the structure, calls a library vector at offset minus 408 and records the result.
- `copy_two_values_to_globals`: Library code in libs/icon.library.
- `call_handler_then_return`: Library code in libs/icon.library that only calls one other routine and returns.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `call_icon_structure_processor`: This library routine unconditionally calls a helper routine to process icon structure fields and then returns.
- `call_library_handler`: This library routine simply calls another routine and then returns.
- `call_library_handler_2`: This library routine calls another routine and then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `trigger_icon_initialization`: This library routine calls a single initialization function to set up icon data structures and then returns.
- `call_handler_pointer`: This library routine fetches a pointer from a data structure and calls the routine located at that address.
- `call_handler_and_return`: This library routine increments a parameter and then calls another routine.
- `call_other_routine`: The routine calls another routine and then returns.
- `call_field_matcher`: This library routine calls a routine that checks for delimited field matches and then returns.
- `call_prefixed_name_builder`: This routine calls a routine that builds a prefixed name and then returns.
- `store_value_in_returned_block`: Library code in libs/icon.library.
- `fill_record_from_stack_buffer`: Library code in libs/icon.library that obtains a structure pointer, then fills a stack buffer through a second routine.
- `copy_stack_fields_to_record`: Library code in libs/icon.library that copies eleven 32-bit values and several smaller fields from a stack area into the structure pointed to by address register two.
- `return_zero_stub`: Library code in libs/icon.library whose body only returns the constant zero.
- `build_stack_header_and_call`: Library code in libs/icon.library that copies eleven 32-bit values from offset 96 of its third parameter's structure into a stack array, copies several more fields into stack slots and calls another routine with a pointer to that stack header, whose first two words are fixed constants.
- `load_icon_info_file`: Library code in libs/icon.library that loads an icon file identified by the text .info.
- `parse_icon_header_fields`: Library code in libs/icon.library that checks the same magic value and version, clears two fields and fills optional sub-structures through library calls.
- `read_optional_icon_entries`: Library code in libs/icon.library that stores two library-call results in the structure and reads entries for several optional fields, allocating storage where a field is empty.
- `read_counted_longword_list`: Library code in libs/icon.library that repeatedly reads a value through a library call, stores it in an array and steps a stack counter down by four until it reaches four or less.
- `call_routine_return_zero`: Library code in libs/icon.library that calls one other routine, ignores its result and returns the constant zero.
- `return_without_operation_2`: Library code in libs/icon.library whose pseudo-code only returns, so it does no work.
- `process_icon_structure_fields`: Library code in libs/icon.library that allocates a 260-byte work buffer through a library call and returns zero after an error call if that fails.
- `process_icon_structure_entries`: Library code in libs/icon.library that uses the text .info and a library vector at offset minus 30.
- `process_icon_pointer_lists`: Library code in libs/icon.library that processes two optional pointer lists stored in the structure, calling a library vector for each entry and storing a size in a stack slot.
- `iterate_null_terminated_pointer_list`: Library code in libs/icon.library that walks a zero-terminated list of pointers, adding four to a stack counter per entry and calling a library vector with the total.
- `iterate_icon_table_entries`: Library code in libs/icon.library that walks a null-terminated table of pointers, calling a routine on each entry and storing its size plus one in a stack slot.
- `walk_node_list_until_end`: Library code in libs/icon.library that repeatedly calls a routine on the address two bytes into the structure and passes each non-zero result to another routine.
- `iterate_linked_list_nodes`: Library code in libs/icon.library that calls a routine on the current node and then a routine that returns the next node, stopping at zero or when the link at offset 8 points back to the node.
- `increment_param_and_recurse`: Library code in libs/icon.library.
- `allocate_and_initialize_block`: Library code in libs/icon.library that requests a 168-byte block with the flag value 65536 and calls routines on the block's second part and on the block.
- `call_routine_then_return`: Library code in libs/icon.library.
- `return_zero_stub_2`: Library code in libs/icon.library whose body only returns the constant zero.
- `append_pair_to_entry_array`: Library code in libs/icon.library that decrements a 16-bit counter in the structure and first calls another routine to grow the array when the counter was zero or negative.
- `decrement_counter_and_store_pair`: Library code in libs/icon.library.
- `allocate_and_prepare_record`: Library code in libs/icon.library that requests a 94-byte block with the flag value 65536 and calls routines on the block's second part and the block.
- `return_without_operation_3`: Library code in libs/icon.library whose pseudo-code only returns, so it does no work.
- `call_routine_and_return_zero`: Library code in libs/icon.library that calls the library function at offset minus 210 from the library base, ignores its result and returns zero.
- `increment_param_and_call`: Library code in libs/icon.library.
- `find_value_after_equals`: Library code in libs/icon.library that walks a zero-terminated list of pointers, calling a routine on each entry until it returns zero and the character at a computed position is an equals sign.
- `find_equals_value_in_list`: Library code in libs/icon.library that checks a string through a routine and looks for an equals sign at a computed position, moving to the next pointer in a list when the check fails.
- `scan_string_list_for_equals`: Library code in libs/icon.library that walks a zero-terminated list of string pointers, calling a routine on each and searching for an equals sign at a computed position.
- `check_delimited_field_matches`: Library code in libs/icon.library that searches the string for the character code 124, measuring each field and comparing its length with a value from an earlier routine.
- `scan_delimited_fields_for_match`: Library code in libs/icon.library that moves through fields separated by the character code 124, measuring each and comparing its length with a value from an earlier routine.
- `match_field_length_in_list`: Library code in libs/icon.library.
- `test_delimited_fields_for_match`: Library code in libs/icon.library that loops over fields delimited by the character code 124, comparing each field length with a value from an earlier routine.
- `scan_fields_restarting_from_start`: Library code in libs/icon.library that restarts a scan of character-code-124 delimited fields from the start of the string on each pass, comparing each length with a value from an earlier routine.
- `build_copy_prefixed_name`: Library code in libs/icon.library that clears a flag byte in the structure and checks whether the string begins with the text copy and a space.
- `parse_decimal_number_field`: Library code in libs/icon.library that checks three characters of text at offset 5 of the structure and converts the decimal digits that follow into a number.
- `parse_decimal_digits_string`: Library code in libs/icon.library that converts a run of decimal digit characters at its pointer into a number.
- `parse_decimal_digits_copy_text`: Library code in libs/icon.library that converts a run of decimal digit characters into a number and checks a four-character text when the number is non-zero.
- `convert_decimal_digit_char`: Library code in libs/icon.library.
- `return_minus_one_stub`: Library code in libs/icon.library whose body only returns the constant minus one.
- `call_library_vector_without_args`: Library code in libs/icon.library that loads a library base from a global variable, calls the library function at offset minus 198 from it and returns.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `chain_two_steps_with_cleanup_2`: Library code in libs/icon.library that calls a routine with its last two parameters and returns zero if that result is zero.
- `allocate_record_and_set_length`: Library code in libs/icon.library that requests a 96-byte block with the flag value 65536 through a routine.
- `return_zero_stub_3`: Library code in libs/icon.library whose body only returns zero.
- `check_result_equals_last_argument`: Library code in libs/icon.library that calls another routine with its last three arguments and returns that result only when it equals the last argument, otherwise zero.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `check_result_equals_last_argument_2`: Library code in libs/icon.library that calls another routine with its last three arguments and returns that result only when it equals the last argument, otherwise zero.
- `return_without_operation_4`: Library code in libs/icon.library whose pseudo-code only returns, so it does no work.
- `divide_32bit_unsigned`: Library code in libs/icon.library that divides a 32-bit dividend in one data register by a divisor in another and leaves the quotient and remainder in registers.
- `divide_shift_subtract_loop`: Library code in libs/icon.library that runs a 16-step shift-and-subtract loop on a 32-bit value in a data register, using the divisor in another register.
- `multiply_32_bit_integers`: Library code in libs/icon.library.
- `forward_call_to_helper`: Library code that calls one helper and returns, with no other work.
- `call_helper_return_register`: Library code that pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `negate_result_on_flag`: Library code that handles the signs of two values, calls a helper, and negates the result when the flag the helper leaves in an address register is nonzero before returning.
- `call_and_negate_on_flag`: Library code that calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `invoke_global_table_entry`: Library code in libs/icon.library.
- `store_byte_to_pointer`: Library code in libs/icon.library.
- `measure_string_length`: Library code in libs/icon.library that counts the characters of a null-terminated string with a 16-bit counter and returns the count.
- `find_character_in_string`: Library code in libs/icon.library.
- `return_zero_stub_4`: Library code in libs/icon.library whose pseudo-code only returns zero.
- `copy_c_string`: Library code in libs/icon.library.
- `append_c_string`: Library code in libs/icon.library.
- `compare_strings_within_limit`: Library code in libs/icon.library that compares two strings character by character for at most the count in its third argument.
- `return_one_stub`: Library code in libs/icon.library whose pseudo-code only returns one.
- `walk_string_then_call_end`: Library code in libs/icon.library that walks a string for at most the count in its third argument.
- `copy_string_with_zero_padding_2`: This library routine calls a routine to copy a string with zero padding.
- `copy_string_with_zero_padding`: Library code in libs/icon.library.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain`: Library code in libs/icon.library.
- `process_icon_request`: Library code in libs/icon.library that returns one when its third argument is zero.
- `process_linked_entries_until_end`: Library code in libs/icon.library that walks a linked list through the pointer at offset 12 of each node.
- `walk_nodes_with_two_checks`: Library code in libs/icon.library that walks nodes linked through offset 12, calling two routines per node and returning zero when either fails.
- `return_zero_stub_5`: Library code in libs/icon.library whose pseudo-code only returns zero.
- `allocate_entry_with_buffer`: Library code in libs/icon.library that, when its fourth argument is non-zero, allocates a 20-byte cleared block and calls two routines on it, then allocates a sized buffer and stores its pointer at offset 10 of the block.
- `return_minus_one_stub_2`: Library code in libs/icon.library whose pseudo-code only returns minus one.
- `allocate_linked_entries_list`: Library code in libs/icon.library that, while its count argument is non-zero, allocates a 16-byte cleared block, links each new block to the previous one at offset 12 and stores further data for each block.
- `allocate_chain_entries_until_end`: Library code in libs/icon.library that repeatedly allocates blocks and links each to the previous one at offset 12, storing a size at offset 8.
- `allocate_entry_chain_from_stack`: Library code in libs/icon.library that loops allocating entries through routines, linking each to the previous entry at offset 12 and storing a size at offset 8 while taking the next entry from a stack argument.
- `return_minus_one_stub_3`: Library code in libs/icon.library whose pseudo-code only returns minus one.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2)[a].adf:POPULOUS.II

- `initialize_program_environment`: This routine initializes several global variables and library pointers, performs conditional logic based on library return values, and manipulates memory contents.
- `store_pointer_and_call_vectors`: Routine in POPULOUS.II. It stores its address-register base plus 92 into a global variable, then calls two library vectors at offsets minus 384 and minus 372 with no arguments, and returns.
- `call_two_library_vectors`: Routine in POPULOUS.II that calls the library function at offset minus 132 and then the one at offset minus 378 from the library base, passing no arguments.
- `clear_color_registers_and_start`: Program routine in POPULOUS.II that writes zero to 32 consecutive 16-bit custom chip color registers, then calls many unnamed routines, including the copper list start and the Workbench and screen close stubs.
- `wait_for_task_pointer`: Program routine in POPULOUS.II that clears a flag and calls the exec FindTask library function until it returns a non-zero task pointer.
- `enable_dma_and_interrupts`: Program routine in POPULOUS.II that copies several saved values back into globals and writes two values with bit 15 set to the DMA control and interrupt enable hardware registers.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `swap_two_globals`: Program routine in POPULOUS.II. It exchanges one global variable with two other global variables, using a temporary value, and returns.
- `install_interrupt_handler`: Program routine in POPULOUS.II that saves the current value of a global variable, writes a fixed address and then a routine address into globals, and writes a value with the enable bit set to the interrupt enable register.
- `handle_mouse_interrupt`: Checks a bit in the interrupt request register, calls a mouse handler, increments several global counters, and writes a value to the interrupt request register.
- `write_interrupt_request_value`: Program routine in POPULOUS.II that writes the value 32 to the interrupt request hardware register and returns.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `setup_screen_buffer_pointers`: Program routine in POPULOUS.II. It makes three calls to unnamed routines, stores the address of a global buffer into one global variable, stores an address-register value plus eight into two further globals, and stores a constant into another global.
- `initialize_copper_dma_table`: Program routine in POPULOUS.II, 82 instructions, that fills a memory table with copper list words and address words, with colour register values chosen by a flag, and stores pointers to parts of it in globals.
- `set_interrupt_enable_flag`: Program routine in POPULOUS.II. It saves the current value of a global variable in a temporary location, writes the address of a routine into that global, and then writes the constant 49160, which has the enable bit set, to the interrupt enable hardware register.
- `swap_and_set_interrupt_enable`: Program routine in POPULOUS.II. It exchanges a global variable with another global, then writes the constant 49160, which has the enable bit set, to the interrupt enable hardware register and returns.
- `process_keyboard_serial_byte`: Program routine in POPULOUS.II that reads the keyboard serial data register through a global.
- `clear_byte_and_decrement_counter`: Program routine in POPULOUS.II. It writes a zero byte to an address formed from an address register and a 16-bit offset, then decrements a global counter while it is positive, and returns its stacked argument value.
- `open_graphics_intuition_libraries`: Program routine in POPULOUS.II that opens libraries through the exec OpenLibrary vector at offset minus 552, with the text graphics.library and intuition.library referenced.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `swap_screen_buffers_2`: Program routine in POPULOUS.II that exchanges two global pointers, toggles bit two of a global index, loads a global from a table at that index, sets another global to that value minus four, and clears a counter.
- `wait_until_counter_nonzero_2`: Program routine in POPULOUS.II. It spins until a global counter becomes non-zero and then returns.
- `update_mouse_position_and_buttons`: Program routine in POPULOUS.II that reads the mouse movement register and converts its low and high bytes into signed deltas with wraparound.
- `poll_right_middle_buttons`: Program routine in POPULOUS.II that clears two counters, writes the constant 3072 to the potentiometer output register and checks bit 1024 of the input register.
- `reset_two_counters_and_call`: Program routine in POPULOUS.II. It clears two global counters, calls one unnamed routine, and returns.
- `read_mouse_movement`: Program routine in POPULOUS.II that stores the low byte of the mouse movement register in one global and the high byte in another, then returns its incoming register values.
- `store_address_halves_and_flags`: Program routine in POPULOUS.II. It writes the low and high halves of a table address into four 16-bit fields of a structure, stores one byte value into two table positions, computes a flag byte from a few bit tests and stores it into two further table positions.
- `set_global_var_pointers`: Checks a global variable against an input word and, if different, stores the address of a global variable plus the input word into two separate structures, writing both the low and high parts of that address into fields of both structures.
- `set_registers_and_call_routine_3`: Routine in POPULOUS.II with the same body as its neighbours: it writes the constants 2048, 1200 and 380 to globals, stores a routine address and calls it, then masks one global with 63 and writes 34816.
- `swap_handler_pointer`: Program routine in POPULOUS.II that exchanges a global variable with another stored global, then writes the constant 34816, which has the enable bit set, to the interrupt enable hardware register.
- `set_global_from_quotient`: Program routine in POPULOUS.II that selects one of two constants depending on whether a global flag is non-zero, divides it by a global divisor, subtracts one and stores the result as a 16-bit value in a global.
- `abs_diff_with_optional_call`: Program routine in POPULOUS.II that computes the absolute difference between two 16-bit globals.
- `reset_two_globals`: Program routine in POPULOUS.II. It stores zero into two global variables and returns.
- `append_byte_to_ring_buffer`: Program routine in POPULOUS.II that, when a flag bit is set, stores the low byte of a global into a ring buffer at the current index, wraps the index at a stored limit and advances it.
- `return_stacked_argument`: Program routine in POPULOUS.II whose pseudo-code returns its stacked argument unchanged.
- `return_without_operation_5`: Program routine in POPULOUS.II whose pseudo-code only returns, so it does no work.
- `return_from_exception`: This routine executes a single return from exception instruction.
- `handle_interrupt_status`: Pushes a word from the stack to the stack, reads a long value from a hardware register, masks it with a bitwise AND, and if the result is zero, clears a long value in a global variable and writes the masked value to a hardware register before performing an interrupt return; otherwise it pops the word from the stack and performs an interrupt return.
- `pop_stack_and_return`: Pops a word from the stack into d0, pushes a long value from a global variable onto the stack, and returns.
- `read_ring_buffer_bytes`: Program routine in POPULOUS.II that copies bytes from a ring buffer into a destination buffer, wrapping the read index at a stored limit, until a count is exhausted.
- `copy_ring_buffer_with_limit`: Program routine in POPULOUS.II that copies bytes out of a two-field ring buffer structure into a destination, advancing and wrapping its read index at a stored limit.
- `copy_ring_buffer_count_bytes`: Program routine in POPULOUS.II that decrements a count and then copies bytes out of a ring buffer into a destination, advancing and wrapping the read index.
- `return_stack_value`: Returns the value currently on the stack.
- `write_bytes_when_ready`: Program routine in POPULOUS.II that, when bit 8192 of a global is set, writes the next source byte into that global, sets a flag and decrements its count, returning when the count reaches minus one.
- `write_serial_data_bytes`: Program routine in POPULOUS.II that, when bit 8192 of a global is set, writes the next source byte into the serial data register and sets a flag in a structure.
- `call_close_work_bench`: Library stub: loads its arguments and the intuition library base, calls CloseWorkBench and returns its result to the caller.
- `set_screen_parameters_and_open`: Program routine in POPULOUS.II that clears a global flag and writes constants into a block of globals, including 20, 320 and 11, then stores the address of a global variable.
- `call_close_screen`: Library stub: loads its arguments and the intuition library base, calls CloseScreen and returns its result to the caller.
- `call_library_vector_twice`: Program routine in POPULOUS.II that calls the function at offset minus 414 from a global base twice in succession and returns its two input register values.
- `return_immediately_2`: This routine is empty and performs no operations before returning to the caller.
- `reset_task_priority_and_blitter`: If a global variable is non-zero, it sets the task priority to zero and disowns the blitter, then clears that global variable before returning.
- `nop_routine`: This routine is empty and performs no operations.
- `wait_for_blit_set_priority`: Program routine in POPULOUS.II that calls one unnamed routine repeatedly until it returns zero while clearing a flag, then calls library vectors through stored base pointers.
- `call_two_library_vectors_2`: Program routine in POPULOUS.II that calls the function at offset minus 132 from a global base and the one at offset minus 456 from a stored base, then returns its two input register values.
- `call_two_library_vectors_3`: Program routine in POPULOUS.II that calls the function at offset minus 138 from a global base and the one at offset minus 462 from a stored base, then returns its two input register values.
- `poll_input_and_update_dma`: Program routine in POPULOUS.II, 603 instructions, with its pseudo-code truncated in the listing.
- `dispatch_handler_by_index_2`: Program routine in POPULOUS.II with the same body as its neighbour: when two flags are set it calls one unnamed routine, then calls a function from a jump table selected by a 16-bit global index.
- `invoke_sequence_of_callbacks`: Program routine in POPULOUS.II that loops four times, computing a table index from two globals and choosing a function pointer from a table entry according to a byte.
- `process_pending_entry_list`: Program routine in POPULOUS.II that returns early when a flag is set.
- `merge_bitplane_rows_from_table`: Program routine in POPULOUS.II, 164 instructions, with the pseudo-code truncated in the listing.
- `select_cell_from_coordinates`: Program routine in POPULOUS.II that derives a cell index from two input coordinates by subtracting offsets and shifting.
- `check_and_invoke_handler`: Program routine in POPULOUS.II that computes a table index from an input register and a global, and when a flag and counter allow it checks a table byte.
- `find_matching_table_entry`: Program routine in POPULOUS.II that returns zero when a flag is clear.
- `scan_table_for_matching_entry`: Program routine in POPULOUS.II that steps through table entries in increments of 32 from its input index and returns one when the scan wraps to a stored start.
- `return_zero_stub_6`: Program routine in POPULOUS.II whose pseudo-code only returns zero.
- `update_offsets_from_record`: Program routine in POPULOUS.II that, when a stored record pointer is non-zero, reads the bytes at offsets 6 and 8, subtracts four from each and stores them in two globals.
- `find_flagged_record_and_load`: Program routine in POPULOUS.II that returns zero when a flag is clear.
- `scan_flagged_records_from_index`: Program routine in POPULOUS.II that steps through table entries in steps of 52 bytes from its input index, looking for a register-matching first byte with flag bit two set.
- `return_zero`: This routine is empty and simply returns a value of zero to the caller.
- `store_pointer_and_arm_counter`: Program routine in POPULOUS.II that, when the stored pointer is non-zero, saves it into a second global if a counter is set and sets the counter to 100.
- `check_entry_type_and_call`: Program routine in POPULOUS.II. It calls one unnamed routine when a global equals 12, then compares a 16-bit field of a table entry with the values 16, 14, 20 and 18, calling one unnamed routine for each match, and finally calls another unnamed routine.
- `xor_bytes_into_rows`: Program routine in POPULOUS.II that XORs source bytes into a destination area, advancing 40 bytes per row for up to twelve rows.
- `decode_escape_coded_text`: Program routine in POPULOUS.II, 60 instructions, that copies a zero-terminated byte stream into a buffer.
- `call_routine_twice`: Program routine in POPULOUS.II. It calls one unnamed routine twice in succession and returns.
- `call_routine_then_return_2`: Routine in POPULOUS.II. Its body makes one call to another routine and then returns.
- `call_other_routine_2`: Calls another routine twice before returning.
- `call_routine_then_return_3`: Program routine in POPULOUS.II. Its body makes one call to another routine and then returns.
- `write_high_nibble_hex_digit`: Program routine in POPULOUS.II that calls an unnamed routine to obtain a byte, swaps its nibbles, keeps the low four bits and stores the result at the address in an address register as an uppercase digit character.
- `write_high_nibble_hex_digit_2`: Program routine in POPULOUS.II that takes the byte in its data register, swaps its nibbles, keeps the low four bits and stores the result at the address in an address register as an uppercase digit character.
- `interpret_text_control_codes`: Program routine in POPULOUS.II, 78 instructions, that copies a text stream character by character.
- `apply_masked_block_to_planes`: Program routine in POPULOUS.II that shifts a five-row block of bytes from a table by a bit offset and combines it into four planes spaced 8000 bytes apart, with a 40-byte row stride.
- `return_from_subroutine_2`: This is an empty routine that performs no operations and immediately returns to its caller.
- `poll_video_position_and_dispatch`: Program routine in POPULOUS.II, 59 instructions, that reads the vertical position register, makes about fourteen calls to unnamed routines and updates stored position values.
- `process_glyph_and_interpolate`: Increments a byte at an offset from a pointer and clamps it between 48 and 57, calculates a value based on a global variable and the byte, then calls three different routines to interpolate tables, copy word blocks, and call library vectors.
- `call_routine_then_return_4`: Routine in POPULOUS.II whose body makes one call to another routine and then returns.
- `dispatch_through_handler_table`: Calls five routines in sequence, one of them only when a flag word is zero.
- `dispatch_selected_handler_entry`: Makes three calls, then calls a routine that returns an index and uses it to jump through a table of offsets to a handler.
- `rewrite_marked_string_byte`: Calls several routines, then either calls a handler through a pointer or clears one byte and stores a fixed value, depending on a constant comparison.
- `skip_past_nul_terminator`: Advances a byte pointer one byte at a time until it has passed a zero byte, then returns.
- `empty_routine`: The routine contains four instructions and performs no observable work.
- `update_game_day_and_status`: Calculates and clamps a value representing game days or time progression, then updates a global status string and performs a calculation involving a timer and a multiplier.
- `reset_buffer_flags_and_dispatch`: Calls three routines, sets a word variable to 40, clears one byte in a buffer and a second word variable, then calls a routine and optionally another one.
- `shuffle_yz_text_marks`: Repeatedly rewrites the y and z letters of a zero-terminated text buffer according to the low bits of a shifting word.
- `repeat_yz_text_marks`: Loops over four calls and the same bounds test; while the position is inside bounds it clears a flag and rewrites the y and z letters of a text buffer from the bits of a word.
- `clear_flag_and_dispatch`: Sets a global flag byte to zero and then calls a routine via an indirect jump using an offset from a global variable.
- `call_handlers_from_table`: Walks a global short table and a byte buffer in pairs.
- `fill_string_pointer_table`: Zeroes a table of twelve long-word slots, then fills successive slots with the addresses of consecutive zero-terminated strings from a text block.
- `update_player_mode_text_markers`: Selects pointer variables from a mode value, then scans a zero-terminated text buffer and rewrites the matching byte markers according to the mode and a state flag.
- `rewrite_text_markers_from_mask`: Sets a flag word and a bitmask from stored values, then scans a zero-terminated text buffer.
- `rewrite_text_markers_entry`: Enters the text-marker rewrite loop with the mask already set.
- `repeat_call_then_dispatch`: Calls a routine and reads a word result in a loop, repeating while a stored level value equals 2 and the result is at least 3 and below a limit.
- `process_character_input`: Performs case conversion on a character and shifts a string buffer to insert the new character.
- `append_char_from_entry`: Enters the character-append logic directly with a buffer pointer and index passed in registers, skipping the table search.
- `find_v_w_delimiter_2`: Searches backward from a pointer to find the character 'v' and then searches forward from that position to find the character 'w'.
- `edit_text_line_from_input`: Measures a zero-terminated text buffer, then loops reading keys.
- `edit_text_line_from_registers`: Runs the same key-editing loop as the line editor, starting from a cursor position and buffer pointer passed in registers.
- `process_text_line_keys`: Runs the key-editing loop with an inner repeat that moves the cursor back while a flag is set.
- `resume_text_line_editing`: Continues the line-editing loop from a point after the key read, checking first for a return key that ends the edit with zero.
- `handle_text_line_edit_keys`: Handles key edits on a zero-terminated line in a loop that reads a key and branches on return, backspace or an inserted character.
- `call_routine_return_zero_2`: Calls one other routine and then returns zero as its result.
- `copy_padded_text_with_marker`: Copies a fixed number of bytes from a source string into a buffer, padding with the letter k once the source ends and writing the letter m at one index when a flag bit is set.
- `clear_flag_and_call`: Sets one global flag byte to zero and then calls another routine.
- `toggle_last_match_and_count`: When a flag is set and a computed position lies inside a bounds window, it scans bytes from that position and adds two to a count for each character that matches a marked set.
- `expand_template_slots`: Copies a template string into a global buffer, replacing each left-brace slot with the next string from an argument list, or a space when that argument is empty.
- `insert_letter_into_entry_field`: Accepts a space, a backspace, a return or an uppercase letter and applies it to one of two ten-character text fields chosen by a flag.
- `shift_buffer_and_append_char`: Shifts a text buffer one position toward its start, then either clears the byte before the end on a backspace or stores a printable character at the end, terminating the string.
- `store_printable_char_with_terminator`: If the byte held in a register is printable (above 31), writes it one position before a pointer and writes a zero at the pointer.
- `decrement_counter_and_call`: If a counter is positive, it decrements the counter, then calls another routine up to twice depending on whether a flag byte is non-zero.
- `call_routine_twice_2`: Calls another routine twice in a row and then returns.
- `copy_glyph_bitmap_rows`: Walks a zero-terminated string, maps each character to a 32-byte entry in a global table and writes that entry into a plane-separated destination buffer one column at a time.
- `process_flagged_tile_entry`: If a table entry at the given index is non-zero and a byte from a second table is positive, it sets two globals, calls several routines and invokes two handlers through pointer tables.
- `increment_and_dispatch_table_value`: Increments a long value in a global variable, reads that value into a register, masks it with 3, uses the result to fetch a byte from a table, masks that byte with 255, and branches to one of several locations.
- `execute_blitter_plane_copies`: Increments a global word, uses the result to select a value from a table, and then enters a loop that calls a blitter routine three times for each entry in a table until a specific value is encountered.
- `branch_to_subsequent_code`: Returns immediately, otherwise it branches to another routine.
- `modify_and_branch_value`: Masks a long value in a global variable with 1, moves that value to a register, masks it with 1, adds 143 to the register, and branches to one of two locations.
- `update_counter_and_dispatch`: Updates a global word value by adding 4, potentially adjusting it based on a value in a table, and then calls another routine to increment a counter and dispatch handlers.
- `store_five_string_pointers_2`: Skips a row-dependent number of zero-terminated strings in a text block, then stores the addresses of the next five strings into a pointer table.
- `reset_buffer_and_process`: Clears several global variables and a pointer, then loops calling routines to process a buffer, toggling a flag and advancing an index on each pass, until the loop-exit flag is set.
- `process_packed_data_buffer`: Decodes a stream of packed 16-bit words by splitting them into 6-bit fields and passing the resulting values to a handler.
- `decode_packed_entry_buffer_2`: Runs the packed-buffer decoding without the initial counter check, splitting each 16-bit word into two 6-bit fields that are passed to a routine with a constant offset.
- `invoke_table_handlers_in_loop`: Calls a routine with two values from a table and invokes a function pointer loaded from a second table.
- `copy_tile_map_block`: Copies twelve bytes per row from a source buffer into a destination spread across planes 8000 bytes apart, advancing 40 bytes per row for 96 rows.
- `unpack_settings_into_buffers`: Reads a settings record, clamps one count to at most 8 to pick a pointer, and packs two pairs of values into the nibbles of two bytes.
- `copy_and_dispatch_data`: Copies blocks of data from a source table to a destination buffer and then executes a series of function pointers retrieved from a table.
- `iterate_table_dispatch`: Runs three passes; each reads a short value from a table and a byte from a buffer, combines them into an index into a pointer table, and calls the routine found there with the pass counter.
- `call_routine_in_nested_loops`: Runs a nested loop of two outer and three inner iterations, calling another routine twice on each inner pass.
- `copy_rows_into_planes`: Copies rows of four words from a source buffer into four plane offsets of a destination, advancing the destination 20 words per row across 32 rows.
- `copy_words_into_planes`: Copies four words per iteration from a source table into four plane offsets of a destination, advancing the destination 20 words per iteration until a counter runs out.
- `program_blitter_grid_steps`: Sets blitter masks, modulo and control registers, then waits for the blitter busy flag in the DMA status register to clear before programming blitter pointers and size for each step.
- `start_blitter_plane_copies`: Starts up to two blitter copies, each only when its table entry is non-zero, waiting for the busy flag before writing source pointers, offsets and size for each plane.
- `setup_blitter_parameters`: Looks up pointer and offset values for an entry and clips them against the left and top edges by adjusting the destination and source offsets.
- `setup_blitter_sequence`: Calls three subroutines, sets the blitter window and mode, and then iterates through a table to conditionally call a routine and eventually configure blitter hardware registers before calling two library vectors.
- `adjust_pointers_and_call`: Increments two registers by 2, calls another routine, decrements those same two registers by 2, and returns.
- `execute_blitter_transfers`: Calculates a size and offset, then performs up to four blitter transfers to different memory areas by writing to hardware registers and waiting for the DMA controller to finish.
- `blit_sprite_tiles_to_memory`: Checks if a value in a6 is within a specific range, then reads a sequence of offsets from a table to perform multiple blitter transfers.
- `write_blitter_row_pointers`: Writes constant blitter mask, modulo and control values, waits for the busy flag to clear, then writes eight row pointers from a table into the blitter destination registers, waiting for idle after each write.
- `set_blitter_control_words`: Waits for the blitter busy flag to clear, then writes fixed mask, modulo and control constants along with two stored values to the blitter registers and calls another routine.
- `clear_table_and_iterate`: Zeroes 4096 long words starting at a global block, then walks a short table calling three routines per entry until it reaches the sentinel value minus 99.
- `generate_random_sequence`: Advances two 15-bit linear congruential states and calls a routine with the first state value.
- `update_tile_flags`: Increments a global counter and, while it is non-negative and the row index is not eight, checks neighbouring tile entries with conditional routine calls.
- `update_tile_flags_reverse`: Increments a global counter and returns early when it is negative or the row index is below one.
- `update_neighbor_tile_counters`: Decrements a three-bit counter for a tile record and masks its flag byte.
- `update_neighbor_tile_counters_2`: Decrements a three-bit counter for a tile record and masks its flag byte.
- `update_single_neighbor_tile`: Decrements a three-bit counter for a tile record and masks its flag byte.
- `decrement_tile_counter_byte`: Decrements a byte counter, wraps it modulo eight, clears the low flag bit of the following byte and increments a global counter.
- `return_stack_value_2`: Returns a value stored on the stack to the caller.
- `check_tile_within_grid`: Tests two signed coordinates against the 64-by-64 grid bounds, branching on the result without changing them, and returns both values unchanged.
- `check_neighbor_tiles_for_marker`: Checks the four neighbouring tiles of a grid position that lie inside the 64-by-64 grid.
- `return_stacked_arguments`: Returns two words taken from the stack as its result and performs no other work.
- `check_three_neighbor_tiles`: Checks up to three neighbouring tiles inside the 64-by-64 grid against a stored byte and walks a linked list of entries looking for the byte 26.
- `combine_two_shorts`: Calls another routine twice and returns a 32-bit value formed by concatenating the short result of the first call in the high word with the short result of the second call in the low word.
- `check_two_neighbor_tiles`: Checks up to two neighbouring tiles inside the 64-by-64 grid against a stored byte and walks a linked list of entries looking for the byte 26.
- `return_stack_contents`: This routine returns the two 16-bit values currently residing at the top of the stack.
- `check_one_neighbor_tile`: Checks one neighbouring tile inside the 64-by-64 grid against a stored byte and walks a linked list looking for the byte 26.
- `return_stack_contents_2`: This routine returns the two 16-bit values currently residing at the top of the stack.
- `return_stack_contents_3`: This routine returns the two 16-bit values currently residing at the top of the stack.
- `combine_two_call_results`: Calls one routine, keeps its short result, calls a second routine, and returns both results as a 32-bit value with the first in the high word.
- `combine_two_shorts_2`: Calls another routine twice and returns a 32-bit value formed by concatenating the 16-bit result of the first call into the high word and the 16-bit result of the second call into the low word.
- `write_byte_grid_from_routine`: Runs 17 by 17 nested passes, calling a routine on each pass and writing a byte register to a pointer each time.
- `decrement_byte_and_test_neighbors`: Decrements a signed byte in a structure when it is non-negative and the result stays non-negative.
- `increment_byte_and_check_neighbors`: Increments a signed byte in a structure when it is non-negative, then tests a set of neighbouring bytes at fixed offsets against the new value, calling routines for each failed comparison.
- `store_tile_code_and_reset`: Stores a tile code word in a global and calls routines; when the code is non-negative it clears a global counter and calls another routine.
- `clear_counter_and_call`: Sets a global counter to zero, calls another routine and returns.
- `update_global_value`: Stores a 16-bit value into a global variable and then either calls another routine or clears a second global variable and calls another routine based on the sign of the value.
- `reset_global_counter`: Sets a global variable to zero and then calls another routine.
- `fill_tile_map_from_table`: Walks a rectangular grid region between two coordinate pairs and reads a byte from a lookup table for each cell.
- `call_callback_in_nested_loops`: Calls a function pointer stored in a global 4096 times, using an inner loop of 64 calls repeated by an outer loop of 64 passes, then returns.
- `call_routine_when_bit_set`: Calls a routine when a global flag is zero, bit 0 is clear and bit 1 is set in a byte from a per-slot record, then returns its two stacked values.
- `call_routine_then_return_5`: Calls another routine once and then returns the value passed on the stack, without other work.
- `return_stack_value_3`: Returns the value currently located at the top of the stack.
- `clear_global_table_words`: Writes zero into successive word pairs of a global table, advancing four bytes per step until the pointer passes a fixed end address.
- `compare_bit_pairs_and_call`: Walks two pointers in opposite directions over 2048 steps, comparing the low three bits of each pair of bytes and calling a routine whenever they differ.
- `repeat_calls_until_zero`: Calls one routine, then repeatedly calls two other routines while a short value returned in a register is non-zero, and then returns.
- `build_random_event_list`: Picks positions from a random source in a counter-bounded loop, and for each unused entry in a global table fills a free 14-byte slot with fixed bytes and words.
- `insert_or_update_list_entry`: Walks a linked list for an entry tagged 22 and updates its matching word when one is found.
- `find_or_claim_list_entry`: Searches a linked list for a matching tagged entry and updates its link when found.
- `populate_list_entry_fields`: Writes a fixed set of bytes and words into a 14-byte entry pointed to by an address register, including a global value and a table word, and then calls another routine.
- `call_other_routine_in_loop`: The routine calls another routine once, then repeatedly calls two other routines in a loop until a short value returned in D1 is zero.
- `generate_random_indexed_value`: Calls a random routine to choose a starting index, then loops a counted number of times, building a value from a second random call and a table lookup.
- `claim_free_list_entry`: Checks a flag and a table entry for the given index and, when they allow it, finds the first 14-byte slot whose tag byte is zero.
- `insert_or_update_tag24_entry`: Walks a linked list for a tag-24 entry and updates its matching word when one is found.
- `find_or_claim_tag24_entry`: Searches a linked list for a tag-24 entry and updates its link when found.
- `populate_tag24_fields`: Writes constant bytes and values from registers and a global variable into a 14-byte structure pointed to by a pointer, sets a two-byte field to a table value, and then calls another routine.
- `advance_message_table_entries`: Steps through a table of 14-byte entries, updating each entry's state, flag and auxiliary bytes and moving its offset.
- `try_place_random_entry`: Picks a position offset by up to three cells from a random sample and returns zero when it falls outside the 0 to 63 range.
- `call_two_routines_return_one`: Calls two other routines and returns one as its result.
- `return_zero_value`: Returns zero as its result and performs no other work.
- `release_list_entry_chain`: Walks a linked list of entries from a per-slot head, clearing the tag byte of each entry whose address falls in a matching range and calling routines for each one released.
- `release_entry_from_pointer`: Follows the link from an entry pointer through a list, clearing the tag byte of an entry that lies in the in-range window and calling routines.
- `release_linked_entry`: Follows the next link of an entry and stops when the link is zero.
- `set_pixel_bits_in_planes`: For a point inside a 320-by-200 area, selects one of sixteen colour patterns from a value and sets or clears the matching bit in four plane bytes at the point's address.
- `double_and_call_twice`: Doubles a short argument, passes it to another routine, then calls that routine again with no argument and returns.
- `call_routine_return_inputs`: Calls another routine and returns the two input register values combined into one 64-bit result.
- `extract_bit_from_table`: Computes a byte index from a 16-bit position and a base address, reads that byte and returns the single bit selected by the low three bits of the position.
- `set_multi_plane_pixel`: Sets or clears a specific bit within four different bitplanes at a given coordinate.
- `set_pixel_bits_in_planes_2`: Sets or clears the matching bit in four plane bytes at an address derived from a position, selecting one of sixteen colour patterns from a value.
- `clear_bit_in_four_planes`: Clears one bit, selected by the low three bits of a register, in four plane bytes spaced 8000 bytes apart from a pointer.
- `set_bit_and_clear_others`: Sets a bit in a byte at a pointer using d0, then clears the same bit in three other bytes at specific offsets from that pointer.
- `clear_bit_and_set_one`: Clears a bit in a byte at a pointer using d0, then sets the same bit in a byte at an offset of 8000 from that pointer.
- `set_two_bits_and_clear_others`: Sets a bit in a byte at a pointer and a bit in a byte at an offset of 8000 from that pointer, then clears the same bit in two other bytes at specific offsets.
- `clear_bits_and_set_one`: Clears a bit in a byte at a pointer and in a byte at an offset of 8000, then sets the same bit in a byte at an offset of 16000 from that pointer.
- `set_two_bits_and_clear_others_alt`: Sets a bit in a byte at a pointer and a bit in a byte at an offset of 16000 from that pointer, then clears the same bit in two other bytes at specific offsets.
- `clear_and_set_bits`: Clears a bit in a byte at the address in a1 using the index in d0, sets bits at offsets 8000 and 16000 of the structure in a1 using the same index, and clears a bit at offset 24000.
- `set_and_clear_bits`: Sets a bit in a byte at the address in a1 using the index in d0, sets bits at offsets 8000 and 16000 of the structure in a1 using the same index, and clears a bit at offset 24000.
- `clear_and_set_bits_alt`: Clears bits at offsets 0, 8000, and 16000 of the structure in a1 using the index in d0, and sets a bit at offset 24000.
- `set_and_clear_bits_alt`: Sets a bit at offset 0 and offset 24000 of the structure in a1 using the index in d0, and clears bits at offsets 8000 and 16000.
- `toggle_bits_mixed`: Clears a bit at offset 0, sets bits at offsets 8000 and 24000, and clears a bit at offset 16000 using the index in d0.
- `set_bits_mixed`: Sets bits at offsets 0, 8000, and 24000 of the structure in a1 using the index in d0, and clears a bit at offset 16000.
- `set_blitter_window_and_mode`: Checks a table flag and, when it is set, calls a routine.
- `evaluate_byte_condition_and_call`: Selects one of sixteen comparison conditions from a table value and compares two bytes from a structure to decide which branch is taken.
- `stub_decompile_timed_out_2`: Contains only four instructions, and no pseudo-code could be produced because the decompiler timed out.
- `dispatch_object_handlers_in_list`: Loads a position for a record and walks a table-driven chain of entries, calling a handler pointer for each one and setting a flag in the record when a condition holds.
- `dispatch_object_handlers_in_list_2`: Continues the same table-driven handler walk from a given position and entry, calling handler pointers and applying the same flag and distance logic until a record's link ends the walk.
- `dispatch_object_handlers`: Calls a routine to dispatch object handlers in a list, then calls another routine to increment a counter and dispatch handlers.
- `process_object_update_and_bounds_check`: Checks if an object's position is within specified bounds using values from a table, then updates a global pointer and calls an indirect routine and two flagged pointer handlers based on bits in a status byte.
- `process_object_data`: Processes a structure by performing bounds checks, calling an indirect routine, calling flagged pointer handlers, and optionally calling blitter routines to set pixel bits in planes.
- `increment_counter_and_dispatch_handlers`: Checks a word argument against a limit and alignment, then increments a counter in a global table when its index maps to a valid entry.
- `call_flagged_pointer_handler`: Selects one of two global pointer slots using a flag byte from a record and calls the function pointer stored eight bytes after it.
- `call_indirect_if_flag_set`: Calls a routine through a stored function pointer when a global value is non-negative, then returns its two input registers combined.
- `blit_rows_to_planes_shifted`: Clips a row range against a 200-row screen, then shifts five-byte source rows by the sub-byte column offset and merges them into four plane words per row.
- `or_rows_into_planes`: Clips a row range to the screen, then ORs five-byte source rows into four plane words per row.
- `combine_bytes_into_planes`: Loops over a set number of rows, combining one source byte and three more bytes with the low byte of four plane words per row using AND and OR. The high byte of each word is kept unchanged, and both pointers advance one row per pass.
- `rotate_rows_into_planes`: Rotates five-byte source rows by a column-dependent amount and combines them with the four plane words in each destination row using AND and OR. It also updates a set of control words with the masked rotated values for each row.
- `program_blitter_sprite_line`: Clips a position to the 200-row screen, then programs the blitter with source pointers, masks and size for one sprite line, using a special case for the last column.
- `update_sprite_line_parameters`: Calculates blitter destination addresses and mask values based on sprite coordinates and dimensions.
- `program_blitter_sprite_line_2`: Clips the row to the 200-row screen, then programs the blitter for one sprite line with the same plane and mask writes, waiting for idle between each one.
- `program_blitter_sprite_column`: Programs blitter copies for one column of a sprite, waiting for the busy flag before each write and calling routines for the plane pointers.
- `return_without_work`: Returns immediately; the routine contains only the return instruction.
- `program_blitter_sprite_cell`: Writes blitter modulo, control and size constants with source and destination pointers for one sprite cell.
- `program_blitter_cell_plane`: Programs the blitter for one plane of a sprite cell with a different control word, using the same busy-flag waits as the cell routine.
- `program_blitter_cell_column`: Programs the blitter for a sprite column cell using the table entry for the index, waiting for the busy flag before and after each write.
- `program_blitter_sprite_span`: Clips a sprite position to the 200-row screen, then chooses one of three blitter setups from the column offset and writes source pointers, masks and size for the plane.
- `configure_blitter_sprite_parameters`: Calculates blitter source and destination pointers, masks, and area sizes based on sprite column and row clipping.
- `program_blitter_sprite_span_2`: Clips the row to the 200-row screen, then chooses one of three blitter setups by column offset, writing source pointers, masks and size for each plane.
- `program_blitter_sprite_column_2`: Chooses one of three blitter setups from a column offset and writes source pointers, masks and size for one sprite column, waiting for the busy flag between writes.
- `program_blitter_cell_plane_2`: Programs the blitter for one plane of a sprite cell with a fixed control constant, using busy-flag waits before and after the writes.
- `program_blitter_cell_plane_3`: Programs the blitter for one plane of a sprite cell with a different control constant and mask value, waiting for the busy flag before and after each write.
- `program_blitter_shifted_cell`: Programs the blitter for a shifted sprite cell, writing plane pointers and a source offset increased by two, and waits for the busy flag between writes.
- `program_blitter_shifted_cell_2`: Programs the blitter for a shifted sprite cell with a different mask and control constant, using busy-flag waits between writes.
- `program_blitter_table_cell`: Looks up a value from a table using a register index, writes fixed mask and control constants and plane pointer values, and waits for the blitter busy flag between writes.
- `extract_and_mask_value`: Calls another routine, stores its 32-bit result in a global, and returns the upper 24 bits of that result combined with the low byte of an input register, masked to 47 bits.
- `multiply_register_halves`: Multiplies the low 16-bit halves of two register values and adds the cross products of the high and low halves shifted into the upper word.
- `parse_signed_decimal_text`: Reads a text string, skipping spaces and tabs, and accumulates decimal digits into a 32-bit value.
- `skip_text_to_number`: Skips spaces and tabs and consumes digits in a text string.
- `skip_text_between_numbers`: Skips spaces and tabs and reads digits, stopping at a line break, at a minus sign that follows digits, or at another character.
- `scan_text_for_number_end`: Scans characters, recording whether a digit has been seen, and returns at a line break or at a non-space character.
- `copy_16_word_blocks`: Copies 500 blocks of 16 words each from a source pointer to a destination pointer, one word at a time, and returns its two input registers combined.
- `clear_zeroed_struct_array`: Writes zero into eight words per element across 1000 elements of a structure array, advancing the pointer by eight words for each element.
- `lookup_signed_ratio_table`: Reduces two signed 16-bit inputs by a shift of five, picks the larger magnitude as an index, and reads a byte from a table at an offset built from both values.
- `lookup_table_byte_at_offset`: Computes an index from two 16-bit values, shifted by a value taken from the second input, and returns the byte from a table at a fixed offset from a base pointer.
- `lookup_negated_table_byte`: Selects a shifted index from the two inputs, reads a byte from a table at a fixed offset and returns a fixed -128 value minus that byte.
- `fetch_offset_table_byte`: Calculates an index from two input words and uses it to fetch a byte from a table at a base pointer plus an offset, returning the value with a fixed offset applied.
- `lookup_signed_ratio_table_2`: Chooses the larger shifted magnitude of two signed inputs as an index and reads a table byte at an offset built from both values.
- `lookup_negated_table_byte_2`: Reads a byte from a table at an offset built from two shifted inputs and returns its negation.
- `lookup_table_byte_offset`: Reads a byte from a table indexed by two shifted inputs and returns it with a fixed -128 offset added.
- `read_byte_from_table`: Reads a byte from a table at a base pointer plus an offset built from two input words, and returns it with a fixed -128 offset added.
- `format_decimal_text`: Converts a 32-bit value to decimal text by repeated subtraction against a table of powers of ten, suppressing leading zeros.
- `format_decimal_word`: Converts a 16-bit value to decimal text by repeated subtraction against a table of powers of ten, suppressing leading zeros.
- `format_decimal_string`: Converts an integer into a null-terminated string of decimal characters by repeatedly subtracting values from the input.
- `stub_bad_instruction_data`: Stores one word through a computed address and then reaches undecodable data, so this is a stub and no real behaviour is recovered.
- `fetch_keymap_character`: Clears a global flag and, when it was set and odd, converts a saved byte into an index into a key-map text table and returns the character stored there.
- `pack_nibbles_and_process_rows`: Calls a library routine, packs sixteen words into nibble-split bytes in a buffer, and then invokes a library handler for each of 200 rows across four plane offsets before a final handler call.
- `interpolate_two_tables`: Blends two 16-entry word tables in 17 steps with the blend factor rising from zero, writing each blended 16-bit value into two output buffers and updating a global flag before each step.
- `compare_text_prefix`: Compares a text string with a table string one byte at a time, up to a counter limit.
- `check_text_nonempty`: Returns one when the first byte of a string is non-zero and zero otherwise.
- `fill_pattern_from_table`: Expands a 16-bit seed into a sequence of bytes by reading pairs from a global table and writing them into a buffer until the buffer is terminated with a zero byte.
- `xor_words_then_loop_calls`: Calls a routine, XORs two consecutive 32-bit words at a pointer with fixed constants, calls a routine repeatedly while a register value is non-zero, and then calls one more routine.
- `encode_value_as_four_letters`: Writes a value into four buffer characters by splitting it into base-26 digits with place values of 17576, 676, 26 and 1, adding the letter A to each digit.
- `swap_bytes_in_struct`: Swaps six pairs of bytes at fixed offsets inside a structure pointed to by an address register.
- `rotate_bits_in_place`: Shifts an eight-byte block one bit at a time in place, carrying bits from byte to byte, and then stores a routine pointer and a global value into two locations at the end of the block.
- `decode_and_xor_structure`: Decodes a series of values, performs a division check, and then applies bitwise XOR operations to a structure using specific constants.
- `return_minus_one`: Returns minus one as its result and performs no other work.
- `decode_four_letter_value`: Reads four characters from a pointer, subtracts 65 from each, and combines them in base-26 into one integer that it returns.
- `unpack_bitstream_block`: Reads a packed bit stream through a bit reader and writes decoded bytes backwards from the end of a buffer.
- `stub_infinite_wait_loop`: Returns when the zero flag is set, otherwise it spins forever in an empty loop.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `copy_and_transform_buffer`: Walks a table of records until one has a negative pointer.
- `dispatch_four_char_tag`: Compares a 32-bit tag with the four-character codes INIT, FRM2 and STOP and returns a fixed 16-bit code for each match.
- `dispatch_four_char_tag_2`: Handles the FRM2 and STOP four-character tags, returning a fixed 16-bit code for STOP and calling routines for FRM2.
- `extract_high_word_duplicate`: Takes a 32-bit register value, extracts its high 16 bits and returns a 32-bit value made of those bits in both halves.
- `check_stop_tag_or_decrement`: Returns the tag unchanged when it equals the STOP code.
- `decrement_length_and_call`: Calls a routine, decrements the low word of the input, and when the count reaches one with a zero high word calls another routine and returns its result.
- `duplicate_word16_into_long`: Takes the low 16-bit word of D0 and returns it repeated in both halves of a 32-bit result.
- `decode_rle_stream`: Processes a run-length encoded byte stream by either repeating a byte a specified number of times or copying literal bytes into a destination.
- `decode_rle_four_bitplanes`: Decodes a run-length packed stream in which a negative control byte repeats the following byte and a non-negative one copies literal bytes.
- `copy_8000_longwords`: Copies 8000 consecutive 32-bit values from one global table to another, one value per step, using a counter that runs from 7999 down to zero.
- `copy_sixteen_words_after_sentinels`: Scans two global word tables for the value 384 and then writes sixteen 16-bit values from the source pointer into both tables just after each sentinel, with a stride of two words.
- `clear_and_unpack_column_bitplanes`: Zeroes a 32000-byte buffer and then decodes four bitplanes column by column from a packed stream.
- `unpack_column_bitplanes`: Decodes four bitplanes column by column from a packed stream without clearing the destination first.
- `unpack_column_bitplanes_strided`: Decodes packed column data into bitplane buffers using a row stride held in a register, with a 255 marker that skips a plane.
- `decode_run_length_columns`: Decodes a column-oriented packed stream into a bitplane, writing repeated or literal bytes down columns with a register-supplied stride.
- `decode_packed_column_stream`: Decodes a run-length packed stream into bitplane columns, writing repeated or literal bytes at a register-supplied stride.
- `decode_bitplanes_until_marker`: Steps through four bitplanes of 8000 bytes each, decoding column data into every plane with the shared column decoder until a 255 marker ends the data.
- `init_table_and_enable_dma`: Initialises a 16-bit table by subtracting a set of weights from each entry, then writes the DMA control register with the channel-enable bit set.
- `poll_status_until_zero`: Repeatedly calls an unnamed routine and tests a status word, returning minus one when a mismatched status is nonzero.
- `wait_for_status_ready`: Calls an unnamed routine that returns a status, resets a mode variable to zero when it equals two, and loops until the status is zero.
- `scan_region_for_matching_entry`: Copies word fields inside a structure and into a global, then walks a memory region in four-byte steps, forwards and then backwards.
- `check_and_adjust_flags`: Calls an unnamed routine and, if its result is zero and a register matches a global value, copies two bytes from a record into a global, subtracting four from each.
- `claim_free_slot_entry`: Scans a global table of 52-byte entries for the first slot whose marker byte is zero and claims it.
- `execute_indirect_handler`: Calls another routine and then executes a routine pointer retrieved from a global variable via an offset.
- `copy_state_blocks_and_reset`: Copies a 57-byte block and then a 59-byte block from fixed source addresses into the global state area.
- `filter_entry_pairs_into_lists`: Walks a set of 314-byte structures and, for each, copies the short pairs whose table flag is set into two output lists.
- `reset_globals_and_init_state`: Zeroes the global variable area up to a fixed end address, then on first run sets a state flag and stores initial byte values.
- `compute_mod5_offset_value`: Calls two unnamed routines, then stores a global value made from the low word of an input register taken modulo five, multiplied by 727, plus a second register value.
- `or_accumulate_word_blocks`: Calls an unnamed routine, then runs a loop whose count is a global counter divided by five.
- `update_timers_and_copy_state`: Polls the sum of two global words for a short bounded period and resets a counter.
- `clamp_two_globals`: Clamps two global variables to a range between zero and fifty-six inclusive.
- `swap_two_slot_entries`: Exchanges eight bytes and two 16-bit fields between two 314-byte entries, one chosen by the input index and one by a global current index.
- `call_two_unnamed_routines`: The decompiler timed out, so no pseudo-code is available.
- `manage_entity_list_entries`: Iterates through a table of structures to either initialize a new entry, update an existing list entry, or modify specific bytes within a structure based on values extracted from a provided parameter.
- `call_three_unnamed_routines`: The decompiler timed out, so no pseudo-code is available.
- `decrement_counter_and_clear_flags`: Subtracts two from a 16-bit counter in a global table entry selected by a byte in the record.
- `move_record_between_bucket_lists`: Stores two 16-bit values into a record and, when its key bytes have changed, unlinks it from its old bucket list.
- `relink_record_into_table_slot`: Copies a link value from a record into a table slot and clears the record's back-reference.
- `push_record_onto_list_head`: Clears the link field in a record, then pushes the record onto the front of the list chosen by its key bytes.
- `remove_record_from_key_list`: Finds the list selected by a record's key bytes and unlinks the record from it, updating the head if the record is first or patching the previous link otherwise.
- `perform_complex_division`: Subtracts words from d0 and d1, and if both results are negative, performs two 16-bit unsigned divisions using a byte from a structure in a0 and executes several byte swaps on d2.
- `perform_division_and_swap`: Performs two 16-bit unsigned divisions using d2, d1, and d0, then swaps the high and low bytes of d2 and d1 multiple times before branching.
- `perform_division_and_negate_2`: Negates a 16-bit word in d1, extends d0 and d1 to 32-bit, and performs two 16-bit unsigned divisions using a byte from a structure in a0, then performs several byte swaps on d2.
- `perform_division_and_negate`: Performs two 16-bit unsigned divisions using d2, d1, and d0, executes several byte swaps on d2 and d1, negates d1, and then branches.
- `calculate_division_results_2`: Subtracts a word from d1, and if the result is not negative, negates d0, extends d0 and d1, performs two 16-bit unsigned divisions using a byte from a structure in a0, and performs several byte swaps on d2.
- `perform_division_and_negate_d0`: Performs two 16-bit unsigned divisions using d2, d1, and d0, executes several byte swaps on d2 and d1, negates d0, and then branches.
- `calculate_division_results`: Negates d0 and d1, extends them to 32-bit, loads a byte from offset 18 of a structure in a0 into d2, and if d0 is not greater than d1, performs two 16-bit unsigned divisions and swaps bytes in d2 and d0.
- `update_division_results`: Performs division operations using d0, d1, and d2, then updates two 16-bit values at offsets 14 and 16 of a structure in a0 if the results differ from existing values.
- `merge_entity_data`: Merges data from a structure in a0 into a structure in a1, clears links and flags, and calls a routine to remove a record from a key list.
- `accumulate_and_drain_entry_totals`: Adds a computed amount to a 32-bit total in a global table entry selected by a byte in the record, then subtracts the same amount from a second total and clamps it to zero if it would go negative.
- `reset_entity_state`: Calls an unnamed routine, then calls another one if a 16-bit field in the structure equals two.
- `set_entity_state_by_kind`: Writes a pair of 16-bit state fields and a state byte in the structure, choosing the second field from a flag byte.
- `set_entity_state_simple`: If the first byte is four it calls two unnamed routines.
- `set_state_word_and_call`: Writes zero and then a constant into a 16-bit field of the structure, overwriting it with a second constant when a flag byte is not one.
- `set_entity_flags_and_timer`: Calls an unnamed routine, then writes constant state values to a 16-bit field and to two bytes of the structure.
- `link_or_append_structures`: Compares two structures by a flag bit, clears a link field and unlinks a previous entry where needed.
- `link_two_structures`: Writes each structure's offset into a link field of the other, then sets a state byte in each.
- `initialize_spawn_entry`: Initialises a structure when its flag bit is clear, writing type and state bytes, a 16-bit value from a global and two flag bytes.
- `call_four_unnamed_routines`: The decompiler timed out, so no pseudo-code is available.
- `mark_matching_list_entry`: Checks a flag word and, when allowed, searches a linked list in a global table for an entry with a matching kind byte and key byte.
- `mark_matching_chain_entry`: Walks a chain of linked entries starting from a record and sets bit four of the first entry whose kind byte and key byte both match.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a0, then returns.
- `call_one_unnamed_routine`: The decompiler timed out, so no pseudo-code is available.
- `call_single_unnamed_routine`: The decompiler timed out, so no pseudo-code is available.
- `expire_timer_and_set_state`: Subtracts a 32-bit amount from a timer field in one structure.
- `set_state_byte_return_one`: Stores a constant state byte in the structure and returns one.
- `stub_return_zero`: Returns zero with no other effect.
- `compute_byte_deltas_and_quotient`: Compares two input bytes with reference bytes in the structure and writes two 16-bit difference values into two fields.
- `store_divide_quotient_word`: Divides a 32-bit input value by a 16-bit divisor and stores the 16-bit quotient in a structure field.
- `normalize_two_byte_fields`: Compares two input bytes with the high bytes of two input words and writes two 16-bit results into the structure.
- `claim_free_slot_and_dispatch`: Scans a global table of 52-byte entries for a free slot and initialises it with fixed state values, the two input bytes and an index.
- `init_slot_and_dispatch`: Initialises a given entry with fixed state values, the two input bytes and an index.
- `validate_and_update_byte`: Reads a byte from the structure and, when it is nonzero and a computed table index matches a global, calls an unnamed routine.
- `forward_routine_result_word`: The recovered code calls one unnamed routine and returns its 16-bit result unchanged.
- `store_masked_keyed_bytes`: Walks a table of 16-bit keys until a sentinel value of minus ninety-nine is reached.
- `return_from_subroutine_3`: An empty routine that performs no operations and returns immediately.
- `scan_masked_keys_into_list`: Walks a table of 16-bit keys until a counter reaches zero.
- `check_keys_against_mask_table`: Walks a table of 16-bit keys until a sentinel value of minus ninety-nine.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `stub_return_zero_2`: Returns zero with no other effect.
- `mark_matching_keyed_bytes`: Walks a table of 16-bit keys and, for each key with no invalid bits whose target byte equals a given value, sets that byte to fifteen.
- `fill_keyed_bytes_from_table`: Walks a table of 16-bit keys until a sentinel value of minus ninety-nine.
- `iterate_linked_structures`: Walks a list of 314-byte records, decrementing a countdown for records of one type and reloading it from a second field when it reaches zero.
- `process_linked_structure_node`: Checks a structure's timer and link fields.
- `update_linked_structure_neighbor`: Decrements a countdown in a structure and, when it expires, reloads it and searches neighbouring table entries for one with the same kind byte, comparing a weight derived from flag bits.
- `find_matching_weight_neighbor`: Checks a timer field and a stored key in a structure, then searches neighbouring table entries for one whose weight matches the structure's own weight.
- `find_matching_weight_entry`: Walks a list of key values and, for each key whose weight matches a given weight, writes a state byte and a position byte to an output record and decrements a counter.
- `write_entry_and_count_down`: Writes a byte and two derived bytes into an output record and copies a 16-bit field into a second field.
- `clear_counter_return_zero`: Clears a 16-bit counter in the structure and returns zero.
- `decrement_flag_and_test_counter`: If a flag field is set, decrements a countdown and reloads it when it expires.
- `dispatch_handler_by_index_3`: When a global counter exceeds 249 and a flag is set, selects a handler from a dispatch table, either from a stored index or from a value returned by another routine taken modulo a size field.
- `find_closest_flagged_key`: Scans a table of 16-bit keys whose table flags include bit eight, choosing the key closest to a target value derived from the structure.
- `find_closest_flagged_key_b`: Scans a table of 16-bit keys whose table flags include bit eight, keeping the key closest to a target value derived from the input.
- `find_closest_flagged_key_2`: Iterates through a table of 16-bit values to find the entry that minimizes the absolute difference between the entry and a target value, provided certain bits are set in a related byte.
- `return_zero_2`: This routine is empty and simply returns zero to the caller.
- `compare_bytes_with_retry_counter`: Compares byte pairs from a structure and a referenced entry.
- `clear_flag_when_below_quarter`: If a 16-bit flag word is nonzero and a counter field is at most a quarter of that value, clears the flag word to zero.
- `store_offset_in_table`: Stores the structure's offset into a table entry selected by a byte in the structure, then sets bit one of the structure's flag byte and returns the input value unchanged.
- `clamp_and_store_two_bytes`: Calls an unnamed routine, then clamps two byte values to the range zero to 63 and writes them into a structure, each followed by a constant byte.
- `initialize_table_entry_from_index`: Computes a table offset from an index byte and stores a pointer to a 14-byte record in a table entry.
- `clear_flag_and_zero_entry`: If bit zero of the structure's flag byte is set, clears that bit, writes zero to a table entry selected by a byte in the structure, and calls an unnamed routine.
- `claim_table_slot_if_free`: Calls an unnamed routine and, when it returns zero, copies two bytes from a reference structure.
- `set_state_fixed_with_global`: Writes a constant 16-bit field and a state byte into the structure, and copies a 16-bit value from a global into another field of the structure.
- `set_two_state_bytes`: Sets two state bytes in the structure to constants and makes no other changes.
- `check_linked_entries_for_cost`: Computes a key from two input values and tests the masks of the table entry it addresses.
- `check_linked_entries_for_limit`: Walks the linked entry chain for a key derived from an input value.
- `check_linked_entries_for_mask`: Walks a linked entry chain starting from an input key.
- `walk_chain_checking_limit`: Walks a linked entry chain from an input key and returns zero at the end of the chain.
- `walk_chain_until_limit`: Repeatedly walks a linked entry chain from a structure's table slot, returning zero when the chain ends and minus two when a blocking entry or a computed limit is reached.
- `stub_return_minus_two`: Returns minus two with no other effect.
- `attach_slot_and_dispatch`: If the table slot for an input index is set, calls an unnamed routine to obtain a value.
- `attach_value_and_dispatch`: Calls an unnamed routine to obtain a value, sets a flag bit and state values in the structure, and stores the value in a field.
- `initialize_and_call_handler`: Sets several bytes and words within a structure, stores a value from a global table into a structure offset, and calls a function pointer retrieved from a global table.
- `link_nearest_unlinked_entry`: Scans the table for the nearest entry by summed absolute byte differences that is active, not already linked and not flagged.
- `link_pair_and_set_state`: Writes each structure's offset into a link field of the other, sets a state byte in the first structure and returns one.
- `return_zero_3`: This routine is an empty function that returns zero to the caller.
- `resolve_linked_entry_state`: Follows a link from the structure to a table entry.
- `initialize_struct_fields_2`: Writes constant values into fields of a structure: a 16-bit field at offset ten, a 16-bit field at offset 20, and two state bytes.
- `resolve_neighbor_link_state`: Walks a chain of linked entries from a structure and calls unnamed routines to check each neighbour.
- `set_structure_bytes`: stores the values four and thirty-eight into two consecutive byte offsets of a structure.
- `compute_neighbour_offsets`: Compares two input bytes with the structure's reference bytes to produce direction steps of minus one, zero or one.
- `compute_neighbour_offsets_alt`: Compares two input register values with the structure's reference bytes to produce direction steps of minus one, zero or one.
- `write_neighbour_table_entry`: Writes a link offset, a 16-bit value and two bytes into a global table entry for the structure.
- `clear_entity_links_and_flags`: Clears link fields of a structure and of its table entries when a flag bit is set.
- `copy_record_into_free_slots`: When a counter is positive and a 32-bit field exceeds twenty, halves that field.
- `copy_word_table_entries`: Copies a 26-word record from a source to a destination, clears two fields and sets a state byte, then calls an unnamed routine.
- `compute_indexed_value`: Reads a 16-bit value from a global table at an index derived from an input word.
- `call_if_table_value_nonnegative`: When a global flag is zero and a 16-bit table value selected by an input byte is nonnegative, calls an unnamed routine.
- `return_stack_argument_pair`: Returns the two 16-bit values passed on the stack as a combined result, with no other effect.
- `call_two_unnamed_routines_2`: The decompiler timed out, so no pseudo-code is available.
- `process_cell_records`: Performs complex updates to a data structure by checking neighbor cell counts, extracting masked values, and moving records between bucket lists or key lists based on various conditional checks and arithmetic operations.
- `validate_address_range`: checks if a value falls within two specific memory ranges, potentially calling another routine or zeroing a byte at an offset if the ranges are not met.
- `clear_slot_unless_in_range`: If the pointer lies inside an address range it calls an unnamed routine and returns.
- `clear_slot_and_call`: Clears a byte at offset twelve of the structure and calls an unnamed routine.
- `call_single_unnamed_routine_2`: The decompiler timed out, so no pseudo-code is available.
- `call_three_unnamed_routines_2`: The decompiler timed out, so no pseudo-code is available.
- `initialize_claimed_entry`: Calls a routine to extract and mask a value, then iterates through a list of structures to find an unused entry to initialize with specific values and calls another routine to push the record onto a list head.
- `init_claimed_entry`: Writes position, kind and state fields into a given entry, adds an index-dependent table value to a 16-bit field, and calls two unnamed routines.
- `return_zero_value_2`: This routine is empty and simply returns a zero value to the caller.
- `initialize_empty_entry_slot`: Scans 32-byte table entries for an empty slot and fills the first one with register bytes, a constant size, a flag, a constant identifier and a table-derived value.
- `fill_entry_and_tag_matches`: Searches a key list, up to a sentinel value, for a key whose table slot is free and fills that slot with the input bytes and a table-derived value.
- `fill_slot_tag_matches`: Fills the first free 32-byte table slot with the input bytes and a table-derived value, then tags matching flagged entries with a sequence number, stopping at a sentinel value in the key list.
- `fill_slot_and_tag_matches`: Fills a given table slot with input bytes and a table-derived value, then tags matching flagged entries with a sequence number, stopping at a sentinel value in the key list.
- `return_stack_argument`: Returns the value passed on the stack with no other effect.
- `return_stack_value_4`: returns the value currently at the top of the stack.
- `claim_slot_for_index`: If the index's table pointer is unset, claims the first free 32-byte slot, stores its offset in that pointer, and fills its kind and position fields before calling an unnamed routine.
- `claim_slot_and_store_pointer`: Claims the first free 32-byte slot, stores its offset in a field of the input structure, fills its kind and position fields and calls an unnamed routine.
- `initialize_entry_fields_and_call`: Writes a pointer offset into a field of the input structure, then fills the kind, position and state fields of the given entry and calls an unnamed routine.
- `spawn_entries_from_offset_table`: While a table slot's chain count is zero, repeatedly claims a free 32-byte entry and fills its position and kind fields with offsets from a table indexed by a modulus of an unnamed routine's result.
- `spawn_linked_pair_entries`: Repeatedly claims a free 32-byte entry and fills its position bytes from a table indexed by a modulus of an unnamed routine's result, clamped to a valid range.
- `clear_active_entity_chain`: If the table slot for an input index is set, clears that slot and writes a state byte and a 16-bit value to the referenced entry.
- `extend_or_create_extent_entry`: If an entry's flag bits are set, it marks the entry and either creates a new extent entry in a free slot for the given index or widens the existing extent to include the entry's position.
- `widen_extent_bounds`: Widens the bounds stored in an entry toward a given position on both axes, keeping the larger margin on each axis, and copies the bounds into the entry's state bytes.
- `widen_extent_bounds_second_axis`: Widens the second-axis bound of an entry toward a given position, keeping the larger margin, and copies the bounds into the entry's state bytes.
- `create_extent_entry`: Claims the first free 32-byte entry for an index and sets its bounds to one less than the given position on both axes, with fixed state bytes.
- `initialize_extent_entry`: Writes an index byte, bounds one less than the given position on both axes, and fixed state bytes into a given entry.
- `set_kind_and_link_offset`: If the structure's state byte is not the reserved value, it writes a 16-bit field from a constant or from a table depending on a flag byte, sets a kind byte, state byte and flag byte, and stores an offset in a 16-bit field.
- `initialize_object_header`: Writes a 16-bit field, a kind byte, a state byte and a flag byte into a structure, then stores an offset derived from the input word in a 16-bit field.
- `reset_object_state`: If a state byte is not the reserved value, clears a 16-bit field, sets the state byte to a constant and writes a 16-bit value from a table to another field.
- `iterate_table_entries`: Walks a table of 16-byte entries.
- `build_or_link_matching_entries`: Checks a key against a flag mask and, when it passes, walks a linked list for matching entries.
- `link_or_create_matching_entry`: Collects entries whose key matches an input value from a linked list into a short list.
- `collect_matching_entries_list`: Walks an input list of keys, collecting entries whose key matches each key into a short list.
- `create_entry_and_update_neighbours`: Creates an entry from an index and input value, then updates the state of three neighbouring entries and sets a state from a flag for the new entry.
- `update_list_entry_states`: Sets a 16-bit field and a state byte on the input entry, then walks a list of entries, updating the state bytes of matching entries until the count runs out.
- `update_entry_states_until_count`: Walks a list of entries, updating the state bytes of matching entries until a counter runs out, then sets a state from a flag.
- `return_null_pointer`: This routine is empty and returns an uninitialized value from the stack.
- `fill_free_slots_with_keys`: Calls an unnamed routine to obtain a 32-bit value and splits its result by a divisor of nine.
- `fill_free_slots_from_position`: Repeatedly claims free 16-byte entries, writing a position from an input word plus a value from an unnamed routine and a key derived from a division by nine, then calls another routine.
- `call_three_unnamed_routines_3`: The decompiler timed out, so no pseudo-code is available.
- `update_linked_entry_kinds`: Walks a linked list of entries, converting entries of one kind into another kind with a state value and clearing four state bytes.
- `update_linked_entry_kinds_alt`: Walks nested linked lists of entries, converting entries of one kind into another kind with a state value and clearing four state bytes.
- `claim_entry_at_position`: If the two input bytes both lie in the range zero to 63, claims the first free 16-byte entry and writes its position, kind, a value from a global and half of a 16-bit input before calling an unnamed routine.
- `initialize_entity_record`: Writes position bytes, a kind byte, a state value and a word from a global into an entry, then calls an unnamed routine.
- `check_neighbour_cell_count`: Checks a table cell at a position.
- `combine_high_word_with_flag`: Unless a flag word equals one, calls an unnamed routine and uses its 16-bit result as the high word.
- `stub_return_minus_one`: Returns minus one with no other effect.
- `store_byte_and_decrement`: Copies a byte from a field of the input structure into a byte of another structure, subtracts a constant from it and returns zero.
- `swap_table_entry`: This routine builds an index from two bytes, checks a flag at a calculated address, and if set, replaces a table entry with a value from another location in the table.
- `update_mask_entries_for_position`: Checks a flag word at a position and, when bit ten is clear and a mask test passes, either finds the cell value in a small table or builds a bitmask from neighbouring cells with flag bit six set.
- `update_mask_entries_for_cell`: Builds a cell value from a small table or from neighbouring cells with flag bit six, then writes it to the cell.
- `update_neighbour_mask_bits`: Writes a value into a cell, updates a linked entry's state, and sets bits in neighbouring cells that have flag bit six.
- `build_neighbour_mask_bits`: Builds a bitmask from neighbouring cells flagged with bit six, writes the resulting value into a cell, updates a linked entry's state and sets bits in neighbouring cells.
- `update_entry_neighbour_bits`: Finds the first linked entry of a list and sets its state values, then sets bits in neighbouring cells using the flag and table lookup.
- `stub_return_one`: Returns one with no other effect.
- `mark_table_cells_by_flags`: Loops a number of times derived from a global value.
- `mark_table_cells_with_mask`: Loops over a count derived from a global value, each time calling an unnamed routine to address a cell in a 90-entry table.
- `mark_table_cells_by_offset`: Loops over a count derived from a global value and an input offset, calling an unnamed routine each time to address a cell in a 90-entry table.
- `spawn_entries_matching_mask_cells`: Scans a list of offsets for cells whose flag bits match a mask.
- `initialize_entity_record_2`: Populates a data structure with specific attribute values, including scaled integer values and flags, derived from a lookup table.
- `stub_return_only`: Routine consisting only of a return, with no calls, memory access or hardware register use; it performs no work.
- `spawn_positioned_entries`: Repeatedly claims free 16-byte entries from a table of positions, writing position bytes, a kind byte, a state byte and a 16-bit value from a per-index table.
- `spawn_positioned_entries_alt`: Repeatedly claims free 16-byte entries from a table of positions, writing position bytes, a kind byte, a state byte and a 16-bit value from a per-index table.
- `claim_entry_with_index_size`: Claims the first free 16-byte entry, writing two position bytes and a kind byte, and sets a 16-bit size field from a per-index table, returning one.
- `initialize_struct_from_registers`: Writes a kind byte and two position bytes from registers into a given entry, sets a 16-bit size field from a per-index table, writes a state byte, and returns one.
- `process_cell_area_around_entry`: Walks a rectangular area of table cells around a position, calling unnamed routines for each cell with flag bits set.
- `process_cell_area_with_params`: Walks a rectangular area of table cells defined by its parameters, calling unnamed routines for flagged cells and adjusting cell values by a fixed offset.
- `process_cell_area_list`: Iterates a list of entries, adjusting table cells in each covered rectangle by a fixed offset when their low nibble is nonzero, and calling unnamed routines for flagged entries.
- `advance_counter_by_four`: Adds four to the 16-bit counter field at offset twenty of the structure and returns.
- `apply_table_cell_updates`: Calls an unnamed routine a fixed number of times, then walks a table of three-byte records.
- `modulo_decrement_loop`: Calls an unnamed routine and takes its result modulo a global value to set a loop count.
- `claim_cell_entry_if_unlinked`: Checks a cell's entry list for an existing entry kind.
- `allocate_free_table_record`: Scans a table of fixed-size records for the first one whose in-use byte is clear, then fills its fields from a lookup table and from a value returned by a helper.
- `initialize_record_from_lookup`: Fills the fields of a record addressed by a register from a lookup table entry and a value returned by a helper, and sets its in-use byte.
- `clear_record_field_status`: Zeroes a four-byte field of the record addressed by a register and returns minus one.
- `return_stack_argument_unchanged`: Returns its stack argument unchanged and touches no other state.
- `analyze_undecompiled_handler_body`: The decompiler produced no pseudo-code for this seven-instruction routine, so its behaviour cannot be read.
- `claim_record_for_empty_cell`: Checks that a cell of a global byte table is empty for the position given in two input registers, then claims the first free record in the list and sets its type, position, and a timer taken from a helper's result.
- `set_marker_and_initialize_record`: Writes a constant marker byte through an address register, then fills a record addressed by another register with a type, a position, and a timer computed from a helper's result.
- `pass_stack_argument_through`: Returns its stack argument unchanged and touches no other state.
- `claim_free_record_with_defaults`: Claims the first unused fixed-size record, sets its in-use byte, and clears its position.
- `initialize_record_fields_from_input`: Sets the in-use byte and clears the position of a record addressed by a register.
- `return_constant_zero`: Returns zero with no side effects.
- `set_flag_for_linked_entries`: Walks a linked list of entries that starts at a table slot chosen by two input bytes.
- `mark_cell_if_nibble_full`: Reads a position from a structure addressed by a register and looks up a cell in a global byte table.
- `return_stack_argument_as_is`: Returns its stack argument unchanged and touches no other state.
- `process_global_table_entries`: Walks a short table terminated by a sentinel value and, for each entry, computes an index from two input bytes plus the entry value.
- `clear_record_inuse_flag`: Clears the byte at the in-use offset of the record addressed by a register.
- `iterate_handler_table`: Walks a table of handler entries stored in global data, calling each handler indirectly and then clearing a byte and a word in the structure it returns, until the walk passes a fixed end address.
- `inspect_undecompiled_routine_body`: The decompiler gave no pseudo-code for this twelve-instruction routine, so only its size and callers are known.
- `branch_to_subroutine`: Performs a short branch to another location.
- `execute_disjoint_subroutines`: This routine acts as a dispatcher that executes various subroutines based on input parameters, including releasing a list entry chain, interpolating tables, filling string pointer tables, inserting a letter into an entry field, and performing indexed calculations using a multiplier of 314.
- `check_and_update_table_entry`: Looks up a 16-bit value in a table at the current global index and, when the value is non-zero and within a limit, advances the index.
- `run_delayed_polling_loop`: Runs nested loops over helper calls and status checks, pausing in two busy-wait countdowns of one hundred thousand steps each, and leaves through a common exit when a check fails.
- `run_status_checked_scan_loop`: Runs loops over helper calls that return status values, choosing between two scanning paths by a state byte.
- `scan_tables_and_set_modes`: Walks a table of entries up to a fixed end address, then runs a countdown loop over helper calls.
- `call_helper_return_argument`: Calls one helper routine and then returns the eight-byte stack argument unchanged.
- `return_stack_value_after_call`: Calls one helper routine and returns the eight-byte value passed on the stack unchanged.
- `check_state_and_merge_flags`: Tests a state byte in a structure reached through a global pointer.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 1 into d0, then returns.
- `remap_entry_state_bytes`: Calls a helper, then visits three structures in a table reached through a global pointer.
- `clear_flags_and_process_table`: Clears each non-zero entry of a block of short flags, calling a helper and an extra helper when a related entry is set.
- `toggle_table_entry_with_masks`: Indexes a table of short entries by a byte in the input structure and compares the entry with a computed key.
- `self_calling_loop_with_conditional_call`: The routine calls another routine repeatedly with a flag that doubles each iteration while a short value stays below a threshold, then conditionally calls the same routine once more if a global variable is non-zero.
- `update_global_flag_and_table`: Checks a global flag.
- `update_entry_flags_and_neighbor`: Indexes a table of entries by the first input and calls a helper twice with values derived from the entry's data.
- `check_flag_and_call_twice`: The routine checks a global flag and, if non-zero, writes the flag plus a constant to a global variable, stores the address of a global variable into another global variable, then calls another routine twice.
- `initialize_interrupt_handlers`: Writes pointers to a shared data block and to handler routines into many RAM locations and sets a few bytes.
- `remove_interrupt_server_if_active`: Removes an interrupt server through the Exec library when a counter is positive, and touches the CIA-B interrupt control register and the DMA control register.
- `apply_mask_flags_to_records`: Walks four records through pointers kept in a global table and, for each record chosen by the low four bits of the parameter, writes flag bytes and words according to the parameter's mask bits.
- `search_triple_table_for_value`: Walks a table of three-short entries from the address in an address register, stopping at a zero terminator or at the first entry equal to the input value.
- `analyze_undecompiled_routine_body`: The decompiler timed out and gave no pseudo-code for this 49-instruction routine.
- `process_flagged_entities`: Takes a 64-bit value from a helper and processes the four records whose bits are set in its low nibble.
- `update_four_channel_registers`: When a pending byte is set it clears that byte and decrements a counter, then loops over four channels writing per-channel values into a block of hardware registers.
- `configure_dma_channel`: Checks the value in d3 and, if it is 1, calculates a bitmask by shifting d7, sets bit 15 and bit 11 of the DMA control register, and writes the result to the hardware register at a fixed address.
- `update_four_entry_states`: Repeats the per-entry state update for four entries.
- `write_four_entry_channel_values`: Loops over four entries and copies each entry's two stored words into a block of hardware registers at a fixed stride.
- `advance_sequence_step_counter`: Advances a stepped sequence structure by one tick.
- `set_record_status_flag`: Sets the status bit with value four in the byte at offset eleven of the record addressed by a register.
- `step_record_byte_sequence`: Updates a record that plays through a byte sequence.
- `advance_record_sequence_timer`: Decrements the timer field of a record and, when it reaches zero, reads the next entries of a sequence.
- `initialize_records_from_sequence`: For each byte in a sequence block it sets a count field, looks up a value in a table at an offset, and writes a cleared state into a target record.
- `initialize_records_from_sequence_2`: Initializes target records from a sequence block using the same pattern as its neighbouring routine.
- `initialize_paired_records_from_sequence`: Initializes a pair of records for each sequence byte.
- `decode_sequence_command_byte`: Decodes one command byte from a sequence.
- `finalize_sequence_record_state`: Stores two table-derived bytes into the record and runs the sequence's termination check.
- `configure_record_from_command_byte`: Handles a command byte.
- `empty_routine_2`: This routine contains no instructions and performs no operations.
- `compute_clamped_record_offset`: Takes a value from a helper and subtracts a threshold derived from a record field and a stored byte.
- `clear_dma_control_register`: Writes zero to the DMA control register and sets a global variable to a bit mask built from the low six bits of an input value.
- `store_sequence_byte_and_process`: Stores the next sequence byte in a record field and then runs the record's command loop until a terminator is reached.
- `start_record_from_sequence_byte`: Reads a sequence byte and initializes a target record with a pointer derived from it, clearing its state fields and setting its active bit.
- `start_record_from_sequence_byte_2`: Reads a sequence byte and initializes a record with a pointer derived from it, setting its state byte to a high-bit value and marking it active.
- `run_sequence_command_loop`: Reads bytes from a sequence and advances a record's pointer by table-indexed offsets.
- `match_uppercased_name_text`: Calls indirect library routines to open an item, then converts the characters of a name to upper case and compares them with a text buffer that contains the text MAG. before continuing the scan.
- `copy_uppercase_text_and_finish`: Copies a zero-terminated text into a buffer while converting lower-case letters to upper case.
- `call_indirect_and_store`: Calls an indirect routine through a pointer computed from a register, then copies one global variable into another and stores a register value in a third global.
- `run_library_call_sequence_2`: Makes indirect calls through two Amiga library entries at fixed negative offsets from a base pointer, saving and restoring a global value around them.
- `check_library_result_matches`: Calls an indirect library routine and, when it returns a positive value, calls a second library routine with the same arguments.
- `invoke_library_and_check_code`: Calls one indirect library routine and then another, and returns one when the first routine's result equals a fixed code and zero otherwise.
- `set_global_and_return_constant`: Copies one global variable into another and returns the constant one in the upper bits of its result.
- `run_library_check_with_status`: Calls an indirect library routine and, when it returns a positive value, calls a second library routine and a helper.
- `check_library_status_word`: Calls two indirect library routines and returns a word-sized status.
- `store_global_and_call`: Copies one global variable into another, then calls one helper.
- `process_instance_record_state`: Selects a per-instance record from a global table by a bit index, unless that bit is already set in a global mask.
- `invert_and_copy_word_blocks`: Skips leading zero entries in a table to find the source start, then copies words in five-word groups into five destination buffers, storing the first word of each group inverted.
- `check_and_update_two_flags`: Calls the Exec AllocMem routine through a library pointer twice.
- `release_memory_if_flagged`: When a first flag is positive it calls the Exec FreeMem routine through a library pointer.
- `copy_250_bytes_forward`: Calls a helper and then copies 250 bytes one at a time, from a source offset held in a data register into the start of the global block.
- `copy_bytes_from_table`: Walks a table of entries that ends at minus one.
- `increment_global_counter`: Calls one helper, adds one to a global counter, and returns its two input registers combined as a 64-bit value.
- `return_without_action`: Returns immediately without any effect.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2)[a].adf:ip_intro

- `open_and_close_graphics_library`: Program entry routine.
- `setup_copper_and_interrupt_state`: Sets the interrupt enable and DMA control registers, waits for the video beam position and DMA busy bits to clear, and writes a block of eight words with fixed constants.
- `copy_character_rows_for_text`: For each character of a newline-separated text, copies a block of rows from a table indexed by the character value into a destination area.
- `copy_character_rows_for_text_2`: Copies character rows from a table for each character of a newline-separated text, advancing the destination by a fixed stride and moving down a line at each newline.
- `copy_character_rows_for_text_3`: Copies character rows from a table for each character of a newline-separated text into a destination area, advancing by a fixed stride and moving down a line at each newline.
- `pass_stack_argument_through_2`: Returns its eight-byte stack argument unchanged.
- `shift_buffer_on_countdown`: Decrements a countdown.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `inspect_undecompiled_runtime_entry`: The decompiler timed out for this fifteen-instruction routine, so its behaviour is not recovered.
