# Game 2020 (populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02020). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802304 top-level bytes in 62 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 762 of 762 named (0 author symbols), 762 of 762 described (42 by rule, 116 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3 of 3 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 21350 of 21357 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 172931 |
| code | 78980 |
| data | 766022 |
| padding | 12787 |
| unused | 771584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 1 of 2).adf:run | 2 | 458 | 12 |
| run | 2 | 458 | 12 |
| EndCLI | 2 | 182 | 3 |
| LoadWB | 12 | 535 | 30 |
| Disk-Validator | 1 | 386 | 1 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 1 of 2).adf:IP.txt | 2 | 24 | 4 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 1 of 2).adf:CREDITS.EXE | 2 | 167 | 7 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 1 of 2).adf:patch | 1 | 99 | 3 |
| icon.library | 10 | 1421 | 110 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 1 of 2).adf:POPULOUS.II | 6 | 17460 | 570 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 1 of 2).adf:ip_intro | 1 | 132 | 7 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |

## Cases handled

- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 2 of 2).adf:s32-3.pif: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (13888 -> 18800 bytes)
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 2 of 2).adf:s32-1.pif: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (18780 -> 26350 bytes)
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 2 of 2).adf:block3.pak: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (27588 -> 47960 bytes)
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 2 of 2).adf:block1.pak: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (26248 -> 48360 bytes)
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 2 of 2).adf:faces.pak: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (6644 -> 10280 bytes)
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 2 of 2).adf:s32-0.pak: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (47044 -> 71080 bytes)
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 2 of 2).adf:conquest.pak: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (11324 -> 50000 bytes)
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 2 of 2).adf:qaz.pak: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (16988 -> 32000 bytes)
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 2 of 2).adf:awards.pak: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (5696 -> 6912 bytes)
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 2 of 2).adf:s32-2.pif: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (8364 -> 11460 bytes)
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 2 of 2).adf:hands.pak: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (6116 -> 10240 bytes)
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 2 of 2).adf:block2.pak: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (27120 -> 48280 bytes)
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 2 of 2).adf:block0.pak: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (27372 -> 48920 bytes)
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 2 of 2).adf:end.pak: high-entropy data file (7.66 bits/byte), classified as packed data
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 2 of 2).adf:s16-0.pak: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (55568 -> 83040 bytes)
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 2 of 2).adf:judge.pak: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (36412 -> 41495 bytes)
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EB8) A7=an address (A7)=an address, slid to exec.library -an address): 494 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 1 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 1 of 2).adf:CREDITS.EXE: Old ProPack (RNC) (xfd) stream found by ancient scan (11888 -> 26856 bytes) but only 0 of 12 sampled windows are in reference RAM: not used
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 1 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 1 of 2).adf:patch: TetraPack 2.2 (xfd) stream found by ancient scan (392 -> 112 bytes) but only 0 of 1 sampled windows are in reference RAM: not used
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip][a](disk 1 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 1 of 2).adf:CREDITS.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 1 of 2).adf:ip_intro

## Copy protection found

- none found

## Functions


### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 1 of 2).adf:run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_block`: If a pointer is non-zero, this routine copies sixteen words from a source offset in a structure to a destination buffer and then calls another routine.
- `initialize_entity_state_structure_2`: Initializes a structure by zeroing several fields, updating a state byte at offset 32 from a table, and setting a pointer at offset 48 to a value from a table.
- `update_entity_state_and_call_handler_2_2`: Reads a byte from a table at an offset determined by a value in a structure, stores it in the structure at offset 20, and updates the structure at offset 24 with the original value incremented by one.
- `update_entity_state_and_call_handler_2`: Calculates a new index and value based on offsets in a structure, writes a byte to a table at a calculated offset in a0, and then updates several longwords in the structure in a1.
- `update_entity_state_and_call_handler`: Updates a structure in a0 by setting a byte at a calculated index to 10, initializing several longwords at offsets 4, 8, 12, and 16, and conditionally increments a value at offset 4.
- `calculate_and_branch`: Loads the value at offset 84 of a structure into a register, adds one to it, and branches to another routine.
- `test_and_branch`: Tests a longword at offset 40 of a structure and branches to another routine if the value is non-zero, otherwise it branches to a third routine.
- `modify_structure_bitmask_and_branch`: Tests a bitmask at offset 48 of a structure against a constant; if the bit is not set, it sets the upper 24 bits of the value at that offset and branches to another routine, otherwise it compares the value at that offset to 133 and branches to another routine or a third routine.
- `update_entity_attribute_and_conditional`: Copies a value from offset 64 of a structure to offset 108 of the same structure, then calls another routine using parameters derived from the structure and a pointer to a table.
- `update_entity_state_and_clear_attributes`: Updates a specific entry in a table using values from a structure, calls two subroutines, and then clears several fields in a different table at an offset calculated from a structure member.
- `update_structure_and_call_subroutines`: Stores a value from a register into a structure at offset 128 of a1, performs several calls to a routine via a5 using various parameters from a1 and a2, and finally performs an indirect jump using a6.

### run

- `process_linked_list_and_expand_buffer`: Iterates through a linked list to find the maximum value in a specific field, then expands a data buffer by writing a header and copying data blocks.
- `copy_memory_block`: Copies a fixed number of words from a source address to a destination address.
- `initialize_entity_state_structure`: Initializes a structure by writing several values to specific offsets, including a zero value at offset 28, a byte read from a table at offset 30, and a value derived from a bit-shifted register at offset 40.
- `update_entity_state_from_table`: Fetches a byte from a table using an index stored in a structure, stores this byte at offset 20 of the structure, and calculates a new index by incrementing the original value and adding 10, which is then stored at offset 12 of the structure.
- `update_entity_state_and_call_handler_3`: Calculates a target index and a source index based on values within a structure in a1, then copies a byte from a table in a0 to a destination offset in a0 using the calculated indices.
- `update_entity_state_and_call_handler_2_3`: Updates an entity's state by modifying values in a structure pointed to by a0, including setting a byte to 10, copying longwords between offsets, and clearing a longword at offset 12.
- `calculate_and_branch_2`: Loads the constant one into register d1, adds a longword from an offset of 84 in a1 to d1, and branches to another routine.
- `test_and_branch_2`: Tests a longword at offset 40 of a structure in a1 and branches to another routine if the value is non-zero, otherwise it branches to a third routine.
- `modify_structure_bitmask_and_branch_2`: Tests a bitmask at offset 48 of a structure against a constant; if the bit is not set, it sets the upper 24 bits of the value at that offset and branches to another routine, otherwise it compares the value at that offset to 133 and branches to another routine or a third routine.
- `update_structure_field_and_call_handler`: Copies a longword from offset 64 of a structure to offset 108 of the same structure, prepares several parameters in registers, and calls a routine via a function pointer in a5.
- `reset_entity_state_and_update_attributes`: Calculates an index into a table using values from a structure, updates a specific entry in that table, and then clears several fields within a secondary structure.
- `process_entity_data_and_dispatch`: Calls several routines using a pointer in a5, updates a long value at offset 128 of a structure, modifies a value at an offset calculated from a structure member, and performs an indirect jump using a6.

### EndCLI

- `process_linked_list_and_populate_buffer`: Traverses a linked list to determine the maximum value within a specific field and then populates a destination buffer with a header and a sequence of incrementing values.
- `copy_memory_block_2`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.
- `update_structure_and_decrement_counter`: Performs several calls to a routine via a pointer in a5, modifies a value in a structure at a1, and decrements a 32-bit value at an offset in a structure at a0 based on a calculated index.

### LoadWB

- `initialize_workbench_environment`: Sets up the execution environment by restoring patched states and loading Workbench icons.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `load_workbench_icon`: This routine processes a global variable by calling several other routines, performing a double XOR decode on a zero-terminated byte string via a pointer, then stores a result from another routine into a buffer obtained earlier.
- `compare_null_terminated_strings`: Walks two null-terminated strings side by side and stops at the first differing byte.
- `build_node_list_from_table`: Walks a chain of table entries, allocating an 8-byte block for each and storing a converted value from the entry into it.
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
- `forward_args_with_stack_pointer`: Passes its two arguments on to another routine together with the address of a stack location twelve bytes into the caller's argument area, then returns.
- `format_message_and_forward`: Formats text into a 140-byte stack buffer through the exec library's formatting function, then measures the string length.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compare_memory_buffers`: Compares a byte at an offset of a structure in a2 with a constant, and if it matches, enters a loop that compares bytes from two memory locations until a mismatch is found.
- `write_dot_and_four_spaces`: Stores a period followed by four space characters into five consecutive bytes of the buffer passed in through the address register, starting at offset 17.
- `skip_matching_global_prefix`: Advances the buffer pointer past every leading character that matches a global string.
- `return_without_action`: Its body is only a return, so it does no work.

### Disk-Validator

- `run_bit_difference_scan`: Program entry that writes a two-digit number into a text buffer, then runs nested loops that count positions where two table bits differ.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 1 of 2).adf:IP.txt

- `close_libraries_and_exit`: Program entry that calls a chain of routines and, when the zero flag is clear, makes one more call and then invokes a function pointer read from a global table.
- `open_dos_library`: Opens the dos library by name through the exec library's old open call and stores the returned library handle in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 1 of 2).adf:CREDITS.EXE

- `unpack_and_relocate_hunk`: Checks the first longword for a packed-data signature and, when it matches, copies bytes backward within a buffer.
- `read_big_endian_longword`: Reads four consecutive bytes from the address register and combines them into one 32-bit value with the first byte most significant.
- `decode_backward_copy_block`: Reads a bit from a bit-stream routine.
- `return_without_action_2`: Its body is only a return, so it does no work.
- `decode_count_and_repeat`: Calls a reader routine in a loop that counts a value down until a flag breaks it or it reaches minus one.
- `read_mode_dependent_sequence`: For mode two, it calls one routine and then repeatedly calls a reader until it returns zero.
- `spin_until_counter_zero`: Decrements a counter held in a data register until it reaches zero, then returns.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 1 of 2).adf:patch

- `write_color16_register`: Writes to the COLOR16 hardware palette register and calls four unnamed routines.
- `shift_stored_longword_with_sentinel`: Reads the longword just before the address register, writes its low byte into a word at a second address together with the high byte of another data register, and returns the value shifted right one place with the top bit set.
- `shift_bits_with_refill`: Shifts a bit register right once per iteration for the count held in a second data register.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_flag`: Library code that increments a 16-bit field at offset 32 from the structure pointer and clears the flag with value eight in the byte at offset 14.
- `decrement_refcount_and_cleanup`: Library code that decrements the 16-bit count at offset 32 of the structure.
- `unlink_node_and_call_library`: Library code that, when the word at offset 8 is zero, unlinks the structure from its list by joining its two neighbours, then calls a library function and returns the value stored at offset 42.
- `set_pending_flag_return_zero`: Library code that sets the flag with value eight in the byte at offset 14 of the structure, then returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `open_dos_library_or_alert`: Library code that saves a name pointer and a library base in the structure, opens the dos library through the exec vector and stores the returned handle.
- `copy_two_longwords_to_globals`: Library code that copies the two longwords at offsets 34 and 38 of the structure into two global cells, the same cells the open routine fills.
- `call_library_handler`: This library routine simply calls another routine and then returns.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `forward_call_and_return`: Calls one other routine and then returns, with no other work.
- `construct_icon_path`: This library routine builds a file path by appending a specific suffix to a string and stores the result in a local buffer.
- `call_handler_pointer`: This library routine retrieves a function pointer from a data structure and calls it.
- `call_library_handler_2`: This library routine calls another routine and then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `call_handler_pointer_2`: This library routine fetches a pointer from a data structure and calls the routine located at that address.
- `call_library_handler_offset`: This library routine calls another routine with the same first argument and an incremented second argument before returning.
- `call_other_routine`: The routine calls another routine and then returns.
- `call_and_return`: This routine calls another routine and then returns.
- `call_and_return_3`: This library routine calls another routine and then immediately returns to its caller.
- `store_value_in_returned_object`: Library code that calls a library function through a vector in the structure, then stores the second argument into the longword at offset 148 of the pointer it returns.
- `load_and_fill_structure`: Library code that obtains a structure from one routine and asks a second routine to fill the area at offset 140 of it.
- `copy_stack_buffer_into_structure`: Library code that copies ten longwords and eight further fields from a stack buffer into fixed offsets of the structure.
- `return_zero_stub`: The routine is an empty stub that immediately returns the constant zero.
- `pack_structure_fields_and_call`: Copies ten longwords and eight further fields from offset 96 onward of the structure into a local block that starts with a fixed two-word header, then calls another routine with that block.
- `load_workbench_icon_file`: Builds a path with the .info suffix and reads the named icon file into a buffer.
- `read_icon_body_blocks`: Continues loading an icon after its header is checked: it reads image and pointer data, allocates blocks and stores them into the structure.
- `load_icon_remaining_blocks`: Continues the icon loader by reading two values into the structure and then filling in optional pointer fields, reading blocks in four-byte steps.
- `read_icon_entry_array`: Reads a series of entries through a function pointer and stores each one in a pointer array while a remaining-byte counter is above four.
- `forward_call_return_zero`: Calls one other routine and then returns the constant zero.
- `empty_routine_2`: The routine contains four instructions and performs no observable work; it ends immediately with a return.
- `load_icon_file_variant`: Builds a path with the .info suffix in a 260-byte buffer and passes it to further routines to locate the file.
- `read_icon_structure_and_lists`: Continues icon loading once the path is built: it reads the icon structure and then reads the pointer list it refers to.
- `process_icon_optional_lists`: Processes the optional lists of an icon structure: it calls a routine for the first list and then for each entry in the second list and the optional pointer at offset 70.
- `process_icon_pointer_list`: Measures the pointer list at offset 54 of the structure, then processes each entry through function-pointer calls and handles the optional pointer at offset 70.
- `process_null_terminated_pointer_table`: Walks a null-terminated table of pointers, calling a routine for each entry and updating a counter, and invokes the function pointer twice per entry.
- `process_list_items_until_end`: Walks a list from an offset in the structure, calling one routine for each item until the list ends or an item is zero.
- `walk_list_with_library_calls`: Walks a linked list by following the next pointer at offset 8 from the head, calling one routine per node.
- `forward_call_at_offset_140`: Calls another routine with the same first argument and the second argument increased by 140, then returns.
- `allocate_icon_block_or_fail`: Allocates a 168-byte cleared block and sets its field at offset 142 through one routine.
- `decrement_table_counter`: This library code decrements a 16-bit slot counter and returns zero.
- `return_zero_3`: Library code that returns the constant zero.
- `store_pair_in_table_slot`: Decrements a 16-bit slot counter, and when the counter is exhausted it refills the table through a routine.
- `store_pair_in_table_slot_2`: Decrements a 16-bit slot counter and stores two longwords into the 8-byte table entry at the new index, the second value coming from the stack.
- `allocate_icon_record_or_fail`: Allocates a 94-byte cleared block and sets a field at offset 80 through one routine.
- `return_immediately_2`: Library code that performs an immediate return.
- `forward_call_return_zero_2`: Calls one other routine through the library and then returns the constant zero.
- `forward_call_at_offset_78`: Calls another routine with the same first argument and the second argument increased by 78, then returns.
- `find_key_value_after_equals`: Searches a null-terminated list of pointers for an entry whose text matches, using a routine to compare each entry.
- `scan_key_list_for_value`: Walks a pointer list, comparing each entry with a routine, and returns the text after an equals sign when the entry is followed by one.
- `search_key_list_for_value`: Walks a null-terminated pointer list, comparing each entry with a routine, and returns the text after an equals sign when the entry is followed by one.
- `match_delimited_segment_list`: Finds each delimiter character (code 124) in a text, comparing each segment's length with a reference length and then checking the segment with a routine.
- `find_matching_delimited_segment`: Walks a text whose segments are separated by the character with code 124, comparing each segment's length with a reference length and checking a match with a routine.
- `search_delimited_string_segments`: Iterates through a string to locate segments separated by a specific delimiter.
- `compare_delimited_segments`: This library code iterates through a text string using a specific delimiter character to compare segment lengths against a reference value.
- `match_delimited_segments`: Library code that iterates through a text string using a specific delimiter character to compare segment lengths against a reference value.
- `build_copy_label_from_name`: Builds the label for a copy of a name: when the source starts with the text 'copy ', it reads the decimal copy number after that and uses one more than that count.
- `parse_copy_number_from_name`: Checks for a three-character marker at offset 5 of the name and reads the decimal number that follows it, then passes the result on through two routines.
- `parse_decimal_suffix_and_finish`: Reads a run of decimal digits from the text into a number and, when the number is nonzero, checks for a following four-character marker.
- `parse_decimal_copy_count`: Reads decimal digits from the text into a number, then checks for the text ' of ' after them when the number is nonzero.
- `convert_decimal_digit`: Returns the numeric value of a character when it is a decimal digit from zero to nine, and minus one for any other character.
- `return_minus_one`: Does nothing and returns the constant minus one to its caller.
- `call_stored_library_function`: Calls one function through the library vector at offset minus 198 of a stored library base and does not use its result.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `create_block_with_cleanup_2`: Obtains a block from one routine and passes it to a second routine to fill.
- `allocate_record_and_set_count`: Allocates a 96-byte cleared block, stores the value ten in its field at offset 14 and in the word the caller passes, then passes the block to another routine along with the next word.
- `return_zero_value`: This library routine returns zero.
- `check_library_result_equals`: Calls one routine through the library with three arguments and returns the same count when its result equals the fourth argument.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `verify_library_result`: This library code calls another routine and returns its result if the result matches a provided parameter, otherwise it returns zero.
- `empty_routine`: This library routine is empty.
- `divide_longword_with_remainder`: Divides a 32-bit dividend by a 32-bit divisor and returns the quotient and remainder as a pair.
- `run_sixteen_step_division`: Runs sixteen shift-and-subtract steps that divide the 32-bit value in the data registers by the divisor, leaving the quotient and remainder in those registers.
- `multiply_low32_from_halves`: Multiplies two 32-bit operands from the data registers using 16-bit partial products and keeps the low 32 bits of the product.
- `forward_call_to_helper`: Pushes a data register onto the stack, shifts two data registers for a helper, calls the helper, restores the saved register, and returns.
- `call_helper_return_register`: Pushes a data register onto the stack, shifts two data registers for a helper, calls the helper, moves the helper's result into a data register, restores the saved register, and returns.
- `negate_result_on_flag`: Normalizes the signs of two input values, calls a helper, and negates the returned value if the result in the address register is non-zero before returning.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `call_raw_format_routine`: Calls the exec library's formatting function through a vector stored in a global, using the arguments the caller has set up.
- `write_byte_at_pointer`: Writes the low byte of a data register to the memory location held in an address register, then returns.
- `measure_string_length`: Counts the bytes of a null-terminated string using an inverted 32-bit counter and returns the length.
- `find_character_in_string`: Scans a null-terminated string for a given byte and returns a pointer to its first occurrence.
- `return_zero_constant`: Does nothing and returns the constant zero to its caller.
- `copy_null_terminated_string`: Copies bytes from the source string to the destination, including the terminating zero byte, and stops after the zero.
- `append_string_to_buffer`: Advances to the terminating zero of the destination string, then copies the source string, including its zero byte, from that point onward.
- `compare_strings_up_to_count`: Compares two strings for at most the given number of bytes.
- `return_one`: Library code that returns the constant one.
- `scan_string_to_limit`: Walks a null-terminated string for at most the given number of bytes.
- `call_and_return_2`: Library code that calls another routine and then returns.
- `copy_string_with_padding`: Copies up to a limit of bytes from the source string into the destination.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_empty_list_header`: Initializes a list header in place: the first longword points just past itself, the second is cleared, and the third points back to the header, which gives an empty list.
- `build_node_from_parameter`: Returns one when the parameter is zero.
- `process_node_list_entries`: Walks a linked list through the next pointer at offset 12.
- `verify_list_entries`: Calls two routines for each node of a linked list whose next pointer is at offset 12.
- `return_zero_2`: This library routine performs no operations and returns a value of zero to the caller.
- `allocate_node_with_pointer`: When the count argument is nonzero, allocates a 20-byte cleared block and initializes it through one routine.
- `return_error_value`: This library routine returns a value representing an error.
- `build_node_chain`: Builds a chain of cleared 16-byte nodes linked through the pointer at offset 12, each with a data block sized from the byte at offset 7 times four and stored at offset 8.
- `build_linked_node_chain`: Builds linked nodes one at a time, each allocated through one routine and linked from the previous one through offset 12, with a data pointer stored at offset 8.
- `build_node_chain_from_stack`: Walks a chain of nodes whose next pointers come from a stack argument, allocating each node and its data pointer through routines.
- `return_negative_one`: Library code that returns the constant value of all bits set to one.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 1 of 2).adf:POPULOUS.II

- `capture_dos_directory_and_streams`: Records the current directory and the standard input and output handles from the DOS library in global cells.
- `store_global_and_call_pair`: Stores a pointer 92 bytes into the structure held in the address register into a global cell, then calls two library functions through the library vector with no arguments.
- `call_two_library_functions`: Calls two library functions through the library vector in sequence, with no arguments, and then returns.
- `clear_palette_and_run_setup`: Clears all 32 color registers starting at COLOR00 to zero, then calls a series of routines, one of them conditional.
- `save_vectors_and_mark_task`: Clears a flag and retries a lookup of the current task through the exec library until it succeeds, then sets a field of that task to minus one.
- `restore_vectors_and_enable_interrupts`: Copies saved interrupt vector values from a table back into low memory, then writes the DMA control and interrupt enable registers with the set bit, which enables the bits it names.
- `return_immediately_3`: This routine is empty and performs no operations before returning to its caller.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `swap_two_global_pairs`: Exchanges the shared global cell with two other global cells, one after the other, using temporary cells to hold the originals.
- `install_handler_and_enable_vblank`: Saves one global cell, stores the handler address into two global cells, then writes the interrupt enable register with the set bit and the vertical-blank bit.
- `handle_interrupt_request_2`: Saves d0 to the stack, reads the interrupt request register, and if bit five is set, it saves registers to the stack, calls another routine, increments four different values in a table, restores registers, and either returns or calls another routine based on a test of a value from the stack.
- `acknowledge_vblank_request`: Writes the vertical-blank bit to the interrupt request register, which clears that pending request.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `setup_two_buffer_pointers`: Calls two setup routines and stores the address returned by each, offset by eight bytes, into a pair of global cells.
- `build_copper_list`: Fills a block with a copper list: it writes a fixed set of register moves, chooses one of two display-window settings from a global flag, and then writes a run of color register moves starting at COLOR00.
- `install_handler_and_enable_interrupts`: Saves one global cell, stores a handler address into another global cell, then writes the constant 49160 into the interrupt enable register with the set bit.
- `swap_saved_value_enable_interrupts`: Swaps one global cell with a saved cell, then writes the constant 49160 into the interrupt enable register.
- `update_state_from_serial_code`: Updates a state table from a code byte read from the serial data register.
- `clear_table_entry_count_down`: Sets one byte of a table, indexed by the argument, to zero and decrements a counter when it is positive.
- `open_graphics_and_intuition_libraries`: Opens two libraries by name through the exec library, records one flag bit from the first library's base, and stores the address of a routine into a vector slot of the second library's base.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `swap_buffers_and_update_state`: Swaps the values of two global variables, toggles a flag via an XOR operation, reads a word from a global table at a computed offset into a global variable, and clears a register.
- `wait_for_flag`: The routine enters a loop that continues as long as a specific global variable remains zero.
- `update_mouse_position_from_counters`: Reads the mouse counter bytes from the joystick data register, works out the change since the last reading with wraparound, and adds it to the X and Y position values.
- `poll_mouse_button_and_latch`: Writes the pot port output register, then checks the left mouse button bit in the input register.
- `reset_button_press_state`: Clears the press flag and the press counter, then calls one routine.
- `read_mouse_counter_bytes`: Reads the mouse counter word from the joystick data register and stores its low byte and high byte in two global cells.
- `update_sprite_header_position`: Writes the high and low halves of two addresses into fields of a block that a global refers to.
- `set_global_var_pointers`: The routine checks a global variable against an input word and, if different, stores the address of a global variable plus the input word into two separate structures.
- `reset_interrupt_state`: The routine writes constants to a global variable and to several other global locations, then overwrites the global variable with a pointer to another routine, calls that routine, masks the global variable with a bitmask, and finally writes a constant to the global variable.
- `swap_saved_value_enable_serial`: Swaps one global cell with a saved cell, then writes the constant 34816 into the interrupt enable register with the set bit and the receive-buffer bit.
- `compute_serial_period_value`: Chooses one of two clock constants according to whether a flag is nonzero, divides it by a global value, and stores the quotient minus one in a global cell.
- `absolute_difference_or_abort`: Returns the absolute difference between two short global index values.
- `reset_buffer_indices`: Clears two global index cells to zero.
- `store_received_byte_in_ring`: When the receive-buffer request bit is set, it stores the received byte at the write index of a ring buffer and advances that index, wrapping it at the end.
- `return_argument_unchanged`: Returns the value passed on the stack without changing it.
- `nop_padding`: This routine is empty and contains no executable instructions.
- `return_from_exception`: Returns from an interrupt or exception by restoring the processor status register and program counter from the stack.
- `check_interrupt_and_clear`: Pushes d0 to the stack, checks if a specific bit is set in the intersection of two interrupt registers, and if it is not set, clears a 16-bit word at a specific location and returns from an interrupt.
- `pop_stack_and_load_address`: pops a word from the stack into d0 and then pushes the contents of a specific memory address onto the stack before returning.
- `read_bytes_from_ring_buffer`: Copies bytes from a ring buffer into the caller's buffer, advancing the read index and wrapping it at the end.
- `copy_ring_bytes_until_limit`: Copies bytes out of a ring buffer into the destination, advancing the read index with wrap.
- `copy_ring_bytes_counted`: Copies a counted number of bytes from the ring buffer to the destination, advancing the read index with wrap.
- `return_stack_value_3`: This routine is empty and simply returns the value currently residing at the top of the stack.
- `send_bytes_over_serial`: Writes a counted run of bytes to the serial data register whenever bit 13 of the serial status register is set.
- `send_counted_serial_bytes`: Writes bytes to the serial data register while the transmit-ready bit is set in the status register, counting down the count and setting a flag in the structure as it goes.
- `call_close_work_bench`: Library stub: loads its arguments and the intuition library base, calls CloseWorkBench and returns its result to the caller.
- `open_fixed_size_screen`: Clears a flag and fills a block of global cells with constant screen values, including a width of 320.
- `call_close_screen`: Library stub: loads its arguments and the intuition library base, calls CloseScreen and returns its result to the caller.
- `close_library_twice`: Calls the exec library's close-library function twice in succession through the vector stored in a global cell.
- `return_from_subroutine`: This routine is empty and performs no operations before returning to the caller.
- `reset_task_priority_and_blitter`: If a global variable is non-zero, it sets the task priority to zero and disowns the blitter, then clears that global variable before returning.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `claim_blitter_and_wait`: Repeatedly calls one routine until it returns zero, clearing a global flag each time.
- `forbid_and_claim_blitter`: Calls the exec library's forbid function, then the graphics library's own-blitter function, and returns.
- `permit_and_release_blitter`: Calls the exec library's permit function, then the graphics library's disown-blitter function, and returns.
- `run_polling_event_loop`: Repeatedly checks several global flags and calls many routines, branching on a state value and updating memory bytes along the way.
- `dispatch_indirect_call`: Checks two global variables and performs an indirect call using an offset stored in a global variable.
- `run_four_step_table_dispatch`: Runs four steps, each picking a routine from a global table entry chosen by index cells, calling it with the step number, and then calling one more routine.
- `check_point_in_bounds`: Returns early through a routine when one flag is set; otherwise, while another flag is set, it tests whether a point lies inside a rectangle given by four global bounds and calls a routine if so.
- `process_global_entry_table`: Iterates over the entries of a global table, deriving values from each entry's index and record, and calls routines for each one, sometimes calling a second routine several times.
- `compute_grid_column_index`: Converts a pair of coordinate values into a column index by subtracting offsets and shifting.
- `check_table_entry_and_act`: Computes a table index from a register and a global index cell, and unless a flag and a counter block it, calls routines for the matching entry when the table byte is positive.
- `scan_aligned_position_entries`: Returns zero unless a flag is set.
- `step_through_entries_by_32`: Steps a position value forward in increments of 32 until it reaches a stored target or passes a limit.
- `return_zero_4`: Returns a zero value to the caller.
- `store_adjusted_record_bytes`: When a global pointer is set, reads two bytes from the structure it points to, subtracts four from each, stores them in two global cells, and calls one routine.
- `scan_entries_and_store_offsets`: Returns zero unless a flag is set.
- `step_entries_for_flagged_match`: Steps through table entries in increments of 52 until it reaches the stored entry or passes a limit.
- `return_zero_value_2`: This routine performs no operations and returns zero to the caller.
- `store_value_shift_previous`: Stores the value passed in the address register into a global cell.
- `dispatch_by_table_value`: Calls routines according to a value read from a global table: one routine for each of four specific values, one more when a separate global equals twelve, and one final routine on every call.
- `xor_source_into_row_block`: XORs bytes from a source buffer into a destination buffer laid out in rows of forty bytes.
- `interpret_packed_command_stream`: Interprets a stream of command bytes, where those with the high bit set select handlers that write a three-byte record or copy a zero-terminated string into the output.
- `call_two_routines_and_return`: Calls two other routines in sequence and then returns.
- `execute_recursive_call`: This routine calls another routine and then returns.
- `call_two_subroutines`: This routine calls two other routines and then returns.
- `call_subroutines`: Calls two other routines and then returns.
- `write_high_nibble_hex_digit`: Stores one digit character for the high nibble of the byte it receives, using a number from zero to nine or a capital letter from A to F.
- `convert_nibble_to_ascii`: Manipulates a byte to transform a nibble into its corresponding ASCII character representation for a hexadecimal digit.
- `render_text_with_control_codes`: Walks a null-terminated text and passes each printable character to one routine.
- `write_masked_bitplane_column`: Processes five rows of a table byte: each byte is rotated by a shift taken from a global, then merged into four bitplanes 8000 bytes apart, with rows 40 bytes apart.
- `return_without_action_3`: The routine contains only a return instruction, so it does nothing when called.
- `sample_beam_position_and_dispatch`: Reads the video beam position register, stores and clears one global, and updates two bytes from a 64-entry word table indexed by a value computed from the call.
- `process_glyph_and_interpolate`: Increments a byte at an offset from a pointer and clamps it between 48 and 57, calculates a value based on a global variable and the byte, then calls three different routines to interpolate tables, copy word blocks, and call library vectors.
- `call_other_routine_2`: The routine performs a single call to another routine and then returns.
- `dispatch_indexed_handler`: Calls five setup routines, runs one more only when a flag is zero, then makes an indirect call through a handler table indexed by a value returned from another routine.
- `run_setup_then_dispatch`: Calls three setup routines, then makes an indirect call through a handler table selected by the value returned from a final routine.
- `patch_marker_byte_then_dispatch`: Calls five routines, sets a flag word and a byte, then scans a byte string for the marker value and replaces the first matching count-limited occurrence with the next character value.
- `skip_to_string_terminator`: Advances a pointer through a byte string until it reaches the zero terminator, then returns.
- `empty_routine_3`: The routine contains four instructions and performs no observable work.
- `clamp_stat_pair_for_labels`: Recalculates two stored values, each limited to the range 1 to 99 after an adjustment chosen by a flag, then copies a constant string into a buffer and computes a third value for another routine.
- `reset_display_state_and_dispatch`: Calls three routines, sets a stored word to 40, clears one byte and a global, then calls a routine and makes an indirect call through a handler table selected by a return value.
- `rewrite_flag_glyphs_in_text`: Walks a text buffer and rewrites each occurrence of the characters y and z according to the low bit of a 16-bit mask, shifting the mask on every character.
- `repeat_flag_glyph_rewrite`: Repeats the same text pass as a sibling routine: it rewrites characters y and z in a buffer from a shifted 16-bit mask, then checks coordinate bounds to decide whether to loop again before dispatching through a handler table.
- `dispatch_handler_from_table`: Calls one routine to obtain an index, then makes an indirect call through a handler table at that index.
- `call_handlers_from_table`: Walks a grid of flag bytes, six per row, alongside a table of pointers.
- `build_menu_string_table`: Clears a block of twelve longword slots, then fills them with pointers to consecutive null-terminated strings from a text area, stopping early when a count runs out.
- `rewrite_mode_glyphs`: Selects one of two global blocks by a flag, then scans a text buffer for the characters c and y and rewrites them according to a mode value and two counters.
- `render_flags_and_dispatch`: Rewrites the characters y and z in a text buffer using the low bits of a 16-bit mask that is shifted for each character, and places a g at a position clamped to 15 wherever h appears.
- `mark_h_and_flag_glyphs`: Walks a text buffer, rewriting y and z from a shifted 16-bit mask and replacing each h with g at a clamped offset.
- `run_routine_until_threshold`: Repeatedly calls a routine and reads its returned value, continuing while a mode value is 2 and the returned value stays between 3 and 26.
- `append_formatted_char`: Clamps a character to a printable range and appends it to a string by shifting existing characters.
- `append_uppercase_char`: Clamps a character to the printable range from space to y, converts lowercase to uppercase, and appends it to a text string, shifting the string left first when it exceeds 33 characters.
- `find_v_w_delimiter_2`: The routine receives a pointer in a0, steps back from the byte after the pointer until it finds the character 'v', then steps forward from that position until it finds the character 'w', and then calls another routine.
- `edit_text_input_line`: Accepts keyboard characters into a text buffer with a maximum length.
- `edit_text_input_loop`: Runs the same kind of text entry loop as its sibling routines: a character is read, carriage return ends entry, backspace shifts the buffer left, and other characters are inserted at the cursor up to the length limit.
- `edit_text_input_redraw`: Accepts characters into a text buffer with a length limit, handling carriage return, backspace and insertion at the cursor.
- `edit_text_input_backspace`: Accepts characters into a text buffer, where backspace deletes the character before the cursor by shifting the remaining text.
- `edit_text_input_insert`: Accepts characters into a text buffer, inserting each one at the cursor and shifting the remaining text right while space remains.
- `call_other_routine_and_return_zero`: The routine calls another routine and then returns zero.
- `copy_string_with_marker`: Copies a fixed number of characters from a source string into a buffer, substituting k when the source ends early and m at one marked position when a flag bit is set.
- `reset_global_flag`: The routine clears a global variable to zero and then calls another routine.
- `toggle_last_match_and_count`: Checks a flag and a bounded range of text bytes, validating each character through a lookup table.
- `format_string_with_placeholders`: Copies a format string from a table and expands brace-delimited placeholders by substituting them with strings from an argument list.
- `append_or_delete_input_char`: Handles one keyboard character for short name entry: space, return, backspace and capital letters are accepted, with a ten-character limit.
- `shift_buffer_and_append_char`: Shifts a text buffer left by one byte, then clears the last character for backspace or stores a printable character at the end and terminates the string.
- `store_printable_char_at_end`: Stores the character when it is above the control code 31, writing it one byte before the buffer end pointer and terminating the string at that pointer.
- `decrement_counter_and_call`: Decrements a global counter while it is positive.
- `call_two_routines_in_sequence`: Calls two other routines one after the other and returns, with no other operations.
- `draw_text_glyphs_to_planes`: Draws a text string into a bitplane buffer one character at a time, using a 32-byte glyph entry per character and writing across eight plane positions at fixed offsets.
- `process_flagged_tile_entry`: If a flag word at the given index is non-zero and a byte from a second table is positive, it records the index, calls several routines, invokes an indirect handler chosen from a table, then runs a second indirect handler.
- `increment_and_branch_3`: Increments a 16-bit value in a global variable, uses the lower two bits of the new value to load a byte from a table, and then branches to one of several locations.
- `perform_blitter_sequence`: Increments a global counter, calculates several destination addresses, and enters a loop that performs three blitter transfers using values from a table until a terminator is reached.
- `branch_to_offset`: returns immediately and then performs a relative branch to a later instruction.
- `modify_and_branch_2`: Performs bitwise operations on a global variable, uses the result to calculate a new value, and then branches to one of two locations.
- `update_counter_and_dispatch`: If a global word is not negative one, it sets it to 3388, otherwise it increments the global word by 4 and potentially updates it using a value from a table, then calls another routine.
- `build_five_text_pointers_2`: Skips a number of text strings computed from a global index, then records pointers to five consecutive strings in a table.
- `process_buffer_until_done`: Clears several global variables and a pointer, then processes the character buffer pointed to by a global, toggling a flag and advancing an index on alternating passes, until a completion flag is set.
- `render_packed_text_run_2`: Runs the same packed-entry text drawing loop as its sibling routines: each 16-bit entry is split into two 6-bit fields that become coordinates, drawn through a routine call, with a 96-unit character limit and a timer that starts a fallback pattern.
- `render_packed_text_run_3`: Draws a packed text buffer with the same entry decoding as its sibling routines, splitting each 16-bit entry into 6-bit coordinate fields and drawing it through a routine call.
- `render_packed_text_loop`: Runs the packed text drawing loop as a jump-based variant: it draws each 16-bit entry from a buffer by splitting it into 6-bit coordinate fields, updates a timer, and ends with a stored return when the buffer is exhausted.
- `copy_planar_tile_rows`: Copies 96 rows of a 12-byte source block into a four-plane bitmap, writing three bytes into each plane per row.
- `load_display_state_and_dispatch`: Derives a display value from a state record, clamping one field to 8 and offsetting a table pointer from it.
- `copy_and_dispatch_data`: Copies blocks of data to a destination and then executes a series of function pointers retrieved from a table.
- `iterate_table_dispatch`: Runs three times, each time combining a value from a short table with a byte from a buffer to pick a handler from a table.
- `nested_double_call_loop`: Runs a nested loop of two outer iterations and three inner iterations, calling two routines in each inner pass.
- `copy_word_rows_clipped`: Copies 32 rows into four bitplanes, writing one 16-bit word per plane per row with planes 8000 bytes apart and rows 40 bytes apart.
- `copy_word_rows_to_planes`: Copies rows of four source words into four bitplanes, writing one 16-bit word per plane per row with planes 8000 bytes apart and rows 40 bytes apart.
- `process_blitter_tile_grid`: Programs the blitter registers and waits for it to finish, then walks an 8-by-8 grid of table entries.
- `blit_tile_pair_entries`: Checks two adjacent table entries and, for each non-zero entry, programs the blitter with a computed source address and size and waits for it to finish.
- `setup_blitter_parameters`: Looks up blitter source values from a table for an index, clips the horizontal and vertical positions to a 64-by-64 area, and stores the adjusted pointer, modulo and size values.
- `configure_blitter_and_traverse_list`: Configures the blitter control and mask registers with specific constants and then traverses a linked list of structures via a pointer in a4, calling another routine for each element.
- `adjust_pointers_and_call`: Increments two registers by 2, calls another routine, decrements those same two registers by 2, and returns.
- `execute_blitter_transfers_conditional`: The routine calculates a size and offset, then performs up to four blitter transfers to different memory areas by writing to hardware registers and waiting for the DMA controller to finish, provided the offset is within a specific range.
- `perform_multi_block_blitter_transfers`: Checks if a value derived from a pointer in a6 is within a specific range, then reads a sequence of offsets from a table in a3 to perform a series of blitter transfers.
- `write_blitter_words_from_table`: Waits for the blitter to become idle, programs its control registers, then writes eight words from a table to the blitter data register, advancing a row offset on each word.
- `program_blitter_control_block`: Waits for the blitter to become idle, writes a fixed block of control words, then writes pointer and size values taken from two global parameters followed by a final control word.
- `clear_table_and_iterate`: Clears 4096 longwords starting at a global buffer, then walks a short table until the sentinel value -99 is reached, calling three routines for each entry.
- `step_random_state_until_break`: Advances two 15-bit linear congruential state values, calling a routine with each new value for up to eight steps.
- `update_tile_flag_bits`: Increments a global counter and, when the result is non-negative, walks neighbouring tile positions, calling routines and reading returned coordinates.
- `decrement_tile_flag_bits`: Increments a global counter and returns early when a position is out of range.
- `decrement_tile_flag_entry`: Decrements the state byte at a tile entry, masks it to three bits, and updates the flag bytes of three neighbouring tiles at fixed offsets.
- `decrement_tile_flag_entry_2`: Decrements the state byte at a tile entry, masks it to three bits, and updates the flag bytes of neighbouring tiles at fixed offsets, decrementing each value and clearing bits where they wrap.
- `decrement_single_flag_neighbour`: Decrements the state byte at a tile entry, masks it to three bits, and updates the flag byte of one neighbouring tile when it lies inside the 64-by-64 grid.
- `decrement_masked_flag_byte`: Decrements the byte at the given pointer, masks it to the low three bits, clears bit 0 of the following flag byte, and increments a global counter.
- `return_stack_argument`: Returns the value that was passed on the stack, without doing any other work.
- `check_tile_position_bounds`: Tests whether two coordinates lie inside the 64-by-64 tile grid and returns both values unchanged.
- `check_neighbour_links_for_marker`: Checks the four neighbouring tile positions that lie inside the 64-by-64 grid.
- `return_stack_coordinate_pair`: Returns the two values passed on the stack as a pair, without any other work.
- `check_three_neighbour_markers`: Checks three neighbouring tile positions inside the 64-by-64 grid for a stored value, and walks each linked chain for the marker value 26.
- `concatenate_stack_values`: Combines two short values from the stack into a single 32-bit return value.
- `check_two_neighbour_markers`: Checks two neighbouring tile positions inside the 64-by-64 grid for a stored value, and walks each linked chain for the marker value 26.
- `concatenate_stack_values_2`: Returns a 32-bit value formed by concatenating two 16-bit values from the stack.
- `check_one_neighbour_marker`: Checks the tile position to the right of the input coordinates, if it lies inside the 64-by-64 grid, by comparing a stored byte and walking a linked chain for the marker value 26.
- `return_stack_contents`: This routine returns the two 16-bit values currently residing at the top of the stack.
- `return_stack_contents_2`: This routine returns the two 16-bit values currently residing at the top of the stack.
- `combine_two_shorts`: Calls one routine to obtain a short result, calls another routine, and returns the first result in the high half and the value left in the data register by the second call in the low half.
- `combine_two_shorts_2`: The routine calls another routine twice and returns a 32-bit value formed by concatenating the short result of the first call in the high word with the short result of the second call in the low word.
- `nested_loop_fill_bytes`: Runs a nested loop of 17 by 17 passes, calling one routine on each pass and storing a fixed byte through a pointer each time.
- `decrement_and_check_neighbours`: Decrements a signed byte in a grid structure and checks its eight neighbouring bytes for underflow or a small difference from the new value.
- `increment_and_check_neighbours`: Increments a signed byte in a grid structure and checks its neighbouring bytes for signed differences from the new value.
- `store_index_and_maybe_reset`: Stores the input value in a global variable and calls one routine.
- `clear_counter_and_call`: Clears a global counter to zero and then calls one other routine.
- `store_index_and_maybe_reset_2`: Stores the input value in a global variable and calls one routine.
- `reset_global_counter`: Sets a global variable to zero and calls another routine.
- `fill_tile_map_from_table`: Walks a rectangular range of tile positions, reading a byte from a lookup table for each one.
- `call_callback_in_nested_loops`: Calls the routine held in a global function pointer once for each position of a 64-by-64 grid, making 4096 calls in total.
- `call_when_flag_bits_match`: When a counter is zero and a flag byte for a structure entry has bit 1 set and bit 0 clear, it calls one routine.
- `call_routine_return_stack_argument`: Calls one other routine, then returns the value that was passed on the stack unchanged.
- `return_stack_argument_2`: Returns a 64-bit value from the stack.
- `zero_global_table`: Writes zero into successive pairs of words across a global table until it reaches a fixed end address.
- `compare_low_bits_and_call`: Walks two pointers in opposite directions over 2048 steps, comparing the low three bits of each pointed-to byte.
- `call_routines_until_zero`: Calls one routine, then repeatedly calls two others until the short value left in a data register by the last call is zero.
- `build_random_event_list`: Repeatedly picks a random index, looks up a 16-bit candidate in a table, and checks its flag bits and grid cell.
- `find_or_allocate_event_slot`: Searches a chain of 14-byte records for a type-22 entry whose stored key matches a value, and advances that key to the next value in a fixed list.
- `allocate_or_advance_event_slot`: Walks a chain of 14-byte records for a type-22 entry.
- `populate_event_record`: Writes a type-22 record into a 14-byte structure, filling its fields from register values, a global variable and a 16-bit table value, and setting two fields to 128.
- `call_other_routine_in_loop`: The routine calls another routine once, then repeatedly calls two other routines in a loop until a short value returned in D1 is zero.
- `sample_random_table_indices`: Draws random values and uses a global value as a modulus to set the loop count.
- `claim_event_slot_for_key`: If a table entry for a byte index is non-zero, its flag bit is clear and no record is linked to it, it searches for the first free 14-byte record.
- `find_or_allocate_type_24`: Searches a chain of 14-byte records for a type-24 entry whose stored key matches a value, and advances that key to the next value in a fixed list.
- `allocate_or_advance_type_24`: Walks a chain of 14-byte records for a type-24 entry.
- `populate_type_24_record`: Writes a type-24 record into a 14-byte structure, filling its fields from register values, a global variable and a table value, and setting two fields to 128.
- `advance_timed_records`: Walks the table of 14-byte records and updates each active one.
- `try_spawn_at_offset`: Adds small offsets to two input coordinates.
- `call_two_routines_return_one`: Calls two other routines and returns 1.
- `return_zero`: Returns 0 without doing any other work.
- `walk_record_chain_clearing_flags`: Walks a chain of 14-byte records starting from a table entry for a byte index, following each link.
- `walk_record_chain_clearing_flags_2`: Follows a chain of 14-byte records until it reaches one in the middle address range, then clears that record's flag byte at offset 12 and calls one routine.
- `clear_chained_record_flag`: Follows a chain of 14-byte records through a link field and stops at a range boundary.
- `plot_pixel_or_line`: Sets one pixel in a four-plane 320-by-200 bitmap, writing one bit into each plane according to the low four bits of a colour value, after a bounds check.
- `double_and_call_twice`: Doubles a short input value and passes it to one routine, then calls the same routine again with no argument.
- `call_and_return_inputs`: Calls one other routine and returns the two register inputs combined into one 64-bit value, unchanged.
- `extract_bit_from_table`: Reads a byte from a table at an index computed from the high and low parts of a 16-bit input, shifts it by the bit position given by the low three bits of the input, and returns the least significant bit.
- `plot_pixel_with_bounds`: Sets or clears one pixel in a four-plane 320-by-200 bitmap, writing one bit into each plane according to the low four bits of a mode value.
- `plot_pixel_without_bounds`: Sets or clears one pixel in a four-plane bitmap, writing one bit into each plane according to the low four bits of a mode value.
- `clear_bit_in_four_planes`: Clears one bit, selected by the low three bits of a register value, in the byte at a pointer and in the three bytes at the same offset in the other three planes, 8000 bytes apart.
- `set_bit_and_clear_others`: sets a bit in a byte at a pointer using d0, then clears the same bit in three other bytes at offsets 8000, 16000, and 24000.
- `clear_bit_and_set_one`: clears a bit in a byte at a pointer using d0, then sets the same bit in a byte at an offset of 8000 from that pointer, and clears it at offsets 16000 and 24000.
- `set_two_bits_and_clear_others`: sets a bit in a byte at a pointer and a bit in a byte at an offset of 8000 from that pointer, then clears the same bit in two other bytes at offsets 16000 and 24000.
- `clear_bits_and_set_one`: clears a bit in a byte at a pointer and in a byte at an offset of 8000, then sets the same bit in a byte at an offset of 16000 from that pointer and clears it at offset 24000.
- `set_two_bits_and_clear_others_alt`: sets a bit in a byte at a pointer and a bit in a byte at an offset of 16000 from that pointer, then clears the same bit in two other bytes at offsets 8000 and 24000.
- `clear_and_set_bits`: clears a bit in a byte at a pointer, sets bits at offsets 8000 and 16000, and clears a bit at offset 24000 using the index in d0.
- `set_and_clear_bits`: Sets bits at offset 0, 8000, and 16000 of the structure in a1 using the index in d0, then clears a bit at offset 24000.
- `clear_and_set_bits_alt`: Clears bits at offset 0, 8000, and 16000 of the structure in a1 using the index in d0, and sets a bit at offset 24000.
- `set_and_clear_bits_alt`: Sets bits at offset 0 and 24000 of the structure in a1 using the index in d0, and clears bits at offsets 8000 and 16000.
- `toggle_bits_mixed`: Clears a bit at offset 0, sets bits at offsets 8000 and 24000, and clears a bit at offset 16000 using the index in d0.
- `set_bits_mixed`: Sets bits at offsets 0, 8000, and 24000 of the structure in a1 using the index in d0, and clears a bit at offset 16000.
- `write_blitter_setup_when_flagged`: When a flag table entry for the current structure is non-zero, it calls one routine and, if a word that routine points to is zero, writes a fixed sequence of eight blitter setup words into the global table.
- `compare_record_bytes_and_call`: Compares two byte fields of a record using a comparison selected by a flag table entry, then calls one routine.
- `empty_routine_5`: This routine contains four instructions.
- `run_linked_entry_handlers`: Walks a linked chain of table entries, calling the handler stored in each one.
- `walk_entry_chain_with_handlers`: Walks a chain of table entries from a given start, calling a routine for each entry and handling marker-valued entries through a flag byte on the record.
- `dispatch_object_handlers`: Calls a routine to process a linked list of entities, then calls another routine to resolve data from a table using a value stored at an offset from a pointer in a3.
- `validate_and_dispatch_event_with_bounds`: Checks if a set of coordinates falls within a boundary defined by a global structure, then updates a global pointer and flag if the constraints are met.
- `validate_and_dispatch_event`: Validates a set of coordinate and boundary constraints against a global structure, updates a pointer and a flag if conditions are met, and then conditionally calls a function pointer or specific subroutines based on bit flags at offset 13 of a structure in a3.
- `dispatch_linked_handlers_skip_known`: When a word argument is below 1330 and a multiple of four, it increments a counter in a global table and walks a linked list of table entries.
- `call_indirect_by_flag`: Selects one of two global blocks according to a flag value in a record, calls the routine stored eight bytes after the chosen block, and returns its two register inputs combined.
- `call_indirect_if_flag_set`: If a global counter is non-negative, calls a routine through a function pointer stored in memory.
- `draw_shifted_sprite_planes`: Draws five-byte sprite row groups into four bitplanes at 40-byte row stride, clipping rows to the 200-line screen.
- `draw_sprite_planes_clipped`: Draws five-byte sprite row groups into four bitplanes after a vertical position check, shifting each row by a sub-byte amount and merging it through a mask.
- `or_sprite_rows_into_planes`: Merges sprite row data into four bitplanes without shifting.
- `shift_sprite_rows_into_planes`: Shifts each five-byte source row by a bit amount taken from a register, merges the shifted bytes with the destination through a mask, and writes four interleaved plane words per row, advancing one row per pass.
- `program_blitter_sprite_copy`: Clips a sprite position to the 200-line screen, then programs the blitter for a row-sized copy.
- `program_blitter_sprite_copy_2`: Clips a sprite position to the screen and programs the blitter for a row copy, choosing source values from a table by a row index.
- `program_blitter_sprite_row`: Programs the blitter for a row copy after a line check, choosing source values from a table by a row index.
- `program_blitter_cell_copy`: Programs the blitter for a single cell copy within the screen column range, using table values selected by a row index.
- `return_without_action_4`: The routine contains only a return instruction, so it does nothing when called.
- `program_blitter_mask_copy`: Programs the blitter for a masked copy.
- `program_blitter_mask_fill`: Programs the blitter using a second table of control values selected by the low nibble of a register.
- `program_blitter_row_copy`: Programs the blitter for a row copy, choosing control words from a table by a shifted register value.
- `program_blitter_sprite_shifted`: Clips a sprite position to the screen, chooses blitter control values from a table by a row index and a shift value, and programs the blitter for a shifted sprite copy.
- `program_blitter_shifted_sprite_cell`: Calculates blitter control words and destination addresses for a sprite cell based on a row index and horizontal shift value.
- `program_blitter_sprite_cell_shifted`: Programs the blitter for a shifted sprite copy within the screen column range, choosing control values from a table by a row index and a shift value.
- `program_blitter_sprite_cell_shifted_2`: Programs the blitter for a shifted sprite cell, choosing control words from one of three table pairs by a row index and shift value.
- `program_blitter_mask_copy_2`: Programs the blitter for a masked copy using a table pair selected by the low nibble of a register.
- `program_blitter_mask_fill_2`: Programs the blitter using a table pair selected by the low nibble of a register, with mask words set to two.
- `program_blitter_mask_copy_3`: Programs the blitter for a masked copy using a table pair selected by the low nibble of a register, with zeroed mask words.
- `program_blitter_sprite_row_2`: Programs the blitter for a sprite row copy using a table pair selected by the low nibble of a register, with mask words set to the same constant.
- `program_blitter_sprite_cell_shifted_3`: Programs the blitter for a shifted sprite cell, choosing control words from a table by a row index.
- `extract_and_mask_value`: Calls one other routine and stores its 32-bit result in a global variable.
- `multiply_split_32bit_values`: Multiplies the low halves of two 32-bit register values and adds the signed cross products, shifted into the upper half, to form a 64-bit result.
- `parse_signed_decimal_text`: Reads a signed decimal number from a text pointer, skipping spaces and tabs and accumulating digits.
- `skip_spaces_before_number`: Skips spaces and tabs in a text pointer, then reads digits and a minus sign, returning at a line end or at a repeated minus sign.
- `skip_digits_and_spaces`: Skips leading spaces and tabs in a text string, then consumes digits and a minus sign, returning at a line end.
- `skip_digits_and_spaces_2`: Skips leading spaces and tabs in a text string, then consumes digits and a minus sign, returning at a line end.
- `copy_sixteen_word_blocks`: Copies 500 blocks of 16 words from a source pointer to a destination pointer, one word at a time, and returns its two input registers combined.
- `clear_zeroed_struct_array`: Writes zero into eight longwords per iteration for 1000 iterations, clearing a fixed-size array of records from a pointer.
- `lookup_signed_table_value`: Looks up a signed 8-bit value from a table indexed by two shifted, sign-adjusted 16-bit inputs, and returns it as a character result.
- `lookup_byte_from_table`: Returns one byte from a table that starts 1024 bytes into a buffer, at an index formed from two 16-bit register inputs shifted by five bits.
- `lookup_negated_table_value`: Looks up a byte from a table, indexed by two sign-adjusted 16-bit inputs, and returns it as a signed value offset by -128.
- `lookup_negated_table_value_2`: Calculates an index from two 16-bit inputs to fetch a byte from a table and returns that byte minus 128.
- `lookup_signed_table_value_2`: Performs a complex table lookup using sign-adjusted 16-bit inputs and returns either a negated byte or a byte offset by 128.
- `lookup_negated_table_byte`: Returns the negated value of one byte from a table that starts 1024 bytes into a buffer, at an index formed from two 16-bit register inputs.
- `lookup_table_value_offset`: Looks up a byte from a table indexed by two sign-adjusted 16-bit inputs and returns it offset by +128.
- `read_byte_from_table`: Reads a byte from a table at an offset derived from two 16-bit inputs, and returns it with the constant 128 subtracted.
- `format_integer_as_decimal`: Converts an integer into decimal digits in a text buffer by repeatedly subtracting powers of ten from a table, suppressing leading zeros.
- `format_integer_as_decimal_2`: Converts a 16-bit unsigned value into decimal digits in a text buffer by repeatedly subtracting powers of ten from a table, suppressing leading zeros.
- `format_long_as_decimal`: Converts a 32-bit value into decimal digits in a text buffer by repeatedly subtracting powers of ten from a table, suppressing leading zeros.
- `copy_word_to_memory`: Stores a 16-bit word from a source pointer into a destination address calculated from two other values.
- `clear_flag_and_fetch_byte`: Clears a global flag and, when the saved byte had its low bit set, derives an index from that byte and returns a byte from a table at that index, after storing a value into the table.
- `unpack_nibbles_and_call_library`: Splits each of sixteen 16-bit words into nibble values packed into a small byte block, then calls library routines through a library base at three offsets.
- `blend_palette_tables`: Interpolates between two sets of 16 source words over 17 steps, writing the blended 16-bit values into two output buffers obtained from a routine.
- `compare_counted_byte_strings`: Compares a zero-terminated byte string with a second byte sequence, counting down the length as it goes.
- `test_first_byte_nonzero`: Tests whether the byte at a pointer is non-zero, returning 1 in the low half of a 64-bit value when it is and zero otherwise.
- `fill_pattern_from_table`: Expands a 16-bit seed into a sequence of byte pairs from a global table and writes them into a buffer until the seed is exhausted.
- `xor_key_words_and_loop`: Calls one routine, XORs two consecutive 32-bit words at a pointer with fixed key values, then calls another routine repeatedly until a count from a register reaches zero.
- `encode_value_as_four_letters`: Encodes a 32-bit value into four capital letters by dividing by fixed constants and adding the letter A to each quotient and remainder, writing the result through a pointer.
- `swap_structure_byte_pairs`: Swaps six pairs of bytes inside a structure: offsets 1 and 4, 2 and 8, 3 and 12, 6 and 9, 7 and 13, and 11 and 14.
- `rotate_bits_in_place`: Rotates a block of eight bytes one bit at a time through a carry, repeating across the block.
- `decode_and_xor_structure`: Iteratively divides values by three, performs a bitwise XOR on a structure using two different constants, and calls other routines.
- `return_minus_one_2`: Returns -1 without doing any other work.
- `decode_four_byte_value`: Reads four bytes through a pointer, adds a fixed offset to each, and combines them into one integer by base-26 weighting.
- `depack_packed_stream`: Decompresses a packed block by reading bits from the end of the stream.
- `spin_unless_zero_flag`: Returns immediately when the zero flag is set, otherwise it loops forever.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `invert_and_copy_block_entries`: Walks a table of entries, each holding a pointer and a size, until it reaches a negative pointer.
- `dispatch_four_byte_tags`: Matches a 32-bit tag against three fixed values, returning a fixed 16-bit code for two of them and the high word after two self-calls for the third.
- `dispatch_two_tags_or_decrement`: Matches a 32-bit tag against two fixed values, returning the high word after two self-calls for one and a fixed 16-bit code for the other.
- `extract_high_word_duplicate`: Takes the high 16 bits of the data register and returns a 32-bit value with that word repeated in both halves.
- `match_tag_or_decrement_chain`: Returns a fixed tag value when the data register matches one constant; otherwise it calls itself once, returns the low word minus one, and calls itself again when that result is one and the high word is zero.
- `decrement_low_word_with_retry`: Calls itself once, then returns the low word of the data register minus one, calling itself again when that result is one and the high word is zero.
- `duplicate_low_word_both_halves`: Takes the low 16 bits of the data register and returns a 32-bit value with that word repeated in both halves.
- `decode_rle_planar_image`: Decodes run-length packed data into 200 rows of four 40-byte blocks.
- `unpack_rle_interleaved_planes`: Unpacks a run-length stream into 40-byte rows, stepping between plane offsets as each row group completes and stopping when the outer counter runs out.
- `copy_8000_words_from_table`: Copies 8000 consecutive 32-bit values from one table to another, counting down from 7999 to -1 with one value per iteration.
- `copy_values_after_sentinel_marker`: Scans two word tables for a sentinel value of 384, then copies sixteen 16-bit values from a source pointer into both tables after their sentinels, advancing through both tables together.
- `clear_and_unpack_rle_planes`: Zeroes an 8000-byte destination block and then decodes run-length data into it one 40-byte row at a time across four plane blocks.
- `unpack_rle_planes_skip_marker`: Decodes run-length data into 40-byte-wide rows across four plane blocks.
- `decode_rle_strided_plane_rows`: Decodes run-length data into destination rows whose step between bytes is a variable stride, copying literal runs and repeating single bytes.
- `fill_strided_runs_from_stream`: Decodes strided run-length records into a destination: a zero-count record repeats one value across the stride, while other records copy literal bytes, and rows and planes advance as the counters run out.
- `decode_skip_literal_repeat_stream`: Decodes a strided byte stream where positive codes advance the destination by a value from a lookup table, negative codes copy literal bytes, and a zero code repeats the following byte.
- `decode_rle_planes_until_marker`: Advances through blocks of 8000 bytes, decoding run-length rows of 40 bytes into each block until a marker byte of all ones ends that block's data.
- `setup_table_then_enable_dma`: Initialises a table with constant values, runs a retry loop of register-access calls until a status word reads zero, then writes a fixed sequence of values to the DMA control register.
- `poll_status_word_until_clear`: Repeatedly exchanges a status value with hardware until a read returns zero, returning minus one on a failed check.
- `wait_nonzero_status_then_write`: Loops reading a status word until one register exchange returns non-zero, returning minus one on an error condition, then issues a fixed sequence of register writes and returns zero.
- `copy_fields_then_scan_entries`: Copies two word fields within a structure to other offsets in the same structure and stores a third word in a global.
- `adjust_global_on_index_match`: Calls a sub-routine, and if its result is zero and an input word matches a global index, writes a value from a structure reduced by four into a global.
- `initialize_first_free_slot`: Finds the first unused entry in a global array of 52-byte records, marks it used, and fills its fields from input values and a lookup table.
- `dispatch_indirect_by_index`: Reads an index from a global variable and jumps indirectly to the routine whose offset is stored in a table at that index.
- `copy_three_blocks_into_globals`: Copies consecutive byte blocks into a global area from fixed source areas, copies two words under a flag condition, then clears one global word and copies another.
- `filter_entries_into_two_lists`: Walks a set of 314-byte records and, for each, builds two lists of word pairs from global table entries whose flag byte is positive.
- `reset_globals_and_init_flags`: Clears several global words and a byte range, then, if a mode flag is zero, sets initial flag values and a table pointer.
- `store_modular_offset_result`: Calls two sub-routines, then stores into a global the register value taken modulo five, multiplied by 727, plus a second register value.
- `or_accumulate_words_into_tables`: Optionally selects a base pointer from a global, then for a counter derived from a global value divided by five ORs eight 32-bit words from a source table into two destination tables.
- `reset_counters_and_sync_state`: Polls two global values for up to nine passes, clears a counter, and conditionally calls a sub-routine when a timer is at or below a threshold.
- `clamp_two_globals`: Clamps two global variables to a range between zero and fifty-six inclusive.
- `swap_two_entries`: Swaps eight bytes and two 16-bit fields between two records selected by an input index and the current global index.
- `invoke_two_subroutines_only`: The decompiled body is unavailable, so only its relationships are known: it calls two other routines and is called by two.
- `update_entity_attributes_via_table`: Reads values from a structure in a0 to index into a table at a global address, then performs conditional updates to the table entries or calls other routines to modify specific bytes and words based on the retrieved data.
- `invoke_three_subroutines_only`: The decompiled body is unavailable, so only its relationships are known: it calls three other routines and is entered from a data pointer and another routine.
- `decrement_counter_and_clear_flags`: Decrements a per-slot counter in a global table by two, applies a further adjustment and a sub-routine call when a flag bit is set, and clears several status bytes in the record depending on a sub-routine's result.
- `move_link_into_table_slot`: Moves a link value from one record into a table slot, updates the neighbouring link with the offset difference between records, increments a table byte, and returns one.
- `relink_entry_when_key_changes`: Writes two new values into a record and, if they differ from the old key, unlinks the record from its old list and links it at the head of the list for the new key, returning one.
- `insert_entry_at_list_head`: Clears a link field in a record and pushes the record onto the head of a list selected by a key computed from two record bytes.
- `unlink_node_from_list`: Removes a record from the singly linked list whose head is selected by a key from the record.
- `calculate_ratio_and_update_structure_2`: Subtracts a word in d6 from d0 and a word in d7 from d1, then performs 16-bit divisions and swaps values in d0 and d2 based on a byte from a structure in a0.
- `perform_division_swap`: Performs two 16-bit divisions and swaps the values in d1 and d2 using a series of swap and move instructions before branching to another location.
- `perform_division_negate_2`: Negates d1, extends d0 and d1 to 32-bit values, fetches a byte from offset 18 of a0 into d2, and if d0 is less than the negated d1, performs two 16-bit divisions and swaps values between d0 and d2 before branching to another routine.
- `perform_division_negate`: Performs two 16-bit divisions, swaps the values in d1 and d2, negates d1, and then branches to another location.
- `calculate_division_results_2`: Subtracts a 16-bit value in d7 from d1 and, if the result is not negative, negates d0, extends d0 and d1 to longwords, and performs two 16-bit divisions using a byte from offset 18 of the structure in a0, then performs a series of swaps and moves on d0 and d2 before returning.
- `perform_division_negate_d0`: Performs two 16-bit divisions, swaps the values in d1 and d2, negates d0, and then branches to another location.
- `calculate_division_results`: Negates d0 and d1, extends them to longwords, compares them, and if d0 is not greater than d1, performs two divisions using a byte from offset 18 of a structure in a0 and then performs a sequence of swaps and moves.
- `update_division_results`: Performs division operations using d0, d1, and d2, then updates two 16-bit values at offsets 14 and 16 of a structure in a0 if the results differ from existing values.
- `merge_entity_data`: Merges data from a structure in a0 into a structure in a1, clears links and flags, and calls a routine to remove a record from a key list.
- `accumulate_entry_and_set_state`: Adds a computed amount to a per-slot total in a global table and increments a counter, then subtracts that amount from another total, clamping at zero.
- `reset_entity_state`: Calls a sub-routine, calls another when a mode field equals two, then clears a 16-bit field and sets a state byte to a fixed value.
- `set_state_fields_and_call`: Unless either of two records has type four, which triggers two sub-routine calls instead, it writes a pair of state bytes and a 16-bit field chosen by an index byte, then calls a sub-routine.
- `call_pair_or_set_state`: When the first record has type four it calls two sub-routines.
- `set_field_value_and_call`: Writes a 16-bit field of the record with a value chosen by an index byte, writes a state byte, then calls a sub-routine.
- `set_entity_state_defaults`: Calls a sub-routine, then sets a 16-bit field and state bytes of the record to fixed defaults, switching to alternative values when a flag bit is set.
- `link_records_into_chain`: When either record has a flag bit set, detaches the first record from its list and, for a special mode value, appends it to a chain and initialises its fields.
- `cross_link_two_records`: Stores each record's offset into a pointer field of the other, and sets state bytes on both.
- `initialize_spawn_entry`: Initialises a record's type and state bytes and a word copied from a global, then calls a sub-routine and, when its result is not positive, calls another.
- `invoke_four_subroutines_only`: The decompiled body is unavailable, so only its relationships are known: it calls four other routines and is called by one.
- `find_matching_entry_and_flag`: Walks a linked list for an entry whose type byte is four and whose key byte matches the input, sets a flag bit on it, and returns one.
- `search_list_for_flagged_entry`: Walks a linked list for an entry of type four with a matching key byte and sets a flag bit on it, returning one.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a0, then returns.
- `invoke_one_subroutine_only`: The decompiled body is unavailable, so only its relationships are known: it calls one other routine and is called by one.
- `invoke_one_subroutine_brief`: The decompiled body is unavailable, so only its relationships are known: it calls one other routine and is called by one.
- `decrement_timer_and_set_state`: Subtracts a value from a counter in the record and, once it drops to one or below, calls a sub-routine and sets a timing word and a state byte chosen by a flag bit.
- `set_state_and_return_one`: Writes a fixed state byte into the record pointed to by the input register and returns one.
- `return_zero_constant_2`: Returns zero and performs no other work.
- `compute_offsets_and_quotient`: Compares two input bytes with the record's stored bytes, derives two adjusted 16-bit offsets from the differences, and stores them with a quotient taken from a record byte.
- `store_quotient_into_field`: Divides an input register value by a second register and stores the 16-bit quotient in a record field.
- `normalize_two_byte_fields`: Reads two signed bytes from the record, compares each with a high byte from the input words, and writes two 16-bit results back.
- `initialize_slot_and_dispatch`: Finds the first empty 52-byte record, initialises its state and input fields, then calls a routine chosen through a jump table indexed by a value.
- `initialize_record_and_dispatch`: Initialises the fields of the record pointed to by the input register, then calls a routine chosen through a jump table indexed by a value.
- `validate_and_update_byte`: Compares an input byte with a record field and a global lookup.
- `return_result_of_subroutine`: Most of its body is unreachable and was removed by the decompiler, leaving a call to a sub-routine whose result it returns.
- `copy_bytes_into_masked_table`: Walks a table of word entries until a sentinel value of minus 99, and for each entry whose masked bits are clear, writes a byte from a second table into a global table at an index computed from the entry.
- `do_nothing`: This routine is empty and performs no operations.
- `scan_table_for_flagged_matches`: Walks a table of word entries and, for each one whose masked index passes a bit test, follows a chain of global entries.
- `check_table_entries_match`: Walks a table of word entries until a minus-99 sentinel and returns one only if each entry passes a mask test and no chained global entry has a particular type byte.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_5`: This routine is empty and simply returns a zero value to the caller.
- `mark_matching_table_entries`: Walks a table of word entries and, for each entry whose masked bits are clear and whose stored byte equals a given value, writes a fixed value into that byte.
- `clear_then_write_matching_entries`: Walks a table of word entries until a minus-99 sentinel, clearing a global byte at each computed index.
- `iterate_linked_structures`: Steps through a set of 314-byte structures, decrementing a counter in those of type four and reloading it when it runs out.
- `process_linked_structure_node`: Inspects a linked node and either writes a two-byte record into the output block, using values from its fields or a global, or clears the node's pointers when no record is needed.
- `update_linked_structure_neighbor`: Counts down a timer on the structure and, when it expires, compares weights derived from flag bits of neighbouring entries.
- `find_matching_neighbor_record`: Looks for a neighbouring entry with a matching identifier and, when a counter allows, writes a type-four record into the output block and returns one.
- `write_record_on_weight_match`: Walks a table of word entries, skipping those whose masked bits are set, until one has a weight byte equal to the input.
- `write_record_and_decrement_counter`: Writes a type byte, a value and a flag byte into an output record, then decrements a counter in the node and resets it when it runs out.
- `clear_word_return_zero`: Clears a 16-bit field of the node pointed to by the input register and returns zero.
- `decrement_flag_and_test_counter`: Checks a flag field in an input structure, decrements a counter and reloads it when it reaches zero, then compares a scaled global value with a value from a second structure.
- `dispatch_handler_by_index`: When a global counter exceeds a threshold and a structure field is non-zero, selects a handler from a jump table by an index derived from a sub-routine result modulo a size field.
- `find_nearest_matching_entry`: Scans a set of word entries, offsetting each by a start value, and keeps the one closest to a reference value among those whose masked bits and flag bit meet a condition.
- `find_nearest_flagged_entry`: Scans a set of word entries and keeps the one closest to a reference value among those whose table flag bit is set.
- `find_nearest_entry_with_retry`: Scans word entries like its neighbours, keeping the one closest to a reference value among entries whose table flag bit is set and whose retry counter has run out.
- `return_zero_6`: This routine is empty and simply returns a value of zero to the caller.
- `check_tile_terrain_match`: Compares two terrain bytes of a tile entry against a matching table entry and, when they differ, writes a status byte and terrain bytes to the output block and returns one.
- `clear_flag_below_quarter_value`: If a 16-bit flag word in the structure is non-zero and a short field is at most a quarter of that flag's value, it clears the flag word.
- `store_offset_in_table`: Writes an offset derived from the structure's address into a table entry indexed by a byte field, sets bit zero of a flag byte, and returns the input register unchanged.
- `clamp_and_store_two_bytes`: Calls a sub-routine, clamps two returned bytes to the range zero to 63, and stores each in the structure followed by a constant byte.
- `populate_table_entry_from_index`: Computes an entry offset from an index byte, writes a pointer and several bytes into a table entry, sets a constant byte, and copies a word from a second table.
- `clear_flag_zero_table_entry`: If bit zero of a flag byte in the record is set, it clears that bit, zeroes a table entry indexed by the record's index byte, and calls a sub-routine.
- `link_record_into_table_slot`: Selects a table slot from a record's index byte and copies a word from it into the record.
- `set_timing_state_from_global`: Writes a fixed 16-bit timing value and a state byte into the record, and copies a 16-bit value from a global into a second field.
- `set_record_kind_and_state`: Writes two state bytes into the record pointed to by the input register and returns.
- `scan_entry_list_status`: Computes a masked index from input bytes and, when it is valid, walks a linked list of entries.
- `check_list_for_blocking_entries`: Walks a linked list of entries from a computed head and returns a failure code when it finds a blocking entry or when a cost limit is exceeded, otherwise zero.
- `scan_list_updating_entries`: Walks a linked list of entries from a given head, updating entries whose key differs from the record's.
- `scan_list_for_blockers`: Walks a linked list of entries, returning zero at the end of the list, a failure code on a blocking entry or cost limit, and otherwise updating qualifying entries.
- `check_cost_limit_across_list`: Repeatedly tests a record's cost value against a limit while walking a linked list of entries from a given head and updating qualifying entries.
- `return_failure_code_constant`: Returns minus two and performs no other work.
- `dispatch_record_by_table_word`: Looks up a word from a table by an index byte.
- `dispatch_record_through_table`: Looks up a word from a table by an index byte, sets a flag bit and state bytes on the record, and dispatches through a jump table.
- `update_record_data`: modifies several bytes within a data structure and performs an indirect call using a value retrieved from a global table.
- `link_nearest_flagged_neighbor`: Scans the record list for the closest eligible entry by summed absolute differences of two byte fields, then cross-links the input record with that entry and sets its state byte.
- `link_records_and_set_state`: Stores each record's offset in a pointer field of the other and sets the first record's state byte, then returns one.
- `return_zero_constant_3`: This routine simply returns a zero value to the caller.
- `scan_records_for_key_mismatch`: Walks a linked list of entries and, for the first entry with a positive count whose key differs from the record's, stores a value and state bytes in the record.
- `set_record_defaults_fields`: Writes fixed values into four fields of the record pointed to by the input register: a 16-bit value, two state bytes and a 16-bit timing value.
- `scan_records_then_set_defaults`: Walks linked entries for one whose key differs from the record's and stores a value and state bytes in the record when found.
- `write_bytes`: stores two specific byte values into a structure at fixed offsets from a pointer.
- `compute_scaled_neighbor_offsets`: Compares two input byte pairs with a record's coordinates, selects a direction from a small table, and searches a set of offsets, calling sub-routines to pick a candidate.
- `search_offset_table_for_candidate`: Calls a sub-routine and, when it reports a mismatch, searches a short table of offset pairs, calling sub-routines to validate candidates.
- `set_tile_properties`: Writes a global table entry from a record's fields, then compares two input offsets against a table of pairs, calling sub-routines when a mismatch occurs.
- `clear_entity_links_and_flags`: Clears the link from a table entry back to the record when it points there, then walks a linked list of entries of certain types, clearing their links and a flag bit.
- `copy_record_into_free_slots`: Copies a 52-byte record into the first free slot of a global table, sets its default fields, and calls a sub-routine for each copy.
- `copy_word_table_entries`: Copies 26 words from a source record into the next free slot of a global table, sets fixed default fields in the copy, and calls a sub-routine.
- `compute_indexed_value`: Reads a 16-bit value from a global table by index.
- `call_routine_when_table_positive`: If a global flag is zero and a byte-indexed global word is non-negative, calls a sub-routine.
- `return_stack_arguments`: Returns two values taken from its stack arguments without other work.
- `invoke_two_subroutines_brief`: The decompiled body is unavailable, so only its relationships are known: it calls two other routines and is called by two.
- `process_entity_updates`: Performs complex updates to entity structures by calculating indices into tables, updating linked list nodes, and executing various conditional logic branches including bitwise operations and jumps to subroutines.
- `clear_slot_byte_if_outside`: Returns its input registers unchanged when the record lies in a first fixed range.
- `clear_slot_byte_or_return`: If the record lies in a fixed range, it calls a sub-routine and returns its inputs unchanged.
- `clear_slot_byte_then_call`: Clears a byte at the record's index offset and calls a sub-routine.
- `invoke_one_subroutine_unclear`: The decompiled body is unavailable, so only its relationships are known: it calls one other routine and no caller is recorded.
- `relay_three_subroutine_calls`: The decompiled body is unavailable, so only its relationships are known: it calls three other routines and is called by three.
- `calculate_and_update_index`: Calls another routine, performs a division, and uses the result to index into a table to add a value to d1, then checks bits in d0 before branching.
- `write_record_fields_and_call`: Writes a record's index, coordinates and a 16-bit timing value, then calls a sub-routine.
- `return_zero_value_3`: The routine returns a constant zero value to the caller.
- `initialize_empty_entry_slot`: Finds the first empty 32-byte entry in a global table and fills it with the input values, a constant size, a flag and an identifier, plus a value from a table.
- `fill_slot_and_mark_neighbours`: Fills the first empty 32-byte entry in a global table with the record's values and a timing value.
- `fill_empty_slot_and_mark`: Fills the first empty 32-byte entry in a global table with the input values and a timing value, then marks neighbouring entries from a sentinel-terminated offset list.
- `mark_neighbours_from_slot`: Walks a sentinel-terminated offset list and writes a timing value into each matching neighbouring entry, using the input record's values.
- `return_stack_value_5`: Returns the value currently residing on the stack.
- `return_stack_value_4`: returns the value currently on the stack.
- `claim_slot_for_keyed_entry`: If the table entry for an index is empty, claims the first free 32-byte entry in a global table, fills it with the input values and a fixed timing value, and links it into the table.
- `claim_slot_and_link_pointer`: Finds the first free 32-byte entry in a global table, fills it with the input values and a fixed timing value, stores a pointer to it in the input record, and calls a sub-routine.
- `write_record_fields_and_call_2`: Stores a pointer to the record in a field of a second structure, writes fixed byte and word fields into the record, and calls a sub-routine.
- `link_records_from_chain_head`: Walks a linked chain from a table head, filling free 32-byte entries with values from a sub-routine result and the chain record, and linking each new entry into the chain.
- `update_linked_pair_entries`: Repeatedly fills a free entry with a sub-routine value indexed through a table, clamps two bytes to a valid range, and links the new entry with the source record, until a sub-routine returns zero.
- `clear_active_entity_chain`: If a table entry references a record, clears that table entry, sets two fields of the record, and walks its linked list clearing a byte in each node and calling a sub-routine for each.
- `record_event_and_update_bounds`: When flags in a table entry match a mask, increments a global counter and marks the entry.
- `widen_record_bounds`: Widens a record's two bounds to include the input values, then copies the bounds into two output fields and returns.
- `widen_record_vertical_bound`: Widens one bound of a record to include an input value, then copies the bounds into two output fields and returns.
- `claim_slot_for_bounds`: Claims the first empty 32-byte entry in a global table and fills it with bounds computed from the input values and a fixed timing value, storing a pointer to it in the input record.
- `initialize_entity_struct`: Writes a fixed set of bytes and two bounds values into the structure pointed to by the input register.
- `set_timing_and_link_pointer`: Unless the record's state byte is the excluded value, writes a timing word, either fixed or taken from a global, and kind and state bytes, then stores a pointer to the input in a field.
- `initialize_object_header`: Writes fixed byte and word fields into the structure pointed to by the input pointer and stores a computed short derived from an input word.
- `reset_object_state`: Unless a state byte holds the excluded value, clears a word field, calls a sub-routine, sets a state byte, and copies a word from a table indexed by a byte.
- `iterate_table_entries`: Walks a table of 16-byte entries, clearing an active entry's status byte when a lookup-table test passes and calling a sub-routine for it.
- `build_entry_from_neighbour_list`: Checks a class code against a lookup table and, when it is allowed, searches a sentinel-terminated list for matching entries.
- `fill_entry_from_neighbour_list`: Searches a sentinel-terminated list for matching entries and, when a free 16-byte entry is available, fills it with a derived kind, value and timing and adjusts linked entries.
- `collect_matching_neighbour_entries`: Collects references to entries whose key bytes match the input from a sentinel-terminated list.
- `fill_entry_and_adjust_links`: Fills a free 16-byte entry with the input values and a derived kind and timing, then adjusts the kinds of linked entries and sets a timing value from a table.
- `reset_linked_entry_kinds`: Sets a timing word and kind byte on the input entry, then walks a list of pointers, resetting the kinds and timing values of linked entries and setting a timing value from a table.
- `adjust_pointer_list_kinds`: Walks a list of pointers and adjusts the kind and timing of each linked entry according to its current kind, then sets a timing value from a table for the input entry and returns.
- `return_stack_arguments_2`: Returns its stack argument unchanged and performs no other work.
- `divide_and_fill_free_entry`: Divides a value returned by a sub-routine, fills the first free 16-byte entry with a remainder-based timing and a kind, and repeats for each result a sub-routine provides.
- `fill_object_entry`: Repeatedly fills the first free 16-byte entry with a kind and timing derived from a sub-routine result, until that result is zero.
- `invoke_three_subroutines_short`: The decompiled body is unavailable, so only its relationships are known: it calls three other routines and is called by one.
- `update_kind_two_entries`: Walks a linked list of entries and, for each entry of kind two whose state byte is not the excluded value, rewrites its kind and timing values and clears a group of state bytes.
- `update_linked_kind_entries`: Walks a linked list of entries, rewriting kind two entries to kind six with cleared state bytes and calling a sub-routine for kind four entries.
- `fill_free_entry_in_range`: When two input coordinates both lie in the range zero to 63, fills the first free 16-byte entry with them, a timing value from a table and a kind, then calls a sub-routine.
- `initialize_entity_record`: Writes fixed fields into the record pointed to by the input register and calls a sub-routine.
- `check_neighbour_kind_mask`: Looks up a cell's class byte and, unless it is in a reserved range, derives a kind from a neighbour's bits.
- `combine_high_word_with_flag`: Returns a 32-bit value whose low word is a flag input and whose high word is the high word of the input value.
- `return_minus_one_constant`: Returns minus one and performs no other work.
- `copy_byte_and_decrement`: Copies a byte from a source record at a fixed offset into a destination byte, subtracts a constant from it, and returns zero.
- `swap_table_entry`: When a flag bit is set for the table entry selected by two input bytes, replaces that entry with the value stored at the index the entry holds.
- `build_neighbour_mask_and_mark`: Unless a flag bit is set, either finds a matching table entry or builds a four-bit mask from neighbouring cells that pass a flag test.
- `rewrite_cell_from_neighbour_mask`: Builds a four-bit neighbour mask from cells that pass a flag test, writes a value derived from it into the cell, updates a linked entry's timing and state, and rewrites neighbouring cells whose mask bit is set.
- `rewrite_neighbours_from_cell_value`: Writes the input cell's class byte into a cell, updates a linked entry's timing and state, and rewrites neighbouring cells whose mask bit is set, using a derived value.
- `rewrite_neighbours_from_mask`: Builds a four-bit mask from neighbouring cells that pass a flag test, writes a derived class value into the cell, and rewrites neighbouring cells whose mask bit is set.
- `update_linked_entry_and_neighbours`: Finds the first linked entry of kind twenty-six and updates its timing and state, then rewrites neighbouring cells whose mask bit is set, using a derived class value.
- `return_one_constant`: Returns one and performs no other work.
- `scan_weighted_table_entries`: Computes a start offset from an index and a table, then loops a calculated number of times.
- `compute_and_mark_table_entries`: Computes an iteration count from a global value and a sub-routine result, then in each pass calls a sub-routine, indexes a short table and marks a global cell with a constant when flag checks pass.
- `mark_entries_when_mask_full`: Computes a starting value from a table and a sub-routine result, then loops over neighbouring positions, marking a global cell with a fixed byte where a combined mask equals fifteen.
- `fill_free_entry_from_list`: Walks a sentinel-terminated list of offsets and, for each match whose flag passes a test, fills the first free entry with a kind, a timing value from a table and two scaled values, then calls a sub-routine.
- `fill_entries_with_flag_matches`: Walks a sentinel-terminated offset list and, for each position whose flag bit passes a test, fills the first free entry with a kind, a timing value and two scaled values, then calls a sub-routine.
- `return_without_work`: Contains only a return and performs no work.
- `fill_free_entries_from_division`: Divides a value from a sub-routine into a quotient and remainder, then fills the first free 16-byte entry with masked values, a kind and a timing value.
- `fill_entry_from_masked_word`: Repeatedly calls a sub-routine and builds a masked word from its low bytes and an input word.
- `claim_free_entry_for_value`: Claims the first free 32-byte entry in a global table, writes two input bytes into it, sets a timing value derived from a table and a kind, and returns one.
- `initialize_struct_from_registers`: Writes two input bytes into the record pointed to by the input register, sets a timing value derived from a table, stores a kind byte, and returns one.
- `update_neighbourhood_cells_by_flags`: Walks a rectangular neighbourhood of cells around the record, calling a sub-routine for each flagged cell and adjusting matching global cells by a fixed amount.
- `adjust_cell_block_values`: Walks a block of cells around the input position, adjusting matching global cells by a fixed amount where a mask bit test passes, and calls sub-routines for flagged neighbours.
- `scan_cells_rewrite_flags`: Walks a block of byte cells in nested loops, looks up each cell's flags in a global table, and rewrites qualifying cells with an adjusted value.
- `add_four_to_counter`: Adds four to the 16-bit counter stored at a fixed offset in the structure its caller passes, and returns.
- `apply_matching_table_entries`: Runs a countdown delay loop, then walks a global table of three-word records until a terminator value, copying a byte into a lookup table wherever the record's condition matches.
- `modulo_decrement_loop`: Calls another routine, takes its result modulo a global value, and uses the remainder as a loop counter.
- `lookup_chain_and_fill_record`: Scans a fixed table of 32-byte records for an unused one, following a lookup chain from a global table.
- `fill_first_free_record`: Scans a fixed table of 32-byte records for the first unused entry and fills it with the given values, including a timer derived from a call result modulo a global plus one.
- `fill_record_from_lookup`: Fills the record at the address register with the given values, a lookup-table entry, and a timer derived from a call result modulo a global plus one.
- `clear_record_return_failure`: Zeroes the 32-bit field at offset six of the record it is given and returns minus one as its status.
- `return_argument_unchanged_2`: Returns its incoming 32-bit argument unchanged, with no other work.
- `leave_undecompiled_routine`: The decompiler timed out, so no pseudo-code exists for this routine.
- `claim_cell_and_fill_record`: Checks a byte cell in a global table and, if it is clear, marks it and claims the first free record in a fixed table of 32-byte records.
- `fill_record_with_timer`: Writes a marker byte into a cell reached through a second address register, then fills the record at the address register with the given values and a timer taken as a call result modulo a global plus four.
- `return_null_value`: This routine returns a zero value to the caller.
- `initialize_first_free_record`: Searches the fixed table of 32-byte records for an unused one and, if found, clears its status field and sets its state byte to a fixed code.
- `initialize_record_state_fields`: Initializes the record at the address register by clearing its status field and setting its state byte to a fixed code.
- `return_zero_status`: Returns zero with no other work.
- `set_flag_for_linked_entries`: Walks a linked list of entries that starts from a global table slot chosen by two input bytes.
- `overwrite_cell_if_full_nibble`: Reads the cell indexed by the record's two bytes from a global table and, when its low nibble is fifteen and a particular bit is clear, overwrites that cell with a fixed value.
- `return_null_value_2`: This routine returns an uninitialized value from the stack.
- `process_global_table_entries`: Walks a global table of short entries until a terminator, and for each entry that passes a mask test it follows a linked list of nodes.
- `clear_record_flag_byte`: Clears the byte at offset twelve of the record it is given, with no other effect and no calls.
- `iterate_handler_table`: Walks a table of handler pointers in a global area and calls each handler through its entry, clearing two bytes in the structure at the current pointer after each call.
- `leave_timed_out_routine`: The decompiler timed out, so no pseudo-code exists for this routine.
- `branch_to_address`: Performs a relative branch to another location.
- `execute_event_callback_table`: Iterates through a table of instructions where each entry consists of a branch to a specific handler, performing tasks such as bitwise negation of a word, copying data between pointers, updating a global counter, and executing conditional logic involving table lookups and arithmetic operations on values derived from an input buffer.
- `check_and_update_table_entry`: Reads a 16-bit value from a table at a global index and, when it is nonzero and within a limit, advances the index by six.
- `run_timed_sequence_loop`: Clears several global state variables, then runs nested loops that repeatedly call unnamed routines and wait on their results, including two long countdown delays.
- `poll_results_and_set_mode`: Calls an unnamed routine until its result is at least one, returning early if a check fails.
- `walk_table_and_set_mode`: Walks the global table, calling a routine for each entry until a bound is reached, then processes a second routine's results in a loop.
- `call_then_return_stack_value`: Calls an unnamed routine and then returns the eight-byte value passed on the stack, unchanged.
- `return_stack_value`: Calls an unnamed routine and returns the eight-byte value that was passed on the stack, unchanged.
- `check_record_type_merge_flags`: Reads a record's type byte and, when it is six or eight, runs a polling loop over unnamed routine calls and then ORs two saved global words with their current values.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 1 into d0, then returns.
- `normalize_three_entry_flags`: Calls an unnamed routine, then walks three ten-byte entries in a global table.
- `clear_flags_and_process_table_2`: Iterates over a block of short flags starting at a global variable, clearing each non-zero flag and invoking another routine, with an extra call when a related entry is set.
- `toggle_entry_and_recurse`: Looks up a word in a table indexed by a byte from the argument.
- `execute_self_calling_loop`: Calls another routine repeatedly with a flag that doubles each iteration while a short value stays below a threshold, then conditionally calls the same routine once more if a global variable is non-zero.
- `call_twice_by_flag_state`: Checks a global flag.
- `update_entry_and_linked_neighbor`: Indexes a global table with its argument, calls an unnamed routine twice with values built from the entry's fields, and repeats both calls for a second entry if the entry's link field is nonzero.
- `check_flag_and_call_twice`: If a global flag is nonzero, stores the flag plus four into one global, stores the address of another global into a second, and calls an unnamed routine twice.
- `initialize_interrupt_handlers`: Stores pointers to a global area and to unnamed routines into many fixed memory locations, and sets two bytes in memory.
- `disable_interrupt_server`: When a global counter is positive, it sets a state global to one, calls the Exec library's remove-interrupt-server routine through an indirect pointer, and then sets the state global to fifteen.
- `set_unit_flags_by_mask`: For each of four unit structures selected by the low bits of its argument, it writes flag and byte fields chosen by the bits of a 16-bit mask.
- `search_table_for_match`: Walks a table of three-word entries, stopping at a zero terminator or at the first entry whose first word equals the given value.
- `leave_timed_out_routine_2`: The decompiler timed out, so no pseudo-code exists for this routine.
- `skip_flagged_entity_updates`: Calls another routine for a 64-bit value, then skips its loop over four records because a flag tested in the listing is set to false just before the test.
- `update_dma_channel_registers`: When a pending flag is set, it clears the flag and decrements a counter, then processes four channel records.
- `configure_dma_channel`: Tests d3 and, if it equals 1, calculates a value by shifting d7, performs a bitwise OR with 32768, and writes the result to the DMA controller register at offset 96.
- `chain_channel_register_updates`: Walks four channel records, writing each record's pointer and length into its hardware register block and marking it pending.
- `finish_channel_record_updates`: Completes a four-record channel update: for each record it resets pending bytes, calls an unnamed routine, and writes the record's pointer and length into its hardware register block.
- `advance_sequence_step_counter`: Advances the step counter and index of a sequence record and adds a stored increment to a field when a flag is set.
- `set_record_flag_bit_four`: Sets bit two of the flag byte at offset eleven of the record it is given, with no other effect.
- `interpret_record_command_stream`: Reads command bytes from a stream attached to a record while a particular status bit is clear, and dispatches them.
- `run_record_command_stream_step`: Decrements a countdown in the record and, when the step flag is clear and the countdown reaches zero, reloads it and reads command bytes from the record's stream.
- `interpret_stream_until_terminator`: Walks a stream table and, for each sub-stream, reads command bytes until a terminator byte is reached.
- `interpret_stream_from_pointer`: Reads command bytes from a stream pointer chosen from a table by its argument.
- `dispatch_stream_commands`: Dispatches command bytes from a stream attached to the record: values with the high bit clear set table-derived fields and call an unnamed routine, while other codes set state bits or select a sub-record.
- `process_stream_command_byte`: Processes one command byte from a record's stream: values with the high bit clear set table-derived fields and call an unnamed routine, while other codes set state bits or select a sub-record.
- `load_entry_then_read_stream`: Copies two table-derived fields from a lookup indexed by the masked command value, then reads command bytes from the record's stream.
- `select_sub_record_then_read`: Points a sub-record at the stream position for the given command value, then continues reading command bytes from the stream.
- `return_from_empty_routine`: The routine performs no operations and immediately returns to the caller.
- `compute_and_store_result`: Calls an unnamed routine and subtracts a limit built from a fixed value and a record byte, clamped at zero.
- `clear_channel_register_and_mask`: Writes zero to the hardware register block selected by the index, which the listing places in the audio register range, and sets a global bit mask to one shifted by the low six bits of the index.
- `read_stream_with_initial_byte`: Stores the first byte of a record's stream as a base value, then reads command bytes from the stream.
- `start_sub_record_stream`: Reads a byte from the record's stream to select a sub-record, initializes that sub-record's fields and pointer, then reads command bytes from the stream.
- `start_stream_with_flag_set`: Reads the first byte of a record's stream and sets a sub-record's status byte to a high flag value before reading command bytes.
- `read_stream_commands_only`: Reads command bytes from a record's stream and dispatches them: values with the high bit clear set table-derived fields and call an unnamed routine, while other codes set state bits or select a sub-record.
- `copy_uppercase_text_on_match`: Calls three unnamed routines through library pointers and, for each candidate string, checks whether it ends with a fixed suffix from a global.
- `copy_uppercase_until_done`: Copies a source string into a buffer, converting lowercase letters to uppercase, until a zero byte is reached.
- `call_indirect_and_store`: Calls an indirect library routine through a pointer in the base register, then restores one global from a saved copy and stores the counter value in another global.
- `run_library_request_checked`: Performs the same indirect library call sequence as its neighbouring routine, saving and restoring a global around it.
- `run_library_status_check`: Queries an indirect library routine and, when the result is positive, runs a second library call and compares its status code with a fixed value.
- `check_library_call_status`: Calls two indirect library routines and returns a success flag, one when the first call's status equals a fixed code and zero otherwise.
- `set_global_and_return_constant`: Copies one global value into another and returns the constant one in the high register word.
- `run_library_result_word`: Calls indirect library routines through a stored library base, with a saved global restored around the sequence.
- `return_library_result_word`: Makes two indirect library calls through a stored base and restores a saved global.
- `store_global_and_call`: Copies a saved global into another global and calls an unnamed routine.
- `claim_record_bit_once`: Returns two if a bit in a global mask is already set.
- `copy_negated_word_blocks`: Walks a global buffer in five-word blocks, copying seven-entry groups between fixed areas and storing the bitwise negation of the first word of each group.
- `record_two_positive_allocations`: Calls an indirect library routine twice and, for each call that returns a positive value, stores that value in a global and a second global and calls an unnamed routine.
- `invoke_callback_and_return_value`: Calls an indirect library routine through a stored pointer when a first global is positive, then calls it again when a second global is positive.
- `copy_250_bytes_forward`: Calls an unnamed routine, then copies 250 bytes from a source offset in a global block, using an index from a register, to the start of that global block.
- `copy_bytes_from_table`: Walks a table in a global variable of entries holding a source offset, a destination offset and a length.
- `increment_global_counter`: Calls an unnamed routine, increments a global counter by one, and returns its two input register values unchanged.
- `return_immediately`: Returns immediately with no work.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 1 of 2).adf:ip_intro

- `open_library_at_entry`: Program entry point that calls two Exec library routines, one of which opens a library by name, and stores the returned base.
- `setup_copper_display_state`: Disables interrupts and DMA by writing the interrupt and DMA enable registers, polls the raster position register until it clears, and fills a block of eight entries with fixed words and a pointer.
- `draw_text_lines_to_screen`: Draws a block of newline-separated text into a screen buffer by looking up each character's glyph in a table and copying glyph rows.
- `copy_glyph_rows_to_screen`: Copies seven rows of glyph bytes for each character in a text line into the screen buffer at a fixed row stride, advancing to the next line when a newline is read.
- `copy_glyph_rows_until_end`: Walks newline-separated text and copies seven rows of glyph bytes for each character into the screen buffer, advancing the destination by one row stride per line.
- `return_stack_value_2`: Returns the eight-byte value passed on the stack, unchanged, with no other work.
- `scroll_table_on_counter`: Counts down a global value and, when it reaches zero, reloads it with two and shifts a block of words down by one entry.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP][a](Disk 2 of 2).adf:bootblock

- `return_dos_library_status`: Boot-block routine that calls an indirect library routine through a stored base and returns minus one when its result is zero, and zero otherwise.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `leave_timed_out_routine_3`: The decompiler timed out, so no pseudo-code exists for this routine.
