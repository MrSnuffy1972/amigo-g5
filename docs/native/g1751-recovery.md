# Game 1751 (a-train (1992)(ocean)(disk 1 of 2)[h skr][a info]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01751). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903312 top-level bytes in 95 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 66 of 66 units identical (bytes and relocations), 24 raw-word lines |
| Functions named and described | pass, 4497 of 4497 named (0 author symbols), 4497 of 4497 described (527 by rule, 3906 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1872 of 1903 (98.37%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 101567 of 101582 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 11010 |
| code | 324470 |
| data | 444128 |
| padding | 22840 |
| unused | 100864 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| Stack | 2 | 117 | 2 |
| Else | 2 | 117 | 2 |
| Mount | 2 | 117 | 2 |
| SetPatch | 35 | 962 | 54 |
| Path | 2 | 117 | 2 |
| Break | 2 | 117 | 2 |
| FF | 3 | 758 | 38 |
| EndSkip | 1 | 2 | 1 |
| Resident | 2 | 117 | 2 |
| Assign | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| Dir | 2 | 117 | 2 |
| NewCLI | 2 | 117 | 2 |
| NewShell | 2 | 117 | 2 |
| Quit | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| Lab | 1 | 2 | 1 |
| GetEnv | 2 | 117 | 2 |
| Skip | 2 | 117 | 2 |
| SetEnv | 2 | 117 | 2 |
| Failat | 2 | 117 | 2 |
| Execute | 2 | 117 | 2 |
| SKID_ROW | 1 | 15 | 1 |
| Copy | 2 | 117 | 2 |
| Makedir | 2 | 117 | 2 |
| CD | 2 | 117 | 2 |
| Date | 2 | 117 | 2 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| IconX | 12 | 806 | 37 |
| LoadWB | 12 | 537 | 30 |
| SetClock | 17 | 1152 | 51 |
| Wait | 2 | 117 | 2 |
| Version | 11 | 574 | 18 |
| Preferences | 52 | 7307 | 245 |
| SetMap | 13 | 674 | 30 |
| Format | 30 | 2009 | 76 |
| CLI | 11 | 607 | 25 |
| DiskCopy | 2 | 4189 | 78 |
| FastMemFirst | 7 | 207 | 6 |
| Disk-Validator | 1 | 386 | 1 |
| Newcon-Handler | 1 | 2022 | 2 |
| Shell-Seg | 1 | 802 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| serial.device | 1 | 996 | 86 |
| usa1 | 1 | 4 | 1 |
| parallel.device | 3 | 285 | 26 |
| clipboard.device | 10 | 1392 | 130 |
| printer.device | 47 | 6411 | 346 |
| EpsonQ | 15 | 563 | 42 |
| CBM_MPS1000 | 15 | 562 | 43 |
| HP_LaserJet | 15 | 604 | 47 |
| EpsonX[CBM_MPS-1250] | 15 | 702 | 51 |
| Okimate_20 | 14 | 462 | 30 |
| HP_DeskJet | 15 | 604 | 47 |
| A-Train (1992)(Ocean)(Disk 1 of 2)[h SKR][a info].adf:Installer | 6 | 45 | 2 |
| icon.library | 10 | 1375 | 109 |
| info.library | 25 | 2598 | 167 |
| version.library | 2 | 41 | 7 |
| diskfont.library | 15 | 1040 | 76 |
| A-Train (1992)(Ocean)(Disk 1 of 2)[h SKR][a info].adf:A-Train | 3 | 55618 | 2429 |
| More | 19 | 2290 | 76 |
| A-Train (1992)(Ocean)(Disk 1 of 2)[h SKR][a info].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 35 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 32 | 32 |

## Cases handled

- runtime segment an address (play-rs-m1500): 31 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 19 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- a-train (1992)(ocean)(disk 1 of 2)[h skr][a info]/A-Train (1992)(Ocean)(Disk 1 of 2)[h SKR][a info].adf:Installer: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `load_and_validate_font_for_size` (FF): Closes any existing font, opens the global font file for the requested size, checks baseline and dimensions, reports mismatches, and stores the new font handle. Kind: font/asset validation check. Pass path: the branch where iVar2 != 0 and the subsequent field comparisons (baseline, width, height) match, leading to u7__sub_0_01BA.
- `process_complex_message` (More): Performs a series of recursive calls and library functions to process a structure, including parsing signed decimal strings, calculating 32-bit products, and iterating through entity lists. Kind: indirect function call/jump table trap. Pass path: the branch following the indirect call (*unaff_A3)().

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_long_word_and_branch`: Copies a 32-bit value from register d1 to register d0 and then performs a relative branch.

### Stack

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `transfer_long_word_and_jump`: Moves a 32-bit value from d1 into d0 and then executes a relative branch.

### Else

- `find_max_and_prepare_buffer_10`: Walks a linked list to find the maximum value in a field, writes a header containing sequential values into a buffer, and sets up a stack frame for a subsequent call.
- `copy_and_skip`: Stores the contents of d1 into d0 and then branches to another routine.

### Mount

- `update_buffer_and_install_callback_3`: Finds the maximum value in a linked list, writes a header of sequential words into a buffer, and performs a series of conditional data copies between a temporary area and a global buffer.
- `transfer_register_and_branch_2`: Moves the contents of register d1 into register d0 and then performs a short branch to another routine.

### SetPatch

- `install_setpatch_environment`: Checks if a patch is already installed and, if not, allocates memory, parses a string to build a list of library and exception patches, and updates system task structures and library pointers.
- `invoke_indirect_pointers`: Invokes two function pointers stored at specific offsets within a structure pointed to by a register.
- `call_indirect_pointers`: Invokes two function pointers stored at offsets within a structure pointed to by a register.
- `update_task_status`: Retrieves a task pointer, modifies a value at an offset within that task's structure using a specific constant, and calls an indirect routine.
- `self_calling_guard`: The routine calls another routine unconditionally, then checks a global variable and, if non-zero, calls another routine with the input parameter, passes the result to another routine, and finally calls another routine unconditionally.
- `initialize_setpatch_environment`: Checks if a patch is already installed and, if not, patches various system libraries and exceptions, including graphics and layers libraries, to ensure resident execution.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_indirect_via_table`: The routine calls another routine through a pointer stored in a global variable.
- `allocate_and_patch_memory`: Checks fields of a global structure and, if conditions are met, obtains a memory block and a code pointer via library calls, copies a small code stub from another routine into the target, updates a pointer in the global structure, computes a checksum over a short array in the structure and stores its complement, then returns.
- `accumulate_and_jump`: Iterates through a list of 22 words starting at an offset of 34 from a pointer in a6, sums them, stores the bitwise NOT of the sum at offset 82 of a6, and performs an indirect jump through a pointer at offset 30 of a5 if a specific long value is found at offset 30 of a5.
- `disable_interrupts_and_patch`: The routine checks a global variable and calls another routine through a table.
- `check_flag_and_copy_word`: The routine calls another routine and tests its return value.
- `compare_and_jump_indirect`: Loads a longword into d1, compares it with a value from a table, and if they are equal, loads a pointer from a table and jumps to it.
- `compare_register_with_negative_one`: Sets a register to negative one and compares it with the value in another register, branching to another routine if they are equal; otherwise it returns.
- `set_interrupt_and_jump`: Sets bit 15 of the INTENA register, writes a 32-bit constant to a global variable if it does not match, and performs an indirect jump to an address calculated from a PC-relative location.
- `jump_to_indirect_address`: Calculates an address by adding an offset to a value stored in a PC-relative location and jumps to it.
- `initialize_patch_function_table`: Writes a computed offset to a global variable and fills a 16-entry table with successive addresses of a routine, incrementing the pointer each iteration.
- `set_d0_zero`: Performs a bitwise OR operation on register d0 with zero.
- `call_input_value_handler`: Calls the routine return_input_value using a relative branch.
- `execute_repeated_calls`: Calls another routine fifteen times in succession and then executes a nop instruction.
- `return_input_value`: The routine receives a value in D0 and returns it unchanged.
- `modify_and_call_relative_2_2`: Compares a word at an offset of a library base to 35, and if it is not less than or equal to that value, calls a routine at an offset from the library base.
- `modify_and_call_relative_2`: Calculates a new size based on a word from a structure, allocates a new block, and copies a list of values from the old block to the new block while calling another routine for each entry.
- `modify_and_call_relative`: Decrements d2, and if it is not negative, decrements a3, loads a1 from offset 0 of a3, loads d0 from offset 4 of a3, calls another routine at an offset relative to a6, and branches to another location.
- `modify_and_call`: Calculates a value by shifting a word from an offset of a4, calls another routine using a6-relative offset, and sets bit 31 of a value at the beginning of a2 before branching.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `restore_processor_state`: Copies the value from a register into a location on the stack, masks the lower 12 bits of the stack pointer, and performs a return from exception.
- `find_node_by_string`: Iterates through a linked list and returns a pointer to a node if a string stored at offset 10 of that node matches the provided input string.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `reset_global_variable`: Sets a global variable to zero and returns its previous value.
- `execute_conditional_callbacks`: Calls a routine using a pointer from a table, and if the return value is non-zero, it calls two more routines using pointers from another table, passing a value from a structure in a1 to the second call.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `scan_for_marker`: Iterates through an array of words starting from an input pointer.
- `compute_word_offset_2`: Reads a 16-bit value from a pointer in a0 and returns a new pointer by adding a calculated offset based on the low byte and bit 6 of that value.
- `compute_word_offset`: Reads a 16-bit value from two bytes after an input pointer.
- `patch_global_pointers`: The routine sequentially calls a helper routine to compute offsets and stores the returned pointers into a series of global variables.
- `restore_registers_and_return_6_2`: Moves the value in address register 2 to address register 1, restores address registers 2 and 3 from the stack, loads a 32-bit constant from a pointer in a3, and calls an indirect routine if specific bits are set in a word at an offset of that pointer.
- `restore_registers_and_return_6`: Moves the value in address register 2 to address register 1, restores address registers 2 and 3 from the stack, loads a 32-bit constant from a location relative to the program counter onto the stack, and returns.
- `select_non_null_pointer`: Checks if the first pointer in a0 is null, otherwise selects the second pointer in a1, and returns the chosen address in d0.
- `compute_and_store_byte_logic`: Calls a function pointer from a global structure and combines its result with register values using bitwise logic.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_other_routine_with_stack`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Path

- `process_max_value_and_sync_buffer`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `move_and_branch_3`: Moves the contents of d1 into d0 and then performs a short branch.

### Break

- `find_max_and_prepare_buffer_15`: Walks a linked list to find the maximum value in a field, writes a header containing sequential words into a buffer, and sets up a stack frame for a subsequent call.
- `move_and_branch_4`: Moves the contents of register d1 into d0 and then branches to another location.

### FF

- `parse_command_line_arguments_4`: Validates three command line strings and calls a function via a pointer in a structure, then parses the resulting string for space-separated arguments and hyphenated switches.
- `process_font_character_data`: Iterates through a data stream to identify specific characters, performs bitwise operations on a value in a global variable based on a table lookup, and calls another routine when a specific character is encountered.
- `flush_ports_and_reply_pending_message`: Saves the word argument, switches to the saved stack, calls a flush helper on four message pointers, finds the current task, forbids, replies to a pending message and returns the argument.
- `branch_and_jump_via_library`: Otherwise it branches to a local label, or moves a value from d0 to a1 and jumps to an offset of a library base in a6.
- `load_argument_to_a1`: Loads a value from an offset of 4 from the stack into register a1.
- `process_font_data`: Iterates through a null-terminated string in a1, and if the string is empty and a global variable is zero, calls two routines via a pointer in a6.
- `prepare_word_swap_arrays`: Loads two pointers from the stack, calls another routine, and then increments both pointers by two and sets d1 to nine.
- `swap_word_arrays_offset`: Exchanges the word at offset 28 of each a0/a1 pair, advances both pointers by two and repeats d1+1 times before returning.
- `allocate_and_init_fastfont_record`: Allocates a record via two allocators, fills fields at offsets 8, 10 and 14, initialises two sub-structures, copies six bytes into offset 34 and returns the record pointer.
- `setup_fastfonts_and_refresh_font_slots`: Builds the font record, optionally loads a replacement font, refreshes slots at offsets 40, 48 and 56 based on flag bits, prints status text and returns.
- `select_font_by_size_and_store_handle`: Stores the requested font size in a global, opens a font via the font opener, sets digit characters for size eleven or eight, waits twenty ticks, and stores the font handle or clears it.
- `close_font_and_clear_handle`: Checks the font handle at offset four of the structure, closes the font with the given base and handle, then clears that handle field and returns.
- `load_and_validate_font_for_size`: Closes any existing font, opens the global font file for the requested size, checks baseline and dimensions, reports mismatches, and stores the new font handle.
- `release_ff_object_and_three_buffers`: Allocates an object, frees three buffers at offsets 40, 48 and 56, frees the third only if non-null, then releases the object and its memory block.
- `abort_pending_text_and_clear_field`: Stores a field from a structure into a global variable, clears it, and if the value is non-zero, prints a specific text string and waits for a key press.
- `clear_second_field_and_report_status`: Stores the field at offset 68 into a global, clears it if non-null, checks a status and returns zero on success or one otherwise.
- `toggle_fasttext_and_init_font`: Calls setup routines, checks the FastText flag, prints the on or off message, frees the old font at offset 40 and optional font at 56, then stores the new font pointers at offsets 64 and 68.
- `print_string_then_free_memory`: Pushes the caller's string argument to the print routine, then calls the free routine with a null pointer and returns.
- `build_font_render_and_install_bitmap`: Opens a graphics library base, reads a font structure, allocates chip memory, patches a blit routine with register moves, copies font bytes, allocates a bitmap, fills it via a helper and stores the result.
- `call_library_vector_420_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `copy_plane_rows_to_buffer`: Saves d1, stores its low byte to a4 offset, resets a2 fields, calls a library function, then copies a2's row count of bytes from a1 stepping 128 bytes into a3 and returns.
- `process_structure_callbacks`: Checks a value at an offset of a4, if non-zero clears it and performs conditional logic involving two library calls and two calls to functions stored within a structure.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a2, then returns.
- `jump_to_sub_a90`: Performs a jump to the address at offset a90.
- `tiny_moveq_move`: Small routine of 2 instructions that loads 0 into d1, then copies (a0)+ to d1.
- `copy_byte_from_table`: Shifts d1 left by three, uses it to calculate an address, and copies a byte from that address to a location pointed to by a6.
- `transfer_data_and_call_library`: Copies sixteen bytes from a source at a2 to a destination at a6 with a stride of 128, updates a value in a table, and calls three library functions before performing an indirect jump.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_via_a4`: Loads a base address from an offset of a4 and calls a function at an offset of minus 198 from that base.
- `call_library_vector_390_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_via_a4_alt`: Loads a base address from an offset of a4 and calls a function at an offset of minus 132 from that base.
- `dispatch_indirect_call`: Loads a pointer from an offset of 4 in the stack frame into a1, loads a value from an offset of 8 in the stack frame into d0, retrieves a base address from an offset of minus 32766 in a4, and performs an indirect jump using an offset of minus 338 from that base address.
- `call_library_vector_138_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.

### EndSkip

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Resident

- `find_max_and_prepare_buffer_16`: Walks a linked list to find the maximum value in a field, then writes a header of sequential words into a buffer.
- `transfer_register_to_d0`: Copies the contents of d1 into d0 and then performs a relative branch to a location forward in the code.

### Assign

- `find_max_and_prepare_buffer_3`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `move_long_word_and_branch`: Transfers a 32-bit value from d1 to d0 before branching to a relative offset.

### EndCLI

- `find_max_and_prepare_buffer_4`: Walks a linked list of entries to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `assign_long_word_and_branch`: Sets register d0 to the value in d1 and then branches to another location.

### Dir

- `find_max_and_prepare_buffer_5`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `store_long_word_and_branch`: Copies the contents of d1 to d0 and then performs a relative branch.

### NewCLI

- `process_list_and_sync_buffer`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `update_long_word_and_branch`: Moves a 32-bit value from d1 to d0 and then jumps to a relative address.

### NewShell

- `find_max_and_prepare_buffer_6`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `load_long_word_and_branch`: Copies a 32-bit value from d1 into d0 and then executes a relative branch.

### Quit

- `prepare_buffer_and_install_callback`: Walks a linked list to find the maximum value in a field, writes a header with sequential words into a buffer, and sets up a stack frame.
- `shift_long_word_and_branch`: Moves the 32-bit value from d1 to d0 and then performs a relative branch.

### Echo

- `find_max_and_prepare_buffer_7`: Walks a linked list to find the maximum value in a field, then writes a header of sequential words into a buffer and sets up a stack frame.
- `set_long_word_and_branch`: Copies the 32-bit value in d1 to d0 and then branches to a relative offset.

### Lab

- `clear_long_word`: Clears register d0 by performing an exclusive OR with itself and then returns.

### GetEnv

- `find_max_and_prepare_buffer_8`: Walks a linked list of entries to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `move_and_branch`: Copies the longword from d1 to d0 and then branches to another routine.

### Skip

- `find_max_and_prepare_buffer_9`: Walks a linked list to find the maximum value in a field, then writes a header and fills a buffer with sequential words.
- `transfer_and_jump`: Moves the value in d1 into d0 and performs a short branch to another routine.

### SetEnv

- `populate_buffer_and_install_callback`: Walks a linked list to find the maximum value in a field, then writes a header of sequential words into a buffer.
- `assign_and_branch`: Sets d0 to the value in d1 and then branches to another routine.

### Failat

- `find_max_and_prepare_buffer_11`: Walks a linked list to find the maximum value in a field, writes a header of sequential words into a buffer, and sets up a stack frame for a subsequent call.
- `update_and_jump`: Moves the longword from d1 to d0 and then executes a short branch to another routine.

### Execute

- `update_buffer_and_install_callback`: Finds the maximum value in a linked list, writes a header of sequential words into a buffer, and copies sixteen words between a temporary area and a global buffer.
- `shift_and_branch`: Copies d1 to d0 and then branches to another routine.

### SKID_ROW

- `invoke_sequential_pointers`: Invokes a sequence of four function pointers stored in a global structure, passing the result of the first call as the base address for the second and third, then returns zero.

### Copy

- `find_max_and_prepare_buffer_12`: Walks a linked list to find the maximum value in a field, then writes a header and fills a buffer with sequential words.
- `load_and_branch`: Moves the longword from d1 to d0 and then performs a short branch to another routine.

### Makedir

- `find_max_and_prepare_buffer_13`: Walks a linked list of entries to find the maximum value in a field and, if a condition is met, writes a header and fills a buffer with sequential words.
- `set_and_jump`: Copies the value in d1 to d0 and then branches to another routine.

### CD

- `find_max_and_prepare_buffer_14`: Walks a linked list to find the maximum value in a field, then writes a header of sequential words into a buffer.
- `write_and_branch`: Moves the longword from d1 into d0 and then branches to another routine.

### Date

- `process_max_value_and_update_buffer`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `transfer_and_branch`: Moves the contents of register d1 into register d0 and then branches to another routine.

### EndIf

- `clear_d0`: Returns immediately after zeroing out register d0.

### If

- `update_buffer_and_install_callback_2`: Walks a linked list to find a maximum value, writes a header to a buffer, and performs a loop to fill the buffer with sequential words.
- `transfer_register_and_branch`: Copies the contents of register d1 into register d0 and then branches to another routine.

### IconX

- `parse_command_line_string`: Parses a command line string into tokens, handling quoted strings and converting specific character sequences into control characters, then stores the resulting pointers in a structure.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start by checking a parameter and a byte in a structure, then proceeds to allocate memory, parse decimal values, and perform various library calls including icon and exec functions.
- `build_wait_and_endcli_command`: Builds a command string from a template and the given name, including a wait command with a long timeout and an endcli redirected to NIL, after checking whether the name starts with a dot.
- `install_library_functions`: Creates and installs a library from a descriptor: it makes the library structure, sets up its function table, and returns the result.
- `scan_ram_entries_for_names`: Allocates a buffer, walks a linked list of entries, copies entries with name bytes above 31 into the buffer using various helper routines, performs string replacements, and returns a value.
- `concatenate_strings_and_call_helper`: Copies a null-terminated string into a destination buffer, appends a second null-terminated string to that buffer, and then calls another routine with the buffer pointer and the total length of the combined strings.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the terminating zero of a string, then passes that position and a second argument to another routine.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_supervisor_2`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule_2`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch_2`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `invoke_global_and_verify_checksum`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_and_increment`: Stores a byte from register d0 at the memory location pointed to by register a3 and then increments that pointer.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_increment_pointer`: Stores a byte from register d0 into the memory location pointed to by a3, then increments a3 by one and returns.

### LoadWB

- `parse_command_line_arguments`: Parses a command line string into a list of argument pointers by scanning for spaces, quotes, and escape characters, then calls another routine to process the resulting argument list.
- `update_and_invoke_cleanup_callbacks`: Retrieves a pointer from a global variable, saves a passed parameter into a slot in the structure at offset 176, and conditionally invokes several callback functions from a table within that structure, passing specific constants to them before each call.
- `start_tool_from_resident_tag`: XOR-decodes a global string with one key applied twice, allocates a 160-byte block, and uses icon, intuition, and workbench library calls to start a tool.
- `compare_null_terminated_strings`: Compares two null-terminated strings byte by byte and returns minus one when the first is lower, one when it is higher, and zero when they match.
- `build_linked_list_from_table`: Walks a table of pointer and value pairs, allocating a fixed-size node for each entry through a memory allocation call and storing the looked-up value in it.
- `call_library_vector_96_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `compare_workbench_string`: Compares a string pointed to by a2 with a global string, checking for a specific character at offset 20 and performing a byte-by-byte comparison if it matches.
- `write_control_bytes`: Writes five specific bytes to offsets 17 through 21 of the structure pointed to by a2, then branches to another routine.
- `compare_memory_blocks`: Compares a block of memory pointed to by a0 with a block pointed to by a1 using a loop, and branches based on the result.
- `load_stack_and_return`: Loads a pointer from a relative offset into a register, restores two registers from the stack, and returns.
- `call_reschedule_3`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch_3`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem_2`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_routine_with_stack_pointer`: Takes two parameters from the stack, pushes a pointer to a local stack slot, and calls another routine with these three values before returning.
- `invoke_global_and_verify_buffer`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_at_pointer`: Stores a byte from d0 into the address pointed to by a3 and increments a3.

### SetClock

- `parse_command_line_string_2`: Parses a command-line string by splitting it into tokens, handling quoted substrings and converting specific character sequences into control characters, then stores the resulting pointers in a structure.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `setclock_command_dispatch`: Parses the load, save or reset argument, reads and writes the battery-backed clock registers, prints usage and error text, and calls the clock load, save and reset helpers.
- `poll_clock_register_change`: Reads the clock register low nibble, delays briefly, reads it again and returns one if the nibble changed, otherwise zero.
- `write_global_and_check`: Writes several constant values to a global array and returns a comparison of a global variable's bits.
- `read_clock_value_after_setting_type_ten`: Allocates a structure, sets its type field to ten, calls the setup and release routines, and returns the value stored at offset 32.
- `start_clock_measurement_store_result`: Allocates a structure, sets its type field to eleven, stores the measurement result at offset 32, clears offset 36, then releases the structure and returns zero.
- `open_timer_device_and_set_clock`: Pops d2 and d3, opens timer.device with two zero arguments, requests unit 40, then calls a routine with the device and argument, closing the device and returning zero on failure or the device pointer on success.
- `free_clock_request_and_close_timer`: Frees the request's timer message if present, deallocates the request block, closes the timer device unit and returns zero.
- `read_clock_bcd_fields_to_seconds`: Reads the clock structure fields, converts packed BCD digits to decimal, adds day and month table offsets, and returns the total time in seconds.
- `set_clock_from_seconds`: Converts a seconds count into days, hours, minutes and seconds, then stores decimal digit pairs and a zero terminator into the clock record and refreshes it.
- `load_pointers_from_stack`: Restores a2 and a3 from the stack, then loads a3 and a2 from offsets 12 and 16 of the stack before branching to another routine.
- `compare_strings_with_case_conversion`: Compares two strings by sign-extending bytes from one and checking them against the other in a loop, returning zero if they match, one if the first is higher, or minus one if the first is lower.
- `compare_and_set_byte`: Compares a byte in a2 with d0; if it is less than d0, it sets d0 to 255 and branches.
- `return_one`: Sets d0 to one, restores a2 and a3 from the stack, and returns.
- `offset_character_value`: Loads a byte from the stack, checks if it is between 65 and 90 inclusive, and if so, sign-extends it and adds 32 to it before branching.
- `tiny_ext_ext_rts`: Small routine of 3 instructions that sign-extends d0, then sign-extends d0, then returns.
- `pulse_clock_lines_and_wait_for_ack`: Writes three clock control patterns to offsets 60 of the structure pointed to by the global pointer, sets bits at offset 52, then polls bit 1 of offset 55 up to 200 times before returning.
- `update_color_structure`: Writes a 32-bit value to offset 52 of a structure pointed to by a1, then checks a bit in that value to decide whether to write a different 32-bit value to the same offset or to write two different 32-bit values to offsets 52 and 56.
- `write_color_registers_via_a1_pointer`: Loads the pointer stored at a1 into a0 and writes two immediate color values to offsets 60 and 56 of that structure, then returns 1 in d0 and pops d2.
- `write_color_registers_from_global_pointer`: Loads the global pointer at absolute address 324 into a0 and writes the same immediate color value to offsets 60 and 52 of that structure, then returns.
- `update_clock_signal_structure`: Writes a 32-bit value to offset 52 of a structure pointed to by a1, then checks a bit in that value to decide whether to write a different 32-bit value to offset 52 or to write a 32-bit value to offset 52 and a 32-bit value to offset 56.
- `set_clock_signal_bits`: Writes 4 to offset 60 of the structure pointed to by a1, then writes the immediate patterns a fixed address and a fixed address to offsets 60 and 56 of that structure and returns 1 in d0.
- `call_subroutine_with_stack_pointer`: Passes two parameters from the stack and a pointer to a local stack slot to another routine, then returns.
- `call_alloc_mem_3`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_alloc_entry_2`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_dispatch_4`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_reschedule_4`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_library_vector_354_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_294_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_360_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `invoke_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_and_increment_2`: Stores the byte from d0 at the address in a3 and then increments a3.
- `compute_modular_result`: Computes a 32-bit result by repeatedly doubling and subtracting the divisor when the running remainder exceeds it, otherwise it performs a 16-bit modulo on the low half of the dividend and returns the combined quotient and remainder.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_arguments_to_helper`: Moves the contents of two registers into two other registers and calls a helper routine before returning.
- `call_helper_and_return_d1_3`: Moves the value from d0 to d1, calls a routine, and returns the resulting value from d1 in d0.
- `negate_result_on_flag_4`: Calls a helper routine and negates the input value if the helper returns a non-zero value in the address register, then returns the result.
- `negate_result_on_nonzero_flag_2`: Calls a helper routine and returns its first result, or the negation of that result if the helper's first return value in an address register is nonzero.
- `allocate_indexed_record_block`: Obtains a slot index from an unnamed routine and, when valid, allocates a 34-byte block through a library call with flags 65537.
- `release_record_chain`: Recursively releases a linked chain of records, processing the linked record first when its link field is set.
- `create_ext_io`: Calls a library-backed allocation routine with a second argument, and if it returns a non-null block, stores a byte value, a word from the second argument, and the first argument pointer into fixed offsets of that block.
- `delete_ext_io`: If the given pointer is non-null, it sets a byte field to 255 and clears two longword fields of the structure, then passes the pointer and a word field to another routine and returns its result.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### Wait

- `prepare_buffer_and_install_callback_2`: Finds the maximum value in a linked list, writes a header containing sequential values to a buffer, and sets up a stack frame.
- `move_and_branch_2`: Moves the contents of d1 into d0 and then branches to another routine.

### Version

- `parse_command_line_arguments_2`: Parses a command-line string into an array of pointers by identifying space-separated tokens and handling quoted strings, including converting specific character sequences into control characters.
- `process_message_reply_2`: Updates a message structure with a new value, then checks flags within that structure to conditionally call three different functions from a library table before returning the updated value.
- `check_library_version`: Parses command-line arguments to open a specified library or device and validates its version and revision numbers against provided thresholds.
- `process_message_and_call_library`: Calls a routine to handle message processing by updating message fields and calling various functions based on flags, then calls a library vector to finalize the operation.
- `parse_integer_string`: Parses a string to convert it into an integer, handling optional sign characters and digit sequences.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
- `calculate_string_length`: Calculates the length of a null-terminated string by iterating until the null byte is found.
- `call_reschedule_5`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch_5`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `recurse_with_stack_slot_2`: Calls another routine passing two parameters and a pointer to a specific location on the stack, then returns.
- `call_library_vector_198_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_414_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_444_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_552_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `invoke_global_and_verify_checksum_6`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_and_return`: Stores the byte from d0 into the memory location pointed to by a3 and increments a3 before returning.

### Preferences

- `parse_configuration_string`: Parses a string by iterating through characters to handle quoted text and whitespace, converting specific escape sequences like newline and escape into their respective characters.
- `execute_library_callbacks`: Pushes a value onto the stack, calls a routine via a pointer from a global variable, and then conditionally calls two other routines via pointers from a global variable and a structure.
- `invoke_two_function_pointers`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `call_library_function_and_store_result`: Calls a function from a library using an offset, stores the returned value into a global variable, and otherwise returns.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_delete_file`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_date_stamp`: Library stub: loads its arguments and the dos library base, calls DateStamp and returns its result to the caller.
- `call_delay`: Library stub: loads its arguments and the dos library base, calls Delay and returns its result to the caller.
- `parse_prefs_args_and_init_settings`: Parses command line arguments to identify POINTER, PRINTER, or SERIAL keywords, updates a global setting type, and initializes the preferences file.
- `initialize_preferences_display_screen`: Opens graphics and intuition libraries, allocates a screen buffer, copies a settings block, and fills a screen structure to open a window and screen.
- `load_preferences_and_run_dialog_loop`: Opens the preferences file, loads its data, and enters a loop that fetches and dispatches menu choices from a preference structure until the dialog closes.
- `close_preferences_and_release_resources`: Calls the preferences cleanup, frees the loaded file and screen resources, then releases three allocated memory blocks and returns.
- `iterate_preference_table_entries`: Calls a routine twice with different integer constants as the first argument, passing a pointer derived from a global variable and an offset as the second argument, and the input parameter as the third argument.
- `dispatch_robot_pref_slots_by_mode`: Reads a mode flag and a pointer from the stack, then calls the slot writer for either four robot slots at offset 110 or three slots at offset 102, advancing two bytes each time.
- `split_colour_word_into_nibbles`: Takes a colour word, splits it into three nibbles for red, green and blue, and passes them with the slot index and pointer to the colour setter, clearing the pointer when the index exceeds four.
- `rebuild_robot_preference_records`: Iterates through 136 robot records, shifting the pointer table down by one entry, copies word values from a preference table into each record, and assigns weapon and armour pointers based on the record type and specific bit flags.
- `apply_preferences_mode_selection`: Stores a mode index into a global variable and updates a corresponding value from a table.
- `dispatch_preferences_menu_choice`: Reads the current menu item number from the preferences structure, bounds-checks it against 37, and jumps through a jump table to the matching case, then formats and returns a status value.
- `refresh_preferences_display_fields`: Optionally calls the redraw helper with the passed pointer, then draws the speed, first and second preference values and the current language byte through graphics calls, returning zero.
- `draw_prefs_robot_colour_sliders`: Reads preference colour words, computes scaled slider positions, and calls the draw routine four times to paint the robot colour bars before refreshing the preview.
- `clamp_and_apply_robot_colour_offset`: Clamps two signed colour offsets to the valid range, stores them in the preference block, and updates the preview object position when it changed.
- `map_colour_index_to_palette_slot`: Reads the word argument from the stack and returns 3 when it equals 9, otherwise returns 4.
- `select_byte_for_odd_flag`: Reads the byte argument from the stack, tests its lowest bit, and returns 36 when set or 37 when clear in d0.
- `compare_strings_case_insensitive`: Walks two byte strings from the stack arguments, folding lowercase letters to uppercase with a mask, and returns true only if both reach a zero terminator together.
- `call_forbid_2`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit_2`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem_4`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem_2`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_add_task`: Library stub: loads its arguments and the exec library base, calls AddTask and returns its result to the caller.
- `call_rem_task`: Library stub: loads its arguments and the exec library base, calls RemTask and returns its result to the caller.
- `call_find_task`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_wait`: Library stub: loads its arguments and the exec library base, calls Wait and returns its result to the caller.
- `call_signal`: Library stub: loads its arguments and the exec library base, calls Signal and returns its result to the caller.
- `call_alloc_signal`: Library stub: loads its arguments and the exec library base, calls AllocSignal and returns its result to the caller.
- `call_free_signal`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `call_add_port`: Library stub: loads its arguments and the exec library base, calls AddPort and returns its result to the caller.
- `call_rem_port`: Library stub: loads its arguments and the exec library base, calls RemPort and returns its result to the caller.
- `call_get_msg`: Library stub: loads its arguments and the exec library base, calls GetMsg and returns its result to the caller.
- `call_reply_msg`: Library stub: loads its arguments and the exec library base, calls ReplyMsg and returns its result to the caller.
- `call_find_port`: Library stub: loads its arguments and the exec library base, calls FindPort and returns its result to the caller.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_open_device`: Library stub: loads its arguments and the exec library base, calls OpenDevice and returns its result to the caller.
- `call_close_device`: Library stub: loads its arguments and the exec library base, calls CloseDevice and returns its result to the caller.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `call_send_i_o`: Library stub: loads its arguments and the exec library base, calls SendIO and returns its result to the caller.
- `call_abort_i_o`: Library stub: loads its arguments and the exec library base, calls AbortIO and returns its result to the caller.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_init_semaphore`: Library stub: loads its arguments and the exec library base, calls InitSemaphore and returns its result to the caller.
- `call_obtain_semaphore`: Library stub: loads its arguments and the exec library base, calls ObtainSemaphore and returns its result to the caller.
- `call_release_semaphore`: Library stub: loads its arguments and the exec library base, calls ReleaseSemaphore and returns its result to the caller.
- `call_attempt_semaphore`: Library stub: loads its arguments and the exec library base, calls AttemptSemaphore and returns its result to the caller.
- `call_library_vector_78_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `allocate_indexed_record_block_2`: Obtains a slot index from another routine and, when valid, allocates a 34-byte block using flags 65537.
- `release_record_chain_2`: Checks if a long value at offset 10 of a structure is non-zero to recursively call itself, then marks a byte at offset 8 as 255, sets a long at offset 20 to negative one, calls another routine with a byte from offset 15, and finally calls another routine with a constant value of 34.
- `format_clock_digits_into_buffer`: Loads a longword from a global variable, writes slash and colon separators into a buffer, and divides the value by 60 and 10 to store the resulting ASCII digits into the buffer.
- `compute_date_from_day_counter`: Reads a day counter from a global variable, subtracts month lengths from a table while handling leap years, and calls another routine with the calculated day, month, and year values.
- `format_three_numbers_as_digits`: Takes three longword arguments from the stack, divides each by ten and writes the tens and units digits plus separators as ASCII characters into a global buffer, then returns zero.
- `draw_prefs_labels_and_selected_value`: Calls text and drawing helpers to print preference labels, copies digit bytes into a scratch buffer, looks up a table entry from the selected digits and draws it, then returns zero.
- `compute_date_from_prefs_and_draw`: Converts digit characters from two global buffers into numbers, adjusts the year and month, accumulates days per month, then calls a drawing routine with the computed date and returns zero.
- `validate_date_input_buffer`: Validates a date by checking ASCII characters in a buffer, converts them to numeric values, and calls another routine with the day, month, and year after ensuring the date is valid for the given month.
- `wrap_month_day_then_validate_date`: Wraps d2 to one when above twelve and d4 to zero when above ninety-nine, pushes d3, d2 and d4, calls the date validation routine and restores the saved registers.
- `clamp_date_input_buffer_2`: Checks characters at specific offsets in a global buffer and replaces them with alternative characters if they fall below certain thresholds to ensure valid date ranges.
- `clamp_date_input_buffer`: Clamps date values parsed from an ASCII buffer to valid ranges and calls another routine with the resulting day, month, and year.
- `clamp_month_year_then_call_date_setter`: Forces d3 to twelve when below one and d4 to ninety-nine when negative, pushes d2, d3 and d4, calls the date setter, cleans the stack and restores d2-d4.
- `open_timer_and_build_date_strings`: Opens timer.device, allocates a structure, computes a date offset, converts day, month, year and time values into digit pairs stored in the structure, then formats and returns one.
- `allocate_and_store_pointers`: Saves d2 through d4 and a2 through a4 on the stack, loads three addresses into a2, a3, and a4, clears two slots on the stack, calls a routine, stores d0 in the address at a3, and if d0 is not zero, pushes the address at a3 onto the stack, calls another routine, stores d0 in the address at a4, and if the address at a3 and a4 are both non-zero, calls another routine before setting d0 to 0 and branching.
- `open_timer_device`: Pushes a pointer from a4 to the stack, calls a routine with device numbers for timer.device, and branches based on the return value.
- `update_preference_display_loop`: Calculates a value based on a subtraction from sixty, enters a loop that calls a routine to draw a preference, updates a bitmask using bytes from a structure, increments a global byte, and conditionally calls another routine using a pointer from a global variable.
- `restore_registers_and_return_5`: Restores several data and address registers from the stack and returns immediately.
- `allocate_time_task_and_install_callback`: Allocates a task structure, sets bit 2 of a global flag, stores the task pointer, installs a callback at offset 10 and schedules it.
- `stop_time_task_and_release_resources`: Checks a signal bit, stops the time task, frees its memory and port, clears the task pointer, clears flag bit 2 and returns.
- `set_prefs_handler_type_and_args`: Loads two long arguments and a structure pointer from the stack, stores type 9 and both arguments into offsets 28, 32 and 36 of that structure, then calls the routine at address 448 with the structure pointer.
- `poll_counter_low_nibble_change`: Saves d2, reads the low nibble of a global counter, calls a delay routine with 100, rereads the counter and returns 1 if the nibble changed, otherwise 0.
- `reset_audio_periods_and_wait_flag`: Writes three period values to offset 60 of the global structure unless bit 2 of byte 63 is set, ORs a period mask into offset 52, then polls bit 1 of byte 55 up to 200 times.
- `clear_audio_mask_bit`: Clears the least significant bit and the bit at offset 16 of a longword in a global structure.
- `open_system_configuration_and_read_prefs`: Opens DEVS:system-configuration, reads preference records with identifiers 1005 and 1006, then reads a 232-byte block into the caller's buffer and returns the result or zero.
- `load_system_configuration`: Attempts to open a file and read its contents into a buffer, then performs conditional logic to either close the file or call other routines to process the data based on the result of the read operation.
- `return_d2_in_d0`: Moves the value from d2 into d0 and returns.
- `call_schedule_2`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_exception_2`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_init_code`: Library stub: loads its arguments and the exec library base, calls InitCode and returns its result to the caller.
- `call_make_functions_2`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_find_resident_2`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_enable_2`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_forbid_3`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `call_set_int_vector`: Library stub: loads its arguments and the exec library base, calls SetIntVector and returns its result to the caller.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_allocate`: Library stub: loads its arguments and the exec library base, calls Allocate and returns its result to the caller.
- `call_alloc_mem_5`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry_3`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_free_entry`: Library stub: loads its arguments and the exec library base, calls FreeEntry and returns its result to the caller.
- `call_find_task_2`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_signal_2`: Library stub: loads its arguments and the exec library base, calls Signal and returns its result to the caller.
- `call_wait_port`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `call_add_library`: Library stub: loads its arguments and the exec library base, calls AddLibrary and returns its result to the caller.
- `call_old_open_library`: Library stub: loads its arguments and the exec library base, calls OldOpenLibrary and returns its result to the caller.
- `call_global_vector_with_stack_args_2`: Loads the first stack argument into d0, pushes a pointer to the second argument and that value, then jumps through a global vector at offset four and cleans the stack before returning.
- `process_preference_selection_loop`: Saves registers, updates several global variables, calls routines to draw sprites and preference fields, and enters a loop that processes input events and copies or swaps preference records until a selection is made.
- `compare_preference_value`: Clears d2, loads a pointer from a2 into a0, compares the longword at offset 20 of that pointer to 16, and if they are equal, loads a pointer from a3 into a0, then loads the address at offset 86 of that pointer into a0, then loads the address at offset 20 of that new pointer into a0 before branching.
- `dispatch_preferences_menu_choice_3`: Checks if a value at offset 20 of a structure is 16, then calls a routine with a pointer from a2 and another with a pointer from a3 to update a value and potentially call a third routine.
- `dispatch_preference_message`: Checks a value at offset 20 of a structure in a2, calls another routine if it equals 64, and then calls a routine to restore and copy data before branching based on the result.
- `call_restore_and_copy`: Pushes a pointer and a pointer from a6 onto the stack, calls another routine, sets d0 to zero, adjusts the stack pointer, and branches to a previous instruction.
- `push_arguments_and_branch_2`: Pushes the address in a4 and a constant onto the stack, then branches to another routine.
- `store_result_from_call`: Loads a pointer from a structure into a0, calls a routine using a pointer at offset 86 of that structure, and stores the result in a location pointed to by a2.
- `restore_registers_and_return`: Restores d2, a2, a3, and a4 from the stack and returns.
- `copy_prefs_fields_a8_to_da`: Copies five words at offsets a8, aa, ac, ae, and da from the source structure in a1 into the destination structure in a0, then returns.
- `dispatch_via_table_3`: Loads a pointer from a table, retrieves a value from offset 38 of the structure pointed to by that pointer, and performs a jump to an address from a table based on that value after checking if it is between 64 and 74 inclusive.
- `call_stub_and_return`: Pushes a constant onto the stack, calls another routine, sets d0 to one, adjusts the stack pointer, and branches to a previous instruction.
- `restore_registers_and_return_2`: Restores a2 and a3 from the stack and returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `draw_selected_pref_entry_with_pen`: Saves d2 and a2, sets pen 1 on a graphics structure, draws two table entries offset by 20 with pens 47 and 56, and then sets pen 255 and returns.
- `save_and_clear_counter`: Pushes the value in d2 onto the stack, loads a word from the stack into d1, clears d2, and branches to another routine.
- `increment_and_compare_table`: Increments d2, stores it in d0, doubles d0, compares d1 against a table at offset 16 using d0 as an index, and branches based on the result or the value of d2.
- `decrement_register_value`: Returns the value of d0, which is the value of d2 minus one.
- `load_six_preference_buffers_and_copy`: Allocates six buffers via a library call using sizes from a table, copies each source block into its buffer, then stores the six pointers into preference fields and returns zero or one on failure.
- `call_library_routine`: Calls another routine with a pointer to a global variable and an integer argument.
- `branch_on_bit_test`: Clears d0, loads a word from an offset of a global variable into d0, tests if bit 12 is set, and if so, pushes three constants onto the stack and branches to another routine.
- `call_other_with_three_args`: Pushes the constants 2 and 3 and a computed pointer value onto the stack, calls another routine with those three arguments, then cleans the stack and returns.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `load_and_sort_printer_list`: Opens the printer device, reads printer names into a buffer, pads them with spaces, allocates a pointer array, links the entries, and bubble-sorts them, returning the sorted list or null.
- `compare_string_prefixes`: Compares two null-terminated strings character by character.
- `allocate_node_and_copy_thirty_bytes`: Allocates a thirty-four byte block, links it at the head of the global list, copies thirty bytes from the caller's buffer into the node body and returns zero.
- `free_all_list_nodes_and_reset_globals`: Frees the global buffer sized from the count, then walks the global node list freeing each node, clears the count and list head and returns zero.
- `scan_pref_entries_until_match`: Walks a table of preference entry pointers starting from an index in a global variable, calling a comparison routine on each entry; if a match is found, it clears thirty bytes of a global buffer and returns 1, otherwise it resets the index and returns 0.
- `compare_pref_entry_against_string`: Builds a space-padded thirty-byte buffer from the global preference bytes, then compares it byte by byte with the caller's string, returning 1 if all match and 0 otherwise.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_78_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_318_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_354_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_draw_for_index_range`: Takes two word arguments from the stack and, for each index from the first through the second, pushes the table entry and a global pointer before calling the drawing routine.
- `draw_sprite_range_with_mask`: Pushes d2-d4, loops from the first word argument to the second, calling the draw routine with each sprite pointer from the table, a mask of minus one and the global bitmap, then restores registers.
- `dispatch_preferences_menu_choice_2`: Saves registers, loads three data pointers, reads the menu selection word, bounds-checks it against 8 and jumps through a table of case handlers, returning 1.
- `render_preferences_panel_lists`: Saves registers, locks the screen, then draws three groups of list entries from the coordinate tables using the passed bitmap and panel index, finally unlocking and returning.
- `calculate_string_length_2`: Calculates the length of a null-terminated string by iterating through memory until a zero byte is found, then returns the difference between the end pointer and the start pointer in d0.
- `draw_robot_icon_grid_cells`: Saves d2-d3/a2, opens a graphics context, then draws several filled and outlined rectangles at computed grid coordinates for a robot icon before closing the context.
- `draw_robot_icon_border_rectangles`: Saves d2-d4/a2, opens a graphics context, then draws filled and outlined rectangles around a robot icon using the passed x, y, width and height values before closing it.
- `select_prefs_sound_effect_id`: Tests the word argument on the stack and pushes four parameters, choosing value 153 when nonzero or 38 when zero, then calls the preference sound routine and cleans the stack.
- `allocate_two_prefs_buffers_and_init`: Allocates a 92-byte and a 16000-byte block via the memory routine, initialises fields at offsets 8, 9, 54, 58 and 62 of the first block, and returns it or zero on failure.
- `flush_pending_prefs_sound_buffer`: If the global sound pointer is set, sets a flag, calls the sound routine with a shifted mask, waits until the pointer clears, then frees the passed block.
- `invoke_global_and_verify_checksum_2`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_at_pointer_2`: Stores a byte from register d0 at the memory location pointed to by a6 and increments the pointer.
- `compute_modular_result_2`: If the divisor in d2 is greater than 65535, it computes a 32-bit quotient and remainder using a bit-shifting loop; otherwise, it performs 16-bit division and modulo operations on the high and low words of the dividend in d1 and returns the combined results in d0.
- `multiply_32_bit_integers_2`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper_7`: Pushes a 32-bit value from a register onto the stack, calls a helper routine, and then restores the value from the stack before returning.
- `call_helper_and_return_d1_4`: Pushes the value in d2 onto the stack, moves d1 into d2 and d0 into d1, calls a helper routine, moves the resulting value from d1 into d0, restores d2 from the stack, and returns.
- `negate_result_on_flag_5`: Calls a helper routine and negates the returned value if the helper leaves a non-zero value in the address register.
- `negate_result_on_nonzero_flag_3`: Calls a helper routine and returns its first result, or returns the negation of that result if the second result in an address register is nonzero.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_increment_3`: Stores a byte from register d0 into the memory location pointed to by a3 and increments a3.
- `preferences_dialog_setup_and_input_loop`: Saves registers, configures the preferences dialog widgets and text, then loops reading input events and copying or swapping preference records until a selection is made.
- `copy_preference_fields_between_records`: Copies the word at offset two and the bytes at offsets 183, 182 and 184 from the source record in a1 to the destination record in a0, then returns.
- `dispatch_menu_selection_2`: Reads a command ID from a structure and uses it to index a jump table or update stateful counters in global variables.
- `branch_to_unreached_code`: Sets d0 to two and performs two unconditional branches.
- `prefs_apply_selected_option`: Pushes a2, calls five preference routines to set flags, draw a box and print the option text indexed by the selection counter, stores that counter to the prefs state and returns zero.
- `update_library_vector_flags`: The routine calls another routine multiple times with a base pointer from a global variable and constants 1, 1, 0 and 430 with 36, then with a value from a table indexed by a global variable and constant 5.
- `process_preferences_message_loop`: Initializes a preference block and enters a loop that retrieves messages, checks if the message type or gadget number matches certain criteria, and either copies a new preference block or updates a global variable before potentially calling a routine to copy or flush buffers and then returns.
- `copy_prefs_block_and_flush_buffers`: Copies three words, two bytes and thirty-six words from the source preference block to the destination, then if the mode byte equals three, flushes three buffers and returns.
- `load_prefs_and_open_requester`: Opens a preferences file, copies four, three, and thirty-six word tables into a buffer, builds and opens a requester, reads three gadget values, and calls an apply routine.
- `dispatch_prefs_gadget_by_number`: Loads a gadget pointer and its associated data from a global structure, reads a gadget number at offset 38, and if the number is between 75 and 88, performs a table lookup to jump to a specific handler routine; otherwise, it returns zero.
- `set_three_color_gadgets_from_palette`: If a global byte equals three, it reads a palette word from a table, splits it into three nibbles, and calls three separate routines to set the red, green, and blue values of three color gadgets.
- `store_palette_entry_and_forward`: Reads five arguments from the stack, computes a palette index from the row and column values, writes the packed colour word into the palette table, then forwards all arguments to another routine.
- `draw_four_edges_of_selected_tile`: Uses the selected tile index to read four corner bytes from a table and calls the rectangle drawing routine four times to outline the tile edges.
- `clear_three_preference_slots`: Calls the preference setter three times with the same screen pointer and minus one, clearing three consecutive preference entries at offsets 304, 308 and 312.
- `query_and_adjust_state`: Adjusts a parameter, then calls another routine repeatedly with coordinates derived from the input parameters and conditional flags based on a global variable.
- `configure_display_modes`: Calls a library routine, then enters a nested loop that iterates through a 16x16 grid, calculates a value from a global configuration table, and calls another routine with that value.
- `set_bitplane_pixel_bits`: Clears then sets two bit-plane bits at a computed offset in the preference bitmap, using the bit index and pixel value passed on the stack, then restores registers and returns.
- `draw_prefs_robot_icon_and_highlight`: Sets up the preferences screen drawing context, draws the robot icon at computed coordinates, checks the returned pixel value and draws a highlight rectangle before restoring registers.
- `process_preferences_input_loop`: Copies a name buffer and three preference values into a structure, then enters a loop that reads input events to update a name field and preference settings.
- `copy_preferences_struct_fields`: Copies the word fields at offsets 126, 176, 180, 158 to 166 and 178 plus the byte at offset 1 and the 30-byte name buffer from the source struct to the destination struct.
- `dispatch_pref_menu_command`: Loads the current preferences object, reads its command word at offset 38, and jumps through a table for values between 38 and 63, returning a status in d0.
- `redraw_pref_slider_values`: Pushes many arguments and calls the preferences drawing routines to render the slider labels and the three values indexed by the current selection counter.
- `clamp_pref_slider_to_max`: Ensures the preference slider value at offset 166 is at least one, clamps it to the maximum, copies it to offset 164 and calls the redraw routine.
- `apply_first_pref_value_and_refresh`: Saves registers, reads the first preference word into d0, stores it to the preference value, formats and displays it, then redraws the selected gadget and returns.
- `clamp_pref_values_to_limits`: Loads the preference block, forces the first word to at least one, clamps the second word against the stored limit, then calls the apply routine and returns.
- `apply_second_pref_value_and_refresh`: Saves registers, reads the second preference word into d0, stores it to the limit value, formats and displays it, then redraws the selected gadget and returns.
- `initialize_preference_entry`: Compares a longword in a global variable to one and, if it is less than one, sets a word at offset 194 of a global variable and calls another routine.
- `copy_pref_word_and_refresh_display`: Copies the word at offset 220 of a structure into a long at a global variable, calls three routines to handle memory and scheduling, and then calls a fourth routine using a table entry indexed by a word from a different structure.
- `draw_preference_dialog_fields`: Saves a2 and a3, calls routines to set up the dialog, then draws five preference fields by masking and shifting words at offsets 158 to 180 and adding fixed offsets before returning.
- `execute_preference_sequence`: Calls three different routines using pointers from a structure and a table, potentially modifying a byte at offset 224 of a table, and uses a value from a global table to index into another table before calling a fourth routine.
- `dispatch_preference_message_alt`: Compares a value at offset 20 of a structure in a2 to 64, calls another routine, and then processes the result through a series of conditional branches and a library call.
- `call_restore_and_copy_tail`: Pushes a pointer from a4 and a constant onto the stack, calls another routine, sets d0 to zero, adjusts the stack pointer, and branches to a previous instruction.
- `push_pointers_and_branch`: Pushes a constant and the address stored in a4 onto the stack, then branches to another routine.
- `process_message_pointer`: Loads a pointer from offset 54 of a structure in a0 into the stack, calls another routine, stores the result in a2, and branches based on the result.
- `restore_registers_and_return_3`: Restores several registers from the stack and returns immediately.
- `copy_prefs_tail_fields`: Copies three words and two bytes from offsets da, dc, de, e0, and e1 of the source structure in a0 into the same offsets of the destination structure in a1, then returns.
- `dispatch_menu_selection`: Reads a value from a structure pointed to by a global variable, checks if it falls within a specific range, and uses it as an index into a jump table to call another routine.
- `call_routine_with_pointer_argument`: Passes the address of a global variable to another routine, sets the return value in register d0 to one, and then branches to a different location.
- `restore_registers_and_return_4`: Restores registers d2, a2, and a3 from the stack and returns immediately.
- `branch_on_bit_status`: Saves registers d2, a2, and a3, loads two constant pointers into a2 and a3, and checks if bit 15 is set in a word at offset 3502 of a3; if bit 15 is not set, it branches to another routine, otherwise it pushes three values onto the stack and branches to another routine.
- `draw_preferences_widgets_2`: Calls a routine via a pointer in a2 with three arguments to draw various widgets based on bits set in bytes at offsets 224 and 219 of a structure in a3, and calls another routine with two arguments and two pointers if bit 3 of offset 219 is set.
- `draw_preferences_widgets`: Calls a routine via a pointer in a2 with three arguments to draw various widgets based on bits set in bytes at offsets 224 and 219 of a structure in a3, and also checks bit 8 of a word at offset 218 of a3 to determine which widget to draw next.
- `check_conditional_preference`: Tests bit 8 of a word at offset 218 of a structure in a3; if the bit is not set, it branches to another routine, otherwise it pushes three arguments onto the stack and branches to another routine.
- `execute_conditional_cleanup_2`: Calls a routine via a pointer in a2 with three arguments, then checks bit 8 of a word at offset 218 of a structure in a3 to decide whether to call the same routine again with a different third argument.
- `execute_conditional_cleanup`: Calls a routine through a pointer in a2 using three arguments, then tests bit 4 of a byte at offset 219 of the structure in a3; if the bit is not set, it pushes three arguments and branches to another routine, otherwise it returns.
- `test_bit_and_push_arguments`: Tests bit 5 of a byte at offset 13 of the structure in a3; if the bit is not set, it pushes three values onto the stack and branches to another routine.
- `call_cleanup_if_bit_set`: Tests bit 6 of a byte at offset 13 of a structure in a3; if set, it pushes three arguments onto the stack and calls a routine through a pointer in a2, otherwise it returns.
- `test_bit_and_call_cleanup`: Tests bit 7 of a byte at an offset from a pointer in a3; if set, it pushes three values onto the stack and calls a routine through a pointer in a2, then moves 3 into d2 and branches to another location; otherwise it branches to another location.
- `call_cleanup_handlers`: Pushes three arguments onto the stack and calls a routine through a pointer in a2, then checks a value at an offset of a3 and calls another routine if a bit is set.
- `call_two_routines_with_d2_argument`: Calls the routine at the fixed address, moves d2 into d1, sign-extends it, pushes it as an argument to a second call, then restores d2, a2 and a3 and returns.
- `draw_prefs_dialog_widgets`: Pushes widget identifiers and the current gadget pointer onto the stack and calls five drawing routines to build the preferences dialog, then releases the stack and returns.
- `setup_stack_frame`: Saves d2 through d4 on the stack, loads a pointer from offset 16 of the stack into a0, loads a byte from offset 23 of the stack into d3, clears d4, sets d0 to 0, and branches.
- `skip_whitespace_in_string`: Increments a pointer in a0, checks if the byte at the new location is null or a space, and otherwise branches to another routine.
- `convert_ascii_to_scaled_value`: Checks if a byte at a pointer in a0 is a digit between 0 and 9 or a period, then performs a scaled calculation using d0 and the digit value to update d0 before returning.
- `calculate_scaled_offset`: Adds d0 to itself, moves the result to d1, shifts d0 left by two, adds d1 to d0, and branches to a previous instruction.
- `convert_ascii_to_scaled_value_2`: Parses a numeric string from a0, performing scaled arithmetic based on decimal digits and an optional period, and returns the result in d0.
- `check_buffer_size`: Compares a value at offset 16 of a structure in a6 with a value at offset 12 of the same structure, and clears d5 if the first is not less than or equal to the second.
- `copy_bytes_from_table_in_loop`: Copies bytes from a table to a destination buffer using a loop that increments a counter and a pointer, then calls a routine using a pointer from a global variable.
- `store_formatted_digits`: Calls two routines to process a value in d4, stores the resulting bytes into a stack-based array indexed by d5, and updates d4 and d5 based on a loop counter.
- `format_robot_number_into_buffer`: Builds a decimal string in the buffer at d2 from the digit bytes indexed by d5, inserting a leading zero and decimal point when the flag byte at offset 27 of a6 has low bits set, then null-terminates and returns 1.
- `draw_preferences_dialog_widgets`: Reads a nibble from the preferences data, then calls drawing routines with widget coordinates and table entries to render the dialog, and finally invokes the refresh routine with the window and data pointers.
- `close_prefs_windows_if_flag_set`: Tests bit 256 of the word at offset 12 of the window structure; if set, calls the close routine for two widgets and releases the two window pointers, then restores a2 and returns.
- `call_two_cleanup_handlers_with_zero`: Pushes zero and two global pointers as arguments and calls two cleanup routines through the library vector, then discards the stacked arguments and returns.
- `draw_pen_with_style_and_bounds`: Saves registers, sets pen colours from two byte arguments, computes rectangle bounds from a table index, then calls the pen, rectangle and release routines before restoring registers.
- `store_packed_colour_and_set_pen`: Saves registers, packs three colour components into a word stored in a table indexed by the adjusted pen number, then calls the pen-setting routine with the same values.
- `apply_palette_nibbles_to_three_objects`: Reads a word from a table indexed by the argument minus thirteen, splits it into three nibbles, stores them in three globals, then passes each nibble with object fields to two subroutines for three objects.
- `create_std_io`: Passes its one argument to another routine together with the constant 48.
- `delete_std_io`: Passes its one argument on to another routine and returns nothing.
- `allocate_and_initialize_structure`: If the first argument is non-zero, it calls an allocation routine with a constant value and a second argument; if successful, it stores the value 5 at offset 8, the second argument as a word at offset 18, and the first argument as a long at offset 14 of the new block, then returns the block pointer.
- `invalidate_and_free_structure`: If the provided pointer is non-null, it sets a byte at offset 8 to 255, sets two longword fields at offsets 20 and 24 to negative one, and calls another routine with the pointer and a word from offset 18; otherwise, it returns zero.

### SetMap

- `initialize_keymap_and_lookup_table`: Loads a keymap file, parses its contents into a lookup table by handling quoted strings and special character conversions, and populates several global pointers with the resulting data structure.
- `update_callback_state`: Retrieves a pointer from a global variable and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `call_dispatch_6`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_user_state_2`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `call_reschedule_6`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `flush_pending_map_entries_then_close`: Walks the pending entry counter backwards, calls the entry processor for each non-empty slot, then closes the map file, frees buffers and passes the argument to the resource cleanup routine.
- `store_map_pointers_and_open_map_file`: Saves the two arguments into the map pointer globals, opens the map file with the given name and mode, then calls the flush routine with a small count and returns.
- `load_keymap_and_build_lookup_table`: Opens and reads a keymap file, converts the filename to lowercase, and populates a lookup table with up to four keymap entries by iterating through available resources and validating pointer addresses.
- `call_library_vector_240_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_198_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_450_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_414_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_456_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_294_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_444_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_552_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_global_vector_with_stack_args_3`: Loads the first stack argument into d0, pushes a pointer to the second argument and that value onto the stack, then calls a routine via a global vector at offset 4 and cleans the stack before returning.
- `call_library_vector_348_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `invoke_global_and_verify_checksum_3`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_and_increment_pointer_2`: Stores a byte from register d0 into the address pointed to by register a3 and increments the pointer by one before returning.

### Format

- `parse_command_line_string_3`: Parses a command line string by splitting it into tokens, handling quoted substrings and converting specific character sequences into control characters, then stores the resulting pointers and data into a structure.
- `update_callback_table_and_cleanup`: Retrieves a pointer from a global variable and stores a passed parameter into a slot within a structure pointed to by that pointer.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `call_alloc_mem_6`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_rem_int_server_2`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_init_resident_2`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_debug`: Library stub: loads its arguments and the exec library base, calls Debug and returns its result to the caller.
- `call_switch_2`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_forbid_4`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_dispatch_7`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_free_mem_3`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_schedule_3`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_make_functions_3`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_alloc_abs_2`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_reschedule_7`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `format_disk_initialize_and_verify`: Allocates memory buffers and initializes resources, parses command line arguments including DRIVE, NAME, FFS, NOICONS, and QUICK, then verifies the disk contents and formats the disk if necessary, reporting errors or prompting for a disk insertion.
- `free_format_buffers_and_resources`: Tests each of five global buffer pointers and frees the allocated ones with their sizes, then calls the cleanup routine with the passed argument and restores d2.
- `handle_disk_error`: Calls a routine to process an error code, then calls another routine with several parameters to display error messages.
- `format_disk_with_error_messages`: Selects a message string based on d2, prints drive and error text, calls the format helper with parameters, then returns the sign-extended byte from d3 in d0.
- `lookup_byte_index_and_store_offset`: Scans eight bytes at a table for the value in d2, stores the matching byte into offset 148 of a structure returned by a call, and returns d2.
- `walk_format_list_match_drive`: Walks a linked list of format entries, compares drive numbers and calls a match helper, builds two message strings and returns one on success or zero.
- `calculate_string_length_3`: Iterates through a null-terminated string starting from a pointer passed on the stack, then returns the number of bytes between the start and the null terminator in d0.
- `copy_cstring_to_buffer`: Library code that copies a null-terminated byte string from a source pointer to a destination pointer, advancing both pointers and writing each byte until a zero byte is encountered.
- `convert_char_to_uppercase`: Checks if a byte is a lowercase ASCII letter and, if so, returns that byte minus thirty-two; otherwise, it returns the original byte unchanged.
- `compare_strings_case_insensitive_2`: Compares two null-terminated strings by calling a routine to convert each character to uppercase and checking for equality; it returns 1 if the strings match and 0 otherwise.
- `truncate_string_at_colon`: Replaces the first colon in a string with a null terminator, cutting the string short.
- `call_helper_with_arguments`: Sets up a stack frame, copies a longword from offset 8 of a6 to d0 and a longword from offset 12 of a6 to d1, stores d0 at offset -16 of a6, pushes three values onto the stack, calls another routine with d1 as an argument, adjusts the stack pointer, and returns 0 if the call succeeds or 126 if it fails.
- `create_and_init_format_context`: Allocates a 68-byte context via the memory allocator, links its list nodes, stores a disk handle and a callback, copies up to seven words from a provided table into the context, then invokes the callback and frees the context, returning a stored result.
- `call_library_vector_354_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_198_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_330_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_450_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_414_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_456_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_294_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_210_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_444_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_552_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_366_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_360_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_306_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_library_vector_384_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_global_vector_with_stack_args`: Loads the first stack argument into d0, pushes a pointer to the second argument and that value, then jumps through the global vector at address four and cleans the stack.
- `call_library_vector_348_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_72_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `format_disk_with_progress_display`: Opens a disk device, allocates memory, and reads disk geometry to prepare for formatting.
- `format_number_with_padding`: Pushes width and pad arguments, calls the formatter with the stream handle, formats the input value, then writes the result through the stream and returns.
- `invoke_global_and_verify_checksum_4`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_at_pointer_3`: Stores a byte from d0 into the memory location pointed to by a3 and increments a3.
- `perform_division_operations`: If a value in d2 is not greater than 65535, it performs several division operations using a1 and d2 to calculate values and returns them in d0 and d1.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers_3`: Computes the 32-bit product of two 16-bit values stored in d0 and d1 by calculating partial products and summing them, then returns the result in d0.
- `invoke_table_entry_via_exec`: Retrieves a function pointer from an offset within a global variable and calls it using a library function address retrieved from a global variable.
- `store_byte_and_increment_pointer_3`: Stores a byte from register d0 into the memory location pointed to by a3, then increments a3 by one before returning.
- `allocate_indexed_record_block_3`: Obtains a slot index from another routine and, if valid, allocates a 34-byte block using flags 65537.
- `release_record_chain_3`: Checks if a pointer at offset 10 of a structure is non-zero and calls itself recursively; then marks a byte at offset 8 as 255, sets a longword at offset 20 to negative one, calls another routine with a byte from offset 15, and finally calls a routine with the value 34.
- `call_io_with_constant`: Passes a single argument from the stack to another routine along with the constant 48.
- `forward_io_argument`: Retrieves a longword argument from the stack, pushes it back onto the stack, and calls another routine.
- `format_trashcan_device_and_open`: Saves registers, swaps a system structure field, fills a device block with defaults or caller values, formats the Trashcan path and opens it, returning the result code.
- `execute_with_status_check`: Calls a routine via a global pointer and returns either a byte from a stack-allocated buffer or a negative error code if the call fails or returns a non-success status.
- `call_library_vector_240_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_342_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `update_linked_list_node`: Stores the address of a pointer argument into the memory location pointed to by that argument, increments the value at that location by four, clears the following word, and stores the original pointer value into the word after that.
- `allocate_and_initialize_structure_2`: Calls an allocation routine with a constant value and a second argument, then if successful, stores the value 5 at offset 8, the second argument as a word at offset 18, and the first argument as a long at offset 14 of the returned block.
- `invalidate_and_call_ext_io`: If the provided pointer is non-null, it sets a byte at offset 8 to 255, sets two longword fields at offsets 20 and 24 to -1, and calls another routine with the pointer and a 16-bit value from offset 18; otherwise, it returns zero.
- `call_library_vector_90_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.

### CLI

- `parse_command_line_and_start`: Opens the DOS library and allocates a control block to parse the command line into individual argument strings, handling double quotes and asterisk escapes for special characters before executing the main routine and cleaning up allocated resources.
- `call_supervisor_3`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr_2`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_switch_3`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_dispatch_8`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_make_library_2`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions_4`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_find_resident_3`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_enable_3`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_entry_4`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `open_cli_window_and_run_startup`: Creates a console window and shell control block based on a specification, then executes the system startup scripts.
- `build_linked_node_chain`: Walks a list of entries, allocating an 8-byte node for each and storing a value from the entry in it, then links the nodes together with shifted pointers.
- `traverse_and_free_linked_list`: Recursively traverses a linked list by calling another routine on the second element of each node before freeing the node itself.
- `parse_signed_decimal_string`: Converts a null-terminated string of ASCII digits into a signed 32-bit integer, handling an optional leading plus or minus sign.
- `call_library_vector_198_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_414_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `compute_modular_result_3`: If the divisor in d2 is greater than 65535, it performs a 32-bit division-like loop by doubling the dividend and subtracting the divisor 16 times; otherwise, it computes a 16-bit quotient and remainder using the low 16 bits of the dividend and the divisor, returning the results in d0 and d1.
- `multiply_32_bit_signed_integers`: Computes the 32-bit product of two 32-bit integers by calculating partial products of their 16-bit components and summing them.
- `negate_result_on_nonzero_address`: Calls another routine and returns its first result, or the negation of that result if the second result in an address register is nonzero.

### DiskCopy

- `allocate_and_initialize_disk_copy_buffer`: Allocates memory for a structure, copies a command line into it, and initializes several fields including a buffer size and a pointer to a string.
- `disk_copy_cleanup_and_exit`: Calls an optional exit hook, frees the allocated memory, closes the opened library, replies pending messages, restores the saved stack pointer and returns the exit code in d0.
- `tokenize_command_line_into_argv`: Parses a string into whitespace-separated tokens, handling double quotes, and stores pointers to these tokens in a table.
- `tiny_move_moveq_rts`: Small routine of 3 instructions that copies d0 to d5, then loads 0 into d0, then returns.
- `execute_disk_copy_process`: Parses disk copy arguments, opens required libraries and windows, and manages a state machine that performs disk reading, writing, and sector verification while handling disk swaps and user input via a loop.
- `cleanup_or_report_diskcopy_error`: Tests the longword at offset 1844 of a4; if set, it calls a routine with the value as an argument, otherwise it calls a disk operation routine with several stack arguments, then calls another routine with a value of ten and returns.
- `close_diskcopy_resources_and_finish`: Frees memory blocks, closes libraries and windows, clears the file list, prints the finished or abandoned message, closes the window and calls the exit routine with the saved argument.
- `advance_disk_copy_state_machine`: Advances the disk copy state machine by checking track and sector counters, handling write protection, and managing disk change signals.
- `report_diskcopy_error_by_code`: Stores a message pointer in the a4-relative error slot based on the error code argument, prints it via the debug or display routine, then calls the error reporting routine with the code and returns zero.
- `check_disk_change_signal_bits`: Calls the exec SetSignal function with a mask, tests bits 12 and 13 of the returned signals and the a4 flags, and returns one if a disk change was detected, otherwise zero.
- `open_device_wait_for_char_or_copy`: Tests the longword at offset 1844 of a4; if set, it calls a routine with a value from a global variable, otherwise it performs a sequence of disk operations including reading a byte, checking signals, and potentially calling a disk copy routine, before calling another routine and returning.
- `compare_and_open_dos_locks`: Compares two lock values, opens DOS locks for the two path strings, stores the results, and reports an error if either lock is missing.
- `close_both_saved_device_handles`: Flushes and closes the two device handles saved in the globals through exec CloseDevice, restoring a6 from the stack afterwards.
- `prompt_retry_until_user_confirms`: Loops showing a requester and waiting for input, calling the delay and key routines, returning the nonzero byte once the user confirms.
- `allocate_and_compact_pointer_array`: Allocates a pointer array with AllocMem, fills each slot with a further allocation, frees failed slots, compacts the array and returns the count of valid entries.
- `format_progress_message_with_counts`: Builds a progress string from the current offset, total size and a caller argument, then prints it or writes it to the console depending on a flag.
- `compare_entry_against_buffer`: Checks a flag, then compares the pointer array entry selected by the word argument against the buffer at offset 60, returning one if equal or empty, zero otherwise.
- `copy_disk_with_verify_and_retry`: Copies source disk to destination using dos.Examine, Info, ParentDir and UnLock, prompting for source and destination names, verifying tracks and retrying on errors before returning.
- `map_error_code_to_task_slot`: Scans six bytes of a table for the given error code, finds the matching slot value, stores it at offset 148 of the current task via FindTask, and returns the code.
- `read_single_byte_from_disk_file`: Calls dos.Read to read one byte from the file handle into a stack buffer, returning the sign-extended byte or minus one on failure.
- `init_io_request_fields_and_compute_offset`: Stores the word, longwords and zero bytes into the IO request structure at offsets 28, 36, 40, 30 and 31, calls a helper to compute a value, adds the fourth argument and stores the sum at offset 44, returning zero.
- `check_do_wait_io_then_set_write_status`: Calls CheckIO on the request, and if it is not done calls DoIO, then clears the length at offset 36, sets the command word at offset 28 to 9 and calls WaitIO, returning zero.
- `send_two_commands_then_finish_io`: Calls CheckIO and DoIO on the request, sets the command word at offset 28 to 4 and calls WaitIO, then sets it to 5 and calls WaitIO again before tail-calling the finish routine.
- `read_disk_block_and_update_checksum`: Opens a disk device, reads a block into a buffer, verifies the DOS magic, copies a volume name, sums 128 longs and subtracts the total from the block count, then closes the device.
- `read_single_disk_block_via_doio`: Opens the disk device, builds a DoIO packet for one 512-byte block at the given offset, waits for completion and returns true when the read succeeded.
- `write_disk_block_with_retry_state`: Checks the disk state, resolves the target block, sends a write IO packet with the buffer and length, waits for it, increments the retry counter and returns one.
- `find_free_node_by_key_or_name`: Walks a linked list of four-byte-scaled nodes from the library base, checking each free node against a key byte or recursing on a name string, and returns the matching node pointer or zero.
- `copy_bytes_count_from_a3_to_a2`: Copies a count of bytes from the address in a3 to the address in a2, incrementing both pointers and d6 until d6 reaches d7, then restores registers and returns.
- `draw_text_clipped_to_screen_width`: Measures the string with TextLength, clamps the pen position between 20 and the screen width minus 78, then sets the pen, moves and draws the text via graphics library calls.
- `normalize_lowercase_char`: Checks if a byte is between 97 and 122 inclusive; if so, subtracts 32 from its sign-extended value, otherwise it returns the sign-extended byte.
- `return_sign_extended_byte_argument`: Loads the byte argument at offset eleven from a5, sign-extends it to a word and long in d0, releases the a5 frame and returns.
- `strip_trailing_colon_from_string`: Scans the string pointed to by the stack argument to its terminating zero, and if the last character is a colon, replaces it with a zero byte.
- `push_pointer_and_branch`: Pushes the value in a3 onto the stack, loads a new pointer into a3 from an offset of 8 bytes from the current stack pointer, and branches to another routine.
- `scan_string_for_colon_terminate`: Advances a3 through the string until a colon or zero byte, inserts a colon if the end was reached, then clears the byte after the colon and returns.
- `allocate_and_post_message_port`: Allocates 68 bytes of memory, initializes a message port, finds the current task, adds the port to its list, waits on it, removes it, frees the memory and returns the message value.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `write_buffered_character`: Writes a character to a stream buffer, handling newline expansion, flushing when the buffer fills, and returning the character or minus one on error.
- `perform_register_manipulations`: Moves a word from the address in a2 to d6, performs two bitwise OR operations on d0, and sets d0 to 97.
- `uppercase_string_in_place`: Walks the null-terminated string at a3, and for each byte whose bit 1 is set in the table at offset 941 of a4, subtracts 32, then returns the original pointer.
- `calculate_string_length_4`: The routine receives a pointer to a zero-terminated byte string and advances a pointer until it finds the terminating zero byte.
- `compare_case_insensitive_strings`: The routine compares two byte strings character by character, converting uppercase letters to lowercase for comparison, and returns the difference between the first mismatching characters or zero if equal.
- `reverse_engineer`: Repeatedly calls another routine, stores a byte from a register through a pointer, and then reverses a 12-byte buffer in place before terminating it with a null byte.
- `convert_number_to_ascii`: The routine converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `reverse_nibbles_from_table`: The routine expands a packed nibble value using a global lookup table and writes the resulting bytes in reverse order to a destination.
- `parse_signed_decimal_2`: Parses a signed decimal integer from a string, stores the result in a memory location pointed to by a parameter, and returns the number of characters processed.
- `append_byte_to_output_buffer`: Increments the byte count at offset 2216 in a4, stores the low byte of the argument at the pointer held at offset 2212, advances that pointer, and returns.
- `format_string_into_buffer_terminate`: Resets the output count and pointer in a4, calls the formatting routine with the format and argument, writes a zero terminator at the pointer, and returns the count.
- `update_linked_list_node_8`: Increments a value at offset 8744 of a structure in a4, decrements a value at offset 8730 of the same structure, and if the decremented value is not less than 1, updates a linked list by storing a byte from d7 into the current node and advancing the pointer at offset 866 in a4.
- `call_with_byte_arg_and_a4_string`: Moves a 32-bit value from d7 into d0, extracts the low byte into d1, pushes a string pointer relative to a4 and the byte into the stack, calls write_buffered_character, and returns the result in d1 while restoring d7.
- `clear_counter_call_callback_and_error`: Clears the a4-relative counter, calls a callback with the argument and a pc-relative function, reports an a4-relative error string with minus one, then returns the counter value in d0.
- `lookup_then_apply_three_args`: Saves d4-d7 and a3, looks up an entry from the first argument, returns minus one if absent, otherwise calls a routine with the entry field and two arguments, and fails if the a4 flag is set.
- `allocate_aligned_memory_block`: Rounds the requested size up to a multiple of four with a minimum of eight, walks the free-list at offset 820 of a4 for a fitting block, splits it if larger, and returns the block pointer or zero.
- `flush_pending_write_buffers`: Walks a linked list of buffer entries starting at offset 828 of a4, and for each entry where bit 1 is set and bit 2 is clear, and the difference between the value at offset 4 and offset 16 is non-zero, calls a routine with the value at offset 28, the value at offset 16, and the calculated difference, then calls another routine with the value at offset 16.
- `write_buffer_via_dos_write`: Clears the error field, calls dos.Write with the file handle, buffer and length from the stack, and on failure stores the IoErr result and sets error code five.
- `seek_disk_offset_with_retry`: Saves registers, calls a helper if a flag is set, clears the error field, then calls the DOS Seek vector with the file, offset and mode arguments, storing IoErr and error code 22 on failure and returning the resulting position.
- `read_disk_bytes_into_buffer`: Saves registers, optionally calls a helper, clears the error field, then calls the DOS Read vector with the file handle, buffer and length, storing IoErr and error code 5 on failure and returning the byte count.
- `cleanup_display_context`: It conditionally invokes a callback when a structure field is nonzero, then writes reset values at two fixed offsets.
- `close_dos_file_if_open`: Saves d7, loads the file handle from the stack, checks the open-file flag at offset 1260 in a4, optionally calls a helper, then calls dos.Close through the DOS library base and returns zero.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `normalize_fixed_point_divide_2`: Ensures the dividend and divisor are positive by negating them if necessary, then calls another routine to perform a fixed-point division.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `initialize_and_link_object`: Allocates a structure and populates its fields with provided arguments, a value from a function call, and a system-derived value.
- `lookup_handle_table_entry`: Clears the error field, validates the index in d7 against the table size, and returns a pointer to the eight-byte entry or zero with error code nine.
- `invoke_auto_request_callback`: The routine calls a function pointer located within a structure pointed to by an offset from a global variable.
- `free_mem_list_and_clear_heads`: Walks the linked list at the a4-relative pointer, freeing each node with exec FreeMem, then clears both list head pointers and returns zero.
- `alloc_mem_node_and_append_to_list`: Allocates a block sized from the argument plus twelve bytes with exec AllocMem, fills its size and links it into the a4-relative list, updating head and tail pointers, and returns the data area.
- `clear_message_buffer`: Writes a byte and two 32-bit values to a structure pointed to by its argument and then calls another routine via a global variable table.
- `set_message_type_and_target`: If a non-zero parameter is provided, it calls another routine via a global table and populates the returned structure with a message type, a zero byte, the original parameter, and a 16-bit value.
- `lookup_and_process_file_entry`: Looks up the first argument, returns -1 if not found, optionally calls a routine when bit 3 of byte 3 is set, then processes the entry's pointer with two arguments and checks a global flag.
- `parse_formatted_numeric_string`: Parses a string containing numeric values with various format specifiers including signs, decimal points, and base indicators.
- `process_format_string`: Iterates through a string and calls another routine to parse numeric values when a percent sign is encountered.
- `allocate_stream_buffer_if_needed`: Checks the stream's buffer pointer and flag byte, allocates a buffer via the memory routine when absent, stores it in the stream and clears the error and position fields.
- `flush_dirty_blocks_before_free`: Walks the block table backwards from the count at offset 804, and for each non-empty, non-busy entry calls the flush routine with its data pointer, then calls the final routine with the saved argument.
- `allocate_and_insert_memory_block`: Rounds the requested size up to a multiple of four with a minimum of eight, then walks the free list at offset 820 to insert or merge the new block, adjusting the total at offset 824 and returning zero or minus one.
- `find_block_then_write_data`: Looks up a block by the first argument, returns minus one if absent, otherwise calls a routine with the block's data area and two arguments, and returns minus one if the error flag at offset 1236 is set.
- `copy_memory_block`: Copies a block of memory from a source address to a destination address.
- `check_signals_and_run_pending_callback`: Reads the task signal mask, and if the watched bits are set and a stored callback exists, calls it, clears the pointer and invokes the cleanup routine with argument 20.

### FastMemFirst

- `allocate_fast_memory_and_parse_args`: Allocates a block of fast memory and stores the requested size, a pointer to the block, and the address of the caller's stack frame into it, then calls a routine that parses the command line from dos.library into a linked list of null-terminated argument strings, handling quoted text and asterisk conversions.
- `find_and_copy_fastmem_block`: Walks a linked list of memory blocks starting from a global variable, storing the first block whose type tag at offset 14 equals 5 and whose value is between an address and an address in d2, and the next block after the first whose type tag equals 3 and whose value is below an address in d3, then returns.
- `call_library_vector_132_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_138_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.

### Disk-Validator

- `update_display_string_and_bitfield`: Formats a numeric value into a character string and stores it in a buffer, then performs bitwise operations to update a bitfield in a structure using values from a table and several calls to other routines.

### Newcon-Handler

- `initialize_structure_from_hunk_table`: Initializes a structure by setting several fields to zero, calculating and storing pointer offsets, and copying data from a table based on an index provided in a register.
- `update_structure_fields`: Performs a series of calls to external routines using pointers from a table, then updates multiple fields within a structure pointed to by a1, including clearing values, setting a 16-bit constant, and performing arithmetic on various offsets.

### Shell-Seg

- `initialize_and_update_entity_data`: Initializes several offsets in a structure with calculated pointer values, calls several subroutines to retrieve data, and then updates a table of structures by copying values from the retrieved data based on an index.

### Ram-Handler

- `execute_indirect_jump`: Sets a value in d0, calls a routine using a pointer from a6, and then performs an indirect jump using a pointer from a6.

### serial.device

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `update_device_state`: Updates several fields in a structure, including incrementing and decrementing a counter, performing a library call, and storing a value into multiple offsets.
- `check_status_flag`: Checks a specific bitmask against a byte in a structure and calls another routine if the condition is met.
- `write_status_byte`: Stores the byte value 253 at offset 31 of the structure in a1 and branches to another routine.
- `toggle_interrupt_enable`: Sets and then clears a bit in a byte at an offset from a pointer while updating a global variable and modifying the interrupt enable bit in the interrupt enable register.
- `check_device_event_type`: Clears a byte at offset 31 of a1, then compares a word at offset 28 of a1 against the values 2, 3, or 10, branching to different locations based on the result.
- `compare_and_clear_pointer`: Compares a longword at offset 74 of a6 with a1, and if they are not equal, clears bit 1 of a byte at offset 440 of a6 and clears the longword at offset 74 of a6 before branching.
- `copy_structure_via_pointers`: Checks bit 6 of a byte at offset 30 of the structure in a1, and if set, copies the structure's first word through a pointer stored in offset 4 of the structure and decrements a word at offset 70 of a6.
- `handle_device_event`: Compares a1 to a value in a6, and if they match, checks if a word at offset 40 of a1 is 10 and if bit 5 of a byte at offset 294 of a6 is set, then writes to a hardware register, calls another routine, and clears two bytes in a6.
- `decrement_counter_and_clear_flag`: The routine checks a flag byte in the structure pointed to by a1 and, if set, copies the structure’s first word through a pointer stored in the structure, updates a pointer stored at an offset from that word, decrements a short counter in a6, modifies two flag bytes in the structure and clears a byte.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `check_word_and_return`: Clears d0, checks if the word at offset 32 of a6 is zero, and otherwise returns.
- `execute_multiple_calls`: Performs a sequence of library calls using pointers retrieved from a structure in a6, updates a linked list using a pointer in a6, and returns a 32-bit value from a structure in a6.
- `register_serial_device_2`: Clears a longword at offset 50 of a1, and if offset 54 of a1 is not zero, clears bit 0 of a byte at offset 48 of a1, sets bit 7 of the INTENA register, and writes 1 to the INTENA register.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `register_serial_device`: The routine links a serial device structure into a global device list and updates a device counter.
- `update_serial_device_status`: Checks various status flags and linked list pointers within a device structure, updates a status byte at offset 47 of a structure in a1, and calls update_serial_status_and_dispatch if specific hardware interrupt conditions are met.
- `update_serial_status_and_dispatch_2`: Decrements a word at an offset from a pointer and branches if it is not zero; otherwise it writes a byte to a structure and branches to another routine.
- `shift_and_call_handler`: Clears d0, loads a word from offset 462 of a6 into d0, shifts d0 left by 6 bits, and calls another routine before branching to another location.
- `update_serial_status_3`: Writes 1500 to an offset of 476 in a6, loads a pointer from an offset of 86 in a6, clears d0, loads a byte from the pointer into d0, tests bit 4 of an offset of 472 in a6, and if set, decrements a longword at an offset of 94 in a6.
- `update_serial_status_and_dispatch_3`: Compares a longword against a value in a structure, sets a bit in the interrupt request register, and performs several bit tests and branches based on the status of various flags in a structure.
- `update_serial_status_and_dispatch`: Calls another routine, updates a status word in the SERDAT register using a bit from a structure, increments two counters in a6, and if a bit is set in a1, performs a loop to compare bytes before calling a library function.
- `process_device_list`: If the shortword at offset 114 of a6 is not zero, it manipulates a linked list by updating pointers and clearing a longword at offset 154 of a6, then sets bits in a byte at offset 30 of a1 and calls another routine; otherwise, it sets a byte to 1 and returns.
- `enable_interrupts`: Sets bit 0 of the INTENA register.
- `check_serial_interrupt_status`: Reads a byte from a global variable and combines it with a value from a hardware register to update a global status flag.
- `validate_and_update_device_2`: Sets the interrupt enable register, calls two routines if specific pointers in a structure are non-zero, clears a byte in a structure, populates a new structure with several longwords, and calls two more routines before updating the interrupt enable register and branching.
- `call_handler_and_branch`: Calls another routine and then branches to another routine.
- `copy_serial_state`: The routine copies a 32-bit value from an offset within a structure pointed to by a register to two other offsets in the same structure, then clears a 32-bit field at a third offset.
- `check_status_and_interrupts`: Loads a longword from offset 4e of the structure in a6 and, if non-zero, sets bit 2 of offset 1d8 of the structure in a6; then loads a longword from offset 4a of the structure in a6 and, if non-zero, calls another routine.
- `clear_status_and_set_interrupts`: Clears bit 2 of a byte at offset 440 of a structure in a6, then if a longword at offset 74 of a6 is non-zero, calls another routine, writes the value 32769 to two hardware registers, and branches to another location.
- `toggle_interrupt_mask`: Writes 2049 to the hardware register at offset 0, calls another routine, writes 34817 to the hardware register at offset 0, and branches to another routine.
- `clear_serial_flag`: The routine calls another routine twice and then clears a 32-bit field in a structure pointed to by an address register.
- `update_linked_list_node_2`: Copies a longword from a pointer in a1 to the location in a0, then calls another routine if the copied value is not zero.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `set_serial_flags_and_maybe_call`: The routine sets a flag byte in a structure pointed to by a1 and writes a constant to the next byte.
- `update_serial_status`: Reads a byte from a hardware register, checks two bits in a6, and if both are set, modifies the byte and copies several values from a6 and a1 into a1, including a longword from offset 88 of a6 to offset 50 of a1 and a byte from offset 470 of a6 to offset 128 of a1.
- `check_device_conditions`: Clears bit 0 of a byte at offset 30 of a1, and then checks if bit 3 of a byte at offset 79 of a1 is set, followed by checking two words at offsets 72 and 78 of a6.
- `set_flag_and_call_routine`: The routine sets a bit in a byte located via a pointer, calls another routine, stores a value returned in A1 into a word at an offset from the same pointer, and writes a constant to a global variable.
- `clear_serial_flag_and_disable_interrupts`: The routine clears a flag in a byte field of a structure pointed to by a1, and if a condition on that byte and a short field is met it sets another flag in the same byte, clears a pointer field, writes a value to a global variable, and calls another routine via a pointer stored in the structure.
- `configure_device_parameters`: Saves a1 and a6, writes 1 to the hardware register at offset 0, and if the byte at offset 8 of a1 is not 5, clears a longword at offset 354 of a6, checks if d0 is less than 600, sets d0 to 600 if it is, and calls another routine via a pointer in a6; otherwise, it returns.
- `restore_registers_and_return_7`: Restores several longwords from the stack into registers a1 and a6, then returns.
- `toggle_serial_interrupt_state`: Tests bit 7 of a byte at offset 79 of a structure in a1; if set, it sets bit 7 of offset 69 of the structure in a6, clears bit 2 of offset 472 of the structure in a6, and if bit 3 of offset 472 is set, it clears bit 7 of offset 79 of the structure in a1, calls another routine, and sets bit 7 of offset 79 of the structure in a1; otherwise it sets bit 7 of offset 69 of the structure in a6 and returns.
- `check_serial_interrupt_conditions`: Clears bit 7 of an offset of 69 in a6, tests bit 1 of an offset of 472 in a6, tests a longword at an offset of 78 in a6, tests bit 4 of an offset of 30 in a1, and if bit 4 is set, writes 1 to an offset of 31 in a1.
- `update_interrupt_registers`: Writes 2049 to a hardware register, calls another routine, then writes 35329 to the same hardware register before branching to another location.
- `validate_and_update_device`: Checks several byte and longword constraints in a1, copies various fields from a1 to a structure in a6, calls two different routines, and if a comparison between a value in a6 and a value in a1 succeeds, sets byte 31 of a1 to 4.
- `compute_serial_status_word`: The routine builds a 16-bit status word from a byte in a structure and a flag in another structure, then stores the word into a global variable.
- `decrement_indexed_table_lookup`: The routine checks a flag byte in a structure pointed to by a register and conditionally decrements a short index.
- `write_byte_and_branch`: Stores the byte value 5 at offset 31 of a1 and branches to another routine.
- `compute_serial_divisor_and_flags_2`: Calculates a divisor by dividing a constant by a value derived from a structure field, stores the result in a structure field and a global variable, and modifies a byte at offset 220 of a structure in a6.
- `compute_serial_divisor_and_flags`: The routine divides an input value by the low word of another input and stores the result in a structure field, copies that field to a global variable, clears a flag bit in a byte field, and conditionally sets a high bit in the global variable and a flag bit in the byte field based on two structure bytes.
- `configure_serial_device_parameters`: Calls a routine to obtain a pointer, then if successful, populates a structure at that pointer with specific values and calls another routine using a pointer from that structure, finally attempting to call a third routine twice with different arguments.
- `configure_serial_requests`: Calls a routine twice using two different addresses in a3 and two different offsets in a6, then branches to another routine.
- `store_a0_to_offset_16`: Stores the contents of a0 into a memory location at offset 16 of the structure pointed to by a1.
- `set_serial_device_io_request`: The routine writes a constant byte to an offset in a structure pointed to by a0, stores a pointer to a global variable at another offset, and copies two caller-provided values into two further offsets.
- `set_serial_device_request`: The routine fills a request block in memory with a pointer to a global variable and several byte fields, then calls a function through a pointer obtained from a table.
- `configure_serial_device_requests`: Sets up two serial device request blocks by updating status bytes and pointers in a structure, calls a library function via a pointer, and enables interrupts by writing to the hardware register.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `clear_d0_and_branch`: Clears d0 and branches to another location.
- `insert_node_into_linked_list`: Checks status flags and a counter in a structure, then performs a linked list insertion by swapping pointers between a new node and an existing list, increments a counter in a structure, and calls another routine.
- `rotate_serial_buffer_node`: Checks if a serial device structure has a pending buffer; if so, it decrements a counter, updates a pointer to the next node in a linked list, clears a status field, and modifies specific bits in a status byte at offset 30 of the new node before calling process_serial_data_buffer.
- `process_serial_data_buffer`: Checks a status bit and a buffer length in a structure in a6, then decrements a counter and advances a pointer within a data buffer.
- `test_status_bit`: Tests bit 0 of a byte at offset 445 of a structure in a6 and branches to another location if the bit is not set, otherwise it branches to another location.
- `update_serial_status_2`: Calls another routine, and if the result is zero, sets byte 1f of a1 to 9 before branching to another location.
- `update_serial_device_buffer`: Reads a byte from a specific offset in a structure, clears a bit in a register, and writes the result to a memory location pointed to by another offset in the same structure.
- `restore_registers_and_return_8`: Restores d2 and a6 from the stack and returns.
- `identify_routine`: This routine is not empty; it contains ten instructions but the decompiler has not produced meaningful pseudo-code.
- `modify_serial_byte_2`: Clears bit 2 of an offset of 470 in a6, and if the bit was not cleared, writes 15 to an offset of 31 in a1.
- `clear_and_set_byte`: Clears bits 0 through 2 of a byte at offset 470 of a structure in a6 and branches to another location if the bit is not set, otherwise it sets a byte at offset 31 of a structure in a1 and branches to another location.
- `modify_serial_byte`: Sets byte 1f of a1 to 12, and if bit 0 of offset 1d6 of the structure in a6 is set, sets byte 1f of a1 to 6, clears a longword at offset 3e of a6, and clears bit 1 of offset 1d6 of a6.
- `set_status_and_branch`: Writes a word to an offset of 156 of a structure in a0, sets bit 0 of a byte at offset 470 of a structure in a6, and branches to another routine.
- `process_serial_status_and_buffer`: Checks a 16-bit value in a structure, updates a status word, and if the value is non-negative, modifies various flags and buffer pointers in a structure in a6.
- `update_serial_interrupt`: Compares a byte at offset 34 of a structure in a6 with d1, and if equal, tests bit 1 of a byte at offset 288 of a6; if bit 1 is set, it clears bit 1 of that same byte and writes the word 32769 to the hardware register at offset 522 of the system.
- `check_serial_status`: Tests bit 5 of a byte at offset 296 of a structure in a6, then tests bit 1 of a byte at offset 285 of a6, calls another routine, tests bit 3 of d0, and branches based on these results.
- `test_bit_and_branch_2`: Tests bit 3 of d0 and branches if it is set, otherwise it branches to another routine.
- `test_serial_status`: Tests bit 0 of a byte at offset 443 of the structure in a6 and branches to another routine based on the result.
- `set_and_copy_longword`: Sets bit 7 of a byte at offset 470 of a structure in a6 and, if a longword at offset 62 of a6 is zero, copies a longword from offset 42 to offset 62 before branching to another location.
- `process_buffer_data`: Compares two longwords at offsets 84 and 88 of a structure in a6 and, if the first is not less than or equal to the second, performs a series of operations including moving a byte to a pointer in a0, incrementing a longword at offset 88, and potentially updating a pointer at offset 58 or 62 before calling another routine.
- `test_signed_word_and_interrupt`: Checks if a longword at offset 3e of a6 is non-zero; if it is zero, it copies a longword from offset 2a to offset 3e, calls another routine, and if offset 4a of a6 is non-zero, performs a jump using an offset from a6.
- `check_signed_word`: Tests if a 16-bit word at offset 70 of the structure in a6 is greater than zero; if not, it restores d2 and a6 from the stack and returns.
- `update_serial_port_status`: Moves a byte from offset 34 of the structure in a6 to offset 473 of the same structure, clears bit 3 of offset 472, and if bit 2 of offset 69 is set, clears bit 6 of the CIA A port register; otherwise it returns.
- `update_hardware_interrupts`: Moves a byte from offset 35 of the structure in a6 to offset 441 of the same structure, sets bit 3 of offset 440, and conditionally sets bit 6 of a hardware register or writes a value to the interrupt enable register.
- `clear_byte_at_offset_473`: Clears a byte at offset 473 of the structure in a6 and returns.
- `set_device_flag_and_call_library`: Sets bit 6 of offset 470 of a6, and if bit 6 of offset 1d6 of a6 is not set, clears a longword at offset 32 of a1, performs a calculation on a longword from offset 106 of a6, and calls a library routine using a pointer from a1.
- `clear_bit_and_branch`: Saves d2 and a6 to the stack, loads a pointer from a1 into a6, clears bit 6 of a byte at offset 470 of the structure in a6, and if that bit is clear, it checks the lower 12 bits of a word from a hardware register and, if non-zero, clears bit 2 of a byte at offset 470 of the structure in a6 before branching to another routine; otherwise it branches to another routine.
- `restore_registers_and_return_9`: Restores the d2 and a6 registers from the stack and returns immediately.
- `clear_and_call_library`: Clears bit 6 of offset 470 in a6, pushes several values to the stack, calls a routine at an offset from a pointer in a6, and then restores a6 and a value from the stack before branching.

### usa1

- `clear_d0_and_decrement_stack`: Performs three bitwise OR operations on d0 with zero and subtracts zero from a 16-bit value at the stack pointer location.

### parallel.device

- `load_d0_with_ff`: Loads d0 with the value negative 255 and returns.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `write_byte_to_offset_31`: Writes the byte negative 3 to offset 31 of the structure in a1 and returns.
- `clear_request_if_not_equal`: Calls another routine, compares a long value in a1 with a long value at offset 38 of the structure in a6, and clears offset 38 if they are not equal.
- `remove_request_from_list_and_signal`: Unlinks a request from a list, decrements a counter in a structure, updates status bits in a structure, calls a library function, and calls another routine before returning.
- `open_parallel_device_unit`: Calls an open helper, then loads a longword from offset 26 of a6 into d0 and, if non-zero, loads d0 into a1 and calls another routine.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `close_parallel_unit_and_reply`: Calls a helper with d0 set to 16, clears bit 0 of the flags byte at offset 30 of the unit, then calls the reply function through the device base and returns.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `process_linked_list_node`: Uses a pointer from offset 118 of the structure in a6 to access a linked list, updates the list pointers, and calls another routine if a value is non-zero.
- `clear_word_at_offset`: Clears a 16-bit word at offset 114 of the structure in a6 and returns.
- `set_strobe_and_call_serial_abort`: Sets bit 5 at offset 30 of the structure in a1, writes 254 to offset 31, then calls the library vector at offset -378 through a6 and restores a1 and a6.
- `read_cia_lines_and_store_parallel_status`: Delays briefly, reads the low three bits of CIAB port A and the parallel port register, merges them into the status byte at offset 116 of a6, copies it to offset 52 of a1, restores the port bits and returns.
- `set_status_byte`: Tests bit 2 of offset 118 of the structure in a6 and, if set, writes the value 1 to offset 31 of the structure in a1.
- `copy_structure_fields`: Copies a byte from one structure to another and, if a specific bit is set, copies two long words as well.
- `initialize_parallel_device_handler`: Queries a device for a specific resource, and if successful, populates a structure with a pointer and a value, then attempts to open a device by calling a routine with a custom handler and a specific command code.
- `clear_d0_and_branch_2`: Clears the d0 register and branches to another routine.
- `set_d0_bits_and_clear_a1_offset_field`: ORs the immediate values into d0 and a byte at a4 minus one, clears the longword at offset 32 of a1, then returns.
- `insert_linked_list_node`: Swaps a5 and a6, clears bit 0 of offset 30 of a1, and if bit 2 of offset 118 of a5 is set, sets bit 6 of offset 30 of a1 and performs a linked list insertion by updating pointers in a1 and a5 before incrementing a word at offset 114 of a5.
- `swap_and_call`: Calls another routine and then swaps the contents of a5 and a6.
- `start_parallel_write_request`: Clears and sets flag bits in the request and unit structures, stores the buffer pointer and length, checks the write length sign, and writes a byte to the CIA parallel port register.
- `handle_parallel_request`: Tests a word at offset 276 of a structure in a5 and, if non-zero, performs a series of pointer manipulations and register updates, decrements a word at offset 276, and calls another routine before branching to another location.
- `transmit_parallel_byte`: Checks a status bit in a structure, then writes a byte from a buffer to a CIA register and updates a pointer and counter in the structure.
- `handle_cia_data`: Reads a byte from a CIA register, updates a buffer and a counter in a structure, and performs a loop to compare bytes if a specific bit is set in a structure in a1.
- `handle_cia_and_buffer`: Copies a longword from offset 46 of a5 to offset 32 of a1, clears bits 3 and 4 of a5 and a1 respectively, calls a routine using a pointer from a6, clears a word at offset 38 of a5, and if a word at offset 114 of a5 is non-zero, calls another routine using a pointer from a6.

### clipboard.device

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `recurse_on_pointer`: Saves a0 and a6 to the stack, calls a routine at an address stored in a pointer, increments the stack pointer by 8, and returns.
- `recursively_calls_itself`: Performs an infinite recursive call to itself, or otherwise acts as an empty wrapper that calls itself before returning.
- `call_handler_via_pointer`: Pushes the contents of a1 onto the stack, calls a routine at the address stored in a pointer, and then returns.
- `jump_to_address`: Performs an indirect jump to an address stored in a longword.
- `call_handler_via_stack`: Pushes the value of a1 onto the stack, calls another routine, and then increments the stack pointer by 4.
- `call_subroutine_with_stack_2`: Pushes the value of a1 onto the stack and calls a routine at an address stored in a pointer.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `execute_callback_sequence_3`: Saves registers to the stack, loads a longword from the stack into d2 and a pointer from the stack into a2, stores a2 in a global variable, calls another routine, and then branches to another location if d0 is not zero.
- `execute_callback_sequence_2`: Performs several indirect calls, including one using a pointer from a structure in a2 and another using a pointer from a global variable, then branches based on the result of a test on d0.
- `invoke_indirect_functions`: Executes a routine using a pointer or a library function offset via a global variable or a table.
- `increment_structure_field`: Calls two routines using values from the stack, then checks a field in a structure pointed to by a0; if that field is non-zero, it increments a longword at offset 14 of that structure and stores it in a new location.
- `clear_and_call_library_2`: Clears a longword at an offset from the pointer in a3, pushes the address of a value at offset 100 of the structure in a2 onto the stack, calls a library function, and then branches to another routine.
- `clear_memory_word`: Clears a word at offset 122 of the structure in a2 and calls another routine.
- `manage_clipboard_fonts`: Pushes registers to the stack, calls a routine to retrieve a pointer, and then enters a loop that performs multiple library calls to manipulate a structure in a3, including calls to extract font names and modify data based on a comparison of a word at an offset of a library base with 35.
- `extract_font_names`: Pushes values from a2 onto the stack, calls another routine twice, moves d0 to d4, and then restores registers from the stack.
- `traverse_list_and_load`: Saves registers to the stack, loads a pointer from the stack into a2, calls another routine, and then loads a pointer from offset 100 of the structure in a2 into a0 before branching to another location.
- `traverse_linked_list_2`: Traverses a linked list, calling a library function for each node, and finally clears a word at offset 114 of a structure in a2.
- `copy_linked_list_buffer_segment`: Traverses a linked list of memory blocks to copy a specific range of bytes from a source buffer to a destination.
- `compare_font_offset`: Loads a longword from offset 118 of a structure in a3 and compares it to a longword at offset 48 of the stack, then branches based on the result.
- `traverse_font_list`: Checks a value on the stack, and if non-zero, calculates a shift value from a longword at offset 48 of the stack, then compares a longword at offset 14 of a structure in a1 with d0 to determine whether to proceed with a comparison.
- `compare_structure_longword`: Updates a1 to point to an offset 4 of the structure in a1, compares a longword at offset 14 of that structure to d1, and branches based on the comparison result.
- `compare_and_branch_11`: Loads a pointer from a1 into a1, loads a longword from d1 into d0, and compares the longword at offset 14 of the structure in a1 to d0, branching if the value in a1 is higher.
- `compare_and_branch_d3`: Compares the value in d3 against a constant, and if d3 is not less than or equal to the result of subtracting d3 from 512, it moves d3 to d2 and branches.
- `copy_byte_loop`: Copies 512 bytes from a table using d2 as an index and a1 as a base pointer to a destination pointed to by a2, then performs arithmetic on d0, d3, and d4 before branching to another routine.
- `move_and_branch_5`: Moves a word from d3 to d2 and branches to another routine.
- `copy_bytes_with_limit`: Enters a loop that copies bytes from a structure in a1 to a destination pointed to by a2 using an index in d2, and if a comparison involving d3 and d4 fails, it calls another routine before returning.
- `manage_clipboard_buffer_contents`: Checks if a clipboard buffer is empty and allocates memory if necessary, then traverses a linked list of data blocks to copy bytes from a source pointer into the appropriate memory segments.
- `validate_and_write_buffer_index`: Compares a value from a structure in a2 with a calculated offset derived from a structure in d3, then calls another routine to write a byte to a specific location if the bounds check passes.
- `validate_and_write_buffer_index_2`: Compares a value from a structure in a2 with a calculated offset derived from a structure in d3, then calls another routine to write a byte.
- `update_clipboard_buffer`: Clears a 16-bit value at offset 122 of a structure and, if non-zero, checks a pointer at offset 68 to traverse a linked list.
- `traverse_and_compare_linked_list`: Retrieves a value from the stack, shifts it right by 9, and masks it with 255.
- `verify_structure_member`: Moves a pointer from d3 to a0, loads a longword from offset 4 of the structure in a0 into d3, then compares the longword at offset 14 of the new structure in a0 with the value in d0, branching if the comparison results in a carry.
- `follow_structure_chain`: Follows a chain of pointers starting from a structure in d3, and if a pointer is null, calls another routine to retrieve it; it then enters a loop that copies bytes from a source to a destination while comparing a value at offset 14 of a structure in a0 with d0.
- `calculate_offset_and_branch_2`: Sets d0 to 512, subtracts the value in d4 from d0, compares d0 to a longword at offset 48 of the stack, and if d0 is below that value, branches to another routine; otherwise it moves d4 into d2 and branches to another routine.
- `copy_bytes_to_structure`: Copies 320 bytes from a source buffer to a destination structure using a loop, then checks a pointer in a structure and calls another routine if the pointer is null.
- `move_and_branch_6`: Moves a word from d4 to d2 and branches to another location.
- `copy_bytes_with_limit_2`: Copies a byte from a source at an offset calculated from a1 and d0 to a destination at offset 18 of a1, increments d2, and loops until a comparison with a value at offset 48 of the stack is met, then calls another routine.
- `check_byte_bits`: Pushes a pointer from a2 to the stack, calls a routine, and then checks bit 7 and bit 0 of a byte at offset 30 of the structure pointed to by a2, branching or calling another routine based on the results.
- `modify_byte_bits`: Clears the least significant bit and sets the most significant bit of a byte at offset 30 in a buffer and calls another routine.
- `modify_byte_bits_2`: Checks bit 7 of a byte at offset 30 of a structure; if not set, it calls another routine, then clears bit 1 of that byte and calls a third routine, finally setting bit 4 of that byte.
- `compare_structure_values_2`: Loads pointers from a structure in a0, checks if they are non-zero, and compares a longword at offset 48 of the first structure with a longword at offset 48 of the second structure, branching if the first is greater than or equal to the second.
- `clear_and_branch`: Clears a word in d0 and branches to another location.
- `branch_to_another_routine`: Performs a short branch to another routine.
- `tiny_moveq_ext_rts`: Small routine of 3 instructions that loads 1 into d0, then sign-extends d0, then returns.
- `copy_structure_fields_2`: Copies a longword from offset 40 of a structure in a1 to offset 180 of a structure in a0, a word from offset 16 of a0 to offset 174 of a0, and a longword from offset 48 of a1 to offset 176 of a0, then pushes d0 and a1 onto the stack and calls another routine.
- `update_clipboard_status_bit`: Sets bit 5 of a byte at offset 30 of a structure pointed to by a3, then checks a byte at offset 66 of a structure pointed to by a2; if the byte is zero, it clears bit 5 of the byte at offset 30 of the structure in a3 and otherwise returns.
- `update_clipboard_segment_status_2`: Sets a status byte at offset 66 of the structure in a2 to 1 and checks a word at offset 64.
- `process_clipboard_segment_status`: Updates a structure's status field based on various segment types, including handling segment transitions, adjusting buffer lengths, and updating counters within a linked list or segment structure.
- `process_clipboard_segment_types`: Copies a longword and a word from a segment structure into a target structure, then checks a status field to determine if it must call handle_clipboard_segment_type_nine, push_and_call_segment_handler, or call_library_vector_72_4.
- `push_and_call_segment_handler`: Pushes the pointer in a2 onto the stack, calls another routine, increments the stack pointer by 4, writes 1 to a word at offset 122 of the structure pointed to by a2, and branches to another routine.
- `handle_clipboard_segment_type_nine`: Checks if a status field at offset 100 of a structure is 9, then calls another routine to process the segment and updates the status field to 3 if a specific pointer is non-zero.
- `reallocate_clipboard_buffer_segment_2`: Checks if a structure's status is two and its linked list is not empty, then increments a counter at offset 138 and clears a bit in a block.
- `update_clipboard_segment_status`: Copies a longword and a word from a segment structure into a management structure, then calls another routine if the segment type is 9.
- `reallocate_clipboard_buffer_segment`: Checks if a structure's status is two and its linked list is not empty, then increments a counter at offset 138 and clears a bit in a block status byte.
- `compare_and_update_buffer_pointer`: Adds a longword from offset 44 to a longword from offset 44 of a structure in a4, compares the result to a value in a2, and if the result is not lower, stores the original value from offset 36 into offset 32 of the structure in a4.
- `update_clipboard_buffer_2`: Subtracts a longword at offset 44 from a longword at offset 44 of a structure in a4, stores the result in offset 32 of that structure, and adds the result back to offset 44 before branching to another routine.
- `clear_clipboard_status`: Clears a longword at offset 48 of a structure in a4, sets a longword at offset 48 to negative one, decrements a word at offset 138 of a structure in a2, calls another routine, and if the word at offset 138 is not zero, clears a word at offset 64 of the structure in a2.
- `expand_clipboard_buffer`: Checks if a structure's status is three and its linked list is not empty, then if the first data block's type is three, it calls another routine to allocate memory, updates the block's size and pointer, and adjusts the total buffer size in the parent structure.
- `copy_long_word_and_call`: Copies a long word from an offset of 44 in a structure pointed to by a4 to an offset of 118 in a structure pointed to by a2, pushes a4 and a3 onto the stack, calls another routine, and then branches to another location.
- `set_word_on_condition`: Compares a 16-bit value at offset 28 of a structure in a4 to 4; if it is not equal, it returns, otherwise it writes 4 to offset 64 of a structure in a2 and branches to another location.
- `increment_buffer_size_on_condition`: Checks if a 16-bit value at offset 64 of a structure is equal to 4.
- `clear_structure_status_bit`: Clears a longword and a word at specific offsets within a structure pointed to by a2, calls another routine, and then checks bit 5 of a byte at offset 46 of a structure in a3; if set, it clears bit 1 of that byte before returning.
- `load_pointer_from_stack`: Pushes a pointer from a2 to the stack, loads a pointer from an offset of the stack into a1, loads a pointer from another offset into a0, loads the value at a1 into a2, and loads the value at a2 into d1 before branching.
- `compare_and_branch`: Compares a longword at offset 48 of a2 with a longword at offset 48 of a0, then either moves the value in d1 to a2 and branches or branches if the first value is not greater than the second.
- `call_library_with_stack_cleanup`: If d1 is non-zero, it pushes several pointers onto the stack, calls a routine, cleans up the stack, and returns a pointer from the stack in a2.
- `set_byte_value`: Writes the constant byte value 253 to offset 31 of a structure and calls another routine.
- `set_structure_member`: Stores a constant byte value into a memory location relative to a pointer and calls another routine.
- `set_structure_member_253`: Pushes a pointer from the stack onto the stack and calls another routine to write the constant byte value 253 to offset 31 of the structure pointed to by that value.
- `set_structure_member_via_stack`: Fetches a pointer from the stack, pushes it onto the stack, and calls another routine to write the constant value 253 to offset 31 of the structure pointed to by that value.
- `set_structure_member_253_2`: Writes the constant byte value 253 to offset 31 of a structure pointed to by a value on the stack and calls another routine.
- `update_structure_byte_sequence`: Calls three separate routines to write the constant byte value 253 to offset 31 of a structure pointed to by a value retrieved from the stack.
- `update_list_node`: Checks if a long value at offset 48 of a structure is zero, and if so, copies a long value from offset 56 of another structure to offset 48, sets bit 4 of a byte at offset 30, clears a long at offset 32, sets a byte at offset 31 to 1, and calls another routine if a comparison between a long at offset 60 and the value at offset 48 succeeds.
- `traverse_list_offset`: Pushes a pointer from a2 to the stack, pushes a pointer to an offset of 124 from a3 to the stack, calls a routine, and restores a2 and a3 from the stack.
- `increment_and_update_list`: Increments a long value at offset 56 of a structure, copies it to offset 48 of another structure if the value at offset 48 is zero, sets bit 4 of a byte at offset 30, clears a long at offset 32, sets a byte at offset 31 to 1, and calls another routine if a comparison between a long at offset 60 and the value at offset 48 succeeds.
- `traverse_list_offset_large`: Pushes a pointer from a2 to the stack, pushes a pointer to an offset of 140 from a3 to the stack, calls a routine, and restores a2 and a3 from the stack.
- `copy_structure_field`: Copies a longword from an offset of 56 relative to a pointer to offset 48 of a structure and calls another routine.
- `copy_structure_field_offset`: Copies a longword from an offset of 60 relative to a pointer to offset 48 of a structure and calls another routine.
- `update_clipboard_status_and_fetch_pointer`: Modifies a global variable by applying a bitwise AND mask with a constant, then retrieves a pointer from a table to load it into a register before branching.
- `compare_and_branch_pointer`: Compares a longword at offset 14 of a3 with d2, then either moves d0 to a3 and branches or branches if the value is less than or equal to d2.
- `allocate_clipboard_buffer_structure`: Compares a longword at offset 14 of a structure in a3 with d2; if they match, it attempts to allocate memory using a constant and a pointer, then initializes several fields within the newly allocated structure including a global variable, a status flag, and several counters.
- `update_clipboard_buffer_and_fonts`: Initializes a structure by setting several fields and calling other routines, then performs a series of library calls to manage memory and data buffers.
- `call_and_branch_on_result`: Pushes a pointer from a3 to the stack, calls another routine, moves the result to a0 and then d3, and branches based on the result.
- `process_buffer_and_switch`: Pushes two addresses onto the stack, calls another routine, compares d0 to 512, clears a word at offset 122 of a3, and calls another routine if certain conditions are met.
- `process_linked_list_data_2`: Saves registers to the stack, calls another routine, and if the result does not match a value from a global variable, clears a word at offset 122 of a structure in a3.
- `recursively_process_data`: Calculates a bit-shifted value from a byte at offset 9 of a0, calls another routine with a pointer and a count, then calls a second routine with the address in a3 plus 22, and repeats the process if the result in a0 is not zero.
- `increment_and_recurse_pointer`: Increments a value at a memory offset and recursively calls other routines using pointers or library vectors based on a conditional check.
- `update_clipboard_structure`: Decrements a word at offset 18 of a structure pointed to by a2, calls a routine via a pointer at offset 24 of a2, and if certain conditions regarding offset 122, offset 68, and offset 60 of a3 are met, calls another routine with a value from a stack and updates offset 68 of a3 with the result.
- `process_buffer_chunks`: Checks if a pointer in a4 is non-zero and if d2 is greater than 512; if so, it calls a routine with a pointer from a3, and if the result in d0 matches d2, it clears a word at offset 122 of a3 and subtracts 512 from d2.
- `store_and_call_routine`: Pushes d2 onto the stack, calls a routine using a pointer at offset 68 of a3, and if d0 equals d2, clears a word at offset 122 of a3 and updates a4 before branching.
- `process_buffer_chunks_2`: Clears a word at offset 122 of a3, checks d2, and if d2 is not greater than 0, checks a pointer at offset 68 of a3, calls another routine, and calls a library function if a value at offset 118 of a3 is non-zero.
- `clear_buffer_and_decrement`: Calls three different routines, clears two longwords at offsets 20 and 24 of a structure in a2, decrements a global word, and if that word is zero and bit 3 of a global byte is set, calls another routine before returning.
- `check_and_call_library`: Checks a word at offset 68 of a2, and if it is non-zero, calls a routine using a pointer at offset 274 of a global variable, then calls another routine using a pointer at offset 36 of a2, and finally moves a longword from offset 278 of a global variable into d0.
- `set_byte_bit`: The routine sets the third bit of a byte at an offset from a register.
- `patch_and_call_handler`: Loads a pointer from the stack into a2, calls another routine, writes the byte value 5 to offset 8 of the pointer in a2, performs bitwise operations on offset 30 of a2, compares a word at offset 28 of a2 to 12, and if the comparison is not below that value, calls another routine.
- `call_table_index`: Calculates an offset from a value in a structure and calls a routine pointer located in a global table.
- `check_bits_and_set_byte`: Checks bits 7 and 6 of a byte at offset 30 of a structure; if both are clear, it writes 254 to offset 31, checks bit 5, and calls other routines based on the results.
- `call_library_vector_132_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_234_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_252_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_276_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_294_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_318_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_324_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_330_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_366_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_414_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_432`: Library stub: loads a library base from a global variable and calls the library function at offset minus 432 of that base, returning its result to the caller.
- `call_library_vector_552_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `update_linked_list_node_3`: Stores the address of a pointer argument into the memory location pointed to by that argument, increments the value at that location by four, clears the following word, and stores the original pointer value in the word after that.
- `call_library_vector_30_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_66_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_84_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `invoke_table_entry_3`: Retrieves a function pointer from an offset within a global variable and calls it using a pointer from a second global variable as the base address.
- `store_byte_at_pointer_4`: Stores a byte from d0 at the memory location pointed to by a3 and increments a3.

### printer.device

- `load_immediate_max`: Loads the value negative one into d0 and returns.
- `process_printer_status`: Performs several memory moves, clears bits in a structure at offset 48 of a3, and conditionally calls three different library routines based on comparisons and bit tests.
- `call_unit_write_vector_from_table`: Scales d2 by four, loads the write vector from the unit table at offset 42 of a6, calls it with a3 in a1, then pops d2 and a3 and returns.
- `update_linked_list_2`: Calls a library routine, checks bits 7 and 5 of a byte at offset 30 of a structure in a1, and performs a series of conditional library calls and bit manipulations based on those bits.
- `update_linked_list`: If bit 4 is set in a byte at offset 48 of a3, it swaps two longwords at offsets 0 and 4 of a3 with the values at the locations they point to, and then calls a library routine if bit 0 is not set in that same byte.
- `manage_linked_list`: Sets a bit in a byte, performs a linked list insertion if a flag is set, and calls functions through pointers retrieved from a table.
- `execute_alert_and_cleanup`: Calls a library function with three arguments, and if the result is zero, calls another library function using a value from a global variable; otherwise, it stores the result in a global variable and calls a different library function.
- `call_library_vector_432_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 432 of that base, returning its result to the caller.
- `process_library_requests`: Performs a sequence of library calls to update several fields in a structure, fills a memory block with the constant 45019 in a loop, and clears a byte at an offset of a global variable.
- `process_request_list`: Performs a series of four library calls using specific constants in d7, unlinks several pointers from a structure, and finally calls a library function after performing arithmetic on a value from a structure.
- `jump_through_stored_vector_preserving_a1`: Saves a1 on the stack, calls the routine whose address is stored at offset zero, restores a1 and returns.
- `call_status_handlers`: Calls three different routines and then returns.
- `trigger_handler_branch`: Loads the value negative one into d0 and calls another routine before returning.
- `set_status_bit`: Sets bit 0 of a byte at offset 61 of the structure in a6 and returns.
- `clear_status_bit`: Clears bit 0 of a byte at offset 61 of a structure in a6 and returns.
- `check_handler_pointer`: Loads a pointer from an offset of a6 into a1, checks if the pointer is null, and either calls another routine or branches to a different location.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `branch_to_nearby_routine_2`: Performs a short relative branch to another routine.
- `update_structure_and_dispatch_callbacks`: Updates several fields in a structure using return values from library calls, clears a block of memory in a table, and populates a structure with data before calling a library function to iterate through a linked list of callbacks.
- `compare_and_set_bit`: Compares d0 with a value loaded from a memory offset of a6, and if they are equal, clears bit 1 of a byte at an offset of a6; otherwise, it branches to another location.
- `dispatch_structure_callback`: Checks bit d5 of d2, and if set, iterates through a pointer at offset 72 of a6, setting bit 5 of a byte at offset 30 of the pointed-to structure, storing that pointer in a global variable, and calling a function from a table; otherwise, it returns.
- `contains_zero_data_table`: Contains a sequence of instructions that perform no operation on register d0.
- `call_status_handler`: Loads a pointer from offset 4 of the stack into a0, loads a 32-bit value from offset 8 of the stack into d0, and calls another routine.
- `update_printer_status`: If a value in d0 is non-zero, it loads a pointer from a global variable into a6, calls another routine, and if the result is zero, sets bit 0 of a byte at offset 2486 of a6, otherwise it sets bit 1 of a byte at offset 2486 of a6, populates a structure at offset 108 of a6 with d0, a0, and the constant 3, calls another routine, calls another routine, and returns.
- `adjust_stack_and_branch`: Increments the stack pointer by eight and branches to another location.
- `set_status_bit_and_branch`: Sets bit 1 of a byte at an offset of a6, loads a value from an offset of a6 into a1, and branches to another location.
- `check_printer_status_4`: Loads a pointer from a fixed address into a0, reads a byte from offset 31 of that pointer, and if it is non-zero, reads a byte from offset 950 of a6, performs a bitwise NOT and AND with 3, and returns a value based on the result.
- `execute_printer_commands`: Calls a subroutine, then performs several library calls using a base address from a pointer in d1, and if a comparison between d0 and a value at offset 108 of a6 succeeds, it clears bit 0 of a byte at offset 2486 of a6 and calls another routine.
- `set_status_bit_and_call`: Loads an address into a1 from offset 94 of a6, compares d0 to a1, clears bit 1 of a byte at offset 2454 of a6, and calls another routine.
- `compare_and_branch_subroutine`: Compares d0 to a pointer at offset 272 of a6, and if they are not equal, calls another routine.
- `call_subroutine_and_branch`: Calls another routine and then branches to another location.
- `update_structure_fields_2`: Loads a pointer from a global variable into a1, writes 9 to offset 28 of that pointer, copies a longword from offset 50 of a0 to offset 32 of a1, and clears a longword at offset 36 of a1.
- `clear_status_byte`: Sets a byte at offset 30 of a structure in a1 to zero and calls a routine through a pointer found at offset 20 of that same structure.
- `check_byte_and_return`: Compares a byte at offset 8 of a structure pointed to by a1 with seven, and returns if they are equal; otherwise, it branches to another location.
- `call_printer_device_close_and_cleanup`: Saves a1 and a6, calls the printer device close vector through a6, restores registers, then calls the global cleanup vector at offset -474 and returns.
- `process_library_status`: Saves a7 to the stack, loads a pointer from a0, checks a byte at offset 31 of that pointer, and if not zero, performs bitwise operations on a byte from a global variable before returning.
- `execute_library_calls_2`: Calls three different library functions using pointers from a structure, sets bit 0 of a byte at offset 327 of that structure, and conditionally clears bit 2486 of a global variable and calls another routine if certain conditions are met.
- `clear_bit_and_call_subroutine`: Compares d0 with a value from an offset of a6, and if they are equal, clears bit 1 of a byte at an offset of a6, calls another routine, and branches to another location.
- `compare_and_branch_address`: Compares a value at offset 176 of a structure in a6 with the value in d0, then branches if they are not equal, calls a subroutine, and branches based on the zero flag in d0.
- `calls_subroutine_and_branches`: Calls another routine and then performs a short branch to another location.
- `branch_on_global_byte`: Saves d2, d3, a2, and a3 to the stack, compares a byte at offset 2457 of a6 to 1, and if not equal, pushes the address of a constant to the stack and branches to another routine.
- `close_device_and_check_status`: Copies values to a structure, calls a library function to close a device, and then uses a table to conditionally clear bit 0 or bit 1 of a byte at offset 2486 based on a value from a structure.
- `close_printer_device_and_clear_bits`: Calls a routine via a pointer to close a device, then checks the return value to conditionally clear bit 0 or bit 1 of a byte at offset 2486 of a structure in a6 using two separate calls to another routine.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `call_nested_routines`: Pushes the contents of a1 onto the stack twice, calls another routine, cleans the stack, restores a1 from the stack, calls another routine, and returns.
- `reset_structure_field`: Pushes a1 onto the stack, clears a 32-bit field at offset 32 of a structure in a0, calls another routine, and if d0 is zero, copies a 32-bit value from offset 36 of a1 to offset 32 of a1, then calls another routine twice.
- `call_nested_routines_2`: Pushes the contents of a1 onto the stack twice, calls another routine, restores a1 from the stack, calls another routine, and returns.
- `call_library_and_nested_routines`: Pushes the contents of a1 onto the stack twice, calls a routine via a pointer, cleans the stack, restores a1 from the stack, calls another routine, and returns.
- `dispatch_pending_entry`: Checks a condition via a routine call, and if met, populates a structure with two parameters and executes several routines through a global base.
- `convert_printer_data`: Checks a byte in a global variable, copies 14 longs and a word from a structure in a1 to a stack buffer, calls another routine, and then writes two bytes to a destination pointer in a3 based on values from the stack and a global variable.
- `copy_14_longs_to_stack_and_convert`: Copies 14 longs plus a word from the printer structure at offset 108 into a stack buffer, calls the conversion routine, writes one result byte and a zero to the output pointer, stores 1 in the request and returns.
- `reset_printer_status_and_copy_buffer`: Clears two global variables, checks a value in a table to set a flag at offset 58 of a structure, and copies bytes from a source buffer to a destination buffer while calling another routine to process the data.
- `initialize_printer_buffer_and_status`: Loads a longword from a pointer in a register into a global variable, clears a 16-bit word at a specific offset in that same global variable, and branches to another routine.
- `transfer_buffer_byte_and_process`: Copies a byte from a source buffer to a destination buffer using indices tracked in global variables, then enters a loop that calls another routine to process the data and checks a status byte in a system structure to decide whether to call additional routines.
- `initialize_registers_and_branch`: Clears d0, a4, d5, and d1 to zero, then branches to another routine.
- `branch_to_offset_3`: Performs a short branch to a location within another routine.
- `process_printer_buffer_and_status_4`: Iterates through a buffer to find a specific byte, then calls several routines to handle status updates, buffer management, and data copying.
- `copy_data_and_update_status`: Copies bytes from one table to another in a loop based on a counter in a5, then updates a longword at offset 32 of the structure in a3 using a value from the stack or zero depending on d1.
- `clear_a3_offset_20_return_d1_byte`: Clears the longword at offset 32 from a3, sets d0 to zero, copies the low byte of d1 into d0, restores d2-d6 and a2-a5 from the frame, then returns.
- `process_printer_device_command`: Calls another routine to process a command, then checks if the returned byte is either 104 or 108; if it matches 108 and a specific value in a global variable is 20, it performs a bitwise AND between the returned byte and 4, calls another routine, and stores the result in a global variable.
- `clear_and_branch_9`: Clears a longword at offset -4 relative to a6 and branches to another routine.
- `map_character_to_byte`: Compares a byte from a global variable with a value from a table indexed by d6; if they match and certain global conditions are met, it calculates a new byte by adding 55 to the bitwise AND of a byte from a2 and 1, then stores the result at a3.
- `compare_and_set_byte_4`: If d0 is 1 and d0 is not equal to d6, it sets a byte at the address in a3 to 255 and branches to another location; otherwise it returns.
- `compare_and_store_byte_4`: Compares d6 with the value 9 in d0, and if they are not equal or if the byte at the address in a2 is zero, it stores the byte 59 at the address in a3 and branches to another location.
- `store_status_byte`: Tests a longword at a fixed location, and if it is zero, compares d0 with the value 10; if d0 equals d6, it stores the byte 64 at the location pointed to by a3.
- `check_constants_and_store`: Compares d0 with the constant 2, branches if not equal, compares d0 with d6, and if equal, stores the byte 75 in the memory location pointed to by a3.
- `compare_and_store_byte_3`: Compares a value in d0 against two constants, and if it matches, loads a byte from a2 into d0, stores it in a global variable, and writes 76 to a location pointed to by a3.
- `copy_byte_from_table_2`: Compares d6 against 6 and 10, and if d6 is between these values, copies a byte from a table at offset 58 into a destination pointed to by a3.
- `write_conditional_byte`: If d6 is not zero, it compares a byte at a pointer in a2 to 29; if that byte is not less than 29, it writes 12 to a location in a3 and branches.
- `search_and_write_byte`: Compares d6 against seven, and if greater, searches a table for a byte from a2, then writes a calculated byte to a3.
- `update_character_table_and_shift_buffer`: Compares a byte from a buffer against a table using an index, and if a match is found, calls another routine to process a value derived from a byte at a pointer.
- `reset_printer_state_and_process_buffer`: Clears several global variables and counters, then enters a loop that processes a buffer of data by copying bytes to a destination, handling specific control characters, and calling another routine to manage printer state or character expansion.
- `fetch_and_copy_byte`: Fetches a byte from a table using an incrementing index, stores it in a destination buffer, and decrements a global counter before returning.
- `write_byte_to_buffer`: Checks if a byte count is within a specific range and if a function pointer at offset 62 of a structure is non-zero; if so, it calls that function with a pointer to a buffer, a source byte, and a global variable, then increments the buffer pointer by the returned value.
- `process_printer_command_byte_2`: Compares a byte at a pointer in a3 with several constants; if it matches a specific escape sequence, it searches a table for a matching byte, updates a destination byte at a4, and calls another routine with a value derived from that byte.
- `process_printer_command_byte`: Compares a byte at a pointer in a3 to 40; if equal, it sets a byte at a4 to 255, calls another routine, and then performs a loop that compares a byte from a3 against a table of 11 values to determine an index to store in a4.
- `check_printer_command`: Compares a byte at the address in a3 with 91; if they are not equal, it branches to another location, otherwise it branches to another routine.
- `process_character_mapping_table`: Iterates through a table of eight entries, comparing a byte from a source pointer to a value in the table; if a match is found, it writes the value to a destination pointer and calls another routine with the extended byte value.
- `handle_printer_byte`: Compares a byte at the address in a3 with 10; if they are not equal, it branches to another location, otherwise it checks a global variable and writes 3 to offset 68 of a structure in a4 if the variable is non-zero.
- `execute_callback_and_update_table`: Calls another routine with parameters pushed to the stack, stores the return value in a global variable, and if the return value is negative, increments a pointer in a2 and writes either the value ten or thirteen to a table at the resulting address, or updates a global index and a value if the pointer exceeds a limit.
- `increment_and_write_bytes`: Reads a longword from a pointer in a2, increments it, writes the byte ten to a table at the new address, repeats the process with the value thirteen, and branches.
- `increment_and_write_ten`: Reads a longword from a pointer in a2, increments it, writes the byte ten to a table at the new address, and branches.
- `compare_and_call_printer_command`: Compares a byte at the address in a3 with 155; if they do not match, it branches to another location, otherwise it pushes two longwords onto the stack and calls another routine before returning via a branch.
- `parse_file_path_string`: Checks if a byte is greater than or equal to 160; if so, it calculates an index into a table of strings based on the byte value, retrieves a pointer from that table, and clears a global counter.
- `parse_numeric_string_segment`: Checks if a byte at the current pointer is a backslash, then verifies if the subsequent three characters are numeric digits between zero and nine; if they are, it increments a counter and advances the pointer past the digits, otherwise it returns.
- `update_global_pointer_2`: Increments a pointer in a1, increments a long value at offset 1060, checks if the byte at a1 is non-zero, and if so, performs arithmetic on a value from a2 to update a long at offset 1038 or a long at offset 103e.
- `calculate_and_load_byte`: Calculates an address by performing arithmetic on a value in d0 derived from a byte at a3, then loads a byte from that calculated address into a1.
- `skip_non_numeric_characters`: Loops through a string starting at a1, skipping characters that are not digits between 0 and 9.
- `parse_numeric_string_to_buffer`: Checks if a byte at a pointer is a digit between 0 and 9, converts up to three consecutive numeric characters into a single integer, and stores that integer into a buffer pointed to by a second pointer while incrementing a counter in that pointer's structure.
- `copy_byte_to_pointer`: Loads a pointer from a2 and increments it, then copies a byte from a1 to the location pointed to by a5, which was loaded from the original pointer, and loops if the byte at a1 is not zero.
- `copy_byte_and_update_printer_state`: Copies a byte from a source buffer to a destination buffer using a pointer and an index, then increments the source pointer.
- `subtract_d4_delta_from_printer_counter`: Subtracts the difference between d4 and the stored value at offset 1038 from the counter at offset 1030, clears the longword at offset 1048, then returns the longword pointed to by a2.
- `execute_callback_with_extension`: Pushes d2 to the stack, loads a byte from offset 11 of the stack into d0, clears a long at offset 1060, extends d0 to a word, calls a subroutine with a pointer from offset 510, and branches based on the result.
- `update_pointer_and_call`: Adds d1 to a pointer from a global variable, compares the result to 256, and if it is greater, stores d1 in another global variable and calls another routine if a byte on the stack is non-zero.
- `update_pointer_and_branch`: Decrements a long value from a global variable by one, stores the result back into another global variable, and branches to another routine.
- `clear_register_and_branch`: Sets register d0 to zero and branches to another routine.
- `copy_bytes_from_table_to_buffer`: Copies bytes from the table at address 766 indexed by d0 into the buffer pointed to by the pointer at address 1056, incrementing that pointer, until d0 reaches d1, then returns zero in d0 and d1.
- `clear_memory_via_stack`: Saves registers d2-d4 and a2-a4 to the stack, loads values from offsets 28 and 32 of the stack, sets a pointer in a3 from offset 36 of the stack, clears a byte at the address in a3, and branches.
- `clear_byte_on_match`: Compares d1 to 59, and if equal, increments a4 and clears a byte at the location pointed to by a4 plus the offset in a3.
- `set_flag_on_match`: Compares d1 to 32, and if they are equal, loads a pointer from the stack and writes 1 to the location pointed to by that pointer before branching.
- `compare_byte_and_branch`: Compares the byte in d1 to 34; if it is not equal, it branches to another routine, otherwise it loads a value from the stack into a1, sets d0 to 2, and branches to another routine.
- `process_character_and_update_pointer`: Multiplies a byte from a buffer by ten, stores the result back into the buffer, calls another routine, and if the byte is within specific ASCII ranges, updates a global pointer with the value of a4.
- `fetch_table_byte_with_increment`: Increments a pointer in a global variable, retrieves a byte from a table using the previous pointer value as an index, and calls another routine if a second global counter exceeds 255.
- `tiny_moveq_move_rts`: Small routine of 3 instructions that loads 0 into d0, then copies d1 to d0, then returns.
- `modify_global_pointer`: Loads a long from a global variable, decrements it by one, stores the result into a global variable, and branches to another routine.
- `scan_back_for_escape_byte_in_table`: Decrements the long at (a1) until it drops below one or the table byte indexed by it masked with 127 equals 27, then sets a flag word and returns the count.
- `reset_printer_state_and_send_escape`: Clears several global bytes and longwords, optionally calls two subroutines to check and copy parameters, then stores three escape-sequence bytes into a table and sets two global variables.
- `copy_data_to_globals`: Copies six 16-bit values from a structure into a set of global variables.
- `compare_structure_values`: Compares six 16-bit values from a structure against six global variables, returning 0 if all match or -1 otherwise.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `transfer_printer_data_2`: Copies four bytes from a structure in a0 to the stack, compares a word at offset 32 of that structure with 76, and if they are equal, stores a byte from offset 34 of the structure into a global variable.
- `transfer_printer_data`: Pushes two longwords from offsets -4 and -264 relative to a6 onto the stack, moves a word from offset 32 of a structure in a0 to d0, pushes d0 onto the stack, calls another routine, and if d0 is greater than or equal to the returned value in d3, branches to another location; otherwise it branches to a different location.
- `process_printer_buffer_data_2`: Checks a condition in a register and, if met, calls another routine with a pointer to a constant; if that returns zero, it checks a byte at offset 2720 of a structure, and if that byte is non-zero, it enters a loop that iterates through a buffer, calling another routine to process segments of data until a specific byte is found or the buffer length reaches zero.
- `load_pointer_and_branch_2`: Loads a pointer from a global variable into a2, copies that pointer to d1, and performs a short branch to another routine.
- `process_buffer_loop`: Increments a2 and decrements d3 in a loop, performing various checks and calls to other routines based on the values in a2, d2, d4, and d5, eventually returning a byte in d0.
- `execute_callback_and_copy_buffer`: Calls a function pointer retrieved from a global table using an index derived from a value at a specific offset in a0, then if the return value is not zero, copies a byte from a source to a destination using offsets in d1 and d2.
- `load_indexed_word`: Loads a pointer from an offset of a0, calculates an index by shifting a word from the stack, and loads a longword from a table at that index into d1.
- `compare_and_branch_d0`: Compares a byte in d0 to 255, branches to another routine if they are not equal, otherwise sets d0 to 255 and branches to another routine.
- `process_character_buffer`: If a byte in d0 is 254, it clears d0 and copies a byte from a source to a destination using offsets in d1 and d2; otherwise, it checks if d0 matches specific values and performs conditional byte copies.
- `update_printer_status_and_callback`: Copies several fields from a structure into a local stack frame, retrieves a value from a global variable and masks it, calls another routine with a pointer to a stack-allocated word, and then checks a bit in a global structure to decide whether to call a function via a pointer or branch to another routine.
- `update_printer_status_flags`: Modifies a bitmask in a stack frame by combining bits from a local word and a global variable, then calls a routine via a pointer stored in the stack frame.
- `test_dimension_scaling_flag`: Copies a longword from a global variable to a location in a stack frame, checks if bit 9 of a word from the stack frame is set, and if so, loads the constant 16 into a register before branching to another routine; otherwise it returns.
- `set_dimension_scaling_flags`: Reads a byte from a structure to determine a mode, then sets specific bits in a local stack frame based on comparisons of values within a data structure.
- `process_dimension_scaling`: Checks specific bits in a structure to set flags in a local stack frame, then calculates scaled dimension values by retrieving data from a table and performing arithmetic operations, including a bit shift by three.
- `calculate_dimension_offsets`: Calculates two 16-bit values by subtracting values from a table via a pointer in a structure and adding one to each result, then stores them in local stack offsets.
- `validate_and_scale_dimensions`: Loads two 16-bit values from a table via a pointer in a structure, scales the first value by eight, and performs multiple bounds checks against a provided value and other structure fields; if all checks pass, it calculates scaled dimensions using values from a global table and stores the results in the structure.
- `update_buffer_dimensions`: Loads two 16-bit values from a table into a structure in a6, then calculates a scaled value by multiplying a field from a global table by a register value if a specific bit is set.
- `check_bit_and_call_routine`: Tests bit 6 of a byte at offset minus 61 from a6; if set, it branches to another location, otherwise it loads a longword from offset minus 108 from a6 into d1, moves d4 into d0, calls a routine, moves the result into d3, and branches to another location.
- `calculate_buffer_size_and_validate_range`: Calculates a value by subtracting one field from another in a table and calling another routine, then compares a 16-bit value from a table against 1024 and 2048 to determine the next branch.
- `increment_and_branch_8`: Copies d4 to d3, adds 6 to d3, sets d1 to 12, and branches to another location.
- `increment_and_branch_different_values`: Copies d4 to d3, adds 7 to d3, sets d1 to 15, and branches to another location.
- `clamp_buffer_size_value`: Increments a value in register d4 by 5, calls another routine to calculate a new size, and then compares this result against a value stored at offset 124 of the structure in a6.
- `calculate_scaled_buffer_size_2`: Loads a 32-bit value from a stack offset and performs a 32-bit multiplication using a 16-bit constant from a table, then adds the result to a value derived from a 16-bit word and shifts the final sum right by 3 bits.
- `check_status_and_call_function`: Tests bit 6 of a byte at offset negative 61 from a6; if set, it branches to a different location, otherwise it loads a longword from offset negative 112 from a6 into d1, moves d2 to d0, calls another routine, moves the result to d3, adds 5 to d3, moves 10 to d1, and branches to another location.
- `calculate_scaled_buffer_size`: Performs a 32-bit multiplication of a value from a stack offset by a 16-bit constant from a table, then shifts the result right by 3 bits.
- `modify_d2_via_d3`: Moves d2 to d3, adds 4 to d3, moves the result back to d2, performs a logical shift right by 3 bits on d2, and branches to another location.
- `update_buffer_size_limit`: Calls another routine to calculate a value, then compares this value against a field in a structure; if the calculated value is larger than the existing field, it updates the field with the new value, otherwise it defaults to a constant of an address if the field is zero.
- `initialize_buffer_dimensions`: Sets a value in a structure to an address if a specific field is zero, and updates two 32-bit dimension values based on a comparison of a global value against 33.
- `calculate_scaled_buffer_dimensions`: Checks a value at an offset in a global structure; if it is at least 33, it scales two separate values by 10 using a multiplication routine and stores the results into a structure in a6 at offsets 56 and 60.
- `update_buffer_dimensions_and_allocate`: Sets two 32-bit values in a structure to 6 and 7, then doubles them if certain flags are set.
- `update_buffer_limits_and_clamp`: Shifts and updates several 16-bit and 32-bit values within a structure based on bitwise flags, including a bitwise manipulation of a value at offset 132, and performs conditional updates to values at offsets 16 and 8 by calling multiply_32_bit_integers_4 and other routines.
- `adjust_buffer_limits_and_clamp`: Checks status bits in a structure to perform bitwise shifts and arithmetic updates on values at offsets 16 and 8, then calls a 32-bit multiplication routine to update these values based on calculated products.
- `update_buffer_and_clamping_limits`: Performs bitwise shifts and arithmetic updates on a 32-bit value at offset 8 of the structure, followed by a loop that calculates products using 16-bit multiplication to update several fields at offsets 44, 48, 60, and 64.
- `update_buffer_size_and_status_4`: Compares a value in d3 with a value in a3 and, if d3 is larger, calls another routine to update a 32-bit field at offset -16 of a structure in a6.
- `update_buffer_size_and_status_3_2`: Calls a helper routine to calculate a value based on a 16-bit word in d6, then updates a 32-bit field at offset 16 of a structure in a6 by either subtracting the result or adding the difference between the 16-bit word and the result to the existing field.
- `update_buffer_size_and_status_3`: Calculates a new value by subtracting a 16-bit word in d4 from a 16-bit word in d6 and adds the result to a 32-bit field at offset 16 of the structure in a6.
- `update_buffer_size_and_status_2`: Calls a helper routine and uses the returned value to update a 16-bit buffer size field at offset 8 of a structure, either by subtracting the value or by adding the value to the existing field after performing a comparison with a shifted version of the original size.
- `update_buffer_size_and_status`: Updates a buffer size in a structure by subtracting a value from a 16-bit word and adding the result to a field at offset 8 of the structure in a6.
- `calculate_buffer_size_and_update_status`: Checks a flag in a global table, calculates a size using a 16-bit multiplication and a helper routine, and compares the result against a value in a structure.
- `fetch_and_store_status_values`: Compares a 16-bit value from the stack with a 32-bit value, then calls two separate routines to store the resulting 16-bit values at offsets 198 and 200 of the stack frame before branching to another location.
- `update_status_and_calculate_quotient`: Calls two helper routines to store values at offsets 198 and 200 of the stack frame, then performs a division using a value from register d6 and a value from offset 16 of the stack frame, storing the quotient at offset 98 and the remainder at offset 90.
- `calculate_division_and_check_status`: Divides a value at offset 16 of the stack frame by a value in d6, stores the quotient at offset 98, and calls another routine to update a value at offset 90.
- `iterate_library_function_table`: Iterates through a table of function pointers by scaling an index, calls a function from the table, and sets a bit in a status word if the return value does not match a specific bitmask.
- `load_function_pointer`: Increments d5, compares it to a word at offset -50 of a6, and if bit 4 of offset -187 of a6 is clear or the longword at offset -36 of a6 is zero, it branches to another routine; otherwise it loads a pointer from offset -36 of a6 into a1 and then loads a pointer from offset 8 of a1 into a0 before branching.
- `check_and_set_bit`: Tests a longword at offset 8 of the structure in a0; if it is non-zero, it performs a bitwise OR on a word at offset -188 of a6, then branches.
- `dispatch_library_call`: Loads a pointer from a0, then if the longword at a0 is non-zero, it checks bit 6 of offset -187 of a6; if bit 6 is set, it pushes two pointers onto the stack and calls a routine at offset 0, then stores d0 at offset -292 of a6 and enters a loop that adds 8 to the stack pointer and branches if the result is non-zero.
- `initialize_and_call_callback`: Copies twenty-four bytes from one array to another, sets a byte at offset 36 to 255, clears a longword at offset 4, and clears a longword at offset 0 of the destination structure.
- `copy_and_initialize_array`: Copies ten longwords from one array to another, clears a value at offset 8 of the destination array, and sets a word at offset 2 of the destination array to 1 before performing a comparison and conditional branch.
- `calculate_and_store_array_size`: Calculates a value by adding 16 to a word read from an offset in a6, then stores the result multiplied by two into a field at offset 4 of a structure pointed to by a6.
- `branch_with_constant_3`: Sets d5 to 1 and branches to another routine.
- `calculate_prefix_sum_and_populate_buffer`: Calculates a prefix sum of values from a table and stores the results into a new list, then calculates a bit-shifted value based on a byte from a structure, performs bitwise XOR operations on a buffer of words, and calls another routine in a loop to populate that buffer.
- `clear_and_branch_11`: Moves a longword from a3 to a negative offset from a6, clears d5, and branches to another routine.
- `unpack_rgb_to_buffer`: Calls a routine to retrieve a 4-bit color value and unpacks its nibbles into three consecutive bytes of a buffer.
- `load_constants_and_compare`: Loads a long constant into d0, loads a pointer from an offset into a0, compares a word at offset 960 of the structure pointed to by a0 with 1, and if equal, loads a long constant into d2 before branching.
- `xor_data_and_branch`: Clears d2, performs an XOR operation on d0 using d2, loads a pointer from an offset of negative 280 from a6 into a3, clears d5, and branches to another routine.
- `xor_printer_command_list`: Iterates through a list of pointers, performing an XOR operation on the data at each pointer using a value in d0, and optionally performs a second XOR at an offset of 128 if bit 7 of a byte at offset 43 of a2 is set.
- `process_printer_command_list`: Iterates through a list of pointers, calling a routine for each element and optionally calling it again with an offset if a specific bit is set.
- `dispatch_printer_command_status`: Calculates a new pointer by adding a word from offset 240 of a structure to a value at offset 24 of the same structure, stores this result at offset 208, and then checks if bit 8 is set in a word at offset 222; if set, it calls another routine with a pointer from a stack-allocated list and a constant value.
- `process_printer_command_status`: Pushes a constant onto the stack, loads a word from offset 240 of a structure into d3, moves d3 to the stack, calls another routine, moves the result from d0 to offset 244 of the structure, increments the stack pointer by 8, and branches to another routine if the result is not zero, otherwise it branches to another routine.
- `dispatch_printer_command_via_library`: Checks if bits 10 and 11 are set in a word at offset 222 from a6; if so, it increments a value at offset 208, shifts it left by two, stores the result at offset 230, and calls a library function with the new value and the constant 40961.
- `dispatch_printer_device_command`: Checks a bit in a word at offset 222 from a6; if set, it increments a value at offset 208, shifts it left by two, stores it at offset 228, and calls a routine using a parameter loaded from offset 228 and a constant value of 40961.
- `scale_and_call_library_function`: Scales a 16-bit value in d6 by four, stores it in a structure at offset negative 234 from a6, and calls a library function using a pointer from the stack.
- `allocate_and_initialize_list_3`: Checks two bits in a structure at offset negative 187 from a6; if both are set, it doubles a 16-bit value from offset negative 30 of the structure, passes this value as an argument to another routine, stores the returned pointer at offset negative 272, and iterates through a loop to populate a list of pointers using values from a structure.
- `branch_to_next_instruction_2`: Moves a word from offset 42 of a structure in a6 into d4 and branches to another routine.
- `allocate_and_initialize_list_2`: Doubles a 16-bit value from a local variable, calls another routine to allocate a memory block of that size, and then iterates through the newly allocated block to populate it with results from a loop of calls to another routine.
- `allocate_and_initialize_list`: Calculates a size by doubling a value from a register, calls a routine to allocate a memory block, and then iterates through a loop to populate the allocated list with results from another routine.
- `update_callback_list_and_dispatch`: Iterates through a list of pointers, calling a routine for each element and storing the result back into the list.
- `reset_and_dispatch_callback`: Fills a memory range with ones based on a count in a structure, then calls a function via a pointer in the same structure while setting bit 5 of a status byte at offset 188.
- `update_printer_status_and_limit`: Calls a function via a pointer in a structure, increments a value in a structure by one, and writes it to offset 134.
- `increment_and_branch_6`: Increments d0 by the value in d4 plus 3 and branches to a different location.
- `set_structure_limit`: Writes the value 4 to offset -134 of a6, then compares the sum of values at offsets -72 and -214 of a6 against the value at offset -210 of a6, and if the sum is greater than or equal to the comparison value, sets bit 7 of the word at offset -188 of a6.
- `process_stack_parameters`: Calls a subroutine using a pointer from a6, loads values from offsets 338 and 354 of a6, and if bit 11 of the value at offset 354 is set, it either fills a memory area pointed to by a3 with 65535 in a loop or calls a subroutine with a decremented value from offset 342.
- `fill_memory_and_call_routines`: Fills a memory area pointed to by a3 with 65535 in a loop, calls a routine with parameters from the stack, and if certain conditions are met, calls another routine using a value from d4.
- `call_conditional_routine`: Tests a longword at a negative offset from a6, and if it is not zero, pushes a word from d4 to the stack and calls another routine.
- `branch_on_test`: Tests a longword at a negative offset from a6, and if it is not zero, it pushes a 16-bit value to the stack, calls another routine, and branches.
- `process_callback_list`: Pushes a word from d4 onto the stack and calls a routine, then enters a loop that compares d2 with a value at offset 210 of a6; if they are equal, it calls a routine using a pointer from a3, increments d4, and compares it against a value at offset 214 of a6.
- `align_offset_value`: Pushes a longword to the stack, calls another routine, and if the returned value in d0 is less than 4, adds 10 to the value in d2 and branches.
- `finish_printer_offset_alignment`: Loads divisor ten and the offset in d2, calls the alignment routine, then pops d2 from the stack and returns to the caller.
- `set_printer_status_bits_from_table`: Reads a byte from the printer device structure, indexes a word table with it, and if nonzero clears bits 8-10 of the caller's longword and ORs in the table value.
- `open_printer_device_and_free_buffers`: Saves registers, calls the device open vector, then frees the raster, color table and several optional buffers through the memory free routine, storing the unit number in the open structure.
- `call_absolute_vector_with_saved_registers`: Saves d2-d4, a2, and a6, loads an absolute vector from a constant address, passes arguments from the stack in d0-d3, a1, a2, and d4, calls the vector, and restores the registers before returning.
- `calculate_dimensions`: Subtracts d0 from d2 and d1 from d3, extending the results to longwords, and if both results are greater than negative 2, sets a3 and a4 to a0 and a1 and clears d6.
- `blit_rows_accumulate_byte_count`: Loops over d3 rows calling another routine with a3/d7 and a4/d5, advancing d5, a2 and d6 by d2 each pass, then returns the accumulated d6 and restores registers.
- `return_negative_one_2`: Sets d0 to negative one, restores registers from the stack, and returns.
- `call_printer_vector_with_stack_args`: Saves d2-d4/a2/a6, loads the printer device base from absolute address 20, copies stack arguments into d0-d2/a0-a2/a1/d4 and calls the function pointer at absolute address 162, then restores registers and returns.
- `save_registers_and_call_subroutine`: Saves d2 through d6 and a2 through a3 on the stack, tests d4, and if not zero, saves a0 and a1, calls a subroutine, restores a0 and a1, moves d2 to d4, and branches.
- `expand_bitstream_to_words`: Expands a bitstream from a source table into a destination buffer by extracting bits from words and propagating them through a sequence of sixteen 16-bit words.
- `fill_memory_with_longword`: Repeatedly writes the contents of register d5 into eight consecutive longword locations starting at the address pointed to by a3, then branches to another routine.
- `shift_bitstream_buffer`: Performs a series of logical left shifts on sixteen 16-bit words in a buffer pointed to by a3, effectively shifting the entire bitstream by one bit position.
- `shift_bitstream_buffer_contents`: Performs a bitwise left shift across a buffer of sixteen 16-bit words, propagating the carry bit from one word to the next.
- `call_subroutine_with_saved_d2`: Pushes d2, loads d0-d2 and a0-a1 from the caller's stack arguments, calls the routine at offset 248, then restores d2 and returns.
- `rotate_and_nested_dbra_restore_2`: Calculates a value using a multiplication and several additions from a structure, then performs nested loops to rotate words in a memory block and copy data to a destination.
- `setup_source_dest_pointers`: Loads two pointers from two memory locations into registers, adds a value in d0 to the first pointer, and branches to another routine.
- `copy_bytes_and_branch`: Copies a byte from a source pointer in a2 to a destination pointer in a3, decrements d4 in a loop, and then branches to another routine.
- `transfer_pointer_and_branch`: Copies the pointer in a3 to a2, moves a word from d2 to d4, and then branches to another routine.
- `rotate_and_nested_dbra_restore`: Rotates a word left through the memory location pointed to by a2, executes three nested dbra loops using d4, d5, and d1 as counters, and restores d2 through d5 and a2 through a3 from the stack before returning.
- `call_graphics_lib_close`: Saves a6, loads the graphics library base from absolute address 20, calls the library function at offset -726 with the argument from the stack, restores a6 and returns.
- `call_graphics_lib_write_pixel_array`: Saves d2 and a6, loads the graphics library base, passes three arguments from the stack to the library function at offset -756, restores registers and returns.
- `call_get_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls GetRGB4 and returns its result to the caller.
- `call_read_pixel`: Library stub: loads its arguments and the graphics library base, calls ReadPixel and returns its result to the caller.
- `perform_division_and_swap`: If d2 is greater than 65535, it performs a series of divisions and swaps using d1 and a1, otherwise it performs a division and swaps the high and low words of d0 and d1.
- `rotate_32bit_value_2`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers_4`: Computes the 32-bit product of two 32-bit registers using 16-bit partial products and returns the result in d0.
- `forward_call_to_helper`: Calls a helper and returns, with no other work.
- `call_helper_return_register_value`: Calls a helper and returns the value the helper leaves in a data register.
- `negate_result_on_flag`: Calls a helper for a value and negates that value when the flag the helper leaves in an address register is nonzero.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `call_library_vector_198_10`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_456_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_210_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_534`: Library stub: loads a library base from a global variable and calls the library function at offset minus 534 of that base, returning its result to the caller.
- `queue_printer_request`: Checks bit 3 of a byte at offset 21 of a structure pointed to by a pointer at offset 4 of a7; if set, it calculates a bitmask by shifting 1 left by the value of a byte at offset 5 of a structure pointed to by offset 4 of a pointer at offset 4 of a1, then calls a routine using a1 and the bitmask as arguments.
- `queue_printer_write_after_flag_check`: Builds a one-bit mask shifted by the byte at offset 5 of the unit structure, pushes the mask, the buffer at offset 20 of the unit structure, and the unit structure itself, then calls another routine and returns.
- `dispatch_device_vector`: Loads a pointer from offset 4 of a7 and a pointer from a constant address, checks bit 3 of a byte at offset 21 of the second pointer; if set, it calls a routine using a0, a word from offset 88 of a0, and the longword at offset 24 of a0 as arguments.
- `call_device_vector_with_offset_args`: Loads a word from offset 88 of a structure into d0, pushes d0, the longword at offset 24 of the structure, and the structure pointer, then calls another routine and returns.
- `initialize_stack_frame_2`: Sets up a stack frame, loads pointers from offsets 8 and 12 of a6 into a1 and a0, and branches if bit 0 of a byte at offset 79 of a1 is not set.
- `test_bit_and_set_d0_2`: Clears d6, tests bit 1 of a byte at offset 79 of a1, and if set, sets d0 to 1 and branches.
- `test_and_set_status`: Clears d0, stores it at offset negative 2 of the structure in a6, tests bit 2 of a byte at offset 79 of a1, and if set, sets d0 to 1 and branches.
- `scale_and_subtract_bytes_3`: Iterates through a buffer, calculating scaled differences between bytes at offsets 0, 1, and 2 using a value from offset 3, and updates the buffer based on comparisons with d4 and status flags in a stack variable.
- `subtract_and_compare_byte`: Subtracts a word from d4 from the byte at offset 2 of a0, then compares d1 and d3 to decide whether to update d0 with either d1 or d0.
- `subtract_bytes_in_loop_2`: Subtracts a word from d1 from the byte at offset 2 of a0, then compares d1 and d4 to decide whether to update d0.
- `subtract_bytes_in_loop`: For each of d2 entries, subtracts the word in d1 from the byte at offset 1 of a0 and advances a0 by four.
- `test_bit_status`: Sets up a stack frame, loads pointers from offsets 8 and 12 of a6 into a1 and a0, and if bit 0 of a byte at offset 79 of a1 is set, sets d5 to 1 and branches.
- `test_bit_and_set_d6`: Clears d5, tests bit 1 of a byte at offset 79 of a1, and if not set, sets d6 to 1 and branches.
- `check_bit_and_set_d0`: Clears d6, tests bit 2 of a byte at offset 79 of a1, and if not set, sets d0 to 1 and branches.
- `process_scaled_buffer`: Loops through a buffer to update bytes at offsets 0, 1, and 2 by adding scaled differences derived from comparisons between the bytes and a value in d4, controlled by d5 and d6.
- `scale_and_add_byte`: Subtracts a word from d4 from d2, adds half of that result to the byte at offset 0 of a0, and performs several comparisons involving d2, d3, d4, and d6 to decide whether to update d0 with d2 or branch to another routine.
- `scale_and_subtract_bytes_2`: Subtracts a word from d4 from d3, adds the half of the result to the byte at offset 2 of a0, and performs several comparisons against d2 and d4 before branching.
- `scale_and_subtract_bytes`: For each of d1 entries, subtracts d3 from d4, divides the result by two, and adds that byte to the byte at offset 1 of a0 before advancing a0 by four.
- `check_library_status_3`: Loads a pointer from offset 8 of a structure, loads another pointer from offset 20 of that pointer, and checks bit 3 of a byte at offset 21 of a third pointer; otherwise it clears d0 and branches.
- `check_library_status_2`: Stores the value 15 in a stack location, loads a pointer from a global variable, compares a word at an offset of that pointer to 1, and branches based on the result.
- `process_buffer_data_stream`: Checks a bit in a structure to determine whether to use an addition or subtraction operation on two words to calculate a loop limit, then stores a value and a word into the stack frame before branching to another routine.
- `prepare_offset_data`: Loads a pointer from offset 28 of a2, a word from offset 86 of a2, and a word from offset 90 of a2, then adds the two words together, stores 1 in a stack location, and stores the word from offset 84 of a2 in another stack location before branching.
- `call_printer_vector`: Pushes several arguments onto the stack, including a value from a structure in a2, a pointer from a2, and a signed version of d2, then calls a routine via a pointer in a2.
- `fetch_table_entry`: Loads a word from a pointer in d5, increments d5, and if the word is greater than 16, loads a longword from a table using an index derived from that word into a location at a negative offset from a6.
- `compare_and_store_byte_2`: Compares a value in d3 with 48, and if d3 is not greater than or equal to 48, performs an XOR between a value from a6 offset -8 and a masked version of d3, then stores the result at a6 offset -12.
- `compare_and_store_byte`: Compares the value 48 with d3 and, if d3 is not greater than or equal to 48, stores the byte from d1 at offset -10 of a6 before branching.
- `iterate_and_call_vector`: Stores a byte from d1 at offset -11 of a6, increments d4, and if d4 is not less than the word at offset -2 of a6, it loads a word from offset 112 of a2 into d0, pushes it to the stack, calls another routine, loads the result into a3 and increments a3, adds the word at offset -4 of a6 to d2, and if d2 equals d6, it returns; otherwise it branches to another routine.
- `branch_to_offset_11`: Performs a short branch to a different location.
- `copy_words_and_call_with_offset`: Copies a longword from a4 to a3, calls a routine with a3 and a word from a2 offset 112, advances a3 by four, adds a stack word to d2, and loops until d2 equals d6.
- `test_status_bits_2`: Saves d2-d4 to the stack, loads a pointer from offset 16 of the stack, loads a word from offset 22 of the structure in a0, sets a pointer in a1 from offset 24 of a0, and tests bits 0 and 3 of specific bytes to set d3 to 2.
- `test_bit_and_set_d0`: Tests bit 0 of a byte at the address in a1, and if the bit is set, sets d0 to 1; otherwise it sets d3 to 0 and branches.
- `clear_d0_and_branch_23`: Sets register d0 to zero and performs a relative branch to another location.
- `test_bit_and_set_d3`: Tests bit 3 of a byte at an offset of 2 from a1, and if the bit is set, sets d3 to 2; otherwise it branches.
- `check_bit_and_branch`: Clears register d3, tests bit 3 of the byte at the address in a1, and branches to another routine if the bit is not set, otherwise it clears register d0 and branches to another routine.
- `check_and_set_byte`: Checks if d3 is not equal to 1, and if so, stores the byte value 6 at offset 3 of a1.
- `scale_and_store_bytes`: For each of d4 entries, calculates a value by adding d3 shifted left by two and d3, stores the result at offset 3 of a1, and advances a1 by four.
- `update_node_data_list`: Iterates through a list of pointers to update a destination array by either copying values from a base address or performing arithmetic additions.
- `check_structure_flag`: Loads a pointer from an absolute address, compares a word at offset 96 of that pointer to 1, and if not equal, sets d0 to 15 and branches.
- `expand_and_accumulate_indexed_data`: Checks flags in a structure to determine whether to copy or accumulate long values from a table into a destination buffer, or to perform a nibble-based expansion of values into the buffer.
- `compare_nibble_and_branch`: Performs bitwise operations on d1 and d0 to compare a nibble, then branches based on whether the result is 1 or 2.
- `store_byte_to_stack`: Stores the byte from register d1 into a location at an offset of 4 bytes below the address in a6, then branches to another routine.
- `store_byte_to_stack_offset_2`: Stores the byte from register d1 into a location at an offset of 2 bytes below the address in a6, then branches to another routine.
- `copy_indexed_longs_to_output_buffer_3`: Calls a routine with a pointer from a6 offset -4, then adds the returned value to the location pointed to by a4 and advances a4, looping until d3 is zero.
- `copy_indexed_long`: Tests bit 3 of a byte at offset 113 of a structure in a2, copies a long from a4 to a location on the stack, loads a word from a3 and increments it, and if the word is less than 16, copies a long from an indexed table into the stack location.
- `filter_and_compare_nibble`: Extracts the lower four bits of a word in register d2, performs an XOR operation between those bits and the lower four bits of register d5, and then branches to different locations based on whether the value in register d2 shifted right by four bits is less than one, equal to two, or neither.
- `store_byte_and_branch`: Stores a byte from register d1 into the memory location at offset negative 4 from register a6, then performs an unconditional short branch.
- `store_byte_and_branch_2`: Stores a byte from register d1 into the memory location at offset negative 2 from the pointer in a6, then performs a short branch to another routine.
- `copy_longs_to_output_buffer`: Calls a routine with a pointer from a6 offset -4, then copies the returned value to the location pointed to by a4 and advances a4, looping until d3 is zero.
- `copy_indexed_longs_to_output_buffer_2`: Reads words from a3, uses each scaled by four as an offset into the table at d4, copies the long to a4, decrements d3 and loops until zero.
- `copy_indexed_longs_to_output_buffer`: Reads words from a3, uses each scaled by four as an offset into the table at d4, copies the long to a4, decrements d3 and loops until zero, then restores registers and returns.
- `render_band_rows`: Iterates through a table of row counts and color data, calculates offsets, and calls another routine four times to write bytes to an output buffer at the address in a4, repeating this process for each entry in the table until a counter in a2 reaches zero.
- `render_band_rows_to_printer_buffer`: Walks a table of row counts and colour bytes, calls the band renderer for each row, stores returned bytes into the output buffer and repeats until the row counter d6 reaches zero, then flushes.
- `dispatch_nested_structure_comparisons`: Iterates through a list of structures and nested sub-structures, performing various comparisons between byte values and calling another routine with a status code, a value, and parameters based on the comparison results.
- `process_nested_data_structures_2`: Iterates through a list of structures and, for each structure, iterates through a nested list of sub-structures.
- `update_loop_counter_and_pointer`: Adds a 16-bit value from the stack to another value on the stack, increments the address in a3 by 4, decrements a 16-bit counter on the stack, and branches to another routine if the counter is zero, otherwise it branches to another routine.
- `loop_and_call_handler`: Decrements a 16-bit counter in a local variable using a value from a2, and if a byte at a specific offset matches a value in d4, calls a routine using a pointer from a4.
- `update_structure_counter`: Reads a 16-bit value from an offset of 6 bytes below the frame pointer, adds it to a 16-bit value at an offset of 4 bytes below the frame pointer, increments a pointer in a3 by 4, and branches to another routine or a different location based on whether the updated value is zero.
- `process_nested_data_structures`: Iterates through a list of structures and, for each structure, iterates through a nested list of sub-structures.
- `process_structured_data_entries`: Iterates through a list of structures, performing comparisons and calling another routine with various parameters based on byte values and a status bit.
- `process_structured_data_entries_2`: Iterates through a list of structures, performing comparisons and calling another routine with various parameters based on byte values and a.
- `convert_scanline_to_printer_dot_row`: Takes a scanline number and callback pointer, reduces the line modulo the printer height at offset 24, then calls the callback with the row and command 2, followed by command 3 on success.
- `pick_minimum_colour_byte_and_store_2`: Calculates a weighted average of three bytes from a0, applies a scaling factor if a bit in a global variable is set, and stores the result in the fourth byte of the structure at a0.
- `pick_minimum_colour_byte_and_store`: Compares the first three bytes at a0, keeps the smallest value, optionally scales it when a status bit is set, and writes it to the fourth byte.
- `swap_buffer_and_baud_fields`: Swaps the two longwords at offsets 40 and 44 of the structure, then swaps the two words at offsets 74 and 76, and returns.
- `update_cached_baud_if_changed`: Compares the word at offset 110 with the cached word at offset 88; if different, copies the new value to offset 88 and calls the swap routine.
- `reload_baud_from_clock_divisor`: When the word at offset 110 equals the cached word at offset 88, computes offset 92 minus the word at offset 106, stores it at offset 88 and calls the swap routine.
- `swap_io_stdin_stdout_ptrs`: Loads the file handle from the stack, swaps the two longword pointers at offsets 24 and 32 in that structure, and returns.
- `advance_byte_offset_with_wrap`: Subtracts a step from the word at the given pointer, increments the returned index and adds a wrap value when the result reaches zero or underflows.
- `process_pixel_buffer_chunks`: Checks a bit in a status structure and, if set, enters a loop that calls a pixel reading routine to fetch data and stores the results into a destination buffer.
- `process_pixel_buffer`: Checks bit 6 of a byte at offset 113 of a3; if set, it loads a pointer from offset 4 of a3, two words from offsets 84 and 88, and a word from d2, then calls a routine using the pointer, the extended d0, and the incremented d3, finally storing the result in a2 and looping if d4 is greater than zero.
- `compare_printer_status_value`: Loads a longword from offset 4 of a structure into d6, a word from offset 84 into d3, and a word from offset 88 into d5, then branches if d5 is greater than or equal to 1008; otherwise it extends d5 to a longword in d4 and branches to another location.
- `loop_printer_calls`: Pushes several arguments onto the stack, calls a routine via a pointer in a3, subtracts 1008 from d5, adds 1008 to d3, and loops if d5 is greater than 0.
- `process_printer_data`: Checks a bit in a word at offset 78 of a structure and, if set, verifies a value at an offset of a pointer retrieved from a global variable, then performs several calls including one with a modified version of the structure and another with multiple values pushed to the stack.
- `process_printer_buffer_loop_2`: Iterates through a loop to call a routine using a value from a structure, then performs a series of conditional checks on flags and values within the structure in a2.
- `process_printer_status_and_buffer_3`: Checks a status bit in the structure at offset 78 and a bit in a global variable to determine if a routine should be called.
- `push_and_branch`: Pushes a longword from offset 56 of a structure and a pointer from a2 onto the stack, increments d0 by one, and branches to another routine.
- `process_printer_buffer_entry`: Calls a routine using a pointer from offset 12 of a structure, then calculates a value by subtracting the word at offset 86 of the structure from a value in register d3 and calls another routine with this result.
- `process_printer_buffer_loop`: Iterates through a buffer using a loop, calling various subroutines to process data entries, check status bits in a structure, and perform arithmetic on values.
- `execute_conditional_subroutine_calls`: Checks a 16-bit value at offset negative 6 from a6; if non-zero, it pushes a pointer from a2 onto the stack, calls a subroutine, and then pushes a 16-bit value from offset negative 4 of a6 onto the stack to call another subroutine.
- `process_printer_buffer_and_stream`: Increments a 16-bit counter in a stack-based structure, calls a routine with the new count and a pointer from a second structure, and performs conditional logic involving memory clearing and further calls to other routines based on the results and specific bit flags in the data structures.
- `process_printer_buffer_entries`: Iterates through a table of entries, calling a conversion routine for each byte at offset 3 or for a block of four bytes starting at the current pointer, then advances the pointer and decrements a counter.
- `advance_printer_buffer_entries`: For each of d3 entries, reads byte at offset 3 of a2, adds d4 and d2, calls a conversion routine, stores result back and advances a2 by four.
- `manage_printer_buffer_status`: Checks status bits in a structure to determine if hardware is ready, then calls two different routines based on specific bit flags.
- `push_pointers_and_increment_counter`: Pushes a longword from an offset of 56 in a2 onto the stack, pushes a2 onto the stack, and increments a 16-bit value located at an offset of -2 from a6 before branching to another routine.
- `process_printer_buffer_and_status_3`: Checks status bits in a structure and calls several routines to manage printer buffer data, including calculating remaining buffer sizes and updating a local counter.
- `dispatch_printer_io_requests`: Checks a condition in d6; if non-zero, it calls a routine using a pointer from a2 and then returns.
- `increment_and_process_printer_buffer`: Increments a counter in d5, calls another routine with the incremented value and a pointer from a2, and then enters a loop that updates values in a structure pointed to by a6 and performs conditional memory fills and calls to other routines based on flags in a2.
- `process_printer_status_and_buffer`: Checks status bits in a structure to determine if a printer is ready or busy, then calls other routines to manage the buffer or clear a memory range with a specific value if certain conditions are met.
- `push_pointer_and_branch_2`: Pushes a longword from offset 56 of the structure in a2 and the value of a2 onto the stack, increments a word from offset -2 of the structure in a6 by 1 in d0, and branches to a relative offset.
- `process_printer_buffer_and_status_2`: Calls a routine using a pointer from a structure, then performs conditional checks on status bits and a specific value in a table before calling several other routines.
- `dispatch_conditional_subroutines`: Checks a word in register d6 and, if non-zero, calls a routine via a pointer and then calls another routine with a value from d5.
- `process_printer_buffer_and_status`: Increments a counter in a structure and decrements another, then performs a loop that calls a routine with a value from a pointer, updates a 16-bit value in a structure, and conditionally fills a memory block with 65535 based on a byte check at offset 113 of a structure.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `process_printer_status_and_buffer_2`: Calls a routine to retrieve status, then checks a value in a global structure against a constant; if the value is within a specific range, it retrieves a pointer from a table and performs a series of conditional calls and data copies involving a string buffer and a global structure.
- `fetch_table_pointer`: Shifts the value in d2 left by 2 bits, uses the result to index into a table at offset 14, loads the resulting pointer into a2 and d4, then branches to another routine.
- `branch_to_offset_9`: Performs a short branch to a specific instruction offset within another routine.
- `test_byte_and_branch`: Tests the byte pointed to by a2; if the byte is non-zero, it branches to another routine, otherwise it branches to a third routine.
- `initialize_printer_context`: Clears a value at offset -4 of a6, calls two routines, and if the second returns zero, it checks a word at offset 8 of a structure pointed to by a pointer at offset 92 of a3; if that word is greater than 35, it clears offset 58 of a structure in a4, then calls a routine at offset 46 of a4 using arguments from a6 and a4.
- `initialize_printer_device`: Checks for an existing pointer in a structure, then attempts to load a segment and initialize a device structure by calling a function via a pointer at offset 12 of a newly allocated block.
- `close_streams_and_return_error`: Pushes d5 and d3, calls the close and cleanup routines, adds eight to the stack pointer, sets d0 to minus one, restores registers and returns.
- `execute_callback_and_recurse`: Checks a global value, calls a function pointer from a table, and then calls itself with a value from a pointer.
- `compare_strings_case_insensitive_3`: Compares two strings pointed to by a0 and a1 by checking if characters are within the lowercase range and normalizing them, returning zero if they match or are both null, or one otherwise.
- `set_d0_based_on_d0`: If d0 is not one, it sets d0 to 255 and branches.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `call_current_dir_2`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_lock_2`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_un_lock_2`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `rotate_printer_three_state_pointers`: Loads the caller's structure pointer from the stack, then rotates three longword fields at offsets 52, 12 and 56 so each takes the next one's value, and returns.
- `process_printer_buffer_data`: Copies data from a source buffer to a destination buffer, performs arithmetic calculations involving multiplication and bit shifting on values retrieved from a structure, and calls another routine to process the result.
- `copy_buffer_and_process_data_2`: Copies a block of data from a source address to a destination address using a loop, then performs a series of comparisons and arithmetic operations on a structure in a6 to determine whether to call another routine or perform a secondary data copy.
- `copy_buffer_byte`: Copies a longword from the address in a4 plus offset 4 to the address in a3 and increments a3, then enters a loop that decrements d1 and branches back to itself if d0 is not zero.
- `calculate_buffer_offset_and_call_io`: Compares several 16-bit values stored at offsets in a stack frame against a reference value, calculates a memory offset by multiplying a value from the stack by a register and adding a signed constant, and calls another routine with the result.
- `copy_buffer_and_process_data`: Copies a block of data from a source pointer to a destination pointer using a loop, then performs conditional checks on values within a structure in a6 to decide whether to call another routine or perform arithmetic and pointer adjustments.
- `copy_buffer_and_call_io`: Copies a block of data from a source pointer to a destination pointer using a loop, then calls two different subroutines based on the value of a field in a structure.
- `apply_color_weight_transformations`: Iterates through nested structures to transform byte values by dividing them by eight and adding an offset, then updates a target buffer with the resulting values.
- `apply_color_component_offsets`: Calculates color component offsets using a weighted average and applies them to two parallel buffers in a loop.
- `calculate_color_component_offsets`: Calculates color component offsets using a weighted average and applies them to two parallel buffers in a loop.
- `apply_weighted_color_offsets`: Calculates color component offsets using a weighted average and applies them to two parallel buffers in a loop.
- `update_buffer_values_2`: Performs a nested loop that updates bytes at offsets 3 and 7 of two different structures using arithmetic derived from d3, decrements two counters, and calls update_buffer_values.
- `update_buffer_values`: Performs a nested loop that updates bytes at offsets 3 and 7 of two different structures using arithmetic derived from d3, decrements two counters in a structure in a6, and calls two different routines based on the value of d2.
- `swap_list_heads_and_clear_new_head`: Swaps the two longword list pointers at offsets 32 and 36 of the argument structure, then clears the longword at the new head node and returns.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `return_negative_one`: Sets d0 to -1 and returns.

### EpsonQ

- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_library_calls_7`: Stores a pointer from the stack into a table, then performs up to three conditional calls to a routine at an offset from a pointer in a global variable, storing each result in a table.
- `execute_conditional_calls_5`: Stores a pointer from a global variable into a1, then performs two sequences of pushing a6 and a1 to the stack before calling a routine at an offset from a6.
- `call_library_functions_sequentially`: Calls three different library functions using pointers retrieved from a structure in d1 and a library base in a6.
- `return_constant_7`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_8`: Returns the constant an address in d0 to the caller without any other effect.
- `divide_and_swap_words`: If d2 is not greater than 65535, it performs a series of word-based divisions using d1 and d2, swapping the high and low words of the registers during the process.
- `rotate_and_wrap_32_bit_value`: Performs sixteen iterations of a 32-bit left bitwise rotation using the carry flag to transfer bits between the high and low words.
- `multiply_32_bit_unsigned`: Computes the 32-bit product of two 32-bit integers provided in registers d0 and d1 using 16-bit partial products and returns the result in d0.
- `forward_call_to_helper_2`: Calls one helper and returns, with no other work.
- `negate_result_on_nonzero_flag`: Calls another routine and returns its first result, or the negation of that result if the second result returned in an address register is nonzero.
- `render_epsonq_dot_matrix_bits`: Calculates bit offsets and destination addresses to set specific bits in a buffer based on comparisons between values in a source table and a data stream.
- `set_epsonq_column_dot_bits_2`: Compares d3 against three different byte values and, if a comparison passes, sets bit zero in the corresponding destination buffer at offset d0, then loops while decrementing two counters.
- `set_column_bits`: Iterates through a nested loop to compare bytes from a table against a threshold, setting bit zero in a destination buffer at intervals of three and advancing the pointer.
- `increment_address_and_loop`: Adds the value in d5 to a1 three times, performs a decrement and branch on a loop counter, and then branches to another location.
- `set_epsonq_column_dot_bits`: Walks the column bitmap table and sets bit zero in the output buffer for each column whose value exceeds the threshold byte, advancing three bytes per column and returning zero.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `dispatch_jump_table_entry`: Validates an index passed in d0 against a minimum value of 5, then uses that index to select and jump to a routine from a jump table located in memory.
- `restore_d2_d6_a2_a5_and_return`: Moves d2 into d0 as the return value, restores d2 through d6 and a2 through a5 from the stack frame, releases a6 and returns.
- `setup_printer_command`: Saves registers d2-d6 and a2-a4 to the stack, loads a pointer from offset 36 of the stack, a longword from offset 40, and a pointer from offset 44, then sets d1, d3, and d4 to zero and d5 to 30 before comparing a word at the pointer to 1.
- `format_epson_printer_command_2`: Copies twenty-nine bytes from a table into a buffer, replacing the end marker byte with zero, and then modifies specific bytes in the buffer based on status values found in a structure.
- `format_epson_printer_command`: Checks a status word in a structure, sets a control byte to fifteen, and copies two bytes from a data source into a buffer.
- `calculate_offset_value_5`: Loads a pointer from a0, checks a word at offset 166 of that pointer, and if it is zero, calculates a value by shifting d1 left by 2 and adding it to itself before branching.
- `compare_and_shift`: Loads a pointer from a0, compares a word at offset 166 of that pointer to 1024, and if equal, calculates a value by shifting d1 left by 2, adding it to itself, and storing it in d1.
- `update_printer_status_and_buffer`: Calls another routine to calculate a value, stores it in a structure at offset 1 and sets a byte at offset 0 to 1, then checks a status word; if the status matches a specific value, it writes a byte to a structure at offset 2720 and updates a byte at offset 3 of a structure based on a condition.
- `modify_pointer_and_copy_byte`: Decrements a value from a pointer on the stack, uses it as a base to write a byte from a table at offset 1 of a structure in a1 to offset 3 of a0, then branches to another routine.
- `copy_buffer_bytes`: Copies bytes from a source table in a1 to a destination buffer in a0 using d0 as a counter and d2 as an offset, looping until d0 is no longer greater than d1.
- `check_printer_status_3`: Compares a word at a3 with 32; if not equal, it branches to another routine, otherwise it tests a byte at a4, and if it is zero, writes 1 to a4 and 27 to a3 before branching to another routine.
- `update_status_byte_2`: Tests a byte at a4; if it is negative, it branches to another routine, otherwise it clears the byte at a4 and writes the word 30 to a location pointed to by a3, then branches to another routine.
- `set_d3_to_negative_one`: Loads negative one into d3 and branches to another routine.
- `update_status_word`: Compares a word at a3 with 33; if not equal, it tests a byte at a4, and if that byte is not zero, it writes 255 to a4, 29 to a3, and branches.
- `update_status_byte_4`: Tests a byte at a pointer in a4; if it is less than or equal to zero, it clears the byte at that pointer, writes the word 28 to a word at a3, and branches to another routine.
- `branch_to_another_routine_3`: Branches to another routine that loads negative one into register d3.
- `map_status_to_values`: Compares a word at a3 against several constants to clear or set a byte at a4, writes specific bytes to stack offsets, or writes a sequence of three bytes to a destination buffer pointed to by d3.
- `write_sequence_of_bytes`: Writes three bytes to a destination buffer at offsets determined by d2, then reads a byte from a source pointed to by a stack offset, calculates a new offset by subtracting that byte from 30, and writes a byte from a table to the destination buffer.
- `write_sequence_of_bytes_3`: Compares a word at a3 to 57; if equal, it writes the byte 27 to offset d2 of a pointer in d3, writes the byte 67 to offset d2 plus 1 of the incremented pointer, then writes a byte from a stack pointer to offset d2 plus 2 of the incremented pointer and branches to another routine; otherwise it branches to another routine.
- `write_conditional_bytes`: Compares a word at a3 to 58; if it is not equal, it branches to another routine, otherwise it writes two specific bytes to a destination buffer in a0 at offsets determined by d2 and increments d3 twice.
- `reset_printer_on_zero_status_word`: Tests the word at a3; when zero, loads the pointer from d4, writes byte 253 to offset 2720 of that structure, clears d3 and d0, restores registers and returns.
- `compare_global_word`: Saves registers to the stack, loads a longword from the stack into d2, sets a3 to 4, sets a2 to 0, compares a word at an offset of a table in a2 to 48, and if not equal branches to another routine; otherwise it sets d0 to 136 and branches to another routine.
- `lookup_char_width_and_advance_index`: Stores 80 into offset 22 of the structure in a3, shifts d2 right by 8, looks up a width word and a byte from tables indexed by that value, calls two helpers, stores results at offsets 26 and 34, and returns the sign-extended byte.
- `call_library_vector_198_11`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### CBM_MPS1000

- `return_constant_9`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_library_calls_3`: Stores several pointers into a table, then calls a library function three times with different arguments in a1, storing the results in a table before returning.
- `call_library_functions_twice`: Calls a routine at an offset from a library base twice using different pointers in a1, then sets d0 to 255 and branches.
- `call_library_functions`: Loads three different pointers from a table and calls a library routine for each one using a vector from another table.
- `return_constant_10`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_11`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_printer_command_table`: Uses a value from a stack parameter to index into a jump table, executing different logic branches based on the index.
- `move_d2_to_d0`: Moves the value from d2 to d0 and restores registers.
- `prepare_escape_sequence`: Calculates a value based on a word at offset 34 of a structure, clears a global variable at offset 26, and branches to another routine.
- `append_esc_j_terminator_and_return_end_2`: Decrements d3 and a2 in a loop until a zero byte is found, then writes an escape sequence and three bytes derived from d3 to a destination, and finally copies a block of memory if a global flag is set.
- `append_esc_j_terminator_and_return_end`: Adds the length to the output pointer, sets the flag if the length differs from the stored value, writes escape-J, the character and carriage return, and returns the new end pointer.
- `clear_memory_block_3`: Calculates a count based on a value at offset 30 of the stack and a value at offset 30 of a global variable, then enters a loop that clears a longword at a0 and decrements the count.
- `clear_memory_block`: Fills a block of memory with zeros using a counter in d0 and an incrementing pointer in a0.
- `divide_word_values`: Compares a longword in d2 to 65535; if d2 is greater than 65535, it performs a series of operations involving d1 and d2, otherwise it divides a word in a1 by a word in d2 and stores the result in d1.
- `rotate_and_subtract_32bit_value`: Performs sixteen iterations of a 32-bit left bitwise rotation using the carry flag to move bits between the high and low words.
- `multiply_two_32bit_integers`: Calculates the 32-bit product of two 32-bit integers by multiplying 16-bit partial products and returns the result in d0.
- `forward_call_to_helper_3`: Moves the contents of two registers into each other and calls a helper routine before returning.
- `call_and_negate_on_flag_2`: Calls another routine and returns its result in d0, or returns the negation of that result if the address register a0 is non-zero after the call.
- `check_command_type`: Clears registers d1 through d4, loads three pointers from the stack into a1, a3, and a4, compares the word at the pointer in a1 to 1, and branches to a different routine if they are not equal; otherwise it performs an unconditional branch.
- `update_printer_status_2_2`: Copies twenty bytes from a table into a buffer using a loop, then checks specific word values at offsets 2646, 2648, and 2650 of a structure pointed to by a value in a register to update status bytes and control characters in a destination buffer.
- `update_printer_status_2`: Moves the value in d3 to a2, loads a pointer from the address in a2 into a0, compares the word at offset 1394 of the structure in a0 to 2048, and if they are not equal branches to another location; otherwise it sets a byte at offset 5 of the structure in a4 to 15, loads a 32-bit value from a global variable into d5, and branches to another location.
- `process_printer_command_buffer`: Copies a string from a source buffer to a destination buffer using a loop, then writes specific bytes to a printer structure based on values read from a data table.
- `set_printer_byte`: Loads a pointer from a2, compares a word at offset 166 of the pointer in a0 to 1024, and if equal, writes the byte 89 to offset 1 of a structure in a0.
- `update_printer_data`: Writes the byte 80 to offset 1 of a structure in a0, writes the word 65 to the memory at a1, and if the word at a1 is 65, it writes the byte from a0 at offset 2 of a1 and then writes the byte from a0 at offset 3 of a1.
- `copy_bytes_in_loop`: Copies a byte from a source at offset d0 of a1 to a destination at offset a4 of a0, then increments d1 and d2 and loops if d2 is not greater than a0.
- `verify_printer_status`: Compares a word at a1 with 32; if not equal, it branches to another routine, otherwise it tests a byte at a3, and if it is zero, writes 1 to a3 and 27 to a1 before branching to another routine.
- `test_and_clear_byte`: Tests a byte at a3; if it is not greater than or equal to zero, it clears the byte and writes the word 46 to a1, otherwise it branches to another location.
- `set_d1_and_branch`: Sets d0 to 255, moves it to d1, and branches to another location.
- `update_status_word_alt`: Compares a word at a1 with 33; if not equal, it tests a byte at a3, and if that byte is not zero, it writes 255 to a3, 29 to a1, and branches.
- `modify_word_at_pointer`: Tests a byte at a pointer in a3; if it is less than or equal to zero, it clears the byte at that pointer, writes the word 28 to a word at a1, and branches to another routine.
- `branch_to_offset_10`: Performs a short branch to a specific offset.
- `map_command_to_byte`: Compares a word at a1 against several constants to set a byte at a3 to specific values, or writes specific bytes to a destination at offset 48 of the stack to a buffer at offset a4 of d1.
- `clear_d1_d0_and_send_fd_if_zero`: Tests the word at a1; if zero, loads a1 from d3, writes byte 253 to offset 2720 of the pointed object, then clears d1 and d0 and restores registers.
- `check_printer_status`: Compares a word at an offset of a table in a2 with 48 and, if equal, sets d0 to 88; otherwise it returns.
- `configure_printer_settings`: Writes 80 to offset 22 of a structure, performs several table lookups to fetch values, and writes them to offsets 34 and 36 of the structure, finally setting offset 24 based on a comparison.
- `set_printer_mode`: Compares a word at offset 24 of the structure in a3 to 144; if they are equal, it writes 16 to offset 24 of the structure in a2, otherwise it branches to another routine.
- `set_printer_mode_8_and_fetch_font_byte`: Writes 8 to the word at offset 24 of the structure pointed to by (a3), then reads a signed byte from the font table indexed by d2, sign-extends it to d0 and restores d2/a2-a3.
- `render_bitmapped_pattern`: Calculates a destination offset based on a hardware identifier and a scaling factor, then enters nested loops to set bits in a destination buffer by comparing values from a source table against a threshold.
- `set_bitmap_bits`: Checks a value at offset 54 of a structure, then enters a nested loop to set bits in a destination buffer based on comparisons between a table and a threshold byte.
- `increment_pointer_and_loop`: Adds the word in d5 to a1, decrements d1 as a loop counter, and branches to another routine if the counter is not zero.
- `render_glyph_rows_using_shift_table`: Walks glyph rows and widths, compares each row byte with a shift-table threshold indexed by the low bits of d2, sets bits in the output at a1 plus d2, then restores registers and returns zero.
- `call_library_vector_198_12`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_10`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### HP_LaserJet

- `return_constant_12`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_library_function_pointers`: Populates a table with three function pointers by calling a routine at a fixed offset from a library base using three different addresses in a1, then returns 255 if any call fails.
- `execute_library_calls_8`: Calls a routine at an offset of a6 twice using values from a1 and a stack-saved pointer, then sets d0 to 255 and returns.
- `execute_three_library_calls`: Fetches three pointers from a structure and calls a library routine for each using the address found at offset 8 of the structure.
- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `update_printer_state_and_buffer`: If a specific value is found in a parameter, it copies thirty bytes from a table into a structure, sets a byte at offset thirteen to thirteen, and modifies bytes at offsets seven, fifteen, and seventeen based on values read from a global table before calling initialize_printer_settings_structure and copy_table_bytes_and_call_routine.
- `initialize_printer_settings_structure`: Copies 30 bytes from a table into a structure at an offset, sets a byte at offset 13 to 13, and checks two values in a table to set bytes at offsets 7 and 15 of the structure.
- `configure_printer_settings_3`: Checks two values in a table to set specific bytes at offsets 15 and 17 of a structure, then copies 11 bytes from a table into a destination buffer and calls another routine.
- `copy_table_bytes_and_call_routine`: Copies 11 bytes from a table into a destination buffer using a loop, calls another routine, and then updates two bytes in a structure using values from a third structure before potentially calling another routine again.
- `copy_and_call_twice`: Copies 12 bytes from a table at a1 to a destination at a0 plus an offset in a4 using a loop, then calls another routine twice with different values derived from a structure at a6.
- `update_laserjet_status`: Compares a word at the address in a2 to 27; if it matches, it checks if the byte at the address in a3 is less than zero, and if so, writes 32 to the address in a2 and 1 to the address in a3.
- `check_laserjet_status`: Compares a word at the address in a2 to 27; if it matches, it checks if the byte at the address in a3 is less than zero, and if so, writes 4 to the address in a2 before branching.
- `update_laserjet_status_flag`: Compares a word at the address in a2 to 28 and a byte at the address in a3 to zero; if both conditions are met, it writes 33 to the address in a2, otherwise it returns.
- `update_laserjet_values`: Compares a word at the address in a2 to 29 and a byte at the address in a3 to zero; if both conditions are met, it writes 33 to the address in a2, writes 255 to the address in a3, and branches to another routine.
- `update_laserjet_buffer`: Compares a word at the address in a2 to 29; if they are not equal, it branches to another routine, otherwise it checks a byte at a3 and, if it is greater than zero, writes 2 to the address in a2.
- `update_buffer_value_2`: Compares a word at the address in a2 to 30; if it is not equal, it branches to another routine, otherwise it checks a byte at a3 and, if it is less than 0, writes 32 to the word at the address in a2.
- `update_buffer_value`: Compares a word at the address in a2 to 31; if not equal, it branches to another routine, otherwise it checks a byte at a3 and writes either 33 or 32 to the address in a2, and clears the byte at a3 before branching.
- `increment_buffer_index`: Compares a word at the address in a2 to 32; if not equal, it branches to another routine, otherwise it increments the byte at the address in a3 and branches to another routine.
- `decrement_and_branch_3`: Compares the word at the address in a2 with 33; if not equal, it branches to another routine, otherwise it decrements the byte at the address in a3 and branches to another routine.
- `compare_word_and_branch_4`: Compares the word at the address in a2 with 64 and branches to another routine if they are not equal; otherwise, it loads a pointer from offset 28 of the structure in a6, tests the byte at that pointer, and if the byte is non-zero, branches to another routine; if the byte is zero, it writes the byte from d5 to that same pointer and branches to another routine.
- `decrement_and_update_byte`: Decrements a byte at the address pointed to by an offset of 28 in a6, then checks if that byte is zero; if it is zero, it writes the byte from d6 to the address at offset 1 of that same pointer, otherwise it branches to another routine.
- `branch_to_offset_14`: Performs a short branch to a specific instruction located at a fixed offset within another routine.
- `copy_and_call_with_subtraction`: Copies 11 bytes from a table at a1 to a destination at a2 plus an offset in a4 using a loop, then calls another routine twice with values derived from a structure at a6.
- `compare_word_and_branch_2`: Compares the word at a2 with 57 and branches to another routine if they are not equal, otherwise it branches to a third location.
- `copy_and_call_routines`: Copies bytes from a table at a1 to a destination at a2 using a loop controlled by d2, then calls another routine twice with different values in d5 and d6.
- `test_a2_word_and_write_fd_to_port`: Tests the word at a2; if zero, writes byte 253 to offset 2720 of the pointer from d3, then returns zero in d0 after restoring registers.
- `write_ascii_digit`: Saves registers to the stack, loads a byte from the stack into d0, a pointer into a1, and an offset into d3; if d0 is not less than or equal to 199, it writes 50 to a1 plus d3 and subtracts 200 from d0.
- `write_digit_byte`: If d0 is not less than or equal to 99, it writes the byte 49 to a destination at offset d3 of a0 and subtracts 100 from d0; otherwise it writes the byte 49 to a destination at offset d3 of a0.
- `format_decimal_digit`: Writes the byte 48 to a destination at an offset specified by d3, and if d0 is greater than 9, divides d0 by 10 and writes the result plus 48 to the next byte.
- `format_byte_as_two_decimal_digits`: Writes the byte 48 at offset d3 in a1, calls another routine to convert a byte in d0 to a value, adds 48 to that value, and stores the result at offset d3 plus 1 in a2.
- `abort_print_on_form_feed_command`: Reads the command byte from the stack, and if it equals twelve clears the longword at offset 58 of the printer structure, then returns minus one in d0.
- `flush_pending_print_job_on_abort`: Checks the pending-job flag at offset 58 of the printer structure; if set, calls the abort and flush vectors from the device structure, clears the flag, pops eight bytes and returns zero.
- `divide_and_swap`: If d2 is not greater than 65535, it performs a series of divisions and swaps using d1, d2, and d0 to calculate a result and returns.
- `rotate_and_subtract_32bit_value_2`: Performs sixteen iterations of a 32-bit left bitwise rotation using the carry flag to move bits between the high and low words, and subtracts a threshold value from the high word if it meets or exceeds it.
- `multiply_32_bit_integers_5`: Computes the 32-bit product of two 32-bit integers using 16-bit partial products and returns the result in d0.
- `forward_call_to_helper_4`: Moves the contents of two registers into a third and a fourth register respectively, calls a helper routine, and returns.
- `call_helper_and_return_d1`: Moves the value from d0 into d1, calls another routine, and returns the value that the called routine leaves in d1.
- `negate_result_on_flag_2`: Calls another routine and negates the returned value if the address register returned by that routine is non-zero.
- `call_and_negate_on_flag_3`: Calls another routine and returns its result in d1, or returns the negation of that result if the address register a0 is non-zero after the call.
- `manage_printer_buffer_state`: Implements a state machine that manages a printer buffer by performing operations such as calculating buffer sizes, writing escape sequences to memory, clearing buffer contents via a loop, and calling a function pointer at offset 100 of a structure to handle data output or status updates.
- `save_and_return_d0`: Copies the value from d5 into d0, restores registers from the stack, and returns.
- `render_glyph_bits`: Uses a nested loop to compare bytes from a table against a threshold, setting bits in a destination buffer at offsets calculated from a word in d2 and then increments d2.
- `add_and_loop`: Adds the contents of d5 to d2, decrements d1, and loops if d1 is not zero, otherwise it branches to another location.
- `render_glyph_rows_with_table_threshold`: Selects a threshold table by the low bits of d0, then sets bits in the output bitmap where each glyph byte exceeds the table value, returning zero after restoring registers.
- `call_library_vector_198_13`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_11`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `fill_printer_pen_and_color_fields`: Takes a pen index from the stack, shifts it right by eight, then writes scaled pen width and height plus two color words and three color bytes into the printer structure pointed to by the global pointer.

### EpsonX[CBM_MPS-1250]

- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.
- `update_structure_pointers`: Stores several values into a structure, then performs three conditional calls to a routine at an offset of a6, storing the results in a structure, and returns.
- `execute_conditional_library_calls`: Loads a pointer from a global variable into a1, then performs two consecutive calls to routines located at an offset from a pointer in a global variable.
- `execute_three_library_calls_2`: Fetches three pointers from a structure, then calls a library function three times using those pointers and a base address from a global variable.
- `return_constant_15`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_16`: Returns the constant an address in d0 to the caller without any other effect.
- `divide_word_by_word`: If a long value in d2 is not greater than 65535, it performs a series of word-based divisions using d1 and a1 to calculate a result stored in d1 and d0, then returns.
- `rotate_and_wrap_32bit_value`: Performs sixteen iterations of a 32-bit left bitwise rotation using the carry flag to transfer bits between the high and low words.
- `multiply_two_32bit_integers_2`: Computes the 32-bit product of two 32-bit integers by calculating partial products of their 16-bit components and returns the result in d0.
- `forward_call_to_helper_5`: Moves the contents of two registers into each other and calls another routine before returning.
- `call_and_negate_on_flag_4`: Calls another routine and returns its first result, or the negation of that result if the second result left in an address register is nonzero.
- `apply_threshold_bitmask_to_buffers`: Calculates a bit offset and destination address based on a parameter and a printer type check, then iterates through source buffers to set specific bits in one of several destination buffers by comparing source bytes against threshold values.
- `apply_threshold_bitmask`: Compares bytes in a source buffer against multiple threshold values and sets bits in one of three destination buffers based on which threshold is met.
- `set_bits_from_thresholds`: Compares bytes in d3 against three different thresholds and sets bits in three separate destination buffers at an offset defined by d2, then decrements two global counters in a loop.
- `render_glyph_bits_2`: Iterates through a nested loop to compare bytes from a table against a threshold, setting bits in a destination buffer based on the comparison result.
- `update_pointer_and_loop`: Adds the value in d5 to a1, performs a loop using d1 as a counter, and then branches to another routine.
- `render_glyph_rows_using_shift_table_2`: Iterates through rows and widths of a glyph structure, comparing each row byte against a threshold from a table indexed by the low two bits of the current index, and sets a bit in an output buffer at a calculated offset based on a bitmask in a register.
- `perform_zero_or_instruction`: Performs a logical OR of byte 0 with d0.
- `dispatch_printer_command_switch`: Uses a value from the stack to index into a jump table, calling various subroutines that manage printer buffer sizes, line lengths, and command sequences based on the printer model identified in a global structure.
- `return_d0_from_d2`: Moves the value from d2 into d0 and restores registers before returning.
- `clear_structure_field`: Saves registers to the stack, loads a longword into d2, a pointer into a0, and a longword into d3; if a longword at an offset from the stack is not zero, it clears a longword at an offset from a2 and branches.
- `calculate_offset_and_branch`: Sets d1 to 0, loads a 32-bit word from an offset of the stack pointer into d1, copies d1 to d4, adds d2 to d4, stores the result in d1, subtracts 1 from d1, moves d1 to a1, and branches to another location.
- `process_data_stream`: Iterates through a buffer, writing specific byte sequences and values derived from a table and register d4 to a destination pointed to by a6, then optionally copies bytes from a source to a destination pointed to by a2.
- `append_data_sequence`: Calculates a destination address in a2, checks several values in a table, and if a condition is met, writes three bytes and a byte from d3 to the destination at a2 and returns.
- `clear_memory_block_4`: Checks a bit in a pointer and clears a block of memory by writing zero to each byte, using either a loop or a word-based clearing method depending on the pointer's alignment.
- `clear_memory_bytes`: Clears a block of memory by writing zero to each byte pointed to by a register, decrementing a counter in another register until it reaches zero, and then returns.
- `initialize_printer_command_parameters`: Loads four pointers and two 32-bit values from the stack, clears the last two values, and checks if the value at offset thirty-six of the first pointer is equal to one; otherwise it returns.
- `format_epson_printer_command_4`: Copies 26 bytes from a table into a buffer using a loop, then checks specific status words in a structure to update bytes at offsets 7, 9, 17, or 24 of that buffer.
- `format_epson_printer_command_3`: Checks a status word at offset 2646 of a structure and, if it matches 2048, updates two bytes in a buffer at offsets 7 and 9.
- `prepare_epson_buffer`: Moves a pointer from d4 to a0, loads a pointer from the address in a0 into a2, compares a word at offset 884 of a2 to 1024, and if they are equal, performs arithmetic on d0 and d1 before branching.
- `update_epson_buffer`: Moves a pointer from d4 to a0, loads a pointer from the address in a0 into a2, compares a word at offset 884 of a2 to 1024, and if they are equal, performs several arithmetic operations on d0 and d1 before branching.
- `update_printer_status_and_buffer_2`: Calculates a value by calling another routine, stores it in a structure, and updates a status byte.
- `modify_pointer_and_byte`: Loads a pointer from the stack, reads a byte from that pointer, subtracts 1 from the byte value, stores the result at offset 3 of a structure pointed to by d5, and then copies a byte from offset 1 of the stack pointer's target to offset 6 of a structure pointed to by d5 before branching.
- `copy_bytes_with_loop`: Copies bytes from a source pointed to by a1 to a destination pointed to by a0 using offsets in d0 and d2, then loops if a pointer in d1 is greater than 8, otherwise it branches to another location.
- `update_epson_status`: Compares a word at the address in a3 to 32 and a byte at the address in a4 to zero; if both conditions are met, it writes 1 to the address in a4, writes 27 to the address in a3, and branches to another routine.
- `update_status_and_branch`: Tests a byte at a4; if it is signed greater than or equal to zero, it branches to another location, otherwise it clears the byte at a4, writes the word 30 to the address in a3, and branches to another location.
- `set_and_branch`: Sets d3 to negative 255 and branches to another location.
- `modify_status_byte`: Compares a word at a3 to 33; if not equal, it branches to another routine, otherwise it checks if a byte at a4 is non-zero, clears a4, and writes 29 to a3 before branching.
- `clear_status_byte_2`: If the byte at a4 is not less than or equal to zero, it clears the byte at a4 and writes the word 28 to a destination at a3; otherwise it branches to another routine.
- `branch_to_offset_15`: Performs an unconditional short branch to another location.
- `write_printer_data`: Compares a word at a3 against several constants to set a byte in a4 or a byte at an offset from a pointer in a2, and if a3 is 2, it writes two specific bytes to the address in d3 plus the offset in d2 and d2 plus 1.
- `write_indexed_bytes`: Compares a word at a3 to 4; if not equal, it branches to another routine, otherwise it writes 27 to a byte at offset d2 of a pointer in d3, increments d3, and writes 106 to a byte at offset d2 of the new pointer.
- `check_conditions_and_branch`: Compares a word at a3 to 12 and branches if not equal, then compares a byte at the pointer on the stack to 39 and branches if not equal, then writes the byte 30 to the pointer on the stack, and finally compares a byte at the pointer on the stack to 37 and branches if the value is unsigned greater than or equal to 37, otherwise it branches to another location.
- `write_sequence_of_bytes_2`: Writes a sequence of bytes to a destination pointed to by d3 and d2, including two constant bytes, one byte from a table based on a value from a stack pointer, and two more constant bytes and a zero byte, then branches to another location.
- `write_printer_status_byte`: Tests a 16-bit word at a3; if zero, it writes the byte 253 to offset 2720 of the structure pointed to by the value in d4, then returns.
- `check_printer_status_2`: Saves registers, loads a value from a stack offset into d2, compares a word at an offset of a pointer in a2 against 48, and if not equal, sets d0 to 88 and branches.
- `update_printer_settings`: Writes a value to offset 22 of a structure, calls two other routines, and then copies values from a table into offsets 26, 34, and 36 of the structure, potentially updating offset 24 if a specific value is found.
- `check_and_set_mode`: Compares a word at offset 36 of the structure in a2 with 144; if they match, it writes 16 to offset 24 of the structure in a2 and branches to another routine, otherwise it branches to another routine.
- `set_printer_mode_8_and_fetch_font_byte_2`: Writes 8 to the word at offset 24 of the structure pointed to by a3, then reads a signed byte from the font table indexed by d2, sign-extends it to d0 and restores d2/a2-a3.
- `call_library_vector_198_14`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_12`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Okimate_20

- `return_constant_17`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_library_calls_4`: Sets up several values in a structure and calls a routine at an offset from a library base three times, branching early if any call returns zero.
- `execute_library_calls_9`: Loads a pointer from a stack offset into a1, saves a6 to the stack, calls a routine at an offset of a6, restores a6 from the stack, loads a new pointer into a1, saves a6 to the stack again, calls the same routine offset, restores a6, and sets d0 to 255.
- `call_three_library_functions`: Loads three different pointers from a table and calls the function at each pointer using a library vector.
- `return_constant_18`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_19`: Returns the constant an address in d0 to the caller without any other effect.
- `divide_and_swap_words_2`: If a 32-bit value in d2 is greater than 65535, it calls another routine to rotate the value; otherwise, it performs division using the high and low words of d1 and d2 and returns the result in d1.
- `rotate_and_decrement_32bit_value`: Performs sixteen iterations of a 32-bit left bitwise rotation using the carry flag to transfer bits between the high and low words.
- `multiply_32_bit_signed_integers_2`: Computes the 32-bit product of two 16-bit values stored in d0 and d1 using partial products, then returns the result in d0.
- `forward_call_to_helper_8`: Moves the contents of d1 into d2 and d0 into d1 before calling another routine, then restores d2 from the stack and returns.
- `call_scan_for_marker_and_return_d1`: Moves the value from d0 to d1, calls scan_for_marker, and returns the value left in d1.
- `negate_result_based_on_sign`: Calls another routine and returns its result in d0, or returns the negation of that result if the second input parameter was negative.
- `dispatch_via_jump_table`: Validates a value passed in a register against a minimum threshold, then uses it as an index into a jump table to call another routine.
- `return_d1_in_d0_restore_regs`: Moves d1 into d0 as the return value, restores d2-d6 and a2-a5 from the a6 frame, releases the frame and returns.
- `restore_and_branch`: Restores registers a2 through a4 from the stack, loads a1 from an offset of 16 from the stack, a2 from an offset of 20 from the stack, and a0 from an offset of 24 from the stack, then compares the word at the address in a1 with 1; if not equal, it branches to another routine, otherwise it branches to another routine.
- `copy_and_verify_status`: Copies 15 bytes from a table to a destination, clears specific bytes in the destination, and then writes specific byte sequences to the destination based on comparisons of words at offsets 2646 and 2648 of a structure.
- `write_status_commands`: Checks a word at offset 2646 of a structure pointed to by a4 and a word at offset 2650; based on these values, it writes a sequence of bytes to a destination pointed to by a3 and a value to the stack.
- `write_sequential_bytes`: Compares the word at the address in a1 with 3 and branches to another routine if they are not equal; otherwise, it writes the byte 13 to the address calculated by adding the value in a2 to the address in a3, increments a3 by 1, writes the byte 10 to the address calculated by adding the value in a2 to the new a3, increments a3 by 1, and branches to another routine.
- `update_status_flags`: Compares a word at the address in a1 to 32; if it is equal, it tests a byte at the address in a0, and if that byte is non-zero, it writes 1 to the address in a0 and 27 to the address in a1, otherwise it branches to another routine.
- `check_status_byte`: Tests a byte at a pointer in a0; if it is negative, it clears the byte at that pointer, writes 30 to a word at a1, and branches to another routine; otherwise it branches to another routine.
- `load_constant_to_registers`: Loads the value negative 255 into d1 and then moves it into d0 before branching to another routine.
- `update_status_word_okimate`: Compares a word at a1 with 33; if not equal, it tests a byte at a0, and if that byte is not zero, it writes 255 to a0, 29 to a1, and branches.
- `test_byte_and_clear`: Tests a byte at the address in a0; if it is less than or equal to zero, it clears the byte at a0, writes 28 to the word at a1, and branches.
- `branch_to_another_routine_7`: This routine performs a short branch to another routine.
- `map_printer_status_to_byte_codes`: Compares the word at (a1) against seven status values and writes corresponding bytes to (a0) or the caller's stack slot, then returns zero in d0.
- `render_bitmapped_planes`: Compares bytes from a source table against threshold values and performs bitwise OR operations to set color bits in a destination buffer.
- `render_plane_rows`: Loops through rows and pixel counts, compares a byte from a table against a value from a source, and ORs a color byte into a destination buffer at an offset determined by a pointer in a6.
- `render_single_plane_rows_and_return`: Loops over rows and pixel counts, compares the fourth data byte against the nibble mask table, ORs the colour bit into the output plane, then restores registers and returns.
- `call_library_vector_198_15`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_13`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### HP_DeskJet

- `return_constant_20`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_library_calls_5`: Stores a value from the stack into a table, sets a pointer in that table to the address of a global variable, and calls a routine at an offset from a library base three times using different pointers in a1, branching based on the results.
- `execute_library_calls_3_2`: Loads a pointer from a global variable into a1, calls a function via a pointer in a table, restores a6 from the stack, then repeats this process with a second pointer from a global variable into a1 before performing a short branch.
- `execute_sequential_library_calls`: Fetches three consecutive longwords from a table and uses each as an argument for three successive calls to a library function retrieved from a separate pointer.
- `return_constant_21`: Returns the constant an address in d0 to the caller without any other effect.
- `process_printer_state_and_settings`: Checks a status value in a structure and, if it equals one, copies thirty bytes from a table into a destination buffer, sets a byte at offset thirteen to thirteen, and updates several bytes based on values read from a global table.
- `configure_hp_deskjet_settings`: Copies 30 bytes from a table into a structure, sets a byte at offset 7 to 13, and checks two values in a table to set bytes at offsets 15 and 17 of the structure based on whether they match 512 or 1024.
- `configure_printer_settings_4`: Checks two values in a table to set specific bytes at offsets 15 and 17 of a structure, then copies 11 bytes from a table into a destination buffer before calling another routine.
- `copy_table_bytes_and_call_routine_2`: Copies 11 bytes from a table to a destination using a loop, calls another routine, and then updates two bytes in a structure using values from a third structure before calling that same routine again.
- `copy_bytes_and_call_twice`: Copies 12 bytes from a table into a destination buffer using a loop, then calls another routine twice with different offsets and values derived from a structure in a6.
- `update_printer_status_word`: Compares a word at the address in a2 to 27; if it matches, it checks if the byte at the address in a3 is less than zero, and if so, writes 32 to the word at a2 and 1 to the byte at a3, otherwise it returns.
- `update_laserjet_status_flag_2`: Compares a word at the address in a2 to 27 and a byte at the address in a3 to zero; if both conditions are met, it writes 4 to the word at the address in a2 and otherwise it returns.
- `update_word_value`: Compares a word at the address in a2 to 28; if it is not equal, it branches to another routine, otherwise it checks if the byte at the address in a3 is greater than zero and, if so, writes 33 to the address in a2.
- `validate_and_update_printer_status`: Compares a word at the address in a2 to 29 and a byte at the address in a3 to zero; if both conditions are met, it writes 33 to the word at a2, writes 255 to the byte at a3, and branches to another routine.
- `update_buffer_word`: Compares a word at the address in a2 to 29; if it is not equal, it branches to another routine, otherwise it checks if the byte at the address in a3 is greater than zero and, if so, writes 2 to the address in a2.
- `update_buffer_value_3`: Compares a word at the address in a2 to 30; if it is not 30, it branches to another routine, otherwise it checks if the byte at the address in a3 is less than 0, and if so, writes 32 to the word at the address in a2 before branching to another routine.
- `test_and_set_buffer`: Compares a word at a2 to 31; if not equal, it branches to another routine, otherwise it checks a byte at a3 and writes either 33 or 32 to the address in a2, and clears the byte at a3 before branching.
- `compare_and_increment_counter`: Compares a word at the address in a2 with 32; if not equal, it branches to another routine, otherwise it increments a byte at the address in a3 and branches to another routine.
- `decrement_and_branch_on_match`: Compares the word at the address in a2 with 33; if not equal, it branches to another routine, otherwise it decrements a byte in a global variable and branches to another routine.

The function list continues in: g1751-recovery-functions-2.md, g1751-recovery-functions-3.md.
